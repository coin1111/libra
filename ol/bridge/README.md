# OL to ETH Bridge

## Build Bridge
```
./build-deps.sh 
```

## Run bridge in dev mode on 0L
run swarm:
```
cd ~/libra
NODE_ENV="test" cargo run -p diem-swarm -- --diem-node target/debug/diem-node -c $HOME/swarm_temp -n 2 -s --cli-path target/debug/cli
```

use alice menmonics:
```
talent sunset lizard pill fame nuclear spy noodle basket okay critic grow sleep legend hurry pitch blanket clerk impose rough degree sock insane purse
```

## Setup Validator accounts
```
./setup-accounts.sh
```

## Create Test Account
```asm
./create-accounts.sh
```


## Test Bridge Contracts
```

# run tests
./test-bridge.sh
```
