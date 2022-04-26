use ethers::types::Address;
use serde_json::{Map, Value};
use std::fs;
#[derive(Debug, Clone)]
pub struct Config {
    obj: Map<String, Value>,
}

impl Config {
    pub fn new(path: &str) -> Result<Config, String> {
        let config =
            fs::read_to_string(path).map_err(|e| format!("error reading config: {}", e))?;
        let parsed: Value =
            serde_json::from_str(&config).map_err(|e| format!("error deseralizing json: {}", e))?;
        Ok(Config {
            obj: parsed.as_object().unwrap().clone(),
        })
    }
    /// Gets escrow contract address
    pub fn get_escrow_contract_address(&self) -> Result<Address, String> {
        let escrow_addr_str = self.obj.get("escrowContract").map_or_else(
            || Err(format!("error escrowContract value is missing")),
            |x| Ok(x),
        )?;
        (escrow_addr_str.to_string().replace("\"", ""))[2..]
            .parse::<Address>()
            .map_err(|e| format!("error parsing address: {:?}", e))
    }

    pub fn get_ol_contract_address(&self) -> Result<Address, String> {
        let addr_str = self.obj.get("olTokenContract").map_or_else(
            || Err(format!("error olTokenContract value is missing")),
            |x| Ok(x),
        )?;
        (addr_str.to_string().replace("\"", ""))[2..]
            .parse::<Address>()
            .map_err(|e| format!("error parsing address: {:?}", e))
    }
    /// Gets provider url
    pub fn get_provider_url(&self) -> Result<String, String> {
        self.obj
            .get("url")
            .map_or_else(
                || Err(format!("error url value is missing")),
                |x| {
                    Ok(x.as_str().map_or_else(
                        || Err(format!("error url value is invalid")),
                        |x| Ok(String::from(x)),
                    ))
                },
            )
            .and_then(|x| x)
    }
    /// Gets gas price
    pub fn get_gas_price(&self) -> Result<u64, String> {
        self.obj
            .get("gasPrice")
            .map_or_else(
                || Err(format!("error gasPrice value is missing")),
                |x| {
                    Ok(x.as_u64().map_or_else(
                        || Err(format!("error gasPrice value is invalid")),
                        |x| Ok(x),
                    ))
                },
            )
            .and_then(|x| x)
    }
}
