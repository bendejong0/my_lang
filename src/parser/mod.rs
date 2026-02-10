use std::collections::LinkedList;
use crate::token::{ self, Token as Token };
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
    if matches!(token_list.front(), Some(Token::R_BRACK(_))) {
        token_list.pop_front();
        return Some(Expr::List(vec![]));
    }

    let mut list_items = vec![];
    if let Some(expr) = rvalue(token_list) {
        list_items.push(expr);
    } else {
        return None;
    }

    while matches!(token_list.front(), Some(Token::COMMA(_))) {
        token_list.pop_front();
        if let Some(expr) = rvalue(token_list) {
            list_items.push(expr);
        } else {
            return None;
        }
    }
    match token_list.front() {
        Some(Token::R_BRACK(_)) => { token_list.pop_front(); return Some(Expr::List(list_items)); }
        _ => return None,
    }
}

fn binary_operator(token_list: &mut LinkedList<Token>) -> Option<Expr> {
    let local_op: BinaryOperator = match token_list.front() {
        Some(Token::PLUS(_)) => BinaryOperator::Add,
        Some(Token::MINUS(_)) => BinaryOperator::Sub,
        Some(Token::STAR(_)) => BinaryOperator::Mul,
        Some(Token::SLASH(_)) => BinaryOperator::Div,
        _ => return None,
    };

    token_list.pop_front();
    match rvalue(token_list) {
        Some(right_expr) => {
            Some(Expr::Binary { op: local_op, left: Box::new(Expr::Int(0)), right: Box::new(right_expr) })
        }
        _ => None,
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
    let number = token_list.front();
    token_list.pop_front();
    match token_list.front().unwrap() {

        // check for binary operators
        Token::PLUS(_) 
        | Token::MINUS(_) 
        | Token::STAR(_) 
        | Token::SLASH(_) => {
            let local_op: BinaryOperator = todo!("Create casting from token to binary operator on line {:?}", line!()); // TODO: implement casting from Token to Binary Operator
            token_list.pop_front();
            match token_list.front().unwrap() {
                Token::IDENT(_)
                | Token::NUM_VALUE(_) => {
                    return Some(Expr::Binary { op: (local_op), left: Box<Expr>(number), right: Box<Token>(token_list.front().unwrap()) });
                }
                _ => {
                    // put it all back
                    token_list.push_front(*local_op);
                }
            }

        },
        _ => {},
    };
    
    match binary_operator(token_list) {
        Some(expr) => return Some(expr),
        None => {}
    };

    if(token_list.front() == Some(&Token::DBL_PLUS("++".to_string()))) {
        //return unary_operator(token_list);
    }
    
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

    // TODO: should this be mutable? 
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
