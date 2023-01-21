mod structs {
    // pub mod name_folder
    pub mod user;
    pub mod enums;
    pub mod shapes;
}
// creando modulo 📂 y extrallendo el archivo a ejecutar
use structs::user::main as user;
use structs::enums::main as enums;

fn main() {
    // get login random number for auth
    user();
    enums();
}
