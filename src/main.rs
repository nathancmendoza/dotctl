
mod config;
mod links;
mod hooks;
mod cli;

use std::error::Error;
use clap::Parser;

use crate::cli::DotterInvocation;
use crate::config::read_config;
use crate::links::setup_link;

fn use_config() {

}

fn setup_config(app_conf: &str) -> Result<(), std::io::Error> {
    let configs = read_config()?;
    for item in configs.all_configs().filter(| config | *config.assigned_name() == app_conf).filter(| config | config.config_os() == std::env::consts::OS) {
        for link in item.links() {
            setup_link(link, configs.config_repo())?;
        }
    }
    Ok(())
}

fn teardwon_config() {

}

fn show_config_status() {

}

fn describe_config() {

}

fn main() -> Result<(), Box<dyn Error>> {
    let invoke = DotterInvocation::parse();
    match invoke.action {
        cli::DotterActionWord::Use { config_path, link_config } => {
            println!("Using {} as the config file source", config_path);
            println!("Will copy the file: {}", !link_config);
        },
        cli::DotterActionWord::Setup { app_conf, dry_run, link_mode } => {
            println!("Setup {} configuration on {}", app_conf, std::env::consts::OS);
            println!("Work will actually occur: {}", !dry_run);
            setup_config(&app_conf)?;
        },
        cli::DotterActionWord::Teardown { app_conf, dry_run } => {
            println!("Teardown {} configuration on{}", app_conf, std::env::consts::OS);
            println!("Work will actually occur: {}", !dry_run);
        },
        cli::DotterActionWord::Status { app_conf } => {
            println!("Check the status of {} configuration", app_conf);
        },
        cli::DotterActionWord::Describe { app_conf  } => {
            match app_conf {
                Some(s) => println!("Describing the {} configuration on {}", s, std::env::consts::OS),
                None => println!("Showing configuration\n{:?}", read_config())
            }
        }
    }
    Ok(())
}
