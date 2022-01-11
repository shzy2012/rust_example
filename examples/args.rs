use std::env;

fn main() {
    for elem in env::args() {
        println!("{}", elem);
    }

    let args: Vec<String> = env::args().collect();
    for elem in args.iter() {
        println!("{}", elem);
    }

    for i in 0..args.len() {
        println!("{}", args[i])
    }
}
