use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let mut user_role = String::new();
    println!("Enter which role you are:");
    println!("a if administrator\np if project manager\ne if employee\nc if customer\nv if vendor\n");

    io::stdin().read_line(&mut user_role).expect("Failed to read");
    let user_role = user_role.trim();

    match user_role {
        "a" => print_file_content("globacon_db.sql"),
        "p" => print_file_content("project_tb.sql"),
        "e" => print_file_content("staff_tb.sql"),
        "c" => print_file_content("customertable_tb.sql"),
        "v" => print_file_content("dataplantable_tb.sql"),
        _ => println!("No access to info"),
    }
}

fn print_file_content(file_name: &str) {
    if let Ok(mut file) = File::open(file_name) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");
        print!("{}", contents);
    } else {
        println!("Error opening file");
    }
}
