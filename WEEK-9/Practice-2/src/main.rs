use std::io::Read;

fn main() {
   let mut file = std::fs::File::open("Welcome_message.txt").unwrap();
   let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    
}
