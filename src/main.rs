mod eng;
use eng::lexer::*;


// lim(var, val, exp)
enum Expr{
    Symb(String),
    Func(String , Vector<Expr>)
}

// lefts side-> a = a <- right side
struct Rules {
    leftex : Expr,
    rightex : Expr,
}


fn main() {
    println!("main");
}
