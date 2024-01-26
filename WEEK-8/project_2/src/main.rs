use std::io;

fn main() {
    // Create a vector to store developer information
    let mut developers: Vec<(String, u32)> = Vec::new();

    // Gather developer information from the user
    loop {
        println!("Enter developer name (or 'done' to finish):");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim();

        if name.to_lowercase() == "done" {
            break;
        }

        println!("Enter years of experience:");
        let mut experience = String::new();
        io::stdin().read_line(&mut experience).expect("Failed to read input");
        let experience: u32 = experience.trim().parse().expect("Invalid input");

        developers.push((String::from(name), experience));
    }

    // Find the developer with the highest years of experience
    let (max_name, max_experience) = developers.iter().max_by_key(|&&(_, exp)| exp).unwrap_or((&String::new(), &0));

    // Display the result
    if *max_experience > 0 {
        println!(
            "The developer with the highest experience is {} with {} years of experience.",
            max_name, max_experience
        );
    } else {
        println!("No developers found.");
    }
}
