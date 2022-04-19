
use diem_types::account_address::AccountAddress;
use bridge_ol::submit_tx::tx_params_wrapper;
use uuid::Uuid;
use ol_types::config::TxType;
use ethers::types::{Address as EthAddress};
use bridge_ol::util::vec_to_array;
use std::env;
use ethers::providers::{Http, JsonRpcClient, Provider};
use std::convert::TryFrom;
use bridge_ethers::oltoken_mod::OLToken;

#[tokio::main]
#[test]
async fn test_transfer_simple() {
    let receiver_addr_str = String::from("15d34aaf54267db7d7c367839aaf71a00a2c6a65");
    let receiver_addr = hex::decode(receiver_addr_str).unwrap();

    // Eth contract
    let eth_cfg_path = env::var("ETH_BRIDGE_ESCROW_CONFIG").unwrap();
    println!("eth cfg path: {:?}", eth_cfg_path.clone());
    let eth_cfg = bridge_ethers::config::Config::new(
        eth_cfg_path.as_str()).unwrap();
    let escrow_eth_addr = eth_cfg.get_escrow_contract_address().unwrap();
    println!("escrow_eth_addr: {:?}", escrow_eth_addr);
    let url = eth_cfg.get_provider_url().unwrap();
    let gas_price = eth_cfg.get_gas_price().unwrap();
    let eth_provider: Provider<Http> = Provider::<Http>::try_from(url.as_str()).unwrap();
    let accounts_path_eth = env::var("ETH_ACCOUNTS_PATH").unwrap();
    let names = vec!["alice", "bob", "carol", "pete", "todd", "bridgeEscrow"];
    let eth_signers = bridge_ethers::signers::get_signers(accounts_path_eth.as_str(), names).unwrap();
    let eth_sender_wallet = bridge_ethers::signers::get_signer(&eth_signers, "pete").unwrap();

    let eth_ol_addr = eth_cfg.get_ol_contract_address().unwrap();
    let eth_client_ol = eth_sender_wallet.clone().connect(eth_provider.clone());
    let eth_ol_token = bridge_ethers::oltoken_mod::OLToken::new(eth_ol_addr, &eth_client_ol);

    // Validate that funds are transferred to the other blockchian
    let receiver_eth_addr = EthAddress::from(vec_to_array(receiver_addr.clone()).unwrap());
    let data = eth_ol_token.balance_of(receiver_eth_addr);
    let call = data
        .call()
        .await
        .map_err(|e| println!("Error pending: {}", e))
        .unwrap();
    println!("call: {:?}", call);

    // 0L contract
    let escrow_addr = "708B1D23219EB737035CB29A68F0F3A8"
        .parse::<AccountAddress>().unwrap();
    let tx_params = tx_params_wrapper(TxType::Mgmt);
    assert!(tx_params.is_ok());
    let contract = bridge_ol::contract::BridgeEscrow::new(
        escrow_addr, tx_params.unwrap());
    assert!(contract.is_ok());

    // Deposit into 0L
    let transfer_id = Uuid::new_v4().as_bytes().to_vec();
    let res = contract
        .unwrap()
        .bridge_deposit(
            AccountAddress::ZERO,
            receiver_addr.clone(),
            10,
            transfer_id,
            None,
        );
    println!("{:?}",res);
    assert!(res.is_ok());

}