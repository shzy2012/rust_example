/*
#[panic_handler] is used to define the behavior of panic! in #![no_std] applications.
The #[panic_handler] attribute must be applied to a function with signature fn(&PanicInfo) -> ! and such function must appear once in the dependency graph of a binary / dylib / cdylib crate.
*/

#![no_std]
use core::fmt::{self, Write};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
