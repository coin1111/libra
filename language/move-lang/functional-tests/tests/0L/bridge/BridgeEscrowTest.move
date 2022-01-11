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
        BridgeEscrow::deposit_to_escrow(&sender, @{{escrow}}, @{{bob}}, amount);
        assert(BridgeEscrow::get_escrow_balance(@{{escrow}}) == amount, 1001);
        assert(BridgeEscrow::get_locked_length(@{{escrow}}) == 1, 1002);
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
        assert(BridgeEscrow::get_escrow_balance(Signer::address_of(&escrow)) == 100, 1004);

        // pick up the first available account and make transfer to the "other" chain
        // which is the same chain with account bob
        let (sender, receiver, balance) = BridgeEscrow::get_locked_at(0, @{{escrow}});

        BridgeEscrow::withdraw_from_escrow(&escrow, sender, receiver, balance);
        assert(BridgeEscrow::get_escrow_balance(Signer::address_of(&escrow)) == 0, 1005);
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
        //assert(BridgeEscrow::has_escrow_balance(@{{alice}}), 1002);
        BridgeEscrow::delete_account(&escrow, @{{alice}},@{{bob}});
       // assert(!BridgeEscrow::has_escrow_balance(@{{alice}}), 1003);
    }
}
//! check: EXECUTED

