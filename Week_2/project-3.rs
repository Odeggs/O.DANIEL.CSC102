fn main() {
    let principal :f64 = 210_000.00;
    let rate :f64 = 0.05;
    let years :f64 = 3.0;
    let amount = principal * ((1.0 - rate/100.0).powf(years));
    let compound_interest = amount - principal;
    println!("The compound interest is {}", compound_interest);
}