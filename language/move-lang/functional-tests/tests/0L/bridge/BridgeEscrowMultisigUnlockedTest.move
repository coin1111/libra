///// Setting up the test fixtures for the transactions below. 
///// The tags below create validators alice and bob, giving them 1000000 GAS coins.

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
        BridgeEscrowMultisig::initialize_escrow(&sender, executors, 2);
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
// check: EXECUTED

///// Test 4:
// transfer funds into local bob account
// do first vote, transfer doesn't happen
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

        BridgeEscrowMultisig::withdraw_from_escrow(&sender, escrow_address,
            sender_eth, // sender on eth chain
            @{{bob}}, // receiver
            100,
            copy transfer_id,
        );
        // not enough votes, transfer is not made
        assert(BridgeEscrowMultisig::get_escrow_balance(escrow_address) == 100, 30004);

        // find unloked entry
        let index_unlocked = BridgeEscrowMultisig::find_unlocked_idx(escrow_address, &transfer_id);
        assert(Option::is_some(&index_unlocked),30005);
        let idx_unlocked = Option::borrow(&index_unlocked);
        assert(*idx_unlocked == 0, 30004);
        let ai_unlocked = BridgeEscrowMultisig::get_unlocked_at(escrow_address, *idx_unlocked);
        let current_votes = BridgeEscrowMultisig::get_current_votes(&ai_unlocked);
        assert(current_votes == 1, 30006);
        assert(!BridgeEscrowMultisig::is_closed(&ai_unlocked), 30009);

        assert(BridgeEscrowMultisig::get_locked_length(escrow_address) == 1, 30007);
        assert(BridgeEscrowMultisig::get_unlocked_length(escrow_address) == 1, 30008);
    }
}
// check: EXECUTED

///// Test 5:
// transfer funds into local bob account
// do  vote by the same account, abort
//! new-transaction
//! sender: carol
//! gas-currency: GAS
script {
use 0x1::BridgeEscrowMultisig;

    fun main(sender: signer){
        let transfer_id: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let sender_eth: vector<u8> = x"90f79bf6eb2c4f870365e785982e1f101e93b906";
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrowMultisig::get_escrow_balance(escrow_address) == 100, 30001);

        BridgeEscrowMultisig::withdraw_from_escrow(&sender, escrow_address,
            sender_eth, // sender on eth chain
            @{{bob}}, // receiver
            100,
            copy transfer_id,
        );
    }
}
// check: ABORTED

///// Test 7:
// transfer funds into local bob account
// do second vote, using invalid params, abort
//! new-transaction
//! sender: dave
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrowMultisig;
    use 0x1::Option;
    use 0x1::DiemAccount;
    use 0x1::GAS::GAS;

    fun main(sender: signer){
        let transfer_id: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let sender_eth: vector<u8> = x"90f79bf6eb2c4f870365e785982e1f101e93b906";
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrowMultisig::get_escrow_balance(escrow_address) == 100, 30001);

        let balance_before = DiemAccount::balance<GAS>(@{{bob}});
        BridgeEscrowMultisig::withdraw_from_escrow(&sender, escrow_address,
            sender_eth, // sender on eth chain
            @{{bob}}, // receiver
            101,
            copy transfer_id,
        );
    }
}
// check: ABORTED

///// Test 7:
// transfer funds into local bob account
// do second vote, transfer happens
//! new-transaction
//! sender: dave
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrowMultisig;
    use 0x1::Option;
    use 0x1::DiemAccount;
    use 0x1::GAS::GAS;

    fun main(sender: signer){
        let transfer_id: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let sender_eth: vector<u8> = x"90f79bf6eb2c4f870365e785982e1f101e93b906";
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrowMultisig::get_escrow_balance(escrow_address) == 100, 30001);

        let balance_before = DiemAccount::balance<GAS>(@{{bob}});
        BridgeEscrowMultisig::withdraw_from_escrow(&sender, escrow_address,
        sender_eth, // sender on eth chain
        @{{bob}}, // receiver
        100,
        copy transfer_id,
        );
        // transfer happened
        assert(BridgeEscrowMultisig::get_escrow_balance(escrow_address) == 0, 30004);
        let balance_after = DiemAccount::balance<GAS>(@{{bob}});
        assert(balance_after-balance_before==100, 3005);

        // find unlocked entry
        let index_unlocked = BridgeEscrowMultisig::find_unlocked_idx(escrow_address, &transfer_id);
        assert(Option::is_some(&index_unlocked),30006);
        let idx_unlocked = Option::borrow(&index_unlocked);
        assert(*idx_unlocked == 0, 30007);
        let ai_unlocked = BridgeEscrowMultisig::get_unlocked_at(escrow_address, *idx_unlocked);
        let current_votes = BridgeEscrowMultisig::get_current_votes(&ai_unlocked);
        assert(current_votes == 0, 30008);
        assert(BridgeEscrowMultisig::is_closed(&ai_unlocked), 30009);

        assert(BridgeEscrowMultisig::get_locked_length(escrow_address) == 1, 30010);
        assert(BridgeEscrowMultisig::get_unlocked_length(escrow_address) == 1, 30011);
    }
}
// check: EXECUTED

///// Test 8:
// cannot vote on closed AccountInfo
//! new-transaction
//! sender: eve
//! gas-currency: GAS
script {
use 0x1::BridgeEscrowMultisig;

    fun main(sender: signer){
        let transfer_id: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let sender_eth: vector<u8> = x"90f79bf6eb2c4f870365e785982e1f101e93b906";
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrowMultisig::get_escrow_balance(escrow_address) == 100, 30001);

        BridgeEscrowMultisig::withdraw_from_escrow(&sender, escrow_address,
        sender_eth, // sender on eth chain
        @{{bob}}, // receiver
        100,
        copy transfer_id,
        );
    }
}
// check: ABORTED


///// Test 9: Delete unlocked entry
//! new-transaction
//! sender: carol
//! gas-currency: GAS
script {
use 0x1::BridgeEscrowMultisig;
use 0x1::Option;
use 0x1::Vector;

    fun main(sender: signer){
        let transfer_id: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrowMultisig::get_unlocked_length(escrow_address) == 1, 50001);

        // find unlocked account using transfer_id and delete locked entry
        let index = BridgeEscrowMultisig::find_unlocked_idx(escrow_address, &transfer_id);
        assert(Option::is_some(&index),50002);
        let idx = Option::borrow(&index);
        assert(*idx == 0, 5003);
        let ai = BridgeEscrowMultisig::get_unlocked_at(escrow_address, *idx);
        // it must be closed, e.g. transfer happened
        assert(BridgeEscrowMultisig::is_closed(&ai), 50002);
        // with no votes
        let current_votes = BridgeEscrowMultisig::get_current_votes(&ai);
        assert(current_votes == 0, 50003);
        let votes = BridgeEscrowMultisig::get_votes(&ai);
        assert( Vector::length(&votes) == 0, 50003);

        BridgeEscrowMultisig::delete_unlocked(&sender, escrow_address, &transfer_id);
        // entry is still there
        assert(BridgeEscrowMultisig::get_unlocked_length(escrow_address) == 1, 50004);
    }
}
//! check: EXECUTED

///// Test 10: Delete unlocked entry
//! new-transaction
//! sender: dave
//! gas-currency: GAS
script {
use 0x1::BridgeEscrowMultisig;
use 0x1::Option;
use 0x1::Vector;

    fun main(sender: signer){
        let transfer_id: vector<u8> = x"00192Fb10dF37c9FB26829eb2CC623cd1BF599E8";
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrowMultisig::get_unlocked_length(escrow_address) == 1, 50001);

        // find unlocked account using transfer_id and delete locked entry
        let index = BridgeEscrowMultisig::find_unlocked_idx(escrow_address, &transfer_id);
        assert(Option::is_some(&index),50002);
        let idx = Option::borrow(&index);
        assert(*idx == 0, 5003);
        let ai = BridgeEscrowMultisig::get_unlocked_at(escrow_address, *idx);
        // it must be closed, e.g. transfer happened
        assert(BridgeEscrowMultisig::is_closed(&ai), 50002);
        // with 1 vote
        let current_votes = BridgeEscrowMultisig::get_current_votes(&ai);
        assert(current_votes == 1, 50003);
        let votes = BridgeEscrowMultisig::get_votes(&ai);
        assert( Vector::length(&votes) == 1, 50004);

        BridgeEscrowMultisig::delete_unlocked(&sender, escrow_address, &transfer_id);
        // entry is removed
        assert(BridgeEscrowMultisig::get_unlocked_length(escrow_address) == 0, 50005);
    }
}
//! check: EXECUTED

