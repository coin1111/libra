
//! Ethereum client
use bridge_eth::config::Config;
use ethers::prelude::{Address, Client as ClientEth, Provider, Http, U256, Wallet, Wallet as WalletEth};
use std::fmt;
use std::convert::TryFrom;
use bridge_eth::util::AccountInfo as AccountInfoEth;
use bridge_eth::bridge_escrow_multisig_mod::BridgeEscrowMultisig as BridgeEscrowEth;

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
    /// Query locked AccountInfo on ETH
    pub async fn query_eth_locked(&self, transfer_id: [u8; 16]) -> Result<AccountInfoEth, String> {
        let contract = BridgeEscrowEth::new(self.escrow_addr, &self.client);
        let data = contract.get_locked_account_info(transfer_id);
        data.call()
            .await
            .map_err(|err| format!("ERROR: call: {:?}", err))
            .and_then(|x| AccountInfoEth::from(x))
    }

    /// Query unlocked AccountInfo on ETH
    pub async fn query_eth_unlocked(&self, transfer_id: [u8; 16]) -> Result<AccountInfoEth, String> {
        let contract = BridgeEscrowEth::new(self.escrow_addr, &self.client);
        let data = contract.get_unlocked_account_info(transfer_id);
        data.call()
            .await
            .map_err(|err| format!("ERROR: call: {:?}", err))
            .and_then(|x| AccountInfoEth::from(x))
    }

    /// Get next unprocessed transfer id (locked AccountInfo) on ETH
    pub async fn get_eth_next_locked_info(
        &self,
        start: U256,
        len: U256,
    ) -> Result<EthLockedInfo, String> {
        let contract = BridgeEscrowEth::new(self.escrow_addr, &self.client);
        let data = contract.get_next_transfer_id(start, len);
        data.call()
            .await
            .map_err(|err| format!("ERROR: call: {:?}", err))
            .and_then(|tuple| {
                Ok(EthLockedInfo {
                    transfer_id: tuple.0,
                    next_start: tuple.1,
                })
            })
    }

}
