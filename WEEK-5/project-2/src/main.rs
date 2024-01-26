fn main() {

    use std::io;
    // Get the input values of experience, age, and incentive
    let mut experience = String::new();
    let mut age = String::new();

    println!("Enter the values of experience and age:");

    io::stdin().read_line(&mut experience).unwrap();
    io::stdin().read_line(&mut age).unwrap();

    // Convert the strings to numbers
    let experience: i32 = experience.trim().parse().unwrap();
    let age: u32 = age.trim().parse().unwrap();

    let incentive = calculate_incentive(experience, age);

    println!("The annual incentive of the employee is: {}", incentive);
}

fn calculate_incentive(experience: i32, age: u32) -> u32 {
    if experience >= 4 {
        if age >= 40 {
            return 1560000;
        } else if age >= 30 {
            return 1480000;
        } else {
            return 1300000;
        }
    } else {
        return 100000;
    }
}
