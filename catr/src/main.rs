use clap::{arg, ArgAction, Command};
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
pub struct Config<'a>{
    files:Vec<&'a str>,
    number_lines:bool,
    number_before_non_blank_lines:bool,
}
pub fn read_file(filedir: &str) -> Result<String, io::Error> {
    let mut file = File::open(filedir)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn main() {
    let matches = Command::new("catr")
            .version("0.1.0")
            .author("Amd AK <ahakra@outlook.com")
            .about("Rust cat")
            .arg(
                arg!(files: [FILE] "Input files(s)").required(true).action(ArgAction::Append),
            )
            .arg(
               arg!(number: -n --number "number all output lines").required(false).conflicts_with("number_nonblank"),
            )
            .arg(
                arg!(number_nonblank : -b --numbernonblank "numbers non-blank lines").required(false),
             )    
           
            .get_matches();

            let files: Vec<&str> = matches
            .get_many("files")
            .unwrap()
            .map(String::as_str)
            .collect();   
         
    
    let conf = Config {
        files:files,
        number_lines: matches.get_flag("number"),
        number_before_non_blank_lines : matches.get_flag("number_nonblank"),
    };
    for f in conf.files {
        println!("FileName: {}",f);
        match read_file(f) {
            Ok(file_content) => {
                let lines: Vec<&str> = file_content.split('\n').collect();
                for (idx, line) in lines.iter().enumerate() {
                    if conf.number_lines {
                        print!("{} ", idx + 1);
                    } else if conf.number_before_non_blank_lines && !line.trim().is_empty() {
                        print!("{} ", idx + 1);
                    }
                    println!("{}", line);
                }
                println!();
            }
            Err(err) => {
                eprintln!("Error reading {}: {}", f, err);
                println!();
                continue; 
            }
        }
    }
    

    }
    
        
