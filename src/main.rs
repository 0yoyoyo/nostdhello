#![feature(start, asm)]
#![no_std]

#[no_mangle]
fn __libc_csu_fini() {}

#[no_mangle]
fn __libc_csu_init() {}

#[no_mangle]
fn __libc_start_main(main: fn() -> isize) {
    main();
    unsafe {
        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") 0,
            out("rcx") _,
            out("r11") _,
        );
    }
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[start]
fn rust_main(_argc: isize, _argv: *const *const u8) -> isize {
    let buf = "Hello, world!\n";
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") 1,
            in("rdi") 1,
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
        );
    }
    ret
}
