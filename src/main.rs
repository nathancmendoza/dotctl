
extern crate clap;
extern crate resolve_path;
extern crate symlink;

mod config;
mod cli;

use std::io;

use clap::Parser;

use crate::cli::DotctlInvocation;
use crate::config::read_config;


fn main() -> io::Result<()> {
    let invoke = DotctlInvocation::parse();
    match invoke.action {
        _ => println!("{:?}", read_config()?)
   }
    Ok(())
}
