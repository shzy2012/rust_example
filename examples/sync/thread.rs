use std::thread;

fn main(){
     
     let builder = thread::Builder::new().name("foo".into()).stack_size(32 * 1024);
     let handler = builder.spawn(|| {
          println!("runing thread-1");
     }).unwrap();

     handler.join().unwrap();
}