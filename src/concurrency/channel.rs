use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let members = ["Erian", "Scilee", "Midnight", "Taki", "Rick"];

        for member in members {
            tx.send(member).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    thread::spawn(move || {
        let members = ["Daizai", "Mario", "Greddy", "Jona", "Nicole"];

        for member in members {
            tx1.send(member).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    for programmer in rx {
        println!("Power Ranger: {programmer}");
    }
}
