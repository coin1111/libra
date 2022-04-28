use crate::util::get_eth_client;
///
/// Test 0L -> ETH transfer
/// Agent must be running
///
use bridge_ol::submit_tx::tx_params_wrapper;
use bridge_ol::util::vec_to_array;
use diem_types::account_address::AccountAddress;
use ethers::types::{Address as EthAddress, U256};
use ol_types::config::TxType;
use std::{thread, time};
use uuid::Uuid;

#[tokio::main]
#[test]
async fn test_transfer_ol_eth() {
    // ETH receiver address
    let receiver_addr_str = String::from("15d34aaf54267db7d7c367839aaf71a00a2c6a65");
    let receiver_addr = hex::decode(receiver_addr_str).unwrap();

    // Eth contract
    let (eth_ol_addr, _, _, eth_client_ol) = get_eth_client("pete");
    let eth_ol_token = bridge_eth::oltoken_mod::OLToken::new(eth_ol_addr, &eth_client_ol);

    // Check ETH balance before
    let receiver_eth_addr = EthAddress::from(vec_to_array(receiver_addr.clone()).unwrap());
    let data = eth_ol_token.balance_of(receiver_eth_addr);
    let receiver_eth_balance_before = data
        .call()
        .await
        .map_err(|e| println!("Error pending: {}", e))
        .unwrap();
    println!(
        "Before receiver_eth_balance: {:?}",
        receiver_eth_balance_before
    );

    // 0L contract
    let escrow_addr = "708B1D23219EB737035CB29A68F0F3A8"
        .parse::<AccountAddress>()
        .unwrap();
    let tx_params = tx_params_wrapper(TxType::Mgmt);
    assert!(tx_params.is_ok());
    let contract = bridge_ol::contract::BridgeEscrow::new(escrow_addr, tx_params.unwrap());
    assert!(contract.is_ok());

    // Deposit into 0L
    let deposit_amount = 10;
    let transfer_id = Uuid::new_v4().as_bytes().to_vec();
    let res =
        contract
            .unwrap()
            .bridge_deposit(receiver_addr.clone(), deposit_amount, transfer_id, None);
    println!("{:?}", res);
    assert!(res.is_ok());

    // Wain until agent transfer funds to ETH chain
    let mut tries = 0;
    let max_tries = 100;
    while tries < max_tries {
        let receiver_eth_balance_after = data
            .call()
            .await
            .map_err(|e| println!("Error pending: {}", e))
            .unwrap();
        println!(
            "After receiver_eth_balance: {:?}",
            receiver_eth_balance_after
        );

        let diff: U256 = receiver_eth_balance_after - receiver_eth_balance_before;
        if diff.as_u64() == deposit_amount {
            break;
        }
        tries += 1;
        thread::sleep(time::Duration::from_millis(1000));
    }
    assert!(tries < max_tries);
}
