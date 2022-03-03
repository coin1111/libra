//! `agent` subcommand

use crate::{agent, entrypoint, node::client, node::node::Node, prelude::app_config};
use abscissa_core::{Command, Options, Runnable};
use std::process::exit;
use std::{thread, time::Duration};

/// `agent` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Default, Options)]
pub struct AgentCmd {}

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
        let agent = agent::Agent {
            node: Node::new(client, &cfg, is_swarm),
            escrow: account,
        };
        loop {
            agent.process_transfers();
            thread::sleep(Duration::from_millis(1000));
        }
    }
}
