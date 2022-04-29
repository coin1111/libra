set -e
pushd ../..
cargo run -p ol -- --swarm-path=$HOME/swarm_temp --swarm-persona=alice init --source-path $HOME/libra

cargo run -p ol -- --swarm-path=$HOME/swarm_temp --swarm-persona=bob init --source-path $HOME/libra

cp ol/fixtures/configs/alice.0L.toml $HOME/swarm_temp/0/0L.toml
cp ol/fixtures/configs/bob.0L.toml $HOME/swarm_temp/1/0L.toml
popd
