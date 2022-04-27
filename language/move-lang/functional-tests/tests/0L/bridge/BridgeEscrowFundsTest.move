///// Setting up the test fixtures for the transactions below. 
///// The tags below create validators alice and bob, giving them 1000000 GAS coins.
/// Test simple deposit-withdraw capability

//! account: alice, 1000000, 0
//! account: bob, 1, 0
//! account: escrow, 1, 0
//! account: carol, 1000000, 0, validator

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
        let amount: u64 = 100;
        BridgeEscrow::deposit_funds(@{{escrow}}, &sender, amount);
        assert(BridgeEscrow::get_escrow_balance(@{{escrow}}) == amount, 20001);
        assert(BridgeEscrow::get_locked_length(@{{escrow}}) == 0, 20002);
    }
}
//! check: EXECUTED

///// Test 3: Carol deposit funds into escrow
//! new-transaction
//! sender: carol
//! gas-currency: GAS
script {
use 0x1::BridgeEscrow;

    fun main(sender: signer){
        let amount: u64 = 100;
        BridgeEscrow::deposit_funds(@{{escrow}}, &sender, amount);
        assert(BridgeEscrow::get_escrow_balance(@{{escrow}}) == 2*amount, 20001);
        assert(BridgeEscrow::get_locked_length(@{{escrow}}) == 0, 20002);
    }
}
//! check: EXECUTED

///// Test 4: Non-validator can't withdraw
//! new-transaction
//! sender: alice
//! gas-currency: GAS
script {
use 0x1::BridgeEscrow;

    fun main(sender: signer){
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrow::get_escrow_balance(escrow_address) == 200, 30001);

        BridgeEscrow::withdraw_funds(&sender, escrow_address,
            @{{alice}}, // receiver
            100,
            );
        assert(BridgeEscrow::get_escrow_balance(escrow_address) == 100, 30004);
        assert(BridgeEscrow::get_locked_length(escrow_address) == 0, 30005);
        assert(BridgeEscrow::get_unlocked_length(escrow_address) == 0, 30006);
    }
}
// check: ABORTED

///// Test 5: Withdraw funds from escrow
//! new-transaction
//! sender: carol
//! gas-currency: GAS
script {
use 0x1::BridgeEscrow;

    fun main(sender: signer){
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrow::get_escrow_balance(escrow_address) == 200, 30001);

        BridgeEscrow::withdraw_funds(&sender, escrow_address,
            @{{bob}}, // receiver
            100,
        );
        assert(BridgeEscrow::get_escrow_balance(escrow_address) == 100, 30004);
        assert(BridgeEscrow::get_locked_length(escrow_address) == 0, 30005);
        assert(BridgeEscrow::get_unlocked_length(escrow_address) == 0, 30006);
    }
}
//! check: EXECUTED

///// Test 6: Withdraw funds from escrow
//! new-transaction
//! sender: carol
//! gas-currency: GAS
script {
use 0x1::BridgeEscrow;

    fun main(sender: signer){
        let escrow_address: address = @{{escrow}};
        assert(BridgeEscrow::get_escrow_balance(escrow_address) == 100, 30001);

        BridgeEscrow::withdraw_funds(&sender, escrow_address,
            @{{alice}}, // receiver
            100,
            );
        assert(BridgeEscrow::get_escrow_balance(escrow_address) == 0, 30004);
        assert(BridgeEscrow::get_locked_length(escrow_address) == 0, 30005);
        assert(BridgeEscrow::get_unlocked_length(escrow_address) == 0, 30006);
    }
}
//! check: EXECUTED


