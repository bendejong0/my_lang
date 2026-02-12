// TODO: Allow things like x = x + 3 
// instead of number x = x + 3

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
        Some(Token::IDENT(name)) => { let n = name.clone(); token_list.pop_front(); return Some(Expr::Ident(n)); }
        Some(Token::NUM_VALUE(val)) => { let v = *val; token_list.pop_front(); return Some(Expr::Int(v)); }
        _ => return None,
    }
}

fn list(token_list: &mut LinkedList<Token>) -> Option<Expr> {
    // Checks to make sure a list is valid.
    // Inputs: LinkedList of Tokens
    // Outputs: Bool. If the list is valid, then true. Otherwise, false.
    match token_list.front() {
        Some(Token::L_BRACK(_)) => { token_list.pop_front(); }
        _ => return None,
    }
    // check for empty bracket
    if Some(&Token::L_BRACK("[".to_string())) == token_list.front() {
        token_list.pop_front();
        return Some(Expr::List(Vec::new()));
    }

    let mut list_items: Vec<Expr> = Vec::new();
    if let Some(expr) = rvalue(token_list) {
        list_items.push(expr);
    }

    while token_list.front() == Some(&Token::COMMA(",".to_string())) {
        token_list.pop_front();
        // if we find an rvalue, then put it on the list_items.
        match rvalue(token_list) {
            Some(expr) => { list_items.push(expr); }
            _ => { return None }
        }
    }
    match token_list.front() {
        Some(Token::R_BRACK(_)) => { token_list.pop_front(); return Some(Expr::List(list_items)); }
        _ => return None,
    }
}

fn unary_operator(token_list: &mut LinkedList<Token>) -> bool {
    if token_list.front() == Some(&Token::DBL_PLUS("++".to_string())) {
        return true;
    }
    return false;
}

fn math_expression(token_list: &mut LinkedList<Token>) -> Option<Expr> {
// Checks to make sure a math expression is valid.
// Inputs: LinkedList of Tokens
// Outputs: Bool. If math_expression is valid, then true. Else, false
    
    // check for binary, then unary, then just a number or identifier.
    match token_list.front().clone().unwrap() {
        Token::NUM_VALUE(_) => {},
        Token::IDENT(_) => {},
        _ => return None,
    }

    let number = (*token_list.front().unwrap()).clone();
    token_list.pop_front();
    match token_list.front().unwrap() {
        // check for binary operators
        Token::PLUS(_) 
        | Token::MINUS(_) 
        | Token::STAR(_) 
        | Token::SLASH(_) => {
            let local_op: BinaryOperator = match token_list.front().unwrap() {
                Token::PLUS(_) => BinaryOperator::Add,
                Token::MINUS(_) => BinaryOperator::Sub,
                Token::STAR(_) => BinaryOperator::Mul,
                Token::SLASH(_) => BinaryOperator::Div,
                _ => unreachable!("{:?}", number.clone()),
            };
            token_list.pop_front();
            
            let left_expr = match Some(number.clone()) {
                Some(Token::NUM_VALUE(val)) => Expr::Int(val),
                Some(Token::IDENT(name)) => Expr::Ident(name.clone()),
                _ => {
                    token_list.push_front(number);
                    return None;
                }
            };
            
            match token_list.front().unwrap() {
                Token::IDENT(name) => {
                    let right_name = name.clone();
                    token_list.pop_front();
                    return Some(Expr::Binary { op: (local_op), left: Box::new(left_expr), right: Box::new(Expr::Ident(right_name)) });
                }
                Token::NUM_VALUE(val) => {
                    let right_val = *val;
                    token_list.pop_front();
                    return Some(Expr::Binary { op: (local_op), left: Box::new(left_expr), right: Box::new(Expr::Int(right_val)) });
                }
                _ => {
                    // put it all back
                    token_list.push_front(number);
                }
            }
        },
        // TODO implement ++ operator
        _ => { 
            match number {
                Token::NUM_VALUE(val) => return Some(Expr::Int(val)),
                Token::IDENT(name) => return Some(Expr::Ident(name)),
                _ => return None,
            }
        },
    };
        
    
    
    match token_list.front() {
        Some(Token::NUM_VALUE(x)) => { let val = *x; token_list.pop_front(); return Some(Expr::Int(val)); }
        _ => {},
    }

    if matches!(token_list.front(), Some(Token::DBL_PLUS(_))) {
        unary_operator(token_list);
    }

    return None;
    
}

fn declaration(token_list: &mut LinkedList<Token>) -> Option<Expr> {
    // Checks to make sure a declaration is valid.
    // Inputs: LinkedList of Tokens
    // Outputs: Bool. If the declaration is valid, then true. Otherwise, false.
    //
    // Note: Consumes tokens used during the declaration.


    // Create an iterator for lookahead
    if matches!(token_list.front(), Some(Token::NUM_IDENT(_))){
        token_list.pop_front();
    } else {
        return None;
    }
        
    // First token must be IDENT
    if matches!(token_list.front(), Some(Token::IDENT(_))) {
        token_list.pop_front();
    } else {
        return None;
    }
    match token_list.front() {
        // Case: IDENT = MATH
        Some(Token::EQ(_)) => {
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
    if matches!(token_list.front(), Some(Token::SEMICLN(_))) {
        token_list.pop_front();
        return Some(Expr::Empty);
    }
    
    let valid_expression: Option<Expr> = if matches!(token_list.front(), Some(Token::NUM_IDENT(_))) {
        declaration(token_list)
    }
    else{
        rvalue(token_list)
    };

    if valid_expression.is_none() {
        return None;
    }
    else if matches!(token_list.front(), Some(Token::SEMICLN(_))) {
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
