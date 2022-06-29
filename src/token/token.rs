#[derive(Debug)]
pub enum IntegerKind {
    Binary,
    Octal,
    Decimal,
    HexaDecimal,
}

#[derive(Debug)]
pub enum TokenError {
    Unknown,
}

#[derive(Debug)]
pub enum Token {
    EndOfInput,
    Invalid(TokenError),

    // identifiers + literals
    Identifier,
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
