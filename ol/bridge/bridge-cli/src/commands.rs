//! TxsApp Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `start`: launches the application
//! - `version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

pub mod bridge_close_transfer_cmd;
pub mod bridge_create_escrow_cmd;
pub mod bridge_deposit_cmd;
pub mod bridge_withdraw_cmd;

use self::{
    bridge_close_transfer_cmd::BridgeCloseTransferCmd,
    bridge_create_escrow_cmd::BridgeCreateEscrowCmd, bridge_deposit_cmd::BridgeDepositCmd,
    bridge_withdraw_cmd::BridgeWithdrawCmd,
};
use crate::config::AppCfg;
use crate::entrypoint;
use abscissa_core::{Command, Configurable, Help, Options, Runnable};
use ol::commands::CONFIG_FILE;
use std::path::PathBuf;

/// TxsApp Subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum TxsCmd {
    // --- End of STDLIB SCRIPT COMMANDS ---
    /// The `help` subcommand
    #[options(help = "get usage information")]
    Help(Help<Self>),

    /// create bridge escrow
    #[options(help = "create bridge escrow")]
    BridgeCreateEscrow(BridgeCreateEscrowCmd),

    /// deposit to bridge escrow
    #[options(help = "deposit to bridge escrow")]
    BridgeDeposit(BridgeDepositCmd),

    /// withdraw from bridge escrow
    #[options(help = "withdraw from bridge escrow")]
    BridgeWithdraw(BridgeWithdrawCmd),

    /// close transfer account
    #[options(help = "close tranfer account")]
    BridgeCloseTransfer(BridgeCloseTransferCmd),
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<AppCfg> for TxsCmd {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.

        let mut config_path = entrypoint::get_node_home();

        config_path.push(CONFIG_FILE);
        if config_path.exists() {
            // println!("initializing from config file: {:?}", config_path);
            Some(config_path)
        } else {
            // println!("config file not yet existing: {:?}", config_path);
            None
        }
    }
}
