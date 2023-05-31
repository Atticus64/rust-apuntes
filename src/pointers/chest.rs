use std::mem::drop;
use std::ops::Deref;

#[derive(Debug)]
struct Chest<T>(T);

impl<T> Chest<T> {
    fn new(value: T) -> Chest<T> {
        Chest(value)
    }
}

impl<T> Deref for Chest<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for Chest<T> {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn main() {
    let x = 5;
    let y = Chest::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
