use anyhow::{anyhow, Error};
use bridge_ol::submit_tx::tx_params_wrapper;
use bridge_ol::submit_tx::TxError;
use cli::{diem_client::DiemClient};
use diem_types::account_address::AccountAddress;
use ethers::types::{U256};
use ol_types::config::TxType;
use std::convert::TryFrom;
use std::{thread, time};
use uuid::Uuid;
use crate::util::get_eth_client;

#[tokio::main]
#[test]
async fn test_transfer_eth_ol() {
    // Eth contract
    let (eth_ol_addr, eth_escrow_addr, eth_gas_price,  eth_client_ol) =
        get_eth_client("pete");
    let eth_ol_token = bridge_eth::oltoken_mod::OLToken::new(eth_ol_addr, &eth_client_ol);
    let eth_ol_bridge =
        bridge_eth::bridge_escrow_mod::BridgeEscrow::new(eth_escrow_addr, &eth_client_ol);

    // 0L contract
    let escrow_addr = "708B1D23219EB737035CB29A68F0F3A8"
        .parse::<AccountAddress>()
        .unwrap();
    let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();
    let ol_waypoint = tx_params.waypoint.clone();
    let ol_url = tx_params.url.clone();
    let contract = bridge_ol::contract::BridgeEscrow::new(escrow_addr, tx_params);
    assert!(contract.is_ok());

    let ol_client = DiemClient::new(ol_url, ol_waypoint)
        .map_err(|e| TxError {
            err: Some(e),
            tx_view: None,
            location: None,
            abort_code: None,
        })
        .unwrap();

    // 0L pete balance
    let receiver_addr_ol = "770B2C65843B25CA12CA48091FC33CD8"
        .parse::<AccountAddress>()
        .unwrap();

    let balance_ol_before = get_ol_balance(&ol_client, &receiver_addr_ol).unwrap();
    println!("Balance before: {}", balance_ol_before);

    // deposit into ETH account
    let amount = 10;
    let data_approve = eth_ol_token
        .approve(eth_escrow_addr, U256::from(amount))
        .gas_price(eth_gas_price);
    let approve_tx = data_approve
        .send()
        .await
        .map_err(|e| println!("Error pending: {}", e))
        .unwrap();
    println!("approve_tx: {:?}", approve_tx);

    let transfer_id = Uuid::new_v4().as_bytes().to_vec();
    let deposit_data = eth_ol_bridge
        .create_transfer_account(
            *receiver_addr_ol,
            amount,
            <[u8; 16]>::try_from(transfer_id.clone()).unwrap(),
        )
        .gas_price(eth_gas_price);
    let pending_tx = deposit_data
        .send()
        .await
        .map_err(|e| println!("Error pending: {}", e))
        .unwrap();
    println!("pending_tx: {:?}", pending_tx);
    let mut tries = 0;
    let max_tries = 100;
    while tries < max_tries {
        // Balance after
        let balance_ol_after = get_ol_balance(&ol_client, &receiver_addr_ol).unwrap();
        println!("Balance after: {}", balance_ol_after);

        let diff = (balance_ol_after - balance_ol_before) as u64;
        if diff == amount {
            break;
        }
        tries += 1;
        thread::sleep(time::Duration::from_millis(1000));
    }
    assert!(tries < max_tries);
}

fn get_ol_balance(ol_client: &DiemClient, receiver_addr_ol: &AccountAddress) -> Result<f64, Error> {
    match ol_client.get_account(&receiver_addr_ol) {
        Ok(Some(account_view)) => {
            match account_view.balances.iter().find(|av| av.currency == "GAS") {
                Some(av) => Ok(av.amount as f64),
                _ => Err(anyhow!("No GAS found on account".to_owned())),
            }
        }
        _ => Err(anyhow!(
            "No account {} found on chain, account",
            receiver_addr_ol
        )),
    }
}

