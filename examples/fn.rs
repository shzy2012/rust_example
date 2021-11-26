/*
fn 关键字来声明，函数的参数需要标注类型。函数返回一个值，返回类型必须在箭头 -> 之后指定
函数最后的表达式将作为返回值。也可在函数内使用 return 语句来提前返回值。return 甚至也可在循环或 if 内部使用。
*/
fn main() {
    fizzbuzz_to(100);

    // 多个返回值
    println!("result=>{:?}",mut_resunt());//result=>(false, 0)
    let (p1,p2) = mut_resunt();
    println!("{}{}",p1,p2); //false0
}

// 当函数返回 `()` 时，函数签名可以省略返回类型
fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}

// 一个 “不” 返回值的函数。实际上会返回一个单元类型 `()`。
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// 一个返回布尔值的函数
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // 边界情况，提前返回
    if rhs == 0 {
        return false;
    }

    // 这是一个表达式，这里可以不用 `return` 关键字
    lhs % rhs == 0
}

fn mut_resunt() -> (bool,u32){
   (false,0)
}
