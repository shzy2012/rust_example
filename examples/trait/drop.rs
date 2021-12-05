// Drop trait 只有一个方法：drop，当对象离开作用域时会自动调用该方法。
// Drop trait 的主要作用是释放实现者的实例拥有的资源。

// Box，Vec，String，File，以及 Process 是一些实现了 Drop trait 来释放资源的类型。
// Drop trait 也可以为任何自定义数据类型手动实现。

struct Droppable{
    name: &'static str,
}

impl Drop for Droppable{
    fn drop(&mut self){
     println!("> Dropping {}",self.name);
    }
}

fn main(){
    let _a =Droppable{name:"a"};

    // 代码块 A
    {
        let _b = Droppable{name:"b"};

        // 代码块 B
        {
            let _c = Droppable{name:"c"};
            let _d = Droppable{name:"d"};

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }

    println!("Just exited block A");
    drop(_a);
    println!("end of the main function");
    // `_a` *不会*在这里再次销毁，因为它已经被（手动）销毁
}