use std::collections::VecDeque;

use crate::ast;
use crate::lexer;
use crate::token::Token;

pub struct Parser {
    pub tokens: VecDeque<Token>,
}

pub fn new(filename: &str) -> Result<Parser, String> {
    Ok(Parser {
        tokens: lexer::lex(filename).map_err(|e| e.to_string())?,
    })
}

impl Parser {
    pub fn parse(&mut self) -> Result<ast::Program, String> {
        Ok(ast::Program::Function(self.parse_function()?))
    }

    fn parse_function(
        &mut self,
    ) -> Result<(Token, Vec<Token>, Vec<ast::Statement>), String> {
        println!("Funtion:");
        let func_type = self.consume(Token::Int, "Expected function type")?;
        println!("type: {:?}", func_type);
        let func_name = self.consume_ident("Expected identifier")?;
        println!("name: {:?}", func_name);
        let _ = self.consume(Token::LeftParen, "Expected function type")?;
        let _ = self.consume(Token::RightParen, "Expected function type")?;
        // TODO: parse parameters
        println!("params: []");
        let statements = self.parse_block()?;
        println!("statements: {:?}", statements);
        Ok((func_type, vec![func_name], statements))
    }

    fn parse_block(&mut self) -> Result<Vec<ast::Statement>, String> {
        match self.peek() {
            None => return Err("Unexpected end of file".to_string()),
            Some(token) => {
                if Token::LeftBrace != *token {
                    return Err("Unexpected end of file".to_string());
                }
                self.advance();
                let mut statements: Vec<ast::Statement> = Vec::new();
                while let Some(token) = self.peek() {
                    if Token::RightBrace == *token {
                        self.advance();
                        return Ok(statements);
                    }
                    statements.push(self.parse_statement()?);
                }
                return Err("Unexpected end of file".to_string());
            }
        }
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, String> {
        match self.peek() {
            None => return Err("Unexpected end of file".to_string()),
            Some(token) => match token {
                Token::Return => {
                    self.advance();
                    let expr = self.parse_expression()?;
                    self.consume(Token::Semicolon, "Expected ';' after return")?;
                    return Ok(ast::Statement::Return((Token::Return, expr)));
                }
                _ => return Err("Unexpected end of file".to_string()),
            },
        }
    }

    fn parse_expression(&mut self) -> Result<Option<ast::Expression>, String> {
        match self.peek() {
            None => return Err("Unexpected end of file".to_string()),
            Some(token) => match token {
                Token::Semicolon => return Ok(None),
                Token::Number(_) => {
                    return Ok(Some(ast::Expression::Literal(self.advance())))
                }
                _ => return Err("Unexpected end of file".to_string()),
            },
        }
    }

    fn consume(&mut self, token_type: Token, message: &str) -> Result<Token, String> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(message.to_string())
    }

    fn consume_ident(&mut self, message: &str) -> Result<Token, String> {
        if let Some(token) = self.peek() {
            if let Token::Identifier(_) = token {
                return Ok(self.advance());
            }
        }
        Err(message.to_string())
    }

    fn advance(&mut self) -> Token {
        self.tokens.pop_front().unwrap()
    }

    fn check(&self, token_type: Token) -> bool {
        if let Some(token) = self.peek() {
            return *token == token_type;
        }
        false
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(0)
    }
}
