// TODO: Allow things like x = x + 3 
// instead of number x = x + 3

use std::collections::LinkedList;
use crate::token::{ Token as Token, Expr as Expr, BinaryOperator as BinaryOperator };
use crate::scanner;

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
    if Some(&Token::R_BRACK("]".to_string())) == token_list.front() {
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
    
    let left_token = match token_list.front() {
        Some(Token::NUM_VALUE(_)) | Some(Token::IDENT(_)) => token_list.pop_front().unwrap(),
        _ => return None,
    };

    let left_expr = left_token.to_expr();

    // Check if next token is a binary operator 
    let is_binop = matches!(token_list.front(),
        Some(Token::PLUS(_)) | Some(Token::MINUS(_)) |
        Some(Token::STAR(_)) | Some(Token::SLASH(_)) | Some(Token::MOD(_))
    );

    if !is_binop {
        token_list.push_front(match left_expr {
            Expr::Int(v) => Token::NUM_VALUE(v),
            Expr::Ident(n) => Token::IDENT(n),
            _ => unreachable!(),
        });
        return None;
    }

    let op = token_list.pop_front().unwrap().to_binaryoperator();

    let right_token = match token_list.front() {
        Some(Token::NUM_VALUE(_)) | Some(Token::IDENT(_)) => token_list.pop_front().unwrap(),
        _ => return None,
    };

    Some(Expr::Binary {
        op,
        left: Box::new(left_expr),
        right: Box::new(right_token.to_expr()),
    })
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

// An expression must end with a semicolon.
fn expression(token_list: &mut LinkedList<Token>) -> Option<Expr> {
    // check for empty expression
    if matches!(token_list.front(), Some(Token::SEMICLN(_))) {
        token_list.pop_front();
        return Some(Expr::Empty);
    }
    println!("Line: {}, Token list remaining elements: {:?}", line!(), token_list);
    
    // check for declaration, then rvalue
    let valid_expression = match token_list.front() {
        Some(Token::NUM_IDENT(_)) => declaration(token_list),
        _ => rvalue(token_list),
    };

    println!("Line: {}, Token list remaining elements: {:?}", line!(), token_list);
    println!("Line: {}, Valid expression: {:?}", line!(), valid_expression);

    // check for semicolon at the end of the expression.
    if valid_expression.is_none() {
        println!("Line: {}, Token list remaining elements: {:?}", line!(), token_list);
        return None;
    }
    match token_list.pop_front() {
        Some(Token::SEMICLN(_)) => return valid_expression,
        _ => return None,
    }
    println!("Line: {}, Token list remaining elements: {:?}", line!(), token_list);

    return valid_expression;
}

// returns true if it's a valid sentence.
pub fn parse(file: &str) -> bool {
    let mut token_list: LinkedList<Token> = scanner::scan(file);
    
    while !token_list.is_empty() {
        match expression(&mut token_list) {
            None => { println!("Token list remaining elements: {:?}", token_list); return false},  // if its not a valid expression, then return false.
            _ => {}                // otherwise continue.
        }
    }
    println!("Token list remaining elements: {:?}", token_list);
    return true;
}
