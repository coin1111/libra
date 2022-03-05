//! BridgeEscrow methods
use crate::submit_tx::{maybe_submit, tx_params_wrapper, TxError};

use diem_json_rpc_types::views::TransactionView;
use diem_transaction_builder::stdlib as transaction_builder;
use diem_types::account_address::AccountAddress;
use ol_types::config::TxType;
use std::path::PathBuf;

/// withdraw into escrow account
pub fn bridge_withdraw(
    escrow: AccountAddress,
    sender_this: AccountAddress,
    sender_other: Vec<u8>,
    receiver: AccountAddress,
    balance: u64,
    transfer_id: Vec<u8>,
    save_path: Option<PathBuf>,
) -> Result<TransactionView, TxError> {
    let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();
    // coins are scaled
    let script = transaction_builder::encode_bridge_withdraw_script_function(
        escrow,
        sender_this,
        sender_other,
        receiver,
        balance,
        transfer_id,
    );
    maybe_submit(script, &tx_params, save_path)
}

/// withdraw into escrow account
pub fn bridge_close_transfer(
    escrow: AccountAddress,
    transfer_id: Vec<u8>,
    close_other: bool,
    save_path: Option<PathBuf>,
) -> Result<TransactionView, TxError> {
    let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();
    // coins are scaled
    let script = transaction_builder::encode_bridge_close_transfer_script_function(
        escrow,
        transfer_id,
        close_other,
    );
    maybe_submit(script, &tx_params, save_path)
}
