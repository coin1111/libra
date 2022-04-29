# OL to ETH Bridge

## Build Bridge
```
./build-deps.sh 
```

## Run 0L Swarm Network
### Start Swarm
```
mkdir ~/swarm_temp
cd ~/libra
NODE_ENV="test" cargo run -p diem-swarm -- --diem-node target/debug/diem-node -c $HOME/swarm_temp -n 2 -s --cli-path target/debug/cli
```

use alice menmonics:
```
talent sunset lizard pill fame nuclear spy noodle basket okay critic grow sleep legend hurry pitch blanket clerk impose rough degree sock insane purse
```

### Setup 0L Validator accounts
```
# in another terminal
cd ~/libra/ol/bridge
./setup-accounts.sh
```

### Create 0L Test Account
```asm
cd ~/libra/ol/bridge
./create-accounts.sh
```

## Launch ETH Hardhat Dev Network 
```asm
`git clone https://github.com/coin1111/eth-bridge`
cd eth-bridge
./setup-hardhat.sh
npx hardhat compile
# run ETH node
./run-local-node.sh
# deploy ETH bridge contracts
./deploy-local.sh
```

## Test Bridge Agent
```
# run agent
cd ~/libra/ol/bridge/
./run-agent.sh

# run integration tests
# in another tempnial
cd ~/libra/ol/bridge/integration-tests
./run-integration-tests.sh
```

## Project Structure
* abigen-eth - cli to generate rust files to interact with ETH bridge contract using abis
* accounts-eth - ETH accounts used for development and testing
* bridge-cli - cli to manage bridge
* bridge-eth - rust library to interact with ETH bridge contract
* bridge-ol - rust library to interact with 0L bridge contract
* integration-tests - end-to-end bridge test

## 0L Bridge Contract
* ~/libra/language/diem-framework/modules/0L/BridgeEscrow.move - contract code
* ~/libra/language/move-lang/functional-tests/tests/0L/bridge - contract test

