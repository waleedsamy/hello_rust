use std::thread;
use std::time::Duration;
fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hallo from spawn thread {}", i);
            thread::sleep(Duration::from_millis(10));
        }
        println!("vector from main {:?}", v);
    });

    // drop(v); // v moved to prev thread

    for i in 1..10 {
        println!("hallo from main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Calling join on the handle blocks the thread currently running until
    // the thread represented by the handle terminates.
    handle.join().unwrap()
}
