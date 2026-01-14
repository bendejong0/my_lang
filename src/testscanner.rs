mod scanner;
use std::env;
use std::path::Path;
use std::io::prelude::*;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        scanner::scan(&arg1)
    }
    else{
        println!("No arguments given!");
    }
}