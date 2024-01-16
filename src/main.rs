
mod config;
mod links;
mod hooks;

use crate::config::read_config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = match read_config() {
        Ok(conf) => println!("{:?}", conf),
        Err(e) => println!("{}", e)
    };

    Ok(())
}
