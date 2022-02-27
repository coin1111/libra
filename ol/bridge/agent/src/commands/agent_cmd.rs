//! `bal` subcommand

use crate::{
    entrypoint, node::client, node::node::Node, node::query::QueryType, prelude::app_config,
};
use abscissa_core::{status_info, Command, Options, Runnable};
use std::process::exit;
use serde_json::json;

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

impl Runnable for AgentCmd {
    fn run(&self) {
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
        let mut node = Node::new(client, &cfg, is_swarm);
        let query_type =  QueryType::MoveValue {
            account,
            module_name: String::from("BridgeEscrow"),
            struct_name: String::from("EscrowState"),
            key_name: String::from("locked"),
        };
        let display = "RESOURCES";

        match node.query_locked(query_type) {
            Ok(info) => {
                let js = info.replace('\n',"");
                let value = json!(js);
                println!("info: {:?}",value.get(0));
            },
            Err(e) => {
                println!("could not query node, exiting. Message: {:?}", e);
            }
        };
    }
}
