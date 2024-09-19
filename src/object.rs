use crate::ast::BlockStatement;
use crate::enviroment::EnviromentVariables;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Err(String),
    Null,
    Integer(i64),
    Bool(bool),
    String(String),
    Identifier(String),
    BuiltInFunction(String),
    Array(Vec<Object>),
    HashMap(HashMap<Object, Object>),
    FunctionLiteral(Vec<String>, BlockStatement, Rc<RefCell<EnviromentVariables>>),
}

impl Eq for Object {}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::String(s) => s.hash(state),
            Object::Integer(i) => i.hash(state),
            _ => panic!(
                "Invalid Hash key {}, only string and integer are allowed",
                self
            ),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Err(e) => write!(f, "{}", e),
            Object::Null => write!(f, "NULL"),
            Object::Integer(i) => write!(f, "{}", i),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Identifier(s) => write!(f, "{}", s),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Array(arr) => write!(
                f,
                "[{}]",
                arr.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Object::HashMap(dict) => {
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
            }
            Object::FunctionLiteral(parameters, block, _) => {
                write!(f, "fn({}){{ {} }}", parameters.join(","), block.to_string())
            }
            _ => panic!("Invalid object"),
        }
    }
}
