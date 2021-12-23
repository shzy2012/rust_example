// 路径
/*
Path 结构体代表了底层文件系统的文件路径。
Path 分为两种：
    posix::Path，针对 类 UNIX 系统；
    windows::Path，针对 Windows。
prelude 会选择并输出符合平台类型 的 Path 种类。

prelude 是 Rust 自动地在每个程序中导入的一些通用的东西，这样我们就不必每写 一个程序就手动导入一番。



Path 可从 OsStr 类型创建，并且它提供数种方法，用于获取路径指向的文件/目录 的信息。
注意 Path 在内部并不是用 UTF-8 字符串表示的，而是存储为若干字节（Vec<u8>）的 vector。因此，将 Path 转化成 &str 并非零开销的（free），且可能失败（因此它 返回一个 Option）。
*/ 

use std::path::Path;

fn main() {
    // 从 `&'static str` 创建一个 `Path`
    let path = Path::new(".");

    // `display` 方法返回一个可显示（showable）的结构体
    let display = path.display();

    // `join` 使用操作系统特定的分隔符来合并路径到一个字节容器，并返回新的路径
    let new_path = path.join("a").join("🐻");

    // 将路径转换成一个字符串切片
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
    // new path is ./a/🐻
}
