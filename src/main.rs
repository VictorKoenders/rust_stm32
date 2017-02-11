#![feature(asm)]
#![feature(lang_items)]

#![no_main]
#![no_std]

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    loop {}
}

#[export_name = "_init"]
pub unsafe fn init() {
}

#[export_name = "_default_exception_handler"]
pub unsafe extern "C" fn exception_handler() {
    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32
) -> ! {
    loop {}
}

pub type Handler = fn();

#[export_name = "_EXCEPTIONS"]
pub static EXCEPTIONS: [Option<Handler>; 14] = [
    None, // Some(_nmi),
    None, // Some(_hard_fault),
    None, // Some(_memmanage_fault),
    None, // Some(_bus_fault),
    None, // Some(_usage_fault),
    None,
    None,
    None,
    None,
    None, // Some(_svcall),
    None,
    None,
    None, // Some(_pendsv),
    None, // Some(_systick)
];

// For more info, see f3/src/interrupts.rs
// TODO: Find documentation on interrupts
// TODO: Add information on the interrupts
// TODO: Figure out how to enable interrupts
#[doc(hidden)]
#[export_name = "_INTERRUPTS"]
pub static INTERRUPTS: [Option<Handler>; 85] = [
    None, // Some(_wwdg),
    None, // Some(_pvd),
    None, // Some(_tamper_stamp),
    None, // Some(_rtc_wkup),
    None, // Some(_flash),
    None, // Some(_rcc),
    None, // Some(_exti0),
    None, // Some(_exti1),
    None, // Some(_exti2_ts),
    None, // Some(_exti3),
    None, // Some(_exti4),
    None, // Some(_dma1_channel1),
    None, // Some(_dma1_channel2),
    None, // Some(_dma1_channel3),
    None, // Some(_dma1_channel4),
    None, // Some(_dma1_channel5),
    None, // Some(_dma1_channel6),
    None, // Some(_dma1_channel7),
    None, // Some(_adc1_2),
    None, // Some(_usb_hp_can_tx),
    None, // Some(_usb_lp_can_rx0),
    None, // Some(_can_rx1),
    None, // Some(_can_sce),
    None, // Some(_exti9_5),
    None, // Some(_tim1_brk_tim15),
    None, // Some(_tim1_up_tim16),
    None, // Some(_tim1_trg_com_tim17),
    None, // Some(_tim1_cc),
    None, // Some(_tim2),
    None, // Some(_tim3),
    None, // Some(_tim4),
    None, // Some(_i2c1_ev),
    None, // Some(_i2c1_er),
    None, // Some(_i2c2_ev),
    None, // Some(_i2c2_er),
    None, // Some(_spi1),
    None, // Some(_spi2),
    None, // Some(_usart1),
    None, // Some(_usart2),
    None, // Some(_usart3),
    None, // Some(_exti15_10),
    None, // Some(_rtc_alarm),
    None, // Some(_usb_wake_up),
    None, // Some(_tim8_brk),
    None, // Some(_tim8_up),
    None, // Some(_tim8_trg_com),
    None, // Some(_tim8_cc),
    None, // Some(_adc3),
    None, // Some(_fmc),
    None,
    None,
    None, // Some(_spi3),
    None, // Some(_uart4),
    None, // Some(_uart5),
    None, // Some(_tim6_dac),
    None, // Some(_tim7),
    None, // Some(_dma2_channel1),
    None, // Some(_dma2_channel2),
    None, // Some(_dma2_channel3),
    None, // Some(_dma2_channel4),
    None, // Some(_dma2_channel5),
    None, // Some(_adc4),
    None,
    None,
    None, // Some(_comp1_2_3),
    None, // Some(_comp4_5_6),
    None, // Some(_comp7),
    None,
    None,
    None,
    None,
    None,
    None, // Some(_i2c3_ev),
    None, // Some(_i2c3_er),
    None, // Some(_usb_hp),
    None, // Some(_usb_lp),
    None, // Some(_usb_wake_up_rmp),
    None, // Some(_tim20_brk),
    None, // Some(_tim20_up),
    None, // Some(_tim20_trg_com),
    None, // Some(_tim20_cc),
    None, // Some(_fpu),
    None,
    None,
    None // Some(_spi4)
];

/*#![feature(asm)]
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


*/