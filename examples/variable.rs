// 变量的生命周期在一个作用域中
// 同名变量,如果在内部嵌套作用域会屏蔽外层的同名变量

fn main(){
      // 此绑定生存于 main 函数中
      let long_lived_binding = 1;

      // 嵌套作用域
      {
        // 此绑定*遮蔽*了外面的绑定
        let long_lived_binding = 5_f32;
        println!("inner long: {}",long_lived_binding) //输出: inner long: 5
      }

      println!("outer long: {}", long_lived_binding); //输出: outer long: 1
}