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
/// `CreateAccount` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct BridgeCreateEscrowCmd {
}

impl Runnable for BridgeCreateEscrowCmd {
    fn run(&self) {
        let entry_args = entrypoint::get_args();
        match bridge_create_escrow(entry_args.save_path) {
            Ok(_) => println!(
                "Success: Create bridge escrow"
            ),
            Err(e) => {
                println!("ERROR: execute create bridge escrow message: {:?}", &e);
                exit(1);
            }
        }
    }
}

/// create an account by sending coin to it
pub fn bridge_create_escrow(
    save_path: Option<PathBuf>,
) -> Result<TransactionView, TxError> {
    let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();

    // NOTE: coins here do not have the scaling factor. Rescaling is the responsibility of the Move script. See the script in ol_accounts.move for detail.
    let script =
        transaction_builder::encode_bridge_create_escrow_script_function();

    maybe_submit(script, &tx_params, save_path)
}
