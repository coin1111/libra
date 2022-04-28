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

Launch ETH devnet using hardhat
```asm
git clone https://github.com/coin1111/eth-bridge
cd eth-bridge
./setup-hardhat.sh
npx hardhat compile
./run-local-node.sh
./deploy-local.sh
```

Launch bridge agent
```asm
cd ~/libra
cargo run -p bridge-agent -- --swarm-path=$HOME/swarm_temp/ --swarm-persona=alice  -a 708B1D23219EB737035CB29A68F0F3A8 agent 
```

## Run Integration Tests
```asm
cd integration-tests
./run-integration-tests.sh
```

## Project Structure
* abigen-cli - generate rust files to interact with ETH bridge contract using abis
* accounts-eth - ETH accounts used for development and testing
* bridge-cli - cli to manage bridge
* bridge-eth - rust library to interact with ETH bridge contract
* bridge-ol - rust library to interact with 0L bridge contract
* integration-tests - end-to-end bridge test
