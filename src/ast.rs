use crate::lexer;

pub enum Program {
    Function((lexer::Token, Vec<lexer::Token>, Vec<Statement>)),
}

#[derive(Debug)]
pub enum Statement {
    Return((lexer::Token, Option<Expression>)),
}

#[derive(Debug)]
pub enum Expression {
    // literal -> NUMBER | STRING ";"
    Literal(lexer::Token),

    // unary -> ( "-" | "!" ) expression ";"
    Unary((Option<lexer::Token>, Box<Expression>)),

    // binary -> expression operator expression ";"
    Binary((Box<Expression>, lexer::Token, Box<Expression>)),

    // grouping -> "(" expression ")" ";"
    Grouping(Box<Expression>),
}

