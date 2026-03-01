use std::collections::HashMap;
use crate::token::{ Expr, BinaryOperator, Stmt };
use std::collections::LinkedList;

fn format_expr(expr: &Expr) -> String {
    match expr {
        Expr::Int(n) => n.to_string(),
        Expr::Ident(name) => name.clone(),
        Expr::Binary { op, left, right } => {
            let op_str = match op {
                BinaryOperator::Add => "+",
                BinaryOperator::Sub => "-",
                BinaryOperator::Mul => "*",
                BinaryOperator::Div => "/",
                BinaryOperator::Mod => "%",
            };
            format!("({} {} {})", format_expr(left), op_str, format_expr(right))
        },
        Expr::List(items) => {
            let inner: Vec<String> = items.iter().map(format_expr).collect();
            format!("[{}]", inner.join(", "))
        },
        Expr::Empty => "<empty>".to_string(),
    }
}

fn format_stmt(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Dec { name, val } => format!("number {} = {}", name, format_expr(val)),
        Stmt::Expr(expr) => format_expr(expr),
    }
}

pub fn interpret(stmts: LinkedList<Stmt>) {
    let mut env: HashMap<String, i64> = HashMap::new();
    for stmt in &stmts {
        println!(">> {}", format_stmt(stmt));
        if let Err(e) = eval_stmt(stmt, &mut env) {
            println!("   => Error: {}", e);
            break;
        }
    }

    println!("\n-- Variables --");
    if env.is_empty() {
        println!("  (none)");
    } else {
        let mut sorted: Vec<(&String, &i64)> = env.iter().collect();
        sorted.sort_by_key(|(k, _)| k.as_str());
        for (name, val) in sorted {
            println!("  {} = {}", name, val);
        }
    }
}

fn eval_stmt(stmt: &Stmt, env: &mut HashMap<String, i64>) -> Result<(), String> {
    match stmt {
        Stmt::Dec { name, val } => {
            let result = eval_expr(val, env)?;
            println!("   => {} = {}", name, result);
            env.insert(name.clone(), result);
        },
        Stmt::Expr(expr) => {
            let result = eval_expr(expr, env)?;
            println!("   => {}", result);
        }
    }
    Ok(())
}

fn eval_expr(expr: &Expr, env: &mut HashMap<String, i64>) -> Result<i64, String> {
    match expr {
        Expr::Int(n) => Ok(*n),
        Expr::Ident(name) => {
            env.get(name).copied().ok_or(format!("Undefined variable: {}", name))
        },
        Expr::Binary { op, left, right } => {
            let l = eval_expr(left, env)?;
            let r = eval_expr(right, env)?;
            match op {
                BinaryOperator::Add => Ok(l + r),
                BinaryOperator::Sub => Ok(l - r),
                BinaryOperator::Mul => Ok(l * r),
                BinaryOperator::Div => {
                    if r == 0 { Err("Division by zero".to_string()) }
                    else { Ok(l / r) }
                },
                BinaryOperator::Mod => Ok(l % r),
            }
        },
        Expr::Empty => Ok(0),
        Expr::List(_) => Err("Can't eval a list to a number yet".to_string()),
    }
}