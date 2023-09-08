mod lib;
mod ast;
mod compiler;
mod parser;

use ast::structs::*;
use compiler::compile::*;
use parser::parser::*;

fn main() {

    /*
    let a = Mathematical::Literal(1.0);
    let b = Mathematical::Literal(3.0);
    let c = Mathematical::Literal(2.0);

    let sum_1 = Mathematical::Add(Box::new(a), Box::new(b));
    let sum_2 = Mathematical::Add(Box::new(sum_1), Box::new(c));

    let string = math2text(Box::new(sum_2));
    let wasm = compile(string.to_owned());

    println!("{}", transpile(string));
    // println!("{}", String::from_utf8(wasm).unwrap());
    */

    let program = parser();

    for s in program.body {
        println!("{:?}", s);
    }

}
