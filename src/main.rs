// Symbolic rewriting engine for now (it will be math language)

use std::collections::HashMap;
use std::fmt;


// AST
#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Symb(String),
    Func(String, Vec<Expr>),
}


// pattern and value
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

fn subs_bindings(bindings : &Bindings, expr : &Expr) -> Expr {
    use Expr::*;
    match expr {
        Symb(name) => {
            if let Some(value) = bindings.get(name){
                value.clone()
            } else {
                expr.clone()
            }
        },
        Func(name, args) => {
            let new_name = match bindings.get(name){
                Some(Symb(new_name)) => new_name.clone(),
                None => name.clone() ,
                Some(_) => panic!("Expected symbol i think in the functor name"),
            };
            let mut new_args = Vec::new();
            for arg in args {
                new_args.push(subs_bindings(bindings, &arg));
            }
            Func(new_name, new_args)
         }
    }
}


impl Rules {
    fn appliesall(&self, expr: &Expr) -> Expr {
        if let Some(bindings) = patm(&self.leftex, expr) {
            println!("MATCH: {:?}", bindings);
            subs_bindings(&bindings, &self.rightex)
        } else {
            use Expr::*;
            match expr {
                Symb(_) => expr.clone(),
                Func(name, args) => {
                    let mut new_args = Vec::new();
                    for arg in args {
                        new_args.push(self.appliesall(arg));
                    }
                    Func(name.clone(), new_args)
                }
            }
        }
    }
}

type Bindings = HashMap<String, Expr>;

fn patm_impl(pattr: &Expr, value: &Expr, bindings: &mut Bindings) -> bool {
    match (pattr, value) {
        // Pattern symbol acts as a variable bind it or verify consistency
        (Expr::Symb(name), _) => {
            if let Some(existing_val) = bindings.get(name) {
                existing_val == value
            } else {
                bindings.insert(name.clone(), value.clone());
                true
            }
        }
        (Expr::Func(p_name, p_args), Expr::Func(v_name, v_args)) => {
            if p_name != v_name || p_args.len() != v_args.len() {
                return false;
            }
            p_args
                .iter()
                .zip(v_args.iter())
                .all(|(p, v)| patm_impl(p, v, bindings))
        }
        _ => false,
    }
}

fn patm(pattr: &Expr, value: &Expr) -> Option<Bindings> {
    let mut bindings = HashMap::new();
    if patm_impl(pattr, value, &mut bindings) {
        Some(bindings)
    } else {
        None
    }
}

fn main() {
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
    // Value swap(pair(f(c), g(d)))

    let expr = Func("foo".to_string(),
                    vec![Func("swap".to_string(),
                              vec![Func("pair".to_string(),
                                        vec![Func("f".to_string(), vec![Symb("a".to_string())]),
                                             Func("g".to_string(), vec![Symb("b".to_string())])])]),
                         Func("swap".to_string(),
                              vec![Func("pair".to_string(),
                                        vec![Func("q".to_string(), vec![Symb("c".to_string())]),
                                             Func("z".to_string(), vec![Symb("d".to_string())])])])]);
    println!("Rule => {}", swap);
    println!("Expr => {}", &expr);
    println!("Expr' => {}", swap.appliesall(&expr));
}
