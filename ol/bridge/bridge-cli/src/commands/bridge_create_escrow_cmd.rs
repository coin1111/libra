//! `bridge-create-escrow` subcommand

#![allow(clippy::never_loop)]

use crate::{
    entrypoint,
    submit_tx::{maybe_submit, tx_params_wrapper, TxError},
};
use abscissa_core::{Command, Options, Runnable};

use diem_json_rpc_types::views::TransactionView;
use diem_transaction_builder::stdlib as transaction_builder;
use ol_types::config::TxType;
use std::{path::PathBuf, process::exit};
use move_core_types::account_address::AccountAddress;

/// `CreateAccount` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct BridgeCreateEscrowCmd {
    #[options(short = "1", help = "executor1 address")]
    executor1: String,
    #[options(short = "2", help = "executor2 address")]
    executor2: String,
    #[options(short = "3", help = "executor3 address")]
    executor3: String,
    #[options(short = "4", help = "executor4 address")]
    executor4: String,
    #[options(short = "5", help = "executor5 address")]
    executor5: String,
    #[options(short = "v", help = "min votes")]
    min_votes: u64,
}

impl Runnable for BridgeCreateEscrowCmd {
    fn run(&self) {
        let entry_args = entrypoint::get_args();
        let mut executors = Vec::new();
        match self.executor1.parse::<AccountAddress>() {
            Ok(a) => executors.push(a),
            _ => executors.push(AccountAddress::ZERO),
        };
        match self.executor2.parse::<AccountAddress>() {
            Ok(a) => executors.push(a),
            _ => executors.push(AccountAddress::ZERO),
        };
        match self.executor3.parse::<AccountAddress>() {
            Ok(a) => executors.push(a),
            _ => executors.push(AccountAddress::ZERO),
        };
        match self.executor4.parse::<AccountAddress>() {
            Ok(a) => executors.push(a),
            _ => executors.push(AccountAddress::ZERO),
        };

        match self.executor5.parse::<AccountAddress>() {
            Ok(a) => executors.push(a),
            _ => executors.push(AccountAddress::ZERO),
        };

        if (executors.len() as u64) < self.min_votes {
            println!("ERROR: number of executors must be greater or equal to min_votes: {:?}", executors.len());
            exit(1);
        }

        match bridge_create_escrow(executors, self.min_votes,entry_args.save_path) {
            Ok(_) => println!(
                "Success: Create bridge escrow"
            ),
            Err(e) => {
                println!("ERROR: execute create bridge escrow message: {:?}", &e);
                exit(1);
            }
        };
    }
}

/// create an account by sending coin to it
pub fn bridge_create_escrow(
    executors: Vec<AccountAddress>,
    min_votes: u64,
    save_path: Option<PathBuf>,
) -> Result<TransactionView, TxError> {
    let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();

    // NOTE: coins here do not have the scaling factor. Rescaling is the responsibility of the Move script. See the script in ol_accounts.move for detail.
    let script =
        transaction_builder::encode_bridge_multisig_create_escrow_script_function(
            executors[0],
            executors[1],
            executors[2],
            executors[3],
            executors[4],
            min_votes
        );

    maybe_submit(script, &tx_params, save_path)
}
