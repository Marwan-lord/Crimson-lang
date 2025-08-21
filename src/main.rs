mod ast;
mod enviroment;
mod evaluator;
mod inbuilt;
mod lexer;
pub mod object;
mod parser;

use crate::enviroment::EnviromentVariables;
use crate::evaluator::eval_program;
use crate::lexer::Lexer;
use linefeed::{Interface, ReadResult};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let reader = Interface::new("Crimson lang").unwrap();
    let mut env = Rc::new(RefCell::new(EnviromentVariables::new()));

    println!("Crimson Lang. \n");
    reader.set_prompt("> ").unwrap();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq("exit") {
            break;
        }
        let lexer = Lexer::new(&input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        println!("{}", eval_program(program.as_ref(), &mut env));
    }
    println!("Bye !");
}
