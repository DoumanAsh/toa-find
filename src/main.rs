extern crate regex;
extern crate walkdir;

use walkdir::{WalkDir};

use std::path;
use std::str;
use std::env;
use std::error;
use std::fmt;
use std::process::exit;

mod cli;

#[inline]
///Filters errors out and prints them
fn filter_error(value: walkdir::Result<walkdir::DirEntry>) -> Option<walkdir::DirEntry> {
    match value {
        Ok(entry) => Some(entry),
        Err(error) => {
            println!("ERROR: {}", error);
            None
        }
    }
}

fn run() -> Result<i32, String> {
    let args = match cli::Parser::new() {
        Ok(Some(args)) => args,
        Ok(None) => {
            println!("{}", cli::Parser::usage());
            return Ok(0);
        }
        Err(error) => return Err(format!("{}", error))
    };

    let paths = args.paths.iter();

    for path in paths {
        let path = path::Path::new(&path);

        if !path.exists() {
            println!("toa: {} cannot access", path.display());
            continue;
        }

        let type_filter = |entry: &walkdir::DirEntry| {
            let entry_type = entry.file_type();

            (entry_type.is_file() && args.flags.file) || (entry_type.is_dir() && args.flags.dir) || false
        };

        let pattern_filter = |entry: &walkdir::DirEntry| {
            let name = entry.file_name().to_str().unwrap();
            args.pattern.is_match(name)
        };

        let walker = WalkDir::new(&path).min_depth(args.opts.hop.0)
                                        .max_depth(args.opts.hop.1)
                                        .follow_links(args.flags.sym)
                                        .into_iter()
                                        .filter_map(filter_error)
                                        .filter(type_filter)
                                        .filter(pattern_filter);
        for entry in walker {
            println!("{}", entry.path().display());
        }
    }

    Ok(0)
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

