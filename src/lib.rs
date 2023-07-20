use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),

            Ok(open_file) => {
                let lines = open_file.lines();
                let mut line_number = 1;
                for line in lines {
                    match line {
                        Ok(x) => {
                            if config.number_lines {
                                println!("     {line_number}\t{x}");
                                line_number += 1;
                            } else if config.number_nonblank_lines {
                                if !x.trim().is_empty() {
                                    println!("     {line_number}\t{x}");
                                    line_number += 1;
                                } else {
                                    // print a newline to preserve it
                                    println!("");
                                }
                            } else {
                                println!("{x}");
                            }
                        }
                        Err(err) => eprintln!("Failed to print line {}", err),
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cat_rust")
        .version("0.1.0")
        .author("Popher Gemzon <gemzon.markchristopher@gmail.com")
        .about("Rust implementation of 'cat' command")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Files you input to cat_rust")
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("number_lines")
                .value_name("NUMBER_LINES")
                .help("Append a number on each line")
                .short("n")
                .long("number")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .value_name("NUMBER_NONBLANK_LINES")
                .help("Append numbers only on non-blank lines")
                .short("b")
                .long("number-nonblank")
                .takes_value(false)
                .conflicts_with("number_lines"),
        )
        .get_matches();

    Ok(Config {
        // We can safely call unwrap here since we are sure that that there is a default
        // value of '-' for 'files' if no input is given.
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
