
/////////////////////////////////////////////////////////////////////////
// 0L Module
// Escrow contract for bridge functionality
// 
/////////////////////////////////////////////////////////////////////////

address 0x1{
    module BridgeEscrow {
        use 0x1::Signer;
        use 0x1::Vector;
        
        const ERROR_ALREADY_ACCOUNT_EXISTS: u64 = 1;
        const ERROR_TARGET_ADDRESS_EMPTY: u64 = 2;
        const ERROR_AMOUNT_MUST_BE_POSITIVE: u64 = 3;
        
        // Individual account containing
        // escrowed balance and target destination on the other chain
        struct AccountState has key {
          balance: u64, // amount escrowed 
          target_address: vector<u8>, // address on the other chain
        }

        public fun deposit(sender: &signer, amount: u64, target_address: vector<u8>) {
            let address = Signer::address_of(sender);
            assert (!exists<AccountState>(address), ERROR_ALREADY_ACCOUNT_EXISTS);
            assert (amount > 0, ERROR_AMOUNT_MUST_BE_POSITIVE);
            assert (Vector::length(&target_address) != 0, ERROR_TARGET_ADDRESS_EMPTY);
            move_to<AccountState>(sender, AccountState{ balance: amount, target_address: target_address });
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
    }

}