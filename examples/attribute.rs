/*
属性是应用于某些模块、crate 或项的元数据（metadata）。这元数据可以用来：
    *条件编译代码
    *设置 crate 名称、版本和类型（二进制文件或库）
    *禁用 lint （警告）
    *启用编译器的特性（宏、全局导入（glob import）等）
    *链接到一个非 Rust 语言的库
    *标记函数作为单元测试
    *标记函数作为基准测试的某个部分

当属性作用于整个crate时，语法为 #![crate_attribute]，
当它们用于模块或项时，   语法为 #[item_attribute]（少了感叹号 !）


属性可以接受参数，有不同的语法形式：
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]

属性可以多个值，它们可以分开到多行中：
#[attribute(value, value2)]
#[attribute(value, value2, value3,value4, value5)]
 * /
 * 案例
 * 死代码 dead_code
 * #[allow(dead_code)]
 * fn unused_function(){}
 *
 *

//  提交编译 cfg

条件编译可能通过两种不同的操作符实现：
cfg 属性：在属性位置中使用 #[cfg(...)]
cfg! 宏：在布尔表达式中使用 cfg!(...)
两种形式使用的参数语法都相同。
*/

// 这个函数仅当目标系统是 Linux 的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}

// target_os 是由 rustc 隐式地提供的，但是自定义条件必须使用 --cfg 标记来传给 rustc。
//
