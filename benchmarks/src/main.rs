extern crate clap;
extern crate i2d_tree;

use std::io::stdin;
use clap::{Arg, App};

fn main() -> i32 {
    let matches = App::new("geoloc integration tests")
                      .version("0.1.0")
                      .author("Mark Shevchenko")
                      .arg(Arg::with_name("command")
                               .help("Commands are: print")
                               .required(true)
                               .index(0))
                      .get_matches();

    let command = matches.value_of("command")
                         .unwrap_or("help");

    let result = match command {
        "print" => print(),
        "help" | _ => help(),
    }

    match result {
        Result=> 0,
        Err(E) => println!("Error: {}", E)
            -1,
    }
}

fn print() -> Result<()> {
    let mut line = String::new();
    while stdin
        let line = std::mem::take(&mut line);
    }
}

fn help() {
    println!("Usage: tests print");
    println!("Prints the 2d-tree built from input.");
    println!("Input are LATITUDE LONGITUDE VALUE");
}
