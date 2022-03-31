if [ "$1" == "" ]; then
    echo "Usage: get-escrow-state.sh [locked|unlocked]"
    exit 1
fi
cargo run -p ol -- --swarm-path=/home/ruslan/swarm_temp/ --swarm-persona=bridge-escrow  -a 708B1D23219EB737035CB29A68F0F3A8 query --move-state --move-module BridgeEscrow --move-struct EscrowState --move-value "$1"
