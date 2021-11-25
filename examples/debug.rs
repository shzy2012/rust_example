// https://rustwiki.org/zh-CN/rust-by-example/hello/print/print_debug.html
// Derive the `fmt::Debug` implementation for `Structure`. `Structure`

// `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印
#[derive(Debug)]
struct Person<'a>{
    name:&'a str,
    age:u8
}

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    let name = "Peter";
    let age = 27;
    let peter = Person{name,age};

    // Pretty print
    println!("{:#?}", peter);
}
