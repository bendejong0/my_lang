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
    println!("Checking for number {:?}", s);

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


fn tokenizer(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    if is_num(s) {
        if s.ends_with(",") {
            // Split the number and the comma
            tokens.push(Token::NUM_VALUE); // Return NUM_VALUE first
            tokens.push(Token::COMMA); // Then COMMA
            return tokens;
        }
        tokens.push(Token::NUM_VALUE);
    } else if is_ident(s) && !is_reserved_word(s) {
        tokens.push(Token::IDENT);
    } else {
        match s {
            ")" => tokens.push(Token::R_PAREN),
            "(" => tokens.push(Token::L_PAREN),
            "for" => tokens.push(Token::FOR),
            "{" => tokens.push(Token::L_CURLY),
            "}" => tokens.push(Token::R_CURLY),
            "+" => tokens.push(Token::PLUS),
            "-" => tokens.push(Token::MINUS),
            "*" => tokens.push(Token::STAR),
            "/" => tokens.push(Token::SLASH),
            "%" => tokens.push(Token::MOD),
            "if" => tokens.push(Token::IF),
            "else" => tokens.push(Token::ELSE),
            ";" => tokens.push(Token::SEMICLN),
            "::" => tokens.push(Token::DBL_CLN),
            "++" => tokens.push(Token::DBL_PLUS),
            "." => tokens.push(Token::DOT),
            ".." => tokens.push(Token::DBL_DOT),
            "number" => tokens.push(Token::NUM_IDENT),
            "main" => tokens.push(Token::MAIN),
            "=" => tokens.push(Token::EQ),
            "[" => tokens.push(Token::L_BRACK),
            "]" => tokens.push(Token::R_BRACK),
            "," => tokens.push(Token::COMMA),
            _ => panic!("Unknown character: {}", s),
        }
    }

    tokens
}

// TODO: Improve to allow declarations such as:
// number x=5;
// currently you must do number x = 5;

// TODO: Improve things like [3, 4, 5] so that the brackets and commas are handled without spaces.
// currently you must do [ 3 , 4 , 5 ]
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

        // Put the semicolon back *if it actually existed*
        if parts.peek().is_some() {
            token_list.push_back(Token::SEMICLN);
        }
    }
    println!("Token List: {:?}", token_list);

    token_list
}