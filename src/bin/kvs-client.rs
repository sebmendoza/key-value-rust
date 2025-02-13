// src/bin/kvs-client.rs
use clap::{arg, command, Command};
use kvs::{error::KvsResult, network::client::KvsClient};
use std::process;

fn main() -> KvsResult<()> {
    let matches = command!()
        .name(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            arg!(--addr <ADDR>)
                .global(true)
                .default_value("127.0.0.1:4000")
                .help("Server address"),
        )
        .subcommand(
            Command::new("set")
                .about("Give a key and value to store in database")
                .arg(arg!([KEY] "The key as a string").required(true))
                .arg(arg!([VALUE] "The value as string to store").required(true)),
        )
        .subcommand(
            Command::new("get")
                .about("Get a value given a ley as input")
                .arg(arg!([KEY] "The key of the data point").required(true)),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a data point given a key as input")
                .arg(arg!([KEY] "Key to identify and delete data point").required(true)), // Ensure this is required
        )
        .get_matches();

    let addr: &String = matches.get_one::<String>("addr").unwrap();
    let mut client = KvsClient::connect(addr)?;


    if let Some(matches) = matches.subcommand_matches("set") {
        let key = matches.get_one::<String>("KEY").unwrap().to_owned();
        let value = matches.get_one::<String>("VALUE").unwrap().to_owned();
        client.set(key, value)?;
    } else if let Some(matches) = matches.subcommand_matches("get") {
        let key = matches.get_one::<String>("KEY").unwrap().to_owned();
        if let Some(value) = client.get(key)? {
            println!("{}", value);
        } else {
            println!("Key not found");
        }
    } else if let Some(matches) = matches.subcommand_matches("rm") {
        let key = matches.get_one::<String>("KEY").unwrap().to_owned();
        client.remove(key)?;
    } else {
        eprintln!("Unknown subcommand");
        process::exit(1);
    }
    process::exit(0);

   
}
