use std::fmt;
use std::collections::HashMap;

// rewrite rules:
// lim(var, val, exp) = #rewrite(var, val, exp)

// lim(var, val, exp)
#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Symb(String),
    Func(String, Vec<Expr>),
}

// left side-> a = a <- right side
#[derive(Debug)]
struct Rules {
    leftex: Expr,
    rightex: Expr,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Symb(name) => write!(f, "{}", name),
            Expr::Func(name, args) => {
                write!(f, "{}(", name)?;

                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} == {}", self.leftex, self.rightex)
    }
}

// it's for writing functions and it's similar to "class" to write methods
impl Rules {
    fn applysmth(&self, _expr: Expr) -> Expr {
        todo!();
    }
}


type Bindings = HashMap<String, Expr>;

fn patm(pattr : Expr, value : Expr) -> Option<Bindings>{
    let Bindings = HashMap::new();


    match (pattr, value) {
        (Symb(name), _) => todo!(),
        (Func(name, args), Func(name, args)),
    }

}

fn main() {
    // swap(pair(a, b)) == pair(b,a)
    use Expr::*;
    let swap = Rules {
        leftex: Func(
            "swap".to_string(),
            vec![Func(
                "pair".to_string(),
                vec![Symb("a".to_string()), Symb("b".to_string())],
            )],
        ),
        rightex: Func(
            "pair".to_string(),
            vec![Symb("b".to_string()), Symb("a".to_string())],
        ),
    };


    // Pattern swap(pair(a, b))
    let pattern = &swap.leftex;
    //Value swap(pair(f(c), g(d)))
    let value = Func(
        "swap".to_string(),
        vec![Func(
            "pair".to_string(),
            vec![
                Func("f".to_string(), vec![Symb("c".to_string())]),
                Func("g".to_string(), vec![Symb("d".to_string())]),
            ],
        )],
    );


    println!("Pattern : {}", pattern);
    println!("Value : {}", value);
    println!("Pattern Match : {:?}", patm(pattern, value));
}
