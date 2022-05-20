//! `bridge-close-transfer` subcommand

#![allow(clippy::never_loop)]

use crate::{
    entrypoint,
    submit_tx::{maybe_submit, tx_params_wrapper, TxError},
};
use abscissa_core::{Command, Options, Runnable};

use diem_json_rpc_types::views::TransactionView;
use diem_transaction_builder::stdlib as transaction_builder;
use diem_types::account_address::AccountAddress;
use ol_types::config::TxType;
use std::{path::PathBuf, process::exit};
/// `BridgeCloseTransfer` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct BridgeCloseTransferCmd {
    #[options(short = "e", help = "escrow address")]
    escrow: String,

    #[options(short = "t", help = "transfer id")]
    transfer_id: String,

    #[options(short = "o", help = "close transfer on the other chain")]
    close_other: bool,
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

impl Runnable for BridgeCloseTransferCmd {
    fn run(&self) {
        let entry_args = entrypoint::get_args();
        let escrow = match self.escrow.parse::<AccountAddress>() {
            Ok(a) => a,
            Err(e) => {
                println!(
                    "ERROR: could not parse this account address: {}, message: {}",
                    self.escrow,
                    &e.to_string()
                );
                exit(1);
            }
        };
        let transfer_id = match hex_to_bytes(&self.transfer_id) {
            Some(a) => a,
            None => {
                println!(
                    "ERROR: could not parse this transfer_id: {}",
                    self.transfer_id
                );
                exit(1);
            }
        };

        match bridge_close_transfer(escrow, transfer_id, self.close_other, entry_args.save_path) {
            Ok(_) => println!("Success: Bridge withdraw posted: {}", self.transfer_id),
            Err(e) => {
                println!("ERROR: execute bridge close transfer message: {:?}", &e);
                exit(1);
            }
        }
    }
}

/// close escrow account
pub fn bridge_close_transfer(
    escrow: AccountAddress,
    transfer_id: Vec<u8>,
    close_other: bool,
    save_path: Option<PathBuf>,
) -> Result<TransactionView, TxError> {
    let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();
    // coins are scaled
    let script = transaction_builder::encode_bridge_multisig_close_transfer_script_function(
        escrow,
        transfer_id,
        close_other,
    );
    maybe_submit(script, &tx_params, save_path)
}
