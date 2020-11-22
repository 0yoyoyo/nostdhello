#![feature(start)]
#![no_std]

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    0
}
