use std::fmt;
use std::fmt::Debug;
use std::iter::from_fn;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Eof,
    Identifiere(String),
    String(String),
    Integer(i64),
    Assign,
    Plus,
    Bang,
    Minus,
    Slash,
    Asterisk,
    Lt,
    Gt,
    Eq,
    NotEq,
    Comma,
    Colon,
    Let,
    True,
    False,
    If,
    Else,
    Ret,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Func,
}

fn from_string(token: &Token) -> String {
    match token {
        Token::Eof => String::from("Eof"),
        Token::Identifiere(s) => s.clone(),
        Token::Integer(i) => i.to_string(),
        Token::String(s) => s.clone(),
        Token::Assign => String::from("="),
        Token::Plus => String::from("+"),
        Token::Gt => String::from(">"),
        Token::Eq => String::from("=="),
        Token::NotEq => String::from("!="),
        Token::Comma => String::from(","),
        Token::Semicolon => String::from(";"),
        Token::Bang => String::from("!"),
        Token::Slash => String::from("/"),
        Token::Asterisk => String::from("*"),
        Token::Lt => String::from("<"),
        Token::Minus => String::from("-"),
        Token::Colon => String::from(':'),
        Token::LParen => String::from("("),
        Token::RParen => String::from(")"),
        Token::Func => String::from("fn"),
        Token::Let => String::from("let"),
        Token::True => String::from("true"),
        Token::False => String::from("false"),
        Token::If => String::from("if"),
        Token::Else => String::from("else"),
        Token::Ret => String::from("return"),
        Token::LBrace => String::from("{"),
        Token::RBrace => String::from("}"),
        Token::LBracket => String::from("["),
        Token::RBracket => String::from("]"),
    }
}
struct Tokenizer<'a> {
    chars: Chars<'a>,
    current: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current = chars.next();
        Self { chars, current }
    }

    fn consume(&mut self, t: Token) -> Option<Token> {
        self.advance_char();
        Some(t)
    }

    fn match_compound_token(
        &mut self,
        expected: char,
        compound: Token,
        single: Token,
    ) -> Option<Token> {
        match self.advance_char()? {
            c if c == expected => self.consume(compound),
            _ => Some(single),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        match self.current? {
            '(' => self.consume(Token::LParen),
            ')' => self.consume(Token::RParen),
            '[' => self.consume(Token::LBracket),
            ']' => self.consume(Token::RBracket),
            '{' => self.consume(Token::LBrace),
            '}' => self.consume(Token::RBrace),
            '+' => self.consume(Token::Plus),
            '-' => self.consume(Token::Minus),
            '/' => self.consume(Token::Slash),
            '*' => self.consume(Token::Asterisk),
            '<' => self.consume(Token::Lt),
            '>' => self.consume(Token::Gt),
            ',' => self.consume(Token::Comma),
            ':' => self.consume(Token::Colon),
            ';' => self.consume(Token::Semicolon),

            '=' => self.match_compound_token('=', Token::Eq, Token::Assign),
            '!' => self.match_compound_token('=', Token::NotEq, Token::Bang),
            '"' => Some(Token::String(self.is_string())),

            a if a.is_alphabetic() => {
                let result = self.is_keyword();
                match result.as_str() {
                    "if" => Some(Token::If),
                    "else" => Some(Token::Else),
                    "fn" => Some(Token::Func),
                    "let" => Some(Token::Let),
                    "true" => Some(Token::True),
                    "false" => Some(Token::False),
                    "return" => Some(Token::Ret),
                    _ => Some(Token::Identifiere(result)),
                }
            }

            n if n.is_numeric() => {
                let result = self.is_number();
                Some(Token::Integer(
                    result.parse::<i64>().expect("Failed to parse int"),
                ))
            }

            _ => None,
        }
    }

    fn is_number(&mut self) -> String {
        from_fn(|| match self.current {
            Some(c) if c.is_ascii_digit() => {
                self.advance_char();
                Some(c)
            }
            _ => None,
        })
        .collect()
    }

    fn is_keyword(&mut self) -> String {
        from_fn(|| match self.current {
            Some(c) if c.is_alphanumeric() || c == '_' => {
                let ch = c;
                self.advance_char();
                Some(ch)
            }
            _ => None,
        })
        .collect()
    }

    fn is_string(&mut self) -> String {
        self.advance_char();

        from_fn(|| match self.current {
            Some('"') => {
                self.advance_char();
                None
            }
            Some(c) => {
                self.advance_char();
                Some(c)
            }
            None => None,
        })
        .collect()
    }

    fn advance_char(&mut self) -> Option<char> {
        self.current = self.chars.next();
        self.current
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current {
            if !c.is_whitespace() {
                break;
            }

            self.advance_char();
        }
    }
}

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Box<Self> {
        let mut tokenizer = Tokenizer::new(input);
        let mut tokens = vec![];

        while let Some(t) = tokenizer.next_token() {
            tokens.push(t);
        }

        tokens.reverse();
        Box::new(Self { tokens })
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", from_string(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token};

    const TEST_STR: &str = "
    let name = \"maro\";
    let age = 16;

    let add = fn(x, y) {
      x + y;
    };

    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;
    
    if (5 < 10) {
      return true;
    } else {
      return false;
    }

    10 == 10;
    10 != 9;

    let x = \"abcd\";
    let y = \"\";
    let arr = arr[1, 2, 3];
    let y = arr[x];
    let x = {z: \"hello\", y: 1};
    ";

    #[test]
    fn test_tokens() {
        let test_token_vec: Vec<Token> = vec![
            Token::Let,
            Token::Identifiere(String::from("name")),
            Token::Assign,
            Token::String(String::from("maro")),
            Token::Semicolon,
            Token::Let,
            Token::Identifiere(String::from("age")),
            Token::Assign,
            Token::Integer(16),
            Token::Semicolon,
            Token::Let,
            Token::Identifiere(String::from("add")),
            Token::Assign,
            Token::Func,
            Token::LParen,
            Token::Identifiere(String::from("x")),
            Token::Comma,
            Token::Identifiere(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Identifiere(String::from("x")),
            Token::Plus,
            Token::Identifiere(String::from("y")),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifiere(String::from("result")),
            Token::Assign,
            Token::Identifiere(String::from("add")),
            Token::LParen,
            Token::Identifiere(String::from("five")),
            Token::Comma,
            Token::Identifiere(String::from("ten")),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Integer(5),
            Token::Semicolon,
            Token::Integer(5),
            Token::Lt,
            Token::Integer(10),
            Token::Gt,
            Token::Integer(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Integer(5),
            Token::Lt,
            Token::Integer(10),
            Token::RParen,
            Token::LBrace,
            Token::Ret,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Ret,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Integer(10),
            Token::Eq,
            Token::Integer(10),
            Token::Semicolon,
            Token::Integer(10),
            Token::NotEq,
            Token::Integer(9),
            Token::Semicolon,
            Token::Let,
            Token::Identifiere(String::from("x")),
            Token::Assign,
            Token::String(String::from("abcd")),
            Token::Semicolon,
            Token::Let,
            Token::Identifiere(String::from("y")),
            Token::Assign,
            Token::String(String::from("")),
            Token::Semicolon,
            Token::Let,
            Token::Identifiere(String::from("arr")),
            Token::Assign,
            Token::Identifiere(String::from("arr")),
            Token::LBracket,
            Token::Integer(1),
            Token::Comma,
            Token::Integer(2),
            Token::Comma,
            Token::Integer(3),
            Token::RBracket,
            Token::Semicolon,
            Token::Let,
            Token::Identifiere(String::from("y")),
            Token::Assign,
            Token::Identifiere(String::from("arr")),
            Token::LBracket,
            Token::Identifiere(String::from("x")),
            Token::RBracket,
            Token::Semicolon,
            Token::Let,
            Token::Identifiere(String::from("x")),
            Token::Assign,
            Token::LBrace,
            Token::Identifiere(String::from("z")),
            Token::Colon,
            Token::String(String::from("hello")),
            Token::Comma,
            Token::Identifiere(String::from("y")),
            Token::Colon,
            Token::Integer(1),
            Token::RBrace,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(TEST_STR);

        for test_token in test_token_vec.iter() {
            let token = lexer.next();
            assert_eq!(token, *test_token);
        }
    }

    #[test]
    fn simple() {
        let input = "let name = \"marwan\";";
        let mut lexer = Lexer::new(input);

        let test_tokens = vec![
            Token::Let,
            Token::Identifiere(String::from("name")),
            Token::Assign,
            Token::String(String::from("marwan")),
            Token::Semicolon,
        ];

        for test_token in test_tokens.iter() {
            let token = lexer.next();
            assert_eq!(token, *test_token);
        }
    }
}
