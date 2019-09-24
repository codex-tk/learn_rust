extern crate greprs;

use std::process;
use std::env;
use greprs::{Config,run};

fn main() {
    let args:Vec<String> = env::args().collect();
    let cfg = Config::from_args(&args)
        .unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });
    if let Err(e) = run(cfg) {
        panic!("{}" , e);
        process::exit(1);
    }
}
