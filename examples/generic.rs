// 泛型（generic）是关于泛化类型和函数功能
// 泛型极大地减少了代码的重复
/*
泛型的类型参数是使用尖括号和大驼峰命名的名称：<Aaa, Bbb, ...> 来指定的。
泛型类型参数一般用 <T> 来表示。在 Rust 中，“泛型的” 除了表示 类型，还表示可以接受一个或多个泛型类型参数 <T> 的任何内容。
任何用泛型类型参数 表示的类型都是泛型，其他的类型都是具体（非泛型）类型。
*/

// 定义一个名为 foo 的 泛型函数，它可接受类型为 T 的任何参数 arg：
// fn foo<T>(arg: T) {}

// 定义A
#[derive(Debug)]
struct A;

// 在定义类型 `Single` 时，第一次使用类型 `A`
struct Single(A);

// 泛型
#[derive(Debug)]
struct SingleGen<T>(T);

struct SGen<T>(T); // 泛型类型 `SGen`。
fn generic<T>(_s: SGen<T>) {}

fn main() {
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _chat = SingleGen('a');
    println!("{:#?}", _t);

    // 泛型函数
    generic(SGen('c'));
}
