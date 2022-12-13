// Unsafe Superpowers
// https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html

/*
1.Dereference a raw pointer
2.Call an unsafe function or method
3.Access or modify a mutable static variable
4.Implement an unsafe trait
5.Access fields of unions
*/

unsafe fn dangerous() {}

// 常量和不可变静态变量之间的细微差别是静态变量中的值在内存中具有固定地址，使用该值将始终访问相同的数据。
// 另一方面，常量允许在使用时复制它们的数据。另一个区别是静态变量可以是可变的。访问和修改可变静态变量是 不安全的
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    // 1. Dereferencing a Raw Pointer
    // Unsafe Rust has two new types called raw pointers that are similar to references.As with references, raw pointers can be immutable or mutable and are written as *const T and *mut T, respectively.
    // Unsafe Rust 有两种裸指针： 不可变裸指针 *const T 和 可变裸指针 *mut T
    // 裸指针的功能有：
    // 1. 允许通过将不可变指针和可变指针或多个可变指针指向同一位置来忽略借用规则
    // 2. 不保证指向有效内存
    // 3. 允许为空
    // 4. 不要实施任何自动清理

    // 可以在安全代码中创建原始指针；
    let mut num = 5;
    let r1 = &num as *const i32; //创建不可变裸指针
    let r2 = &mut num as *mut i32; //创建可变裸指针

    println!("{:?},{:?}", r1, r2);
    // 不能在不安全块之外取消引用原始指针;
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    // 创建指向内存中任意位置的原始指针
    let address = 0x012345usize;
    let r = address as *const i32;
    println!("{:?}", r);
    // unsafe {
    //     println!("r is {}", *r); //无效值
    // }

    // 调用不安全代码
    unsafe {
        dangerous();
    }

    // 修改静态可变变量
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
