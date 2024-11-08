use std::fmt;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ill,
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

fn from_string(token: &Token) -> Box<String> {
    let str_repr = match token {
        Token::Ill => String::from("Illegal"),
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
    };
    Box::new(str_repr)
}

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Box<Self> {
        let mut tokens: Vec<_> = vec![];
        let len = input.len();
        let mut index = 0;

        while index < len {
            while index < len && input.as_bytes()[index].is_ascii_whitespace() {
                index += 1;
            }

            if index >= len {
                break;
            }

            // this is important for the keyword and identifier matching
            let start = index;
            let token = match input.chars().nth(index).unwrap() {
                ';' => Token::Semicolon,
                ':' => Token::Colon,
                '(' => Token::LParen,
                ')' => Token::RParen,
                ',' => Token::Comma,
                ']' => Token::RBracket,
                '>' => Token::Gt,
                '<' => Token::Lt,
                '*' => Token::Asterisk,
                '-' => Token::Minus,
                '+' => Token::Plus,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                '[' => Token::LBracket,
                '/' => Token::Slash,
                '!' => {
                    if len - index > 1 {
                        match input.chars().nth(index + 1).unwrap() {
                            '=' => {
                                index += 1;
                                Token::NotEq
                            }
                            _ => Token::Bang,
                        }
                    } else {
                        Token::Bang
                    }
                }
                '=' => {
                    if len - index > 1 {
                        match input.chars().nth(index + 1).unwrap() {
                            '=' => {
                                index += 1;
                                Token::Eq
                            }
                            _ => Token::Assign,
                        }
                    } else {
                        Token::Assign
                    }
                }
                _ => Token::Ill,
            };
            if token != Token::Ill {
                index += 1;
                tokens.push(token);
                continue;
            }

            if input.chars().nth(index).unwrap() == '"' {
                index += 1;
                while index < len && input.chars().nth(index).unwrap() != '"' {
                    index += 1;
                }

                let end = index;
                if index < len {
                    index += 1;
                }
                let str_token = Token::String(input[(start + 1)..end].to_string());
                tokens.push(str_token);
            }

            if index < len && input.as_bytes()[index].is_ascii_alphabetic() {
                while index < len
                    && (input.chars().nth(index).unwrap().is_alphanumeric()
                        || input.chars().nth(index).unwrap() == '_')
                {
                    index += 1;
                }

                if start < index {
                    let s = match &input[start..index] {
                        "let" => Token::Let,
                        "true" => Token::True,
                        "false" => Token::False,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "return" => Token::Ret,
                        "fn" => Token::Func,
                        _ => Token::Identifiere(input[start..index].to_string()),
                    };
                    tokens.push(s);
                }
            }

            if index < len && input.as_bytes()[index].is_ascii_digit() {
                while index < len && (input.as_bytes()[index].is_ascii_digit()) {
                    index += 1;
                }

                if start < index {
                    tokens.push(Token::Integer(
                        input[start..index].to_string().parse::<i64>().unwrap(),
                    ));
                }
            }
        }
        tokens.reverse();

        Box::new(Lexer { tokens })
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
}
