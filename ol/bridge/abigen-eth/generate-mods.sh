cargo run --package abigen-eth --bin abigen-eth -- OLToken ../bridge-eth/abi/contracts/OLToken.sol/OLToken.json ../bridge-eth/src/oltoken_mod.rs
cargo run --package abigen-eth --bin abigen-eth -- BridgeEscrow ../bridge-eth/abi/contracts/BridgeEscrow.sol/BridgeEscrow.json ../bridge-eth/src/bridge_escrow_mod.rs
cargo run --package abigen-eth --bin abigen-eth -- BridgeEscrowMultisig ../bridge-eth/abi/contracts/BridgeEscrowMultisig.sol/BridgeEscrowMultisig.json ../bridge-eth/src/bridge_escrow_multisig_mod.rs

echo "*** Please don't forget to fix generated files. See README.md.Note section"