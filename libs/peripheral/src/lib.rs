#![no_std]
#![allow(dead_code, non_upper_case_globals)]
#![deny(warnings)]
use core::ptr;
pub mod gpio;
pub mod tsc;
pub mod crc;
pub mod flash;
pub mod rcc;
pub mod dma;
pub mod tims;
pub mod usart;
pub mod spi;
pub mod exti;
pub mod comp;
pub mod pwr;
pub mod can;
pub mod usb_fs;
pub mod i2c;
pub mod iwdg;
pub mod wwdg;
pub mod rtc;
pub mod dac;
pub mod nvic;
pub mod fpu;
pub mod dbgmcu;
pub mod adc;
pub mod syscfg;
pub mod opamp;

#[macro_use] extern crate f3;

pub fn read(address: u32, offset: u8, width: u8) -> u32 {
	let offset = offset as u32;
	let width = width as u32;
	// TODO: Deal with boundaries
	debug_assert!(offset % 8 + width < 8);
	let value = unsafe { ptr::read_volatile((address + (offset / 8)) as *mut u32) };
	let mask = (1 << width) - 1;
	(value >> (offset % 8)) & mask
}

pub fn write(address: u32, offset: u8, width: u8, value: u32) {
	let offset = offset as u32;
	let width = width as u32;
	// TODO: Deal with boundaries
	debug_assert!(offset % 8 + width < 8);
	let value = value & (1 << (width - 1));
	let value = value << (offset % 8);
	iprintln!("Writing 0b{:0b} to address 0x{:08X}", value, (address + offset / 8));
	unsafe { ptr::write_volatile((address + offset / 8) as *mut u32, value) };
}
