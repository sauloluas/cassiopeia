


fn main() -> Result<(), Box<dyn std::error::Error>> {
    use symcom::Expr::{self, *};

    let s: symcom::expr::from_str::TokenParser<f64> = "2+a".parse()?;
    println!("{:#?}", s.clone());
    println!("");
    let s = Expr::try_from(s)?;
    println!("{s:#?}");
    // let equation = a2.equals(c);    // 2a = 6
    // let solution = equation.solve();

    // println!("{equation:#?}");
    Ok(())

}
