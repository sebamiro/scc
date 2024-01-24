use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::VecDeque;

use crate::token::Token;

struct Lexer {
    source: String,
    current: usize,
}

const DELIMITER: &str = " \t\n\r(){}[];,.:,.+-*/%&|^!~<>=";

impl Lexer {
    fn tokenize(&mut self, tokens: &mut VecDeque<Token>) {
        for c in self.source.chars().skip(self.current) {
            if c.is_whitespace() {
                self.current += 1;
                continue;
            }
            break;
        }
        match self.source.chars().nth(self.current) {
            None => return,
            Some(c) => match c {
                '(' => self.advance(tokens, Some(Token::LeftParen)),
                ')' => self.advance(tokens, Some(Token::RightParen)),
                '{' => self.advance(tokens, Some(Token::LeftBrace)),
                '}' => self.advance(tokens, Some(Token::RightBrace)),
                ',' => self.advance(tokens, Some(Token::Comma)),
                '.' => self.advance(tokens, Some(Token::Dot)),
                '-' => self.advance(tokens, Some(Token::Minus)),
                '+' => self.advance(tokens, Some(Token::Plus)),
                ':' => self.advance(tokens, Some(Token::Colon)),
                ';' => self.advance(tokens, Some(Token::Semicolon)),
                '/' => self.advance(tokens, Some(Token::Slash)),
                '*' => self.advance(tokens, Some(Token::Star)),
                '!' => self.is_equal(tokens, Token::Bang, Token::BangEqual),
                '=' => self.is_equal(tokens, Token::Equal, Token::EqualEqual),
                '>' => self.is_equal(tokens, Token::Greater, Token::GreaterEqual),
                '<' => self.is_equal(tokens, Token::Less, Token::LessEqual),
                _ => self.is_identifier(tokens),
            },
        }
    }

    fn advance(&mut self, tokens: &mut VecDeque<Token>, token: Option<Token>) {
        if let Some(token) = token {
            tokens.push_back(token);
        }
        self.current += 1;
        while let Some(c) = self.source.chars().nth(self.current) {
            if DELIMITER.contains(c) {
                break;
            }
            self.current += 1;
        }
        if self.current < self.source.len() {
            self.tokenize(tokens);
        }
    }

    fn is_equal(&mut self, tokens: &mut VecDeque<Token>, default: Token, equal: Token) {
        match self.source.chars().nth(self.current + 1) {
            Some('=') => {
                self.current += 1;
                self.advance(tokens, Some(equal))
            }
            _ => self.advance(tokens, Some(default)),
        }
    }

    fn is_identifier(&mut self, tokens: &mut VecDeque<Token>) {
        let mut identifier = String::new();
        // TODO: Literals
        while let Some(c) = self.source.chars().nth(self.current) {
            if DELIMITER.contains(c) {
                break;
            }
            identifier.push(c);
            self.current += 1;
        }
        if identifier.is_empty() {
            return;
        }
        tokens.push_back(match identifier.as_str() {
            "break" => Token::Break,
            "case" => Token::Case,
            "char" => Token::Char,
            "const" => Token::Const,
            "constinue" => Token::Constinue,
            "default" => Token::Default,
            "do" => Token::Do,
            "dobule" => Token::Dobule,
            "else" => Token::Else,
            "enum" => Token::Enum,
            "extern" => Token::Extern,
            "float" => Token::Float,
            "for" => Token::For,
            "goto" => Token::Goto,
            "if" => Token::If,
            "int" => Token::Int,
            "long" => Token::Long,
            "register" => Token::Register,
            "return" => Token::Return,
            "short" => Token::Short,
            "signed" => Token::Signed,
            "sizeof" => Token::Sizeof,
            "static" => Token::Static,
            "struct" => Token::Struct,
            "switch" => Token::Switch,
            "typedef" => Token::Typedef,
            "union" => Token::Union,
            "unsigned" => Token::Unsigned,
            "void" => Token::Void,
            "volatile" => Token::Volatile,
            "while" => Token::While,
            "Inline" => Token::Inline,
            "_Bool" => Token::_Bool,
            "_Complex" => Token::_Complex,
            "_Imaginary" => Token::_Imaginary,
            "Restrict" => Token::Restrict,
            x if x.starts_with("\"") => Token::String(identifier),
            x if x.starts_with(|c: char| c.is_digit(10)) => {
                Token::Number(identifier.parse::<i64>().unwrap())
            }
            _ => Token::Identifier(identifier),
        });
        if self.current < self.source.len() {
            self.tokenize(tokens);
        }
    }
}

pub fn lex(filename: &str) -> Result<VecDeque<Token>, io::Error> {
    let mut tokens: VecDeque<Token> = VecDeque::new();
    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        let mut lexer = Lexer {
            source: line,
            current: 0,
        };
        lexer.tokenize(&mut tokens);
    }
    Ok(tokens)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let filename = File::open(filename)?;
    Ok(io::BufReader::new(filename).lines())
}
