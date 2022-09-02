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
        receiver_other: vector<u8>,
        value: u64,
        transfer_id: vector<u8>,
    ) {
        BridgeEscrow::create_transfer_account(escrow, &sender, receiver_other, value, transfer_id);
    }

    public(script) fun bridge_deposit_funds(
        sender: signer,
        escrow: address,
        value: u64,
    ) {
        BridgeEscrow::deposit_funds(escrow, &sender, value);
    }

    public(script) fun bridge_withdraw(
        sender: signer,
        escrow: address,
        sender_other: vector<u8>,
        receiver: address,
        balance: u64,
        transfer_id: vector<u8>,
    ) {
        BridgeEscrow::withdraw_from_escrow(&sender, escrow,
            sender_other,
            receiver, // receiver
            balance, // balance
            transfer_id, // transfer_id
        );
    }

    public(script) fun bridge_withdraw_funds(
        sender: signer,
        escrow: address,
        receiver: address,
        balance: u64,
    ) {
        BridgeEscrow::withdraw_funds(&sender, escrow,
            receiver, // receiver
            balance, // balance
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