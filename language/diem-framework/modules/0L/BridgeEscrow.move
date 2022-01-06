
/////////////////////////////////////////////////////////////////////////
// 0L Module
// Escrow contract for bridge functionality
// 
/////////////////////////////////////////////////////////////////////////

address 0x1{
    module BridgeEscrow {
        use 0x1::Vector;
        use 0x1::Signer;
        
        // Individual account containing
        // escrowed coins and target destination on the other chain
        struct AccountState has key {
          funds: u64, // amount escrowed 
          target_address: vector<u8>, // address on the other chain
        }

        // Initialize AccountState
        public fun initialize(sender: &signer){
          move_to<AccountState>(sender, AccountState{ funds: 0, target_address: Vector::empty() });
        }

        // Validate initialization
        spec initialize {
            let addr = Signer::address_of(sender);
            ensures Vector::length(global<AccountState>(addr).target_address) == 0;
        }

        public fun get_target_address(sender: &signer): vector<u8> acquires AccountState{
          let st = borrow_global<AccountState>(Signer::address_of(sender));
          let tg = *&st.target_address;
          copy tg
        }
    }

}