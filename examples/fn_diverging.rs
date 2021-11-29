
// 发散函数（diverging function）绝不会返回。 它们使用 ! 标记，这是一个空类型。


fn main(){
    let x: ! = panic!("This call never returns.");
    println!("You will never see this line!");
}

fn foo() -> !{
    panic!("This call never returns.");
}