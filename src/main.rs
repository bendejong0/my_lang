use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    if let Some(arg1) = env::args().nth(1) {
        println!("{arg1}");
        
    }
    else{
        println!("No arguments given!");
    }
    
    
}
