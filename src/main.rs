use std::io;

enum Token {
    Eded(f64),
    Plus,     // '+'
    Minus,    // '-'
    Multiply, // '*'
    Divide,   // '/'
    LParen,   // '('
    RParen,   // ')'
}

// lexer
fn lexer() -> Vec<Token> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");

    let mut tokens = Vec::new();
    for c in input.
        // TODO: Burada `match c` yazıb simvolları tokena cevir
    }
}

//parser


//exec
fn main() {
}
