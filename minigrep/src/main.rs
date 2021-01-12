extern crate minigrep;

use minigrep::Config; 
use std::env; 
use std::process;

fn main() {
    let args = env::args(); 

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}" , err); 
        process::exit(1); 
    });  

    eprintln!("Searching for {}", config.query);

    eprintln!("In file {}", config.file_name);

    if let Err(err) = minigrep::run(&config) {
        eprintln!("Apllication error occured! , {}" , err); 

        process::exit(1); 
    }

}
