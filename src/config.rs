use crate::utils;
use std::fs::File;
use std::io::Error;
use std::io::Read;

fn readFile(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn getConfig() -> Result<(), Error> {
    let content = readFile("config/config.toml").unwrap();
    println!("{}", content);
    Ok(())
}
