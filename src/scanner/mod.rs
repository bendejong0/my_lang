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
    // Match the entire string: optional number followed by optional comma
    Regex::new(r"^[1-9][0-9]*,?$")
        .unwrap()
        .is_match(s)
}

fn is_ident(s: &str) -> bool {
    Regex::new(r"^[a-zA-Z_][a-zA-Z_0-9]*$")
        .unwrap()
        .is_match(s)
}


fn tokenizer(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_num = String::new();
    //"for", "if", "else", "number", "main"
    // if it's a reserved word
    if is_reserved_word(s) {
        match s {
            "for" => tokens.push(Token::FOR),
            "number" => tokens.push(Token::NUM_IDENT),
            "if" => tokens.push(Token::IF),
            "else" => tokens.push(Token::ELSE),
            "main" => tokens.push(Token::MAIN),
            _ => {},
        }
    }

    for ch in s.chars() {
        match ch {
            '+' | '-' | '*' | '/' | '%' | '(' | ')' | '[' | ']' | ',' | '=' | ';' | '.' => {
                // If we have a number built up, push it first
                if !current_num.is_empty() {
                    if is_num(&current_num) {
                        tokens.push(Token::NUM_VALUE);
                    } else if is_ident(&current_num) && !is_reserved_word(&current_num) {
                        tokens.push(Token::IDENT);
                    }
                    current_num.clear();
                }
                
                // Handle the operator/symbol
                match ch {
                    '+' => tokens.push(Token::PLUS),
                    '-' => tokens.push(Token::MINUS),
                    '*' => tokens.push(Token::STAR),
                    '/' => tokens.push(Token::SLASH),
                    '%' => tokens.push(Token::MOD),
                    '(' => tokens.push(Token::L_PAREN),
                    ')' => tokens.push(Token::R_PAREN),
                    '[' => tokens.push(Token::L_BRACK),
                    ']' => tokens.push(Token::R_BRACK),
                    ',' => tokens.push(Token::COMMA),
                    '=' => tokens.push(Token::EQ),
                    ';' => tokens.push(Token::SEMICLN),
                    '.' => tokens.push(Token::DOT),
                    _ => {}
                }
            }
            _ => current_num.push(ch),
        }
    }

    // Handle any remaining number or identifier
    if !current_num.is_empty() {
        if is_num(&current_num) {
            tokens.push(Token::NUM_VALUE);
        } else if is_ident(&current_num) && !is_reserved_word(&current_num) {
            tokens.push(Token::IDENT);
        }
    }

    tokens
}

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
            let tokens = tokenizer(token_str);
            for token in tokens {
                token_list.push_back(token);
            }
        }

        if parts.peek().is_some() {
            token_list.push_back(Token::SEMICLN);
        }
    }
    println!("Token List: {:?}", token_list);

    token_list
}
