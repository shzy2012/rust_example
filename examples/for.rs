fn main() {
    // 区间标记 a..b,步长为1,包含a,不包含b
    for n in 1..100
    /*1..=100*/
    {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    /*********************迭代器********************/
    let mut names = vec!["A", "B", "C"];
    // iter - 在每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍 可以使用
    for elem in names.iter() {
        match elem {
            &"B" => println!("{}", elem),
            _ => println!(""),
        }
    }

    // iter_mut - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。
    for elem in names.iter_mut() {
        *elem = match elem {
            &mut "B" => "B1",
            _ => "Hello",
        }
    }

    println!("{:?}", names);

    // into_iter - 会消耗集合。在每次迭代中，集合中的数据本身会被提供。一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 “移除”（move）了。
    for elem in names.into_iter() {
        match elem {
            "B1" => println!("into:{}", elem),
            _ => println!(""),
        }
    }
}
