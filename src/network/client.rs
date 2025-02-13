// src/network/client.rs

use crate::error::{KvsErrors, KvsResult, NetworkErrors};
use crate::protocol::{Protocol, Request, Response};
use std::net::TcpStream;

pub struct KvsClient {
    stream: TcpStream,
}

impl KvsClient {
    pub fn connect(addr: &str) -> KvsResult<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(KvsClient { stream })
    }

    pub fn set(&mut self, key: String, value: String) -> KvsResult<()> {
        self.stream.send_request(&Request::Set { key, value })?;
        match self.stream.receive_response()? {
            Response::Ok(_) => Ok(()),
            Response::Err(e) => Err(KvsErrors::from(NetworkErrors::NetworkError(e))),
        }
    }

    pub fn get(&mut self, key: String) -> KvsResult<Option<String>> {
        self.stream.send_request(&Request::Get { key })?;
        match self.stream.receive_response()? {
            Response::Ok(value) => Ok(value),
            Response::Err(e) => Err(KvsErrors::from(NetworkErrors::NetworkError(e))),
        }
    }

    pub fn remove(&mut self, key: String) -> KvsResult<()> {
        self.stream.send_request(&Request::Remove { key })?;
        match self.stream.receive_response()? {
            Response::Ok(_) => Ok(()),
            Response::Err(e) => Err(KvsErrors::from(NetworkErrors::NetworkError(e))),
        }
    }
}
