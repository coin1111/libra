# Crosss Chain Transfer Mechanics using Escrow Account

## User Scenario
User Alice on Chain1 wants to transfer 
coins to Bob on Chain2. To facilitate a transfer, 
Chain1 and Chain2 both have individual
escrow accounts. An escrow account contains a balance to be transferred
as well as en entry about initiated or completed transfer.
Initiated tranfer entry is recorded in ```locked``` vector, while completed transfers are recorded
using ```unlocked``` vector. Each entry in ```locked``` and ```unlocked``` is identified by transfer_id . There is a trusted actor which performs
actions of moving coins and updating escrow account state.

## Smart contract methods
The following methods are used to perform a transfer
1. ```create_transfer_account(sender,receiver,amount,transfer_id)```
moves coins from Alice's account (sender) into escrow account on Chain1
Invoked by Alice.
2. ```withdraw_from_escrow(transfer_id)```
move coins from escrow account on Chain2 to Bob's.
Invoked by the bridge agent.
3. ```delete_transfer_account(transfer_id)```
Deletes escrow account on Chain1 after transfer.
Invoked by the bridge agent.
4. ```delete_unlocked(transfer_id)```
Deletes an entry on Chain2 after transfer is completed.
Invoked by the bridge agent.

Using example above, ```sender``` is Alice's account on Chain1. ```receiver``` is Bob's account on Chain2.
```transfer_id``` is unique identifier  on both chains which helps to track
state of the transfer.

Each method invocation is transactional. A method either completes or fails
.In the case of failure a state of blockchain is unchanged.

## Steps in the transfer

1. Alice calls ```create_transfer_account(alice,bob,100,0x12345)```. Inside the method,
funds are moved from Alice's account on Chain1 into escrow on Chain1. An entry in ```unlocked```
vector for this ```transfer_id=0x12345``` is also added to indicate that transfer is initiated
2. A bridge agent detects that new entry in ```locked``` has been created on Chain1. The agent 
calls ```withdraw_from_escrow(0x12345)``` on Chain2. This method
will tranfer funds from escrow on Chain2 into Bob's account. It also adds an entry in ```unlocked``` vector 
to indicate that Bob received the funds.
3. A bridge agent detects that ```unlocked``` 
entry has been added on Chain2. The agent invokes
```delete_transfer_account(0x12345)``` on Chain1 to indicate
that funds were transferred on Chain2. This method
deletes an entry in ```locked``` on Chain1.
4. A bridge agent detects that an ```locked``` entry has been deleted on Chain1. The agent 
invokes ```delete_unlocked(0x12345)``` on Chain2 to indicate
that transfer has been completed on both chains. This method removes ```unlocked```
entry on Chain2.

## Handling Bridge Agent Crash
An agent can crash. However steps above ensure that an agent can restart
and resume from the last action maintaining integrity of the transfer on both chains.
When an agent recovers from a crash, first it processes ```unlocked``` entries on Chain2:
1. it checks unlocked entries on Chain2 agains locked entries 
on Chain 1. If there are no corresponding entries on Chain1, then 
it calls ```delete_unlocked(transfer_id)``` for each orphaned unlocked entry on Chain2 (step 4).
2. if there are matching locked entries on Chain1 it proceeds to execute step 3 for each unlocked entry.

Second the agent processes remaining ```locked``` on Chain1. For each ```locked```
entry it resumes execution from step 2.

## Distributed (multi-sig) version of the bridge
The main algorithm of distributed (multi-sig)  version is the same as for single-agent version. Steps 2-4 of the transfaer are handled by multiple agents instead of one. Each agent is run by a validator and executes under context a validator. Bridge contract methods are modified to count votes of agents. Whenever a threshold is met, e.g. 3 out 5 required votes by validator agent are cast, method is executed. Otherwise vote count is incremented. When method is executed it is marked as executed for this invocation (based on hash of its arguments) such that an agent cannot accidentally call the same method twice.

Only selected subset of validators are selected to be participants in bridge operation. This is needed to reduce amount of transactions required on both move and eth side. Addresses of validators are stored in bridge contract and can be configured by any of the validator in the bridge set. This approach simplifies management, introducing a tradeoff that validators must be highly trusted. To ensure bridge validator complience, a stake can be requested to be held in s special account to compencate any potential loss due to a validator malicious behaviour.


## Running Tests
```
cd language/move-lang/functional-tests/tests/0L
cargo  test BridgeEscrowMultiTest
```

