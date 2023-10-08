#[derive(Debug)]
pub enum Error {
    Syntax(SyntaxError)
}

#[derive(Debug)]
pub enum SyntaxError {
    UnknownSymbol,
    EntangledOperators,

    MissingParentheses,
    ParenthesesPlacement,
    VoidParentheses,
}