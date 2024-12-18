use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    String(String),
    Bool(bool),
    Prefix(Prefix, Box<Expression>),
    Infix(Infix, Box<Expression>, Box<Expression>),
    If(
        Box<Expression>,
        Box<BlockStatement>,
        Option<Box<BlockStatement>>,
    ),
    FunctionLiteral(Vec<String>, Box<BlockStatement>),
    HashMapLiteral(Vec<(Expression, Expression)>),
    ArrayLiteral(Vec<Expression>),
    Index(Box<Expression>, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(String, Box<Expression>),
    Return(Option<Box<Expression>>),
    Expression(Box<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub stmts: Vec<Statement>,
}

pub struct Program {
    pub stmts: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Prefix {
    Minus,
    Bang,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Infix {
    Eq,
    NotEq,
    Lt,
    Gt,
    Plus,
    Minus,
    Asterisk,
    Slash,
    LBracket,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for stmt in &self.stmts {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for stmt in &self.stmts {
            write!(f, "{}", stmt)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Prefix::Minus => write!(f, "-"),
            Prefix::Bang => write!(f, "!"),
        }
    }
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Infix::Eq => write!(f, "=="),
            Infix::NotEq => write!(f, "!="),
            Infix::Lt => write!(f, "<"),
            Infix::Gt => write!(f, ">"),
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Asterisk => write!(f, "*"),
            Infix::Slash => write!(f, "/"),
            Infix::LBracket => write!(f, "["),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(s, exp) => write!(f, "let {} = {};", s, exp),
            Statement::Return(None) => write!(f, "return;"),
            Statement::Return(Some(val)) => write!(f, "return {};", val),
            Statement::Expression(exp) => write!(f, "{};", exp),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(s) => write!(f, "{}", s),
            Expression::IntegerLiteral(i) => write!(f, "{}", i),
            Expression::String(s) => write!(f, "\"{}\"", s),
            Expression::Bool(b) => write!(f, "{}", b),
            Expression::Prefix(p, exp) => write!(f, "({}, {})", p, exp),
            Expression::Infix(op, left, right) => write!(f, "({} {} {})", op, left, right),
            Expression::If(exp, true_blk, Some(false_blk)) => {
                write!(f, "if ({}) {} else {}", exp, true_blk, false_blk)
            }
            Expression::If(exp, true_blk, None) => write!(f, "if ({}) {}", exp, true_blk),
            Expression::HashMapLiteral(key_values) => {
                let mut str = String::new();
                str.push('{');
                for (k, v) in key_values {
                    str.push_str(format!("{}:{},", k, v).as_str());
                }

                if str.ends_with(',') {
                    str.pop();
                }
                str.push('}');
                write!(f, "{}", str)
            }
            Expression::ArrayLiteral(members) => write!(
                f,
                "[{}]",
                members
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Expression::Index(arr, idx) => write!(f, "{}[{}]", arr, idx),
            Expression::FunctionLiteral(params, block) => {
                write!(f, "fn({}){}", params.join(","), block)
            }
            Expression::Call(exp, params) => write!(
                f,
                "{}({})",
                exp,
                params
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}
