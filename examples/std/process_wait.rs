// 等待
// 如果你想等待一个 process::Child 完成，就必须调用 Child::wait，这会返回 一个 process::ExitStatus。

use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}

