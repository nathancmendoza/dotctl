
extern crate clap;
extern crate resolve_path;
extern crate symlink;

mod config;
mod links;
mod hooks;
mod cli;

use std::error::Error;
use std::env;
use std::io;

use clap::Parser;

use crate::cli::DotctlInvocation;
use crate::config::{read_config, CONFIG_FILE};
use crate::links::{setup_link, LinkSpec, LinkMode, teardown_link};

fn use_config(config_path: &str) -> io::Result<()> {
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

fn teardown_config(app_conf: &str) -> io::Result<()> {
    let configs = read_config()?;
    for item in configs.all_configs().filter(| config | *config.assigned_name() == app_conf).filter(| config | config.config_os() == std::env::consts::OS) {
        for link in item.links() {
            teardown_link(link)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let invoke = DotctlInvocation::parse();
    match invoke.action {
        cli::DotctlActionWord::Use { config_path } => {
            println!("Using {} as the config file source", config_path);
            use_config(&config_path)?;
        },
        cli::DotctlActionWord::Setup { app_conf } => {
            println!("Setup {} configuration on {}", app_conf, std::env::consts::OS);
            setup_config(&app_conf)?;
        },
        cli::DotctlActionWord::Teardown { app_conf } => {
            println!("Teardown {} configuration on{}", app_conf, std::env::consts::OS);
            teardown_config(&app_conf)?;
        }
   }
    Ok(())
}
