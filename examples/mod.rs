// Rust 提供了一套强大的模块（module）系统，可以将代码按层次分成多个逻辑 单元（模块），
// 并管理这些模块之间的可见性（公有（public）或私有（private））
// 模块是项（item）的集合，项可以是：函数，结构体，trait，impl 块，甚至其它模块。

// 可见性
// 默认对外不可见，通过pub实现对外可见
// https://rustwiki.org/zh-CN/rust-by-example/mod/visibility.html

fn main() {
    // 模块机制消除了相同名字的项之间的歧义。
    function();
    my_mod::function();

    my_mod::function();
    my_mod::nested::function();
}

fn function() {
    println!("called `function()`");
}

mod my_mod {
    // 默认私有可见
    fn func1() {
        println!("called  private function:func1()");
    }

    // 对外可见
    pub fn function() {
        nested::public_function_in_super_mod();
        nested::publib_function_in_my_mod();
        println!("called  private function:function()");
    }

    // 嵌套mod
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
        pub(in crate::my_mod) fn publib_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
            public_function_in_nested()
        }

        // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested");
        }

        // 使用 `pub(super)` 语法定义的函数只在父模块中可见
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }
}
