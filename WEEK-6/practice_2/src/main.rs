use std::io;
fn checker() {

let mut input = String:: new();
println!("Enter a character:");
io::stdin().read_line(&mut input).expect("Failed to read input");
let ch:char =input.trim().parse().expect("invalud input");


if ch >='0' && ch <='9'
{
println!("Character '{}' is a digit",ch);
}
else
 {
println!("character'{}' is not digit",ch);
 }
}

fn main() {
    // calling function

    println!("Welcome!This program checks whether a character variable contains a digit or not");
    checker()
}