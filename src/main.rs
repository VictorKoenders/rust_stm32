#![feature(asm)]
#![feature(lang_items)]

#![no_main]
#![no_std]

extern crate peripheral;

mod timer;

/*#[macro_use] extern crate f3;
//extern crate cortex_m;
mod gpio;*/

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
pub extern "C" fn default_handler(_sf: &StackFrame) -> ! {
    //let exception = f3::exception::Exception::current();
    //iprintln!("EXCEPTION {:?} @ PC=0x{:08x}", exception, sf.pc);

    bkpt!();

    loop {}
}
#[doc(hidden)]
pub fn default_handler_no_stack() {
    bkpt!();

    loop {}
}

pub enum Direction {
    Clockwise,
    CounterClockwise
}

impl Direction {
    pub fn reverse(self) -> Direction {
        match self {
            Direction::Clockwise => Direction::CounterClockwise,
            Direction::CounterClockwise => Direction::Clockwise
        }
    }

    pub fn apply(&self, value: u8, min: u8, max: u8) -> u8 {
        match *self {
            Direction::Clockwise => if value == max { min } else { value + 1 },
            Direction::CounterClockwise => if value == min { max } else { value - 1 }
        }
    }

    pub fn negative_offset(&self, value: u8, offset: u8, min: u8, max: u8) -> u8 {
        match *self {
            Direction::Clockwise => {
                if min + offset > value {
                    (max + 1) - offset + (value - min)
                } else {
                    value - offset
                }
            },
            Direction::CounterClockwise => {
                if max - offset < value {
                    min + offset - ((max + 1) - value)
                } else {
                    value + offset
                }
            }
        }
    }
}

#[inline(never)]
#[no_mangle]
#[export_name = "_main"]
pub fn main() -> ! {
    let timer = timer::Timer::new(timer::TimerSlot::TIM7);
    // enable LEDs
    let mut ahbenr = peripheral::rcc::ahbenr::modify();
    ahbenr.iopaen = true;
    ahbenr.iopeen = true;
    ahbenr.iopden = true;
    ahbenr.save();

    let mut moder = peripheral::gpio::a::moder::modify();
    moder.moder0 = peripheral::ModerType::InputMode;
    moder.save();

    let mut moder = peripheral::gpio::e::moder::modify();
    moder.moder8 = peripheral::ModerType::OutputMode;
    moder.moder9 = peripheral::ModerType::OutputMode;
    moder.moder10 = peripheral::ModerType::OutputMode;
    moder.moder11 = peripheral::ModerType::OutputMode;
    moder.moder12 = peripheral::ModerType::OutputMode;
    moder.moder13 = peripheral::ModerType::OutputMode;
    moder.moder14 = peripheral::ModerType::OutputMode;
    moder.moder15 = peripheral::ModerType::OutputMode;
    moder.save();

    let mut moder = peripheral::gpio::d::moder::modify();
    moder.moder12 = peripheral::ModerType::OutputMode;
    moder.moder13 = peripheral::ModerType::OutputMode;
    moder.moder14 = peripheral::ModerType::OutputMode;
    moder.moder15 = peripheral::ModerType::OutputMode;
    moder.save();

    let mut led_on_index = 10;

    let mut led_skip_count = u8::max_value();
    let mut motor_on_index = 13;
    let mut turn_motor_on = true;

    let mut direction = Direction::Clockwise;
    let mut was_high = false;

    loop {
        if led_skip_count > 100 {
            // set LEDs
            peripheral::gpio::e::bsrr::bs(led_on_index, true);
            let led_off_index = direction.negative_offset(led_on_index, 2, 8, 15);
            if led_off_index > 15 || led_off_index < 8 {
                bkpt!();
            }
            peripheral::gpio::e::bsrr::br(led_off_index, true);

            led_on_index = direction.apply(led_on_index, 8, 15);

            led_skip_count = 0;
        }
        led_skip_count += 1;

        if turn_motor_on {
            motor_on_index = direction.apply(motor_on_index, 12, 15);
            peripheral::gpio::d::bsrr::bs(motor_on_index, true);
        } else {
            let motor_off_index = direction.negative_offset(motor_on_index, 1, 12, 15);
            peripheral::gpio::d::bsrr::br(motor_off_index, true);
        }
        turn_motor_on = !turn_motor_on;

        let value = peripheral::gpio::a::idr::idr(0);
        if value && !was_high {
            direction = direction.reverse();
        }
        was_high = value;

        timer.sleep_ms(1);

    }
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    use core::fmt::{Result, Write};
    struct Test;
    impl Write for Test {
        fn write_str(&mut self, s: &str) -> Result {
            bkpt!();
            Ok(())
        }
    }

    Test{}.write_fmt(_msg);
    bkpt!();
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
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
