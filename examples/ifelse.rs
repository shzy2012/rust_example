//rust增加布尔判断条件,每个条件后面都跟着一个代码块(所有分支都必须返回相同的类型)
fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    // ifelse表达式
    let m = if n > 10 { 10 } else { 1 };

    println!("m is {}", m); //m is 1
}
