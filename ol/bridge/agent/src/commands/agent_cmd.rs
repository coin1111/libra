//! `bal` subcommand

use crate::{
    entrypoint, node::client, node::node::Node, node::query::QueryType, prelude::app_config,
};
use abscissa_core::{Command, Options, Runnable};
use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::exit;
use std::{thread, time::Duration};

/// `bal` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Default, Options)]
pub struct AgentCmd {
}

pub struct Agent {
    node: Node,
    escrow: AccountAddress,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountInfo {
    sender_this: String,
    sender_other: String,
    receiver_this: String,
    receiver_other: String,
    balance: u64,
    transfer_id: String,
}

impl Runnable for AgentCmd {
    fn run(& self) {
        let args = entrypoint::get_args();
        let is_swarm = *&args.swarm_path.is_some();
        let mut cfg = app_config().clone();
        let account = if args.account.is_some() {
            args.account.unwrap()
        } else {
            cfg.profile.account
        };

        let client = client::pick_client(args.swarm_path.clone(), &mut cfg).unwrap_or_else(|e| {
            println!("ERROR: Cannot connect to a client. Message: {}", e);
            exit(1);
        });
        let agent = Agent{
            node: Node::new(  client, &cfg, is_swarm),
            escrow: account,
        };
        loop {
            agent.process_transfers();
            thread::sleep(Duration::from_millis(1000));
        }
    }
}

impl Agent {
    pub fn process_transfers(&self) {
        let ais = self.query_locked();
        if ais.is_err() {
            println!("WARN: Failed to get locked: {}",ais.unwrap_err());
            return
        }
        for ai in ais.unwrap() {
            match self.process_transfer( &ai)  {
                Ok(()) => println!("INFO: Succesfully processed transfer: {}",ai.transfer_id),
                Err(err) => println!("ERROR: Failed to process transfer: {}, error: {}", ai.transfer_id, err)
            }
        }
    }

    // Process transfer as follows
    // 1. Check transfer_id entry in unlocked. If this entry exists that means that withdrawal
    // has been made already. At this point we can close locked entry and remove transfer_id
    // from pending transfers in locked_idx
    // 2. If unlocked has no entry for given transfer_id, that means that withdrawal didn't happen. Thus we need
    // to withdraw funds into user account and then repeat step 1. above
    fn process_transfer(&self,ai:&AccountInfo) ->Result<(),String> {
        println!("INFO: Processing transfer: {:?}",ai);
        // Query unlocked
        let unlocked = self.query_unlocked();
        if unlocked.is_err() {
            return Err(format!("ERROR: Failed to get unlocked: {}",unlocked.unwrap_err()))
        }
        Ok(())
    }
    fn query_locked(&self) -> Result<Vec<AccountInfo>, String> {
        return self.query_account_info("locked")
    }
    fn query_unlocked(&self) -> Result<Vec<AccountInfo>, String> {
        return self.query_account_info("unlocked")
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
    fn query_account_info(&self,field_name: &str) -> Result<Vec<AccountInfo>, String> {
        let query_type = QueryType::MoveValue {
            account: self.escrow.clone(),
            module_name: String::from("BridgeEscrow"),
            struct_name: String::from("EscrowState"),
            key_name:  String::from(field_name),
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
