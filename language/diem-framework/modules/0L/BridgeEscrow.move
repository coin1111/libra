
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
        
        struct AccountState has key {
            // escrow address
            escrow: address,
            // address on this chain
            target_address: vector<u8>,
            // value sent
            balance: u64,
        }
    
        struct EscrowState has key {
            // current total balance of the bridge
            balance: u64,
        }

        // executed under escrow account
        public fun initialize_escrow(escrow: &signer) {
            assert(!exists<EscrowState>(Signer::address_of(escrow)), ERROR_BRIDGE_STORE_EXISTS);
            move_to<EscrowState>(escrow, EscrowState{
                balance: 0,
            });
        }

        // executed under user account
        public fun deposit_to_escrow(sender: &signer, escrow: address, amount: u64,
                                     target_address: vector<u8>) acquires EscrowState {
            // validate arguments
            assert (amount > 0, ERROR_AMOUNT_MUST_BE_POSITIVE);
            assert (Vector::length(&target_address) != 0, ERROR_TARGET_ADDRESS_EMPTY);

            // sender has enough funds
            let address = Signer::address_of(sender);
            assert(DiemAccount::balance<GAS>(address) >= amount, ERROR_INSUFFICIENT_BALANCE);

            // account doesn't exist
            // currently support only one transfer at a time
            assert (!exists<AccountState>(address), ERROR_ALREADY_ACCOUNT_EXISTS);

            // escrow account exists
            assert (exists<EscrowState>(escrow), ERROR_NO_ESCROW_ACCOUNT);

            // move funds to escrow account
            let with_cap = DiemAccount::extract_withdraw_capability(sender);
            DiemAccount::pay_from<GAS>(&with_cap, escrow, amount, x"", x"");
            DiemAccount::restore_withdraw_capability(with_cap);

            // record account balance
            move_to<AccountState>(sender, AccountState{ balance: amount,
                escrow: escrow, target_address: target_address });

            // update escrow balance
            let c_ref = &mut borrow_global_mut<EscrowState>(escrow).balance;
            *c_ref = *c_ref + amount;
        }

        // executed under escrow account
        public fun withdraw_from_escrow(escrow: &signer, target: address, amount: u64) acquires EscrowState  {
            // escrow has enough funds
            let escrow_address = Signer::address_of(escrow);
            assert(DiemAccount::balance<GAS>(escrow_address) >= amount, ERROR_INSUFFICIENT_BALANCE);

            // move funds from escrow to user account
            let with_cap = DiemAccount::extract_withdraw_capability(escrow);
            DiemAccount::pay_from<GAS>(&with_cap, target, amount, x"", x"");
            DiemAccount::restore_withdraw_capability(with_cap);

            // update balance
            let c_ref = &mut borrow_global_mut<EscrowState>(escrow_address).balance;
            assert(*c_ref >= amount, ERROR_INSUFFICIENT_BALANCE);
            *c_ref = *c_ref - amount;
        }

        // executed under escrow account
        public fun delete_account(escrow: &signer, target: address) acquires AccountState {
            // target account must exist
            assert (exists<AccountState>(target), ERROR_ACCOUNT_NOT_EXISTS);

            // destroy target account
            let target_account = move_from<AccountState>(target);
            // ensure that caller is escrow account
            assert(target_account.escrow == Signer::address_of(escrow), ERROR_NOT_ALLOWED);
            let AccountState { balance, escrow, target_address } = target_account;
            let _ = balance;
            let _ = escrow;
            let _ = target_address;
        }

        public fun get_target_address(account: address): vector<u8> acquires AccountState{
            let st = borrow_global<AccountState>(account);
            let tg = *&st.target_address;
            copy tg
        }
        public fun get_balance(account: address): u64 acquires AccountState {
            let st = borrow_global<AccountState>(account);
            st.balance
        }

        public fun get_escrow_balance(escrow: address): u64 acquires EscrowState {
            let st = borrow_global<EscrowState>(escrow);
            st.balance
        }

        public fun get_escrow(account: address): address acquires AccountState {
            let st = borrow_global<AccountState>(account);
            st.escrow
        }
        public fun has_escrow_balance(addr: address):  bool {
            let has = exists<AccountState>(addr);
            has
        }
    }

}