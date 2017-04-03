extern crate regex;
extern crate walkdir;

use std::path;
use std::str;
use std::env;
use std::error;
use std::fmt;
use std::process::exit;

mod cli;
mod find;

use find::Find;

fn run() -> Result<i32, String> {
    let args = match cli::Parser::new() {
        Ok(Some(args)) => args,
        Ok(None) => {
            println!("{}", cli::Parser::usage());
            return Ok(0);
        }
        Err(error) => return Err(format!("{}", error))
    };

    Ok(Find::from_parser(args).run())
}

fn main() {
    let code: i32 = match run() {
        Ok(res) => res,
        Err(error) => {
            println!("{}", error);
            1
        }
    };

    exit(code);
}

