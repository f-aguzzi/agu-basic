use crate::ast::structs::*;
use super::lexer::{lexer, Token, Tokens, Line};

pub fn parser() -> Program {
    let mut code: Vec<Line> = lexer("LET X = 1".to_string());
    
    parse(code)
}

pub fn parseLogical(expr: Line) {
    
    for token in expr.tokens {

    }
}

pub fn parse_mathematical(expr: &mut Line) -> Mathematical {

    match expr.tokens.len() {
        1 => {
            let x = expr.tokens.get(0).unwrap();
            match x.text.parse::<f64>().is_err() {
                true => { return Mathematical::Var(x.text.clone()) }
                false => { return Mathematical::Literal(x.text.parse::<f64>().unwrap()) }
            } 
        }
        2 => { panic!("Line {}: incomplete math expression", expr.line) }
        _=> {
            let mut first_expression = expr.clone();
            first_expression.tokens = vec![first_expression.tokens.get(0).unwrap().clone()];

            let mut line_remainder = expr.clone();
            line_remainder.tokens.remove(0);
            line_remainder.tokens.remove(1);

            match expr.tokens.get(1).unwrap().text.as_str() {
                "+" => { return Mathematical::from_add(parse_mathematical(&mut first_expression), parse_mathematical(&mut line_remainder)) }
                "-" => { return Mathematical::from_sub(parse_mathematical(&mut first_expression), parse_mathematical(&mut line_remainder)) }
                "*" => { return Mathematical::from_mul(parse_mathematical(&mut first_expression), parse_mathematical(&mut line_remainder)) }
                "/" => { return Mathematical::from_div(parse_mathematical(&mut first_expression), parse_mathematical(&mut line_remainder)) }
                _=> panic!("Line {}: unrecognized operator", expr.line)
            }
         }
    }
}

pub fn parse_let(expr: &mut Line) -> LetStatement {

    match expr.tokens.len() {
        0 | 1 | 2 => panic!("Line {}: Syntax Error: incomplete LET statement", expr.line),
        3 => {
            if expr.tokens.get(1).unwrap().text != String::from("=") {
                panic!("Line {}: Sytnax Error: invalid LET statement", expr.line);
            }
            let name = expr.tokens.get(0).expect("Variable parsing error").text.clone();
            let value = expr.tokens
                .get(2)
                .expect("Variable parsing error")
                .text.parse::<f64>()
                .unwrap();
        }
        _=> {
            
        }
    }

    LetStatement { name: String::from("Peppino"), expression: Mathematical::Literal(1.00) }
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