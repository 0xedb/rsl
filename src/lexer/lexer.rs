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

    fn is_letter(ch: char) -> bool {
        ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z'
    }

    fn read_identifier(&mut self) -> &str {
        let pos = self.current;

        while Lexer::is_letter(self.input.chars().nth(self.current).unwrap()) {
            self.read_char()
        }

        &self.input[pos..self.current]
    }

    fn is_digit(ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn next_token(&mut self) -> Token {
        let mut tok = Token::default();

        self.eat_whitespace();

        match self.ch {
            literal::Plus => tok = Token::Plus,
            literal::Minus => tok = Token::Minus,
            literal::Slash => tok = Token::Slash,
            literal::Star => tok = Token::Star,
            literal::Exclamation => tok = Token::Exclamation,
            literal::Gt => tok = Token::Gt,
            literal::Lt => tok = Token::Lt,
            literal::Comma => tok = Token::Comma,
            literal::Assign => tok = Token::Assign,
            literal::Semicolon => tok = Token::Semicolon,
            literal::LeftCurly => tok = Token::LeftCurly,
            literal::RightCurly => tok = Token::RightCurly,
            literal::LeftParen => tok = Token::LeftParen,
            literal::RightParen => tok = Token::RightParen,
            literal::EndOfInput => (),
            ch => {
                if Lexer::is_letter(ch) {
                    let identifier = self.read_identifier();

                    tok = Token::Identifier(identifier.to_owned());
                } else if Lexer::is_digit(ch) {
                }
            }
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
            r#"let
                =+(){},;
            "#,
        );

        let input_tokens = vec![
            Identifier(String::from("let")),
            Assign,
            Plus,
            LeftParen,
            RightParen,
            LeftCurly,
            RightCurly,
            Comma,
            Semicolon,
            EndOfInput,
        ];

        let mut index = 0;
        let mut tok = lx.next_token();

        while tok != EndOfInput {
            if tok != input_tokens[index] {
                return Err(format!(
                    "Wanted {:?} BUT got {:?}",
                    input_tokens[index], tok
                ));
            }

            index += 1;
            tok = lx.next_token();
        }

        Ok(())
    }
}
