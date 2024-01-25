use crate::token::Token;

pub enum Program {
    Function((Token, Token, Vec<Statement>)),
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
                for statement in statements {
                    print!("\t\t");
                    statement.print();
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Statement {
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
            }
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

