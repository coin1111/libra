//! Bridge agent
use crate::entrypoint::tx_params_wrapper;
use crate::util::{read_eth_checkpoint, save_eth_checkpoint};
use crate::{node::node::Node, node::query::QueryType};
use bridge_eth::bridge_escrow_multisig_mod::BridgeEscrowMultisig as BridgeEscrowEth;
use bridge_eth::config::Config;
use bridge_eth::util::AccountInfo as AccountInfoEth;
use bridge_ol::contract::BridgeEscrowMultisig;
use ethers::prelude::{ H160, Wallet};
use ethers::types::{Address, U256};
use move_core_types::account_address::AccountAddress;
use ol_types::config::TxType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use crate::transfer::agent_eth::{AgentEth, EthLockedInfo};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AccountInfo {
    sender_this: String,
    sender_other: String,
    receiver_this: String,
    receiver_other: String,
    balance: u64,
    transfer_id: String,
    votes: Vec<String>,
    current_votes: u64,
    is_closed: bool,
}

/// Bridge processor struct
pub struct Processor {
    /// Node to connect to blockchain
    pub node_ol: Node,

    /// BridgeEscrow contract for 0L
    pub bridge_escrow_ol: BridgeEscrowMultisig,

    agent_eth: AgentEth,
}

impl Processor {
    /// Create a new bridge agent
    pub fn new(
        ol_escrow: AccountAddress,
        node_ol: Node,
        config_eth: Option<Config>,
        agent_eth: Option<Wallet>,
    ) -> Result<Processor, String> {
        let agent_eth = AgentEth::new(&config_eth, &agent_eth)?;
        let tx_params = tx_params_wrapper(TxType::Mgmt).map_err(|err| err.to_string())?;

        Ok(Processor {
            node_ol,
            bridge_escrow_ol: BridgeEscrowMultisig::new(ol_escrow, tx_params).unwrap(),
            agent_eth,
        })
    }

    /// Process outstanding transfers
    pub async fn process_transfers_eth(&mut self) -> Result<(), String> {
        println!("INFO: process deposits from ETH to 0L");
        // use checkpoint to get start element
        let start_idx = read_eth_checkpoint();

        // Query unlocked on ETH
        let start = U256::from(start_idx);
        let len: U256 = U256::from(10);
        let locked_eth = self.agent_eth.get_eth_next_locked_info(start, len).await?;

        println!("INFO: next locked on ETH chain : {}", locked_eth);
        if locked_eth.transfer_id == [0u8; 16] {
            // transfer_id is 0, nothing to do
            return Ok(());
        }
        self.process_transfer_eth(locked_eth).await
    }

    /// Process transfer
    /// 1. check if account on ETH chain is not processed (not closed) in locked struct
    /// 2. check in unlocked struct exists on on 0L chain for this transfer_id. If exists, then
    /// account on ETH can be closed for this transfer. If unlcoked entry doesn't exist on 0L,
    /// then make a withdrawal on 0L, which will create an entry in unlocked struct on 0L.
    /// 3. If locked entry is marked as closed on ETH chain then unlocked entry can be removed on 0L chain,
    /// this completes a transfer on both chains.
    async fn process_transfer_eth(&mut self, locked_eth: EthLockedInfo) -> Result<(), String> {
        println!(
            "INFO: processing transfer_id: {:?} on ETH chain",
            hex::encode(locked_eth.transfer_id)
        );
        // Check if this transfer is processed already,
        // e.g. locked entry on ETH chain is marked as closed
        let locked_ai = self.agent_eth.query_eth_locked(locked_eth.transfer_id).await?;
        if locked_ai.is_closed {
            println!("INFO: transfer is processed already: {:?}", locked_ai);
            return Ok(());
        }

        let transfer_id_str = hex::encode(locked_eth.transfer_id);

        println!("INFO: Processing ETH transfer: {:?}", locked_ai);

        // Locked entry on ETH side is not closed
        // Check if corresponding unlocked exists on 0L chain.
        // this means that withdrawal to a recepient on 0L has been made already
        // and we need to cleanup this transfer account on both chains - 1) close locked entry on ETH
        // and 2) remove unlocked entry on 0L strictly in this order
        let unlocked_ol_exists = self.query_ol_unlocked().and_then(|v| {
            Ok(v.iter()
                .find(|ai| ai.transfer_id == transfer_id_str)
                .and_then(|ai| Some(ai.clone())))
        })?;
        let is_closed = match &unlocked_ol_exists {
            Some(v) => v.is_closed,
            _ => false,
        };
        if unlocked_ol_exists.is_some()  && is_closed {
            if Self::is_voted_eth(&locked_ai, self.agent_eth.client.address()) {
                // skip if this agent voted on ETH side already
                return Ok(());
            }
            let unlocked_ol = &unlocked_ol_exists.unwrap();

            // Unlocked entry already exists on 0L chain and is closed, which means that
            // withdrawal has been made already,
            // thus mark ETH side as completed
            println!(
                "INFO: 0L unlocked entry exists for transfer_id on 0L {}, close transfer account on ETH chain",
                transfer_id_str
            );
            // Mark ETH entry as completed
            self.close_eth_account(locked_eth.transfer_id).await?;

            // Query ETH locked account we just closed.
            // Note we don't rely on success or failure of close_eth_account()
            // instead we directly query ETH chain to ensure that account is indeed closed.
            let ai_eth = self.agent_eth.query_eth_locked(locked_eth.transfer_id).await?;
            if !ai_eth.is_closed {
                return Ok(());
            }
            // Now that ETH account is closed
            // we can remove corresponding unlocked entry on 0L chain
            println!(
                "INFO: ETH account is closed for transfer_id: {:?}",
                hex::encode(locked_eth.transfer_id)
            );

            return self.remove_unlocked_ol(&unlocked_ol, &locked_eth, transfer_id_str);
        } else {
            return self.withdraw_ol(locked_ai, &unlocked_ol_exists);
        }
    }

    fn is_voted_eth(locked_ai: &AccountInfoEth, address_eth: Address) -> bool {
        locked_ai.votes.iter().find(|x| **x == address_eth).is_some()
    }

    fn remove_unlocked_ol(&mut self, ai_ol: &AccountInfo, locked_eth: &EthLockedInfo, transfer_id_str: String) -> Result<(), String> {
        // remove 0L unlocked entry
        println!("INFO: will close unlocked 0L account for transfer_id: {:?}", transfer_id_str);
        // if account voted, skip
        if Self::is_voted_ol(ai_ol, self.node_ol.app_conf.profile.account) {
            return Ok(());
        }
        self.bridge_escrow_ol.bridge_close_transfer(
            &locked_eth.transfer_id.to_vec(),
            true, // close_other=true => remove unlocked entry
            None,
        )
            .map_err(|err| format!("ERROR: failed to remove locked: {:?}", err))
            .map(|tx| {
                println!("INFO: closed unlocked 0L account for transfer_id: {:?}, tx: {:?}",
                         transfer_id_str, tx)
            })?;

        // Save checkpoint of the last transfer id processed to a file
        // so that the next time we know where to start searching for unprocessed transfers
        return save_eth_checkpoint(*locked_eth);
    }

    fn withdraw_ol(&mut self, locked_ai: AccountInfoEth, unlocked_ol_exists: &Option<AccountInfo>) -> Result<(), String>{
        // unlocked entry on 0L doesn't exist or not yet closed
        // if already voted then skip
        if unlocked_ol_exists.as_ref().and_then(|ai| {
            Some(Self::is_voted_ol(ai,
                              self.node_ol.app_conf.profile.account))
        }).is_some() {
            return Ok(());
        }

        // transfer fund on 0L
        let receiver_this =
            AccountAddress::from_bytes(locked_ai.receiver_other).map_err(|err| {
                format!(
                    "cannot parse receiver_other: {:?}, error: {:?}",
                    locked_ai.receiver_other, err
                )
            })?;

        println!("INFO: withdraw from bridge on 0L chain, transfer_id: {:?}, from: {:?}, to {:?}, amount: {:?}",
                 locked_ai.transfer_id,
                 hex::encode(locked_ai.sender_this.clone()), receiver_this, locked_ai.balance);
        self.bridge_escrow_ol
            .bridge_withdraw(
                locked_ai.sender_this.as_bytes().to_vec(),
                receiver_this,
                locked_ai.balance,
                locked_ai.transfer_id.to_vec(),
                None,
            )
            .map_err(|err| format!("ERROR: 0L chain bridge_withdraw, error: {:?}", err))
            .map(|tx| println!("INFO: 0L transaction: {:?}", tx))
    }

    async fn withdraw_eth_aux(
        &self,
        ai: &AccountInfo,
        sender_this: AccountAddress,
        receiver_eth: H160,
        transfer_id: [u8; 16],
    ) -> Result<(), String> {
        // transfer 0L -> ETH
        let contract = BridgeEscrowEth::new(self.agent_eth.escrow_addr, &self.agent_eth.client);
        let data = contract
            .withdraw_from_escrow(sender_this.to_u8(), receiver_eth, ai.balance, transfer_id)
            .gas_price(self.agent_eth.gas_price);
        data.send()
            .await
            .map_err(|e| format!("failed withdraw from 0L: {:?}", e))
            .map(|tx| println!("INFO: withdraw from 0L, tx: {:?}", tx))
    }

    async fn close_eth_account(&self, transfer_id: [u8; 16]) -> Result<(), String> {
        // close ETH transfer account
        let contract = BridgeEscrowEth::new(self.agent_eth.escrow_addr, &self.agent_eth.client);
        let data = contract
            .close_transfer_account_sender(transfer_id)
            .gas_price(self.agent_eth.gas_price);
        data.send()
            .await
            .map_err(|e| format!("Error pending: {}", e))
            .map(|tx| println!("INFO: transaction: {:?}", tx))
    }

    /// Process autstanding transfers
    pub async fn process_transfers_ol(&mut self) -> Result<(), String> {
        println!("INFO: process 0L transfers");
        let ais = self.query_ol_locked()?;

        // Process one entry at a time
        match ais.get(0) {
            Some(ai) => self
                .process_transfer_ol(&ai)
                .await
                .map(|_| println!("INFO: Succesfully processed 0L transfer: {:?}", ai)),
            _ => Ok(()),
        }
    }

    /// Process individual transfer
    // Transfer deposit from escrow to destination receiver
    // Ensure that unlocked doesn't have an entry for this transfer
    // This indicates that transfer has not been made, thus proceed with transfer
    async fn process_transfer_ol(&mut self, ai: &AccountInfo) -> Result<(), String> {
        println!("INFO: Processing deposit: {:?}", ai);
        if ai.transfer_id.is_empty() {
            return Err(format!("ERROR: Empty deposit id: {:?}", ai));
        }

        let transfer_id = hex_to_bytes(&ai.transfer_id)
            .and_then(|v| bridge_eth::util::vec_to_array::<u8, 16>(v))?;

        // Query unlocked on ETH
        let unlocked_eth: AccountInfoEth = self.agent_eth.query_eth_unlocked(transfer_id.clone()).await?;
        // Withdraw funds on ETH if unlocked entry on ETH is not present
        if unlocked_eth.transfer_id == [0u8; 16] || !unlocked_eth.is_closed {
            return self.withdraw_eth(&ai, transfer_id, unlocked_eth).await;
        } else {
            // Unlocked entry exists on ETH chain, can remove locked entry on 0L chain now
            println!(
                "INFO: withdrawal for transfer_id {:?} has been made on ETH, remove unlocked entry on 0L",
                ai.transfer_id
            );
            // Query locked entries on 0L chain
            let locked_ol_entries = self.query_ol_locked()?;

            // Find entry for this transfer id
            let locked_ol = locked_ol_entries
                .iter()
                .find(|x| x.transfer_id == ai.transfer_id);

            // if entry is removed, nothing to do
            if locked_ol.is_none() {
                return Ok(());
            }

            return self.close_account_ol(&transfer_id, locked_ol);
        }
    }

    async fn withdraw_eth(&mut self, ai: &&AccountInfo, transfer_id: [u8; 16], unlocked_eth: AccountInfoEth) -> Result<(), String>{
        // if voted already then skip
        if unlocked_eth
            .votes
            .iter()
            .find(|x| **x == self.agent_eth.client.address())
            .is_none() {
            return Ok(());
        }

        let sender_this =
            AccountAddress::from_str(&ai.sender_this).map_err(|err| err.to_string())?;

        // try to parse receiver address on ETH chain
        let receiver_eth = hex::decode(&ai.receiver_other)
            .map_err(|err| err.to_string())
            .and_then(|v| bridge_eth::util::vec_to_array::<u8, 20>(v))
            .and_then(|a| Ok(ethers::types::Address::from(a)))?;

        // Transfer is not happened => transfer funds on ETH chain
        println!(
            "INFO: invoke withdraw_eth for transfer_idL {:?} by agent {:?}",
            transfer_id,
            self.agent_eth.client.address()
        );
        return self
            .withdraw_eth_aux(ai, sender_this, receiver_eth, transfer_id)
            .await;
    }

    fn close_account_ol(&mut self, transfer_id: &[u8; 16], locked_ol: Option<&AccountInfo>) -> Result<(), String> {
        println!("INFO: remove locked on 0L: {:?}", locked_ol.unwrap());
        // if this agent voted, skip
        if Self::is_voted_ol(locked_ol.unwrap(), self.node_ol.app_conf.profile.account) {
            return Ok(());
        }

        // otherwise vote
        return self
            .bridge_escrow_ol
            .bridge_close_transfer(
                &transfer_id.to_vec(),
                false, //close_other = false -> remove locked entry
                None,
            )
            .map_err(|err| format!("Failed to remove locked: {:?}", err))
            .map(|tx| {
                println!(
                    "INFO: removed unlocked entry on 0L chain for {:?}, tx: {:?}",
                    hex::encode(transfer_id),
                    tx
                )
            });
    }

    fn is_voted_ol(ai: &AccountInfo, agent_address: AccountAddress) -> bool {
        ai.votes.iter().find(|x| {
            match (**x).parse::<AccountAddress>()
                .and_then(|v| Ok(v == agent_address)) {
                Ok(a) => a,
                _ => false,
            }
        }).is_some()
    }

    fn query_ol_locked(&mut self) -> Result<Vec<AccountInfo>, String> {
        return self.query_account_info("locked");
    }
    fn query_ol_unlocked(&mut self) -> Result<Vec<AccountInfo>, String> {
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
    fn query_account_info(&mut self, field_name: &str) -> Result<Vec<AccountInfo>, String> {
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
