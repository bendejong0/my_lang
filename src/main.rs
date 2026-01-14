use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
pub mod scanner;
pub mod parser;
pub mod token;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        println!("{arg1}");
        assert!(Path::new(&arg1).exists(), "File doesnt exist");
        let token_list = scanner::scan(&arg1);
        parser::parse()
    }
    else{
        println!("No arguments given!");
    }
    
    
}
