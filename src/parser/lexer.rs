use std::str::Lines;

pub fn lexer(code: String) -> Vec<String> {
    let lines = code.lines();
    let mut lines_vector: Vec<String> = Vec::new();
    for line in lines {
        if !line.is_empty() {
            lines_vector.push(line.to_string());
        }
    }

    lines_vector
}