//! `bridge-deposit` subcommand

#![allow(clippy::never_loop)]

use crate::{
    submit_tx::{maybe_submit, tx_params_wrapper, TxError},
};
use diem_json_rpc_types::views::TransactionView;
use diem_transaction_builder::stdlib as transaction_builder;
use diem_types::account_address::AccountAddress;
use ol_types::config::TxType;
use std::{path::PathBuf};

/// Deposit into escrow account
pub fn bridge_deposit(
    escrow: AccountAddress,
    receiver_this: AccountAddress,
    receiver: Vec<u8>,
    coins: u64,
    transfer_id: Vec<u8>,
    save_path: Option<PathBuf>,
) -> Result<TransactionView, TxError> {
    let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();
    // coins are scaled
    let script = transaction_builder::encode_bridge_deposit_script_function(
        escrow,
        receiver_this,
        receiver,
        coins,
        transfer_id,
    );
    maybe_submit(script, &tx_params, save_path)
}
