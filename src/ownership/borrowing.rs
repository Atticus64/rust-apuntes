use std::io::stdin;

pub fn main() {
    let mut name = String::new();
    println!("Ingresa tu nombre");
    stdin().read_line(&mut nombre).expect("Fallo al leer stdin");
    let len = name.len();
    name.truncate(len - 1); // delete \n

    if name.is_empty() {
        println!("Nombre no provisto usando pepe");
        name = String::from("pepe");
    }
    take_name(&name);
    // el & significa que se presto o se hizo un borrow
    // nombre regresa a este scope y es valido
    modify_name(&mut name);
    println!("El nombre es: {}", name)
}

fn take_name(name: &String) {
    println!("el nombre que me prestaron es: {name}");
}

fn modify_name(name: &mut String) {
    name.push_str("Dev");
    println!("Nombre modificado: {}", name);
}

