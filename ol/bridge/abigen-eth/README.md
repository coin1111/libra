# Generate Abis for Rust Compiler for ETH Contract

```asm
cargo run --package abigen-eth --bin abigen-eth --  <contract-name> <contract-abi.json> [contract-abi.rs]
```

## Generate eth bridge mods
```asm
./generate-mods.sh
```

abigen has a bug and incorrectly generates code for bridge_escrow_mod.rs. To correct that replace tuple (...) in the signature with Token in the following 2 methods in this file :
```
pub fn get_locked_account_info(
            &self,
            transfer_id: [u8; 16],
        ) -> ContractCall<
            'a,
            P,
            S,
            Token, // **** abigen generates tuple (...) here ****, replace with Token
        > 
        
pub fn get_unlocked_account_info(
            &self,
            transfer_id: [u8; 16],
        ) -> ContractCall<
            'a,
            P,
            S,
            Token, // **** abigen generates tuple (...) here ****, replace with Token
        > 
```