// Method to print the get value
fn print_value(n: Option<&char>) {
    match n {
        Some(value) => println!("Element of vector: {}", value),
        None => println!("Index out of bounds"),
    }
}

fn main() {
    let v = vec!['R', 'u', 's', 't'];

    let mut input_string = String::new();
    println!("Enter an index value between 0 and 3");
    std::io::stdin().read_line(&mut input_string).expect("Failed to read input");

    // Index is the non-negative value which is smaller than the size of the vector
    let index: usize = input_string.trim().parse().expect("Invalid input");

    // Getting value at the given index value
    let ch: Option<&char> = v.get(index);
    print_value(ch);
}
