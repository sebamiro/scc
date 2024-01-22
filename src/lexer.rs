use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub enum Token {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Colon,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    Number(i64),
    String(String),

    // Keywords
    Break,
    Case,
    Char,
    Const,
    Constinue,
    Default,
    Do,
    Dobule,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Int,
    Long,
    Register,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,

    Inline,
    _Bool,
    _Complex,
    _Imaginary,

    Restrict,
}

struct Lexer {
    source: String,
    current: usize,
}

const DELIMITER: &str = " \t\n\r(){}[];,.:,.+-*/%&|^!~<>=";

impl Lexer {
    fn tokenize(&mut self, tokens: &mut Vec<Token>) {
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

    fn advance(&mut self, tokens: &mut Vec<Token>, token: Option<Token>) {
        if let Some(token) = token {

            tokens.push(token);
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

    fn is_equal(&mut self, tokens: &mut Vec<Token>, default: Token, equal: Token) {
        match self.source.chars().nth(self.current + 1) {
            Some('=') => {
                self.current += 1;
                self.advance(tokens, Some(equal))
            }
            _ => self.advance(tokens, Some(default)),
        }
    }

    fn is_identifier(&mut self, tokens: &mut Vec<Token>) {
        let mut identifier = String::new();
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
        match identifier.as_str() {
            "break" => tokens.push(Token::Break),
            "case" => tokens.push(Token::Case),
            "char" => tokens.push(Token::Char),
            "const" => tokens.push(Token::Const),
            "constinue" => tokens.push(Token::Constinue),
            "default" => tokens.push(Token::Default),
            "do" => tokens.push(Token::Do),
            "dobule" => tokens.push(Token::Dobule),
            "else" => tokens.push(Token::Else),
            "enum" => tokens.push(Token::Enum),
            "extern" => tokens.push(Token::Extern),
            "float" => tokens.push(Token::Float),
            "for" => tokens.push(Token::For),
            "goto" => tokens.push(Token::Goto),
            "if" => tokens.push(Token::If),
            "int" => tokens.push(Token::Int),
            "long" => tokens.push(Token::Long),
            "register" => tokens.push(Token::Register),
            "return" => tokens.push(Token::Return),
            "short" => tokens.push(Token::Short),
            "signed" => tokens.push(Token::Signed),
            "sizeof" => tokens.push(Token::Sizeof),
            "static" => tokens.push(Token::Static),
            "struct" => tokens.push(Token::Struct),
            "switch" => tokens.push(Token::Switch),
            "typedef" => tokens.push(Token::Typedef),
            "union" => tokens.push(Token::Union),
            "unsigned" => tokens.push(Token::Unsigned),
            "void" => tokens.push(Token::Void),
            "volatile" => tokens.push(Token::Volatile),
            "while" => tokens.push(Token::While),
            "Inline" => tokens.push(Token::Inline),
            "_Bool" => tokens.push(Token::_Bool),
            "_Complex" => tokens.push(Token::_Complex),
            "_Imaginary" => tokens.push(Token::_Imaginary),
            "Restrict" => tokens.push(Token::Restrict),
            x if x.starts_with("\"") => tokens.push(Token::String(identifier)),
            x if x.starts_with(|c: char| c.is_digit(10)) => {
                tokens.push(Token::Number(identifier.parse::<i64>().unwrap()))
            }
            _ => tokens.push(Token::Identifier(identifier)),
        }
        if self.current < self.source.len() {
            self.tokenize(tokens);
        }
    }
}

pub fn lex(filename: &str) -> Result<Vec<Token>, io::Error> {
    let mut tokens: Vec<Token> = Vec::new();
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
