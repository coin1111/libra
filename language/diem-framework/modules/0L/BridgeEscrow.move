
/////////////////////////////////////////////////////////////////////////
// 0L Module
// Escrow contract for bridge functionality
// 
/////////////////////////////////////////////////////////////////////////

address 0x1 {
    module BridgeEscrow {
        use 0x1::Signer;
        use 0x1::Diem;
        use 0x1::DiemAccount;
        use 0x1::DiemSystem;
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
        }
        // Creates en empty escrow account state
        public fun initialize_escrow(escrow: &signer) {
            let escrow_addr = Signer::address_of(escrow);
            assert(!exists<EscrowState>(escrow_addr), ERROR_BRIDGE_STORE_EXISTS);
            move_to<EscrowState>(escrow, EscrowState{
                locked: Vector::empty<AccountInfo>(),
                unlocked: Vector::empty<AccountInfo>(),
                tokens: Diem::zero<GAS>(),
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

        // Withdraw funds from escrow
        public fun withdraw_funds(sender: &signer,
                                        escrow_address: address,
                                        receiver_this:address, // receiver on this chain
                                        balance: u64
        ) acquires EscrowState  {
            let sender_address= Signer::address_of(sender);
            assert(DiemSystem::is_validator(sender_address) == true ||
                   sender_address == escrow_address , ERROR_MUST_BE_VALIDATOR);

            // update escrow state
            let state = borrow_global_mut<EscrowState>( escrow_address);

            // escrow has enough funds
            assert(Diem::get_value(&state.tokens) >= balance, ERROR_INSUFFICIENT_BALANCE);

            // withdraw tokens from escrow
            let tokens = Diem::withdraw(&mut state.tokens,balance);

            // move funds from escrow to user account
            DiemAccount::deposit_tokens<GAS>(sender, escrow_address, receiver_this, tokens, x"", x"");
        }


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
            assert(DiemSystem::is_validator(sender_address) == true ||
                   sender_address == escrow_address , ERROR_MUST_BE_VALIDATOR);

            // check that transfer id is not present
            let idx_opt = find_unlocked_idx( escrow_address, &transfer_id);
            assert(Option::is_none(&idx_opt), ERROR_TRANSFER_ID_EXISTS);

            // update escrow state
            let state = borrow_global_mut<EscrowState>( escrow_address);

            // escrow has enough funds
            assert(Diem::get_value(&state.tokens) >= balance, ERROR_INSUFFICIENT_BALANCE);

            // withdraw tokens from escrow
            let tokens = Diem::withdraw(&mut state.tokens,balance);

            // add entry to unlocked to indicate that funds were transferred
            let ai = AccountInfo {
                sender_this: ZERO_ADDRESS,
                sender_other: sender_other,
                receiver_this: copy receiver_this,
                receiver_other: Vector::empty<u8>(),
                balance: balance,
                transfer_id: transfer_id,
            };
            Vector::push_back<AccountInfo>(&mut state.unlocked, ai);

            // move funds from escrow to user account
            DiemAccount::deposit_tokens<GAS>(sender, escrow_address, receiver_this, tokens, x"", x"");
        }

        // Remove transfer account when transfer is completed
        // Removes entry in locked vector.
        // Executed under escrow account
        public fun delete_transfer_account(sender: &signer, escrow_address: address, transfer_id: &vector<u8>)
        acquires EscrowState {
            let sender_address= Signer::address_of(sender);
            assert(DiemSystem::is_validator(sender_address) == true ||
                   sender_address == escrow_address , ERROR_MUST_BE_VALIDATOR);

            let idx_opt = find_locked_idx(escrow_address, transfer_id);
            assert(Option::is_some(&idx_opt), ERROR_INVALID_TRANSFER_ID);
            let idx = Option::borrow(&idx_opt);
            let state = borrow_global_mut<EscrowState>(escrow_address);
            Vector::remove<AccountInfo>(&mut state.locked, *idx);
        }

        // Remove unlocked vector entry to indiicate transfer completion
        // Executed under escrow account
        public fun delete_unlocked(sender: &signer, escrow_address: address, transfer_id: &vector<u8>)
        acquires EscrowState {
            let sender_address= Signer::address_of(sender);
            assert(DiemSystem::is_validator(sender_address) == true ||
                   sender_address == escrow_address , ERROR_MUST_BE_VALIDATOR);

            let idx_opt = find_unlocked_idx(escrow_address, transfer_id);
            assert(Option::is_some(&idx_opt), ERROR_INVALID_TRANSFER_ID);
            let idx = Option::borrow(&idx_opt);
            let state = borrow_global_mut<EscrowState>(escrow_address);
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
    }

}