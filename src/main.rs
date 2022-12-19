//! # Hash delivery network cache server
//!
//! This server respresents node in between user and data server. It has it's own 
//! [database](https://docs.rs/sled/latest/sled/) of
//! hashes which is updated from data server only on demand.
//!
//! This project uses [tokio](https://docs.rs/tokio/latest/tokio/) as a runtime and
//! [sled](https://docs.rs/sled/latest/sled/) as it's database to be blazingly fast.
//!
//! > ATTENTION: This server doesn't support standalone mode. It means that data server should be
//! > accessable for this server to run.
//!
//! # Configuration
//!
//! By default server will create configuration file in usual directory for your OS 
//! (on *nix it will be `$XDG_CONFIG_HOME/hdn-cache-server/default-config.toml`).
//! If you wish to use another config you can provide its path through `--config` parameter.
//! Note that if none such file exists it will be created with default parameters.
//!
//! Database will also auto create it's files if none exist.
//!
//! ## Default config
//! ```toml
//! listener_addr = '127.0.0.1:9002'
//! data_server_addr = '127.0.0.1:9001'
//! db_dir = 'data' # Any path can be provided here
//! ```
//!
//! # Communication with user
//!
//! Cache server supports two types of requests in form of json's:
//! ## Store
//! Request
//! ```json
//! {
//!  "request_type": "store",
//!  "key": "some_key",
//!  "hash": "0b672dd94fd3da6a8d404b66ee3f0c83"
//! }
//! ```
//! Response
//! ```json
//! {
//!  "response_status": "success"
//! }
//! ```
//!
//! ## Load
//! Request
//! ```json
//! {
//!  "request_type": "load",
//!  "key": "some_key"
//! }
//! ```
//! Responses
//! ```json
//! {
//!  "response_status": "success",
//!  "requested_key": "some_key",
//!  "requested_hash": "0b672dd94fd3da6a8d404b66ee3f0c83",
//! }
//! {
//!  "response_status": "key not found",
//! }
//! ```
//!
//! # Communication with data server
//!
//! Communication with data server uses same types of requests in more optimal scheme and uses
//! [postcard](https://docs.rs/postcard/latest/postcard/) format. See [messages](crate::message::data_server) for more details.

use handler::connection_handler;
use handler::HandlerContext;
use log::{debug, info};
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpListener;

mod cli;
mod config;
mod data_server;
mod handler;
mod message;

/// App runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::formatted_timed_builder()
        .filter(None, log::LevelFilter::Debug)
        .init();

    let context = Arc::new(HandlerContext::new().await?);
    let listener = TcpListener::bind(context.config.listener_addr).await?;
    info!("Listening on port {}", context.config.listener_addr.port());

    loop {
        let (socket, addr) = listener.accept().await?;
        let context = context.clone();
        debug!("Accepted connection from address {}", addr);

        tokio::spawn(async move {
            match connection_handler(socket, context).await {
                Ok(_) => debug!("Closed connection with {}", addr),
                Err(err) => {
                    debug!(
                        "Error occured while handling {}. Closing connection. Error: {:?}",
                        addr, err
                    )
                }
            }
        });
    }
}
