fn main() {
    let mut i = 0u32;

    while i < 101 {
        i += 1;

        if i % 15 == 0 {
            println!("FizzBuzz")
        } else if i % 3 == 0 {
            println!("Fizz")
        } else if i % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", i)
        }
    }
}
