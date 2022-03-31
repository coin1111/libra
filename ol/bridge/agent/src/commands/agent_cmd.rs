//! `agent` subcommand

use crate::agent::Agent;
use crate::{entrypoint, node::client, node::node::Node, prelude::app_config};
use abscissa_core::{Command, Options, Runnable};
use std::env;
use std::process::exit;
use std::str::FromStr;
use std::{thread, time::Duration};
use tokio::runtime::Runtime;

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
        let _ = Runtime::new().unwrap();
        let args = entrypoint::get_args();
        let is_swarm = *&args.swarm_path.is_some();
        let mut cfg = app_config().clone();
        let ol_escrow = if args.ol_escrow_account.is_some() {
            args.ol_escrow_account.unwrap()
        } else {
            cfg.profile.account
        };

        let ol_client =
            client::pick_client(args.swarm_path.clone(), &mut cfg).unwrap_or_else(|e| {
                println!("ERROR: Cannot connect to a client. Message: {}", e);
                exit(1);
            });

        // Eth config
        let config_eth = (match args.eth_escrow_config {
            Some(path) => bridge_ethers::config::Config::new(path.as_str()),
            None => env::var("ETH_BRIDGE_ESCROW_CONFIG")
                .map_err(|e| {
                    format!(
                        "cannot read eth config from env var ETH_BRIDGE_ESCROW_CONFIG, err: {:?}",
                        e
                    )
                })
                .and_then(|x| bridge_ethers::config::Config::new(x.as_str())),
        })
        .map_err(|e| {
            println!(
                "WARN: cannot read read ETH config: {:?}. Will run in 0L mode",
                e
            );
            e
        });

        // ETH agent account
        let account_eth = match env::var("ETH_BRIDGE_ESCROW_ACCOUNT")
            .map_err(|e| {
                format!(
                    "cannot read eth account from env var ETH_BRIDGE_ESCROW_ACCOUNT, err: {:?}",
                    e
                )
            })
            .and_then(|account_str| {
                bridge_ethers::signers::get_private_key(&account_str).and_then(|x| {
                    ethers::signers::Wallet::from_str(&x[2..]).map_err(|e| e.to_string())
                })
            }) {
            Ok(a) => Some(a),
            Err(err) => {
                println!("WARN: failed to create ETH account wallet: {:?}", err);
                None
            }
        };

        let agent = Agent::new(
            ol_escrow,
            Node::new(ol_client, &cfg, is_swarm),
            config_eth.map_or_else(|_| None, |x| Some(x)),
            account_eth,
        ).unwrap_or_else(|err|{
            println!("ERROR: failed to create agent, error: {}",err);
            exit(1);
        });

        loop {
            // 0L->0L
            let _ = agent.process_transfers_eth().
                map_err(|err|println!("ERROR: failed to process eth deposits, error: {:?}",err));
            thread::sleep(Duration::from_millis(10000));

            // 0L->ETH
            let _ = agent.process_transfers_ol()
                .map_err(|err|println!("ERROR: failed to process 0L deposits, error: {:?}",err));
            thread::sleep(Duration::from_millis(10000));
        }
    }
}

impl AgentCmd {}
