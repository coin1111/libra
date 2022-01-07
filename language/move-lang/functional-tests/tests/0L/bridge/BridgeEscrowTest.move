///// Setting up the test fixtures for the transactions below. 
///// The tags below create validators alice and bob, giving them 1000000 GAS coins.

//! account: alice, 1000000, 0
//! account: bob, 1000000, 0
//! account: carol, 1000000, 0, validator

///// Test 1: Can deposit funds to Alice account
//! new-transaction
//! sender: alice
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;

    fun main(alice: signer){
        let amount: u64 = 100;
        let target_address: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        BridgeEscrow::deposit(&alice, amount, target_address);
        let balance = BridgeEscrow::get_balance(&alice);
        assert(balance == 100,1);
    }
}
//! check: EXECUTED

///// Test 2: Cannot deposit funds to account twice
//! new-transaction
//! sender: alice
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;

    fun main(alice: signer){
        let amount: u64 = 100;
        let target_address: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        BridgeEscrow::deposit(&alice, amount, copy target_address);
        BridgeEscrow::deposit(&alice, amount, target_address);
    }
}
// check: "ABORTED { code: 1,"
