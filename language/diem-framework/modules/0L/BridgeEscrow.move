
/////////////////////////////////////////////////////////////////////////
// 0L Module
// Escrow contract for bridge functionality
// 
/////////////////////////////////////////////////////////////////////////

address 0x1 {
    module BridgeEscrow {
        use 0x1::Signer;
        use 0x1::DiemAccount;
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

        struct AccountInfo has copy, store, drop {
            // user address on this chain
            sender: address,
            // user address on the other chain
            receiver: address,
            // value sent
            balance: u64,
            // transfer id
           transfer_id: vector<u8>,
        }
    
        struct EscrowState has key {
            locked: vector<AccountInfo>,
            unlocked: vector<AccountInfo>,
            // current total balance of the bridge
            balance: u64,
        }

        public fun initialize_escrow(escrow: &signer) {
            let escrow_addr = Signer::address_of(escrow);
            assert(!exists<EscrowState>(escrow_addr), ERROR_BRIDGE_STORE_EXISTS);
            move_to<EscrowState>(escrow, EscrowState{
                locked: Vector::empty<AccountInfo>(),
                unlocked: Vector::empty<AccountInfo>(),
                balance: 0,
            });
        }

        // executed under user account
        public fun create_transfer_account(escrow: address,
                                           sender: &signer,
                                           receiver: address,
                                           amount: u64,
                                           transfer_id: vector<u8>) acquires EscrowState {
            // validate arguments
            assert (amount > 0, ERROR_AMOUNT_MUST_BE_POSITIVE);

            // sender has enough funds
            let address = Signer::address_of(sender);
            assert(DiemAccount::balance<GAS>(address) >= amount, ERROR_INSUFFICIENT_BALANCE);

            // escrow account exists
            assert (exists<EscrowState>(escrow), ERROR_NO_ESCROW_ACCOUNT);

            // move funds to escrow account
            let with_cap = DiemAccount::extract_withdraw_capability(sender);
            DiemAccount::pay_from<GAS>(&with_cap, escrow, amount, x"", x"");
            DiemAccount::restore_withdraw_capability(with_cap);

            // record account balance

            // update escrow balance
            let state = borrow_global_mut<EscrowState>(escrow);
            *&mut state.balance = *&mut state.balance + amount;

            Vector::push_back<AccountInfo>(&mut state.locked, AccountInfo{
                sender: Signer::address_of(sender),
                receiver: receiver,
                balance: amount,
                transfer_id: transfer_id,
            });
        }
        // executed under escrow account
        public fun withdraw_from_escrow(escrow: &signer, transfer_id:&vector<u8>) acquires EscrowState  {
            let escrow_address = Signer::address_of(escrow);

            let idx_opt = find_locked_idx(escrow_address,transfer_id);
            assert(Option::is_some(&idx_opt), ERROR_INVALID_TRANSFER_ID);
            let idx = Option::borrow(&idx_opt);

            let ai = get_locked_at(escrow_address, *idx);

            // escrow has enough funds
            assert(DiemAccount::balance<GAS>(escrow_address) >= ai.balance, ERROR_INSUFFICIENT_BALANCE);


            // move funds from escrow to user account
            let with_cap = DiemAccount::extract_withdraw_capability(escrow);
            DiemAccount::pay_from<GAS>(&with_cap, ai.receiver, ai.balance, x"", x"");
            DiemAccount::restore_withdraw_capability(with_cap);

            // update balance
            let state = borrow_global_mut<EscrowState>(escrow_address);
            assert(state.balance >= ai.balance, ERROR_INSUFFICIENT_BALANCE);
            *&mut state.balance = *&mut state.balance - ai.balance;

            // add entry to unlocked to indicate that funds were transferred
            Vector::push_back<AccountInfo>(&mut state.unlocked, ai);
        }

        // executed under escrow account
        public fun delete_transfer_account(escrow: &signer, transfer_id: &vector<u8>)
        acquires EscrowState {
            let escrow_address = Signer::address_of(escrow);
            let idx_opt = find_locked_idx(escrow_address, transfer_id);
            assert(Option::is_some(&idx_opt), ERROR_INVALID_TRANSFER_ID);
            let idx = Option::borrow(&idx_opt);
            let state = borrow_global_mut<EscrowState>(escrow_address);
            Vector::remove<AccountInfo>(&mut state.locked, *idx);
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
            let info = Vector::borrow(&state.locked, index);
            *info
        }
        public fun get_unlocked_at(escrow_address: address, index: u64): AccountInfo acquires EscrowState  {
            assert(get_unlocked_length(escrow_address) > index, ERROR_LOCKED_EMPTY);
            let state = borrow_global<EscrowState>(escrow_address);
            let info = Vector::borrow(&state.unlocked, index);
            *info
        }

        public fun get_escrow_balance(escrow: address): u64 acquires EscrowState {
            let state = borrow_global<EscrowState>(escrow);
            state.balance
        }

        public fun get_locked_length(escrow: address): u64 acquires EscrowState {
            let state = borrow_global<EscrowState>(escrow);
            Vector::length(&state.locked)
        }

        public fun get_unlocked_length(escrow: address): u64 acquires EscrowState {
            let state = borrow_global<EscrowState>(escrow);
            Vector::length(&state.unlocked)
        }
        public fun get_sender_from_ai(ai: &AccountInfo): address {
            *&ai.sender
        }

        public fun get_receiver_from_ai(ai: &AccountInfo): address {
            *&ai.receiver
        }

        public fun get_balance_from_ai(ai: &AccountInfo): u64 {
            *&ai.balance
        }

        public fun get_transfer_id_from_ai(ai: &AccountInfo): vector<u8> {
            *&ai.transfer_id
        }
    }

}