
extern crate serde;
extern crate serde_yaml;

use serde_yaml::from_reader;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod config;
mod links;
mod hooks;

use crate::config::DotfileConfiguration;

fn read_yaml_from_file<P: AsRef<Path>>(path: P) -> DotfileConfiguration {
    let file = match File::open(path) {
        Ok(v) => v,
        Err(e) => {
            println!("File opening error: {:?}", e);
            panic!("Failed because of previous error.");
        }
    };

    let reader = BufReader::new(file);

    match from_reader(reader) {
        Ok(v) => v,
        Err(e) => {
            println!("Deserialization error: {:?}", e);
            panic!("Failed because of previous error");
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let value = read_yaml_from_file("test.yaml".to_owned());
    println!("{:?}", value);
    Ok(())
}
