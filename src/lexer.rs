use crate::TOKEN;

pub fn lex_expr(expr: &String) -> Vec<TOKEN> {
    let mut vector: Vec<TOKEN> = Vec::new();
    let mut chars = expr.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '0'..='9' => {
                let mut num = ch.to_string();

                while let Some(next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() || *next_ch == '.' {
                        num.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                let num = num.parse::<f64>().unwrap();
                vector.push(TOKEN::NUMBER(num));
            }
            ' ' => continue,
            '(' => vector.push(TOKEN::LPAREN),
            ')' => vector.push(TOKEN::RPAREN),
            '+' => vector.push(TOKEN::PLUS),
            '-' => vector.push(TOKEN::MINUS),
            '*' => vector.push(TOKEN::MULTIPLY),
            '/' => vector.push(TOKEN::DIVIDE),
            _ => ()
        }
    }

    vector
}
