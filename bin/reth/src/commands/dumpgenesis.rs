//! Command to dump the genesis block configuration to stdout

use crate::{
    args::utils::{chain_spec_value_parser, SUPPORTED_CHAINS},
    dirs::{DataDirPath, MaybePlatformPath},
};
use clap::Parser;
use reth_primitives::ChainSpec;
use std::sync::Arc;

/// Dumps genesis block JSON configuration to stdout
#[derive(Debug, Parser)]
pub struct Command {
    /// The path to the data dir for all reth files and subdirectories.
    ///
    /// Defaults to the OS-specific data directory:
    ///
    /// - Linux: `$XDG_DATA_HOME/reth/` or `$HOME/.local/share/reth/`
    /// - Windows: `{FOLDERID_RoamingAppData}/reth/`
    /// - macOS: `$HOME/Library/Application Support/reth/`
    #[arg(long, value_name = "DATA_DIR", verbatim_doc_comment, default_value_t)]
    datadir: MaybePlatformPath<DataDirPath>,

    /// The chain this node is running.
    ///
    /// Possible values are one of the built-in chains.
    #[arg(
        long,
        value_name = "CHAIN_OR_PATH",
        long_help = format!("Possible value is one of the built-in chains.\nBuilt-in chains:\n {}", SUPPORTED_CHAINS.join(", ")),
        default_value = SUPPORTED_CHAINS[0],
        value_parser = chain_spec_value_parser
    )]
    chain: Arc<ChainSpec>,
}

impl Command {
    /// Execute the `dumpgenesis` command
    pub async fn execute(&self) -> eyre::Result<()> {
        Ok(())
    }
}
