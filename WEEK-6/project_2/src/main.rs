use std::io;

fn main() {
    let mut siblings_data = Vec::new();
    let mut num_siblings = String::new();

    io::stdin().read_line(&mut num_siblings)
        .expect("Failed to read number of siblings");
    let num_siblings: i32 = num_siblings.trim().parse()
        .expect("Please type a number!");

    for _ in 0..num_siblings {
        let mut first_name = String::new();
        let mut age = String::new();

        println!("Enter the first name of the sibling: ");
        io::stdin().read_line(&mut first_name)
            .expect("Failed to read first name");

        println!("Enter the age of the sibling: ");
        io::stdin().read_line(&mut age)
            .expect("Failed to read age");

        let age: i32 = age.trim().parse()
            .expect("Please type a number!");

        let sibling_info = if age > 18 {
            let mut marital_status = String::new();
            println!("Is the sibling married or single? (Enter M or S): ");
            io::stdin().read_line(&mut marital_status)
                .expect("Failed to read marital status");

            let marital_status = marital_status.trim().to_uppercase();
            let marital_info = if marital_status == "M" {
                let mut num_offspring = String::new();
                let mut city = String::new();

                println!("How many offspring does the sibling have? (Enter a number): ");
                io::stdin().read_line(&mut num_offspring)
                    .expect("Failed to read number of offspring");

                let num_offspring: i32 = num_offspring.trim().parse()
                    .expect("Please type a number!");

                println!("What city does the family live in? (Enter a string): ");
                io::stdin().read_line(&mut city)
                    .expect("Failed to read city");

                format!(" with {} offspring, living in {}", num_offspring, city)
            } else {
                String::new()
            };

            format!("{} is {} years old, married, and has a family in the city.", first_name.trim(), age)
        } else {
            let mut waec_status = String::new();
            let mut secondary_school = String::new();
            let mut class_level = String::new();

            println!("Has the sibling completed WAEC? (Y/N): ");
            io::stdin().read_line(&mut waec_status)
                .expect("Failed to read WAEC status");

            let waec_status = waec_status.trim().to_uppercase();
            if waec_status == "Y" {
                println!("Enter the secondary school attended: ");
                io::stdin().read_line(&mut secondary_school)
                    .expect("Failed to read secondary school");

                format!("{} is {} years old, has completed WAEC, and attended {}.", first_name.trim(), age, secondary_school.trim())
            } else {
                println!("Enter the current class level: ");
                io::stdin().read_line(&mut class_level)
                    .expect("Failed to read class level");

                format!("{} is {} years old, has not completed WAEC, and is in class {}.", first_name.trim(), age, class_level.trim())
            }
        };

        siblings_data.push(sibling_info);
    }

    println!("Siblings data:");
    for sibling in siblings_data {
        println!("{}", sibling);
    }
}