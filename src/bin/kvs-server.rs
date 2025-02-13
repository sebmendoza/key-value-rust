// src/bin/kvs-server.rs

use clap::Parser;
use env_logger::Env;
use kvs::{
    error::{KvsErrors, KvsResult},
    network::server::KvsServer,
    KvStore, KvsEngine, SledKvsEngine,
};
use log::{error, info};
use std::env::current_dir;
use std::fs;
use std::path::Path;

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
    let address = &cli.addr;
    let engine = cli.engine.as_deref().unwrap_or("kvs");

    // Validate the address format
    validate_address(address)?;

    // Validate the engine name if provided
    if engine != "kvs" && engine != "sled" {
        return Err(KvsErrors::InvalidEngine());
    }

    let current_dir = current_dir()?;
    if let Some(existing_engine) = read_engine_type(&current_dir)? {
        if existing_engine != engine {
            error!(
                "Wrong engine type. Previous: {}, Current: {}",
                existing_engine, engine
            );
            return Err(KvsErrors::EngineTypeMismatch());
        }
    } else {
        // No engine type saved yet, save it
        save_engine_type(&current_dir, engine)?;
    }

    // Log configuration
    info!("Server configured to listen at {}", cli.addr);
    info!("Storage engine: {}", cli.engine.as_deref().unwrap_or("kvs"));

    save_engine_type(&current_dir, engine)?;
    // Parse engine string into enum
    match engine {
        "kvs" => run_with_engine(KvStore::open(current_dir)?, address),
        "sled" => run_with_engine(SledKvsEngine::new(current_dir)?, address),
        _ => Err(KvsErrors::InvalidEngine()),
    }
}

fn validate_address(addr: &str) -> KvsResult<()> {
    if !addr.contains(':') {
        return Err(KvsErrors::InvalidAddress());
    }
    Ok(())
}

fn run_with_engine<E: KvsEngine>(engine: E, addr: &String) -> KvsResult<()> {
    let mut server = KvsServer::new(addr, engine)?;
    info!("Server listening on {}", addr);
    server.run()
}

const ENGINE_FILE: &str = "engine";

pub fn save_engine_type(dir: &Path, engine_type: &str) -> std::io::Result<()> {
    fs::write(dir.join(ENGINE_FILE), engine_type)
}

pub fn read_engine_type(dir: &Path) -> std::io::Result<Option<String>> {
    match fs::read_to_string(dir.join(ENGINE_FILE)) {
        Ok(content) => Ok(Some(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}
