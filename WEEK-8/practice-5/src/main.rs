fn main() {
    // Create an empty vector "City"
    let mut city: Vec<String> = Vec::new();

    // Print City Vector
    println!("The City vector has {} elements", city.len());

    // Push new elements into the vector
    let mut input1 = String::new();
    println!("How many Cities do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num: usize = input1.trim().parse().expect("Invalid input");

    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter City {}: ", count + 1);
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_city: String = input2.trim().to_string();
        city.push(new_city);
    }

    println!("Your preferred cities are:");
    for (index, city_name) in city.iter().enumerate() {
        println!("{}: {}", index + 1, city_name);
    }
}
