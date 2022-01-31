// For bridge related scripts.
address 0x1 {
module BridgeScripts {
    use 0x1::BridgeEscrow;

    public(script) fun bridge_create_escrow(
        sender: signer,
    ) {
        BridgeEscrow::initialize_escrow(&sender);
    }
}
}