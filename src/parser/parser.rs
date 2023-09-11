use crate::ast::structs::*;
use super::lexer::{lexer, Token, Tokens, Line};


pub fn parser() -> Program {
    let code: Vec<Line> = lexer("LET X = 1".to_string());
    
    parse(code)
}

pub fn parse_logical(expr: &Line) -> Logical {

    let is_logical = |i: &Token| -> bool {
        match i.tpe {
            Tokens::EQUAL | Tokens::GREATER | Tokens::GREATEREQUAL | Tokens::LESSER | Tokens::LESSEREQUAL => true,
            _=> false
        }
    };

    let find_logical_pos = |v: &Vec<Token>| -> usize {
        for (i, token) in v.iter().enumerate() {
            if is_logical(token) {
                return i
            }
        }
        panic!("Error: invalid logical expression.");
    };
    
    match expr.tokens.len() {
        1 => {
            let x = expr.tokens.get(0).unwrap();
            match x.text.parse::<i32>().is_err() {
                true => { return Logical::Var(x.text.clone()) }
                false => {
                    match x.text.as_str() {
                        "TRUE" | "1" => { return Logical::Literal(1) }
                        "FALSE" | "0" => { return Logical::Literal(0) }
                        _=> panic!("Invalid boolean value at line {}", expr.line)
                    }
                }
            } 
        }
        2 => { panic!("Line {}: incomplete logical expression", expr.line) }
        _=> {
            let index = find_logical_pos(&expr.tokens);
            let halves = expr.tokens.split_at(index);
            let mut first_half = Line { tokens: halves.0.to_vec(), line: expr.line };
            let mut second_half_tokens = halves.1.to_vec();
            second_half_tokens.remove(0);
            let mut second_half = Line { tokens: second_half_tokens, line: expr.line };

            println!("First half: {:?}", first_half);
            println!("Second half: {:?}", second_half);
            println!("Operator: {:?}", expr.tokens.get(index).unwrap().tpe);

            match expr.tokens.get(index).unwrap().tpe {
                Tokens::EQUAL => { return Logical::from_eq(parse_mathematical(&mut first_half), parse_mathematical(&mut second_half)) }
                Tokens::GREATER => { return Logical::from_gt(parse_mathematical(&mut first_half), parse_mathematical(&mut second_half)) }
                Tokens::GREATEREQUAL => { return Logical::from_ge(parse_mathematical(&mut first_half), parse_mathematical(&mut second_half)) }
                Tokens::LESSER => { return Logical::from_ls(parse_mathematical(&mut first_half), parse_mathematical(&mut second_half)) }
                Tokens::LESSEREQUAL => { return Logical::from_le(parse_mathematical(&mut first_half), parse_mathematical(&mut second_half)) }
                Tokens::AND => { return Logical::from_and(parse_logical(&mut first_half), parse_logical(&mut second_half)) }
                Tokens::OR => {  return Logical::from_or(parse_logical(&mut first_half), parse_logical(&mut second_half))  }
                _=> panic!("Line {}: invalid logical expression.", expr.line)
            }

        }
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
            line_remainder.tokens.remove(0);

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
        _=> {
            if expr.tokens.get(1).unwrap().tpe != Tokens::EQUAL {
                panic!("Line {}: Sytnax Error: invalid LET statement", expr.line);
            }

            let mut line_remainder = expr.clone();
            line_remainder.tokens.remove(0);
            line_remainder.tokens.remove(0);

            let name = expr.tokens.get(0).expect("Variable parsing error").text.clone();
            let value = parse_mathematical(&mut line_remainder);

            LetStatement { name: name, expression: value }
        }
    }
}

pub fn parse_if(line: &Line, code: &Vec<Line>, program: &mut Program) -> Program {

    let if_body: Vec<Line>;

    if_body = Vec::new();

    IfStatement {
        condition: parse_logical(line),
        body: parse(if_body)
    };

    println!("parse_if: THIS METHOD IS UNIMPLEMENTED");
    
    program.clone()
}

pub fn parse(code: Vec<Line>) -> Program {

    let mut program: Program = Program {body: Vec::new(), variables: Vec::new()};
    let mut owned_code = code.to_owned();

    for (i, line) in owned_code.iter_mut().enumerate() {
        
        let first_token = line.tokens.remove(0);

        println!("Primo token: {:?}", first_token);

        match first_token.tpe {
            Tokens::IF => { 
                let mut prog = parse_if(&line, &code, &mut program);
                program.concat(&mut prog);
            },
            Tokens::ENDIF => {  },
            Tokens::FOR => {  },
            Tokens::WHILE => {  },
            Tokens::LET => { program.body.push(Statement::Let(parse_let(line))); },
            _=> { panic!("Line {}: Syntax Error", i); }
        }

    }

    program

}


#[test]
fn parse_let_test() {
    let test_code = String::from("LET X = 11 + 2 + 14");
    let lexed_code = lexer(test_code);
    let parsed_code = parse(lexed_code);

    println!("{:?}", parsed_code);
    
    let test_object = LetStatement {
        name: String::from("X"),
        expression: Mathematical::from_add(
            Mathematical::Literal(11.0),
            Mathematical::from_add(Mathematical::Literal(2.0), Mathematical::Literal(14.0)))
    };

    assert_eq!(parsed_code.body.get(0).unwrap().clone(), Statement::Let(test_object));
}

#[test]
fn parse_logical_test() {
    let test_code = String::from("11 + 1 < 2");
    let mut lexed_code = lexer(test_code).get(0).unwrap().clone();
    let parsed_code = parse_logical(&mut lexed_code);

    println!("{:?}", parsed_code);

    let test_object = Logical::from_ls(
        Mathematical::from_add(Mathematical::Literal(11.0), Mathematical::Literal(1.0)),
        Mathematical::Literal(2.00)
    );

    assert_eq!(parsed_code, test_object);
}