///// Setting up the test fixtures for the transactions below. 
///// The tags below create validators alice and bob, giving them 1000000 GAS coins.

//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator
//! account: carol, 1000000, 0


///// Test 1: Can initializa Alice's account,

//! new-transaction
//! sender: alice
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;
    use 0x1::Vector;

    fun main(alice: signer){
        BridgeEscrow::initialize(&alice);
        let target_address = BridgeEscrow::get_target_address(&alice);
        assert(Vector::length(&target_address) == 0,0);
        let balance = BridgeEscrow::get_balance(&alice);
        assert(balance == 0,1);
    }
}
//! check: EXECUTED
