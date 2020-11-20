use std::sync::mpsc;
use std::{thread, time};

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hello");
        tx.send(val).unwrap(); // send take ownership of val
    });

    // will block the main thread’s execution and
    // wait until a value is sent down the channel.
    // `try_recv` doesn’t block
    let received = rx.recv().unwrap();
    println!("Got {}!", received);

    let (tx, rx) = mpsc::channel();
    let tx_1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        for i in 1..10 {
            let val = String::from(format!("T1: Hello {}", i));
            tx.send(val).unwrap(); // send take ownership of val
            thread::sleep(time::Duration::from_secs(1))
        }
    });

    thread::spawn(move || {
        for i in 1..10 {
            let val = String::from(format!("T2: Hello {}", i));
            tx_1.send(val).unwrap(); // send take ownership of val
            thread::sleep(time::Duration::from_secs(1))
        }
    });

    for v in rx {
        println!("M: Got {}!", v);
    }
}
