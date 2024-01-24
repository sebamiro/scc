use crate::ast;
use crate::lexer;

pub struct Parser {
    pub tokens: Vec<lexer::Token>,
    pub current: usize,
}

pub fn new(filename: &str) -> Result<Parser, String> {
    Ok(Parser {
        tokens: lexer::lex(filename).map_err(|e| e.to_string())?,
        current: 0,
    })
}

impl Parser {
    pub fn parse(&mut self) -> Result<ast::Program, String> {
        Ok(ast::Program::Function(self.parse_function()?))
    }

    fn parse_function(
        &mut self,
    ) -> Result<(lexer::Token, Vec<lexer::Token>, Vec<ast::Statement>), String> {
        println!("Funtion:");
        let func_type = self.consume(lexer::Token::Int, "Expected function type")?;
        println!("type: {:?}", func_type);
        let func_name = self.consume_ident("Expected identifier")?;
        println!("name: {:?}", func_name);
        let _ = self.consume(lexer::Token::LeftParen, "Expected function type")?;
        let _ = self.consume(lexer::Token::RightParen, "Expected function type")?;
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
                if lexer::Token::LeftBrace != *token {
                    return Err("Unexpected end of file".to_string());
                }
                self.advance();
                let mut statements: Vec<ast::Statement> = Vec::new();
                while let Some(token) = self.peek() {
                    if lexer::Token::RightBrace == *token {
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
            Some(token) => {
                match token {
                    lexer::Token::Return => {
                        self.advance();
                        let expr = self.parse_expression()?;
                        self.consume(lexer::Token::Semicolon, "Expected ';' after return")?;
                        return Ok(ast::Statement::Return((lexer::Token::Return, expr)));
                    }
                    _ => return Err("Unexpected end of file".to_string()),
                }
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Option<ast::Expression>, String> {
        match self.peek() {
            None => return Err("Unexpected end of file".to_string()),
            Some(token) => match token {
                lexer::Token::Semicolon => return Ok(None),
                lexer::Token::Number(_) => return Ok(Some(ast::Expression::Literal(self.advance().clone()))),
                _ => return Err("Unexpected end of file".to_string()),
            },
        }
    }

    fn consume(
        &mut self,
        token_type: lexer::Token,
        message: &str,
    ) -> Result<lexer::Token, String> {
        if self.check(token_type) {
            return Ok(self.advance().clone());
        }
        Err(message.to_string())
    }

    fn consume_ident(&mut self, message: &str) -> Result<lexer::Token, String> {
        if let Some(token) = self.peek() {
            if let lexer::Token::Identifier(_) = token {
                return Ok(self.advance().clone());
            }
        }
        Err(message.to_string())
    }

    fn advance(&mut self) -> &lexer::Token {
        self.current += 1;
        &self.tokens[self.current - 1]
    }

    fn check(&self, token_type: lexer::Token) -> bool {
        if let Some(token) = self.peek() {
            return *token == token_type;
        }
        false
    }

    fn peek(&self) -> Option<&lexer::Token> {
        self.tokens.get(self.current)
    }
}
