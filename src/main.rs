#![feature(start, asm)]
#![no_std]

#[no_mangle]
fn __libc_csu_fini() {}

#[no_mangle]
fn __libc_csu_init() {}

#[no_mangle]
extern "C" fn __libc_start_main(
    main: extern "C" fn(isize, *const *const u8) -> isize,
    argc: isize,
    argv: *const *const u8
) {
    let ret = main(argc, argv);
    unsafe {
        asm!(
            "syscall",
            in("rax") 60, // _exit
            in("rdi") ret,
            out("rcx") _, // destroyed in kernel
            out("r11") _, // destroyed in kernel
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
            in("rax") 1, // write
            in("rdi") 1, // stdout
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
            out("rcx") _, // destroyed in kernel
            out("r11") _, // destroyed in kernel
            lateout("rax") ret,
        );
    }
    ret
}
