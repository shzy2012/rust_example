use std::thread;
use std::time;
fn main() {
    let mut c = 1;
    loop {
        println!("定时器: {}秒", c);
        thread::sleep(time::Duration::from_secs(1));
        c += 1;
    }
}
