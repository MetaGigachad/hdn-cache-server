//! All handlers of server

use crate::cli::Args;
use crate::config::Config;
use crate::data_server::DataServer;
use crate::message::{cache_server as cs, data_server as ds};
use clap::Parser;
use log::{debug, info};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

/// Stores data which is shared between handlers
pub struct HandlerContext {
    pub db: sled::Db,
    pub config: Config,
    pub data_server: Mutex<DataServer>,
}

impl HandlerContext {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let config = match Args::parse().config {
            Some(path) => confy::load_path::<Config>(path)?,
            None => confy::load::<Config>("hdn-cache-server", None)?,
        };

        Ok(Self {
            db: sled::open(&config.db_dir)?,
            data_server: Mutex::new(DataServer::connect(&config.data_server_addr).await?),
            config,
        })
    }
}

/// Handles connection with user
pub async fn connection_handler(
    mut socket: TcpStream,
    context: Arc<HandlerContext>,
) -> Result<(), Box<dyn Error>> {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);

    // Send greeting
    writer
        .write_all("{\"student_name\": \"Yaroslav Plishan\"}".as_bytes())
        .await?;

    loop {
        // Read incoming json
        let mut raw_request = Vec::new();
        reader.read_until(b'}', &mut raw_request).await?;
        let request = serde_json::from_str::<cs::Request>(&String::from_utf8(raw_request)?)?;
        info!(
            "From {} recieved request {:?}",
            writer.peer_addr()?,
            request
        );

        // Handle request
        let mut response = match request {
            cs::Request::Load(request) => load_request_handler(request, context.clone()).await?,
            cs::Request::Store(request) => store_request_handler(request, context.clone()).await?,
        };

        // Send response
        let raw_response = serde_json::to_string(&mut response)?;
        writer.write_all(&raw_response.as_bytes()).await?;
        debug!("Responded to {} with {:?}", writer.peer_addr()?, response);
    }
}

/// Handles [Load](crate::message::cache_server::request::Load) request from user
async fn load_request_handler(
    request: cs::request::Load,
    context: Arc<HandlerContext>,
) -> Result<cs::Response, Box<dyn Error>> {
    let response = match context.db.get(&request.key)? {
        Some(hash) => cs::response::Load::Success {
            requested_key: request.key,
            requested_hash: String::from_utf8(hash.to_vec())?,
        },
        None => {
            let data_server_response = context
                .data_server
                .lock()
                .await
                .load(ds::request::Load {
                    key: request.key.clone(),
                })
                .await?;

            match data_server_response.hash {
                Some(hash) => {
                    context.db.insert(&request.key, hash.clone())?;
                    cs::response::Load::Success {
                        requested_key: request.key,
                        requested_hash: String::from_utf8(hash)?,
                    }
                }
                None => cs::response::Load::NotFound,
            }
        }
    };

    Ok(cs::Response::Load(response))
}

/// Handles [Store](crate::message::cache_server::request::Store) request from user
async fn store_request_handler(
    request: cs::request::Store,
    context: Arc<HandlerContext>,
) -> Result<cs::Response, Box<dyn Error>> {
    let hash = request.hash.as_bytes();

    if let Some(val) = context.db.get(&request.key)? {
        if val == hash {
            return Ok(cs::Response::Store(cs::response::Store::Success));
        }
    }

    let data_server_response = context
        .data_server
        .lock()
        .await
        .store(ds::request::Store {
            key: request.key.clone(),
            hash: request.hash.as_bytes().to_vec(),
        })
        .await?;
    context.db.insert(request.key, hash)?;

    let response = if data_server_response.success {
        cs::response::Store::Success
    } else {
        cs::response::Store::Error
    };

    Ok(cs::Response::Store(response))
}
