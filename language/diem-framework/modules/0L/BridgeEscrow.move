
/////////////////////////////////////////////////////////////////////////
// 0L Module
// Escrow contract for bridge functionality
// 
/////////////////////////////////////////////////////////////////////////

address 0x1{
    module BridgeEscrow {
        use 0x1::Signer;
        use 0x1::DiemAccount;
        use 0x1::GAS::GAS;
        use 0x1::Vector;

        const ERROR_ALREADY_ACCOUNT_EXISTS: u64 = 1;
        const ERROR_TARGET_ADDRESS_EMPTY: u64 = 2;
        const ERROR_AMOUNT_MUST_BE_POSITIVE: u64 = 3;
        const ERROR_INSUFFICIENT_BALANCE: u64 = 4;
        const ERROR_ACCOUNT_NOT_EXISTS: u64 = 5;

        // Individual account containing
        // escrowed balance and target destination on the other chain
        struct AccountState has key {
            balance: u64, // amount escrowed
            escrow: address,
            target_address: vector<u8>, // address on the other chain
        }

        public fun deposit(escrow: address, sender: &signer, amount: u64, target_address: vector<u8>) {
            // validate arguments
            assert (amount > 0, ERROR_AMOUNT_MUST_BE_POSITIVE);
            assert (Vector::length(&target_address) != 0, ERROR_TARGET_ADDRESS_EMPTY);

            // sender has enough funds
            let address = Signer::address_of(sender);
            assert(DiemAccount::balance<GAS>(address) >= amount, ERROR_INSUFFICIENT_BALANCE);

            assert (!exists<AccountState>(address), ERROR_ALREADY_ACCOUNT_EXISTS);

            // move funds to escrow account
            let with_cap = DiemAccount::extract_withdraw_capability(sender);
            DiemAccount::pay_from<GAS>(&with_cap, escrow, amount, x"", x"");
            DiemAccount::restore_withdraw_capability(with_cap);

            // record account balance
            move_to<AccountState>(sender, AccountState{ balance: amount,
                escrow: escrow, target_address: target_address });
        }

        public fun withdraw(escrow: &signer, target: address) acquires AccountState {
            // target account must exist
            assert (exists<AccountState>(target), ERROR_ACCOUNT_NOT_EXISTS);
            // balance must be non-empty
            let state = borrow_global<AccountState>(target);
            let balance = state.balance;
            assert(balance > 0, ERROR_INSUFFICIENT_BALANCE);

            // escrow has enough funds
            let escrow_address = Signer::address_of(escrow);
            assert(DiemAccount::balance<GAS>(escrow_address) >= balance, ERROR_INSUFFICIENT_BALANCE);

            // move funds from escrow to user account
            let with_cap = DiemAccount::extract_withdraw_capability(escrow);
            DiemAccount::pay_from<GAS>(&with_cap, target, balance, x"", x"");
            DiemAccount::restore_withdraw_capability(with_cap);

            // destroy target account
            let target_account = move_from<AccountState>(target);
            let AccountState { balance, escrow, target_address } = target_account;
            let _ = balance;
            let _ = escrow;
            let _ = target_address;
        }


        public fun get_target_address(sender: &signer): vector<u8> acquires AccountState{
          let st = borrow_global<AccountState>(Signer::address_of(sender));
          let tg = *&st.target_address;
          copy tg
        }
        public fun get_balance(sender: &signer): u64 acquires AccountState {
          let st = borrow_global<AccountState>(Signer::address_of(sender));
          st.balance
        }

        public fun get_escrow(sender: &signer): address acquires AccountState {
            let st = borrow_global<AccountState>(Signer::address_of(sender));
            st.escrow
        }
    }

}