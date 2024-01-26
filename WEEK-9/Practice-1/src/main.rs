use std::fs::File;
use std::io::Write; // Import the Write trait

fn main() {
    let announce = "Week 9 - Rust File Input & Output\n";
    let dept = "Department of Software Engineering";

    let mut file = File::create("data.txt").expect("create failed");

    // Write text to the file
    file.write_all(b"Welcome to Rust Programming\n")
        .expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");


    println!("Data written to file.");
}
