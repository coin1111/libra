amount=100
# deposit
cargo run -p txs -- --swarm-path=/home/ruslan/swarm_temp/ --swarm-persona=pete bridge-deposit -e 708B1D23219EB737035CB29A68F0F3A8 -l 8671AF7A44F80253F3E141123FF4A7D2 -c "$amount" -t 1234

# withdraw
cargo run -p txs -- --swarm-path=/home/ruslan/swarm_temp/ --swarm-persona=alice bridge-withdraw -e 708B1D23219EB737035CB29A68F0F3A8 -l 770B2C65843B25CA12CA48091FC33CD8 -b "$amount" -r 8671AF7A44F80253F3E141123FF4A7D2 -t 1234 

# close transfer account on this chain
cargo run -p txs -- --swarm-path=/home/ruslan/swarm_temp/ --swarm-persona=alice bridge-close-transfer -e 708B1D23219EB737035CB29A68F0F3A8  -t 1234

# close transfer account on the other chain
cargo run -p txs -- --swarm-path=/home/ruslan/swarm_temp/ --swarm-persona=alice bridge-close-transfer -e 708B1D23219EB737035CB29A68F0F3A8 -t 1234 -o
