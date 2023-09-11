/**
 * ## AST Data Structures
 * 
 * This file contains the data structures for the language's
 * Abstract Syntax Tree.
 * 
**/

#[derive(Debug, Clone, PartialEq)]
pub enum Mathematical {
    Literal(f64),
    Var(String),
    Add(Box<Mathematical>, Box<Mathematical>),
    Sub(Box<Mathematical>, Box<Mathematical>),
    Mul(Box<Mathematical>, Box<Mathematical>),
    Div(Box<Mathematical>, Box<Mathematical>),
}

impl Mathematical {
    pub fn from_add(expr1: Mathematical, expr2: Mathematical) -> Mathematical {
        Mathematical::Add(
            Box::new(expr1),
            Box::new(expr2)
        )
    }

    pub fn from_sub(expr1: Mathematical, expr2: Mathematical) -> Mathematical {
        Mathematical::Sub(
            Box::new(expr1),
            Box::new(expr2)
        )
    }

    pub fn from_mul(expr1: Mathematical, expr2: Mathematical) -> Mathematical {
        Mathematical::Mul(
            Box::new(expr1),
            Box::new(expr2)
        )
    }

    pub fn from_div(expr1: Mathematical, expr2: Mathematical) -> Mathematical {
        Mathematical::Div(
            Box::new(expr1),
            Box::new(expr2)
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Logical {
    Literal(i32),
    Var(String),
    Eq(Box<Mathematical>, Box<Mathematical>),
    Ne(Box<Mathematical>, Box<Mathematical>),
    Gt(Box<Mathematical>, Box<Mathematical>),
    Ls(Box<Mathematical>, Box<Mathematical>),
    Ge(Box<Mathematical>, Box<Mathematical>),
    Le(Box<Mathematical>, Box<Mathematical>),
    And(Box<Logical>, Box<Logical>),
    Or(Box<Logical>, Box<Logical>),
}

impl Logical {
    pub fn from_eq(expr1: Mathematical, expr2: Mathematical) -> Logical {
        Logical::Eq(
            Box::new(expr1),
            Box::new(expr2)
        )
    }

    pub fn from_ne(expr1: Mathematical, expr2: Mathematical) -> Logical {
        Logical::Ne(
            Box::new(expr1),
            Box::new(expr2)
        )
    }

    pub fn from_gt(expr1: Mathematical, expr2: Mathematical) -> Logical {
        Logical::Gt(
            Box::new(expr1),
            Box::new(expr2)
        )
    }

    pub fn from_ls(expr1: Mathematical, expr2: Mathematical) -> Logical {
        Logical::Ls(
            Box::new(expr1),
            Box::new(expr2)
        )
    }

    pub fn from_ge(expr1: Mathematical, expr2: Mathematical) -> Logical {
        Logical::Ge(
            Box::new(expr1),
            Box::new(expr2)
        )
    }

    pub fn from_le(expr1: Mathematical, expr2: Mathematical) -> Logical {
        Logical::Le(
            Box::new(expr1),
            Box::new(expr2)
        )
    }

    pub fn from_and(expr1: Logical, expr2: Logical) -> Logical {
        Logical::And(
            Box::new(expr1),
            Box::new(expr2)
        )
    }

    pub fn from_or(expr1: Logical, expr2: Logical) -> Logical {
        Logical::Or(
            Box::new(expr1),
            Box::new(expr2)
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    If(IfStatement),
    For(ForStatement),
    While(WhileStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Logical,
    pub body: Program
}


#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    range_left: Mathematical,
    range_right: Mathematical,
    body: Program
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    condition: Logical,
    body: Program
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub value: f64
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub name: String,
    pub expression: Mathematical
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub body: Vec<Statement>,
    pub variables: Vec<Variable>
}

impl Program {

    pub fn split_at(&self, i: usize) -> Program {
        Program { body: self.body.split_at(i).1.to_vec(), variables: self.variables.clone() }
    }

    pub fn push_statement(&mut self, stat: Statement) {
        self.body.push(stat);
    }

    pub fn concat(&mut self, prog: &mut Program) {
        self.body.append(&mut prog.body);
        self.variables.append(&mut prog.variables);
    }
}