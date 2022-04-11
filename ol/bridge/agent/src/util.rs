//! Various util function for agent
use std::fs;
use crate::agent::EthLockedInfo;

const ETH_AGENT_CHECKPOINT: &'static str = ".agent_checkpoint";

/// Read checkpoint to query ETH bridge contract
pub fn read_eth_checkpoint() -> i32 {
    let start_idx = fs::read_to_string(ETH_AGENT_CHECKPOINT)
        .and_then(|ss| {
            let v: Vec<&str> = ss.split('\n').collect();
            let start = v.get(0).and_then(|s| {
                let idx = s
                    .split(',')
                    .collect::<Vec<&str>>()
                    .last()
                    .and_then(|v| Some(v.parse::<i32>().unwrap_or(0)));
                idx
            });
            Ok(start.unwrap_or(0))
        })
        .map_err(|err|println!("WARN: cannot read checkpoint file: {:?}, error: {:?}",
            ETH_AGENT_CHECKPOINT, err.to_string()))
        .unwrap_or(0);
    start_idx
}

/// Save checkpoint to query ETH side of the bridge
pub fn save_eth_checkpoint(locked: EthLockedInfo) -> Result<(),String>{
    let data = format!("{},{}", hex::encode(locked.transfer_id), locked.next_start);
    fs::write(".agent_checkpoint", data).map_err(|err| {
        format!("Unable to write file {:?}, error: {:?}", ETH_AGENT_CHECKPOINT, err)
    })
}