// For bridge related scripts.
address 0x1 {
module BridgeMultisigScripts {
    use 0x1::BridgeEscrowMultisig;
    use 0x1::Vector;

    const ZERO_ADDRESS: address = @0x0;

    public(script) fun bridge_multisig_create_escrow(
        sender: signer,
        executor1: address,
        executor2: address,
        executor3: address,
        executor4: address,
        executor5: address,
        min_votes:u64
    ) {
        let executors = Vector::empty<address>();
        if (executor1 != ZERO_ADDRESS) {
            Vector::push_back(&mut executors, executor1)
        };
        if (executor2 != ZERO_ADDRESS) {
            Vector::push_back(&mut executors, executor2)
        };
        if (executor3 != ZERO_ADDRESS) {
            Vector::push_back(&mut executors, executor3)
        };
        if (executor4 != ZERO_ADDRESS) {
            Vector::push_back(&mut executors, executor4)
        };
        if (executor5 != ZERO_ADDRESS) {
            Vector::push_back(&mut executors, executor5)
        };
        BridgeEscrowMultisig::initialize_escrow(&sender, executors, min_votes);
    }

    public(script) fun bridge_multisig_deposit(
        sender: signer,
        escrow: address,
        receiver_other: vector<u8>,
        value: u64,
        transfer_id: vector<u8>,
    ) {
        BridgeEscrowMultisig::create_transfer_account(escrow, &sender, receiver_other, value, transfer_id);
    }

    public(script) fun bridge_multisig_deposit_funds(
        sender: signer,
        escrow: address,
        value: u64,
    ) {
        BridgeEscrowMultisig::deposit_funds(escrow, &sender, value);
    }

    public(script) fun bridge_multisig_withdraw(
        sender: signer,
        escrow: address,
        sender_other: vector<u8>,
        receiver: address,
        balance: u64,
        transfer_id: vector<u8>,
    ) {
        BridgeEscrowMultisig::withdraw_from_escrow(&sender, escrow,
            sender_other,
            receiver, // receiver
            balance, // balance
            transfer_id, // transfer_id
        );
    }

    public(script) fun bridge_multisig_close_transfer(
        sender: signer,
        escrow: address,
        transfer_id: vector<u8>,
        close_other: bool,
    ) {
        if (!close_other) {
            BridgeEscrowMultisig::delete_transfer_account( & sender, escrow, &transfer_id);
        } else {
            BridgeEscrowMultisig::delete_unlocked( & sender, escrow, &transfer_id);
        }
    }
}
}
