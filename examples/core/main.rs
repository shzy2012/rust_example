// https://doc.rust-lang.org/std/cell/struct.RefCell.html
// A mutable memory location with dynamically checked borrow rules
use std::cell::RefCell;

fn main() {
    let c = RefCell::new(5);
    let five = c.into_inner();
}
