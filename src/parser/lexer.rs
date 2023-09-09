use std::str::Lines;

#[derive(Clone, Debug, PartialEq)]
pub enum Tokens {
    // literals:
    LITERAL,
    // mathematical operators:
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    // logical operators:
    GREATER,
    LESSER,
    EQUAL,
    GREATEREQUAL,
    LESSEREQUAL,
    AND,
    OR,
    // booleans:
    TRUE,
    FALSE,
    // control flow:
    IF,
    ENDIF,
    FOR,
    WHILE,
    // let statement:
    LET,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub tpe: Tokens,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct Line {
    pub tokens: Vec<Token>,
    pub line: u16
}



pub fn lexer(code: String) -> Vec<Line> {
    // Split lines, remove empty lines
    let lines = code.lines();
    let mut lines_vector: Vec<String> = Vec::new();
    for line in lines {
        if !line.is_empty() {
            lines_vector.push(line.to_string());
        }
    }

    // Storage vector for tokenized lines
    let mut tokenized_lines: Vec<Line> = Vec::new();

    for (i, line) in lines_vector.iter().enumerate() {

        // Temporary vector for current line tokens
        let mut tokenized_vec: Vec<Token> = Vec::new();

        let tokens: Vec<_> = line.split_whitespace().collect();

        for token in tokens {
            let tpe = match token {
                "+" => Tokens::PLUS,
                "-" => Tokens::MINUS,
                "*" => Tokens::TIMES,
                "/" => Tokens::DIVIDE,
                ">" => Tokens::GREATER,
                "<" => Tokens::LESSER,
                "=" => Tokens::EQUAL,
                ">=" => Tokens::GREATEREQUAL,
                "<=" => Tokens::LESSEREQUAL,
                "TRUE" => Tokens::TRUE,
                "FALSE" => Tokens::FALSE,
                "AND" => Tokens::AND,
                "OR" => Tokens::OR,
                "IF" => Tokens::IF,
                "ENDIF" => Tokens::ENDIF,
                "FOR" => Tokens::FOR,
                "WHILE" => Tokens::WHILE,
                "LET" => Tokens::LET,
                _=> Tokens::LITERAL
            };

            let text: String = token.to_owned();
            tokenized_vec.push(
                Token {
                    tpe,
                    text
                }
            )
        }

        tokenized_lines.push(
            Line { tokens: tokenized_vec, line: (i+1) as u16 }
        );

    }

    tokenized_lines
}


#[test]
fn test_lexer() {
    let test_code = String::from("LET X = 11\nFOR 1 X");
    let lexed_vec = lexer(test_code);

    // First line, first token
    assert_eq!(lexed_vec.get(0).unwrap().tokens.get(0).unwrap().clone(), Token { tpe: Tokens::LET, text: String::from("LET") });
    // First line, second token
    assert_eq!(lexed_vec.get(0).unwrap().tokens.get(1).unwrap().clone(), Token { tpe: Tokens::LITERAL, text: String::from("X") });
    // First line, third token
    assert_eq!(lexed_vec.get(0).unwrap().tokens.get(2).unwrap().clone(), Token { tpe: Tokens::EQUAL, text: String::from("=") });
    // First line, fourth token
    assert_eq!(lexed_vec.get(0).unwrap().tokens.get(3).unwrap().clone(), Token { tpe: Tokens::LITERAL, text: String::from("11") });

    // Second line, first token
    assert_eq!(lexed_vec.get(1).unwrap().tokens.get(0).unwrap().clone(), Token { tpe: Tokens::FOR, text: String::from("FOR") });
    // Second line, second token
    assert_eq!(lexed_vec.get(1).unwrap().tokens.get(1).unwrap().clone(), Token { tpe: Tokens::LITERAL, text: String::from("1") });
    // Second line, third token
    assert_eq!(lexed_vec.get(1).unwrap().tokens.get(2).unwrap().clone(), Token { tpe: Tokens::LITERAL, text: String::from("X") });

    // Line numbers
    assert_eq!(lexed_vec.get(0).unwrap().line, 1);
    assert_eq!(lexed_vec.get(1).unwrap().line, 2);
}