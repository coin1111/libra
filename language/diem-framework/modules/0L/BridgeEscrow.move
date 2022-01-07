
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
        const ERROR_NO_ESCROW_ACCOUNT: u64 = 6;

        // Individual account containing
        // escrowed balance and target destination on the other chain
        struct AccountState has key {
            balance: u64, // amount escrowed
            escrow: address,
            target_address: vector<u8>, // address on the other chain
        }

        // existence of this resource marks an account to become escrow
        struct EscrowAccount has key {
            enabled: bool,
        }

        public fun init(escrow: &signer) {
            assert (!exists<EscrowAccount>(Signer::address_of(escrow)), ERROR_ALREADY_ACCOUNT_EXISTS);
            move_to<EscrowAccount>(escrow, EscrowAccount{enabled:true});
        }

        public fun deposit_to_escrow(escrow: address, sender: &signer, amount: u64, target_address: vector<u8>) {
            // validate arguments
            assert (amount > 0, ERROR_AMOUNT_MUST_BE_POSITIVE);
            assert (Vector::length(&target_address) != 0, ERROR_TARGET_ADDRESS_EMPTY);

            // sender has enough funds
            let address = Signer::address_of(sender);
            assert(DiemAccount::balance<GAS>(address) >= amount, ERROR_INSUFFICIENT_BALANCE);

            // account doesn't exist
            assert (!exists<AccountState>(address), ERROR_ALREADY_ACCOUNT_EXISTS);

            // escrow account exists
            assert (exists<EscrowAccount>(escrow), ERROR_NO_ESCROW_ACCOUNT);

            // move funds to escrow account
            let with_cap = DiemAccount::extract_withdraw_capability(sender);
            DiemAccount::pay_from<GAS>(&with_cap, escrow, amount, x"", x"");
            DiemAccount::restore_withdraw_capability(with_cap);

            // record account balance
            move_to<AccountState>(sender, AccountState{ balance: amount,
                escrow: escrow, target_address: target_address });
        }

        public fun withdraw_from_escrow(escrow: &signer, target: address, amount: u64) {
            // escrow has enough funds
            let escrow_address = Signer::address_of(escrow);
            assert(DiemAccount::balance<GAS>(escrow_address) >= amount, ERROR_INSUFFICIENT_BALANCE);

            // move funds from escrow to user account
            let with_cap = DiemAccount::extract_withdraw_capability(escrow);
            DiemAccount::pay_from<GAS>(&with_cap, target, amount, x"", x"");
            DiemAccount::restore_withdraw_capability(with_cap);
        }

        public fun delete_escrow_account(target: address) acquires AccountState {
            // target account must exist
            assert (exists<AccountState>(target), ERROR_ACCOUNT_NOT_EXISTS);

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
        public fun has_escrow(addr: address):  bool {
            let has = exists<AccountState>(addr);
            has
        }
    }

}