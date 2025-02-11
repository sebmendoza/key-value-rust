// src/network/server.rs

use crate::error::KvsResult;
use crate::protocol::{Protocol, Request, Response};
use crate::KvStore;
use log::{error, info};
use std::net::TcpListener;

pub struct KvsServer {
    listener: TcpListener,
    store: KvStore,
}

impl KvsServer {
    pub fn new(addr: &str, store: KvStore) -> KvsResult<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(KvsServer { listener, store })
    }

    pub fn run(&mut self) -> KvsResult<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    info!("New connection from {}", stream.peer_addr()?);

                    // Handle the single request directly here instead of in a separate method to avoid borrowing error.
                    let request = stream.receive_request()?;
                    let response = match request {
                        Request::Set { key, value } => match self.store.set(key, value) {
                            Ok(()) => Response::Ok(None),
                            Err(e) => Response::Err(e.to_string()),
                        },
                        Request::Get { key } => match self.store.get(key) {
                            Ok(value) => Response::Ok(value),
                            Err(e) => Response::Err(e.to_string()),
                        },
                        Request::Remove { key } => match self.store.remove(key) {
                            Ok(()) => Response::Ok(None),
                            Err(e) => Response::Err(e.to_string()),
                        },
                    };

                    if let Err(e) = stream.send_response(&response) {
                        error!("Failed to send response: {}", e);
                    }
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
        Ok(())
    }
}
