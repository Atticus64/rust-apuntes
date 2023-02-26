use std::collections::HashMap;

pub fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores.get("something").unwrap());
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    println!("{:?}", scores);
}
