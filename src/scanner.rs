use std::process::exit;
use std::collections::LinkedList;
use regex::Regex;
#[derive(Debug)]
enum Token{
    IDENT,
    PLUS,
    MINUS,
    MUL,
    DIV,
    MOD,
    R_CURLY,
    L_CURLY,
    L_PAREN,
    R_PAREN,
    IF,
    ELSE,
    FOR,
    SEMICLN,
    DBL_CLN,
    DBL_PLUS,
    DOT,
    DBL_DOT,
    NUM
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn tokenizer(s: &str) -> Token {
    digitRe = Regex::new(r"\d+");
    if let Some(num) = digitRe.captures(s){
        return Token::NUM;
    }
    else {
        println!("match)");
    }
    match s {
       ")" => Token::R_PAREN,
       "(" => Token::L_PAREN,
       "for" => Token::FOR,
       "{" => Token::L_CURLY,
       "}" => Token::R_CURLY,
       "+" => Token::PLUS,
       "-" => Token::MINUS,
       "*" => Token::MUL,
       "/" => Token::DIV,
       "%" => Token::MOD,
       "if" => Token::IF,
       "else" => Token::ELSE,
       ";" => Token::SEMICLN,
       "::" => Token::DBL_CLN,
       "++" => Token::DBL_PLUS,
       "." => Token::DOT,
       ".." => Token::DBL_DOT,
        _ => Token::IDENT
    }
}

fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Correct usage: {} <filename>", args[0]);
        exit(1);
    } 
    let filename = &args[1];
    let contents = match std::fs::read_to_string(filename){
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            exit(1);
        }
    };

    let split: Vec<&str> = contents.split_whitespace().collect();
    let mut tokenize: LinkedList<Token> = LinkedList::new();
    for part in split {
        tokenize.push_back(tokenizer(part));
    }
    for token in tokenize.iter() {
        print!("{}", token);
    }
}
