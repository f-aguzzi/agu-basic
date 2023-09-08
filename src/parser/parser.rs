use crate::ast::structs::{Program, IfStatement, Logical, Variable, Statement};
use super::lexer::{lexer, Token, Tokens, Line};

pub fn parser() -> Program {
    let mut code: Vec<Line> = lexer("LET X = 1".to_string());
    
    parse(code)
}

pub fn parseLogical(expr: Line) {
    
    for token in expr.tokens {

    }
}

pub fn parse_mathematical(expr: &mut Line) {

}

pub fn parse_let(expr: &mut Line) -> Variable {

    if expr.tokens.len() != 3 {
        panic!("Line {}: Syntax Error: incomplete LET statement", expr.line);
    }

    if expr.tokens.get(1).unwrap().text != String::from("=") {
        panic!("Line {}: Sytnax Error: invalid LET statement", expr.line);
    }

    let name = expr.tokens.get(0).expect("Variable parsing error").text.clone();
    let value = expr.tokens
        .get(2)
        .expect("Variable parsing error")
        .text.parse::<f64>()
        .unwrap();

    Variable { name: name, value: value }
}

pub fn parse(code: Vec<Line>) -> Program {

    let mut program: Program = Program {body: Vec::new(), variables: Vec::new()};
    let mut owned_code = code.to_owned();

    for (i, line) in owned_code.iter_mut().enumerate() {
        
        let first_token = line.tokens.remove(0);

        let statement: Statement;

        println!("Primo token: {:?}", first_token);

        match first_token.tpe {
            Tokens::IF => { statement = Statement::Let(parse_let(line)); },
            Tokens::ENDIF => { statement = Statement::Let(parse_let(line)); },
            Tokens::GOTO => { statement = Statement::Let(parse_let(line)); },
            Tokens::FOR => { statement = Statement::Let(parse_let(line)); },
            Tokens::WHILE => { statement = Statement::Let(parse_let(line)); },
            Tokens::LET => { statement = Statement::Let(parse_let(line)); },
            _=> { panic!("Line {}: Syntax Error", i); }
        }

        program.body.push(statement);
    }

    program

}