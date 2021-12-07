// 可变参数接口可以接受任意数目的参数。比如说 println 就可以，其参数的数目是由 格式化字符串指定的。


macro_rules! calculate {
    // 单个 `eval` 的模式
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    // 递归地拆解多重的 `eval`
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

fn main() {
    //可变参数的 `calculate!`！
    calculate! { 
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
