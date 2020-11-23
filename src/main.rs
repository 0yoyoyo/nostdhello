#![feature(start)]
#![no_std]

#[no_mangle]
fn __libc_csu_fini() {}

#[no_mangle]
fn __libc_csu_init() {}

#[no_mangle]
fn __libc_start_main(f: fn() -> isize) {
    f();
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    0
}
