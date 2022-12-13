// https://course.rs/advance/smart-pointer/cell-refcell.html
// Cell: 允许修改不可变引用,通过unsafe实现
// Cell 和 RefCell 在功能上没有区别，区别在于 Cell<T> 适用于 T 实现 Copy 的情况：
use std::cell::Cell;

fn main() {
    let c = Cell::new("asdf");
    let one = c.get();
    c.set("qwer");
    let two = c.get();
    println!("{},{}", one, two);
}
