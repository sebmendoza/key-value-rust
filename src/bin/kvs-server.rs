// src/bin/kvs-server.rs

use clap::Parser;
use env_logger::Env;
use kvs::{
    error::{KvsErrors, KvsResult},  // Add proper error imports
    KvStore,
    network::server::KvsServer
};
use log::info;  // Add error macro
use std::env::current_dir;

#[derive(Parser)]
#[command(name = "kvs-server")]
#[command(about = "A key-value store server")]
#[command(version)]
struct Cli {
    /// The IP:PORT address to bind to
    #[arg(long, default_value = "127.0.0.1:4000")]
    addr: String,

    /// The storage engine to use (kvs or sled)
    #[arg(long)]
    engine: Option<String>,
}

fn main() -> KvsResult<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Log version number
    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    // Validate the address format
    validate_address(&cli.addr)?;

    // Validate the engine name if provided
    if let Some(engine) = &cli.engine {
        if engine != "kvs" && engine != "sled" {
            return Err(KvsErrors::InvalidEngine());
        }
    }
    
    // Log configuration
    info!("Server configured to listen at {}", cli.addr);
    info!("Storage engine: {}", cli.engine.as_deref().unwrap_or("kvs"));

    // Initialize the store
    let store = KvStore::open(current_dir()?)?;
    
    // Create and run the server
    let mut server = KvsServer::new(&cli.addr, store)?;
    info!("Server listening on {}", cli.addr);
    
    // Must return the Result from run()
    server.run()
}

fn validate_address(addr: &str) -> KvsResult<()> {
    if !addr.contains(':') {
        return Err(KvsErrors::InvalidAddress());
    }
    Ok(())
}