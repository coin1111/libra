///// Setting up the test fixtures for the transactions below. 
///// The tags below create validators alice and bob, giving them 1000000 GAS coins.

//! account: alice, 1000000, 0
//! account: bob, 0, 0
//! account: escrow, 1, 0

///// Test 1: Init escrow account
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;

    fun main(sender: signer){
        BridgeEscrow::initialize_escrow(&sender);
    }
}
//! check: EXECUTED

///// Test 2: Alice deposit funds into escrow
//! new-transaction
//! sender: alice
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;

    fun main(sender: signer){
        //let target_address: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let amount: u64 = 100;
        BridgeEscrow::create_transfer_account(&sender, @{{escrow}}, @{{bob}}, amount);
        assert(BridgeEscrow::get_escrow_balance(@{{escrow}}) == amount, 20001);
        assert(BridgeEscrow::get_locked_length(@{{escrow}}) == 1, 20002);
    }
}
//! check: EXECUTED

///// Test 3: Bridge agent detects that Alice deposited funds and
/// create complimentary transaction on the other chain
/// in this case the other chain is the same.
/// Complimentary account is bob
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;
    use 0x1::Signer;

    fun main(escrow: signer){
        assert(BridgeEscrow::get_escrow_balance(Signer::address_of(&escrow)) == 100, 30001);

        // pick up the first available account and make transfer to the "other" chain
        // which is the same chain with account bob
        let (sender, receiver, balance) = BridgeEscrow::get_locked_at(0, @{{escrow}});

        BridgeEscrow::withdraw_from_escrow(&escrow, sender, receiver, balance);
        assert(BridgeEscrow::get_escrow_balance(Signer::address_of(&escrow)) == 0, 30003);

        assert(BridgeEscrow::get_locked_length(@{{escrow}}) == 1, 30003);
        assert(BridgeEscrow::get_unlocked_length(@{{escrow}}) == 1, 30004);
    }
}
//! check: EXECUTED

///// Test 4: Delete alice escrow account
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;

    fun main(escrow: signer){
        assert(BridgeEscrow::get_locked_length(@{{escrow}}) == 1, 40001);

        // pick up the first unlocked account and delete locked entry
        let (sender, receiver, _) = BridgeEscrow::get_unlocked_at(0, @{{escrow}});
        BridgeEscrow::delete_transfer_account(&escrow, sender, receiver);
        assert(BridgeEscrow::get_locked_length(@{{escrow}}) == 0, 40002);
    }
}
//! check: EXECUTED

