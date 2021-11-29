// Rust中的闭包（closure），也叫做 lambda 表达式或者 lambda，是一类能够捕获周围作用域中变量的函数。例如，一个可以捕获 x 变量的闭包如下：
// 调用一个闭包和调用一个 函数完全相同，不过调用闭包时，输入和返回类型两者都可以自动推导，而输入变量名必须指明
/*
其他的特点包括：

声明时使用 || 替代 () 将输入参数括起来。
函数体定界符（{}）对于单个表达式是可选的，其他情况必须加上。
有能力捕获外部环境的变量。
*/

fn main() {
    // 通过闭包和函数分别实现自增。
    fn function(i: i32) -> i32 {
        i + 1
    }

    // 调用函数和闭包。
    println!("function: {}", function(12i32));

    // 闭包是匿名的，这里我们将它们绑定到引用。
    // 类型标注和函数的一样，不过类型标注和使用 `{}` 来围住函数体都是可选的。
    // 这些匿名函数（nameless function）被赋值给合适地命名的变量。
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    // 调用函数和闭包。
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // 没有参数的闭包，返回一个 `i32` 类型。
    // 返回类型是自动推导的。
    let one = || 1;
    println!("closure returning one: {}", one());

    // capture
    /*
    闭包天生就是灵活的，它会自动满足函数功能的要求.
    捕获（capture）灵活地适应使用场合，既可移动（move）又可 借用（borrow）变量.

    闭包可以通过以下手段捕获变量：
        引用：&T
        可变引用：&mut T
        值：T
    */

    // 这个闭包打印 `color`。它会借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color` 会一直保持被借用状态直到
    // `print` 离开作用域。
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let color = "green";
    let print = || println!("color:{}", color);

    // 调用闭包，闭包又借用 `color`。
    print();
    print();

    let mut count = 0;

    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 调用闭包。
    inc();
    inc();

    println!("{}", count);

    let reborrow = &mut count;
    // ^ 试一试：将此行注释去掉。
    println!("{}", reborrow);

    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    use std::mem;
    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
    // consume();
    // ^ 试一试：将此行注释去掉

    // 在竖线 | 之前使用 move 会强制闭包取得被捕获变量的所有权：
    // `Vec` 在语义上是不可复制的。
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    //println!("There're {} elements in vec", haystack.len());
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走
    // 之后继续使用它。

    // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `haystack`，因此之后
    // `haystack` 仍然可用，取消上面的注释也不会导致错误。
}
