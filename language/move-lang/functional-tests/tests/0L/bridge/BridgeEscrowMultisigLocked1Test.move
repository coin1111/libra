///// Setting up the test fixtures for the transactions below. 
///// The tags below create validators alice and bob, giving them 1000000 GAS coins.
/// Multisig test with min_votes = 1

//! account: alice, 1000000, 0
//! account: bob, 1, 0
//! account: escrow, 1, 0
//! account: carol, 1000000, 0, validator
//! account: dave, 1000000, 0, validator
//! account: eve, 1000000, 0, validator

///// Test 1: Init escrow account
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrowMultisig;
    use 0x1::Vector;

    fun main(sender: signer){
        let executors = Vector::empty<address>();
        Vector::push_back(&mut executors, @{{carol}});
        Vector::push_back(&mut executors, @{{dave}});
        Vector::push_back(&mut executors, @{{eve}});
        BridgeEscrowMultisig::initialize_escrow(&sender, executors, 1);
    }
}
//! check: EXECUTED

///// Test 2: Alice deposit funds into escrow
//! new-transaction
//! sender: alice
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrowMultisig;

    fun main(sender: signer){
        let transfer_id: vector<u8> = x"11192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let receiver_eth: vector<u8> = x"15d34aaf54267db7d7c367839aaf71a00a2c6a65";
        let amount: u64 = 100;
        BridgeEscrowMultisig::create_transfer_account(@{{escrow}}, &sender, receiver_eth, amount, transfer_id);
        assert(BridgeEscrowMultisig::get_escrow_balance(@{{escrow}}) == amount, 20001);
        assert(BridgeEscrowMultisig::get_locked_length(@{{escrow}}) == 1, 20002);
    }
}
//! check: EXECUTED

///// Test 4: Delete alice escrow account
//! new-transaction
//! sender: carol
//! gas-currency: GAS
script {
use 0x1::BridgeEscrowMultisig;

    fun main(sender: signer){
        let transfer_id: vector<u8> = x"11192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrowMultisig::get_locked_length(escrow_address) == 1, 40001);

        BridgeEscrowMultisig::delete_transfer_account(&sender, escrow_address, &transfer_id);
        // record is not removed
        assert(BridgeEscrowMultisig::get_locked_length(escrow_address) == 0, 40004);
    }
}
//! check: EXECUTED

///// Test 7: Delete alice escrow account, cannot delete deleted
//! new-transaction
//! sender: eve
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrowMultisig;

    fun main(sender: signer){
        let transfer_id: vector<u8> = x"11192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrowMultisig::get_locked_length(escrow_address) == 0, 40001);

        BridgeEscrowMultisig::delete_transfer_account(&sender, escrow_address, &transfer_id);
    }
}
// check: VMExecutionFailure(ABORTED

