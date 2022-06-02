# OL to ETH Bridge Multisig Version

This version of an bridge uses multisig or voting approach.
With this approach there are multiple trusted agents (e.g. alice, bob, carol),
which vote on withdrawal and close outstanding transfer accounts.
Smart contract keep tallis of votes and when cound reaches 
2 out 3 votes required it will execute method. This bridge is symmetric,
e.g. both 0L and ETH chains have the same wegith and agents vote on both chains.

Example:
User Pete deposits coins into bridge on 0L side by invoking
createTransferAccount(eth_target_addr,amount,transfer_id), where
- eth_target_addr - destination address o ETH
- amount - amount to transfer
- transfer_id - unique guid for the transfer
- 
Agent Alice detects the deposit and invokes withdraw(eth_target_addr, amount,transfer_id).
At this time there is only 1 vote from Alice recorded on ETH.
Agent Bob also detects the deposit and invokes withdraw(eth_target_addr, amount,transfer_id).
At this time there are  2 vote from Alice and Bob recorded on ETH. It is above threshold of 2 out 3, 
thus withdrawal is permitted and funds are transferred into eth_target_addr.

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
./create-accounts.sh 2 # init bridge escrow with at least 2 signers for mutisig
```

## Launch ETH Hardhat Dev Network 
```asm
git clone https://github.com/coin1111/eth-bridge`
cd eth-bridge

# setup hardhat
./setup-hardhat.sh
npx hardhat compile
# run ETH node
./run-local-node.sh
# deploy ETH bridge contracts
./deploy-local.sh 2 # init bridge escrow with at least 2 signers for mutisig
```

## Test Bridge Agent
```
# run agent under alice identity
cd ~/libra/ol/bridge/
./run-agent.sh alice

# in separate terminal
# run agent under bob identity
cd ~/libra/ol/bridge/
./run-agent.sh bob

# in separate terminal
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

