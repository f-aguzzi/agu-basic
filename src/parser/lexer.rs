use std::str::Lines;

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
    // booleans:
    TRUE,
    FALSE,
    // control flow:
    IF,
    ENDIF,
    GOTO,
    FOR,
    WHILE,
    // let statement:
    LET,
}

pub struct Token {
    tpe: Tokens,
    text: String,
}



pub fn lexer(code: String) -> Vec<Vec<Token>> {
    // Split lines, remove empty lines
    let lines = code.lines();
    let mut lines_vector: Vec<String> = Vec::new();
    for line in lines {
        if !line.is_empty() {
            lines_vector.push(line.to_string());
        }
    }

    // Storage vector for tokenized lines
    let mut tokenized_lines: Vec<Vec<Token>> = Vec::new();

    for line in lines_vector {

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
                "IF" => Tokens::IF,
                "ENDIF" => Tokens::ENDIF,
                "GOTO" => Tokens::GOTO,
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

        tokenized_lines.push(tokenized_vec);

    }

    tokenized_lines
}