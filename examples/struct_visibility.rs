// 结构体默认不可见
// 通过pub使之可见

fn main() {
    // 带有公有字段的公有结构体，可以像平常一样构造
    let open_box = my::OpenBox {
        contents: "public infomation",
    };
    println!("The open box contains: {}", open_box.contents);

    // 私有字段的结构体可以使用公有的构造器来创建。
    let _close_box = my::ClosedBox::new("classified information");
    // 私有字段不可见
    // println!("The closed box contains: {}", _closed_box.contains);
}

mod my {
    // 一个pub结构体,带有一个pub字段,类型为泛型 T
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // 一个带私有字段的结构体,类型为泛型 T
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // 公有的构造器方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}
