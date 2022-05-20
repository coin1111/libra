//! `deposit-funds` subcommand

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
/// `DepositFunds` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct DepositFundsCmd {
    #[options(short = "e", help = "escrow address address")]
    escrow_account: String,
    #[options(short = "c", help = "the amount of coins to send to escrow, scaled")]
    coins: u64,
}

impl Runnable for DepositFundsCmd {
    fn run(&self) {
        let entry_args = entrypoint::get_args();
        let escrow = match self.escrow_account.parse::<AccountAddress>() {
            Ok(a) => a,
            Err(e) => {
                println!(
                    "ERROR: could not parse this account address: {}, message: {}",
                    self.escrow_account,
                    &e.to_string()
                );
                exit(1);
            }
        };

        match deposit_funds(escrow, self.coins, entry_args.save_path) {
            Ok(_) => println!("Success: Bridge deposit posted: {}", self.escrow_account),
            Err(e) => {
                println!("ERROR: execute bridge deposit message: {:?}", &e);
                exit(1);
            }
        }
    }
}

/// Deposit funds into escrow account
pub fn deposit_funds(
    escrow: AccountAddress,
    coins: u64,
    save_path: Option<PathBuf>,
) -> Result<TransactionView, TxError> {
    let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();
    // coins are scaled
    let script = transaction_builder::encode_bridge_multisig_deposit_funds_script_function(escrow, coins);
    maybe_submit(script, &tx_params, save_path)
}
