//! Bridge agent
use crate::bridge_escrow::BridgeEscrow;
use crate::{node::node::Node, node::query::QueryType};
use bridge_ethers::bridge_escrow_mod::BridgeEscrow as BridgeEscrowEth;
use bridge_ethers::config::Config;
use bridge_ethers::util::AccountInfo as AccountInfoEth;
use ethers::prelude::Wallet as WalletEth;
use ethers::prelude::{Client as ClientEth, Wallet, H160};
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};
use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;
use std::fs;
use tokio::runtime::Runtime;

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
    pub node_ol: Node,

    /// BridgeEscrow contract for 0L
    pub bridge_escrow_ol: BridgeEscrow,

    agent_eth: Option<AgentEth>,
}

struct AgentEth {
    /// ETH Escrow Contract Address
    escrow_addr: Address,

    /// ETH client
    client: ClientEth<Http, WalletEth>,

    /// ETH gas price
    gas_price: u64,
}

impl AgentEth {
    pub fn new(
        config_eth: &Option<Config>,
        agent_eth: &Option<Wallet>,
    ) -> Result<AgentEth, String> {
        let escrow_addr = match &config_eth {
            Some(c) => c.get_escrow_contract_address(),
            None => Err(String::from("cannot get eth config")),
        }?;

        let provider_eth = match &config_eth {
            Some(c) => c.get_provider_url().and_then(|url| {
                Provider::<Http>::try_from(url.as_str()).map_err(|e| e.to_string())
            }),
            None => Err(String::from("cannot get eth config")),
        }?;

        let gas_price = match &config_eth {
            Some(c) => c.get_gas_price(),
            None => Err(String::from("cannot get eth config")),
        }?;

        let client = match &agent_eth {
            Some(w) => Ok(w.clone().connect(provider_eth.clone())),
            _ => Err(format!("wallet is not provided")),
        }?;
        Ok(AgentEth {
            escrow_addr,
            client,
            gas_price,
        })
    }
}

impl Agent {
    /// Create a new bridge agent
    pub fn new(
        ol_escrow: AccountAddress,
        node_ol: Node,
        config_eth: Option<Config>,
        agent_eth: Option<Wallet>,
    ) -> Agent {
        let agent_eth = match AgentEth::new(&config_eth, &agent_eth) {
            Ok(a) => Some(a),
            _ => None,
        };

        Agent {
            node_ol,
            bridge_escrow_ol: BridgeEscrow { escrow: ol_escrow },
            agent_eth,
        }
    }

    /// Process autstanding transfers
    pub fn process_deposits_eth_ol(&self) -> Result<(), String> {
        println!("INFO: process deposits from ETH to 0L");
        // use checkpoint to get start element
        let start_idx = fs::read_to_string(".agent_checkpoint")
            .and_then(|ss| {
                let v: Vec<&str> = ss.split('\n').collect();
                let start = v.get(0).and_then(|s| {
                    let idx = s
                        .split(',')
                        .collect::<Vec<&str>>()
                        .last()
                        .and_then(|v| Some(v.parse::<i32>().unwrap_or(0)));
                    idx
                });
                Ok(start.unwrap_or(0))
            })
            .unwrap_or(0);

        // Query unlocked on ETH
        let start = U256::from(start_idx);
        let len: U256 = U256::from(10);
        let locked = self.get_next_locked_info(start, len)?;

        println!("next locked: {:?}", locked);
        if locked.0 == [0u8; 16] {
            return Ok(());
        }
        println!("INFO: processing transfer_id: {:?}", locked.0);
        // check if it is processed already
        let locked_ai = self.query_locked_eth(locked.0)?;
        if locked_ai.is_closed {
            return Ok(());
        }
        let transfer_id_str = hex::encode(locked.0);
        // check if unlocked exists on 0L
        let unlocked_exists = self.query_unlocked().and_then(|v| {
            Ok(v.iter()
                .find(|ai| ai.transfer_id == transfer_id_str)
                .and_then(|ai| Some(ai.clone())))
        })?;
        if unlocked_exists.is_some() {
            println!(
                "INFO: 0L unlocked entry exists for transfer_id {}, remove it",
                transfer_id_str
            );
            // mark eth entry as completed
            self.close_eth_account(locked.0);
            // query ETH locked account
            let ai = self.query_locked_eth(locked.0)?;
            if ai.is_closed {
                println!("INFO: transfer_id: {:?} is processed ignore it", locked.0);
                // check if 0L unlocked is prersent and remove it
                self.query_unlocked().and_then(|v| {
                    match v.iter().find(|ai| ai.transfer_id == transfer_id_str) {
                        Some(ai) => {
                            self.close_eth_account(locked.0);
                            Ok(())
                        }
                        _ => Ok(()),
                    }
                })?;
                // dave checkpoint of the last transfer id processed to a file
                let data = format!("{},{}", hex::encode(locked.0), locked.1);
                fs::write(".agent_checkpoint", data).map_err(|err| {
                    format!("Unable to write file agent_checkpoint, error: {:?}", err)
                })?;
            }

            Ok(())
        } else {
            // unlocked doesn't exist
            // transfer fund on 0L
            let receiver_this =
                AccountAddress::from_bytes(locked_ai.receiver_other).map_err(|err| {
                    format!(
                        "cannot parse receiver_other: {:?}, error: {:?}",
                        locked_ai.receiver_other, err
                    )
                })?;
            self.withdraw_eth_ol(
                locked_ai.sender_other.to_vec(),
                receiver_this,
                locked_ai.balance,
                locked_ai.transfer_id,
            );

            Ok(())
        }
    }

    /// Process individual transfer
    // Transfer deposit from escrow to destination receiver
    // Ensure that unlocked doesn't have an entry for this transfer
    // This indicates that transfer has not been made, thus proceed with transfer
    fn process_deposit_eth_ol(&self, ai: &AccountInfo) -> Result<(), String> {
        use std::str::FromStr;
        println!("INFO: Processing deposit: {:?}", ai);
        if ai.transfer_id.is_empty() {
            return Err(format!("Empty deposit id: {:?}", ai));
        }
        // Query unlocked
        let unlocked = self
            .query_unlocked()
            .map_err(|err| format!("Failed to get unlocked: {:?}", err))?;

        let unlocked_ai = unlocked
            .iter()
            .find(|x| x.transfer_id == ai.transfer_id)
            .and_then(|x| Some(x.clone()));
        if unlocked_ai.is_none() {
            let sender_this = AccountAddress::from_str(&ai.sender_this)
                .map_err(|err| format!("Failed to parse sender address: {:?}", err))?;

            // try to parse receiver address on 0L chain
            let receiver_this = match AccountAddress::from_str(&ai.receiver_this) {
                Ok(r) => Some(r),
                Err(err) => {
                    println!(
                        "WARN: cannot parse receiver_this address: {:?}",
                        err.to_string()
                    );
                    None
                }
            };

            // try to parse receiver address on ETH chain
            let receiver_eth = match hex_to_bytes(&ai.receiver_other)
                .map_err(|err| {
                    println!("{:?}", err);
                    err
                })
                .and_then(|v| bridge_ethers::util::vec_to_array::<u8, 20>(v))
                .map_err(|err| {
                    println!("Can't convert vector to array {:?}", err);
                    err
                })
                .and_then(|a| Ok(ethers::types::Address::from(a)))
            {
                Ok(r) => Some(r),
                _ => None,
            };

            let transfer_id = hex_to_bytes(&ai.transfer_id)
                .map_err(|err| {
                    println!("{:?}", err);
                    err
                })
                .and_then(|v| bridge_ethers::util::vec_to_array::<u8, 16>(v))
                .map_err(|err| {
                    println!("Can't convert vector to array {:?}", err);
                    err
                })?;

            // Transfer is not happened => transfer funds
            if receiver_this.is_some() {
                self.withdraw_ol_ol(ai, sender_this, receiver_this.unwrap(), transfer_id);
            } else if receiver_eth.is_some() {
                self.withdraw_ol_eth(ai, sender_this, receiver_eth.unwrap(), transfer_id)
            } else {
                println!("ERROR: receiver_this and receiver_eth are both empty, skip transfer");
            }
        }

        Ok(())
    }

    fn withdraw_ol_ol(
        &self,
        ai: &AccountInfo,
        sender_this: AccountAddress,
        receiver_this: AccountAddress,
        transfer_id: [u8; 16],
    ) {
        // transfer 0L->0L
        println!("INFO: withdraw from bridge, ai: {:?}", ai);
        let res = self.bridge_escrow_ol.bridge_withdraw(
            sender_this,
            Vec::new(),
            receiver_this,
            ai.balance,
            transfer_id.to_vec(),
            None,
        );
        if res.is_err() {
            println!("Failed to withdraw from escrow: {:?}", res.unwrap_err());
        } else {
            println!("INFO: withdraw from bridge: {:?}", res.unwrap());
        }
    }

    fn withdraw_eth_ol(
        &self,
        sender_other: Vec<u8>,
        receiver_this: AccountAddress,
        balance: u64,
        transfer_id: [u8; 16],
    ) {
        // transfer 0L->0L
        println!("INFO: withdraw from bridge, transfer_id: {:?}", transfer_id);
        let res = self.bridge_escrow_ol.bridge_withdraw(
            AccountAddress::ZERO,
            sender_other,
            receiver_this,
            balance,
            transfer_id.to_vec(),
            None,
        );
        if res.is_err() {
            println!("Failed to withdraw from escrow: {:?}", res.unwrap_err());
        } else {
            println!("INFO: withdraw from bridge: {:?}", res.unwrap());
        }
    }

    fn withdraw_ol_eth(
        &self,
        ai: &AccountInfo,
        sender_this: AccountAddress,
        receiver_eth: H160,
        transfer_id: [u8; 16],
    ) {
        // transfer 0L -> ETH
        match &self.agent_eth {
            Some(a) => {
                let rt = Runtime::new().unwrap();
                let handle = rt.handle();
                handle.block_on(async move {
                    let contract = BridgeEscrowEth::new(a.escrow_addr, &a.client);
                    let data = contract
                        .withdraw_from_escrow(
                            sender_this.to_u8(),
                            receiver_eth,
                            ai.balance,
                            transfer_id,
                        )
                        .gas_price(a.gas_price);
                    let pending_tx = data
                        .send()
                        .await
                        .map_err(|e| println!("Error pending: {}", e))
                        .unwrap();
                    println!("pending_tx: {:?}", pending_tx);
                });
            }
            _ => println!("Warn: agent_eth is not initialized"),
        }
    }

    fn close_eth_account(&self, transfer_id: [u8; 16]) {
        // close ETH transfer account
        match &self.agent_eth {
            Some(a) => {
                let rt = Runtime::new().unwrap();
                let handle = rt.handle();
                handle.block_on(async move {
                    let contract = BridgeEscrowEth::new(a.escrow_addr, &a.client);
                    let data = contract
                        .close_transfer_account_sender(transfer_id)
                        .gas_price(a.gas_price);
                    let pending_tx = data
                        .send()
                        .await
                        .map_err(|e| println!("Error pending: {}", e))
                        .unwrap();
                    println!("pending_tx: {:?}", pending_tx);
                });
            }
            _ => println!("Warn: agent_eth is not initialized"),
        }
    }

    fn query_locked_eth(&self, transfer_id: [u8; 16]) -> Result<AccountInfoEth, String> {
        match &self.agent_eth {
            Some(a) => {
                let rt = Runtime::new().unwrap();
                let handle = rt.handle();
                let mut res: Result<AccountInfoEth, String> =
                    Err(String::from("uninited contract"));
                handle.block_on(async {
                    let contract = BridgeEscrowEth::new(a.escrow_addr, &a.client);
                    let data = contract.get_locked_account_info(transfer_id);
                    res = data
                        .call()
                        .await
                        .map_err(|err| format!("ERROR: call: {:?}", err))
                        .and_then(|x| AccountInfoEth::from(x));
                });
                res
            }
            _ => Err(String::from("agent is not initialized")),
        }
    }

    fn query_unlocked_eth(&self, transfer_id: [u8; 16]) -> Result<AccountInfoEth, String> {
        match &self.agent_eth {
            Some(a) => {
                let rt = Runtime::new().unwrap();
                let handle = rt.handle();
                let mut res: Result<AccountInfoEth, String> =
                    Err(String::from("uninited contract"));
                handle.block_on(async {
                    let contract = BridgeEscrowEth::new(a.escrow_addr, &a.client);
                    let data = contract.get_unlocked_account_info(transfer_id);
                    res = data
                        .call()
                        .await
                        .map_err(|err| format!("ERROR: call: {:?}", err))
                        .and_then(|x| AccountInfoEth::from(x));
                });
                res
            }
            _ => Err(String::from("agent is not initialized")),
        }
    }

    fn get_next_locked_info(&self, start: U256, len: U256) -> Result<([u8; 16], U256), String> {
        match &self.agent_eth {
            Some(a) => {
                let rt = Runtime::new().unwrap();
                let handle = rt.handle();
                let mut res: Result<([u8; 16], U256), String> =
                    Err(String::from("uninited contract"));
                handle.block_on(async {
                    let contract = BridgeEscrowEth::new(a.escrow_addr, &a.client);
                    let data = contract.get_next_transfer_id(start, len);
                    res = data
                        .call()
                        .await
                        .map_err(|err| format!("ERROR: call: {:?}", err));
                });
                res
            }
            _ => Err(String::from("agent is not initialized")),
        }
    }

    /// Process autstanding transfers
    pub fn process_deposits_ol_eth(&self) {
        println!("INFO: process deposits");
        let ais = self.query_locked();
        if ais.is_err() {
            println!("WARN: Failed to get locked: {}", ais.unwrap_err());
            return;
        }
        for ai in ais.unwrap() {
            match self.process_deposit_ol_eth(&ai) {
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
    fn process_deposit_ol_eth(&self, ai: &AccountInfo) -> Result<(), String> {
        use std::str::FromStr;
        println!("INFO: Processing deposit: {:?}", ai);
        if ai.transfer_id.is_empty() {
            return Err(format!("ERROR: Empty deposit id: {:?}", ai));
        }

        let transfer_id = hex_to_bytes(&ai.transfer_id)
            .and_then(|v| bridge_ethers::util::vec_to_array::<u8, 16>(v))?;

        // Query unlocked on ETH
        let unlocked: AccountInfoEth = self.query_unlocked_eth(transfer_id.clone())?;
        if unlocked.transfer_id == [0u8; 16] {
            let sender_this =
                AccountAddress::from_str(&ai.sender_this).map_err(|err| err.to_string())?;

            // try to parse receiver address on ETH chain
            let receiver_eth = hex_to_bytes(&ai.receiver_other)
                .and_then(|v| bridge_ethers::util::vec_to_array::<u8, 20>(v))
                .and_then(|a| Ok(ethers::types::Address::from(a)))?;

            // Transfer is not happened => transfer funds
            self.withdraw_ol_eth(ai, sender_this, receiver_eth, transfer_id);
        } else {
            println!(
                "Withdrawal for transfer_id {:?} has been made on ETH",
                ai.transfer_id
            );
            // Query locked
            let locked = self.query_locked()?;

            // Transfer happened , remove locked
            let locked_ai = locked
                .iter()
                .find(|x| x.transfer_id == ai.transfer_id)
                .and_then(|x| Some(x.clone()));
            if locked_ai.is_some() {
                println!("INFO: remove locked: {:?}", locked_ai);
                let res = self.bridge_escrow_ol.bridge_close_transfer(
                    &transfer_id.to_vec(),
                    false, //close_other
                    None,
                );
                if res.is_err() {
                    return Err(format!("Failed to remove locked: {:?}", res.unwrap_err()));
                }
                println!("INFO: removed locked: {:?}", res.unwrap());
            }
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
            account: self.bridge_escrow_ol.escrow.clone(),
            module_name: String::from("BridgeEscrow"),
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

fn hex_to_bytes(s: &String) -> Result<Vec<u8>, String> {
    if s.len() % 2 == 0 {
        match (0..s.len())
            .step_by(2)
            .map(|i| {
                s.get(i..i + 2)
                    .and_then(|sub| u8::from_str_radix(sub, 16).ok())
            })
            .collect()
        {
            Some(r) => Ok(r),
            _ => Err(format!("Cannot conver string {} to hex", s)),
        }
    } else {
        Err(format!("Can't conver string {:?} to hex", s))
    }
}
