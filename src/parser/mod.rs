use std::collections::LinkedList;
use crate::token::Token as Token;
use crate::scanner;

fn declaration(token_list: &mut LinkedList<Token>) -> bool {
// Checks to make sure a declaration is valid.
// Inputs: LinkedList of Tokens
// Outputs: Bool. If the declaration is valid, then true. Otherwise, false.
//
// Note: Consumes tokens used during the declaration.

    // Create an iterator for lookahead
    let mut iter = token_list.iter();

    // First token must be IDENT
    match iter.next() {
        Some(Token::IDENT) => {}
        _ => return false,
    }

    // Look ahead one token
    match iter.next() {
        // Case: IDENT = NUM_VALUE ;
        Some(Token::EQ) => {
            match (iter.next(), iter.next()) {
                (Some(Token::NUM_VALUE), Some(Token::SEMICLN)) => {
                    token_list.pop_front(); // consume IDENT
                    return true;
                }
                _ => return false,
            }
        }

        // Case: IDENT ;
        Some(Token::SEMICLN) => {
            return true;
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
        valid_sentence = false;
    }
    return valid_sentence;
}

// returns true if it's a valid sentence.
pub fn parse(file: &str) -> bool {
    let mut token_list: LinkedList<Token> = scanner::scan(file);
    

    return true;
}
