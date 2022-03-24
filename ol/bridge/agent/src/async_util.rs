//! Async utilities
use std::error::Error;
use tokio::runtime::Handle;
use crossbeam::channel;
use ethers::prelude::PendingTransaction;
use ethers::providers::{Http, Provider};
use ethers::prelude::builders::ContractCall;
use ethers::prelude::Wallet;
use ethers::types::H256;
use std::sync::Arc;

// async fn send_tx_async(data: ContractCall<'_, Http, Wallet, H256>) -> Result<PendingTransaction<'_, Http>, Box<dyn Error + Send + Sync>> {
//     let pending_tx = data
//         .send()
//         .await?;
//     println!("pending_tx: {:?}", pending_tx);
//     Ok(pending_tx)
// }
//
// /// Execute ETH transaction synchronously
// pub fn send_eth_tx<'a>(handle: &Handle,
//                        data: ContractCall<Http, Wallet, H256>) -> Result<PendingTransaction<'a,Http>, Box<dyn Error + Send + Sync>> {
//     let (tx, rx) = channel::bounded(1);
//     handle.spawn(async move {
//         //let pending_tx = send_tx_async(data).await;
//         let pending_tx = data.send().await;
//         let _ = tx.send(pending_tx);
//     });
//     Ok(rx.recv()??)
// }

async fn send_eth_tx_async(s:&str) -> Result<String, Box<dyn Error + Send + Sync>> {
    Ok(String::from(s))
}

pub fn send_eth_tx(handle: &Handle, s: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let (tx, rx) = channel::bounded(1);
    handle.spawn(async move {
        let score_res = send_eth_tx_async(&s).await;
        let _ = tx.send(score_res);
    });
    Ok(rx.recv()??)
}