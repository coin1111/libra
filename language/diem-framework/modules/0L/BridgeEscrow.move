
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

        const ERROR_BRIDGE_STORE_EXISTS:u64 = 3000;
        const ERROR_ALREADY_ACCOUNT_EXISTS: u64 = 3001;
        const ERROR_TARGET_ADDRESS_EMPTY: u64 = 3002;
        const ERROR_AMOUNT_MUST_BE_POSITIVE: u64 = 3003;
        const ERROR_INSUFFICIENT_BALANCE: u64 = 3004;
        const ERROR_ACCOUNT_NOT_EXISTS: u64 = 3005;
        const ERROR_NO_ESCROW_ACCOUNT: u64 = 3006;
        const ERROR_NOT_ALLOWED: u64 = 3007;

        struct AccountInfo has store, drop {
            // user address on this chain
            address: address,
            // user address on the other chain
            target_address: address,
            // value sent
            balance: u64,
        }
    
        struct EscrowState has key {
            locked: vector<AccountInfo>,
            unlocked: vector<AccountInfo>,
            // current total balance of the bridge
            balance: u64,
        }

        // executed under escrow account
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
        public fun deposit_to_escrow(sender: &signer, escrow: address, amount: u64,
                                     target_address: address) acquires EscrowState {
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
                address: Signer::address_of(sender),
                target_address: target_address,
                balance: amount,
            });
        }

        // executed under escrow account
        public fun withdraw_from_escrow(escrow: &signer, receiver: address, amount: u64, sender: address) acquires EscrowState  {
            // escrow has enough funds
            let escrow_address = Signer::address_of(escrow);
            assert(DiemAccount::balance<GAS>(escrow_address) >= amount, ERROR_INSUFFICIENT_BALANCE);

            // move funds from escrow to user account
            let with_cap = DiemAccount::extract_withdraw_capability(escrow);
            DiemAccount::pay_from<GAS>(&with_cap, receiver, amount, x"", x"");
            DiemAccount::restore_withdraw_capability(with_cap);

            // update balance
            let state = borrow_global_mut<EscrowState>(escrow_address);
            assert(state.balance >= amount, ERROR_INSUFFICIENT_BALANCE);
            *&mut state.balance = *&mut state.balance - amount;

            // add entry to unlocked to indicate that funds were transferred
            Vector::push_back<AccountInfo>(&mut state.unlocked, AccountInfo{
                address: receiver,
                target_address: sender,
                balance: amount,
            });
        }

        // executed under escrow account
        public fun delete_account(escrow: &signer, receiver: address, sender: address)
            acquires EscrowState {

            let escrow_address = Signer::address_of(escrow);
            let state = borrow_global_mut<EscrowState>(escrow_address);
            let ai: AccountInfo = AccountInfo{
                address: receiver,
                // user address on the other chain
                target_address: sender,
                // value sent
                balance: 0,
            };
            let (_, i) = Vector::index_of<AccountInfo>(&state.unlocked, &ai);
            Vector::remove<AccountInfo>(&mut state.unlocked, i);
        }

//        public fun get_balance(account: address): u64 acquires AccountState {
//            let st = borrow_global<AccountState>(account);
//            st.balance
//        }

        public fun get_escrow_balance(escrow: address): u64 acquires EscrowState {
            let st = borrow_global<EscrowState>(escrow);
            st.balance
        }

//        public fun get_escrow(account: address): address acquires AccountState {
//            let st = borrow_global<AccountState>(account);
//            st.escrow
//        }
//        public fun has_escrow_balance(addr: address):  bool {
//            let has = exists<AccountState>(addr);
//            has
//        }
    }

}