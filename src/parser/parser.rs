use crate::ast::structs::{Program, IfStatement, Logical};
use super::lexer::{lexer, Token};

pub fn parse() {
    let mut code: Vec<Vec<Token>> = lexer("IF".to_string());

    let mut program: Program;

}