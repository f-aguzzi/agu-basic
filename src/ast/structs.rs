/**
 * ## AST Data Structures
 * 
 * This file contains the data structures for the language's
 * Abstract Syntax Tree.
 * 
**/

#[derive(Debug)]
pub enum Mathematical {
    Literal(f64),
    Add(Box<Mathematical>, Box<Mathematical>),
    Sub(Box<Mathematical>, Box<Mathematical>),
    Mul(Box<Mathematical>, Box<Mathematical>),
    Div(Box<Mathematical>, Box<Mathematical>),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Statement {
    Let(Variable),
    Goto(GotoStatement),
    If(IfStatement),
    For(ForStatement),
    While(WhileStatement),
}

#[derive(Debug)]
pub struct IfStatement {
    condition: Logical,
    body: Program,
    line: u16
}

#[derive(Debug)]
pub struct GotoStatement {
    destination: u16,
    body: Program,
    line: u16
}

#[derive(Debug)]
pub struct ForStatement {
    range_left: Mathematical,
    range_right: Mathematical,
    body: Program,
    line: u16
}

#[derive(Debug)]
pub struct WhileStatement {
    condition: Logical,
    body: Program,
    line: u16
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub value: f64
}

#[derive(Debug)]
pub struct Program {
    pub body: Vec<Statement>,
    pub variables: Vec<Variable>
}