
use std::fs::File;
use std::io::prelude::*;
use std::error::Error; 

pub struct Config {
    pub query: String , 
    pub file_name: String , 
}

impl Config{
    pub fn new(mut args: std::env::Args) -> Result<Config , &'static str>{

        args.next(); 
        
        let query = match args.next(){
            Some(val) => val, 
            None => return Err("No query was given"), 
        } ;

        let file_name = match args.next(){
            Some(val) => val, 
            None => return Err("No file name was given"), 
        };

        Ok(Config{query , file_name}) 
    }
}

pub fn run(config: &Config) -> Result<() , Box<dyn Error>>{
    let mut f = File::open(&config.file_name)?;

    let mut contents = String:: new(); 
    
    f.read_to_string(&mut contents)?; 

    let res = search(&config.query, &contents); 

    println!("{:?}" ,  res); 

    Ok(())

}


pub fn search<'a> (query: &str , conetnts: &'a str) -> Vec<&'a str> {

    conetnts.lines().filter(|line| {
        line.contains(query) 
    }).collect()

}