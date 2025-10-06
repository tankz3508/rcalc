mod lexer;
mod parser;
use std::io;
#[derive(Debug, Clone)]
enum TOKEN {
    NUMBER(f64),
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    LPAREN,
    RPAREN
}

fn eval(expr: &parser::Expr) -> f64 {
    match expr {
        parser::Expr::Number(n) => *n,
        parser::Expr::Op(left, op, right) => {
            let l = eval(left);
            let r = eval(right);
            match op {
                TOKEN::PLUS => l + r,
                TOKEN::MINUS => l - r,
                TOKEN::MULTIPLY => l * r,
                TOKEN::DIVIDE => l / r,
                _ => panic!("Invalid operator"),
            }
        }
    }
}

fn main() {
    let mut expr_str = String::new();

    println!("Enter expression: ");
    io::stdin().read_line(&mut expr_str).expect("Error reading input");

    let tokens = lexer::lex_expr(&expr_str);
    let mut parser = parser::Parser::new(tokens);
    let tree = parser.parse_expr();

    println!("Result = {}", eval(&tree));
}