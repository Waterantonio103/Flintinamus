#![allow(unused)]

use clap::Parser;
use std::error::Error;
use std::fs::{self, *};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead};
use std::time::Instant;
use std::collections::HashMap;
use rayon::prelude::*;

use crate::{
    args::{Args, Context}, 
    help::help,
    filesearch::recurse_files,
    matching::{relaxed, strict},
    fileprocessing::find_lines_individual,
    errors::FileReadError,
};

mod errors;
mod help;
mod args;
mod filesearch;
mod fileprocessing;
mod matching;

fn main() -> Result<(), FileReadError> {
    let args = Args::parse();

    run(&args);

    Ok(())
}

fn run(args: &Args) -> Result<(), FileReadError> {
    let paths = &args.directory;
    let target = &args.target;

    let context = if let Some(full) = args.context {
        Some(Context::Full(full))
    } else if let Some(right) = args.after_context {
        Some(Context::Right(right))
    } else if let Some(left) = args.before_context {
        Some(Context::Left(left))
    } else {
        None
    };
    

    for path in paths {
        let result = recurse_files(path, args.recursive)?;
        //think of handling empty file
        let results = result.par_iter()
            .map(|file| {
                find_lines_individual(
                    file, 
                    target.as_str(), 
                    args.whole, 
                    args.insensitive, 
                    args.only, 
                    args.invert, 
                    args.max_count,
                    context,
                )
                    .map(|matches| (file, matches))
            })
            .collect::<Result<HashMap<&PathBuf, Vec<(usize, String)>>, FileReadError>>()?;
            // dbg!(&results);
        let to_read = results.clone();
    
        for (file, matches) in results {
            let count = matches.len();
            if !matches.is_empty() {
                if args.count && args.line_numbers {
                    println!("{file:?}");
                    for (line_number, line) in &matches {
                        println!("{line_number}:{line}");
                    }
                    println!("total matches: {count}");
                } else if args.count {
                    println!("{file:?}:{count}");
                } else if args.count && args.files_with_matches {
                    println!("{file:?}:{count}");
                } else if args.files_with_matches {
                    println!("{file:?}");
                } if args.files_without_matches {
                    //to pass it non-empty matches
                } if args.quiet {
                    //to pass it non-empty matches
                } else {
                    println!("{file:?}");
                    for (line_number, line) in &matches {
                        if args.line_numbers {
                            println!("{line_number}:{line}");
                        } else {
                            println!("{line}");
                        }
                    }
                }
            } else {
                if args.quiet {
                    //do nothing
                } else if args.files_without_matches {
                    println!("{file:?}");
                }
            }
        }
        //check for no matches
        let no_matches = to_read.values().all(|x| x.is_empty());
        if no_matches {
            println!("No matches found");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = find_lines_individual(&PathBuf::from("src/tests.txt"), "hello", false, false, false, false, None, Context);
    }
}
