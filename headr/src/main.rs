
use std::error::Error;
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{self, BufRead, BufReader};


type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg( value_name = "FILE",action = clap::ArgAction::Append)]
    files: Vec<String>,
  
    #[arg(short, long, value_name = "LINE",default_value="10")]
    lines: usize,
    
    #[arg(short,long,value_name = "BYTE",conflicts_with("lines"),required=false)]
    bytes: Option<usize>,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
        }
}
    
pub fn run(config: Config) -> MyResult<()>  {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                for line in file.lines().take(config.lines) {
                    println!("{}", line?);
                }   
            }
        }
    }
    
    Ok(())
}

fn main() {
    let conf = Config::parse();
    let _ = run(conf);
}
