extern crate wabt;
use wabt::wat2wasm;

use crate::ast::structs::*;
use crate::compiler::math::*;
use crate::compiler::logic::*;

pub fn math2text(math: Box<Mathematical>) -> String {
    let text = match *math {
        Mathematical::Literal(a) => { "f64.const ".to_owned() + &a.to_string() }
        Mathematical::Add(a, b) => { sum(&math2text(a), &math2text(b)) }
        Mathematical::Sub(a, b) => { sub(&math2text(a), &math2text(b)) }
        Mathematical::Mul(a, b) => { mul(&math2text(a), &math2text(b)) }
        Mathematical::Div(a, b) => { div(&math2text(a), &math2text(b)) }
        _=> { String::from("SISTEMARE") }
    };

    text
}

pub fn log2text(log: Box<Logical>) -> String {
    let text = match *log {
        Logical::Literal(a) => { "f64.const ".to_owned() + &a.to_string() }
        Logical::Eq(a, b) => { eq(&math2text(a), &math2text(b)) }
        Logical::Ne(a, b) => { ne(&math2text(a), &math2text(b)) }
        Logical::Gt(a, b) => { gt(&math2text(a), &math2text(b)) }
        Logical::Ls(a, b) => { ls(&math2text(a), &math2text(b)) }
        Logical::Ge(a, b) => { ge(&math2text(a), &math2text(b)) }
        Logical::Le(a, b) => { le(&math2text(a), &math2text(b)) }
        Logical::And(a, b) => { and(&log2text(a), &log2text(b)) }
        Logical::Or(a, b) => { or(&log2text(a), &log2text(b)) }
        _=> { String::from("SISTEMARE") }
    };

    text
}

pub fn compile(a: String) -> Vec<u8> {
    let program =  "(module\n".to_owned() + "(func $exec (result f64)\n" + &a + ")\n" + "(export \"exec\" (func $exec))\n)";
    let text = program.as_bytes().to_vec();
    wat2wasm(text).unwrap()
}

pub fn transpile(a: String) -> String {
    "(module\n".to_owned() + "(func $exec (result f64)\n" + &a + ")\n" + "(export \"exec\" (func $exec))\n)"
}