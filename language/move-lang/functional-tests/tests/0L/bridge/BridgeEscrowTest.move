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
        let transfer_id: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let amount: u64 = 100;
        BridgeEscrow::create_transfer_account(@{{escrow}}, &sender, @{{bob}}, amount, transfer_id);
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
    use 0x1::Option;

    fun main(escrow: signer){
        let transfer_id: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let escrow_address = Signer::address_of(&escrow);
        assert(BridgeEscrow::get_escrow_balance(escrow_address) == 100, 30001);

        // find accoutn by transfer_id t and make transfer to the "other" chain
        // which is the same chain with account bob
        let index = BridgeEscrow::find_locked_by_transfer_id(&transfer_id, escrow_address);
        assert(Option::is_some(&index),30002);
        let idx = Option::borrow(&index);

        let (sender, receiver, balance, transfer_id) = BridgeEscrow::get_locked_at(*idx, escrow_address);

        BridgeEscrow::withdraw_from_escrow(&escrow, sender, receiver, balance, transfer_id);
        assert(BridgeEscrow::get_escrow_balance(escrow_address) == 0, 30003);

        assert(BridgeEscrow::get_locked_length(escrow_address) == 1, 30004);
        assert(BridgeEscrow::get_unlocked_length(escrow_address) == 1, 30005);
    }
}
//! check: EXECUTED

///// Test 4: Delete alice escrow account
//! new-transaction
//! sender: escrow
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;
    use 0x1::Option;
    use 0x1::Signer;

    fun main(escrow: signer){
        let transfer_id: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let escrow_address = Signer::address_of(&escrow);
        assert(BridgeEscrow::get_locked_length(escrow_address) == 1, 40001);

        // find unlocked account using transfer_id and delete locked entry
        let index = BridgeEscrow::find_unlocked_by_transfer_id(&transfer_id, escrow_address);
        assert(Option::is_some(&index),40002);
        let idx = Option::borrow(&index);

        let (sender, receiver, amount, transfer_id) = BridgeEscrow::get_unlocked_at(*idx, escrow_address);
        BridgeEscrow::delete_transfer_account(&escrow, sender, receiver, amount, transfer_id);
        assert(BridgeEscrow::get_locked_length(escrow_address) == 0, 40003);
    }
}
//! check: EXECUTED

