use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let n = Arc::new(Mutex::new(0));
    let n1 = Arc::clone(&n);
    let n2 = Arc::clone(&n);

    let handle1 = thread::spawn(move || {
        let mut data = n1.lock().unwrap();
        *data += 1;
        println!("[thread 1]=> {}", data)
    });

    let handle2 = thread::spawn(move || {
        let mut data = n2.lock().unwrap();
        *data += 1;
        println!("[thread 2]=> {}", data)
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
