use crate::token;

pub enum Program {
    Function((token::Token, Vec<token::Token>, Vec<Statement>)),
}

#[derive(Debug)]
pub enum Statement {
    Return((token::Token, Option<Expression>)),
}

#[derive(Debug)]
pub enum Expression {
    // literal -> NUMBER | STRING ";"
    Literal(token::Token),

    // unary -> ( "-" | "!" ) expression ";"
    Unary((Option<token::Token>, Box<Expression>)),

    // binary -> expression operator expression ";"
    Binary((Box<Expression>, token::Token, Box<Expression>)),

    // grouping -> "(" expression ")" ";"
    Grouping(Box<Expression>),
}

