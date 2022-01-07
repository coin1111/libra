///// Setting up the test fixtures for the transactions below. 
///// The tags below create validators alice and bob, giving them 1000000 GAS coins.

//! account: alice, 1000000, 0
//! account: bob, 0, 0
//! account: escrow, 0, 0
//! account: carol, 1000000, 0, validator

///// Test 1: Init escrow account
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;

    fun main(sender: signer){
        BridgeEscrow::init(&sender);
    }
}
//! check: EXECUTED

///// Test 2: Can deposit funds to Alice account
//! new-transaction
//! sender: alice
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;
    use 0x1::DiemAccount;
    use 0x1::Signer;
    use 0x1::GAS::GAS;

    fun main(sender: signer){
        let amount: u64 = 100;
        let target_address: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let bal_before = DiemAccount::balance<GAS>(Signer::address_of(&sender));
        BridgeEscrow::deposit_to_escrow(@{{escrow}}, &sender, amount, target_address);
        let bal_after = DiemAccount::balance<GAS>(Signer::address_of(&sender));
        let escrow_balance = BridgeEscrow::get_balance(&sender);
        assert(escrow_balance == amount,1001);
        assert(DiemAccount::balance<GAS>(@{{escrow}}) == amount, 1002);
        assert (bal_after + amount == bal_before, 1003);
    }
}
//! check: EXECUTED

///// Test 3: Can withdraw funds to Bob's account
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;
    use 0x1::DiemAccount;
    use 0x1::GAS::GAS;

    fun main(sender: signer){
        let amount: u64 = 100;
        let escrow_addr = @{{escrow}};
        let bob_addr = @{{bob}};
        let bal_before = DiemAccount::balance<GAS>(escrow_addr);
        BridgeEscrow::withdraw_from_escrow(&sender, bob_addr, amount);
        let bal_after = DiemAccount::balance<GAS>(escrow_addr);
        assert(DiemAccount::balance<GAS>(bob_addr) == amount, 1004);
        assert (bal_after + amount == bal_before, 1005);
    }
}
//! check: EXECUTED

///// Test 4: Delete Alice's escrow account
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;

    fun main(_sender: signer){
        let alice_addr = @{{alice}};
        BridgeEscrow::delete_escrow_account(alice_addr);
        assert(!BridgeEscrow::has_escrow(alice_addr),1006);
    }
}
//! check: EXECUTED

