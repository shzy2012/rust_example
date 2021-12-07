// Option 和 unwrap
/*
在标准库（std）中的 Option<T>枚举类型，用于有 “不存在” 的可能性的情况。它表现为以下两个 “option”（选项）中 的一个：
    Some(T)：找到一个属于 T 类型的元素
    None：找不到相应元素
这些选项可以通过 match 显式地处理，或使用 unwrap 隐式地处理。隐式处理要么 返回 Some 内部的元素，要么就 panic。
*/ 


fn give_commoner(gift: Option<&str>) {
    // 指出每种情况下的做法。
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No gift? Oh well."),
    }
}

fn give_princess(gift: Option<&str>) {
    // `unwrap` 在接收到 `None` 时将返回 `panic`。
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main(){
    let food = Some("chicken");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}