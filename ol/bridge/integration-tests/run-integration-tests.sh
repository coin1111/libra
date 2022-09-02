set -e
export SWARM_PATH=$HOME/swarm_temp/
export SWARM_PERSONA=alice
export ETH_BRIDGE_ESCROW_CONFIG=$HOME/projects/eth-bridge/.bridge_escrow_avax.config
export ETH_ACCOUNTS_PATH=$HOME/libra/ol/bridge/accounts-avax
# 0L->ETH transfer test
#cargo test -v --package integration-tests --lib transfer_test_ol_eth::test_transfer_ol_eth -- --exact --nocapture
# ETH->0L transfer test
cargo test -v --package integration-tests --lib transfer_test_eth_ol::test_transfer_eth_ol -- --exact --nocapture 

