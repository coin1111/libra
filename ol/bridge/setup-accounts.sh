set -e

pushd ../..
cargo run -p ol -- --swarm-path=$HOME/swarm_temp --swarm-persona=alice init --source-path $HOME/libra

cargo run -p ol -- --swarm-path=$HOME/swarm_temp --swarm-persona=bob init --source-path $HOME/libra

# fix $HOME dir inside fixture files
i=0
for v in alice.0L.toml bob.0L.toml
do
    v1=ol/bridge/fixtures/configs/"$v"
    b=$HOME;c=$(echo $b | sed -e s/\\//\\\\\\//g);echo $c
    a='sed -e s/\$HOME/"$c"/ "$v1"'
    eval $a > $HOME/swarm_temp/"$i"/0L.toml
    i=$((i+1))
done

popd
