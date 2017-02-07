#![feature(asm)]
#![feature(lang_items)]

//#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate f3;
//extern crate peripheral;

pub use f3::delay;
pub use core::iter;

//mod peripheral;
#[macro_use] mod log;
mod address;
mod gpio;
mod rcc;

#[macro_export]
macro_rules! bkpt {
    () => {
        asm!("bkpt" :::: "volatile");
    };
    ($imm:expr) => {
        asm!(concat!("bkpt #", stringify!($imm)) :::: "volatile");
    };
}

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let mut current_led_index = 9;
    let mut previous_led_index = 8;

    let interval = 100;
    loop {
        gpio::e::set(current_led_index);
        gpio::e::clear(previous_led_index);

        delay::ms(interval);

        previous_led_index = current_led_index;
        current_led_index += 1;
        if current_led_index == 16 {
            current_led_index = 8;
        }
    }
}

#[doc(hidden)]
#[export_name = "_init"]
pub unsafe fn init() {
    f3::delay::init();
    log::init();

    let mut sides = rcc::Side::load();
    sides.e = true;
    sides.write();

    for i in 8..16 {
        gpio::e::configure_pin_as_output(i);
    }
}

#[doc(hidden)]
#[export_name = "_default_exception_handler"]
pub unsafe extern "C" fn exception_handler() {
    bkpt!();

    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments,
                               file: &'static str,
                               line: u32) -> ! {
    iprintln!("Panic at {} line {}: ", file, line);
    log::write_arguments(msg);
    log::write_string("\n");
    loop { unsafe { bkpt!(); } } // PANIC! Check itm output
}


// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

// This function may be needed based on the compilation target.
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}


