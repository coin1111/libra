# Generate Abis for Rust Compiler for ETH Contract

```asm
cargo run --package abigen-ol --bin abigen-ol --  <contract-name> <contract-abi.json> [contract-abi.rs]
```

## Generate eth bridge mods
```asm
./generate-mods.sh
```

abigen has a bag and incorrectly generates code for bridge_escrow_mod.rs. To correct that replace the following 2 methods in this file to have signature like this:
```
pub fn get_locked_account_info(
            &self,
            transfer_id: [u8; 16],
        ) -> ContractCall<
            'a,
            P,
            S,
            Token,
        > 
        
pub fn get_unlocked_account_info(
            &self,
            transfer_id: [u8; 16],
        ) -> ContractCall<
            'a,
            P,
            S,
            Token,
        > 
```