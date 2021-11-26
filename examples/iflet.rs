


fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

     // If you need to specify a failure, use an else:
     if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

}
