use std::env;
use std::fs::File;
use std::io::prelude::*;
mod scanner;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        println!("{arg1}");
        _ = scanner::scan(&arg1)
    }
    else{
        println!("No arguments given!");
    }
    
    
}
