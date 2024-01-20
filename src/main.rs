
extern crate clap;
extern crate resolve_path;
extern crate symlink;

mod config;
mod links;
mod hooks;
mod cli;

use std::error::Error;
use std::env;

use clap::Parser;

use crate::cli::DotterInvocation;
use crate::config::{read_config, CONFIG_FILE};
use crate::links::{setup_link, LinkSpec, LinkMode};

fn use_config(config_path: &str) -> Result<(), std::io::Error> {
    let config_link_spec = LinkSpec::new(config_path, CONFIG_FILE, LinkMode::Soft);
    let cwd = env::current_dir()?;
    setup_link(&config_link_spec, &cwd)
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
            use_config(&config_path)?;
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
