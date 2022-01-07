///// Setting up the test fixtures for the transactions below. 
///// The tags below create validators alice and bob, giving them 1000000 GAS coins.

//! account: alice, 1000000, 0
//! account: bob, 0, 0
//! account: carol, 1000000, 0, validator
//! account: escrow, 0, 0

///// Test 1: Can deposit funds to Alice account
//! new-transaction
//! sender: alice
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;
    use 0x1::DiemAccount;
    use 0x1::GAS::GAS;

    fun main(sender: signer){
        let amount: u64 = 100;
        let target_address: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        BridgeEscrow::deposit(@{{escrow}}, &sender, amount, target_address);
        let balance = BridgeEscrow::get_balance(&sender);
        assert(balance == amount,1);
        assert(DiemAccount::balance<GAS>(@{{escrow}}) == amount, 2);
    }
}
//! check: EXECUTED

///// Test 2: Escrow can withdraw
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;
    use 0x1::DiemAccount;
    use 0x1::GAS::GAS;

    fun main(sender: signer){
        let initial = DiemAccount::balance<GAS>(@{{alice}});
        let amount: u64 = 100;
        BridgeEscrow::withdraw(&sender, @{{alice}});
        assert(DiemAccount::balance<GAS>(@{{alice}}) == initial+amount, 2);
    }
}
//! check: EXECUTED
//
/////// Test 2: Cannot deposit funds to account twice
////! new-transaction
////! sender: alice
////! gas-currency: GAS
//script {
//    use 0x1::BridgeEscrow;
//
//    fun main(alice: signer){
//        let amount: u64 = 100;
//        // account is created in Test1
//        let target_address: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
//        BridgeEscrow::deposit(&alice, amount, target_address);
//    }
//}
//// check: "ABORTED { code: 1,"
//
/////// Test 3: Cannot deposit 0 amount
////! new-transaction
////! sender: alice
////! gas-currency: GAS
//script {
//    use 0x1::BridgeEscrow;
//
//    fun main(alice: signer){
//        let amount: u64 = 0;
//        let target_address: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
//        BridgeEscrow::deposit(&alice, amount, copy target_address);
//    }
//}
//// check: "ABORTED { code: 3,"
