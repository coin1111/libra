set -e
export SWARM_PATH=/home/ruslan/swarm_temp/
export SWARM_PERSONA=alice
export ETH_BRIDGE_ESCROW_CONFIG=/home/ruslan/projects/eth-bridge/.bridge_escrow.config
export ETH_ACCOUNTS_PATH=/home/ruslan/libra/ol/bridge/accounts-eth
cargo test --package integration-tests --lib transfer_test_eth_ol::test_transfer_ol_eth -- --exact
cargo test --package integration-tests --lib transfer_test_ol_eth::test_transfer_eth_ol -- --exact
