mod lexer;
mod ast;
mod parser;
pub mod object;
mod enviroment;
mod evaluator;
mod inbuilt;

use linefeed::{Interface, ReadResult};
use crate::lexer::Lexer;
use crate::evaluator::eval_program;
use std::rc::Rc;
use std::cell::RefCell;
use crate::enviroment::EnviromentVariables;

fn main() {

    let reader = Interface::new("Crimson lang").unwrap();
    let mut env = Rc::new(RefCell::new(EnviromentVariables::new()));

    println!("Hello.");
    reader.set_prompt("> ").unwrap();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq("exit") {
            break;
        }
        let lexer = Lexer::new(&*input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        println!("{}", eval_program(program.as_ref(), &mut env));
    }
    println!("Exit.");
}
