extern crate greprs;

use std::process;
use std::env;
use greprs::{Config,run};

fn main() {
    let cfg = Config::from_args(env::args().collect()).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    run(cfg);    
}
