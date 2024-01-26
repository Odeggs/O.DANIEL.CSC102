use std::io;

fn main() {
    // Get the input values of a, b and c
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter the values of a, b and c:");

    io::stdin().read_line(&mut a).unwrap();
    io::stdin().read_line(&mut b).unwrap();
    io::stdin().read_line(&mut c).unwrap();

    // Convert the strings to numbers
    let a: f64 = a.trim().parse().unwrap();
    let b: f64 = b.trim().parse().unwrap();
    let c: f64 = c.trim().parse().unwrap();

    // Calculate the discriminant
    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant > 0.0 {
        // Calculate the roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);

        println!("The roots of the quadratic equation are {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        // Calculate the root
        let root = -b / (2.0 * a);

        println!("The root of the quadratic equation is {}", root);
    } else {
        println!("The quadratic equation has no real roots");
    }
}