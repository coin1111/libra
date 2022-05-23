
//! Ethereum client
use bridge_eth::config::Config;
use ethers::prelude::{Address, Client as ClientEth, Provider, Http, U256, Wallet, Wallet as WalletEth};
use std::fmt;
use std::convert::TryFrom;
use bridge_eth::util::AccountInfo as AccountInfoEth;
use bridge_eth::bridge_escrow_multisig_mod::BridgeEscrowMultisig as BridgeEscrowEth;
use anyhow::{Error,anyhow,bail};

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
    ) -> Result<AgentEth, Error> {
        let config = config_eth.as_ref().ok_or(anyhow!("cannot get eth config"))?;
        let escrow_addr = config.get_escrow_contract_address()?;
        let provider_eth:Provider<Http> = config.get_provider_url()
            .and_then(|url|Provider::<Http>::try_from(url.as_str())
                .map_err(|e|anyhow!("error parsing url: {:?}",e.to_string())))?;

        let gas_price = config.get_gas_price()?;

        let client_r: Result<ClientEth<Http, WalletEth>,Error> = match &agent_eth {
            Some(w) =>
                Ok(w.clone().connect(provider_eth.clone())),
            _ => bail!("wallet is not provided"),
        };
        let client = client_r?;
        Ok(AgentEth {
            escrow_addr,
            client,
            gas_price,
        })
    }
    /// Query locked AccountInfo on ETH
    pub async fn query_eth_locked(&self, transfer_id: [u8; 16]) -> Result<AccountInfoEth, Error> {
        let contract = BridgeEscrowEth::new(self.escrow_addr, &self.client);
        let data = contract.get_locked_account_info(transfer_id);
        data.call()
            .await
            .map_err(|err| anyhow!("ERROR: call: {:?}", err))
            .and_then(|x| AccountInfoEth::from(x))
    }

    /// Query unlocked AccountInfo on ETH
    pub async fn query_eth_unlocked(&self, transfer_id: [u8; 16]) -> Result<AccountInfoEth, Error> {
        let contract = BridgeEscrowEth::new(self.escrow_addr, &self.client);
        let data = contract.get_unlocked_account_info(transfer_id);
        data.call()
            .await
            .map_err(|err| anyhow!("ERROR: call: {:?}", err))
            .and_then(|x| AccountInfoEth::from(x))
    }

    /// Get next unprocessed transfer id (locked AccountInfo) on ETH
    pub async fn get_eth_next_locked_info(
        &self,
        start: U256,
        len: U256,
    ) -> Result<EthLockedInfo, Error> {
        let contract = BridgeEscrowEth::new(self.escrow_addr, &self.client);
        let data = contract.get_next_transfer_id(start, len);
        data.call()
            .await
            .map_err(|err| anyhow!("ERROR: call: {:?}", err))
            .and_then(|tuple| {
                Ok(EthLockedInfo {
                    transfer_id: tuple.0,
                    next_start: tuple.1,
                })
            })
    }

}
