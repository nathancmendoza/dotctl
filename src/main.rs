
mod config;
mod links;
mod hooks;
mod cli;

use std::error::Error;
use clap::Parser;

use crate::cli::DotterInvocation;

fn main() -> Result<(), Box<dyn Error>> {
    let invoke = DotterInvocation::parse();
    match invoke.action {
        cli::DotterActionWord::Use { config_path, link_config } => {
            println!("Using {} as the config file source", config_path);
            println!("Will copy the file: {}", !link_config);
        },
        cli::DotterActionWord::Setup { app_conf, dry_run } => {
            println!("Setup {} configuration on {}", app_conf, std::env::consts::OS);
            println!("Work will actually occur: {}", !dry_run);
        },
        cli::DotterActionWord::Teardown { app_conf, dry_run } => {
            println!("Teardown {} configuration on{}", app_conf, std::env::consts::OS);
            println!("Work will actually occur: {}", !dry_run);
        },
        cli::DotterActionWord::Status { app_conf } => {
            println!("Check the status of {} configuration", app_conf);
        },
        cli::DotterActionWord::Describe { app_conf, links_only, hooks_only } => {
            println!("Describing the {} configuration on {}", app_conf, std::env::consts::OS);
            println!("Listing links only: {}", links_only);
            println!("Listing hooks only: {}", hooks_only);
        }
    }
    Ok(())
}
