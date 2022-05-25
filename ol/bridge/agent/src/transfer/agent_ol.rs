//! Agent for 0L
use crate::entrypoint::tx_params_wrapper;
use crate::transfer::processor::AccountInfo;
use crate::{node::node::Node, node::query::QueryType};
use anyhow::{anyhow, Error};
use bridge_ol::contract::BridgeEscrowMultisig;
use move_core_types::account_address::AccountAddress;
use ol_types::config::TxType;
use serde_json::Value;

pub struct Agent0L {
    /// Node to connect to blockchain
    pub node_ol: Node,

    /// BridgeEscrow contract for 0L
    pub bridge_escrow_ol: BridgeEscrowMultisig,
}

impl Agent0L {
    /// Create a new 0L agent
    pub fn new(ol_escrow: AccountAddress, node_ol: Node) -> Result<Agent0L, Error> {
        let tx_params = tx_params_wrapper(TxType::Mgmt)?;
        let bridge_escrow_ol = BridgeEscrowMultisig::new(ol_escrow, tx_params)?;
        Ok(Agent0L {
            node_ol,
            bridge_escrow_ol,
        })
    }

    pub fn query_ol_locked(&mut self) -> Result<Vec<AccountInfo>, Error> {
        return self.query_account_info("locked");
    }
    pub fn query_ol_unlocked(&mut self) -> Result<Vec<AccountInfo>, Error> {
        return self.query_account_info("unlocked");
    }

    // Example of account info
    /*
    {
    "modifiers":["copy","drop","store"],
    "struct":{"0x1::BridgeEscrowMultisig::AccountInfo":{
    "sender_this": "770b2c65843b25ca12ca48091fc33cd8",
    "sender_other": "",
    "receiver_this": "8671af7a44f80253f3e141123ff4a7d2",
    "receiver_other": "",
    "balance": 100,
    "transfer_id": "1111",
    "votes":[],
    "current_votes":0,
    "is_closed":false,
    }}},
    */
    pub fn query_account_info(&mut self, field_name: &str) -> Result<Vec<AccountInfo>, Error> {
        let query_type = QueryType::MoveValue {
            account: self.bridge_escrow_ol.escrow.clone(),
            module_name: String::from("BridgeEscrowMultisig"),
            struct_name: String::from("EscrowState"),
            key_name: String::from(field_name),
        };

        match self.node_ol.query_locked(query_type) {
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
                                .and_then(|o| o.get("0x1::BridgeEscrowMultisig::AccountInfo"))
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
                        return Err(anyhow!(format!("parse error: {:?}", e)));
                    }
                }
            }
            Err(e) => {
                println!("ERROR: {}", e);
                return Err(anyhow!(format!("query error: {:?}", e)));
            }
        }
    }
}
