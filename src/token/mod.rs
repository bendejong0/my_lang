use std::fmt;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum Token{
    MAIN(String),
    IDENT(String),
    PLUS(String),
    MINUS(String),
    STAR(String),
    SLASH(String),
    MOD(String),
    R_CURLY(String),
    L_CURLY(String),
    L_PAREN(String),
    R_PAREN(String),
    IF(String),
    ELSE(String),
    FOR(String),
    SEMICLN(String),
    DBL_CLN(String),
    DBL_PLUS(String),
    DOT(String),
    DBL_DOT(String),
    NUM_VALUE(i64),
    NUM_IDENT(String),
    EQ(String),
    L_BRACK(String),
    R_BRACK(String),
    COMMA(String)
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Token::IDENT(x) => format!("IDENT {:?}", x),
            Token::PLUS(x) => format!("PLUS {:?}", x),
            Token::MINUS(x) => format!("MINUS {:?}", x),
            Token::STAR(x) => format!("STAR {:?}", x),
            Token::SLASH(x) => format!("SLASH {:?}", x),
            Token::MOD(x) => format!("MOD {:?}", x),
            Token::R_CURLY(x) => format!("R_CURLY {:?}", x),
            Token::L_CURLY(x) => format!("L_CURLY {:?}", x),
            Token::L_PAREN(x) => format!("L_PAREN {:?}", x),
            Token::R_PAREN(x) => format!("R_PAREN {:?}", x),
            Token::IF(x) => format!("IF {:?}", x),
            Token::ELSE(x) => format!("ELSE {:?}", x),
            Token::FOR(x) => format!("FOR {:?}", x),
            Token::SEMICLN(x) => format!("SEMICLN {:?}", x),
            Token::DBL_CLN(x) => format!("DBL_CLN {:?}", x),
            Token::DBL_PLUS(x) => format!("DBL_PLUS {:?}", x),
            Token::DOT(x) => format!("DOT {:?}", x),
            Token::DBL_DOT(x) => format!("DBL_DOT {:?}", x),
            Token::NUM_VALUE(x) => format!("NUM_VALUE {:?}", x),
            Token::NUM_IDENT(x) => format!("NUM_IDENT {:?}", x),
            Token::EQ(x) => format!("EQ {:?}", x),
            Token::MAIN(x) => format!("MAIN {:?}", x),
            Token::L_BRACK(x) => format!("L_BRACK {:?}", x),
            Token::R_BRACK(x) => format!("R_BRACK {:?}", x),
            Token::COMMA(x) => format!("COMMA {:?}", x),
        };
        write!(f, "{}", s)
    }
}
