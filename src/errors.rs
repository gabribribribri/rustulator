#[derive(Debug)]
pub enum Error {
    Syntax(SyntaxError),
}

// #[derive(Debug)]
// pub enum

#[derive(Debug)]
pub enum SyntaxError {
    UnknownSymbol,
    UnallowedStranger,
    
    EntangledOperators,

    MissingParentheses,
    ParenthesesPlacement,
    VoidParentheses,
}