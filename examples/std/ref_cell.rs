// https://doc.rust-lang.org/std/cell/struct.RefCell.html
// A mutable memory location with dynamically checked borrow rules
// https://course.rs/advance/smart-pointer/cell-refcell.html
/*

我们可以将所有权、借用规则与这些智能指针做一个对比：

Rust 规则	                    |  智能指针带来的额外规则
一个数据只有一个所有者	           |     Rc/Arc让一个数据可以拥有多个所有者
要么多个不可变借用，要么一个可变借用  |	 RefCell实现编译期可变、不可变引用共存
违背规则导致编译错误	           | 违背规则导致运行时panic
*/

// 当值不可变时，可能会有多个不可变的引用指向它，此时若将修改其中一个为可变的，会造成可变引用与不可变引用共存的情况；
// 而当值可变时，最多只会有一个可变引用指向它，将其修改为不可变，那么最终依然是只有一个不可变的引用指向它。

/*
RefCell<T> 在运行时记录借用
当创建不可变和可变引用时，我们分别使用 & 和 &mut 语法。对于 RefCell<T> 来说，则是 borrow 和 borrow_mut 方法，这属于 RefCell<T> 安全 API 的一部分。borrow 方法返回 Ref<T> 类型的智能指针，borrow_mut 方法返回 RefMut<T> 类型的智能指针。这两个类型都实现了 Deref，所以可以当作常规引用对待。
RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针。每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。当 Ref<T> 值离开作用域时，不可变借用计数减一。就像编译时借用规则一样，RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用。
*/

use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: String);
}

pub struct MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        // 一个值可以在其方法内部被修改，同时对于其它代码不可变
        self.msg_cache.borrow_mut().push(msg)
    }
}

fn main() {
    let c = RefCell::new(5);
    let five = c.into_inner();
    println!("{five}");

    let mq = MsgQueue {
        msg_cache: RefCell::new(Vec::new()), //内部可变
    };
    mq.send("hello, world".to_string());

    // 违背规则导致运行时panic
    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    let s2 = s.borrow_mut();

    println!("{},{}", s1, s2);
}
