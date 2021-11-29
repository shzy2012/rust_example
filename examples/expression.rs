// 表达式

fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 将此表达式赋给 `y`
        x_cube + x_squared + x //代码块中的最后一条语句是代码块中实际执行的最后一条语句，而不一 定是代码块中最后一行的语句
    };

    println!("x->{},y->{}", x, y); //x->5,y->155
}
