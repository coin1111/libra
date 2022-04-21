use bridge_ol::submit_tx::tx_params_wrapper;
use bridge_ol::util::vec_to_array;
use diem_types::account_address::AccountAddress;
use ethers::prelude::Address;
use ethers::prelude::Client;
use ethers::prelude::Wallet;
use ethers::providers::{Http, Provider};
use ethers::types::{Address as EthAddress, U256};
use ol_types::config::TxType;
use std::convert::TryFrom;
use std::env;
use std::{thread, time};
use uuid::Uuid;

#[tokio::main]
#[test]
async fn test_transfer_ol_eth() {
    let receiver_addr_str = String::from("15d34aaf54267db7d7c367839aaf71a00a2c6a65");
    let receiver_addr = hex::decode(receiver_addr_str).unwrap();

    // Eth contract
    let (eth_ol_addr, eth_client_ol) = get_eth_client("pete");
    let eth_ol_token = bridge_ethers::oltoken_mod::OLToken::new(eth_ol_addr, &eth_client_ol);

    // Validate that funds are transferred to the other blockchian
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
    let res = contract.unwrap().bridge_deposit(
        AccountAddress::ZERO,
        receiver_addr.clone(),
        deposit_amount,
        transfer_id,
        None,
    );
    println!("{:?}", res);
    assert!(res.is_ok());

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

fn get_eth_client(account_name: &str) -> (Address, Client<Http, Wallet>) {
    let eth_cfg_path = env::var("ETH_BRIDGE_ESCROW_CONFIG").unwrap();
    println!("eth cfg path: {:?}", eth_cfg_path.clone());
    let eth_cfg = bridge_ethers::config::Config::new(eth_cfg_path.as_str()).unwrap();
    let escrow_eth_addr = eth_cfg.get_escrow_contract_address().unwrap();
    println!("escrow_eth_addr: {:?}", escrow_eth_addr);
    let url = eth_cfg.get_provider_url().unwrap();
    let eth_provider: Provider<Http> = Provider::<Http>::try_from(url.as_str()).unwrap();
    let accounts_path_eth = env::var("ETH_ACCOUNTS_PATH").unwrap();
    let names = vec!["alice", "bob", "carol", "pete", "todd", "bridgeEscrow"];
    let eth_signers =
        bridge_ethers::signers::get_signers(accounts_path_eth.as_str(), names).unwrap();
    let eth_sender_wallet = bridge_ethers::signers::get_signer(&eth_signers, account_name).unwrap();

    let eth_ol_addr = eth_cfg.get_ol_contract_address().unwrap();
    let eth_client_ol = eth_sender_wallet.clone().connect(eth_provider.clone());
    (eth_ol_addr, eth_client_ol)
}
