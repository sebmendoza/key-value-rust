use clap::{arg, command, Command};
use std::process;

fn main() {
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
                .arg(arg!([KEY] "Key to identify and delete data point").required(true)),
        )
        .get_matches();

    // println!("{:#?}", matches);

    if let Some(_matches) = matches.subcommand_matches("set") {
        eprintln!("unimplemented");
        process::exit(1);
    } else if let Some(_matches) = matches.subcommand_matches("get") {
        eprintln!("unimplemented");
        process::exit(1);
    } else if let Some(_matches) = matches.subcommand_matches("rm") {
        eprintln!("unimplemented");
        process::exit(1);
    } else {
        eprintln!("No recognizeable commands were run. Try cargo run -- --help for more info.");
        process::exit(1)
    }
}
