use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    // 生产1
    thread::spawn(move || {
        let val = String::from("Hi from tx");
        tx.send(val).unwrap();

        for i in 0..=10 {
            tx.send(format!("{} from tx", i)).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 生产2
    thread::spawn(move || {
        let vals = vec![
            String::from("order-1  from tx2"),
            String::from("order-2  from tx2"),
            String::from("order-3  from tx2"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(1))
        }
    });

    // 消费
    for received in rx {
        println!("Got: {}", received);
    }

    // while let Ok(received) = rx.recv() {
    //     println!("Got: {}", received);
    // }
}
