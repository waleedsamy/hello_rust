use std::sync::{Arc, Mutex};

fn main() {
    let mx = Mutex::new(5);
    {
        let mut m = mx.lock().unwrap();
        *m += 1;
        // lock released here (Drop)
    }
    println!("{:?}", mx);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter_rc_clone = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut val = counter_rc_clone.lock().unwrap();
            *val += 1;
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", counter);
}
