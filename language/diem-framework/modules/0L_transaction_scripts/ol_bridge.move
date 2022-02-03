// For bridge related scripts.
address 0x1 {
module BridgeScripts {
    use 0x1::BridgeEscrow;

    public(script) fun bridge_create_escrow(
        sender: signer,
    ) {
        BridgeEscrow::initialize_escrow(&sender);
    }

    public(script) fun bridge_deposit(
        sender: signer,
        escrow: address,
        destination: address,
        value: u64,
        transfer_id: vector<u8>,
    ) {
        BridgeEscrow::create_transfer_account(escrow, &sender, destination, value, transfer_id);
    }

    public(script) fun bridge_withdraw(
        sender: signer,
        transfer_id: vector<u8>,
    ) {
        BridgeEscrow::withdraw_from_escrow(&sender, &transfer_id);
    }

    public(script) fun bridge_close_transfer(
        sender: signer,
        transfer_id: vector<u8>,
        close_other: bool,
    ) {
        if (!close_other) {
            BridgeEscrow::delete_transfer_account( & sender, &transfer_id);
        } else {
            BridgeEscrow::delete_unlocked( & sender, &transfer_id);
        }
    }
}
}