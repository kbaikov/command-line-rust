use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(version, about="Rust cat", long_about = None)]
pub struct Config {
    #[arg(default_value = "-", help = "Input file(s)")]
    files: Vec<String>,

    #[arg(
        long = "number",
        short = 'n',
        help = "Number lines",
        conflicts_with = "number_nonblank_lines"
    )]
    number_lines: bool,

    #[arg(
        long = "number-nonblank",
        short = 'b',
        help = "Number non-blank lines",
        conflicts_with = "number_lines"
    )]
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    if filename == "-" {
        Ok(Box::new(BufReader::new(io::stdin())))
    } else {
        Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

pub fn get_args() -> MyResult<Config> {
    let args = Config::parse();
    Ok(args)
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Cannot open file {}: {}", filename, err),
            Ok(file) => {
                let mut prev_line = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if config.number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            prev_line += 1;
                            println!("{:6}\t{}", prev_line, line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}
