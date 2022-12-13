// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/raw-pointers.html
// 不可变裸指针 *const T
// 可变裸指针  *mut T

fn main() {
    let x = 5;
    let raw = &x as *const i32;
    println!("raw address at {:?}", raw);

    let points_at = unsafe { *raw };
    println!("raw points at {}", points_at);

    let mut y = 10;
    let raw_mut = &mut y as *mut i32;

    // ---------------------------------//

    // Explicit cast:
    let i: u32 = 1;
    let p_imm: *const u32 = &i as *const u32;

    // Implicit coercion:
    let mut m: u32 = 2;
    let p_mut: *mut u32 = &mut m;
    unsafe {
        let ref_imm: &u32 = &*p_imm;
        let ref_mut: &mut u32 = &mut *p_mut;

        println!("{:?},{:?}", ref_imm, ref_mut)
    }
}
