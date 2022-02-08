// For bridge related scripts.
address 0x1 {
module BridgeScripts {
    use 0x1::BridgeEscrow;
    use 0x1::Vector;

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
        BridgeEscrow::create_transfer_account(escrow, &sender, destination, Vector::empty<u8>(), value, transfer_id);
    }

    public(script) fun bridge_withdraw(
        sender: signer,
        escrow: address,
        transfer_id: vector<u8>,
    ) {
        BridgeEscrow::withdraw_from_escrow(&sender, escrow,
            escrow,
            Vector::empty<u8>(), // sender_other
            escrow, // receiver
            0, // balance
            transfer_id, // transfer_id
        );
    }

    public(script) fun bridge_close_transfer(
        sender: signer,
        escrow: address,
        transfer_id: vector<u8>,
        close_other: bool,
    ) {
        if (!close_other) {
            BridgeEscrow::delete_transfer_account( & sender, escrow, &transfer_id);
        } else {
            BridgeEscrow::delete_unlocked( & sender, escrow, &transfer_id);
        }
    }
}
}