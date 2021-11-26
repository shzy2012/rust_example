// match关键字来提供模式匹配,类似switch

#[allow(dead_code)]
enum Color {
    // 这三个取值仅由它们的名字（而非类型）来指定。
    Red,
    Blue,
    Green,
    // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn age() -> u32 {
    18
}

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

    // 枚举
    let color = Color::RGB(122, 17, 40);
    // 试一试 ^ 将不同的值赋给 `color`

    println!("What color is it?");
    // 可以使用 `match` 来解构 `enum`。
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
        // 不需要其它分支，因为所有的情形都已覆盖
    }

    //引用

    let reference = 20;
    match reference{
        ref v/*引用*/ => println!("v is {}",v/*引用值*/),
    }

    match &reference{
        &v/*引用*/ => println!("v is {}",v/*引用值*/),
    }

    /*绑定
    在match中，若间接地访问一个变量，则不经过重新绑定就无法在分支中再使用它。match提供了 @ 符号来绑定变量到名称：
    */
    match age() {
        0 => (),
        n @ 10 => println!("n is {}", n),
        n @ 18 => println!("n is {}", n),
        _ => (),
    }
}
