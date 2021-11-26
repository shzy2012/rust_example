// 方法（method）是依附于对象的函数
// 这些方法通过关键字 self 来访问对象中的数据和 其他。方法在 impl 代码块中定义。

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

// 实现的代码块，`Point` 的所有方法都在这里给出

impl Point {
    fn origin() -> Point {
        Point { x: 0.00, y: 0.00 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

// 允许多次impl
impl Point {
    fn plugin() -> (&'static str, bool) {
        (&"A", true)
    }
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // `&self` 是 `self: &Self` 的语法糖（sugar），其中 `Self` 是方法调用者的类型
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

fn main() {
    let p1 = Point::new(12.0, 12.0);
    let p2 = Point::origin();
    println!("p1=>{:#?} \np2=>{:#?}", p1, p2);

    Point::plugin();

    let mut rectangle = Rectangle {
        // 静态方法使用双冒号调用
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    rectangle.translate(10.00, 2.00);

    println!("{:#?}", rectangle.p1);
}
