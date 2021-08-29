#![no_std]
#![no_main]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use core::fmt::{self, Write};

struct Stdout;
// write!() és writeln!() kiterjesztése
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let ret = unsafe { libc::write(libc::STDOUT_FILENO, s.as_ptr() as *const _, s.len()) };
        if ret == s.len() as isize {
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let _ = write!(&mut Stdout, "Hello world!  ");
    let _ = writeln!(&mut Stdout, "Hello world!");
    0
}
