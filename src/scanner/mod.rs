use std::process;
use std::fs;
use std::collections::LinkedList;
use regex::Regex;
use crate::token::Token as Token;


impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn is_num(s: &str) -> bool {
    // numbers cannot begin with any leading zeroes.
    Regex::new(r"^[1-9][0-9]*")
        .unwrap()
        .is_match(s)
}

fn is_ident(s: &str) -> bool {
    Regex::new(r"^[a-zA-Z_][a-zA-Z_0-9]*$")
        .unwrap()
        .is_match(s)
}


fn tokenizer(s: &str) -> Token {
    if is_num(s) {
        return Token::NUM_IDENT;
    }
    else if is_ident(s) {
        return Token::IDENT;
    }
    else {
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
            "number" => Token::NUM_IDENT,
            _ => panic!("Unknown character: {}", s)
        }
    }

}

pub fn scan(file: &str) -> LinkedList<Token>{
    let contents = match std::fs::read_to_string(file){
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            process::exit(1);
        }
    };

    let mut token_list: LinkedList<Token> = LinkedList::new();

    for part in contents.split(';') {
        for token_str in part.split_whitespace() {
            println!("{}", token_str); // See each token string
            token_list.push_back(tokenizer(token_str));
        }
    }

    for token in token_list.iter() {
        println!("{}", token);
    }
    return token_list;
}
