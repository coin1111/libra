//! `bridge-withdraw` subcommand

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
/// `BridgeWithdraw` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct BridgeWithdrawCmd {
    #[options(short = "e", help = "escrow address")]
    escrow: String,

    #[options(short = "s", help = "sender on this chain")]
    sender: String,

    #[options(short = "o", help = "sender on the other chain")]
    sender_other: String,

    #[options(short = "r", help = "receiver")]
    receiver: String,

    #[options(short = "b", help = "balance")]
    balance: u64,

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

impl Runnable for BridgeWithdrawCmd {
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

        let sender = if !self.sender.is_empty() {
            match self.sender.parse::<AccountAddress>() {
                Ok(a) => a,
                Err(e) => {
                    println!(
                        "ERROR: could not parse this account address: {}, message: {}",
                        self.sender,
                        &e.to_string()
                    );
                    exit(1);
                }
            }
        } else {
            AccountAddress::ZERO
        };

        let sender_other = if !self.sender_other.is_empty() {
            match hex_to_bytes(&self.sender_other) {
                Some(a) => a,
                None => {
                    println!(
                        "ERROR: could not parse this sender_other: {}",
                        self.sender_other
                    );
                    exit(1);
                }
            }
        } else {
            Vec::new()
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

        match bridge_withdraw(escrow, sender, sender_other, receiver, self.balance, transfer_id, entry_args.save_path) {
            Ok(_) => println!("Success: Bridge withdraw posted: {}", self.transfer_id),
            Err(e) => {
                println!("ERROR: execute bridge withdraw message: {:?}", &e);
                exit(1);
            }
        }
    }
}

/// withdraw into escrow account
pub fn bridge_withdraw(
    escrow: AccountAddress,
    sender: AccountAddress,
    sender_other: Vec<u8>,
    receiver: AccountAddress,
    balance: u64,
    transfer_id: Vec<u8>,
    save_path: Option<PathBuf>,
) -> Result<TransactionView, TxError> {
    let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();
    // coins are scaled
    let script = transaction_builder::encode_bridge_withdraw_script_function(escrow, sender,
                                                                             sender_other, receiver, balance, transfer_id);
    maybe_submit(script, &tx_params, save_path)
}
