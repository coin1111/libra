pushd ../..
cargo run -p ol -- --swarm-path=$HOME/swarm_temp --swarm-persona=alice init --source-path $HOME/libra

cargo run -p ol -- --swarm-path=$HOME/swarm_temp --swarm-persona=bob init --source-path $HOME/libra

cp ol/devnet/fixtures/config/alice.0L.toml $HOME/swarm_temp/0/0L.toml
cp ol/devnet/fixtures/config/bob.0L.toml $HOME/swarm_temp/1/0L.toml
popd
