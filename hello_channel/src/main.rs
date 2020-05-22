use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msgs = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for msg in rx {
        println!("messeage: {}", msg);
    }
}
