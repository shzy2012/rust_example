/*
crate是Rust的编译单元。当调用rustcsome_file.rs时，some_file.rs被当作crate文件。如果some_file.rs里面含有mod声明，那么模块文件的内容将在编译之前被插入crate文件的相应声明处。换句话说，模块不会单独被编译，只有crate才会被编译。
crate可以编译成二进制可执行文件（binary）或库文件（library）。默认情况下，rustc将从crate产生二进制可执行文件。这种行为可以通过rustc的选项--crate-type重载。
*/

fn main(){
  
}