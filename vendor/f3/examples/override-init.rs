//! Override the initialization routine that runs before `main`
//!
//! The `default-init` Cargo feature must be disabled for this to work.

#![feature(asm)]
#![no_main]
#![no_std]

#[macro_use]
extern crate f3;

#[export_name = "_init"]
pub fn init() {
    unsafe {
        // You'll hit this breakpoint first and ...
        bkpt!();
    }
}

#[export_name = "main"]
pub fn main() -> ! {
    unsafe {
        // ... then you'll reach this breakpoint.
        bkpt!();
    }

    loop {}
}
