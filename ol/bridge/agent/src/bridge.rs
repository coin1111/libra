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
    let script = transaction_builder::encode_bridge_withdraw_script_function(escrow, sender_this,
                                                                             sender_other, receiver, balance, transfer_id);
    maybe_submit(script, &tx_params, save_path)
}
