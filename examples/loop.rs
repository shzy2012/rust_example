// loop关键字来实现一个无限循环
fn main() {
    let mut count = 0u32;

    loop {
        count += 1;
        if count % 2 == 0 {
            // 跳过此次循环
            continue;
        }

        if count > 100 {
            //退出循环
            break;
        }

        println!("{}", count);
    }
}
