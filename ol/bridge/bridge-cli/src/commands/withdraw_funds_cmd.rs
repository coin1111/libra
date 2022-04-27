//! `withdraw-funds` subcommand

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
/// `WithdrawFunds` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct WithdrawFundsCmd {
    #[options(short = "e", help = "escrow address")]
    escrow: String,

    #[options(short = "r", help = "receiver")]
    receiver: String,

    #[options(short = "b", help = "balance")]
    balance: u64,
}

impl Runnable for WithdrawFundsCmd {
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

        let receiver = match self.receiver.parse::<AccountAddress>() {
            Ok(a) => a,
            Err(e) => {
                println!(
                    "ERROR: could not parse this account address: {}, message: {}",
                    self.receiver,
                    &e.to_string()
                );
                exit(1);
            }
        };

        match withdraw_funds(escrow, receiver, self.balance, entry_args.save_path) {
            Ok(_) => println!("Success: Bridge withdraw posted: {}", receiver),
            Err(e) => {
                println!("ERROR: execute bridge withdraw message: {:?}", &e);
                exit(1);
            }
        }
    }
}

/// withdraw into escrow account
pub fn withdraw_funds(
    escrow: AccountAddress,
    receiver: AccountAddress,
    balance: u64,
    save_path: Option<PathBuf>,
) -> Result<TransactionView, TxError> {
    let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();
    // coins are scaled
    let script =
        transaction_builder::encode_bridge_withdraw_funds_script_function(escrow, receiver, balance);
    maybe_submit(script, &tx_params, save_path)
}
