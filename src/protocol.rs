use crate::error::{KvsResult, NetworkErrors};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// Protocol messages from client to server
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

/// Protocol messages from server to client
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Ok(Option<String>),
    Err(String),
}

/// Protocol implementation for sending/receiving messages
pub trait Protocol: Read + Write {
    /// Send a request over the connection
    fn send_request(&mut self, request: &Request) -> KvsResult<()> {
        let json = serde_json::to_string(request)
            .map_err(|e| NetworkErrors::SerializationError(e.to_string()))?;
        let len = json.len() as u32;
        self.write_all(&len.to_be_bytes())?;
        self.write_all(json.as_bytes())?;
        self.flush()?;
        Ok(())
    }

    /// Send a response over the connection
    fn send_response(&mut self, response: &Response) -> KvsResult<()> {
        let json = serde_json::to_string(response)
            .map_err(|e| NetworkErrors::SerializationError(e.to_string()))?;
        let len = json.len() as u32;
        self.write_all(&len.to_be_bytes())?;
        self.write_all(json.as_bytes())?;
        self.flush()?;
        Ok(())
    }

    /// Receive a request from the connection
    fn receive_request(&mut self) -> KvsResult<Request> {
        let mut len_bytes = [0u8; 4];
        self.read_exact(&mut len_bytes)?;
        let len = u32::from_be_bytes(len_bytes);

        let mut json = vec![0u8; len as usize];
        self.read_exact(&mut json)?;

        let request = serde_json::from_slice(&json)
            .map_err(|e| NetworkErrors::SerializationError(e.to_string()))?;
        Ok(request)
    }

    /// Receive a response from the connection
    fn receive_response(&mut self) -> KvsResult<Response> {
        let mut len_bytes = [0u8; 4];
        self.read_exact(&mut len_bytes)?;
        let len = u32::from_be_bytes(len_bytes);

        let mut json = vec![0u8; len as usize];
        self.read_exact(&mut json)?;

        let response = serde_json::from_slice(&json)
            .map_err(|e| NetworkErrors::SerializationError(e.to_string()))?;
        Ok(response)
    }
}

// Implement Protocol for any type that implements Read + Write
impl<T: Read + Write> Protocol for T {}
