use std::collections::LinkedList;
use crate::token::Token as Token;

fn declaration(token_list: &LinkedList<Token>){
    if *token_list.front().unwrap() == Token::IDENT {
        if(token_list[1] == Token::EQ){} // TODO: Finish
    }
}

// right now, the only expression is addition or declaration.
fn expression(token_list: LinkedList<Token>){
    if(token_list[0] == Token::NUM_IDENT){
        token_list.pop_front();
        declaration(token_list);
    }
}

// returns true if it's a valid sentence.
fn parse(file: &str) -> bool {
    let mut token_list: LinkedList<Token> = scan(file);
    

    return true;
}