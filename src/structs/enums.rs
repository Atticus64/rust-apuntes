// Where structs give you a way of grouping together related fields and data, 
// like a Rectangle with its width and height, enums give you a way of 
// saying a value is one of a possible set of values. 
// For example, we may want to say that Rectangle is one of a set of possible 
// shapes that also includes Circle and Triangle. To do this, 
// Rust allows us to encode these possibilities as an enum.

// -> Large version
// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6
// }

// #[derive(Debug)]
// struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String
    // }
    
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}


pub fn main() {

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {home:?}");
    println!("loopback: {loopback:?}");
}
