use std::{env, fs, process};
use minigrep::search;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("####################################################");
    println!(
        "#     Searching for {} in file {}",
        config.query, config.path
    );
    println!("####################################################");
    let content = fs::read_to_string(config.path).unwrap();
    println!("{}", content);
    //dbg!(args_string);
}

struct Config {
    query: String,
    path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Missing argumets!");
        }
        Ok(Config {
            query: args[0].clone(),
            path: args[1].clone(),
        })
    }
}
