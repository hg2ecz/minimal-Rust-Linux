#![no_std]
#![no_main]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // printf use null terminated string
    let hello1 = "Hello world!\n\0";
    unsafe {
        libc::printf(hello1.as_ptr() as *const _);
    }

    // write use array of char and length
    let hello2 = "Hello world!\n";
    unsafe {
        libc::write(1, hello2.as_ptr() as *const _, hello2.len());
    }
    0
}
