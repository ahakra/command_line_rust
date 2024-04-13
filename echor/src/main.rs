use clap::{arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
            .version("0.1.0")
            .author("Amd AK <ahakra@outlook.com")
            .about("Rust echo")
            .arg(
                arg!(text: [TEXT] "Input Text").required(true).action(ArgAction::Append),
            )
            .arg(
               arg!(-n --omitnewline "Do not print newline").required(false),
            )

            .get_matches();

            
    let text: Vec<&str> = matches
            .get_many("text")
            .unwrap()
            .map(String::as_str)
            .collect();   
         
    let omit_new_line = matches.get_flag("omitnewline");
    
   
    
    print!(
        "{}{}",
        text.join(" "),
        if omit_new_line { "" } else { "\n" }
    );
}