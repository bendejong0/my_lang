use std::collections::LinkedList;
use crate::token::{ Token as Token };
use crate::scanner;

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Ident(String),
    Binary {
        op: BinaryOperator,
        left: Box<Expr>,
        right: Box<Expr>
    },
    List(Vec<Expr>),
    Empty,
}

pub enum Stmt {
    Dec {
        name: String,
        val: Expr
    }
}


// rvalue: either a math expression, a numeric literal, or an identifier.
fn rvalue(token_list: &mut LinkedList<Token>) -> Option<Expr> {
    
    match math_expression(token_list) {
        Some(expr) => return Some(expr),
        None => {}
    };
    // if it's not a math expression, then check if it's a list.
    match list(token_list) {
        Some(expr) => return Some(expr),
        None => {}
    };
    // if it's not a math expression or a list, then check if it's an identifier.
    match token_list.front() {
        Some(Token::IDENT) => { token_list.pop_front(); return Some(Expr::Ident(String::new())); }
        Some(Token::NUM_VALUE) => { token_list.pop_front(); return Some(Expr::Int(0)); }
        _ => return None,
    }
}

fn list(token_list: &mut LinkedList<Token>) -> Option<Expr> {
    // Checks to make sure a list is valid.
    // Inputs: LinkedList of Tokens
    // Outputs: Bool. If the list is valid, then true. Otherwise, false.
    match token_list.front() {
        Some(Token::L_BRACK) => { token_list.pop_front(); }
        _ => return None,
    }
    // check for empty bracket
    if token_list.front() == Some(&Token::R_BRACK) {
        token_list.pop_front();
        return Some(Expr::List(vec![]));
    }

    //if Some(rvalue(token_list)) {
    //    return None;
    //}
    if rvalue(token_list).is_none() {
        return None;
    }

    while token_list.front() == Some(&Token::COMMA) {
        token_list.pop_front();
        if rvalue(token_list).is_none() {
            return None;
        }
    }
    match token_list.front() {
        Some(Token::R_BRACK) => { token_list.pop_front(); return Some(Expr::List(vec![])); }
        _ => return None,
    }
}

fn binary_operator(token_list: &mut LinkedList<Token>) -> Option<Expr> {
    match token_list.front() {
        Some(Token::PLUS | Token::MINUS | Token::STAR | Token::SLASH) => {}
        _ => return None,
    }

    let local_op: BinaryOperator = match token_list.front() {
        Some(Token::PLUS) => BinaryOperator::Add,
        Some(Token::MINUS) => BinaryOperator::Sub,
        Some(Token::STAR) => BinaryOperator::Mul,
        Some(Token::SLASH) => BinaryOperator::Div,
        _ => unreachable!(),
    };

    token_list.pop_front();
    match token_list.front() {
        Some(Token::NUM_VALUE) => { token_list.pop_front(); return Some(Expr::Binary { op: local_op, left: Box::new(Expr::Int(0)), right: Box::new(Expr::Int(0)) }); }
        _ => None,
    }
}


fn unary_operator(token_list: &mut LinkedList<Token>) -> bool {
    if token_list.front() == Some(&Token::DBL_PLUS) {
        return true;
    }

    return false;
}

fn math_expression(token_list: &mut LinkedList<Token>) -> Option<Expr> {
// Checks to make sure a math expression is valid.
// Inputs: LinkedList of Tokens
// Outputs: Bool. If math_expression is valid, then true. Else, false
    let mut iter = token_list.iter();
    // TODO: CUrrently broke, fix later.
    match iter.next() {
        Some(Token::NUM_VALUE) => {}, // Some(Expr::Int(n)),
        _ => return None,
    }  
    token_list.pop_front();  // why is this unreachable?

    if token_list.iter().next() == Some(&Token::PLUS)  || 
       token_list.iter().next() == Some(&Token::MINUS) ||
       token_list.iter().next() == Some(&Token::STAR)  || 
       token_list.iter().next() == Some(&Token::SLASH) {
        return binary_operator(token_list);
    }

    else if token_list.iter().next() == Some(&Token::DBL_PLUS) {
        unary_operator(token_list);
    }

    // TODO:
    // THIS SHOULD NOT ALWAYS BE 0
    return Option::Some(Expr::Int(0)); // TEMPORARY VARIABLE REDO LATER
    
}

fn declaration(token_list: &mut LinkedList<Token>) -> Option<Expr> {
    // Checks to make sure a declaration is valid.
    // Inputs: LinkedList of Tokens
    // Outputs: Bool. If the declaration is valid, then true. Otherwise, false.
    //
    // Note: Consumes tokens used during the declaration.


    // Create an iterator for lookahead
    if token_list.iter().next() == Some(&Token::NUM_IDENT){
        token_list.pop_front();
    } else {
        return None;
    }
        
    // First token must be IDENT
    if token_list.iter().next() == Some(&Token::IDENT) {
        token_list.pop_front();
    } else {
        return None;
    }
    match token_list.iter().next() {
        // Case: IDENT = MATH
        Some(Token::EQ) => {
                token_list.pop_front();
                match rvalue(token_list) {
                    Some(expr) => return Some(expr),
                    None => return None
                }
        },
        
        _ => return None,
    } 
}

// right now, the only expression is declaration.
// An expression must end with a semicolon.
fn expression(token_list: &mut LinkedList<Token>) -> Option<Expr> {
    // check for empty expression
    if token_list.front() == Some(&Token::SEMICLN) {
        token_list.pop_front();
        return Some(Expr::Empty);
    }

    // TODO: should this be mutable? 
    let mut valid_expression: Option<Expr> = if token_list.front() == Some(&Token::NUM_IDENT) {
        declaration(token_list)
    }
    else{ 
        rvalue(token_list)
    };

    if valid_expression.is_none() {
        return None;
    }

    else if Some(&Token::SEMICLN) == token_list.front() {
        token_list.pop_front();
    }

    return valid_expression;
}

// returns true if it's a valid sentence.
pub fn parse(file: &str) -> bool {
    let mut token_list: LinkedList<Token> = scanner::scan(file);
    while !token_list.is_empty() {
        match expression(&mut token_list) {
            None => return false,  // if its not a valid expression, then return false.
            _ => {}                // otherwise continue.
        }
    }
    return true;
}
