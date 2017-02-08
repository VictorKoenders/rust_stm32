#![feature(asm)]
#![feature(lang_items)]

//#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use(bkpt)] extern crate f3;
pub extern crate stm32f30x_memory_map as peripheral;

pub use core::iter;

//mod peripheral;
#[macro_use] mod log;
mod address;
mod delay;
mod gpio;
mod rcc;
mod usb;

enum Direction {
    Clockwise,
    CounterClockwise
}

impl core::ops::Not for Direction {
    type Output = Direction;
    fn not(self) -> Self::Output {
        match self {
            Direction::Clockwise => Direction::CounterClockwise,
            Direction::CounterClockwise => Direction::Clockwise
        }
    }
}

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let mut current_led_index = 9;
    let mut previous_led_index = 8;
    let mut was_high = false;
    let mut direction = Direction::Clockwise;

    let interval = 100;
    loop {
        gpio::e::set(current_led_index);
        gpio::e::clear(previous_led_index);

        delay::ms(interval);

        previous_led_index = current_led_index;
        match direction {
            Direction::Clockwise => current_led_index += 1,
            Direction::CounterClockwise => current_led_index -= 1,
        };

        let is_high = gpio::b::is_high(1);
        if is_high && !was_high{
            direction = !direction;
        }
        was_high = is_high;

        if current_led_index == 16 {
            current_led_index = 8;
        }
        if current_led_index == 7 {
            current_led_index = 15;
        }
    }
}


#[doc(hidden)]
#[export_name = "_init"]
pub unsafe fn init() {
    delay::init();
    log::init();

    let mut sides = rcc::Side::read();
    sides.e = true;
    sides.b = true;
    sides.write();

    for i in 8..16 {
        gpio::e::configure_pin_as_output(i);
    }
    gpio::b::configure_pin_as_output(1);
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
    log::write_itm_arguments(msg);
    log::write_itm_string("\n");
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


