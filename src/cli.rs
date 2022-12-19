//! Scheme for command line arguments

use clap::Parser;
use std::path::PathBuf;

/// Hash delivery network cache server
///
/// This server represents a node which is directly accessed by user.
/// It connects to data server provided in configuration and in case of data obsolescence
/// asks it from it. All user data loads are also passed to data server.
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Config file path. If none provided $XDF_CONFIG_HOME/hdn-cache-server dir is used
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}
