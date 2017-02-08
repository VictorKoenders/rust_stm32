use core::fmt::{Arguments, Error, Write};
use f3::peripheral;
use address;

const ITM_OFFSET: u32 = 0xE000_0000;
const APB2_CLOCK: u32 = 8_000_000;

pub fn init(){
    unsafe { ::f3::itm::init() };
    
    let gpioa = unsafe { peripheral::gpioa_mut() };

    let rcc = unsafe { peripheral::rcc_mut() };
    let usart2 = unsafe { peripheral::usart2_mut() };

    // RCC: Enable USART2 and GPIOC
    rcc.apb1enr.modify(|_, w| w.usart2en(true));
    rcc.ahbenr.modify(|_, w| w.iopaen(true));

    // GPIO: configure PA9 as TX and PA10 as RX
    // AFRH9: USART2_TX
    // AFRH10: USART2_RX
    gpioa.afrl.modify(|_, w| w.afrl2(7).afrl3(7));
    // MODER9: Alternate mode
    // MODER10: Alternate mode
    gpioa.moder.modify(|_, w| w.moder2(0b10).moder3(0b10));

    // USART2: 115200 - 8N1
    usart2.cr2.write(|w| w.stop(0b00));

    // Disable hardware flow control
    usart2.cr3.write(|w| w.rtse(false).ctse(false));

    const BAUD_RATE: u32 = 115200;
    let brr = (APB2_CLOCK / BAUD_RATE) as u16;
    usart2.brr.write(|w| {
        w.div_fraction((brr & 0b1111) as u8)
            .div_mantissa(brr >> 4)
    });

    // UE: Enable USART
    // RE: Enable the receiver
    // TE: Enable the transmitter
    // PCE: No parity
    // OVER8: Oversampling by 16 -- to set the baud rate
    usart2.cr1.write(|w| {
        w.ue(true)
            .re(true)
            .te(true)
            .pce(false)
            .over8(false)
            .rtoie(true)
            .txeie(true)
            .tcie(true)
            .idleie(true)
    });
}

#[allow(dead_code)]
pub fn write_itm_string(s: &str) {
    let mut log = Itm;
    log.write_str(s).ok();
}

#[allow(dead_code)]
pub fn write_itm_arguments(a: Arguments) {
    let mut log = Itm;
    log.write_fmt(a).ok();
}

struct Itm;

impl Itm {
    fn is_fifo_ready() -> bool {
        address::read_u32(ITM_OFFSET) == 1
    }
}

impl Write for Itm {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        
        for byte in s.bytes() {
            while !Itm::is_fifo_ready() {}
            address::write_u8(ITM_OFFSET, byte);
        }
        Ok(())
    }
}

/// Macro for sending `print!`-formatted messages to the ITM (Instrumentation
/// Trace Macrocell).
#[macro_export]
macro_rules! iprint {
    ($s:expr) => {
        $crate::log::write_itm_string($s)
    };
    ($($arg:tt)*) => {
        $crate::log::write_itm_arguments(format_args!($($arg)*))
    };
}

struct Usart;

impl Write for Usart {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        let usart2 = unsafe { peripheral::usart2_mut() };
        
        for byte in s.bytes() {
            while !usart2.isr.read().txe() {}
            usart2.tdr.write(|w| w.tdr(u16::from(byte)));
        }
        Ok(())
    }
}

#[allow(dead_code)]
pub fn write_usart_string(s: &str) {
    let mut usart = Usart;
    usart.write_str(s).ok();
}

#[allow(dead_code)]
pub fn write_usart_arguments(a: Arguments) {
    let mut usart = Usart;
    usart.write_fmt(a).ok();
}


/// Macro for sending `print!`-formatted messages to the ITM, with a newline
#[macro_export]
macro_rules! iprintln {
    ($fmt:expr) => {
        iprint!(concat!($fmt, "\r\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        iprint!(concat!($fmt, "\r\n"), $($arg)*)
    };
}

/// Macro for sending `print!`-formatted messages over the Serial Port
#[macro_export]
macro_rules! uprint {
    ($s:expr) => {
        $crate::log::write_usart_string($s)
    };
    ($($arg:tt)*) => {
        $crate::log::write_usart_arguments(format_args!($($arg)*))
    };
}

/// Macro for sending `print!`-formatted messages over the Serial Port, with a
/// newline
#[macro_export]
macro_rules! uprintln {
    ($fmt:expr) => {
        uprint!(concat!($fmt, "\r\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        uprint!(concat!($fmt, "\r\n"), $($arg)*)
    };
}
