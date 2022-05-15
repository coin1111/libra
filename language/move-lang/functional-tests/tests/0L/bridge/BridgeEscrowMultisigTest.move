///// Setting up the test fixtures for the transactions below. 
///// The tags below create validators alice and bob, giving them 1000000 GAS coins.

//! account: alice, 1000000, 0
//! account: bob, 1, 0
//! account: escrow, 1, 0
//! account: carol, 1000000, 0, validator

///// Test 1: Init escrow account
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrowMultisig;
    use 0x1::Vector;

    fun main(sender: signer){
        let executors = Vector::empty<address>();
        Vector::push_back(&mut executors, @{{alice}});
        Vector::push_back(&mut executors, @{{bob}});
        Vector::push_back(&mut executors, @{{carol}});
        BridgeEscrowMultisig::initialize_escrow(&sender, executors, 2);
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
        let transfer_id: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let receiver_eth: vector<u8> = x"15d34aaf54267db7d7c367839aaf71a00a2c6a65";
        let amount: u64 = 100;
        BridgeEscrowMultisig::create_transfer_account(@{{escrow}}, &sender, receiver_eth, amount, transfer_id);
        assert(BridgeEscrowMultisig::get_escrow_balance(@{{escrow}}) == amount, 20001);
        assert(BridgeEscrowMultisig::get_locked_length(@{{escrow}}) == 1, 20002);
    }
}
//! check: EXECUTED

///// Test 3: Non-validator can't withdraw
//! new-transaction
//! sender: alice
//! gas-currency: GAS
script {
use 0x1::BridgeEscrowMultisig;
use 0x1::Option;

    fun main(sender: signer){
        let transfer_id: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let sender_eth: vector<u8> = x"90f79bf6eb2c4f870365e785982e1f101e93b906";
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrowMultisig::get_escrow_balance(escrow_address) == 100, 30001);

        // find account by transfer_id t and make transfer to the "other" chain
        // which is the same chain with account bob
        let index = BridgeEscrowMultisig::find_locked_idx(escrow_address, &transfer_id);
        assert(Option::is_some(&index),30002);
        let idx = Option::borrow(&index);
        assert(*idx == 0, 30003);

        let ai = BridgeEscrowMultisig::get_locked_at(escrow_address,*idx);
        BridgeEscrowMultisig::withdraw_from_escrow(&sender, escrow_address,
        sender_eth, // sender on eth chain
        @{{bob}}, // receiver
        BridgeEscrowMultisig::get_balance(&ai),
        BridgeEscrowMultisig::get_transfer_id(&ai),
        );
        assert(BridgeEscrowMultisig::get_escrow_balance(escrow_address) == 0, 30004);

        assert(BridgeEscrowMultisig::get_locked_length(escrow_address) == 1, 30005);
        assert(BridgeEscrowMultisig::get_unlocked_length(escrow_address) == 1, 30006);
    }
}
// check: ABORTED

///// Test 4: Assume that deposit was made on the other chain
// transfer funds into local bob account
//! new-transaction
//! sender: carol
//! gas-currency: GAS
script {
use 0x1::BridgeEscrowMultisig;
use 0x1::Option;

    fun main(sender: signer){
        let transfer_id: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let sender_eth: vector<u8> = x"90f79bf6eb2c4f870365e785982e1f101e93b906";
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrowMultisig::get_escrow_balance(escrow_address) == 100, 30001);

        // find account by transfer_id t and make transfer to the "other" chain
        // which is the same chain with account bob
        let index = BridgeEscrowMultisig::find_locked_idx(escrow_address, &transfer_id);
        assert(Option::is_some(&index),30002);
        let idx = Option::borrow(&index);
        assert(*idx == 0, 30003);

        let ai = BridgeEscrowMultisig::get_locked_at(escrow_address,*idx);
        BridgeEscrowMultisig::withdraw_from_escrow(&sender, escrow_address,
        sender_eth, // sender on eth chain
        @{{bob}}, // receiver
        BridgeEscrowMultisig::get_balance(&ai),
        BridgeEscrowMultisig::get_transfer_id(&ai),
        );
        assert(BridgeEscrowMultisig::get_escrow_balance(escrow_address) == 0, 30004);

        assert(BridgeEscrowMultisig::get_locked_length(escrow_address) == 1, 30005);
        assert(BridgeEscrowMultisig::get_unlocked_length(escrow_address) == 1, 30006);
    }
}
//! check: EXECUTED

///// Test 5: Delete alice escrow account
//! new-transaction
//! sender: carol
//! gas-currency: GAS
script {
use 0x1::BridgeEscrowMultisig;
use 0x1::Option;

    fun main(sender: signer){
        let transfer_id: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrowMultisig::get_locked_length(escrow_address) == 1, 40001);

        // find unlocked account using transfer_id and delete locked entry
        let index = BridgeEscrowMultisig::find_unlocked_idx(escrow_address, &transfer_id);
        assert(Option::is_some(&index),40002);
        let idx = Option::borrow(&index);
        assert(*idx == 0, 4003);

        BridgeEscrowMultisig::delete_transfer_account(&sender, escrow_address, &transfer_id);
        assert(BridgeEscrowMultisig::get_locked_length(escrow_address) == 0, 40004);
    }
}
//! check: EXECUTED

///// Test 6: Delete unlocked entry
//! new-transaction
//! sender: carol
//! gas-currency: GAS
script {
use 0x1::BridgeEscrowMultisig;
use 0x1::Option;

    fun main(sender: signer){
        let transfer_id: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrowMultisig::get_unlocked_length(escrow_address) == 1, 50001);

        // find unlocked account using transfer_id and delete locked entry
        let index = BridgeEscrowMultisig::find_unlocked_idx(escrow_address, &transfer_id);
        assert(Option::is_some(&index),50002);
        let idx = Option::borrow(&index);
        assert(*idx == 0, 5003);

        BridgeEscrowMultisig::delete_unlocked(&sender, escrow_address, &transfer_id);
        assert(BridgeEscrowMultisig::get_unlocked_length(escrow_address) == 0, 50004);
    }
}
//! check: EXECUTED

