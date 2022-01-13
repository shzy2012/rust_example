use std::sync::Arc;
use std::thread;

// https://doc.rust-lang.org/rust-by-example/std/arc.html
/**
 * 当需要线程之间共享所有权时，Arc可以使用（Atomic Reference Counted）。
 * 这个结构，通过Clone实现可以为内存堆中的值的位置创建一个引用指针，同时增加引用计数器。
 * 由于它在线程之间共享所有权，因此当指向某个值的最后一个引用指针超出范围时，该变量将被删除。
 * **/ 
 use std::sync::Arc;
 use std::thread;
 
 fn main() {
     // This variable declaration is where its value is specified.
     let apple = Arc::new("the same apple");
 
     for _ in 0..10 {
         // Here there is no value specification as it is a pointer to a reference
         // in the memory heap.
         let apple = Arc::clone(&apple);
 
         thread::spawn(move || {
             // As Arc was used, threads can be spawned using the value allocated
             // in the Arc variable pointer's location.
             println!("{:?}", apple);
         });
     }
 }
 
 