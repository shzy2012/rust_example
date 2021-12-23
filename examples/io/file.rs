// 文件输入输出（I/O）
/*
File结构体表示一个被打开的文件（它包裹了一个文件描述符），并赋予了对所表示的 文件的读写能力。
由于在进行文件 I/O（输入/输出）操作时可能出现各种错误，因此 File 的所有方法都 返回 io::Result<T> 类型，它是 Result<T, io::Error> 的别名。
这使得所有 I/O 操作的失败都变成显式的。借助这点，程序员可以看到所有的失败 路径，并被鼓励主动地处理这些情形。
*/ 

// File拥有资源，即文件描述符（file descriptor），它会在自身被 drop 时关闭文件。
use std::error::Error;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main(){
    // create 静态方法以只写模式（write-only mode）打开一个文件。若文件已经存在，则 旧内容将被销毁。否则，将创建一个新文件。
    let path = Path::new("out.txt");
    let display = path.display();

    // 以只写模式打开文件，返回 `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {:?}",display, why.description()),
        Ok(file) => file,
    };

    // 将字符串写进 `file`，返回 `io::Result<()>`
    let mut context = "hello world";
    match file.write_all(context.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }


    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // 读取文件内容到一个字符串，返回 `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }


    // 在生成输出之前，文件主机必须存在于当前路径中
    if let Ok(lines) = read_lines("./hosts") {
        // 使用迭代器，返回一个（可选）字符串
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }      
        }   
    }

}

// 输出包裹在 Result 中以允许匹配错误，
// 将迭代器返回给文件行的读取器（Reader）。
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}