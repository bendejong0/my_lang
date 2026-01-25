use std::collections::LinkedList;
use crate::token::Token as Token;
use crate::scanner;


// TODO: implement this.
fn rvalue(tokens: &mut Vec<Token>) -> bool {
    math_expression(tokens)
        || matches!(tokens.first(), Some(Token::NUM_VALUE)) && {
            tokens.pop_front();
            true
        }
        || matches!(tokens.first(), Some(Token::IDENT)) && {
            tokens.pop_front();
            true
        }
}

fn binary_operator(token_list: &mut LinkedList<Token>) -> bool {
    match token_list.pop_front() {
        Some(Token::PLUS | Token::MINUS | Token::DIV) => {}
        _ => return false,
    }

    match token_list.pop_front() {
        Some(Token::NUM_VALUE) => true,
        _ => false,
    }
}


fn unary_operator(token_list: &mut LinkedList<Token>) -> bool {
    if token_list.front() == Some(Token::DBL_PLUS) {
        return true;
    }

    return false;
}

fn math_expression(token_list: &mut LinkedList<Token>) -> bool {
// Checks to make sure a math expression is valid.
// Inputs: LinkedList of Tokens
// Outputs: Bool. If math_expression is valid, then true. Else, false
    
    let mut iter = token_list.iter();
    match iter.next() {
        Some(Token::NUM_VALUE) => {}
        _ => return false,
    }
    token_list.pop_front();
    if unary_operator(token_list) {
        return true;
    }
    else if binary_operator(token_list) {
        return true;
    }

    return false;
}

fn declaration(token_list: &mut LinkedList<Token>) -> bool {
// Checks to make sure a declaration is valid.
// Inputs: LinkedList of Tokens
// Outputs: Bool. If the declaration is valid, then true. Otherwise, false.
//
// Note: Consumes tokens used during the declaration.

    // Create an iterator for lookahead
    let mut iter = token_list.iter();
    match iter.next() {
        Some(Token::NUM_IDENT) => { iter.next(); token_list.pop_front(); }
        _ => return false,
    }
    // First token must be IDENT
    match iter.next() {
        Some(Token::IDENT) => { iter.next(); token_list.pop_front(); }
        _ => return false,
    }

    
    match iter.next() {
        // Case: IDENT = MATH
        Some(Token::EQ) => {
            match iter.next() {
                Some(Token::NUM_VALUE) => {
                    math_expression(token_list); 
                    return true;
                }
                _ => return false,
            }
        }
        _ => return false,
    }
}

// right now, the only expression is addition or declaration.
// An expression must end with a semicolon.
fn expression(token_list: &mut LinkedList<Token>) -> bool {
    let mut valid_sentence: bool= true;
    if token_list.front() == Some(&Token::NUM_IDENT) {
        token_list.pop_front();
        valid_sentence = declaration(token_list);
    }
    // If the declaration is ok, check for a semicolon.
    if let Some(Token::SEMICLN) = token_list.front() {
        valid_sentence = true;
    }
    return valid_sentence;
}

// returns true if it's a valid sentence.
pub fn parse(file: &str) -> bool {
    let mut token_list: LinkedList<Token> = scanner::scan(file);
    

    return true;
}
