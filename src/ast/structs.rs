/**
 * ## AST Data Structures
 * 
 * This file contains the data structures for the language's
 * Abstract Syntax Tree.
 * 
**/


pub enum Mathematical {
    Literal(f64),
    Add(Box<Mathematical>, Box<Mathematical>),
    Sub(Box<Mathematical>, Box<Mathematical>),
    Mul(Box<Mathematical>, Box<Mathematical>),
    Div(Box<Mathematical>, Box<Mathematical>),
}

pub enum Logical {
    Literal(i32),
    Eq(Box<Mathematical>, Box<Mathematical>),
    Ne(Box<Mathematical>, Box<Mathematical>),
    Gt(Box<Mathematical>, Box<Mathematical>),
    Ls(Box<Mathematical>, Box<Mathematical>),
    Ge(Box<Mathematical>, Box<Mathematical>),
    Le(Box<Mathematical>, Box<Mathematical>),
    And(Box<Logical>, Box<Logical>),
    Or(Box<Logical>, Box<Logical>),
}

pub enum Statement {
    Let,
    Goto(GotoStatement),
    If(IfStatement),
    For(ForStatement),
    While(WhileStatement),
}

pub struct IfStatement {
    Condition: Logical,
    Body: Program,
    Line: u16
}

pub struct GotoStatement {
    Destination: u16,
    Body: Program,
    Line: u16
}

pub struct ForStatement {
    RangeLeft: Mathematical,
    RangeRight: Mathematical,
    Body: Program,
    Line: u16
}

pub struct WhileStatement {
    Condition: Logical,
    Body: Program,
    Line: u16
}

pub struct Program {
    Body: Vec<Statement>,
    Variables: Vec<f64>
}