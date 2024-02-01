use crate::token::Token;

pub enum Program {
    Function((Token, Token, Block)),
}

impl Program {
    pub fn print(&self) {
        match self {
            Program::Function((func_type, func_name, statements)) => {
                print!("FUN ");
                print!("{:?}", func_type);
                if let Token::Identifier(name) = func_name {
                    println!(" {}:", name);
                }
                println!("\tparams: []");
                println!("\tstatements:");
                // for statement in statements {
                //     print!("\t\t");
                //     statement.print();
                // }
            }
        }
    }
}

pub struct Block {
    pub statement: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    While(Expression, Box<Statement>),
    Return((Token, Option<Expression>)),
}

impl Statement {
    fn print(&self) {
        match self {
            Statement::Return((_, expr)) => {
                print!("RETURN ");
                if let Some(expr) = expr {
                    println!("{:?}", expr);
                } else {
                    println!("");
                }
            },
            Statement::If(expr, then_branch, else_branch) => {
                println!("IF {:?}", expr);
                print!("\tTHEN ");
                then_branch.print();
                if let Some(else_branch) = else_branch {
                    print!("\tELSE ");
                    else_branch.print();
                }
            },
            Statement::While(expr, body) => {
                println!("WHILE {:?}", expr);
                print!("\t");
                body.print();
            },
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    // literal -> NUMBER | STRING ";"
    Literal(Token),

    // unary -> ( "-" | "!" ) expression ";"
    Unary((Option<Token>, Box<Expression>)),

    // binary -> expression operator expression ";"
    Binary((Box<Expression>, Token, Box<Expression>)),

    // grouping -> "(" expression ")" ";"
    Grouping(Box<Expression>),
}

