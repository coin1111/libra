
/////////////////////////////////////////////////////////////////////////
// 0L Module
// Escrow contract for bridge functionality
// 
/////////////////////////////////////////////////////////////////////////

address 0x1 {
    module BridgeEscrowMultisig {
        use 0x1::Signer;
        use 0x1::Diem;
        use 0x1::DiemAccount;
        //use 0x1::DiemSystem;
        use 0x1::GAS::GAS;
        use 0x1::Vector;
        use 0x1::Option::{Self, Option};

        const ERROR_BRIDGE_STORE_EXISTS:u64 = 3000;
        const ERROR_ALREADY_ACCOUNT_EXISTS: u64 = 3001;
        const ERROR_AMOUNT_MUST_BE_POSITIVE: u64 = 3003;
        const ERROR_INSUFFICIENT_BALANCE: u64 = 3004;
        const ERROR_NO_ESCROW_ACCOUNT: u64 = 3006;
        const ERROR_LOCKED_EMPTY: u64 = 3308;
        const ERROR_INVALID_TRANSFER_ID : u64 = 3309;
        const ERROR_TRANSFER_ID_EXISTS : u64 = 3310;
        const ERROR_MUST_BE_VALIDATOR: u64 = 3311;
        const ERROR_NO_RECEIVER_ACCOUNT: u64 = 3312;
        const ERROR_TOO_MANY_EXECUTORS: u64 = 3313;
        const ERROR_IS_CLOSED: u64 = 3314;
        const ERROR_UNLOCKED_EMPTY: u64 = 3315;
        const ERROR_ALREADY_VOTED: u64 = 3316;
        const ERROR_MUST_BE_EXECUTOR: u64 = 3317;
        const ERROR_UNLOCKED_MUST_BE_CLOSED: u64 = 3318;
        const ERROR_PARAMETER_MISMATCH: u64 = 3319;

        const ZERO_ADDRESS: address = @0x0;

        const ETH_ACCOUNT_LENGTH: u64 = 20;

        struct AccountInfo has copy, store, drop {
            // user address on this chain
            // 0L->eth transfer
            sender_this: address,
            // user address on the other chain
            // eth->0L transfer
            sender_other: vector<u8>,
            // user address on the other chain
            // receiver address on 0L chain
            // eth->0L transfer
            receiver_this: address,
            // receiver address on eth chain
            // 0L->eth transfer
            receiver_other: vector<u8>,
            // value sent
            balance: u64,
            // transfer id
            transfer_id: vector<u8>,
            // multisig votes
            votes: vector<address>,
            current_votes: u64,
            // transfer is closed
            is_closed: bool,
        }

        // State of escrow account
        // Escrow account holds funds to be tranferred on the other chain
        // as well as state of transfer
        struct EscrowState has key {
            // tracks funds locked on this chain and ready to be
            // transferred on the other chain
            locked: vector<AccountInfo>,
            // tracks funds unlocked on the other chian and already transferred
            unlocked: vector<AccountInfo>,
            // tokens
            tokens: Diem::Diem<GAS>,

            // executors which are allowed to withdraw funds
            executors: vector<address>,

            // minimum votes/signatures required for multisig
             min_votes:u64,
        }
        // Creates en empty escrow account state
        public fun initialize_escrow(escrow: &signer, executors: vector<address>, min_votes:u64) {
            assert(Vector::length(&executors) < 256, ERROR_TOO_MANY_EXECUTORS);
            let escrow_addr = Signer::address_of(escrow);
            assert(!exists<EscrowState>(escrow_addr), ERROR_BRIDGE_STORE_EXISTS);
            move_to<EscrowState>(escrow, EscrowState{
                locked: Vector::empty<AccountInfo>(),
                unlocked: Vector::empty<AccountInfo>(),
                tokens: Diem::zero<GAS>(),
                executors:executors,
                min_votes:min_votes,
            });
        }

        // Transfer token to escrow account to be used as
        // for ETH->0L transfers
        public fun deposit_funds(escrow: address,
                                           sender: &signer,
                                           amount: u64) acquires EscrowState {
            // validate arguments
            assert (amount > 0, ERROR_AMOUNT_MUST_BE_POSITIVE);

            // sender has enough funds
            let sender_this = Signer::address_of(sender);
            assert(DiemAccount::balance<GAS>(sender_this) >= amount, ERROR_INSUFFICIENT_BALANCE);

            // escrow account exists
            assert (exists<EscrowState>(escrow), ERROR_NO_ESCROW_ACCOUNT);

            // 1. move funds from user to escrow account
            let with_cap = DiemAccount::extract_withdraw_capability(sender);
            let tokens = DiemAccount::withdraw_tokens<GAS>(&with_cap, escrow, amount, x"");
            DiemAccount::restore_withdraw_capability(with_cap);

            // 2. update escrow state
            let state = borrow_global_mut<EscrowState>(escrow);
            Diem::deposit(&mut state.tokens,tokens);
        }

        // TODO: implement multisig version
//        // Withdraw funds from escrow
//        public fun withdraw_funds(sender: &signer,
//                                        escrow_address: address,
//                                        receiver_this:address, // receiver on this chain
//                                        balance: u64
//        ) acquires EscrowState  {
//            let sender_address= Signer::address_of(sender);
//            assert(DiemSystem::is_validator(sender_address) == true ||
//                   sender_address == escrow_address , ERROR_MUST_BE_VALIDATOR);
//
//            // update escrow state
//            let state = borrow_global_mut<EscrowState>( escrow_address);
//
//            // escrow has enough funds
//            assert(Diem::get_value(&state.tokens) >= balance, ERROR_INSUFFICIENT_BALANCE);
//
//            // withdraw tokens from escrow
//            let tokens = Diem::withdraw(&mut state.tokens,balance);
//
//            // move funds from escrow to user account
//            DiemAccount::deposit_tokens<GAS>(sender, escrow_address, receiver_this, tokens, x"", x"");
//        }


        // Creates an account for transfer between 0L->eth accounts
        // When user initiates a transfer it calls this method which
        // moves funds from user account into an escrow account.
        // It also creates an entry in locked to indicate such transfer.
        // Executed under user account
        public fun create_transfer_account(escrow: address,
                                           sender: &signer,
                                           receiver_other: vector<u8>,
                                           amount: u64,
                                           transfer_id: vector<u8>) acquires EscrowState {
            let idx_opt = find_locked_idx(escrow, &transfer_id);
            assert(Option::is_none(&idx_opt), ERROR_TRANSFER_ID_EXISTS);

            // validate arguments
            assert (amount > 0, ERROR_AMOUNT_MUST_BE_POSITIVE);

            // sender has enough funds
            let sender_this = Signer::address_of(sender);
            assert(DiemAccount::balance<GAS>(sender_this) >= amount, ERROR_INSUFFICIENT_BALANCE);

            // escrow account exists
            assert (exists<EscrowState>(escrow), ERROR_NO_ESCROW_ACCOUNT);

            // receiver_other must be non-empty OR receiver must exists and have no -
            assert(Vector::length(&receiver_other) == ETH_ACCOUNT_LENGTH, ERROR_NO_RECEIVER_ACCOUNT);

            // 1. move funds from user to escrow account
            let with_cap = DiemAccount::extract_withdraw_capability(sender);
            let tokens = DiemAccount::withdraw_tokens<GAS>(&with_cap, escrow, amount, x"");
            DiemAccount::restore_withdraw_capability(with_cap);

            // 2. update escrow state

            // update escrow balance
            let state = borrow_global_mut<EscrowState>(escrow);
            Diem::deposit(&mut state.tokens,tokens);

            // create an entry in locked vector
            Vector::push_back<AccountInfo>(&mut state.locked, AccountInfo{
                sender_this: sender_this,
                sender_other: Vector::empty<u8>(),
                receiver_this: ZERO_ADDRESS,
                receiver_other: receiver_other,
                balance: amount,
                transfer_id: transfer_id,
                votes: Vector::empty<address>(),
                current_votes: 0,
                is_closed: false,
            });
        }

        // Moves funds from escrow account to user account between eth->0L accounts
        // Creates an entry in unlocked vector to indicate such transfer.
        // Executed under escrow account
        public fun withdraw_from_escrow(sender: &signer,
                                        escrow_address: address,
                                        sender_other: vector<u8>, // sender on the other chain
                                        receiver_this:address, // receiver on this chain
                                        balance: u64, // balance to transfer
                                        transfer_id: vector<u8>, // transfer_id
                                        ) acquires EscrowState  {
            let sender_address= Signer::address_of(sender);
            assert(is_executor(&escrow_address, &sender_address), ERROR_MUST_BE_EXECUTOR);

            // check that transfer id is not present
            let idx_opt = find_unlocked_idx( escrow_address, &transfer_id);
            let state = borrow_global_mut<EscrowState>( escrow_address);
            if (Option::is_none(&idx_opt)) {
                // if this is the first call init transfer entry
                let votes = Vector::empty<address>();
                // special case of min_votes = 1
                let is_closed = false;
                let current_votes = 0;
                if (state.min_votes == 1) {
                    // insert AccountInfo entry in closed state,e .g. single voter already voted
                    is_closed = true;
                    // transfer funds
                    // escrow has enough funds
                    assert(Diem::get_value(&state.tokens) >= balance, ERROR_INSUFFICIENT_BALANCE);

                    // withdraw tokens from escrow
                    let tokens = Diem::withdraw(&mut state.tokens, balance);

                    // move funds from escrow to user account
                    DiemAccount::deposit_tokens<GAS>(sender, escrow_address, receiver_this, tokens, x"", x"")
                } else {
                    Vector::push_back(&mut votes, sender_address);
                    current_votes = 1;
                };
                let ai = AccountInfo{
                    sender_this: ZERO_ADDRESS,
                    sender_other: sender_other,
                    receiver_this: copy receiver_this,
                    receiver_other: Vector::empty<u8>(),
                    balance: balance,
                    transfer_id: transfer_id,
                    votes: votes,
                    current_votes,
                    is_closed,
                };
                // update escrow state
                Vector::push_back<AccountInfo>(&mut state.unlocked, ai);
                return
            } else {
                // add voter
                let idx = Option::borrow(&idx_opt);
                let ai = Vector::borrow_mut<AccountInfo>(&mut state.unlocked, *idx);
                // transfer must not be closed
                assert(!ai.is_closed, ERROR_IS_CLOSED);
                // make sure this votes didn't vote before
                let vote_idx = find_address_idx(&sender_address, &ai.votes);
                assert(Option::is_none(&vote_idx), ERROR_ALREADY_VOTED);

                // make suure call params match
                assert(&ai.sender_other == &sender_other &&
                       ai.receiver_this == receiver_this &&
                       ai.balance == balance, ERROR_PARAMETER_MISMATCH);

                // update votes
                ai.current_votes = ai.current_votes + 1;
                Vector::push_back<address>(&mut ai.votes, sender_address);
                if (ai.current_votes < state.min_votes) {
                    // threshold of voters is not reached
                    return
                } else {
                    // reached threshold
                    ai.is_closed = true;
                    // clear votes
                    ai.current_votes = 0;
                    ai.votes = Vector::empty<address>();

                    // escrow has enough funds
                    assert(Diem::get_value(&state.tokens) >= balance, ERROR_INSUFFICIENT_BALANCE);

                    // withdraw tokens from escrow
                    let tokens = Diem::withdraw(&mut state.tokens, balance);

                    // move funds from escrow to user account
                    DiemAccount::deposit_tokens<GAS>(sender, escrow_address, receiver_this, tokens, x"", x"")
                }
            };

        }

        // Remove transfer account when transfer is completed
        // Removes entry in locked vector.
        // Executed under escrow account
        public fun delete_transfer_account(sender: &signer, escrow_address: address, transfer_id: &vector<u8>)
            acquires EscrowState {
            let sender_address= Signer::address_of(sender);
            assert(is_executor(&escrow_address, &sender_address), ERROR_MUST_BE_EXECUTOR);

            let idx_opt = find_locked_idx(escrow_address, transfer_id);
            assert(Option::is_some(&idx_opt), ERROR_INVALID_TRANSFER_ID);
            let idx = Option::borrow(&idx_opt);
            let state = borrow_global_mut<EscrowState>(escrow_address);

            // add voter
            let ai = Vector::borrow_mut<AccountInfo>(&mut state.locked, *idx);
            // transfer must not be closed
            assert(!ai.is_closed, ERROR_IS_CLOSED);
            // make sure this votes didn't vote before
            let vote_idx = find_address_idx(&sender_address, &ai.votes);
            assert(Option::is_none(&vote_idx), ERROR_ALREADY_VOTED);
            // update votes
            ai.current_votes = ai.current_votes + 1;
            Vector::push_back<address>(&mut ai.votes, sender_address);
            if (ai.current_votes < state.min_votes) {
                // threshold of voters is not reached
                return
            };

            Vector::remove<AccountInfo>(&mut state.locked, *idx);
        }

        // Remove unlocked vector entry to indiicate transfer completion
        // Executed under escrow account
        public fun delete_unlocked(sender: &signer, escrow_address: address, transfer_id: &vector<u8>)
        acquires EscrowState {
            let sender_address= Signer::address_of(sender);
            assert(is_executor(&escrow_address, &sender_address), ERROR_MUST_BE_EXECUTOR);

            let idx_opt = find_unlocked_idx(escrow_address, transfer_id);
            assert(Option::is_some(&idx_opt), ERROR_INVALID_TRANSFER_ID);
            let idx = Option::borrow(&idx_opt);
            let state = borrow_global_mut<EscrowState>(escrow_address);

            // add voter
            let ai = Vector::borrow_mut<AccountInfo>(&mut state.unlocked, *idx);
            assert(ai.is_closed, ERROR_UNLOCKED_MUST_BE_CLOSED);

            // make sure this votes didn't vote before
            let vote_idx = find_address_idx(&sender_address, &ai.votes);
            assert(Option::is_none(&vote_idx), ERROR_ALREADY_VOTED);
            // update votes
            ai.current_votes = ai.current_votes + 1;
            Vector::push_back<address>(&mut ai.votes, sender_address);
            if (ai.current_votes < state.min_votes) {
                // threshold of voters is not reached
                return
            };
            Vector::remove<AccountInfo>(&mut state.unlocked, *idx);
        }

        public fun find_locked_idx(escrow_address: address, transfer_id: &vector<u8>):
        Option<u64> acquires EscrowState {
            let state = borrow_global<EscrowState>(escrow_address);
            let i = 0;
            let n = Vector::length(&state.locked);
            while (i < n) {
                let ai = Vector::borrow(&state.locked, i);
                if (*&ai.transfer_id == *transfer_id) return Option::some(i);
                i = i + 1
            };
            Option::none()
        }

        public fun find_unlocked_idx(escrow_address: address,transfer_id: &vector<u8>):
            Option<u64> acquires EscrowState {
            let state = borrow_global<EscrowState>(escrow_address);
            let i = 0;
            let n = Vector::length(&state.unlocked);
            while (i < n) {
                let ai = Vector::borrow(&state.unlocked, i);
                if (*&ai.transfer_id == *transfer_id) return Option::some(i);
                i = i + 1
            };
            Option::none()
        }

        public fun get_locked_at(escrow_address: address, index: u64): AccountInfo acquires EscrowState  {
            assert(get_locked_length(escrow_address) > index, ERROR_LOCKED_EMPTY);
            let state = borrow_global<EscrowState>(escrow_address);
            let ai = Vector::borrow(&state.locked, index);
            *ai
        }

        public fun get_unlocked_at(escrow_address: address, index: u64): AccountInfo acquires EscrowState  {
            assert(get_unlocked_length(escrow_address) > index, ERROR_UNLOCKED_EMPTY);
            let state = borrow_global<EscrowState>(escrow_address);
            let ai = Vector::borrow(&state.unlocked, index);
            *ai
        }

        public fun get_escrow_balance(escrow: address): u64 acquires EscrowState {
            let state = borrow_global<EscrowState>(escrow);
            Diem::value(&state.tokens)
        }

        public fun get_locked_length(escrow: address): u64 acquires EscrowState {
            let state = borrow_global<EscrowState>(escrow);
            Vector::length(&state.locked)
        }

        public fun get_unlocked_length(escrow: address): u64 acquires EscrowState {
            let state = borrow_global<EscrowState>(escrow);
            Vector::length(&state.unlocked)
        }

        public fun get_sender_other(ai: &AccountInfo): vector<u8> {
            *&ai.sender_other
        }

        public fun get_receiver_other(ai: &AccountInfo): vector<u8> {
            *&ai.receiver_other
        }

        public fun get_balance(ai: &AccountInfo): u64 {
            *&ai.balance
        }

        public fun get_transfer_id(ai: &AccountInfo): vector<u8> {
            *&ai.transfer_id
        }

        public fun get_current_votes(ai: &AccountInfo): u64 {
            *&ai.current_votes
        }

        public fun get_votes(ai: &AccountInfo): vector<address> {
            *&ai.votes
        }

        public fun is_closed(ai: &AccountInfo): bool {
            *&ai.is_closed
        }

        public fun find_address_idx(target: &address, addresses: &vector<address>):
            Option<u64>  {
            let i = 0;
            let n = Vector::length(addresses);
            while (i < n) {
                let v = Vector::borrow(addresses, i);
                if (*v == *target) return Option::some(i);
                i = i + 1
            };
            Option::none()
        }

        fun is_executor(escrow: &address, candidate: &address): bool acquires EscrowState {
            let state = borrow_global<EscrowState>(*escrow);
            let idx = find_address_idx(candidate, &state.executors);
            Option::is_some(&idx)
        }
    }

}