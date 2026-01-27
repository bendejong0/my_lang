use std::fmt;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum Token{
    MAIN,
    IDENT,
    PLUS,
    MINUS,
    MUL,
    DIV,
    MOD,
    R_CURLY,
    L_CURLY,
    L_PAREN,
    R_PAREN,
    IF,
    ELSE,
    FOR,
    SEMICLN,
    DBL_CLN,
    DBL_PLUS,
    DOT,
    DBL_DOT,
    NUM_VALUE,
    NUM_IDENT,
    EQ
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Token::IDENT => "IDENT",
            Token::PLUS => "PLUS",
            Token::MINUS => "MINUS",
            Token::MUL => "MUL",
            Token::DIV => "DIV",
            Token::MOD => "MOD",
            Token::R_CURLY => "R_CURLY",
            Token::L_CURLY => "L_CURLY",
            Token::L_PAREN => "L_PAREN",
            Token::R_PAREN => "R_PAREN",
            Token::IF => "IF",
            Token::ELSE => "ELSE",
            Token::FOR => "FOR",
            Token::SEMICLN => "SEMICLN",
            Token::DBL_CLN => "DBL_CLN",
            Token::DBL_PLUS => "DBL_PLUS",
            Token::DOT => "DOT",
            Token::DBL_DOT => "DBL_DOT",
            Token::NUM_VALUE => "NUM_VALUE",
            Token::NUM_IDENT => "NUM_IDENT",
            Token::EQ => "EQ",
            Token::MAIN => "MAIN",
        };
        write!(f, "{}", s)
    }
}
