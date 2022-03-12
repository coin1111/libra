# Bridge Agent


## Quick start

```
# launch swarm
cd ~/libra
NODE_ENV="test" cargo run -p diem-swarm -- --diem-node target/debug/diem-node -c $HOME/swarm_temp -n 2 -s --cli-path target/debug/cli
# use the following mnemonics
talent sunset lizard pill fame nuclear spy noodle basket okay critic grow sleep legend hurry pitch blanket clerk impose rough degree sock insane purse
```

Setup accounts
```asm
~/libra/ol/bridge
./setup-accounts.sh

# create and transfer funds
./create-accounts.sh
```

Launch bridge agent
```asm
cd ~/libra
cargo run -p bridge-agent -- --swarm-path=$HOME/swarm_temp/ --swarm-persona=alice  -a 708B1D23219EB737035CB29A68F0F3A8 agent 
```

Make deposit
```asm
cd ~/libra
cargo run -p txs -- --swarm-path=$HOME/swarm_temp/ --swarm-persona=pete bridge-deposit -e 708B1D23219EB737035CB29A68F0F3A8 -l 8671AF7A44F80253F3E141123FF4A7D2 -c 100 -t 1122
```

Check balance of target account
```asm
cargo run -p ol -- --swarm-path=$HOME/swarm_temp/ --swarm-persona=alice -a 8671AF7A44F80253F3E141123FF4A7D2 query --balance
```
