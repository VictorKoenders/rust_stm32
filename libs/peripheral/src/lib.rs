#![no_std]
#![deny(warnings)]
#[derive(Copy, Clone)]
pub enum ModerType {
    InputMode = 0b00,
    OutputMode = 0b01,
    AlternateMode = 0b10,
    AnalogMode = 0b11,
}
impl ::core::convert::From<u32> for ModerType {
    fn from(val: u32) -> ModerType {
        match val {
            0b00 => ModerType::InputMode,
            0b01 => ModerType::OutputMode,
            0b10 => ModerType::AlternateMode,
            0b11 => ModerType::AnalogMode,
            x => panic!("ModerType::From out of range: {}", x)
        }
    }
}

#[no_mangle]
#[export_name = "__aeabi_memcpy"]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    return dest
}

#[no_mangle]
#[export_name = "__aeabi_memclr4"]
pub unsafe extern fn memclr4(dest: *mut u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = 0;
        i += 1;
    }
    return dest
}
/// STM32F30x
pub mod gpio;
pub mod dma;
pub mod tims;
pub mod usart;
pub mod spi;
pub mod i2c;
pub mod adc;
pub mod tsc;
pub mod crc;
pub mod flash;
pub mod rcc;
pub mod exti;
pub mod comp;
pub mod pwr;
pub mod can;
pub mod usb_fs;
pub mod iwdg;
pub mod wwdg;
pub mod rtc;
pub mod dac;
pub mod nvic;
pub mod fpu;
pub mod dbgmcu;
pub mod syscfg;
pub mod opamp;
