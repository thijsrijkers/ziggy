use crate::token::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            tokens.push(Token::BlankLine);
            continue;
        }

        let context = line.split_whitespace();

        for word in context {
            let mut buffer = String::new();

            for ch in word.chars() {
                if ch.is_ascii_alphanumeric() {
                    buffer.push(ch);
                } else {
                    if !buffer.is_empty() {
                        tokens.push(classify_token(&buffer));
                        buffer.clear();
                    }

                    match ch {
                        '(' => tokens.push(Token::ParenOpen),
                        ')' => tokens.push(Token::ParenClose),
                        '{' => tokens.push(Token::BraceOpen),
                        '}' => tokens.push(Token::BraceClose),
                        '[' => tokens.push(Token::BracketOpen),
                        ']' => tokens.push(Token::BracketClose),
                        '+' | '-' | '*' | '/' | '=' => tokens.push(Token::Operator(ch)),
                        _ => tokens.push(Token::Unknown),
                    }
                }
            }

            if !buffer.is_empty() {
                tokens.push(classify_token(&buffer));
            }
        }
    }

    tokens
}

fn classify_token(token: &str) -> Token {
    if token.chars().all(|c| c.is_ascii_digit()) {
        Token::Number(token.to_string())
    } else if token.chars().all(|c| c.is_ascii_alphabetic()) {
        Token::Identifier(token.to_string())
    } else {
        Token::Unknown
    }
}

