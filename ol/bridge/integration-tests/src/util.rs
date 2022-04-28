use ethers::prelude::Address;
use ethers::prelude::Client;
use ethers::prelude::Wallet;
use ethers::providers::{Http, Provider};
use std::convert::TryFrom;
use std::env;
pub fn get_eth_client(account_name: &str) -> (Address, Address, u64, Client<Http, Wallet>) {
    let eth_cfg_path = env::var("ETH_BRIDGE_ESCROW_CONFIG").unwrap();
    println!("eth cfg path: {:?}", eth_cfg_path.clone());
    let eth_cfg = bridge_eth::config::Config::new(eth_cfg_path.as_str()).unwrap();
    let escrow_eth_addr = eth_cfg.get_escrow_contract_address().unwrap();
    println!("escrow_eth_addr: {:?}", escrow_eth_addr);
    let gas_price = eth_cfg.get_gas_price().unwrap();
    println!("gas_proce: {:?}", gas_price);
    let url = eth_cfg.get_provider_url().unwrap();
    let eth_provider: Provider<Http> = Provider::<Http>::try_from(url.as_str()).unwrap();
    let accounts_path_eth = env::var("ETH_ACCOUNTS_PATH").unwrap();
    let names = vec!["alice", "bob", "carol", "pete", "todd", "bridgeEscrow"];
    let eth_signers = bridge_eth::signers::get_signers(accounts_path_eth.as_str(), names).unwrap();
    let eth_sender_wallet = bridge_eth::signers::get_signer(&eth_signers, account_name).unwrap();

    let eth_ol_addr = eth_cfg.get_ol_contract_address().unwrap();
    let eth_escrow_addr = eth_cfg.get_escrow_contract_address().unwrap();
    let eth_client_ol = eth_sender_wallet.clone().connect(eth_provider.clone());
    (eth_ol_addr, eth_escrow_addr, gas_price, eth_client_ol)
}
