use std::thread;
use std::time::Duration;

fn main() {
    let handler = thread::spawn(|| {
        for i in 0..10 {
            println!("spawned thread: Hello, {}", i);
            thread::sleep(Duration::from_millis(3));
        }
    });

    for i in 10..20 {
        println!("main thread: Hello, {}", i);
        thread::sleep(Duration::from_millis(2));
    }

    handler.join().unwrap();
}
