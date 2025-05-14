use cassiopeia as cas;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cas::Expr::{self};

    let s: Expr<f64> = "5 * 4 - 2".parse()?;
    println!("{s}");
    Ok(())
}
