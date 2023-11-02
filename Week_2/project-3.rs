<<<<<<< HEAD
fn main() {
    let principal :f64 = 210_000.00;
    let rate :f64 = 0.05;
    let years :f64 = 3.0;
    let amount = principal * ((1.0 - rate/100.0).powf(years));
    let compound_interest = amount - principal;
    println!("The compound interest is {}", compound_interest);
=======
fn main() {
    let principal :f64 = 210_000.00;
    let rate :f64 = 0.05;
    let years :f64 = 3.0;
    let amount = principal * ((1.0 - rate/100.0).powf(years));
    let compound_interest = amount - principal;
    println!("The compound interest is {}", compound_interest);
>>>>>>> fcb3da0e790e18cb2e940d846f2a72e80ceb0b11
}