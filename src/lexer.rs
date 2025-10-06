use crate::TOKEN;

pub fn lex_expr( expr: &str ) -> Vec<TOKEN> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();
    
    while let Some( ch ) = chars.next() {
        match ch {
            '0'..='9' => {
                let mut num = String::from( ch );
                let mut has_dot = false;
                
                while let Some( &next_ch ) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        num.push( chars.next().unwrap() );
                    } else if next_ch == '.' && !has_dot {
                        has_dot = true;
                        num.push( chars.next().unwrap() );
                    } else {
                        break;
                    }
                }
                
                if let Ok( value ) = num.parse::<f64>() {
                    tokens.push( TOKEN::NUMBER( value ) );
                }
            }
            ' ' | '\t' | '\n' | '\r' => continue,
            '(' => tokens.push( TOKEN::LPAREN ),
            ')' => tokens.push( TOKEN::RPAREN ),
            '+' => tokens.push( TOKEN::PLUS ),
            '-' => tokens.push( TOKEN::MINUS ),
            '*' => tokens.push( TOKEN::MULTIPLY ),
            '/' => tokens.push( TOKEN::DIVIDE ),
            _ => {}
        }
    }
    
    tokens
}
