// https://www.twle.cn/c/yufei/rust/rust-basic-bitwise-operators.html

fn main() {
    let a: i32 = 2; // 二进制表示为 0 0 0 0 0 0 1 0
    let b: i32 = 3; // 二进制表示为 0 0 0 0 0 0 1 1

    let mut result: i32;

    // 相同位都是 1 则返回 1 否则返回 0
    result = a & b;
    println!("(a & b) => {} ", result);

    // 相同位只要有一个是 1 则返回 1 否则返回 0
    result = a | b;
    println!("(a | b) => {} ", result);

    // 相同位不相同则返回 1 否则返回 0
    result = a ^ b;
    println!("(a ^ b) => {} ", result);

    // 把位中的 1 换成 0 ， 0 换成 1
    result = !b;
    println!("(!b) => {} ", result);

    // 操作数中的所有位向左移动指定位数，右边的位补 0
    result = a << b;
    println!("(a << b) => {}", result);

    // 操作数中的所有位向右移动指定位数，左边的位补 0
    result = a >> b;
    println!("(a >> b) => {}", result);
}
