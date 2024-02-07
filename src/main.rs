

mod config;
mod cli;

use std::io;

use clap::Parser;

use crate::cli::DotctlInvocation;
use crate::config::DotfileConfiguration;


fn main() -> io::Result<()> {
    let invoke = DotctlInvocation::parse();
    match invoke.action {
        cli::DotctlActionWord::Use { config_path } => {
            println!("{:?}", DotfileConfiguration::read_config())
        },
        cli::DotctlActionWord::Setup { application } => {
            println!("{:?}", DotfileConfiguration::read_config()?.find_appconf(&application))
        },
        cli::DotctlActionWord::Teardown { application } => {
            println!("{:?}", DotfileConfiguration::read_config()?.find_appconf(&application))
        }
   }
    Ok(())
}
