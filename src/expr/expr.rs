use super::dyad_op::DyadOp::{self, *};


pub type ExprRef<T> = Box<Expr<T>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<T> {
    Val(T),
    Var(String),
    Sym(String, ExprRef<T>),
    Zero,
    One,
    Undef,
    Inf,
    Neg(ExprRef<T>),
    Dyad(DyadOp, ExprRef<T>, ExprRef<T>),
    Equals(ExprRef<T>, ExprRef<T>),
}

impl<T> Expr<T> {
    pub fn var(name: &str) -> Expr<T> {
        Expr::Var(name.to_string())
    }
}

impl<T> std::ops::Add for Expr<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Expr::Dyad(Plus, Box::new(self), Box::new(rhs))
    }
}

impl<T> std::ops::Sub for Expr<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Expr::Dyad(Minus, Box::new(self), Box::new(rhs))
    }
}

impl<T> std::ops::Mul for Expr<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Expr::Dyad(Times, Box::new(self), Box::new(rhs))
    }
}

impl<T> std::ops::Div for Expr<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Expr::Dyad(Frac, Box::new(self), Box::new(rhs))
    }
}

impl<T> Expr<T> {
    pub fn equals(self, rhs: Self) -> Self {
        Expr::Equals(Box::new(self), Box::new(rhs))
    }
}


// impl<T> Expr<T> {
//     pub fn solve(self) -> Self {
//         match self {
//             Equals => {

//             }
//             _ => todo!()
//         }
//     }
// }

impl<T> std::fmt::Display for Expr<T> 
where T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expr::*;
        match self {
            Val(v) => write!(f, "{v}"),
            Var(v) => write!(f, "{v}"),
            Sym(v, _) => write!(f, "{v}"),
            Zero => write!(f, "0"),
            One => write!(f, "1"),
            Dyad(op, a, b) => write!(f, "{a} {op} {b}"),
            _ => todo!(),
        }
    }
}

