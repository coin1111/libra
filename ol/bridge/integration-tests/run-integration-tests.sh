set -e
export SWARM_PATH=$HOME/swarm_temp/
export SWARM_PERSONA=alice
export ETH_BRIDGE_ESCROW_CONFIG=$HOME/projects/eth-bridge/.bridge_escrow.config
export ETH_ACCOUNTS_PATH=$HOME/libra/ol/bridge/accounts-eth
# 0L->ETH transfer test
cargo test --package integration-tests --lib transfer_test_ol_eth::test_transfer_ol_eth -- --exact
# ETH->0L transfer test
cargo test --package integration-tests --lib transfer_test_eth_ol::test_transfer_eth_ol -- --exact

