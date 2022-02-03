# deposit
cargo run -p txs -- --swarm-path=/home/ruslan/swarm_temp/ --swarm-persona=pete bridge-deposit -e 708B1D23219EB737035CB29A68F0F3A8 -d 8671AF7A44F80253F3E141123FF4A7D2 -c 100 -t 1234

# withdraw
cargo run -p txs -- --swarm-path=/home/ruslan/swarm_temp/ --swarm-persona=bridge-escrow bridge-withdraw -t 1234

# close transfer account on this chain
cargo run -p txs -- --swarm-path=/home/ruslan/swarm_temp/ --swarm-persona=bridge-escrow bridge-close-transfer -t 1234

# close transfer account on the other chain
cargo run -p txs -- --swarm-path=/home/ruslan/swarm_temp/ --swarm-persona=bridge-escrow bridge-close-transfer -t 1234 -o
