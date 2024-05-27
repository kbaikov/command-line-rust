use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(version, about, author)]
pub struct Config {
    #[arg(default_value = "-", help = "Input file(s)")]
    files: Vec<String>,

    #[arg(
        long,
        short = 'n',
        help = "Number of lines to print.",
        conflicts_with = "bytes",
        default_value = "10",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,

    #[arg(
        long,
        short = 'c',
        help = "Number of bytes to print.",
        conflicts_with = "lines"
    )]
    bytes: Option<usize>,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let args = Config::parse();
    Ok(args)
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }

                if let Some(num_bytes) = config.bytes {
                    // cool shorter way to read n bytes is:
                    // let bytes: Result<Vec<_>, _> = file.bytes().take(num_bytes).collect();
                    // print!("{}", String::from_utf8_lossy(&bytes?));
                    // The underscores (_) indicate partial type annotation, which basically instructs the compiler to infer the types.

                    let mut reader = BufReader::with_capacity(num_bytes, file);
                    println!("{}", String::from_utf8_lossy(&reader.fill_buf()?));
                } else {
                    for line in file.lines().take(config.lines as usize) {
                        println!("{}", line?);
                    }
                }
            }
        }
    }
    Ok(())
}
