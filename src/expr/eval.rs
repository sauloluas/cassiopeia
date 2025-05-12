use crate::Expr;


impl<T> Expr<T> 
where
    T: std::ops::Add<Output = T> 
    + std::ops::Mul<Output = T> 
    + std::ops::Sub<Output = T>
    + lin2::Zero
    + lin2::One
{
    pub fn eval(self) -> T {
        use Expr::*;
        match self {
            Val(v) => v,
            Zero => T::zero(),
            One => T::one(),
            Plus(l, r) => {                
                l.eval() + r.eval()
            },
            Times(l, r) => {
                l.eval() * r.eval()
            },
            Minus(l, r) => {
                l.eval() - r.eval()
            }
            _ => todo!()
        }
    }
}
