//! `bridge-deposit` subcommand

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
/// `BridgeDeposit` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct BridgeDepositCmd {
    #[options(short = "e", help = "escrow address address")]
    escrow_account: String,
    #[options(short = "r", help = "receiver address")]
    receiver_account: String,
    #[options(short = "o", help = "receiver address on the other chain")]
    receiver_other: String,
    #[options(short = "c", help = "the amount of coins to send to escrow, scaled")]
    coins: u64,
    #[options(short = "t", help = "transfer id")]
    transfer_id: String,
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

impl Runnable for BridgeDepositCmd {
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
        let receiver = match self.receiver_account.parse::<AccountAddress>() {
            Ok(a) => a,
            Err(e) => {
                println!(
                    "ERROR: could not parse this account address: {}, message: {}",
                    self.receiver_account,
                    &e.to_string()
                );
                exit(1);
            }
        };

        let receiver_other = !if self.receiver_other.empty() {
            match hex_to_bytes(&self.receiver_other) {
                Some(a) => a,
                None => {
                    println!(
                        "ERROR: could not parse this receiver_other: {}",
                        self.receiver_other
                    );
                    exit(1);
                }
            }
        } else {
            Vec::new();
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

        match bridge_deposit(
            escrow,
            receiver,
            receiver_other,
            self.coins,
            transfer_id,
            entry_args.save_path,
        ) {
            Ok(_) => println!("Success: Bridge deposit posted: {}", self.escrow_account),
            Err(e) => {
                println!("ERROR: execute bridge deposit message: {:?}", &e);
                exit(1);
            }
        }
    }
}

/// Deposit into escrow account
pub fn bridge_deposit(
    escrow: AccountAddress,
    receiver: AccountAddress,
    receiver_other: Vec<u8>,
    coins: u64,
    transfer_id: Vec<u8>,
    save_path: Option<PathBuf>,
) -> Result<TransactionView, TxError> {
    let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();
    // coins are scaled
    let script = transaction_builder::encode_bridge_deposit_script_function(
        escrow,
        receiver,
        receiver_other,
        coins,
        transfer_id,
    );
    maybe_submit(script, &tx_params, save_path)
}
