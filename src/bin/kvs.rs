use clap::{arg, command, Command};
use kvs::{error::KvsResult, KvStore};
use std::{env::current_dir, process};

fn main() -> KvsResult<()> {
    let matches = command!()
        .name(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
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

    // println!("{:#?}", matches);
    let file_to_use = format!("{}/log.txt", current_dir()?.display());

    if let Some(matches) = matches.subcommand_matches("set") {
        let key = matches.get_one::<String>("KEY").unwrap().to_owned();
        let value = matches.get_one::<String>("VALUE").unwrap().to_owned();
        let mut store = KvStore::open(file_to_use)?;
        store.set(key, value)?;
        Ok(())
    } else if let Some(matches) = matches.subcommand_matches("get") {
        let key = matches.get_one::<String>("KEY").unwrap().to_owned();
        // println!("Before opening: {:?}", file_to_use);
        let mut store = KvStore::open(file_to_use)?;
        store.get(key)?;

        Ok(())
    } else if let Some(matches) = matches.subcommand_matches("rm") {
        let key = matches.get_one::<String>("KEY").unwrap().to_owned();
        let mut store = KvStore::open(file_to_use)?;

        store.remove(key)?;

        Ok(())
    } else {
        eprintln!("No recognizeable commands were run. Try cargo run -- --help for more info.");
        process::exit(1)
    }
}
