use std::collections::HashMap;

pub fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores.get("something").copied().unwrap_or(0));
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    println!("{:?}", scores);
}
