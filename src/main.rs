#![feature(asm)]
#![feature(lang_items)]

#![no_main]
#![no_std]

extern crate peripheral;
#[macro_use] extern crate f3;
//extern crate cortex_m;
mod gpio;

macro_rules! bkpt {
    () => {
        unsafe { asm!("bkpt" :::: "volatile") }
    };
    ($imm:expr) => {
        unsafe { asm!(concat!("bkpt #", stringify!($imm)) :::: "volatile") }
    };
}

#[repr(C)]
pub struct StackFrame {
    /// (General purpose) Register 0
    pub r0: u32,
    /// (General purpose) Register 1
    pub r1: u32,
    /// (General purpose) Register 2
    pub r2: u32,
    /// (General purpose) Register 3
    pub r3: u32,
    /// (General purpose) Register 12
    pub r12: u32,
    /// Linker Register
    pub lr: u32,
    /// Program Counter
    pub pc: u32,
    /// Program Status Register
    pub xpsr: u32,
}

#[doc(hidden)]
#[export_name = "_default_exception_handler"]
pub extern "C" fn default_handler(sf: &StackFrame) -> ! {
    let exception = f3::exception::Exception::current();
    iprintln!("EXCEPTION {:?} @ PC=0x{:08x}", exception, sf.pc);

    bkpt!();

    loop {}
}
#[doc(hidden)]
pub fn default_handler_no_stack() {
    //let exception = f3::exception::Exception::current();
    //iprintln!("EXCEPTION {:?} @ PC=0x{:08x}", exception, sf.pc);

    bkpt!();

    loop {}
}

#[inline(never)]
#[no_mangle]
#[export_name = "_main"]
pub fn main() -> ! {
    unsafe { f3::itm::init(); }
    iprintln!("Hello!");
    
    //peripheral::rcc::ahbenr::set_iopeen(1);
    //peripheral::gpio::gpioe::moder::set_moder8(1);
    unsafe { 
        let gpioe = f3::peripheral::gpioe_mut();
        let rcc = f3::peripheral::rcc_mut();

        // RCC: Enable GPIOE
        //rcc.ahbenr.modify(|_, w| w.iopeen(true));
        peripheral::rcc::ahbenr::set_iopeen(1);
        peripheral::gpio::gpioe::moder::set_moder8(1);

        // GPIOE: Configure pins 8-15 as outputs
        gpioe.moder.modify(|_, w| {
            let result = w.moder8(0b01);
            bkpt!();
            result
        });
    }
    peripheral::gpio::gpioe::bsrr::set_bs8(1);
    loop {
    }
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
/*
#[export_name = "_EXCEPTIONS"]
pub static EXCEPTIONS: [Option<Handler>; 14] = [
    Some(default_handler_no_stack), // Some(_nmi),
    Some(default_handler_no_stack), // Some(_hard_fault),
    Some(default_handler_no_stack), // Some(_memmanage_fault),
    Some(default_handler_no_stack), // Some(_bus_fault),
    Some(default_handler_no_stack), // Some(_usage_fault),
    None,
    None,
    None,
    None,
    Some(default_handler_no_stack), // Some(_svcall),
    None,
    None,
    Some(default_handler_no_stack), // Some(_pendsv),
    Some(default_handler_no_stack), // Some(_systick)
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
*/