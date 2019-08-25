
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn from_args(args: Vec<String>) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config{
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}


pub fn run(config: Config){
    let mut f = File::open(config.filename).expect("File Error");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("Error Read");

    println!("{}", contents );
}