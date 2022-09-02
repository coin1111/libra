//! `bridge-deposit` subcommand

#![allow(clippy::never_loop)]

use crate::submit_tx::TxParams;
use crate::submit_tx::{maybe_submit, TxError};
use diem_json_rpc_types::views::TransactionView;
use diem_transaction_builder::stdlib as transaction_builder;
use diem_types::account_address::AccountAddress;
use std::path::PathBuf;
use anyhow::Error;

pub struct BridgeEscrowMultisig {
    /// Bridge escrow account
    pub escrow: AccountAddress,

    tx_params: TxParams,
}

impl BridgeEscrowMultisig {
    pub fn new(escrow: AccountAddress,tx_params: TxParams) -> Result<BridgeEscrowMultisig, Error> {
        Ok(BridgeEscrowMultisig { escrow, tx_params })
    }

    /// Deposit into escrow account
    pub fn bridge_deposit(
        &self,
        receiver: Vec<u8>,
        coins: u64,
        transfer_id: Vec<u8>,
        save_path: Option<PathBuf>,
    ) -> Result<TransactionView, TxError> {
        // coins are scaled
        let script = transaction_builder::encode_bridge_multisig_deposit_script_function(
            self.escrow,
            receiver,
            coins,
            transfer_id,
        );
        maybe_submit(script, &self.tx_params, save_path)
    }

    /// withdraw into escrow account
    pub fn bridge_withdraw(
        &self,
        sender_other: Vec<u8>,
        receiver: AccountAddress,
        balance: u64,
        transfer_id: Vec<u8>,
        save_path: Option<PathBuf>,
    ) -> Result<TransactionView, TxError> {
        // coins are scaled
        let script = transaction_builder::encode_bridge_multisig_withdraw_script_function(
            self.escrow,
            sender_other,
            receiver,
            balance,
            transfer_id,
        );
        maybe_submit(script, &self.tx_params, save_path)
    }

    /// withdraw into escrow account
    pub fn bridge_close_transfer(
        &self,
        transfer_id: &Vec<u8>,
        close_other: bool,
        save_path: Option<PathBuf>,
    ) -> Result<TransactionView, TxError> {
        // coins are scaled
        let script = transaction_builder::encode_bridge_multisig_close_transfer_script_function(
            self.escrow,
            transfer_id.clone(),
            close_other,
        );
        maybe_submit(script, &self.tx_params, save_path)
    }
}