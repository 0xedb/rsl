use crate::token::{literal, IntegerKind, Token};

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
        let mut cur = self.input.chars().nth(self.current).unwrap();

        while cur == '\t' || cur == ' ' || cur == '\n' || cur == '\r' {
            self.read_char();
            cur = self.input.chars().nth(self.current).unwrap();
        }

        self.ch = self.input.chars().nth(self.current).unwrap()
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
        ch >= '0' && ch <= '9' || Self::is_letter(ch)
    }

    fn read_digit(&mut self) -> &str {
        let pos: usize = self.current;

        while Lexer::is_digit(self.input.chars().nth(self.current).unwrap()) {
            self.read_char()
        }

        &self.input[pos..self.current]
    }

    fn peek_next_char(&self) -> char {
        if self.next >= self.input.len() {
            literal::Invalid
        } else {
            self.input.chars().nth(self.next).unwrap()
        }
    }

    fn next_token(&mut self) -> Token {
        let mut tok = Token::default();

        self.eat_whitespace();
        match self.ch {
            literal::Gt => tok = Token::Gt,
            literal::Lt => tok = Token::Lt,
            literal::Star => tok = Token::Star,
            literal::Plus => tok = Token::Plus,
            literal::Minus => tok = Token::Minus,
            literal::Slash => tok = Token::Slash,
            literal::Comma => tok = Token::Comma,
            literal::LeftCurly => tok = Token::LeftCurly,
            literal::Semicolon => tok = Token::Semicolon,
            literal::LeftParen => tok = Token::LeftParen,
            literal::RightCurly => tok = Token::RightCurly,
            literal::RightParen => tok = Token::RightParen,
            literal::EndOfInput => tok = Token::EndOfInput,
            literal::Assign => {
                tok = if self.peek_next_char() == literal::Assign {
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            literal::Exclamation => {
                tok = if self.peek_next_char() == literal::Assign {
                    Token::NotEqual
                } else {
                    Token::Exclamation
                }
            }
            ch => {
                if Lexer::is_letter(ch) {
                    tok = Token::get_keyword(self.read_identifier().to_owned());
                } else if Lexer::is_digit(ch) {
                    tok = Token::Integer(IntegerKind::get_kind(self.read_digit()));
                }
            }
        }

        self.read_char();

        tok
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{IntegerKind::*, Token::*};

    use super::Lexer;

    #[test]
    fn test_lexer() -> Result<(), String> {
        let mut lx = Lexer::new(
            r#"
                    let five = 5;
                    let ten = 10;
                    let add = fn(x, y) {
                    x + y;
                    };
                    let result = add(five, ten);
                    !-/*5;
                    5 < 10 > 5;
                    if (5 < 10) {
                    return true;
                    } else {
                    return false;
                    }
                    10 == 10;
                    10 != 9;
            "#,
        );

        let input_tokens = vec![
            Let,
            Identifier(String::from("five")),
            Assign,
            Integer(Decimal),
            Semicolon,
            Let,
            Identifier(String::from("ten")),
            Equal,
            Integer(Decimal),
        ];

        let mut index = 0;
        let mut tok = lx.next_token();

        let mut msg: String;

        while tok != EndOfInput {
            msg = format!(
                "TEST #{index}: Wanted {:?} Got {:?}",
                input_tokens[index], tok
            );

            if tok != input_tokens[index] {
                return Err(msg);
            }

            println!("{msg}");
            index += 1;
            tok = lx.next_token();
        }

        Ok(())
    }
}
