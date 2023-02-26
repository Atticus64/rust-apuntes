use crate::error::panic::main as panic;
use crate::error::result::main as result;

pub mod error;

fn main() {
    result();
    panic();
}
