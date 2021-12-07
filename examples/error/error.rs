/*
在 Rust 中有多种处理错误的方式,它们多少有些 区别，使用场景也不尽相同。总的来说：
    1.显式的 panic 主要用于测试，以及处理不可恢复的错误。在原型开发中这很有用，比如 用来测试还没有实现的函数，不过这时使用 unimplemented 更能表达意图。另外在 测试中，panic 是一种显式地失败（fail）的好方法。
    2.Option 类型是为了值是可选的、或者缺少值并不是错误的情况准备的。比如说寻找 父目录时，/ 和 C: 这样的目录就没有父目录，这应当并不是一个错误。当处理 Option 时，unwrap 可用于原型开发，也可以用于能够确定 Option 中一定有值 的情形。然而 expect 更有用，因为它允许你指定一条错误信息，以免万一还是出现了错误。
    3.当错误有可能发生，且应当由调用者处理时，使用 Result。你也可以 unwrap 然后 使用 expect，但是除了在测试或者原型开发中，请不要这样做。
*/ 

// 我们将要看到的最简单的错误处理机制就是 panic。它会打印一个错误消息，开始 回退（unwind）任务，且通常会退出程序。这里我们显式地在错误条件下调用 panic：

fn give_princess(gift:&str){
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}

fn main(){
    give_princess("teddy bear");
    give_princess("snake");
}