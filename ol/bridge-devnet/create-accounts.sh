cargo run -p txs -- --swarm-path=$HOME/swarm_temp --swarm-persona=alice  create-account -a b471d15e523f4fbba50983ed0cb2bdf5770b2c65843b25ca12ca48091fc33cd8  -c 1

cargo run -p txs -- --swarm-path=$HOME/swarm_temp --swarm-persona=bob  create-account -a 1626d2f926949f5f8363df25f053d5258671af7a44f80253f3e141123ff4a7d2  -c 1

argo run -p txs -- --swarm-path=$HOME/swarm_temp --swarm-persona=alice  create-account -a 6c1578bcb229521f78099149d7f578f0708b1d23219eb737035cb29a68f0f3a8  -c 1
