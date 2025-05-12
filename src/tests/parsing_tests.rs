use crate::Expr;

#[test]
fn parse_mruv1() {
	let s: Expr<f64> = "S".parse().expect("failure parsing");
	assert_eq!(format!("{s}"), "S");
}

#[test]
fn parse_mruv2() {
	let mru_txt: Expr<f64> = "So + V*t".parse().expect("failure parsing");
	let mru_nat = 
		Expr::new_var("So") 
		+ Expr::new_var("V") 
		* Expr::new_var("t");
	assert_eq!(mru_txt, mru_nat);
}
