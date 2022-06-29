use crate::token::{self, literal, Token};

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

    fn next_token(&mut self) {
        use literal::*;

        let mut tok = Token::default();

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
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;

    #[test]
    fn hello() -> Result<(), ()> {
        let lex = Lexer::new("hello world");
        println!("{lex:?}");

        Ok(())
    }
}
