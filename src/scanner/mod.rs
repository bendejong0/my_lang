use std::process;
use std::fs;
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
    let digit_re = Regex::new(r"^\d+$").unwrap();
    if digit_re.is_match(s){
        return Token::NUM
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

pub fn scan(file: &str){
    let contents = match std::fs::read_to_string(file){
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            process::exit(1);
        }
    };

    let split: Vec<&str> = contents.split_whitespace().collect();
    let mut token_list: LinkedList<Token> = LinkedList::new();

    for part in split {
        token_list.push_back(tokenizer(part));
    }
    for token in token_list.iter() {
        println!("{}", token);
    }
}
