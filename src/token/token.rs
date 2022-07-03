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

    // delimiters
    Comma,
    Semicolon,

    LeftCurly,
    RightCurly,
    LeftParen,
    RightParen,

    // keywords
    If,
    Else,
    Elif,
    True,
    False,
    Let,
    Return,
    Function,
}

impl Default for Token {
    fn default() -> Self {
        Token::EndOfInput
    }
}
