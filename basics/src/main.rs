use std::{fs, io};

fn main() {
    let him = read_file("sample.txt".to_string());
    println!("{:?}", him);
    if let Ok(content) = him {
        println!("{content}");
    } else {
        println!("Error reading file");
    }
}

fn read_file(file_name: String) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}
