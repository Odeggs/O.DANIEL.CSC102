/*
Your MTH 101 Professor has asked you to develop a Rust program that performs the following calculations:
Area of Trapezium formula = height/2*(base1+base2)
Area of the rhombus formula = ½ × diagonal1 × diagonal2
Area of Parallelogram formula = base x altitude
Area of Cube formula = 6 x (length of the side)2 
Volume of Cylinder formula = π*radius2 *height

Using your knowledge of Rust Functions, develop the program that prompts a user to select an equation, reads inputs and then performs the corresponding calculations.


*/
use std::io;

fn main() {
    let mut choice = String::new();

    println!("Please enter the number corresponding to the equation you want to use:");
    println!("1. Area of Trapezium");
    println!("2. Area of the Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    io::stdin().read_line(&mut choice)
        .expect("Failed to read line");
    let choice: i32 = choice.trim().parse().expect("Invalid");

    if choice == 1 {
        area_of_trapezium();
    }
    else if choice == 2 {
        area_of_rhombus();
    }
    else if  choice == 3{
     area_of_parallelogram();   
    }
     else if choice == 4{
       area_of_cube();
     }
     else if choice == 5 {
     volume_of_cylinder();
     }
     return 
    /*let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        },
    };

    match choice {
        1 => area_of_trapezium(),
        2 => area_of_rhombus(),
        3 => area_of_parallelogram(),
        4 => area_of_cube(),
        5 => volume_of_cylinder(),
        _ => println!("Please enter a valid option!"),
    }*/
}

fn area_of_trapezium() {
    let mut height = String::new();
    let mut base1 = String::new();
    let mut base2 = String::new();

    println!("Please enter the height, base1, and base2 of the trapezium, separated by spaces:");

    io::stdin().read_line(&mut height)
        .expect("Failed to read line");
    io::stdin().read_line(&mut base1)parse().expect("Failed to read line");
    io::stdin().read_line(&mut base2)parse().expect("Failed to read line");

    
    let area = (height / 2.0) * (base1 + base2);
    println!("The area of the trapezium is: {}", area);
}

fn area_of_rhombus() {
    let mut diagonal1 = String::new();
    let mut diagonal2 = String::new();

    println!("Please enter the diagonal1 and diagonal2 of the rhombus, separated by a space:");

    io::stdin().read_line(&mut diagonal1)
        .expect("Failed to read line");
    io::stdin().read_line(&mut diagonal2)
        .expect("Failed to read line");

    let diagonal1: f64 = match diagonal1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for diagonal1!");
            return;
        },
    };

    let diagonal2: f64 = match diagonal2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for diagonal2!");
            return;
        },
    };

    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is: {}", area);
}

fn area_of_parallelogram() {
    let mut base = String::new();
    let mut altitude = String::new();

    println!("Please enter the base and altitude of the parallelogram, separated by a space:");

    io::stdin().read_line(&mut base)
        .expect("Failed to read line");
    io::stdin().read_line(&mut altitude)
        .expect("Failed to read line");

    let base: f64 = match base.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for base!");
            return;
        },
    };

    let altitude: f64 = match altitude.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for altitude!");
            return;
        },
    };

    let area = base * altitude;
    println!("The area of the parallelogram is: {}", area);
}

fn area_of_cube() {
    let mut edge_length = String::new();

    println!("Please enter the edge length of the cube:");

    io::stdin().read_line(&mut edge_length)
        .expect("Failed to read line");

    let edge_length: f64 = match edge_length.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for edge length!");
            return;
        },
    };

    let area = edge_length.powi(2);
    println!("The area of the cube is: {}", area);
}

fn volume_of_cylinder() {
    let mut radius = String::new();
    let mut height = String::new();

    println!("Please enter the radius and height of the cylinder, separated by a space:");

    io::stdin().read_line(&mut radius)
        .expect("Failed to read line");
    io::stdin().read_line(&mut height)
        .expect("Failed to read line");

    let radius: f64 = match radius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for radius!");
            return;
        },
    };

    let height: f64 = match height.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for height!");
            return;
        },
    };

    let volume = std::f64::consts::PI * radius.powi(2) * height;
    println!("The volume of the cylinder is: {}", volume);

}

