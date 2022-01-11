use std::sync::mpsc;
use std::thread;

fn main() {
    // 声明channle
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    // 单线程
    thread::spawn(move || {
        for i in 0..3 {
            if let Err(_) = tx1.send(i) {
                println!("发送失败");
            }
        }
    });

    // 多线程
    for i in 3..6 {
        let txn = tx.clone();
        thread::spawn(move || {
            if let Err(_) = txn.send(i) {
                println!("发送失败");
            }
        });
    }

    // drop(tx),确保返回None
    drop(tx);

    while let Ok(x) = rx.recv() {
        println!("got {}", x);
    }
}
