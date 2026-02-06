use std::collections::LinkedList;
use crate::token::Token as Token;
use crate::scanner;


// rvalue: either a math expression, a numeric literal, or an identifier.
fn rvalue(token_list: &mut LinkedList<Token>) -> bool {
    return math_expression(token_list) || list(token_list);
}

fn list(token_list: &mut LinkedList<Token>) -> bool {
    // Checks to make sure a list is valid.
    // Inputs: LinkedList of Tokens
    // Outputs: Bool. If the list is valid, then true. Otherwise, false.
    match token_list.front() {
        Some(Token::L_BRACK) => { token_list.pop_front(); }
        _ => return false,
    }
    // check for empty bracket
    if token_list.front() == Some(&Token::R_BRACK) {
        token_list.pop_front();
        return true;
    }

    if !rvalue(token_list){
        return false;
    }

    while token_list.front() == Some(&Token::COMMA) {
        token_list.pop_front();
        if !rvalue(token_list){
            return false;
        }
    }

    match token_list.front() {
        Some(Token::R_BRACK) => { token_list.pop_front(); return true; }
        _ => return false,
    }
}

fn binary_operator(token_list: &mut LinkedList<Token>) -> bool {
    match token_list.front() {
        Some(Token::PLUS | Token::MINUS | Token::STAR | Token::SLASH) => {}
        _ => return false,
    }
    token_list.pop_front();
    match token_list.front() {
        Some(Token::NUM_VALUE) => { token_list.pop_front(); return true; }
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

    if token_list.iter().next() == Some(&Token::PLUS) || token_list.iter().next() == Some(&Token::MINUS) || token_list.iter().next() == Some(&Token::SLASH) {
        return binary_operator(token_list);
    }

    else if token_list.iter().next() == Some(&Token::DBL_PLUS) {
        unary_operator(token_list);
    }


    return true;
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

// right now, the only expression is declaration.
// An expression must end with a semicolon.
fn expression(token_list: &mut LinkedList<Token>) -> bool {
    // check for empty expression
    if token_list.front() == Some(&Token::SEMICLN) {
        token_list.pop_front();
        return true;
    }

    let mut valid_expression = if token_list.front() == Some(&Token::NUM_IDENT) {
        declaration(token_list)
    }
    else{ 
        rvalue(token_list)
    };

    if !valid_expression {
        return false;
    }

    else if Some(&Token::SEMICLN) == token_list.front() {
        valid_expression = true;
    }

    return valid_expression;
}

// returns true if it's a valid sentence.
pub fn parse(file: &str) -> bool {
    let mut token_list: LinkedList<Token> = scanner::scan(file);
    return expression(&mut token_list);
}
