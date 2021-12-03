// trait 未知类型的方法集合。该类型也可以访问同一个trait中定义的其他方法。
// 对任何数据类型都可以实现 trait

/* Self:
所有的trait都定义了一个隐式的类型Self,它指当前实现此接口的类型

self
当self用作函数的第一个参数时，
    self 参数等价于 self:Self
    &self 参数等价于 self:&Self
    &mut self 参数等价于 self:&mut Self

Self:方法参数中的Self是一种语法糖，是方法的接收类型,
它可能出现在trait或impl中。但经常出现在trait中，它是任何最终实现trait的类型代替
*/

struct Sheep{
    naked:bool,
    name: &'static str
}

trait Animal{

    // 静态方法签名；`Self` 表示实现者类型（implementor type）
    fn new(name: &'static str) -> Self;

    // 实例方法签名；这些方法将返回一个字符串
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // trait 提供默认的方法定义
    fn talk(&self){
        println!("{} say {}",self.name(),self.noise());
    }
}

impl Sheep{
    fn is_naked(&self) -> bool{
        self.naked
    }

    fn shear(&mut self){
        if self.is_naked(){
            // 实现者可以使用它的 trait 方法
            println!("{} is already naked...",self.name());
        }else{
            println!("{} gets a haircut!",self.name);
            self.naked=true;
        }
    }
}

// 对 `Sheep` 实现 `Animal` trait
impl Animal for Sheep{
    
    fn new(name:&'static str)->Sheep{
        Sheep{name:name,naked:false}
    }

    fn name(&self) -> &'static str{
        self.name
    }

    fn noise(&self) -> &'static str{
        if self.is_naked(){
            "hahhahhahha?"
        }else{
            "hahhahahahh!"
        }
    }

    // 默认 trait 方法可以重载
    fn talk(&self){
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main(){
    let mut dolly:Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
}