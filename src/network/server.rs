// src/network/server.rs

use crate::engines::KvsEngine;
use crate::error::KvsResult;
use crate::protocol::{Protocol, Request, Response};
use log::{error, info};
use std::net::TcpListener;

pub struct KvsServer<E: KvsEngine> {
    listener: TcpListener,
    engine: E,
}

impl<E: KvsEngine> KvsServer<E> {
    pub fn new(addr: &str, engine: E) -> KvsResult<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(KvsServer { listener, engine })
    }

    pub fn run(&mut self) -> KvsResult<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    info!("New connection from {}", stream.peer_addr()?);

                    // Handle the single request directly here instead of in a separate method to avoid borrowing error.
                    let request = stream.receive_request()?;
                    let response = match request {
                        Request::Set { key, value } => match self.engine.set(key, value) {
                            Ok(()) => Response::Ok(None),
                            Err(e) => Response::Err(e.to_string()),
                        },
                        Request::Get { key } => match self.engine.get(key) {
                            Ok(value) => Response::Ok(value),
                            Err(e) => Response::Err(e.to_string()),
                        },
                        Request::Remove { key } => match self.engine.remove(key) {
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
