// loop关键字来实现一个无限循环
fn main() {
    let mut count = 0u32;

    // 1.无限循环
    loop {
        count += 1;
        if count % 2 == 0 {
            // 跳过此次循环
            continue;
        }

        if count > 10 {
            //退出循环
            break;
        }

        println!("loop->{}", count);
    }

    count = 0;

    // 2.返回处理结果
    let loop_result = loop {
        count += 1;
        if count == 10 {
            break count;
        }
    };

    println!("loop_result is {}", loop_result);

    // 3.嵌套+标签
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            // 这只是中断内部的循环
            // break;

            // 中断outer层循环
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
