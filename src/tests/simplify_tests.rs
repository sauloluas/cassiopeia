use crate::Expr;

#[test]
fn sub_display() {
	let a = Expr::new_var("a");
    let c = Expr::Val(6);
    let d = a.clone() - a + c;
    assert_eq!(format!("{d}"), "a - a + 6");
}

#[test]
fn a_minus_a_equals_0() {
	let a = Expr::new_var("a");
    let c = Expr::Val(6);
    let d = a.clone() - a + c;
    assert_eq!(d.simplify(), Expr::Val(6))
}
