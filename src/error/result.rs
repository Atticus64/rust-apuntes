use std::fs::File;
use std::io::{self, Read};

fn read_string_file() -> Result<String, io::Error> {
    let mut content = String::new();

    File::open("foo.txt")?.read_to_string(&mut content)?;

    Ok(content)
}

pub fn main() {
    let file = read_string_file().expect("Error reading file foo.txt");
    println!("{}", file);
}
