use std::fs::File;
use std::io::{Write, Result};

fn main() {
    let lager = vec!["33 export", "Desperados", "Goldberg", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo", "Williams", "", ""];
    let non_alch = vec!["Maltina", "Amstel Malta", "Malta gold", "Fayrouz"];
      
      
      let content = ("lager, stout, non_alch");


    let mut file = File::create("data.txt").expect("create failed");

    write_vector_to_file(&mut file, &lager).expect("write failed");
    write_vector_to_file(&mut file, &stout).expect("write failed");
    write_vector_to_file(&mut file, &non_alch).expect("write failed");

    println!("Stock data has been inserted");
}

fn write_vector_to_file(file: &mut File, data: &Vec<&str>) -> Result<()> {
    let content: String = data.join("\n");
    file.write_all(content.as_bytes())
}
