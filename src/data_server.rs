//! Provides [DataServer](crate::data_server::DataServer) object which wraps all communication with data server

use crate::message::data_server::*;
use log::info;
use std::{error::Error, net::SocketAddr};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

/// Wrapper around connection with data server
pub struct DataServer {
    socket: TcpStream,
}

impl DataServer {
    /// Connectects to data server and returns wrapper for it
    pub async fn connect(addr: &SocketAddr) -> Result<Self, Box<dyn Error>> {
        let socket = TcpStream::connect(addr).await?;
        info!("Opened connection with data server {}", addr);

        Ok(Self { socket })
    }

    /// Executes Store request
    pub async fn store(
        &mut self,
        request: request::Store,
    ) -> Result<response::Store, Box<dyn Error>> {
        let (reader, mut writer) = self.socket.split();
        let mut reader = BufReader::new(reader);

        let request = Request::Store(request);
        let raw_request = postcard::to_stdvec_cobs(&request)?;
        writer.write_all(&raw_request).await?;
        info!("Sent request to data server. Body: {:?}", request);

        let mut raw_response = Vec::new();
        reader.read_until(0u8, &mut raw_response).await?;
        let response = postcard::from_bytes_cobs::<response::Store>(&mut raw_response)?;
        info!("Got response from data server. Body: {:?}", response);

        Ok(response)
    }

    /// Executes Load request
    pub async fn load(&mut self, request: request::Load) -> Result<response::Load, Box<dyn Error>> {
        let (reader, mut writer) = self.socket.split();
        let mut reader = BufReader::new(reader);

        let request = Request::Load(request);
        let raw_request = postcard::to_stdvec_cobs(&request)?;
        writer.write_all(&raw_request).await?;
        info!("Sent request to data server. Body: {:?}", request);

        let mut raw_response = Vec::new();
        reader.read_until(0u8, &mut raw_response).await?;
        let response = postcard::from_bytes_cobs::<response::Load>(&mut raw_response)?;
        info!("Got response from data server. Body: {:?}", response);

        Ok(response)
    }
}
