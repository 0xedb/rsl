#[derive(Debug, PartialEq, Eq)]
pub enum IntegerKind {
    Binary,
    Octal,
    Decimal,
    HexaDecimal,
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

    // delimiters
    Comma,
    Semicolon,

    LeftCurly,
    RightCurly,
    LeftParen,
    RightParen,

    // keywords
    Let,
    Function,
}

impl Default for Token {
    fn default() -> Self {
        Token::EndOfInput
    }
}
