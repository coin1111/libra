export ETH_BRIDGE_ESCROW_CONFIG=$HOME/projects/eth-bridge/.bridge_escrow.config
export ETH_BRIDGE_ESCROW_ACCOUNT=$HOME/projects/eth-bridge/accounts/alice.txt
cargo run -p bridge-agent -- --swarm-path=$HOME/swarm_temp/ --swarm-persona=alice  -a 708B1D23219EB737035CB29A68F0F3A8 agent 
