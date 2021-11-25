// match关键字来提供模式匹配,类似switch

fn main() {
    let n = 1;

    match n {
        // 匹配单个值
        1 => println!("n is {}", n),
        // 匹配多个值
        2 | 4 | 6 => println!("n is {}", n),
        // 匹配一个闭区间范围
        13..=19 => println!("A teen"),
        _ => (),
    }

    let boolean = true;
    let binary = match boolean {
        // match 分支必须覆盖所有可能的值
        true => 1,
        false => 0,
    };

    println!("binary is {}", binary);

    // match元组中的结构
    let pair = (0, "a");
    println!("Tell me about {:?}", pair);
    match pair {
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, "a") => println!("`x` is `{:?}` and last is `0`", x),
        _ => println!("It doesn't matter what they are"),
    };
}
