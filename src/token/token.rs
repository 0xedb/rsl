use crate::token::literal;

#[derive(Debug, PartialEq, Eq)]
pub enum IntegerKind {
    Binary,
    Octal,
    Decimal,
    HexaDecimal,
}

impl Default for IntegerKind {
    fn default() -> Self {
        Self::Decimal
    }
}

impl IntegerKind {
    pub fn get_kind(s: &str) -> Self {
        if s == "0" {
            Self::Decimal
        } else {
            match &s[..2] {
                "0b" | "0B" => Self::Binary,
                o if s.chars().all(|n| n >= '0' && n < '8') => Self::Octal,
                "0x" | "0X" => Self::HexaDecimal,
                _ => panic!("INVALID DIGIT"),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenError {
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    EndOfInput,
    Invalid(TokenError),

    // identifiers + literals
    Identifier(String),
    Integer(IntegerKind),

    // operators
    Assign,
    Plus,
    Minus,
    Slash,
    Star,
    Exclamation,

    Gt,
    Lt,

    Equal,
    NotEqual,

    // delimiters
    Comma,
    Semicolon,

    LeftCurly,
    RightCurly,
    LeftParen,
    RightParen,

    // keywords
    Kw_Start, // just a marker not a keyword
    If,
    Else,
    Elif,
    True,
    False,
    Let,
    Return,
    Function,
    Kw_End, // just a marker not a keyword
}

impl Default for Token {
    fn default() -> Self {
        Token::Invalid(TokenError::Unknown)
    }
}

impl Token {
    fn is_keyword(s: &str) -> bool {
        use literal::*;

        if let Some(_) = [If, Else, Elif, True, False, Let, Function, Return]
            .iter()
            .position(|&tok| tok == s)
        {
            true
        } else {
            false
        }
    }

    pub fn get_keyword(s: String) -> Token {
        use literal::*;
        if Token::is_keyword(&s) {
            match s.as_str() {
                If => Token::If,
                Else => Token::Else,
                Elif => Token::Elif,
                True => Token::True,
                False => Token::False,
                Let => Token::Let,
                Function => Token::Function,
                Return => Token::Return,
                _ => panic!("NOT A KEYWORD"),
            }
        } else {
            Token::Identifier(s)
        }
    }
}
