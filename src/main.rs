use cassiopeia as cas;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cas::Expr::{self, *};

    let s: cas::expr::from_str::TokenParser<f64> = "5^2".parse()?;
    let s = Expr::try_from(s)?;
    println!("{s:#?}");
    Ok(())

}
