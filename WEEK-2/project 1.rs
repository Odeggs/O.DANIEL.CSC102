<<<<<<< HEAD
fn main() {
    let principal = 520_000_000.0;
    let rate = 0.1;
    let years = 5;
    let amount = principal * (1.0 + rate/100).powr(years as f64);
    let compound_interest = amount - principal;
    println!("The compound interest is {}", compound_interest);
=======
fn main() {
    let principal = 520_000_000.0;
    let rate = 0.1;
    let years = 5;
    let amount = principal * (1.0 + rate/100).powr(years as f64);
    let compound_interest = amount - principal;
    println!("The compound interest is {}", compound_interest);
>>>>>>> fcb3da0e790e18cb2e940d846f2a72e80ceb0b11
}