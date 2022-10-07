use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut mybox = Actor::new();
    let handle = mybox.start();

    // 发送消息
    mybox.send(String::from("Hello"));
    mybox.send(String::from("Actor"));

    handle.join().unwrap();
}

struct Actor {
    status: i32,
    mailbox: Arc<Mutex<VecDeque<String>>>, //信箱
}

impl Actor {
    fn new() -> Self {
        Actor {
            status: 0,
            mailbox: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    fn start(&mut self) -> thread::JoinHandle<()> {
        self.status = 1;
        let data = Arc::clone(&self.mailbox);

        // 开启新的线程，循环处理消息
        thread::spawn(move || loop {
            let mut mailbox = data.lock().unwrap();
            if let Some(message) = mailbox.pop_front() {
                println!("[DO]=> {}", message)
            }
        })
    }

    fn send(&mut self, message: String) {
        let mut mailbox = self.mailbox.lock().unwrap();
        mailbox.push_back(message);
    }

    fn stop(&mut self) {
        self.status = 2;
    }
}
