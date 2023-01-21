#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct ColorRbg(i32, i32, i32);

impl User {
    fn build(name: &str, age: u8) -> Self {
        Self {
            name: String::from(name),
            age,
        }
    }

    fn get_name(&self) -> String {
        let name = &self.name;
        name.to_string()
    }

    fn get_age(&self) -> u8 {
        let age = self.age;
        age
    }

    fn change_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }
}

pub fn main() {
    let mut user = User::build("Mario", 35);
    let name = user.get_name();
    let age = user.get_age();
    let black = ColorRbg(0, 0, 0);

    println!("user: {name} edad: {age}");
    println!("color: {black:?}");
    user.change_name("Juan");
    println!("new user: {:?}", user);
}
