use std::collections::LinkedList;
use crate::token::Token as Token;
use crate::scanner;


// rvalue: either a math expression, a numeric literal, or an identifier.
fn rvalue(token_list: &mut LinkedList<Token>) -> bool {
    println!("Rvalue: Front token: {:?}", token_list.front());
    if math_expression(token_list) {
        return true;
    }
    // works similar to equal
    else if token_list.front() == Some(&Token::NUM_VALUE) {
        token_list.pop_front();
        return true;
    }
    return false;
}

fn binary_operator(token_list: &mut LinkedList<Token>) -> bool {
    match token_list.front() {
        Some(Token::PLUS | Token::MINUS | Token::DIV) => {}
        _ => return false,
    }

    match token_list.front() {
        Some(Token::NUM_VALUE) => true,
        _ => false,
    }
}


fn unary_operator(token_list: &mut LinkedList<Token>) -> bool {
    if token_list.front() == Some(&Token::DBL_PLUS) {
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
    if token_list.iter().next() == Some(&Token::NUM_IDENT){
        token_list.pop_front();
    } else {
        return false;
    }
        
    // First token must be IDENT
    if token_list.iter().next() == Some(&Token::IDENT) {
        token_list.pop_front();
    } else {
        return false;
    }
    match token_list.iter().next() {
        // Case: IDENT = MATH
        Some(Token::EQ) => {
            token_list.pop_front();
            match token_list.front() {
                Some(Token::NUM_VALUE) => {
                    return rvalue(token_list);
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
        //token_list.pop_front();
        valid_sentence = declaration(token_list);
    }
    // If the declaration is ok, check for a semicolon.
    if let Some(Token::SEMICLN) = token_list.front() {
        valid_sentence = true;
    }
    else {
        valid_sentence = false;
    }
    return valid_sentence;
}

// returns true if it's a valid sentence.
pub fn parse(file: &str) -> bool {
    let mut token_list: LinkedList<Token> = scanner::scan(file);
    if !expression(&mut token_list) {
        return false;
    }    

    return true;
}
