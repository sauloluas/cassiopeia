use crate::Expr;


impl<T> Expr<T> 
where
    T: PartialEq
    + std::ops::Add<Output = T>
{
    pub fn simplify(self) -> Self {
        use Expr::*;
        match self {
            Plus(l, r) => {
                let unbox_l = *l;
                let unbox_r = *r;
                match(unbox_l, unbox_r) {

                    // a + 0 = a
                    (a, Zero) => a.simplify(),

                    // 0 + b = b
                    (Zero, b) => b.simplify(),

                    // a + (-b) = a - b
                    (a, Neg(b)) => Minus(Box::new(a), b).simplify(),

                    // (-a) + b = b - a
                    (Neg(a), b) => Minus(Box::new(b), a).simplify(),

                    // a + b = a + b
                    (Val(a), Val(b)) => Val(a + b),

                    // a + b = a + b
                    (a, b) => {
                        Plus(
                            Box::new(a.simplify()), 
                            Box::new(b.simplify())
                        ).simplify()
                    }

                }
            },

            // a - a = 0
            Minus(l, r) if *l == *r => Zero,

            other => other,
        }
    }
}
