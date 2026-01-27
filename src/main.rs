use std::env;
// use std::fs::File;
use std::path::Path;
pub mod scanner;
pub mod parser;
pub mod token;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        println!("{arg1}");
        assert!(Path::new(&arg1).exists(), "File doesnt exist");
        // output the token list to a file i guess.
        if parser::parse(&arg1) {
            println!("The file is valid!");
        } 
        else {
            println!("The file is invalid!");
        }
        
    }
    else{
        println!("No arguments given!");
    }
}
