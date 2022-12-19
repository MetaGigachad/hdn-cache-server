#![doc = include_str!("../README.md")]

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
