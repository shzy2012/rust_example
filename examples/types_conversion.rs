use std::convert::TryFrom;
use std::convert::TryInto;

// Rust 使用 trait 解决类型之间的转换问题。常见使用from和into两个trait
// from trait允许一种类型定义"怎么根据另一种类型生成自己"，因此它提供了一种类型转换的简单机制,在标准库中实现from
// into trait就是把from倒过来而已。也就是说，如果实现了from,那么同时自动实现了into
#[derive(Debug)]
struct Number {
    value: i64,
}

impl From<i64> for Number {
    fn from(item: i64) -> Self {
        Number { value: item }
    }
}

fn from_and_into() {
    let str1 = "hello";
    let str2 = String::from(str1);
    println!("str2->{}", str2);

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

/*
try_from和try_into用于易出错的转换,其返回值是result型
*/

#[derive(Debug)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn try_from_and_into() {
    EvenNumber::try_from(8);
}

fn main() {
    // from_and_into()
    try_from_and_into()
}
