if [ "$1" == "-h" ||  "$1" == "--help" ]; then
  echo "Usage: run-agent.sh [<executor>]"
  echo -e "\texecutor: alice, bob, carol"
fi

rm .agent_checkpoint
eth_executor="$1"
if [ "$eth_executor" == "" ]; then
  eth_executor="alice"
fi
export ETH_BRIDGE_ESCROW_CONFIG=~/eth-bridge/.bridge_escrow.config
export ETH_BRIDGE_ESCROW_ACCOUNT=~/eth-bridge/accounts/"$eth_executor".txt

cargo run -p bridge-agent -- --swarm-path=$HOME/swarm_temp/ --swarm-persona=alice  -a 708B1D23219EB737035CB29A68F0F3A8 agent
