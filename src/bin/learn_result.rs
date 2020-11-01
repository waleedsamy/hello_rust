#![allow(dead_code)]
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let path = "README.md";

    let s = read_string_from_file_1(path);
    match s {
        Ok(content) => println!("content of {}:\n{}", path, content),
        Err(e) => panic!("error in reading {} {}", path, e),
    };
    return Ok(());
}

fn read_string_from_file_1(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn read_string_from_file_2(path: &str) -> Result<String, io::Error> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn read_string_from_file_3(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
