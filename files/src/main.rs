#![allow(unused_variables)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use crate::database::{self as db, Colors::{self, GREEN, RED}};

pub mod database;
fn main() {
    read_file("Cargo.toml").unwrap();
    println!("*****************************");
    read_file_with_buffer("Cargo.toml").unwrap();
    let first_person = db::Person::new(String::from("Mougang"), String::from("Thomas"), 35);
    let second = 
    println!("{:?}", first_person);
    let color = Colors::RED;
    color.show();
    let result =match  color {
        RED => 10,
        GREEN => 20,
        _=>30
    };
    println!("{}", result);
    const MY_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    println!("{}", MY_VERSION.unwrap());
}

fn read_file(path: &str) -> std::io::Result<()> {
    let content = std::fs::read_to_string(path)?;
    for line in content.lines() {
        println!("{}", line);
    }
    Ok(())
}

fn read_file_with_buffer(path: &str) -> std::io::Result<()> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
    Ok(())
}
