//! Bridge agent
use crate::bridge_escrow::BridgeEscrow;
use crate::{node::node::Node, node::query::QueryType};
use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AccountInfo {
    sender_this: String,
    sender_other: String,
    receiver_this: String,
    receiver_other: String,
    balance: u64,
    transfer_id: String,
}

/// Bridge agent struct
pub struct Agent {
    /// Node to connect to blockchain
    pub node: Node,
    /// Escrow address of BridgeEscrow
    pub escrow: AccountAddress,

    /// BridgeEscrow contract
    pub bridge_escrow: BridgeEscrow,
}

impl Agent {
    /// Create a new bridge agent
    pub fn new_agent(account: AccountAddress,  node: Node) -> Agent {
        Agent {
            node,
            escrow: account,
            bridge_escrow: BridgeEscrow {
                escrow: account,
            },
        }
    }
    /// Process autstanding transfers
    pub fn process_deposits(&self) {
        println!("INFO: process deposits");
        let ais = self.query_locked();
        if ais.is_err() {
            println!("WARN: Failed to get locked: {}", ais.unwrap_err());
            return;
        }
        for ai in ais.unwrap() {
            match self.process_deposit(&ai) {
                Ok(()) => println!("INFO: Succesfully processed transfer: {}", ai.transfer_id),
                Err(err) => println!(
                    "ERROR: Failed to process transfer: {}, error: {}",
                    ai.transfer_id, err
                ),
            }
        }
    }

    /// Process individual transfer
    // Transfer deposit from escrow to destination receiver
    // Ensure that unlocked doesn't have an entry for this transfer
    // This indicates that transfer has not been made, thus proceed with transfer
    fn process_deposit(&self, ai: &AccountInfo) -> Result<(), String> {
        use std::str::FromStr;
        println!("INFO: Processing deposit: {:?}", ai);
        if ai.transfer_id.is_empty() {
            return Err(format!("Empty deposit id: {:?}", ai));
        }
        // Query unlocked
        let unlocked = self.query_unlocked();
        if unlocked.is_err() {
            return Err(format!("Failed to get unlocked: {}", unlocked.unwrap_err()));
        }
        let unlocked_ai = unlocked
            .unwrap()
            .iter()
            .find(|x| x.transfer_id == ai.transfer_id)
            .and_then(|x| Some(x.clone()));
        if unlocked_ai.is_none() {
            let sender_this = AccountAddress::from_str(&ai.sender_this);
            if sender_this.is_err() {
                return Err(format!(
                    "Failed to parse sender address: {}",
                    sender_this.unwrap_err()
                ));
            }

            // try to parse receiver address on 0L chain
            let receiver_this = if ai.receiver_this.len() != 32 {
                let p = AccountAddress::from_str(&ai.receiver_this);
                if p.is_err() {
                    return Err(format!(
                        "Failed to parse receiver address: {}",
                        p.unwrap_err()
                    ));
                } else {
                    p.unwrap()
                }
            } else {
                AccountAddress::new([0;16])
            };

            // try to parse receiver address on ETH chain

            let transfer_id = hex_to_bytes(&ai.transfer_id);
            if transfer_id.is_none() {
                return Err(format!("Failed to parse transfer_id: {}", ai.transfer_id));
            }
            // Transfer is not happened => transfer funds
            println!("INFO: withdraw from bridge, ai: {:?}", ai);
            let res = self.bridge_escrow.bridge_withdraw(
                sender_this.unwrap(),
                Vec::new(),
                receiver_this,
                ai.balance,
                transfer_id.unwrap(),
                None,
            );
            if res.is_err() {
                return Err(format!(
                    "Failed to withdraw from escrow: {:?}",
                    res.unwrap_err()
                ));
            }
            println!("INFO: withdraw from bridge: {:?}", res.unwrap());
        }

        Ok(())
    }

    /// For compeleted transfers, remove locked and unlocked entries in this  porder
    pub fn process_withdrawals(&self) {
        println!("INFO: process withdrawals");
        let ais = self.query_unlocked();
        if ais.is_err() {
            println!("WARN: Failed to get unlocked: {}", ais.unwrap_err());
            return;
        }
        for ai in ais.unwrap() {
            match self.process_withdrawal(&ai) {
                Ok(()) => println!("INFO: Succesfully processed withdrawal: {}", ai.transfer_id),
                Err(err) => println!(
                    "ERROR: Failed to process withdrawal: {}, error: {}",
                    ai.transfer_id, err
                ),
            }
        }
    }

    /// Process individual transfer
    // If unlocked exists, remove locked and then unlocked in this order
    fn process_withdrawal(&self, ai: &AccountInfo) -> Result<(), String> {
        println!("INFO: Processing withdrawal: {:?}", ai);
        if ai.transfer_id.is_empty() {
            return Err(format!("Empty transfer id: {:?}", ai));
        }
        let _transfer_id = hex_to_bytes(&ai.transfer_id);
        if _transfer_id.is_none() {
            return Err(format!("Failed to parse transfer_id: {}", ai.transfer_id));
        }
        let transfer_id = _transfer_id.unwrap();
        // Query locked
        let locked = self.query_locked();
        if locked.is_err() {
            return Err(format!("Failed to get locked: {}", locked.unwrap_err()));
        }
        // Transfer happened , remove locked
        let locked_ai = locked
            .unwrap()
            .iter()
            .find(|x| x.transfer_id == ai.transfer_id)
            .and_then(|x| Some(x.clone()));
        if locked_ai.is_some() {
            println!("INFO: remove locked: {:?}", locked_ai);
            let res = self.bridge_escrow.bridge_close_transfer(
                &transfer_id,
                false, //close_other
                None,
            ) ;
            if res.is_err() {
                return Err(format!(
                    "Failed to remove locked: {:?}",
                    res.unwrap_err()
                ));
            }
            println!("INFO: removed locked: {:?}", res.unwrap());
        }

        // Locked is removed , remove unlocked
        // Query locked
        let unlocked = self.query_unlocked();
        if unlocked.is_err() {
            return Err(format!("Failed to get unlocked: {}", unlocked.unwrap_err()));
        }
        let unlocked_ai = unlocked
            .unwrap()
            .iter()
            .find(|x| x.transfer_id == ai.transfer_id)
            .and_then(|x| Some(x.clone()));
        if unlocked_ai.is_some() {
            println!("INFO: remove unlocked: {:?}", unlocked_ai);
            let res = self.bridge_escrow.bridge_close_transfer(
                &transfer_id,
                true, //close_other
                None,
            ) ;
            if res.is_err() {
                return Err(format!(
                    "Failed to remove unlocked: {:?}",
                    res.unwrap_err()
                ));
            }
            println!("INFO: removed unlocked: {:?}", res.unwrap());
        }
        Ok(())
    }


    fn query_locked(&self) -> Result<Vec<AccountInfo>, String> {
        return self.query_account_info("locked");
    }
    fn query_unlocked(&self) -> Result<Vec<AccountInfo>, String> {
        return self.query_account_info("unlocked");
    }

    // Example of account info
    // {
    // "modifiers":["copy","drop","store"],
    // "struct":{"0x1::BridgeEscrow::AccountInfo":{
    // "sender_this": "770b2c65843b25ca12ca48091fc33cd8",
    // "sender_other": "",
    // "receiver_this": "8671af7a44f80253f3e141123ff4a7d2",
    // "receiver_other": "",
    // "balance": 100,
    // "transfer_id": "1111",
    // }}},
    fn query_account_info(&self, field_name: &str) -> Result<Vec<AccountInfo>, String> {
        let query_type = QueryType::MoveValue {
            account: self.escrow.clone(),
            module_name: String::from("BridgeEscrow"),
            struct_name: String::from("EscrowState"),
            key_name: String::from(field_name),
        };

        match self.node.query_locked(query_type) {
            Ok(info) => {
                let res: serde_json::Result<Value> = serde_json::from_str(info.as_str());
                let mut ais: Vec<AccountInfo> = Vec::new();
                match res {
                    Ok(v) => {
                        let mut i = 0;
                        loop {
                            let r = v
                                .get(i)
                                .and_then(|o| o.as_object())
                                .and_then(|o| o.get("struct"))
                                .and_then(|o| o.get("0x1::BridgeEscrow::AccountInfo"))
                                .and_then(|o| {
                                    let ai: serde_json::Result<AccountInfo> =
                                        serde_json::from_value(o.clone());
                                    match ai {
                                        Ok(i) => ais.push(i),
                                        _ => {}
                                    }
                                    Some({})
                                });
                            if r.is_none() {
                                break;
                            }
                            i += 1;
                        }
                        return Ok(ais);
                    }

                    Err(e) => {
                        println!("ERROR: {}", e);
                        return Err(format!("parse error: {:?}", e));
                    }
                }
            }
            Err(e) => {
                println!("ERROR: {}", e);
                return Err(format!("query error: {:?}", e));
            }
        }
    }
}

fn hex_to_bytes(s: &String) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| {
                s.get(i..i + 2)
                    .and_then(|sub| u8::from_str_radix(sub, 16).ok())
            })
            .collect()
    } else {
        None
    }
}
