// 类型转换
// rust提供 as 关键字进行显式类型转换

fn main() {
    let decimal = 3.1415926_f64;
    println!("{}", get_type(&decimal)); //f64

    let int = decimal as i64;
    println!("{}", get_type(&int)); //i64

    // only `u8` can be cast as `char`, not `i64`
    let character = 88_u8 as char;
    println!("{}", get_type(&character)); //char

    let string = decimal.to_string();
    println!("{}", get_type(&string)); //alloc::string::String

    println!(" 128 as a i8 is : {}", 128 as i16);

    // `size_of_val` 返回一个变量所占的字节数
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&string)); //size of `x` in bytes: 8
}

fn get_type<T>(_: &T) -> &str {
    return std::any::type_name::<T>();
}
