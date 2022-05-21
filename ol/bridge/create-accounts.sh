set -e
# pete
cargo run -p txs -- --swarm-path=$HOME/swarm_temp --swarm-persona=alice  create-account -a b471d15e523f4fbba50983ed0cb2bdf5770b2c65843b25ca12ca48091fc33cd8  -c 1

# todd
cargo run -p txs -- --swarm-path=$HOME/swarm_temp --swarm-persona=bob  create-account -a 1626d2f926949f5f8363df25f053d5258671af7a44f80253f3e141123ff4a7d2  -c 1

# bridge-escrow account
cargo run -p txs -- --swarm-path=$HOME/swarm_temp --swarm-persona=alice  create-account -a 6c1578bcb229521f78099149d7f578f0708b1d23219eb737035cb29a68f0f3a8  -c 1

# init escrow: alice,bob, dave
cargo run -p bridge-cli -- --swarm-path=$HOME/swarm_temp/ --swarm-persona=bridge-escrow bridge-create-escrow -1 4c613c2f4b1e67ca8d98a542ee3f59f5 \
  -2 88e74dfed34420f2ad8032148280a84b -3  3DC18D1CF61FAAC6AC70E3A63F062E4B -v 1

# deposit some funds into bridge
cargo run -p bridge-cli -- --swarm-path=$HOME/swarm_temp/ --swarm-persona=alice deposit-funds -e 708B1D23219EB737035CB29A68F0F3A8 -c 500


rm $HOME/libra/.agent_checkpoint
