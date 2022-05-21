
//! Ethereum client
use bridge_eth::config::Config;
use ethers::prelude::{Address, Client as ClientEth, Provider, Http, U256, Wallet, Wallet as WalletEth};
use std::fmt;
use std::convert::TryFrom;

/// ETH Agent
pub struct AgentEth {
    /// ETH Escrow Contract Address
    pub escrow_addr: Address,

    /// ETH client
    pub client: ClientEth<Http, WalletEth>,

    /// ETH gas price
    pub gas_price: u64,
}

/// Contains current transfer_id to process and the next start element
/// to start searching from for the next transfer_id to process
#[derive(Debug, Copy, Clone)]
pub struct EthLockedInfo {
    /// Current transfer_id to process
    pub transfer_id: [u8; 16],
    ///  Index to start searching for the next transfer_id to process
    pub next_start: U256,
}

impl fmt::Display for EthLockedInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "transfer_id: {}, next_start: {}",
            hex::encode(self.transfer_id),
            self.next_start
        )
    }
}

impl AgentEth {
    /// Create new ETH agent
    pub fn new(
        config_eth: &Option<Config>,
        agent_eth: &Option<Wallet>,
    ) -> Result<AgentEth, String> {
        let escrow_addr = match &config_eth {
            Some(c) => c.get_escrow_contract_address(),
            None => Err(String::from("cannot get eth config")),
        }?;

        let provider_eth = match &config_eth {
            Some(c) => c.get_provider_url().and_then(|url| {
                Provider::<Http>::try_from(url.as_str()).map_err(|e| e.to_string())
            }),
            None => Err(String::from("cannot get eth config")),
        }?;

        let gas_price = match &config_eth {
            Some(c) => c.get_gas_price(),
            None => Err(String::from("cannot get eth config")),
        }?;

        let client = match &agent_eth {
            Some(w) => Ok(w.clone().connect(provider_eth.clone())),
            _ => Err(format!("wallet is not provided")),
        }?;
        Ok(AgentEth {
            escrow_addr,
            client,
            gas_price,
        })
    }
}
