mod eng;
use eng::lexer::*;


enum Expr{
    Sym(String),
    Func(String , Vector<Expr>)
}

fn main() {
    println!("main");
}
