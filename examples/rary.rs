
// rustc --crate-type=lib rary.rs
// library.rlib
// 默认情况下，库会使用 crate 文件的名字，前面加上 “lib” 前缀，但这个默认名称可以 使用 crate_name 属性 覆盖。

pub fn public_function(){
    println!("called  the `public_function()`");
}

fn private_function(){
    println!("called the `private_function()`");
}

pub fn indirect_access(){
    println!("called the `indirect_access()`");
    private_function();
}