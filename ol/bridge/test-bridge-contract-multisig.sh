amount=100
# deposit
cargo run -p bridge-cli -- --swarm-path=$HOME/swarm_temp/ --swarm-persona=pete bridge-deposit -e 708B1D23219EB737035CB29A68F0F3A8 -l 8671AF7A44F80253F3E141123FF4A7D2 -c "$amount" -t 1234

