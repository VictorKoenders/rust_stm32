/// Clock control register
pub mod cr {
    /// Internal High Speed clock enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set Internal High Speed clock enable
    pub fn set_hsion(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Internal High Speed clock enable
    pub fn get_hsion() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Internal High Speed clock ready flag
    /// Access: read-only, Width: 1, Offset: 1
    /// Get Internal High Speed clock ready flag
    pub fn hsirdy() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// Internal High Speed clock trimming
    /// Access: read-write, Width: 5, Offset: 3
    /// Set Internal High Speed clock trimming
    pub fn set_hsitrim(value: u8) {
        debug_assert!(value <= 0b11111, "set_hsitrim out of range");
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Internal High Speed clock trimming
    pub fn get_hsitrim() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x0u32) as *mut u32) };
        let value = value & (0b11111 << 3);
        value as u8
    }
    /// Internal High Speed clock Calibration
    /// Access: read-only, Width: 8, Offset: 8
    /// Get Internal High Speed clock Calibration
    pub fn hsical() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x0u32) as *mut u32) };
        let value = value & (0b11111111 << 8);
        value as u8
    }
    /// External High Speed clock enable
    /// Access: read-write, Width: 1, Offset: 16
    /// Set External High Speed clock enable
    pub fn set_hseon(value: bool) {
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get External High Speed clock enable
    pub fn get_hseon() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 16);
        value > 0
    }
    /// External High Speed clock ready flag
    /// Access: read-only, Width: 1, Offset: 17
    /// Get External High Speed clock ready flag
    pub fn hserdy() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 17);
        value > 0
    }
    /// External High Speed clock Bypass
    /// Access: read-write, Width: 1, Offset: 18
    /// Set External High Speed clock Bypass
    pub fn set_hsebyp(value: bool) {
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get External High Speed clock Bypass
    pub fn get_hsebyp() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 18);
        value > 0
    }
    /// Clock Security System enable
    /// Access: read-write, Width: 1, Offset: 19
    /// Set Clock Security System enable
    pub fn set_csson(value: bool) {
        let value = value as u32;
        let value = value << 19;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Clock Security System enable
    pub fn get_csson() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 19);
        value > 0
    }
    /// PLL enable
    /// Access: read-write, Width: 1, Offset: 24
    /// Set PLL enable
    pub fn set_pllon(value: bool) {
        let value = value as u32;
        let value = value << 24;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get PLL enable
    pub fn get_pllon() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 24);
        value > 0
    }
    /// PLL clock ready flag
    /// Access: read-only, Width: 1, Offset: 25
    /// Get PLL clock ready flag
    pub fn pllrdy() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 25);
        value > 0
    }
}
/// Clock configuration register (RCC_CFGR)
pub mod cfgr {
    /// System clock Switch
    /// Access: read-write, Width: 2, Offset: 0
    /// Set System clock Switch
    pub fn set_sw(value: u8) {
        debug_assert!(value <= 0b11, "set_sw out of range");
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get System clock Switch
    pub fn get_sw() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b11 << 0);
        value as u8
    }
    /// System Clock Switch Status
    /// Access: read-only, Width: 2, Offset: 2
    /// Get System Clock Switch Status
    pub fn sws() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b11 << 2);
        value as u8
    }
    /// AHB prescaler
    /// Access: read-write, Width: 4, Offset: 4
    /// Set AHB prescaler
    pub fn set_hpre(value: u8) {
        debug_assert!(value <= 0b1111, "set_hpre out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get AHB prescaler
    pub fn get_hpre() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1111 << 4);
        value as u8
    }
    /// APB Low speed prescaler (APB1)
    /// Access: read-write, Width: 3, Offset: 8
    /// Set APB Low speed prescaler (APB1)
    pub fn set_ppre1(value: u8) {
        debug_assert!(value <= 0b111, "set_ppre1 out of range");
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get APB Low speed prescaler (APB1)
    pub fn get_ppre1() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b111 << 8);
        value as u8
    }
    /// APB high speed prescaler (APB2)
    /// Access: read-write, Width: 3, Offset: 11
    /// Set APB high speed prescaler (APB2)
    pub fn set_ppre2(value: u8) {
        debug_assert!(value <= 0b111, "set_ppre2 out of range");
        let value = value as u32;
        let value = value << 11;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get APB high speed prescaler (APB2)
    pub fn get_ppre2() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b111 << 11);
        value as u8
    }
    /// PLL entry clock source
    /// Access: read-write, Width: 1, Offset: 16
    /// Set PLL entry clock source
    pub fn set_pllsrc(value: bool) {
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get PLL entry clock source
    pub fn get_pllsrc() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 16);
        value > 0
    }
    /// HSE divider for PLL entry
    /// Access: read-write, Width: 1, Offset: 17
    /// Set HSE divider for PLL entry
    pub fn set_pllxtpre(value: bool) {
        let value = value as u32;
        let value = value << 17;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get HSE divider for PLL entry
    pub fn get_pllxtpre() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 17);
        value > 0
    }
    /// PLL Multiplication Factor
    /// Access: read-write, Width: 4, Offset: 18
    /// Set PLL Multiplication Factor
    pub fn set_pllmul(value: u8) {
        debug_assert!(value <= 0b1111, "set_pllmul out of range");
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get PLL Multiplication Factor
    pub fn get_pllmul() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1111 << 18);
        value as u8
    }
    /// USB prescaler
    /// Access: read-write, Width: 1, Offset: 22
    /// Set USB prescaler
    pub fn set_usbpres(value: bool) {
        let value = value as u32;
        let value = value << 22;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get USB prescaler
    pub fn get_usbpres() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 22);
        value > 0
    }
    /// Microcontroller clock output
    /// Access: read-write, Width: 3, Offset: 24
    /// Set Microcontroller clock output
    pub fn set_mco(value: u8) {
        debug_assert!(value <= 0b111, "set_mco out of range");
        let value = value as u32;
        let value = value << 24;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Microcontroller clock output
    pub fn get_mco() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b111 << 24);
        value as u8
    }
    /// Microcontroller Clock Output Flag
    /// Access: read-only, Width: 1, Offset: 28
    /// Get Microcontroller Clock Output Flag
    pub fn mcof() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 28);
        value > 0
    }
    /// I2S external clock source selection
    /// Access: read-write, Width: 1, Offset: 23
    /// Set I2S external clock source selection
    pub fn set_i2ssrc(value: bool) {
        let value = value as u32;
        let value = value << 23;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get I2S external clock source selection
    pub fn get_i2ssrc() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 23);
        value > 0
    }
}
/// Clock interrupt register (RCC_CIR)
pub mod cir {
    /// LSI Ready Interrupt flag
    /// Access: read-only, Width: 1, Offset: 0
    /// Get LSI Ready Interrupt flag
    pub fn lsirdyf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// LSE Ready Interrupt flag
    /// Access: read-only, Width: 1, Offset: 1
    /// Get LSE Ready Interrupt flag
    pub fn lserdyf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// HSI Ready Interrupt flag
    /// Access: read-only, Width: 1, Offset: 2
    /// Get HSI Ready Interrupt flag
    pub fn hsirdyf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// HSE Ready Interrupt flag
    /// Access: read-only, Width: 1, Offset: 3
    /// Get HSE Ready Interrupt flag
    pub fn hserdyf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// PLL Ready Interrupt flag
    /// Access: read-only, Width: 1, Offset: 4
    /// Get PLL Ready Interrupt flag
    pub fn pllrdyf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// Clock Security System Interrupt flag
    /// Access: read-only, Width: 1, Offset: 7
    /// Get Clock Security System Interrupt flag
    pub fn cssf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// LSI Ready Interrupt Enable
    /// Access: read-write, Width: 1, Offset: 8
    /// Set LSI Ready Interrupt Enable
    pub fn set_lsirdyie(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get LSI Ready Interrupt Enable
    pub fn get_lsirdyie() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// LSE Ready Interrupt Enable
    /// Access: read-write, Width: 1, Offset: 9
    /// Set LSE Ready Interrupt Enable
    pub fn set_lserdyie(value: bool) {
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get LSE Ready Interrupt Enable
    pub fn get_lserdyie() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// HSI Ready Interrupt Enable
    /// Access: read-write, Width: 1, Offset: 10
    /// Set HSI Ready Interrupt Enable
    pub fn set_hsirdyie(value: bool) {
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get HSI Ready Interrupt Enable
    pub fn get_hsirdyie() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 10);
        value > 0
    }
    /// HSE Ready Interrupt Enable
    /// Access: read-write, Width: 1, Offset: 11
    /// Set HSE Ready Interrupt Enable
    pub fn set_hserdyie(value: bool) {
        let value = value as u32;
        let value = value << 11;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get HSE Ready Interrupt Enable
    pub fn get_hserdyie() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// PLL Ready Interrupt Enable
    /// Access: read-write, Width: 1, Offset: 12
    /// Set PLL Ready Interrupt Enable
    pub fn set_pllrdyie(value: bool) {
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get PLL Ready Interrupt Enable
    pub fn get_pllrdyie() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 12);
        value > 0
    }
    /// LSI Ready Interrupt Clear
    /// Access: write-only, Width: 1, Offset: 16
    /// Set LSI Ready Interrupt Clear
    pub fn lsirdyc(value: bool) {
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x8u32) as *mut u32, value) };
    }
    /// LSE Ready Interrupt Clear
    /// Access: write-only, Width: 1, Offset: 17
    /// Set LSE Ready Interrupt Clear
    pub fn lserdyc(value: bool) {
        let value = value as u32;
        let value = value << 17;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x8u32) as *mut u32, value) };
    }
    /// HSI Ready Interrupt Clear
    /// Access: write-only, Width: 1, Offset: 18
    /// Set HSI Ready Interrupt Clear
    pub fn hsirdyc(value: bool) {
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x8u32) as *mut u32, value) };
    }
    /// HSE Ready Interrupt Clear
    /// Access: write-only, Width: 1, Offset: 19
    /// Set HSE Ready Interrupt Clear
    pub fn hserdyc(value: bool) {
        let value = value as u32;
        let value = value << 19;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x8u32) as *mut u32, value) };
    }
    /// PLL Ready Interrupt Clear
    /// Access: write-only, Width: 1, Offset: 20
    /// Set PLL Ready Interrupt Clear
    pub fn pllrdyc(value: bool) {
        let value = value as u32;
        let value = value << 20;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x8u32) as *mut u32, value) };
    }
    /// Clock security system interrupt clear
    /// Access: write-only, Width: 1, Offset: 23
    /// Set Clock security system interrupt clear
    pub fn cssc(value: bool) {
        let value = value as u32;
        let value = value << 23;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x8u32) as *mut u32, value) };
    }
}
/// APB2 peripheral reset register (RCC_APB2RSTR)
pub mod apb2rstr {
    pub struct ReadonlyCache {
        /// SYSCFG and COMP reset
        pub syscfgrst: bool,
        /// TIM1 timer reset
        pub tim1rst: bool,
        /// SPI 1 reset
        pub spi1rst: bool,
        /// TIM8 timer reset
        pub tim8rst: bool,
        /// USART1 reset
        pub usart1rst: bool,
        /// TIM15 timer reset
        pub tim15rst: bool,
        /// TIM16 timer reset
        pub tim16rst: bool,
        /// TIM17 timer reset
        pub tim17rst: bool,
    }
    pub struct Cache {
        /// SYSCFG and COMP reset
        pub syscfgrst: bool,
        /// TIM1 timer reset
        pub tim1rst: bool,
        /// SPI 1 reset
        pub spi1rst: bool,
        /// TIM8 timer reset
        pub tim8rst: bool,
        /// USART1 reset
        pub usart1rst: bool,
        /// TIM15 timer reset
        pub tim15rst: bool,
        /// TIM16 timer reset
        pub tim16rst: bool,
        /// TIM17 timer reset
        pub tim17rst: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            syscfgrst: ((value >> 0) & 0b1) > 0,
            tim1rst: ((value >> 11) & 0b1) > 0,
            spi1rst: ((value >> 12) & 0b1) > 0,
            tim8rst: ((value >> 13) & 0b1) > 0,
            usart1rst: ((value >> 14) & 0b1) > 0,
            tim15rst: ((value >> 16) & 0b1) > 0,
            tim16rst: ((value >> 17) & 0b1) > 0,
            tim17rst: ((value >> 18) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0xCu32) as *mut u32) };
        Cache {
            syscfgrst: ((value >> 0) & 0b1) > 0,
            tim1rst: ((value >> 11) & 0b1) > 0,
            spi1rst: ((value >> 12) & 0b1) > 0,
            tim8rst: ((value >> 13) & 0b1) > 0,
            usart1rst: ((value >> 14) & 0b1) > 0,
            tim15rst: ((value >> 16) & 0b1) > 0,
            tim16rst: ((value >> 17) & 0b1) > 0,
            tim17rst: ((value >> 18) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.syscfgrst as u32) << 0)
                | ((self.tim1rst as u32) << 11)
                | ((self.spi1rst as u32) << 12)
                | ((self.tim8rst as u32) << 13)
                | ((self.usart1rst as u32) << 14)
                | ((self.tim15rst as u32) << 16)
                | ((self.tim16rst as u32) << 17)
                | ((self.tim17rst as u32) << 18)
            ;
            unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// APB1 peripheral reset register (RCC_APB1RSTR)
pub mod apb1rstr {
    pub struct ReadonlyCache {
        /// Timer 2 reset
        pub tim2rst: bool,
        /// Timer 3 reset
        pub tim3rst: bool,
        /// Timer 14 reset
        pub tim4rst: bool,
        /// Timer 6 reset
        pub tim6rst: bool,
        /// Timer 7 reset
        pub tim7rst: bool,
        /// Window watchdog reset
        pub wwdgrst: bool,
        /// SPI2 reset
        pub spi2rst: bool,
        /// SPI3 reset
        pub spi3rst: bool,
        /// USART 2 reset
        pub usart2rst: bool,
        /// USART3 reset
        pub usart3rst: bool,
        /// UART 4 reset
        pub uart4rst: bool,
        /// UART 5 reset
        pub uart5rst: bool,
        /// I2C1 reset
        pub i2c1rst: bool,
        /// I2C2 reset
        pub i2c2rst: bool,
        /// USB reset
        pub usbrst: bool,
        /// CAN reset
        pub canrst: bool,
        /// Power interface reset
        pub pwrrst: bool,
        /// DAC interface reset
        pub dacrst: bool,
    }
    pub struct Cache {
        /// Timer 2 reset
        pub tim2rst: bool,
        /// Timer 3 reset
        pub tim3rst: bool,
        /// Timer 14 reset
        pub tim4rst: bool,
        /// Timer 6 reset
        pub tim6rst: bool,
        /// Timer 7 reset
        pub tim7rst: bool,
        /// Window watchdog reset
        pub wwdgrst: bool,
        /// SPI2 reset
        pub spi2rst: bool,
        /// SPI3 reset
        pub spi3rst: bool,
        /// USART 2 reset
        pub usart2rst: bool,
        /// USART3 reset
        pub usart3rst: bool,
        /// UART 4 reset
        pub uart4rst: bool,
        /// UART 5 reset
        pub uart5rst: bool,
        /// I2C1 reset
        pub i2c1rst: bool,
        /// I2C2 reset
        pub i2c2rst: bool,
        /// USB reset
        pub usbrst: bool,
        /// CAN reset
        pub canrst: bool,
        /// Power interface reset
        pub pwrrst: bool,
        /// DAC interface reset
        pub dacrst: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            tim2rst: ((value >> 0) & 0b1) > 0,
            tim3rst: ((value >> 1) & 0b1) > 0,
            tim4rst: ((value >> 2) & 0b1) > 0,
            tim6rst: ((value >> 4) & 0b1) > 0,
            tim7rst: ((value >> 5) & 0b1) > 0,
            wwdgrst: ((value >> 11) & 0b1) > 0,
            spi2rst: ((value >> 14) & 0b1) > 0,
            spi3rst: ((value >> 15) & 0b1) > 0,
            usart2rst: ((value >> 17) & 0b1) > 0,
            usart3rst: ((value >> 18) & 0b1) > 0,
            uart4rst: ((value >> 19) & 0b1) > 0,
            uart5rst: ((value >> 20) & 0b1) > 0,
            i2c1rst: ((value >> 21) & 0b1) > 0,
            i2c2rst: ((value >> 22) & 0b1) > 0,
            usbrst: ((value >> 23) & 0b1) > 0,
            canrst: ((value >> 25) & 0b1) > 0,
            pwrrst: ((value >> 28) & 0b1) > 0,
            dacrst: ((value >> 29) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x10u32) as *mut u32) };
        Cache {
            tim2rst: ((value >> 0) & 0b1) > 0,
            tim3rst: ((value >> 1) & 0b1) > 0,
            tim4rst: ((value >> 2) & 0b1) > 0,
            tim6rst: ((value >> 4) & 0b1) > 0,
            tim7rst: ((value >> 5) & 0b1) > 0,
            wwdgrst: ((value >> 11) & 0b1) > 0,
            spi2rst: ((value >> 14) & 0b1) > 0,
            spi3rst: ((value >> 15) & 0b1) > 0,
            usart2rst: ((value >> 17) & 0b1) > 0,
            usart3rst: ((value >> 18) & 0b1) > 0,
            uart4rst: ((value >> 19) & 0b1) > 0,
            uart5rst: ((value >> 20) & 0b1) > 0,
            i2c1rst: ((value >> 21) & 0b1) > 0,
            i2c2rst: ((value >> 22) & 0b1) > 0,
            usbrst: ((value >> 23) & 0b1) > 0,
            canrst: ((value >> 25) & 0b1) > 0,
            pwrrst: ((value >> 28) & 0b1) > 0,
            dacrst: ((value >> 29) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.tim2rst as u32) << 0)
                | ((self.tim3rst as u32) << 1)
                | ((self.tim4rst as u32) << 2)
                | ((self.tim6rst as u32) << 4)
                | ((self.tim7rst as u32) << 5)
                | ((self.wwdgrst as u32) << 11)
                | ((self.spi2rst as u32) << 14)
                | ((self.spi3rst as u32) << 15)
                | ((self.usart2rst as u32) << 17)
                | ((self.usart3rst as u32) << 18)
                | ((self.uart4rst as u32) << 19)
                | ((self.uart5rst as u32) << 20)
                | ((self.i2c1rst as u32) << 21)
                | ((self.i2c2rst as u32) << 22)
                | ((self.usbrst as u32) << 23)
                | ((self.canrst as u32) << 25)
                | ((self.pwrrst as u32) << 28)
                | ((self.dacrst as u32) << 29)
            ;
            unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x10u32) as *mut u32, value) };
        }
    }
}
/// AHB Peripheral Clock enable register (RCC_AHBENR)
pub mod ahbenr {
    pub struct ReadonlyCache {
        /// DMA1 clock enable
        pub dmaen: bool,
        /// DMA2 clock enable
        pub dma2en: bool,
        /// SRAM interface clock enable
        pub sramen: bool,
        /// FLITF clock enable
        pub flitfen: bool,
        /// CRC clock enable
        pub crcen: bool,
        /// I/O port A clock enable
        pub iopaen: bool,
        /// I/O port B clock enable
        pub iopben: bool,
        /// I/O port C clock enable
        pub iopcen: bool,
        /// I/O port D clock enable
        pub iopden: bool,
        /// I/O port E clock enable
        pub iopeen: bool,
        /// I/O port F clock enable
        pub iopfen: bool,
        /// Touch sensing controller clock enable
        pub tscen: bool,
        /// ADC1 and ADC2 clock enable
        pub adc12en: bool,
        /// ADC3 and ADC4 clock enable
        pub adc34en: bool,
    }
    pub struct Cache {
        /// DMA1 clock enable
        pub dmaen: bool,
        /// DMA2 clock enable
        pub dma2en: bool,
        /// SRAM interface clock enable
        pub sramen: bool,
        /// FLITF clock enable
        pub flitfen: bool,
        /// CRC clock enable
        pub crcen: bool,
        /// I/O port A clock enable
        pub iopaen: bool,
        /// I/O port B clock enable
        pub iopben: bool,
        /// I/O port C clock enable
        pub iopcen: bool,
        /// I/O port D clock enable
        pub iopden: bool,
        /// I/O port E clock enable
        pub iopeen: bool,
        /// I/O port F clock enable
        pub iopfen: bool,
        /// Touch sensing controller clock enable
        pub tscen: bool,
        /// ADC1 and ADC2 clock enable
        pub adc12en: bool,
        /// ADC3 and ADC4 clock enable
        pub adc34en: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x14u32) as *mut u32) };
        ReadonlyCache {
            dmaen: ((value >> 0) & 0b1) > 0,
            dma2en: ((value >> 1) & 0b1) > 0,
            sramen: ((value >> 2) & 0b1) > 0,
            flitfen: ((value >> 4) & 0b1) > 0,
            crcen: ((value >> 6) & 0b1) > 0,
            iopaen: ((value >> 17) & 0b1) > 0,
            iopben: ((value >> 18) & 0b1) > 0,
            iopcen: ((value >> 19) & 0b1) > 0,
            iopden: ((value >> 20) & 0b1) > 0,
            iopeen: ((value >> 21) & 0b1) > 0,
            iopfen: ((value >> 22) & 0b1) > 0,
            tscen: ((value >> 24) & 0b1) > 0,
            adc12en: ((value >> 28) & 0b1) > 0,
            adc34en: ((value >> 29) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x14u32) as *mut u32) };
        Cache {
            dmaen: ((value >> 0) & 0b1) > 0,
            dma2en: ((value >> 1) & 0b1) > 0,
            sramen: ((value >> 2) & 0b1) > 0,
            flitfen: ((value >> 4) & 0b1) > 0,
            crcen: ((value >> 6) & 0b1) > 0,
            iopaen: ((value >> 17) & 0b1) > 0,
            iopben: ((value >> 18) & 0b1) > 0,
            iopcen: ((value >> 19) & 0b1) > 0,
            iopden: ((value >> 20) & 0b1) > 0,
            iopeen: ((value >> 21) & 0b1) > 0,
            iopfen: ((value >> 22) & 0b1) > 0,
            tscen: ((value >> 24) & 0b1) > 0,
            adc12en: ((value >> 28) & 0b1) > 0,
            adc34en: ((value >> 29) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.dmaen as u32) << 0)
                | ((self.dma2en as u32) << 1)
                | ((self.sramen as u32) << 2)
                | ((self.flitfen as u32) << 4)
                | ((self.crcen as u32) << 6)
                | ((self.iopaen as u32) << 17)
                | ((self.iopben as u32) << 18)
                | ((self.iopcen as u32) << 19)
                | ((self.iopden as u32) << 20)
                | ((self.iopeen as u32) << 21)
                | ((self.iopfen as u32) << 22)
                | ((self.tscen as u32) << 24)
                | ((self.adc12en as u32) << 28)
                | ((self.adc34en as u32) << 29)
            ;
            unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x14u32) as *mut u32, value) };
        }
    }
}
/// APB2 peripheral clock enable register (RCC_APB2ENR)
pub mod apb2enr {
    pub struct ReadonlyCache {
        /// SYSCFG clock enable
        pub syscfgen: bool,
        /// TIM1 Timer clock enable
        pub tim1en: bool,
        /// SPI 1 clock enable
        pub spi1en: bool,
        /// TIM8 Timer clock enable
        pub tim8en: bool,
        /// USART1 clock enable
        pub usart1en: bool,
        /// TIM15 timer clock enable
        pub tim15en: bool,
        /// TIM16 timer clock enable
        pub tim16en: bool,
        /// TIM17 timer clock enable
        pub tim17en: bool,
    }
    pub struct Cache {
        /// SYSCFG clock enable
        pub syscfgen: bool,
        /// TIM1 Timer clock enable
        pub tim1en: bool,
        /// SPI 1 clock enable
        pub spi1en: bool,
        /// TIM8 Timer clock enable
        pub tim8en: bool,
        /// USART1 clock enable
        pub usart1en: bool,
        /// TIM15 timer clock enable
        pub tim15en: bool,
        /// TIM16 timer clock enable
        pub tim16en: bool,
        /// TIM17 timer clock enable
        pub tim17en: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x18u32) as *mut u32) };
        ReadonlyCache {
            syscfgen: ((value >> 0) & 0b1) > 0,
            tim1en: ((value >> 11) & 0b1) > 0,
            spi1en: ((value >> 12) & 0b1) > 0,
            tim8en: ((value >> 13) & 0b1) > 0,
            usart1en: ((value >> 14) & 0b1) > 0,
            tim15en: ((value >> 16) & 0b1) > 0,
            tim16en: ((value >> 17) & 0b1) > 0,
            tim17en: ((value >> 18) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x18u32) as *mut u32) };
        Cache {
            syscfgen: ((value >> 0) & 0b1) > 0,
            tim1en: ((value >> 11) & 0b1) > 0,
            spi1en: ((value >> 12) & 0b1) > 0,
            tim8en: ((value >> 13) & 0b1) > 0,
            usart1en: ((value >> 14) & 0b1) > 0,
            tim15en: ((value >> 16) & 0b1) > 0,
            tim16en: ((value >> 17) & 0b1) > 0,
            tim17en: ((value >> 18) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.syscfgen as u32) << 0)
                | ((self.tim1en as u32) << 11)
                | ((self.spi1en as u32) << 12)
                | ((self.tim8en as u32) << 13)
                | ((self.usart1en as u32) << 14)
                | ((self.tim15en as u32) << 16)
                | ((self.tim16en as u32) << 17)
                | ((self.tim17en as u32) << 18)
            ;
            unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x18u32) as *mut u32, value) };
        }
    }
}
/// APB1 peripheral clock enable register (RCC_APB1ENR)
pub mod apb1enr {
    pub struct ReadonlyCache {
        /// Timer 2 clock enable
        pub tim2en: bool,
        /// Timer 3 clock enable
        pub tim3en: bool,
        /// Timer 4 clock enable
        pub tim4en: bool,
        /// Timer 6 clock enable
        pub tim6en: bool,
        /// Timer 7 clock enable
        pub tim7en: bool,
        /// Window watchdog clock enable
        pub wwdgen: bool,
        /// SPI 2 clock enable
        pub spi2en: bool,
        /// SPI 3 clock enable
        pub spi3en: bool,
        /// USART 2 clock enable
        pub usart2en: bool,
        /// I2C 1 clock enable
        pub i2c1en: bool,
        /// I2C 2 clock enable
        pub i2c2en: bool,
        /// USB clock enable
        pub usben: bool,
        /// CAN clock enable
        pub canen: bool,
        /// Power interface clock enable
        pub pwren: bool,
        /// DAC interface clock enable
        pub dacen: bool,
    }
    pub struct Cache {
        /// Timer 2 clock enable
        pub tim2en: bool,
        /// Timer 3 clock enable
        pub tim3en: bool,
        /// Timer 4 clock enable
        pub tim4en: bool,
        /// Timer 6 clock enable
        pub tim6en: bool,
        /// Timer 7 clock enable
        pub tim7en: bool,
        /// Window watchdog clock enable
        pub wwdgen: bool,
        /// SPI 2 clock enable
        pub spi2en: bool,
        /// SPI 3 clock enable
        pub spi3en: bool,
        /// USART 2 clock enable
        pub usart2en: bool,
        /// I2C 1 clock enable
        pub i2c1en: bool,
        /// I2C 2 clock enable
        pub i2c2en: bool,
        /// USB clock enable
        pub usben: bool,
        /// CAN clock enable
        pub canen: bool,
        /// Power interface clock enable
        pub pwren: bool,
        /// DAC interface clock enable
        pub dacen: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x1Cu32) as *mut u32) };
        ReadonlyCache {
            tim2en: ((value >> 0) & 0b1) > 0,
            tim3en: ((value >> 1) & 0b1) > 0,
            tim4en: ((value >> 2) & 0b1) > 0,
            tim6en: ((value >> 4) & 0b1) > 0,
            tim7en: ((value >> 5) & 0b1) > 0,
            wwdgen: ((value >> 11) & 0b1) > 0,
            spi2en: ((value >> 14) & 0b1) > 0,
            spi3en: ((value >> 15) & 0b1) > 0,
            usart2en: ((value >> 17) & 0b1) > 0,
            i2c1en: ((value >> 21) & 0b1) > 0,
            i2c2en: ((value >> 22) & 0b1) > 0,
            usben: ((value >> 23) & 0b1) > 0,
            canen: ((value >> 25) & 0b1) > 0,
            pwren: ((value >> 28) & 0b1) > 0,
            dacen: ((value >> 29) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x1Cu32) as *mut u32) };
        Cache {
            tim2en: ((value >> 0) & 0b1) > 0,
            tim3en: ((value >> 1) & 0b1) > 0,
            tim4en: ((value >> 2) & 0b1) > 0,
            tim6en: ((value >> 4) & 0b1) > 0,
            tim7en: ((value >> 5) & 0b1) > 0,
            wwdgen: ((value >> 11) & 0b1) > 0,
            spi2en: ((value >> 14) & 0b1) > 0,
            spi3en: ((value >> 15) & 0b1) > 0,
            usart2en: ((value >> 17) & 0b1) > 0,
            i2c1en: ((value >> 21) & 0b1) > 0,
            i2c2en: ((value >> 22) & 0b1) > 0,
            usben: ((value >> 23) & 0b1) > 0,
            canen: ((value >> 25) & 0b1) > 0,
            pwren: ((value >> 28) & 0b1) > 0,
            dacen: ((value >> 29) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.tim2en as u32) << 0)
                | ((self.tim3en as u32) << 1)
                | ((self.tim4en as u32) << 2)
                | ((self.tim6en as u32) << 4)
                | ((self.tim7en as u32) << 5)
                | ((self.wwdgen as u32) << 11)
                | ((self.spi2en as u32) << 14)
                | ((self.spi3en as u32) << 15)
                | ((self.usart2en as u32) << 17)
                | ((self.i2c1en as u32) << 21)
                | ((self.i2c2en as u32) << 22)
                | ((self.usben as u32) << 23)
                | ((self.canen as u32) << 25)
                | ((self.pwren as u32) << 28)
                | ((self.dacen as u32) << 29)
            ;
            unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x1Cu32) as *mut u32, value) };
        }
    }
}
/// Backup domain control register (RCC_BDCR)
pub mod bdcr {
    /// External Low Speed oscillator enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set External Low Speed oscillator enable
    pub fn set_lseon(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x20u32) as *mut u32, value) };
    }
    /// Get External Low Speed oscillator enable
    pub fn get_lseon() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x20u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// External Low Speed oscillator ready
    /// Access: read-only, Width: 1, Offset: 1
    /// Get External Low Speed oscillator ready
    pub fn lserdy() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x20u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// External Low Speed oscillator bypass
    /// Access: read-write, Width: 1, Offset: 2
    /// Set External Low Speed oscillator bypass
    pub fn set_lsebyp(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x20u32) as *mut u32, value) };
    }
    /// Get External Low Speed oscillator bypass
    pub fn get_lsebyp() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x20u32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// LSE oscillator drive capability
    /// Access: read-write, Width: 2, Offset: 3
    /// Set LSE oscillator drive capability
    pub fn set_lsedrv(value: u8) {
        debug_assert!(value <= 0b11, "set_lsedrv out of range");
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x20u32) as *mut u32, value) };
    }
    /// Get LSE oscillator drive capability
    pub fn get_lsedrv() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x20u32) as *mut u32) };
        let value = value & (0b11 << 3);
        value as u8
    }
    /// RTC clock source selection
    /// Access: read-write, Width: 2, Offset: 8
    /// Set RTC clock source selection
    pub fn set_rtcsel(value: u8) {
        debug_assert!(value <= 0b11, "set_rtcsel out of range");
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x20u32) as *mut u32, value) };
    }
    /// Get RTC clock source selection
    pub fn get_rtcsel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x20u32) as *mut u32) };
        let value = value & (0b11 << 8);
        value as u8
    }
    /// RTC clock enable
    /// Access: read-write, Width: 1, Offset: 15
    /// Set RTC clock enable
    pub fn set_rtcen(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x20u32) as *mut u32, value) };
    }
    /// Get RTC clock enable
    pub fn get_rtcen() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x20u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// Backup domain software reset
    /// Access: read-write, Width: 1, Offset: 16
    /// Set Backup domain software reset
    pub fn set_bdrst(value: bool) {
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x20u32) as *mut u32, value) };
    }
    /// Get Backup domain software reset
    pub fn get_bdrst() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x20u32) as *mut u32) };
        let value = value & (0b1 << 16);
        value > 0
    }
}
/// Control/status register (RCC_CSR)
pub mod csr {
    /// Internal low speed oscillator enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set Internal low speed oscillator enable
    pub fn set_lsion(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x24u32) as *mut u32, value) };
    }
    /// Get Internal low speed oscillator enable
    pub fn get_lsion() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Internal low speed oscillator ready
    /// Access: read-only, Width: 1, Offset: 1
    /// Get Internal low speed oscillator ready
    pub fn lsirdy() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// Remove reset flag
    /// Access: read-write, Width: 1, Offset: 24
    /// Set Remove reset flag
    pub fn set_rmvf(value: bool) {
        let value = value as u32;
        let value = value << 24;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x24u32) as *mut u32, value) };
    }
    /// Get Remove reset flag
    pub fn get_rmvf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1 << 24);
        value > 0
    }
    /// Option byte loader reset flag
    /// Access: read-write, Width: 1, Offset: 25
    /// Set Option byte loader reset flag
    pub fn set_oblrstf(value: bool) {
        let value = value as u32;
        let value = value << 25;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x24u32) as *mut u32, value) };
    }
    /// Get Option byte loader reset flag
    pub fn get_oblrstf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1 << 25);
        value > 0
    }
    /// PIN reset flag
    /// Access: read-write, Width: 1, Offset: 26
    /// Set PIN reset flag
    pub fn set_pinrstf(value: bool) {
        let value = value as u32;
        let value = value << 26;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x24u32) as *mut u32, value) };
    }
    /// Get PIN reset flag
    pub fn get_pinrstf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1 << 26);
        value > 0
    }
    /// POR/PDR reset flag
    /// Access: read-write, Width: 1, Offset: 27
    /// Set POR/PDR reset flag
    pub fn set_porrstf(value: bool) {
        let value = value as u32;
        let value = value << 27;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x24u32) as *mut u32, value) };
    }
    /// Get POR/PDR reset flag
    pub fn get_porrstf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1 << 27);
        value > 0
    }
    /// Software reset flag
    /// Access: read-write, Width: 1, Offset: 28
    /// Set Software reset flag
    pub fn set_sftrstf(value: bool) {
        let value = value as u32;
        let value = value << 28;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x24u32) as *mut u32, value) };
    }
    /// Get Software reset flag
    pub fn get_sftrstf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1 << 28);
        value > 0
    }
    /// Independent watchdog reset flag
    /// Access: read-write, Width: 1, Offset: 29
    /// Set Independent watchdog reset flag
    pub fn set_iwdgrstf(value: bool) {
        let value = value as u32;
        let value = value << 29;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x24u32) as *mut u32, value) };
    }
    /// Get Independent watchdog reset flag
    pub fn get_iwdgrstf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1 << 29);
        value > 0
    }
    /// Window watchdog reset flag
    /// Access: read-write, Width: 1, Offset: 30
    /// Set Window watchdog reset flag
    pub fn set_wwdgrstf(value: bool) {
        let value = value as u32;
        let value = value << 30;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x24u32) as *mut u32, value) };
    }
    /// Get Window watchdog reset flag
    pub fn get_wwdgrstf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1 << 30);
        value > 0
    }
    /// Low-power reset flag
    /// Access: read-write, Width: 1, Offset: 31
    /// Set Low-power reset flag
    pub fn set_lpwrrstf(value: bool) {
        let value = value as u32;
        let value = value << 31;
        unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x24u32) as *mut u32, value) };
    }
    /// Get Low-power reset flag
    pub fn get_lpwrrstf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
}
/// AHB peripheral reset register
pub mod ahbrstr {
    pub struct ReadonlyCache {
        /// I/O port A reset
        pub ioparst: bool,
        /// I/O port B reset
        pub iopbrst: bool,
        /// I/O port C reset
        pub iopcrst: bool,
        /// I/O port D reset
        pub iopdrst: bool,
        /// I/O port E reset
        pub ioperst: bool,
        /// I/O port F reset
        pub iopfrst: bool,
        /// Touch sensing controller reset
        pub tscrst: bool,
        /// ADC1 and ADC2 reset
        pub adc12rst: bool,
        /// ADC3 and ADC4 reset
        pub adc34rst: bool,
    }
    pub struct Cache {
        /// I/O port A reset
        pub ioparst: bool,
        /// I/O port B reset
        pub iopbrst: bool,
        /// I/O port C reset
        pub iopcrst: bool,
        /// I/O port D reset
        pub iopdrst: bool,
        /// I/O port E reset
        pub ioperst: bool,
        /// I/O port F reset
        pub iopfrst: bool,
        /// Touch sensing controller reset
        pub tscrst: bool,
        /// ADC1 and ADC2 reset
        pub adc12rst: bool,
        /// ADC3 and ADC4 reset
        pub adc34rst: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x28u32) as *mut u32) };
        ReadonlyCache {
            ioparst: ((value >> 17) & 0b1) > 0,
            iopbrst: ((value >> 18) & 0b1) > 0,
            iopcrst: ((value >> 19) & 0b1) > 0,
            iopdrst: ((value >> 20) & 0b1) > 0,
            ioperst: ((value >> 21) & 0b1) > 0,
            iopfrst: ((value >> 22) & 0b1) > 0,
            tscrst: ((value >> 24) & 0b1) > 0,
            adc12rst: ((value >> 28) & 0b1) > 0,
            adc34rst: ((value >> 29) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x28u32) as *mut u32) };
        Cache {
            ioparst: ((value >> 17) & 0b1) > 0,
            iopbrst: ((value >> 18) & 0b1) > 0,
            iopcrst: ((value >> 19) & 0b1) > 0,
            iopdrst: ((value >> 20) & 0b1) > 0,
            ioperst: ((value >> 21) & 0b1) > 0,
            iopfrst: ((value >> 22) & 0b1) > 0,
            tscrst: ((value >> 24) & 0b1) > 0,
            adc12rst: ((value >> 28) & 0b1) > 0,
            adc34rst: ((value >> 29) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.ioparst as u32) << 17)
                | ((self.iopbrst as u32) << 18)
                | ((self.iopcrst as u32) << 19)
                | ((self.iopdrst as u32) << 20)
                | ((self.ioperst as u32) << 21)
                | ((self.iopfrst as u32) << 22)
                | ((self.tscrst as u32) << 24)
                | ((self.adc12rst as u32) << 28)
                | ((self.adc34rst as u32) << 29)
            ;
            unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x28u32) as *mut u32, value) };
        }
    }
}
/// Clock configuration register 2
pub mod cfgr2 {
    pub struct ReadonlyCache {
        /// PREDIV division factor
        pub prediv: u8,
        /// ADC1 and ADC2 prescaler
        pub adc12pres: u8,
        /// ADC3 and ADC4 prescaler
        pub adc34pres: u8,
    }
    pub struct Cache {
        /// PREDIV division factor
        pub prediv: u8,
        /// ADC1 and ADC2 prescaler
        pub adc12pres: u8,
        /// ADC3 and ADC4 prescaler
        pub adc34pres: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x2Cu32) as *mut u32) };
        ReadonlyCache {
            prediv: ((value >> 0) & 0b1111) as u8,
            adc12pres: ((value >> 4) & 0b1111) as u8,
            adc34pres: ((value >> 9) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x2Cu32) as *mut u32) };
        Cache {
            prediv: ((value >> 0) & 0b1111) as u8,
            adc12pres: ((value >> 4) & 0b1111) as u8,
            adc34pres: ((value >> 9) & 0b1111) as u8,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.prediv as u32) << 0)
                | ((self.adc12pres as u32) << 4)
                | ((self.adc34pres as u32) << 9)
            ;
            unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x2Cu32) as *mut u32, value) };
        }
    }
}
/// Clock configuration register 3
pub mod cfgr3 {
    pub struct ReadonlyCache {
        /// USART1 clock source selection
        pub usart1sw: u8,
        /// I2C1 clock source selection
        pub i2c1sw: u8,
        /// I2C2 clock source selection
        pub i2c2sw: u8,
        /// USART2 clock source selection
        pub usart2sw: u8,
        /// USART3 clock source selection
        pub usart3sw: u8,
        /// Timer1 clock source selection
        pub tim1sw: u8,
        /// Timer8 clock source selection
        pub tim8sw: u8,
        /// UART4 clock source selection
        pub uart4sw: u8,
        /// UART5 clock source selection
        pub uart5sw: u8,
    }
    pub struct Cache {
        /// USART1 clock source selection
        pub usart1sw: u8,
        /// I2C1 clock source selection
        pub i2c1sw: u8,
        /// I2C2 clock source selection
        pub i2c2sw: u8,
        /// USART2 clock source selection
        pub usart2sw: u8,
        /// USART3 clock source selection
        pub usart3sw: u8,
        /// Timer1 clock source selection
        pub tim1sw: u8,
        /// Timer8 clock source selection
        pub tim8sw: u8,
        /// UART4 clock source selection
        pub uart4sw: u8,
        /// UART5 clock source selection
        pub uart5sw: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x30u32) as *mut u32) };
        ReadonlyCache {
            usart1sw: ((value >> 0) & 0b11) as u8,
            i2c1sw: ((value >> 4) & 0b11) as u8,
            i2c2sw: ((value >> 5) & 0b11) as u8,
            usart2sw: ((value >> 16) & 0b11) as u8,
            usart3sw: ((value >> 18) & 0b11) as u8,
            tim1sw: ((value >> 8) & 0b11) as u8,
            tim8sw: ((value >> 9) & 0b11) as u8,
            uart4sw: ((value >> 20) & 0b11) as u8,
            uart5sw: ((value >> 22) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40021000u32 + 0x30u32) as *mut u32) };
        Cache {
            usart1sw: ((value >> 0) & 0b11) as u8,
            i2c1sw: ((value >> 4) & 0b11) as u8,
            i2c2sw: ((value >> 5) & 0b11) as u8,
            usart2sw: ((value >> 16) & 0b11) as u8,
            usart3sw: ((value >> 18) & 0b11) as u8,
            tim1sw: ((value >> 8) & 0b11) as u8,
            tim8sw: ((value >> 9) & 0b11) as u8,
            uart4sw: ((value >> 20) & 0b11) as u8,
            uart5sw: ((value >> 22) & 0b11) as u8,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.usart1sw as u32) << 0)
                | ((self.i2c1sw as u32) << 4)
                | ((self.i2c2sw as u32) << 5)
                | ((self.usart2sw as u32) << 16)
                | ((self.usart3sw as u32) << 18)
                | ((self.tim1sw as u32) << 8)
                | ((self.tim8sw as u32) << 9)
                | ((self.uart4sw as u32) << 20)
                | ((self.uart5sw as u32) << 22)
            ;
            unsafe { ::core::ptr::write_volatile((0x40021000u32 + 0x30u32) as *mut u32, value) };
        }
    }
}
/// RCC global interrupt
pub const INTERRUPT_RCC: u32 = 5;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40021000</baseAddress>
  <description>Reset and clock control</description>
  <groupName>RCC</groupName>
  <interrupt>
    <description>RCC global interrupt</description>
    <name>RCC</name>
    <value>5</value>
  </interrupt>
  <name>RCC</name>
  <registers>
    <register>
      <addressOffset>0x0</addressOffset>
      <description>Clock control register</description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Internal High Speed clock
                                enable
                            </description>
          <name>HSION</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Internal High Speed clock ready
                                flag
                            </description>
          <name>HSIRDY</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>5</bitWidth>
          <description>
                                Internal High Speed clock
                                trimming
                            </description>
          <name>HSITRIM</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>
                                Internal High Speed clock
                                Calibration
                            </description>
          <name>HSICAL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                External High Speed clock
                                enable
                            </description>
          <name>HSEON</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                External High Speed clock ready
                                flag
                            </description>
          <name>HSERDY</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                External High Speed clock
                                Bypass
                            </description>
          <name>HSEBYP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Clock Security System
                                enable
                            </description>
          <name>CSSON</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PLL enable</description>
          <name>PLLON</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PLL clock ready flag</description>
          <name>PLLRDY</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x00000083</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x4</addressOffset>
      <description>
                        Clock configuration register
                        (RCC_CFGR)
                    </description>
      <displayName>CFGR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>System clock Switch</description>
          <name>SW</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>System Clock Switch Status</description>
          <name>SWS</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>AHB prescaler</description>
          <name>HPRE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                APB Low speed prescaler
                                (APB1)
                            </description>
          <name>PPRE1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>11</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                APB high speed prescaler
                                (APB2)
                            </description>
          <name>PPRE2</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PLL entry clock source</description>
          <name>PLLSRC</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>HSE divider for PLL entry</description>
          <name>PLLXTPRE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>4</bitWidth>
          <description>PLL Multiplication Factor</description>
          <name>PLLMUL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USB prescaler</description>
          <name>USBPRES</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>24</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Microcontroller clock
                                output
                            </description>
          <name>MCO</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Microcontroller Clock Output
                                Flag
                            </description>
          <name>MCOF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                I2S external clock source
                                selection
                            </description>
          <name>I2SSRC</name>
        </field>
      </fields>
      <name>CFGR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x8</addressOffset>
      <description>
                        Clock interrupt register
                        (RCC_CIR)
                    </description>
      <displayName>CIR</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LSI Ready Interrupt flag</description>
          <name>LSIRDYF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LSE Ready Interrupt flag</description>
          <name>LSERDYF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>HSI Ready Interrupt flag</description>
          <name>HSIRDYF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>HSE Ready Interrupt flag</description>
          <name>HSERDYF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PLL Ready Interrupt flag</description>
          <name>PLLRDYF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Clock Security System Interrupt
                                flag
                            </description>
          <name>CSSF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LSI Ready Interrupt Enable</description>
          <name>LSIRDYIE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LSE Ready Interrupt Enable</description>
          <name>LSERDYIE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>HSI Ready Interrupt Enable</description>
          <name>HSIRDYIE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>HSE Ready Interrupt Enable</description>
          <name>HSERDYIE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PLL Ready Interrupt Enable</description>
          <name>PLLRDYIE</name>
        </field>
        <field>
          <access>write-only</access>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LSI Ready Interrupt Clear</description>
          <name>LSIRDYC</name>
        </field>
        <field>
          <access>write-only</access>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LSE Ready Interrupt Clear</description>
          <name>LSERDYC</name>
        </field>
        <field>
          <access>write-only</access>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>HSI Ready Interrupt Clear</description>
          <name>HSIRDYC</name>
        </field>
        <field>
          <access>write-only</access>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>HSE Ready Interrupt Clear</description>
          <name>HSERDYC</name>
        </field>
        <field>
          <access>write-only</access>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PLL Ready Interrupt Clear</description>
          <name>PLLRDYC</name>
        </field>
        <field>
          <access>write-only</access>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Clock security system interrupt
                                clear
                            </description>
          <name>CSSC</name>
        </field>
      </fields>
      <name>CIR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>
                        APB2 peripheral reset register
                        (RCC_APB2RSTR)
                    </description>
      <displayName>APB2RSTR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SYSCFG and COMP reset</description>
          <name>SYSCFGRST</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM1 timer reset</description>
          <name>TIM1RST</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SPI 1 reset</description>
          <name>SPI1RST</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM8 timer reset</description>
          <name>TIM8RST</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USART1 reset</description>
          <name>USART1RST</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM15 timer reset</description>
          <name>TIM15RST</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM16 timer reset</description>
          <name>TIM16RST</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM17 timer reset</description>
          <name>TIM17RST</name>
        </field>
      </fields>
      <name>APB2RSTR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>
                        APB1 peripheral reset register
                        (RCC_APB1RSTR)
                    </description>
      <displayName>APB1RSTR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Timer 2 reset</description>
          <name>TIM2RST</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Timer 3 reset</description>
          <name>TIM3RST</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Timer 14 reset</description>
          <name>TIM4RST</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Timer 6 reset</description>
          <name>TIM6RST</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Timer 7 reset</description>
          <name>TIM7RST</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Window watchdog reset</description>
          <name>WWDGRST</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SPI2 reset</description>
          <name>SPI2RST</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SPI3 reset</description>
          <name>SPI3RST</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USART 2 reset</description>
          <name>USART2RST</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USART3 reset</description>
          <name>USART3RST</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>UART 4 reset</description>
          <name>UART4RST</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>UART 5 reset</description>
          <name>UART5RST</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I2C1 reset</description>
          <name>I2C1RST</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I2C2 reset</description>
          <name>I2C2RST</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USB reset</description>
          <name>USBRST</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CAN reset</description>
          <name>CANRST</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Power interface reset</description>
          <name>PWRRST</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC interface reset</description>
          <name>DACRST</name>
        </field>
      </fields>
      <name>APB1RSTR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>
                        AHB Peripheral Clock enable register
                        (RCC_AHBENR)
                    </description>
      <displayName>AHBENR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DMA1 clock enable</description>
          <name>DMAEN</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DMA2 clock enable</description>
          <name>DMA2EN</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                SRAM interface clock
                                enable
                            </description>
          <name>SRAMEN</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FLITF clock enable</description>
          <name>FLITFEN</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CRC clock enable</description>
          <name>CRCEN</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I/O port A clock enable</description>
          <name>IOPAEN</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I/O port B clock enable</description>
          <name>IOPBEN</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I/O port C clock enable</description>
          <name>IOPCEN</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I/O port D clock enable</description>
          <name>IOPDEN</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I/O port E clock enable</description>
          <name>IOPEEN</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I/O port F clock enable</description>
          <name>IOPFEN</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Touch sensing controller clock
                                enable
                            </description>
          <name>TSCEN</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADC1 and ADC2 clock enable</description>
          <name>ADC12EN</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADC3 and ADC4 clock enable</description>
          <name>ADC34EN</name>
        </field>
      </fields>
      <name>AHBENR</name>
      <resetValue>0x00000014</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>
                        APB2 peripheral clock enable register
                        (RCC_APB2ENR)
                    </description>
      <displayName>APB2ENR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SYSCFG clock enable</description>
          <name>SYSCFGEN</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM1 Timer clock enable</description>
          <name>TIM1EN</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SPI 1 clock enable</description>
          <name>SPI1EN</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM8 Timer clock enable</description>
          <name>TIM8EN</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USART1 clock enable</description>
          <name>USART1EN</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM15 timer clock enable</description>
          <name>TIM15EN</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM16 timer clock enable</description>
          <name>TIM16EN</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM17 timer clock enable</description>
          <name>TIM17EN</name>
        </field>
      </fields>
      <name>APB2ENR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>
                        APB1 peripheral clock enable register
                        (RCC_APB1ENR)
                    </description>
      <displayName>APB1ENR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Timer 2 clock enable</description>
          <name>TIM2EN</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Timer 3 clock enable</description>
          <name>TIM3EN</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Timer 4 clock enable</description>
          <name>TIM4EN</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Timer 6 clock enable</description>
          <name>TIM6EN</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Timer 7 clock enable</description>
          <name>TIM7EN</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Window watchdog clock
                                enable
                            </description>
          <name>WWDGEN</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SPI 2 clock enable</description>
          <name>SPI2EN</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SPI 3 clock enable</description>
          <name>SPI3EN</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USART 2 clock enable</description>
          <name>USART2EN</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I2C 1 clock enable</description>
          <name>I2C1EN</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I2C 2 clock enable</description>
          <name>I2C2EN</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USB clock enable</description>
          <name>USBEN</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CAN clock enable</description>
          <name>CANEN</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Power interface clock
                                enable
                            </description>
          <name>PWREN</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC interface clock enable</description>
          <name>DACEN</name>
        </field>
      </fields>
      <name>APB1ENR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x20</addressOffset>
      <description>
                        Backup domain control register
                        (RCC_BDCR)
                    </description>
      <displayName>BDCR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                External Low Speed oscillator
                                enable
                            </description>
          <name>LSEON</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                External Low Speed oscillator
                                ready
                            </description>
          <name>LSERDY</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                External Low Speed oscillator
                                bypass
                            </description>
          <name>LSEBYP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                LSE oscillator drive
                                capability
                            </description>
          <name>LSEDRV</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>RTC clock source selection</description>
          <name>RTCSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTC clock enable</description>
          <name>RTCEN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Backup domain software
                                reset
                            </description>
          <name>BDRST</name>
        </field>
      </fields>
      <name>BDCR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x24</addressOffset>
      <description>
                        Control/status register
                        (RCC_CSR)
                    </description>
      <displayName>CSR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Internal low speed oscillator
                                enable
                            </description>
          <name>LSION</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Internal low speed oscillator
                                ready
                            </description>
          <name>LSIRDY</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Remove reset flag</description>
          <name>RMVF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Option byte loader reset
                                flag
                            </description>
          <name>OBLRSTF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PIN reset flag</description>
          <name>PINRSTF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>POR/PDR reset flag</description>
          <name>PORRSTF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software reset flag</description>
          <name>SFTRSTF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Independent watchdog reset
                                flag
                            </description>
          <name>IWDGRSTF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Window watchdog reset flag</description>
          <name>WWDGRSTF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Low-power reset flag</description>
          <name>LPWRRSTF</name>
        </field>
      </fields>
      <name>CSR</name>
      <resetValue>0x0C000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28</addressOffset>
      <description>AHB peripheral reset register</description>
      <displayName>AHBRSTR</displayName>
      <fields>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I/O port A reset</description>
          <name>IOPARST</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I/O port B reset</description>
          <name>IOPBRST</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I/O port C reset</description>
          <name>IOPCRST</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I/O port D reset</description>
          <name>IOPDRST</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I/O port E reset</description>
          <name>IOPERST</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I/O port F reset</description>
          <name>IOPFRST</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Touch sensing controller
                                reset
                            </description>
          <name>TSCRST</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADC1 and ADC2 reset</description>
          <name>ADC12RST</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADC3 and ADC4 reset</description>
          <name>ADC34RST</name>
        </field>
      </fields>
      <name>AHBRSTR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2C</addressOffset>
      <description>Clock configuration register 2</description>
      <displayName>CFGR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>PREDIV division factor</description>
          <name>PREDIV</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>5</bitWidth>
          <description>ADC1 and ADC2 prescaler</description>
          <name>ADC12PRES</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>5</bitWidth>
          <description>ADC3 and ADC4 prescaler</description>
          <name>ADC34PRES</name>
        </field>
      </fields>
      <name>CFGR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x30</addressOffset>
      <description>Clock configuration register 3</description>
      <displayName>CFGR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                USART1 clock source
                                selection
                            </description>
          <name>USART1SW</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                I2C1 clock source
                                selection
                            </description>
          <name>I2C1SW</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                I2C2 clock source
                                selection
                            </description>
          <name>I2C2SW</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                USART2 clock source
                                selection
                            </description>
          <name>USART2SW</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                USART3 clock source
                                selection
                            </description>
          <name>USART3SW</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Timer1 clock source
                                selection
                            </description>
          <name>TIM1SW</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Timer8 clock source
                                selection
                            </description>
          <name>TIM8SW</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                UART4 clock source
                                selection
                            </description>
          <name>UART4SW</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                UART5 clock source
                                selection
                            </description>
          <name>UART5SW</name>
        </field>
      </fields>
      <name>CFGR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
