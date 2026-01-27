use std::process;
use std::collections::LinkedList;
use regex::Regex;
use crate::token::Token as Token;

const RESERVED_WORDS: [&str; 5] = [
    "for", "if", "else", "number", "main"
];

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn is_reserved_word(s: &str) -> bool {
    for word in RESERVED_WORDS.iter() {
        if s == *word {
            return true;
        }
    }
    return false;
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
        return Token::NUM_VALUE;
    }
    if is_ident(s) && !is_reserved_word(s) {
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
            "main" => Token::MAIN,
            "=" => Token::EQ,
            _ => panic!("Unknown character: {}", s)
        }
    }
}

// TODO: Improve to allow declarations such as:
// number x=5;
// currently you must do number x = 5;
pub fn scan(file: &str) -> LinkedList<Token> {
    let contents = match std::fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            process::exit(1);
        }
    };

    let mut token_list: LinkedList<Token> = LinkedList::new();

    let mut parts = contents.split(';').peekable();

    while let Some(part) = parts.next() {
        for token_str in part.split_whitespace() {
            token_list.push_back(tokenizer(token_str));
        }

        // Put the semicolon back *if it actually existed*
        if parts.peek().is_some() {
            token_list.push_back(tokenizer(";"));
        }
    }

    token_list
}