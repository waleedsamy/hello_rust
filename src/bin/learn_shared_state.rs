use std::sync::Mutex;

fn main() {
    let mx = Mutex::new(5);

    {
        let mut m = mx.lock().unwrap();
        *m += 1;
        // lock released here (Drop)
    }

    println!("{}", *(mx.lock().unwrap()))
}
