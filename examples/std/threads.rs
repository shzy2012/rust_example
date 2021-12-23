// 线程
/*
Rust 通过 spawn 函数提供了创建本地操作系统（native OS）线程的机制，该函数的参数是一个通过值捕获变量的闭包（moving closure）。
*/ 

use std::thread;

static NTHREADS: i64 = 10;

fn main(){
    
     // 提供一个 vector 来存放所创建的子线程（children）。
     let mut children = vec![];

    for i in 0..NTHREADS{
        children.push(thread::spawn(move||{
            // println!("this is thread number {}", i)
            i //return i
        }));
    }

    for child in children{
        // 等待线程结束。返回一个结果。
        let i = child.join();
        println!("this is thread number {}", i.unwrap())
    }
}