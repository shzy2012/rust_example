// Rust 没有“继承”，但是您可以将一个 trait 定义为另一个 trait 的超集（即父 trait）

trait Person{
    fn name(&self) -> String;
}

// Person 是 Student 的父 trait。
// 实现 Student 需要 impl 了 Person
trait Student: Person{
    fn university(&self) -> String;
}

trait Programmer{
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student，计算机科学的学生) 是 Programmer 和 Student 两者的子类。
// 实现 CompSciStudent 需要你同时 impl 了两个父 trait。
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {
      Student::name();
}

