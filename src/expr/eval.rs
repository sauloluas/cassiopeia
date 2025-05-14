use crate::{expr::DyadOp, Expr};
use linearspace as lin;


impl<T> Expr<T> 
where
    T: std::ops::Add<Output = T> 
    + std::ops::Mul<Output = T> 
    + std::ops::Sub<Output = T>
    + std::ops::Div<Output = T>
    + lin::Zero
    + lin::One 
    // + lin::Pow
{
    pub fn eval(self) -> T {
        use Expr::*;
        use DyadOp::*;
        match self {
            Val(v) => v,
            Zero => T::zero(),
            One => T::one(),
            Dyad(op, l, r) => match op {
                Plus => {                
                    l.eval() + r.eval()
                },
                Times => {
                    l.eval() * r.eval()
                },
                Minus => {
                    l.eval() - r.eval()    
                },
                Frac => {
                    l.eval() / r.eval()
                },
                Power => {
                    todo!()
                }
            }
            _ => todo!()
        }
    }
}
