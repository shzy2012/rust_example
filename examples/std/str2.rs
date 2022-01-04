
// &str类型的字符串可以存储在任何地方
/*
静态存储区: &'static str 
堆分配: String::from
栈分配: str::from_utf8
*/ 

fn main(){

    // a 存储在堆上
    let mut a = String::from("where are you");

    println!("堆中字节序列的指针地址：{:p}",a.as_ptr());
    println!("字符串变量在栈上指针的地址：{:p}",&a);
    println!("字符串变量a长度：{}",a.len());//获取字节序列的字节数，而非字符个数
    println!("字符串变量a的容量：{}",a.capacity());
}