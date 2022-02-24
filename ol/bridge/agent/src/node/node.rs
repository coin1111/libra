//! `node` module

use super::client;
use crate::config::AppCfg;
use anyhow::Error;
use cli::diem_client::DiemClient;
use diem_config::config::{NodeConfig, RocksdbConfig};
use diem_json_rpc_client::views::TowerStateResourceView;
use diem_types::account_state::AccountState;
use diem_types::waypoint::Waypoint;
use diemdb::DiemDB;
use std::path::PathBuf;
use std::str;
use storage_interface::DbReader;

/// name of key in kv store for sync
pub const SYNC_KEY: &str = "is_synced";

/// node process name:
pub const NODE_PROCESS: &str = "diem-node";

/// miner process name:
pub const MINER_PROCESS: &str = "tower";

/// Configuration and state of node, account, and host.
pub struct Node {
    /// 0L configs
    pub app_conf: AppCfg,
    /// diemclient for connecting
    pub client: DiemClient,
    /// node conf
    pub node_conf: Option<NodeConfig>,
    /// TODO: deduplicate these
    pub chain_state: Option<AccountState>,
    miner_state: Option<TowerStateResourceView>,
}

impl Node {
    /// Create a instance of Check
    pub fn new(client: DiemClient, conf: &AppCfg, is_swarm: bool) -> Self {
        let node_yaml = if is_swarm {
            "node.yaml"
        } else {
            "validator.node.yaml"
        };

        let node_conf = match NodeConfig::load(conf.workspace.node_home.join(node_yaml)) {
            Ok(c) => Some(c),
            Err(_) => {
                println!("Warn: could not find a validator config file, trying fullnode");
                match NodeConfig::load(conf.workspace.node_home.join("fullnode.node.yaml")) {
                    Ok(c) => Some(c),
                    Err(_) => {
                        println!("ERROR: could not find any *.node.yaml file. Will start without knowing the Node configs");
                        None
                    }
                }
            }
        };

        return Self {
            client,
            app_conf: conf.clone(),
            node_conf,
            miner_state: None,
            chain_state: None,
        };
    }

    /// default node connection from configs
    pub fn default_from_cfg(mut cfg: AppCfg, swarm_path: Option<PathBuf>) -> Node {
        // NOTE: not intended for swarm.
        let client = client::pick_client(swarm_path.clone(), &mut cfg).unwrap();
        Node::new(client, &cfg, swarm_path.is_some())
    }

    /// return tower height on chain
    pub fn tower_height_on_chain(&self) -> u64 {
        match &self.miner_state {
            Some(s) => s.verified_tower_height,
            None => 0,
        }
    }

    /// return tower height on chain
    pub fn mining_epoch_on_chain(&self) -> u64 {
        match &self.miner_state {
            Some(s) => s.latest_epoch_mining,
            None => 0,
        }
    }
    /// validator sets
    pub fn validator_set_count(&self) -> usize {
        match &self.chain_state {
            Some(s) => s.get_validator_set().unwrap().unwrap().payload().len(),
            None => 0,
        }
    }

    /// Current monitor account
    pub fn account(&self) -> Vec<u8> {
        self.app_conf.profile.account.to_vec()
    }

    /// Get waypoint from client
    pub fn waypoint(&mut self) -> Result<Waypoint, Error> {
        match self.client.update_and_verify_state_proof() {
            Ok(_t) => self.client.waypoint(),
            Err(_) => self.app_conf.get_waypoint(None),
        }
    }

    /// Is current account in validator set
    pub fn is_in_validator_set(&self) -> bool {
        match &self.chain_state {
            Some(s) => {
                for v in s.get_validator_set().unwrap().unwrap().payload().iter() {
                    if v.account_address().to_vec() == self.app_conf.profile.account.to_vec() {
                        return true;
                    }
                }
                false
            }
            None => false,
        }
    }

    /// nothing is configured yet, empty box
    pub fn configs_exist(&mut self) -> bool {
        // check to see no files are present
        let home_path = self.app_conf.workspace.node_home.clone();

        let c_exist = home_path.join("vdf_proofs/proof_0.json").exists()
            && home_path.join("validator.node.yaml").exists()
            && home_path.join("key_store.json").exists();
        c_exist
    }

    /// the owner and operator accounts exist on chain
    pub fn accounts_exist_on_chain(&mut self) -> bool {
        let addr = self.app_conf.profile.account;
        let account = self.client.get_account(&addr);
        match account {
            Ok(opt) => match opt {
                Some(_) => true,
                None => false,
            },
            Err(_) => false,
        }
    }

    /// database is initialized, Please do NOT invoke this function frequently
    pub fn db_bootstrapped(&mut self) -> bool {
        let file = self.app_conf.workspace.db_path.clone();
        if file.exists() {
            // When not committing, we open the DB as secondary so the tool
            // is usable along side a running node on the same DB.
            // Using a TempPath since it won't run for long.
            match DiemDB::open(file, true, None, RocksdbConfig::default()) {
                Ok(db) => {
                    return db.get_latest_version().is_ok();
                }
                Err(_e) => (),
            }
        }
        return false;
    }

    /// database is initialized, Please do NOT invoke this function frequently
    pub fn db_files_exist(&mut self) -> bool {
        // check to see no files are present
        let db_path = self.app_conf.workspace.db_path.clone().join("diemdb");
        db_path.exists()
    }
}
