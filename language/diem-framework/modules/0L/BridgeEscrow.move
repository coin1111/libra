
/////////////////////////////////////////////////////////////////////////
// 0L Module
// Escrow contract for bridge functionality
// 
/////////////////////////////////////////////////////////////////////////

address 0x1 {
    module BridgeEscrow {
        use 0x1::Signer;
        use 0x1::CoreAddresses;
        use 0x1::Diem;
        use 0x1::DiemAccount;
        use 0x1::GAS::GAS;
        use 0x1::Vector;

        const ERROR_BRIDGE_STORE_EXISTS:u64 = 3001;
        const ERROR_ALREADY_ACCOUNT_EXISTS: u64 = 1;
        const ERROR_TARGET_ADDRESS_EMPTY: u64 = 2;
        const ERROR_AMOUNT_MUST_BE_POSITIVE: u64 = 3;
        const ERROR_INSUFFICIENT_BALANCE: u64 = 4;
        const ERROR_ACCOUNT_NOT_EXISTS: u64 = 5;
        const ERROR_NO_ESCROW_ACCOUNT: u64 = 6;
        
        struct AccountState has store {
            addr: address,
            // address on this chain
            target_addr: vector<u8>,
            // address on the other chain
            value: u64,
        }
    
        struct BridgeState has key {
            accounts: vector<AccountState>,
            balance: Diem::Diem<GAS>,
        }
    
        public fun init_escrow_state(vm: &signer) {
            CoreAddresses::assert_diem_root(vm);
            assert(!exists<BridgeState>(Signer::address_of(vm)), ERROR_BRIDGE_STORE_EXISTS);
            move_to<BridgeState>(vm, BridgeState{
                accounts: Vector::empty<AccountState>(),
                balance: Diem::zero<GAS>()
            });
        }
    
        public fun deposit_to_escrow(sender: &signer, target_address: vector<u8>,
                                      amount: u64) acquires BridgeState {
            let state = borrow_global_mut<BridgeState>(CoreAddresses::DIEM_ROOT_ADDRESS());
            let coin = DiemAccount::withdraw_from_sender(amount);
            Vector::push_back<AccountState>(&mut state.accounts, AccountState{
                addr: Signer::address_of(sender),
                target_addr: target_address,
                value: Diem::value<GAS>(&coin),
            });
            Diem::deposit(&mut state.balance, coin);
        }
    }
}