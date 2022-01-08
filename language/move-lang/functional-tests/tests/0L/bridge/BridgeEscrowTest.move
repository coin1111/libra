///// Setting up the test fixtures for the transactions below. 
///// The tags below create validators alice and bob, giving them 1000000 GAS coins.

//! account: alice, 1000000, 0
//! account: bob, 0, 0
//! account: escrow, 0, 0
//! account: carol, 1000000, 0, validator

///// Test 1: Init escrow account
//! new-transaction
//! sender: diemroot
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;

    fun main(vm: signer){
        BridgeEscrow::init_escrow_state(&vm);
    }
}

///// Test 2: Alice deposit funds into escrow
//! new-transaction
//! sender: alice
//! gas-currency: GAS
script {
    use 0x1::BridgeEscrow;
    use 0x1::Diem;

    fun main(sender: signer){
        let target_address = @0x2c7536E3605D9C16a7a3D7b1898e529396a65c23;
        let amount: u64 = 100;
        deposit_to_escrow(&sender, target_address, amount);
    }
}


