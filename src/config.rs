//! Scheme for config file and it's defaults

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Server configuration scheme
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub listener_addr: SocketAddr,
    pub data_server_addr: SocketAddr,
    pub db_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listener_addr: "127.0.0.1:9002".parse().unwrap(),
            data_server_addr: "127.0.0.1:9001".parse().unwrap(),
            db_dir: "data".to_string(),
        }
    }
}
