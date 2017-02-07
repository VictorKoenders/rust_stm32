use address;
use core::fmt::{Arguments, Error, Write};

const ITM_OFFSET: u32 = 0xE000_0000;

pub fn init(){
    unsafe { ::f3::itm::init() };
    // let dbgmcu = peripheral::dbgmcu_mut();
    // let dcb = cortex_m::peripheral::dcb_mut();
    // let itm = cortex_m::peripheral::itm_mut();

    // // DBGMCU: enable asynchronous tracing
    // // NOTE(0b00) asynchronous mode
    // dbgmcu.cr.modify(|_, w| w.trace_ioen(true).trace_mode(0b00));

    // // DCB: enable the ITM
    // let demcr = dcb.demcr.read();
    // dcb.demcr.write({
    //     // Enable DWT and ITM
    //     const TRCENA: u32 = 1 << 24;

    //     demcr | TRCENA
    // });

    // // ITM: unlock the peripheral
    // const KEY: u32 = 0xc5acce55;
    // itm.lar.write(KEY);

    // // ITM: enable the whole peripheral and assign an ID
    // let tcr = itm.tcr.read();
    // itm.tcr.write({
    //     // Enables the ITM
    //     const ITMENA: u32 = 1;
    //     // The ID of the ITM port. Anything different than 0 will do
    //     const TRACE_BUS_ID: u32 = 0b1 << 16;
    //     const TRACE_BUS_ID_MASK: u32 = 0b1111111 << 16;

    //     ((tcr | ITMENA) & !TRACE_BUS_ID_MASK) | TRACE_BUS_ID
    // });

    // // ITM: enable the stimulus port 0
    // let ter = itm.ter[0].read();
    // itm.ter[0].write({
    //     // Which port
    //     const N: u32 = 0;

    //     ter | 1 << N
    // });
}

#[allow(dead_code)]
pub fn write_string(s: &str) {
    let mut log = Log;
    log.write_str(s).ok();
}

#[allow(dead_code)]
pub fn write_arguments(a: Arguments) {
    let mut log = Log;
    log.write_fmt(a).ok();
}

struct Log;

impl Log {
    fn is_fifo_ready() -> bool {
        address::read_u32(ITM_OFFSET) == 1
    }
}

impl Write for Log {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        
        for byte in s.bytes() {
            while !Log::is_fifo_ready() {}
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
        $crate::log::write_string($s)
    };
    ($($arg:tt)*) => {
        $crate::log::write_arguments(format_args!($($arg)*))
    };
}

/// Macro for sending `print!`-formatted messages to the ITM, with a newline
#[macro_export]
macro_rules! iprintln {
    ($fmt:expr) => {
        iprint!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        iprint!(concat!($fmt, "\n"), $($arg)*)
    };
}
/*
/// Macro for sending `print!`-formatted messages over the Serial Port
#[macro_export]
macro_rules! uprint {
    ($s:expr) => {
        $crate::serial::write_str($s)
    };
    ($($arg:tt)*) => {
        $crate::serial::write_fmt(format_args!($($arg)*))
    };
}

/// Macro for sending `print!`-formatted messages over the Serial Port, with a
/// newline
#[macro_export]
macro_rules! uprintln {
    ($fmt:expr) => {
        uprint!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        uprint!(concat!($fmt, "\n"), $($arg)*)
    };
}
*/