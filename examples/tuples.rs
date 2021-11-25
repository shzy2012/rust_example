/*
https://rustwiki.org/zh-CN/rust-by-example/primitives/tuples.html

元组是一个可以包含各种类型的值的组合。
元组使用括号 () 来构造（construct），而 每个元组自身又是一个类型标记为 (T1, T2, ...) 的值，其中 T1、T2 是每个元素 的类型。函数可以使用元组来返回多个值，因为元组可以拥有任意多的值。
*/

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!("{}", long_tuple.6)
}
