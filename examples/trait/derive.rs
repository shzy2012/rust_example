// 派生
// 通过 #[derive] 属性,编译器能够提供某些trait的基础实现。
// 如果需要更复杂的行为，这些 trait 也可以手动实现。
/* 下面是可以自动派生的 trait：
比较 trait: Eq, PartialEq, Ord, PartialOrd
Clone, 用来从 &T 创建副本 T。
Copy，使类型具有 “复制语义”（copy semantics）而非 “移动语义”（move semantics）。
Hash，从 &T 计算哈希值（hash）。
Default, 创建数据类型的一个空实例。
Debug，使用 {:?} formatter 来格式化一个值。
*/ 

// `Centimeters`，比较元组结构体
#[derive(PartialEq,PartialOrd)]
struct Centimeters(f64);

// `Inches`，打印元组结构体
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters{
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`，不带附加属性的元组结构体
struct Seconds(i32);

fn main(){
    let _one_second = Seconds(1);
    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters()<meter{
                "smaller"
            }else{
                "bigger"
            };

    println!("One foot is {} than one meter.", cmp);
}