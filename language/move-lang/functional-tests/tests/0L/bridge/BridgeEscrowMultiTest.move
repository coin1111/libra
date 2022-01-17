///// Setting up the test fixtures for the transactions below. 
///// The tags below create validators alice and bob, giving them 1000000 GAS coins.

//! account: alice, 100, 0
//! account: bob, 10, 0
//! account: carol, 50, 0
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

///// Test 2: Alice deposit funds into escrow for bob
//! new-transaction
//! sender: alice
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;

    fun main(sender: signer){
        let transfer_id1: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let amount: u64 = 100;
        BridgeEscrow::create_transfer_account(@{{escrow}}, &sender, @{{bob}}, amount, transfer_id1);
        assert(BridgeEscrow::get_escrow_balance(@{{escrow}}) == amount, 20001);
        assert(BridgeEscrow::get_locked_length(@{{escrow}}) == 1, 20002);
    }
}
//! check: EXECUTED

///// Test 3: Bob deposit funds into escrow for carol
//! new-transaction
//! sender: bob
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;

    fun main(sender: signer){
        let transfer_id2: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E9";
        let amount: u64 = 10;
        BridgeEscrow::create_transfer_account(@{{escrow}}, &sender, @{{carol}}, amount, transfer_id2);
        assert(BridgeEscrow::get_escrow_balance(@{{escrow}}) == amount + 100, 20001);
        assert(BridgeEscrow::get_locked_length(@{{escrow}}) == 2, 20002);
    }
}
//! check: EXECUTED

///// Test 4: Carol deposit funds into escrow for alice
//! new-transaction
//! sender: carol
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;

    fun main(sender: signer){
        let transfer_id3: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E7";
        let amount: u64 = 50;
        BridgeEscrow::create_transfer_account(@{{escrow}}, &sender, @{{alice}}, amount, transfer_id3);
        assert(BridgeEscrow::get_escrow_balance(@{{escrow}}) == amount + 100 + 10, 20001);
        assert(BridgeEscrow::get_locked_length(@{{escrow}}) == 3, 20002);
    }
}

///// Test 3: Bridge agent detects that Bob deposited funds and
/// create complimentary transaction on the other chain
/// in this case the other chain is the same.
/// Complimentary account is carol
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;
    use 0x1::Signer;
    use 0x1::Option;

    fun main(escrow: signer){
        let transfer_id2: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E9";
        let escrow_address = Signer::address_of(&escrow);
        assert(BridgeEscrow::get_escrow_balance(escrow_address) == 100+ 50 + 10, 30001);

        // find account by transfer_id and make transfer to the "other" chain
        // which is the same chain with account carol
        let index = BridgeEscrow::find_locked_idx(escrow_address, &transfer_id2);
        assert(Option::is_some(&index),30002);
        let idx = Option::borrow(&index);
        assert(*idx == 1, 30003);

        BridgeEscrow::withdraw_from_escrow(&escrow, &transfer_id2);
        assert(BridgeEscrow::get_escrow_balance(escrow_address) == 50 + 100, 30004);

        assert(BridgeEscrow::get_locked_length(escrow_address) == 3, 30005);
        assert(BridgeEscrow::get_unlocked_length(escrow_address) == 1, 30006);
    }
}
//! check: EXECUTED

///// Test 4: Delete Bob escrow account
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;
    use 0x1::Option;
    use 0x1::Signer;

    fun main(escrow: signer){
        let transfer_id2: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E9";
        let escrow_address = Signer::address_of(&escrow);
        assert(BridgeEscrow::get_locked_length(escrow_address) == 3, 40001);

        // find unlocked account using transfer_id and delete locked entry
        let index = BridgeEscrow::find_unlocked_idx(escrow_address, &transfer_id2);
        assert(Option::is_some(&index),40002);
        let idx = Option::borrow(&index);
        assert(*idx == 0, 4003);

        BridgeEscrow::delete_transfer_account(&escrow, &transfer_id2);
        assert(BridgeEscrow::get_locked_length(escrow_address) == 2, 40004);
    }
}
//! check: EXECUTED

///// Test 4: Delete Bob escrow account
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;
    use 0x1::Option;
    use 0x1::Signer;

    fun main(escrow: signer){
        let transfer_id2: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E9";
        let escrow_address = Signer::address_of(&escrow);
        assert(BridgeEscrow::get_unlocked_length(escrow_address) == 1, 50001);

        let index = BridgeEscrow::find_unlocked_idx(escrow_address, &transfer_id2);
        assert(Option::is_some(&index),50002);
        let idx = Option::borrow(&index);
        assert(*idx == 0, 5003);

        BridgeEscrow::delete_unlocked(&escrow, &transfer_id2);
        assert(BridgeEscrow::get_unlocked_length(escrow_address) == 0, 50004);
    }
}
//! check: EXECUTED
