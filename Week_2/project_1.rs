fn main() {
    let principal = 520_000_000.0;
    let rate = 0.1;
    let n = 5.0;
    let amount = principal * (1.0 + rate)*(n);
    let compound_interest = amount - principal;
    println!("The compound interest is {}", compound_interest);
}