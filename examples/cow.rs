// 标准库定义Cow为智能指针，目的是为了避免程序在内存操作上多余的复制拷贝数据
// https://doc.rust-lang.org/std/borrow/enum.Cow.html
// 它的定义是 Clone-on-write，即写时克隆。本质上是一个智能指针。

/*
它有两个可选值：
    Borrowed，用于包裹对象的引用（通用引用）；
    Owned，用于包裹对象的所有者；
*/ 

use std::borrow::Cow;

fn main(){
    let mut cow:Cow<[_]> = Cow::Owned(vec![1,2,3]);
    let nums = cow.to_mut();
    println!("{:?}",nums);
}