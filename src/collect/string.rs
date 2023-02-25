// string in essential are just u8 arrays but they are complicated

pub fn main() {
    let mut s = String::new();
    let s2 = String::from("initial content");
    let s3 = " and second content".to_string();
    s.push_str("bar");
    s.push_str("foo");
    s.push('o');
    println!("{}", s);
    let phrase = s2 + &s3;
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", phrase);
    for b in "hola".bytes() {
        println!("{b}");
    }
    for c in "hello".chars() {
        println!("{c}");
    }
    println!("{}", s);
    let string_concat = concat!(10, "some text", 23.5);
    println!("{string_concat}");
}
