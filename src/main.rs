
mod config;
mod links;
mod hooks;
mod cli;

use std::{error::Error, iter::once};
use clap::Parser;

use crate::cli::DotterInvocation;

//#[derive(Parser, Debug)]
//#[command(author, version, about, long_about = None)]
//struct Args {
//    /// Name of the person to greet
//    #[arg(short, long)]
//    name: String,
//
//    /// Number of times to greet
//    #[arg(short, long, default_value_t = 1)]
//    count: u8,
//}

//#[derive(Parser)]
//#[command(author, version, about, long_about = None)]
//struct Cli {
//    #[command(subcommand)]
//    action: Action
//}
//
//#[derive(Subcommand)]
//enum Action {
//    Vowels { s: String},
//    Consonants {s: String}
//}


fn main() -> Result<(), Box<dyn Error>> {
    let invoke = DotterInvocation::parse();
    match invoke.action {
        cli::DotterActionWord::Use { config_path, link_config } => {
            println!("Using {} as the config file source", config_path);
            println!("Will copy the file: {}", !link_config);
        },
        _ => {
            println!("Action not supported");
        }
    }
    Ok(())
}
