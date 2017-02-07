#![allow(dead_code)]
use core::ptr;

pub fn read_u8(address: u32) -> u8 {
    unsafe { ptr::read_volatile(address as *mut u32 as *mut u8) }
}
pub fn read_u16(address: u32) -> u16 {
    unsafe { ptr::read_volatile(address as *mut u32 as *mut u16) }
}
pub fn read_u32(address: u32) -> u32 {
    unsafe { ptr::read_volatile(address as *mut u32) }
}

pub fn write_u8(address: u32, value: u8) {
    unsafe { ptr::write_volatile(address as *mut u32 as *mut u8, value); }
}
pub fn write_u16(address: u32, value: u16) {
    unsafe { ptr::write_volatile(address as *mut u32 as *mut u16, value); }
}
pub fn write_u32(address: u32, value: u32) {
    unsafe { ptr::write_volatile(address as *mut u32, value); }
}

pub fn set_bit_u8(address: u32, offset: u8) {
    let mut value = read_u8(address);
    value |= 1 << offset;
    write_u8(address, value);
}
pub fn set_bit_u16(address: u32, offset: u8) {
    let mut value = read_u16(address);
    value |= 1 << offset;
    write_u16(address, value);
}
pub fn set_bit_u32(address: u32, offset: u8) {
    let mut value = read_u32(address);
    value |= 1 << offset;
    write_u32(address, value);
}
