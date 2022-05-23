//! Agent for 0L
use crate::entrypoint::tx_params_wrapper;
use crate::util::{read_eth_checkpoint, save_eth_checkpoint};
use crate::{node::node::Node, node::query::QueryType};
use bridge_eth::bridge_escrow_multisig_mod::BridgeEscrowMultisig as BridgeEscrowEth;
use bridge_eth::config::Config;
use bridge_eth::util::AccountInfo as AccountInfoEth;
use bridge_ol::contract::BridgeEscrowMultisig;
use ethers::prelude::{ H160, Wallet};
use ethers::types::{Address, U256};
use move_core_types::account_address::AccountAddress;
use ol_types::config::TxType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use crate::transfer::agent_eth::{AgentEth, EthLockedInfo};

