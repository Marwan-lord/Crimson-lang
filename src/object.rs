use std::collections::HashMap;
use std::hash::{Hasher, Hash};
use std::fmt::Display;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ast::BlockStatement;
use crate::enviroment::Enviroment;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Error(String),
    Nil,
    Integer(i64),
    Boolean(bool),
    String(String),
    Identifier(String),
    FunctionInBuilt(String),
    Array(Vec<Object>),
    Dict(HashMap<Object, Object>),
    FunctionLiteral(Vec<String>, BlockStatement, Rc<RefCell<Enviroment>>),
}

impl Eq for Object {}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::String(s) => s.hash(state),
            Object::Integer(i) => i.hash(state),
            _ => panic!("Invalid dict key {}, allowed types are string and integer", self),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Error(e) => write!(f, "{}", e),
            Object::Nil => write!(f, "nil"),
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Identifier(s) => write!(f, "{}", s),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Array(arr) =>  write!(f, "[{}]", arr.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(",")),
            Object::Dict(dict) => {
                let mut str = String::new();
                str.push_str("{");
                for (k, v) in dict {
                    str.push_str(format!("{}:{},", k, v).as_str());
                }

                if str.ends_with(',') {
                    str.pop();
                }
                str.push_str("}");
                write!(f, "{}", str)
            },
            Object::FunctionLiteral(parameters, block, _) => write!(f, "fn({}){{ {} }}",
                                                                    parameters.join(","), block.to_string()),
            _ => panic!("Invalid object {}", self),
        }
    }
}
