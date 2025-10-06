use crate::TOKEN;

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Op(Box<Expr>, TOKEN, Box<Expr>),
}

pub struct Parser {
    tokens: Vec<TOKEN>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TOKEN>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&TOKEN> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn parse_fac(&mut self) -> Expr {
        match self.current() {
            Some(TOKEN::NUMBER(n)) => {
                let val = *n;
                self.advance();
                Expr::Number(val)
            }
            Some(TOKEN::LPAREN) => {
                self.advance();
                let expr = self.parse_expr();
                self.advance();
                expr
            }
            _ => panic!("Unexpected token"),
        }
    }

    fn parse_term(&mut self) -> Expr {
        let mut node = self.parse_fac();

        while let Some(token) = self.current() {
            match token {
                TOKEN::MULTIPLY | TOKEN::DIVIDE => {
                    let op = token.clone();
                    self.advance();
                    let right = self.parse_fac();
                    node = Expr::Op(Box::new(node), op, Box::new(right));
                }
                _ => break,
            }
        }

        node
    }

    pub fn parse_expr(&mut self) -> Expr {
        let mut node = self.parse_term();

        while let Some(token) = self.current() {
            match token {
                TOKEN::PLUS | TOKEN::MINUS => {
                    let op = token.clone();
                    self.advance();
                    let right = self.parse_term();
                    node = Expr::Op(Box::new(node), op, Box::new(right));
                }
                _ => break,
            }
        }

        node
    }
}