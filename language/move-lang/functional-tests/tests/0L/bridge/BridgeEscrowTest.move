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
    use 0x1::Signer;

    fun main(sender: signer){
        let target_address: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let amount: u64 = 100;
        BridgeEscrow::deposit_to_escrow(&sender, @{{escrow}}, amount, target_address);
        assert(BridgeEscrow::get_balance(Signer::address_of(&sender)) == amount, 1001);
    }
}
//! check: EXECUTED

///// Test 3: Delete alice escrow account
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;

    fun main(escrow: signer){
        assert(BridgeEscrow::has_escrow_balance(@{{alice}}), 1002);
        BridgeEscrow::delete_account(&escrow,@{{alice}});
        assert(!BridgeEscrow::has_escrow_balance(@{{alice}}), 1003);
    }
}
//! check: EXECUTED

///// Test 4: Transfer to bob's account
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;
    use 0x1::Signer;
    use 0x1::Debug;

    fun main(escrow: signer){
        let bal = BridgeEscrow::get_escrow_balance(Signer::address_of(&escrow));
        Debug::print(&bal);
        assert(BridgeEscrow::get_escrow_balance(Signer::address_of(&escrow)) == 100, 1004);
        BridgeEscrow::withdraw_from_escrow(&escrow,@{{bob}},50);
        assert(BridgeEscrow::get_escrow_balance(Signer::address_of(&escrow)) == 50, 1005);
    }
}
//! check: EXECUTED


