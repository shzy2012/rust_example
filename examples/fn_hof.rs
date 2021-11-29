// 高阶函数（Higher Order Function, HOF）
// 输入一个或多个 函数，并且/或者产生一个更有用的函数的函数
// HOF 和惰性迭代器（lazy iterator）给 Rust 带来了函数式（functional）编程的风格。

fn is_old(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    let mut acc = 0;
    // 迭代：0，1, 2, ... 到无穷大
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break; //若大于上限则退出循环
        }

        if is_old(n_squared) {
            acc += n_squared //如果是奇数就计数
        }
    }

    println!("imperative style: {}", acc);

    // 函数式的写法
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // 所有自然数取平方
        .take_while(|&n| n < upper) // 取小于上限的
        .filter(|&n| is_old(n)) // 取奇数
        .fold(0, |sum, i| sum + i); // 最后加起来

    println!("functional style: {}", sum_of_squared_odd_numbers);
}
