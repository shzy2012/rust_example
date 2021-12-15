use regex::Regex;
use std::mem;

fn main() -> Result<(), std::io::Error> {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));

    let x = b'x';
    let mut y = x;
    y = b'y';
    println!("x:{:p}-y:{:p}", &x, &y);

    let mut v = vec![1, 2, 3];
    v.push(4);

    // while let
    while let Some(x) = v.pop() {
        println!("{}", x);
    }

    let n = 11;
    let big_n = if n <= 10 && n > -10 { 10 * n } else { n / 2 };

    println!("what max n {}", big_n);

    assert_eq!(1, 1);

    println!("{:?}", std::mem::size_of::<&[u32; 5]>()); //8 普通指针
    println!("{:?}", std::mem::size_of::<&[u32]>()); //16 胖指针

    // 生命周期参数
    let i = &10i32;
    let m: &'static i32 = &10i32;
    let str1 = Box::new("hello world");

    // 内存地址分配
    /*
    0x7ff7b23cbd1c   栈
    0x1082001fd      静态数据区
    0x10d708214      静态数据区
    0x600000b840a0   堆
    0x600000e7c0a0
    */
    println!("i=>{:p}\nm=>{:p}\nst1=>{:p}", i, m, str1);

    // 内存布局
    /*
     char -> 4 bytes
     u8   -> 1 byte
     i32  -> 4 bytes
    */
    let a: (char, u8, i32) = ('a', 10, 356);
    println!("{}", mem::size_of::<(char, u8, i32)>());
    println!("{}", mem::align_of::<(char, u8, i32)>());
    println!("{}", mem::size_of_val(&a));

    // main 返回错误
    let f = std::fs::File::open("bat.txt")?;
    Ok(())
}
