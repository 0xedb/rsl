use crate::token::{literal, Token};

#[derive(Debug, Default)]
pub struct Lexer<'a> {
    ch: char,
    next: usize,
    current: usize,
    input: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut lx = Lexer {
            input,
            ..Default::default()
        };

        lx.read_char();

        lx
    }

    fn read_char(&mut self) {
        self.ch = if self.current >= self.input.len() {
            literal::EndOfInput
        } else {
            self.input.chars().nth(self.current).unwrap()
        };

        self.current = self.next;
        self.next += 1;
    }

    fn eat_whitespace(&mut self) {
        while self.ch == '\t' || self.ch == ' ' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn next_token(&mut self) -> Token {
        use literal::*;

        let mut tok = Token::default();

        self.eat_whitespace();

        match self.ch {
            Plus => tok = Token::Plus,
            Comma => tok = Token::Comma,
            Assign => tok = Token::Assign,
            Semicolon => tok = Token::Semicolon,
            LeftCurly => tok = Token::LeftCurly,
            RightCurly => tok = Token::RightCurly,
            LeftParen => tok = Token::LeftParen,
            RightParen => tok = Token::RightParen,

            EndOfInput => (),
            _ => (),
        }

        self.read_char();

        tok
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token::*;

    use super::Lexer;

    #[test]
    fn test_lexer() -> Result<(), String> {
        let mut lx = Lexer::new(
            r#"
                =+(){},;
            "#,
        );

        let input_tokens = vec![
            Assign, Plus, LeftParen, RightParen, LeftCurly, RightCurly, Comma, Semicolon,
            EndOfInput,
        ];

        let mut index = 0;
        let mut tok = lx.next_token();

        while tok != EndOfInput {
            if tok != input_tokens[index] {
                return Err(format!(
                    "Wanted {:?} but got {:?}",
                    input_tokens[index], tok
                ));
            }

            index += 1;
            tok = lx.next_token();
        }

        Ok(())
    }
}
