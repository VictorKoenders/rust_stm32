/// Control register 1
pub mod cr1 {
    /// Peripheral enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set Peripheral enable
    pub fn set_pe(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Peripheral enable
    pub fn get_pe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// TX Interrupt enable
    /// Access: read-write, Width: 1, Offset: 1
    /// Set TX Interrupt enable
    pub fn set_txie(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get TX Interrupt enable
    pub fn get_txie() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// RX Interrupt enable
    /// Access: read-write, Width: 1, Offset: 2
    /// Set RX Interrupt enable
    pub fn set_rxie(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get RX Interrupt enable
    pub fn get_rxie() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// Address match interrupt enable (slave only)
    /// Access: read-write, Width: 1, Offset: 3
    /// Set Address match interrupt enable (slave only)
    pub fn set_addrie(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Address match interrupt enable (slave only)
    pub fn get_addrie() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// Not acknowledge received interrupt enable
    /// Access: read-write, Width: 1, Offset: 4
    /// Set Not acknowledge received interrupt enable
    pub fn set_nackie(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Not acknowledge received interrupt enable
    pub fn get_nackie() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// STOP detection Interrupt enable
    /// Access: read-write, Width: 1, Offset: 5
    /// Set STOP detection Interrupt enable
    pub fn set_stopie(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get STOP detection Interrupt enable
    pub fn get_stopie() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// Transfer Complete interrupt enable
    /// Access: read-write, Width: 1, Offset: 6
    /// Set Transfer Complete interrupt enable
    pub fn set_tcie(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Transfer Complete interrupt enable
    pub fn get_tcie() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Error interrupts enable
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Error interrupts enable
    pub fn set_errie(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Error interrupts enable
    pub fn get_errie() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Digital noise filter
    /// Access: read-write, Width: 4, Offset: 8
    /// Set Digital noise filter
    pub fn set_dnf(value: u8) {
        debug_assert!(value <= 0b1111, "set_dnf out of range");
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Digital noise filter
    pub fn get_dnf() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1111 << 8);
        value as u8
    }
    /// Analog noise filter OFF
    /// Access: read-write, Width: 1, Offset: 12
    /// Set Analog noise filter OFF
    pub fn set_anfoff(value: bool) {
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Analog noise filter OFF
    pub fn get_anfoff() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 12);
        value > 0
    }
    /// Software reset
    /// Access: write-only, Width: 1, Offset: 13
    /// Set Software reset
    pub fn swrst(value: bool) {
        let value = value as u32;
        let value = value << 13;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// DMA transmission requests enable
    /// Access: read-write, Width: 1, Offset: 14
    /// Set DMA transmission requests enable
    pub fn set_txdmaen(value: bool) {
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get DMA transmission requests enable
    pub fn get_txdmaen() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// DMA reception requests enable
    /// Access: read-write, Width: 1, Offset: 15
    /// Set DMA reception requests enable
    pub fn set_rxdmaen(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get DMA reception requests enable
    pub fn get_rxdmaen() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// Slave byte control
    /// Access: read-write, Width: 1, Offset: 16
    /// Set Slave byte control
    pub fn set_sbc(value: bool) {
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Slave byte control
    pub fn get_sbc() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 16);
        value > 0
    }
    /// Clock stretching disable
    /// Access: read-write, Width: 1, Offset: 17
    /// Set Clock stretching disable
    pub fn set_nostretch(value: bool) {
        let value = value as u32;
        let value = value << 17;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Clock stretching disable
    pub fn get_nostretch() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 17);
        value > 0
    }
    /// Wakeup from STOP enable
    /// Access: read-write, Width: 1, Offset: 18
    /// Set Wakeup from STOP enable
    pub fn set_wupen(value: bool) {
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Wakeup from STOP enable
    pub fn get_wupen() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 18);
        value > 0
    }
    /// General call enable
    /// Access: read-write, Width: 1, Offset: 19
    /// Set General call enable
    pub fn set_gcen(value: bool) {
        let value = value as u32;
        let value = value << 19;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get General call enable
    pub fn get_gcen() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 19);
        value > 0
    }
    /// SMBus Host address enable
    /// Access: read-write, Width: 1, Offset: 20
    /// Set SMBus Host address enable
    pub fn set_smbhen(value: bool) {
        let value = value as u32;
        let value = value << 20;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get SMBus Host address enable
    pub fn get_smbhen() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 20);
        value > 0
    }
    /// SMBus Device Default address enable
    /// Access: read-write, Width: 1, Offset: 21
    /// Set SMBus Device Default address enable
    pub fn set_smbden(value: bool) {
        let value = value as u32;
        let value = value << 21;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get SMBus Device Default address enable
    pub fn get_smbden() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 21);
        value > 0
    }
    /// SMBUS alert enable
    /// Access: read-write, Width: 1, Offset: 22
    /// Set SMBUS alert enable
    pub fn set_alerten(value: bool) {
        let value = value as u32;
        let value = value << 22;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get SMBUS alert enable
    pub fn get_alerten() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 22);
        value > 0
    }
    /// PEC enable
    /// Access: read-write, Width: 1, Offset: 23
    /// Set PEC enable
    pub fn set_pecen(value: bool) {
        let value = value as u32;
        let value = value << 23;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get PEC enable
    pub fn get_pecen() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 23);
        value > 0
    }
}
/// Control register 2
pub mod cr2 {
    pub struct ReadonlyCache {
        /// Packet error checking byte
        pub pecbyte: bool,
        /// Automatic end mode (master mode)
        pub autoend: bool,
        /// NBYTES reload mode
        pub reload: bool,
        /// Number of bytes
        pub nbytes: bool,
        /// NACK generation (slave mode)
        pub nack: bool,
        /// Stop generation (master mode)
        pub stop: bool,
        /// Start generation
        pub start: bool,
        /// 10-bit address header only read direction (master receiver mode)
        pub head10r: bool,
        /// 10-bit addressing mode (master mode)
        pub add10: bool,
        /// Transfer direction (master mode)
        pub rd_wrn: bool,
        /// Slave address bit 9:8 (master mode)
        pub sadd8: bool,
        /// Slave address bit 7:1 (master mode)
        pub sadd1: bool,
        /// Slave address bit 0 (master mode)
        pub sadd0: bool,
    }
    pub struct Cache {
        /// Packet error checking byte
        pub pecbyte: bool,
        /// Automatic end mode (master mode)
        pub autoend: bool,
        /// NBYTES reload mode
        pub reload: bool,
        /// Number of bytes
        pub nbytes: bool,
        /// NACK generation (slave mode)
        pub nack: bool,
        /// Stop generation (master mode)
        pub stop: bool,
        /// Start generation
        pub start: bool,
        /// 10-bit address header only read direction (master receiver mode)
        pub head10r: bool,
        /// 10-bit addressing mode (master mode)
        pub add10: bool,
        /// Transfer direction (master mode)
        pub rd_wrn: bool,
        /// Slave address bit 9:8 (master mode)
        pub sadd8: bool,
        /// Slave address bit 7:1 (master mode)
        pub sadd1: bool,
        /// Slave address bit 0 (master mode)
        pub sadd0: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            pecbyte: ((value >> 26) & 0b1) > 0,
            autoend: ((value >> 25) & 0b1) > 0,
            reload: ((value >> 24) & 0b1) > 0,
            nbytes: ((value >> 16) & 0b1) > 0,
            nack: ((value >> 15) & 0b1) > 0,
            stop: ((value >> 14) & 0b1) > 0,
            start: ((value >> 13) & 0b1) > 0,
            head10r: ((value >> 12) & 0b1) > 0,
            add10: ((value >> 11) & 0b1) > 0,
            rd_wrn: ((value >> 10) & 0b1) > 0,
            sadd8: ((value >> 8) & 0b1) > 0,
            sadd1: ((value >> 1) & 0b1) > 0,
            sadd0: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x4u32) as *mut u32) };
        Cache {
            pecbyte: ((value >> 26) & 0b1) > 0,
            autoend: ((value >> 25) & 0b1) > 0,
            reload: ((value >> 24) & 0b1) > 0,
            nbytes: ((value >> 16) & 0b1) > 0,
            nack: ((value >> 15) & 0b1) > 0,
            stop: ((value >> 14) & 0b1) > 0,
            start: ((value >> 13) & 0b1) > 0,
            head10r: ((value >> 12) & 0b1) > 0,
            add10: ((value >> 11) & 0b1) > 0,
            rd_wrn: ((value >> 10) & 0b1) > 0,
            sadd8: ((value >> 8) & 0b1) > 0,
            sadd1: ((value >> 1) & 0b1) > 0,
            sadd0: ((value >> 0) & 0b1) > 0,
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
                | ((self.pecbyte as u32) << 26)
                | ((self.autoend as u32) << 25)
                | ((self.reload as u32) << 24)
                | ((self.nbytes as u32) << 16)
                | ((self.nack as u32) << 15)
                | ((self.stop as u32) << 14)
                | ((self.start as u32) << 13)
                | ((self.head10r as u32) << 12)
                | ((self.add10 as u32) << 11)
                | ((self.rd_wrn as u32) << 10)
                | ((self.sadd8 as u32) << 8)
                | ((self.sadd1 as u32) << 1)
                | ((self.sadd0 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// Own address register 1
pub mod oar1 {
    pub struct ReadonlyCache {
        /// Interface address
        pub oa1_0: bool,
        /// Interface address
        pub oa1_1: bool,
        /// Interface address
        pub oa1_8: bool,
        /// Own Address 1 10-bit mode
        pub oa1mode: bool,
        /// Own Address 1 enable
        pub oa1en: bool,
    }
    pub struct Cache {
        /// Interface address
        pub oa1_0: bool,
        /// Interface address
        pub oa1_1: bool,
        /// Interface address
        pub oa1_8: bool,
        /// Own Address 1 10-bit mode
        pub oa1mode: bool,
        /// Own Address 1 enable
        pub oa1en: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            oa1_0: ((value >> 0) & 0b1) > 0,
            oa1_1: ((value >> 1) & 0b1) > 0,
            oa1_8: ((value >> 8) & 0b1) > 0,
            oa1mode: ((value >> 10) & 0b1) > 0,
            oa1en: ((value >> 15) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x8u32) as *mut u32) };
        Cache {
            oa1_0: ((value >> 0) & 0b1) > 0,
            oa1_1: ((value >> 1) & 0b1) > 0,
            oa1_8: ((value >> 8) & 0b1) > 0,
            oa1mode: ((value >> 10) & 0b1) > 0,
            oa1en: ((value >> 15) & 0b1) > 0,
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
                | ((self.oa1_0 as u32) << 0)
                | ((self.oa1_1 as u32) << 1)
                | ((self.oa1_8 as u32) << 8)
                | ((self.oa1mode as u32) << 10)
                | ((self.oa1en as u32) << 15)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// Own address register 2
pub mod oar2 {
    pub struct ReadonlyCache {
        /// Interface address
        pub oa2: u8,
        /// Own Address 2 masks
        pub oa2msk: u8,
        /// Own Address 2 enable
        pub oa2en: u8,
    }
    pub struct Cache {
        /// Interface address
        pub oa2: u8,
        /// Own Address 2 masks
        pub oa2msk: u8,
        /// Own Address 2 enable
        pub oa2en: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            oa2: ((value >> 1) & 0b1111111) as u8,
            oa2msk: ((value >> 8) & 0b1111111) as u8,
            oa2en: ((value >> 15) & 0b1111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0xCu32) as *mut u32) };
        Cache {
            oa2: ((value >> 1) & 0b1111111) as u8,
            oa2msk: ((value >> 8) & 0b1111111) as u8,
            oa2en: ((value >> 15) & 0b1111111) as u8,
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
                | ((self.oa2 as u32) << 1)
                | ((self.oa2msk as u32) << 8)
                | ((self.oa2en as u32) << 15)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// Timing register
pub mod timingr {
    pub struct ReadonlyCache {
        /// SCL low period (master mode)
        pub scll: u8,
        /// SCL high period (master mode)
        pub sclh: u8,
        /// Data hold time
        pub sdadel: u8,
        /// Data setup time
        pub scldel: u8,
        /// Timing prescaler
        pub presc: u8,
    }
    pub struct Cache {
        /// SCL low period (master mode)
        pub scll: u8,
        /// SCL high period (master mode)
        pub sclh: u8,
        /// Data hold time
        pub sdadel: u8,
        /// Data setup time
        pub scldel: u8,
        /// Timing prescaler
        pub presc: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            scll: ((value >> 0) & 0b11111111) as u8,
            sclh: ((value >> 8) & 0b11111111) as u8,
            sdadel: ((value >> 16) & 0b11111111) as u8,
            scldel: ((value >> 20) & 0b11111111) as u8,
            presc: ((value >> 28) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x10u32) as *mut u32) };
        Cache {
            scll: ((value >> 0) & 0b11111111) as u8,
            sclh: ((value >> 8) & 0b11111111) as u8,
            sdadel: ((value >> 16) & 0b11111111) as u8,
            scldel: ((value >> 20) & 0b11111111) as u8,
            presc: ((value >> 28) & 0b11111111) as u8,
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
                | ((self.scll as u32) << 0)
                | ((self.sclh as u32) << 8)
                | ((self.sdadel as u32) << 16)
                | ((self.scldel as u32) << 20)
                | ((self.presc as u32) << 28)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x10u32) as *mut u32, value) };
        }
    }
}
/// Status register 1
pub mod timeoutr {
    pub struct ReadonlyCache {
        /// Bus timeout A
        pub timeouta: u16,
        /// Idle clock timeout detection
        pub tidle: u16,
        /// Clock timeout enable
        pub timouten: u16,
        /// Bus timeout B
        pub timeoutb: u16,
        /// Extended clock timeout enable
        pub texten: u16,
    }
    pub struct Cache {
        /// Bus timeout A
        pub timeouta: u16,
        /// Idle clock timeout detection
        pub tidle: u16,
        /// Clock timeout enable
        pub timouten: u16,
        /// Bus timeout B
        pub timeoutb: u16,
        /// Extended clock timeout enable
        pub texten: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x14u32) as *mut u32) };
        ReadonlyCache {
            timeouta: ((value >> 0) & 0b111111111111) as u16,
            tidle: ((value >> 12) & 0b111111111111) as u16,
            timouten: ((value >> 15) & 0b111111111111) as u16,
            timeoutb: ((value >> 16) & 0b111111111111) as u16,
            texten: ((value >> 31) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x14u32) as *mut u32) };
        Cache {
            timeouta: ((value >> 0) & 0b111111111111) as u16,
            tidle: ((value >> 12) & 0b111111111111) as u16,
            timouten: ((value >> 15) & 0b111111111111) as u16,
            timeoutb: ((value >> 16) & 0b111111111111) as u16,
            texten: ((value >> 31) & 0b111111111111) as u16,
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
                | ((self.timeouta as u32) << 0)
                | ((self.tidle as u32) << 12)
                | ((self.timouten as u32) << 15)
                | ((self.timeoutb as u32) << 16)
                | ((self.texten as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x14u32) as *mut u32, value) };
        }
    }
}
/// Interrupt and Status register
pub mod isr {
    /// Address match code (Slave mode)
    /// Access: read-only, Width: 7, Offset: 17
    /// Get Address match code (Slave mode)
    pub fn addcode() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1111111 << 17);
        value as u8
    }
    /// Transfer direction (Slave mode)
    /// Access: read-only, Width: 1, Offset: 16
    /// Get Transfer direction (Slave mode)
    pub fn dir() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 16);
        value > 0
    }
    /// Bus busy
    /// Access: read-only, Width: 1, Offset: 15
    /// Get Bus busy
    pub fn busy() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// SMBus alert
    /// Access: read-only, Width: 1, Offset: 13
    /// Get SMBus alert
    pub fn alert() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 13);
        value > 0
    }
    /// Timeout or t_low detection flag
    /// Access: read-only, Width: 1, Offset: 12
    /// Get Timeout or t_low detection flag
    pub fn timeout() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 12);
        value > 0
    }
    /// PEC Error in reception
    /// Access: read-only, Width: 1, Offset: 11
    /// Get PEC Error in reception
    pub fn pecerr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Overrun/Underrun (slave mode)
    /// Access: read-only, Width: 1, Offset: 10
    /// Get Overrun/Underrun (slave mode)
    pub fn ovr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 10);
        value > 0
    }
    /// Arbitration lost
    /// Access: read-only, Width: 1, Offset: 9
    /// Get Arbitration lost
    pub fn arlo() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// Bus error
    /// Access: read-only, Width: 1, Offset: 8
    /// Get Bus error
    pub fn berr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Transfer Complete Reload
    /// Access: read-only, Width: 1, Offset: 7
    /// Get Transfer Complete Reload
    pub fn tcr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Transfer Complete (master mode)
    /// Access: read-only, Width: 1, Offset: 6
    /// Get Transfer Complete (master mode)
    pub fn tc() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Stop detection flag
    /// Access: read-only, Width: 1, Offset: 5
    /// Get Stop detection flag
    pub fn stopf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// Not acknowledge received flag
    /// Access: read-only, Width: 1, Offset: 4
    /// Get Not acknowledge received flag
    pub fn nackf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// Address matched (slave mode)
    /// Access: read-only, Width: 1, Offset: 3
    /// Get Address matched (slave mode)
    pub fn addr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// Receive data register not empty (receivers)
    /// Access: read-only, Width: 1, Offset: 2
    /// Get Receive data register not empty (receivers)
    pub fn rxne() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// Transmit interrupt status (transmitters)
    /// Access: read-write, Width: 1, Offset: 1
    /// Set Transmit interrupt status (transmitters)
    pub fn set_txis(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Transmit interrupt status (transmitters)
    pub fn get_txis() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// Transmit data register empty (transmitters)
    /// Access: read-write, Width: 1, Offset: 0
    /// Set Transmit data register empty (transmitters)
    pub fn set_txe(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Transmit data register empty (transmitters)
    pub fn get_txe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// Interrupt clear register
pub mod icr {
    /// Alert flag clear
    /// Access: write-only, Width: 1, Offset: 13
    /// Set Alert flag clear
    pub fn alertcf(value: bool) {
        let value = value as u32;
        let value = value << 13;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Timeout detection flag clear
    /// Access: write-only, Width: 1, Offset: 12
    /// Set Timeout detection flag clear
    pub fn timoutcf(value: bool) {
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// PEC Error flag clear
    /// Access: write-only, Width: 1, Offset: 11
    /// Set PEC Error flag clear
    pub fn peccf(value: bool) {
        let value = value as u32;
        let value = value << 11;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Overrun/Underrun flag clear
    /// Access: write-only, Width: 1, Offset: 10
    /// Set Overrun/Underrun flag clear
    pub fn ovrcf(value: bool) {
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Arbitration lost flag clear
    /// Access: write-only, Width: 1, Offset: 9
    /// Set Arbitration lost flag clear
    pub fn arlocf(value: bool) {
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Bus error flag clear
    /// Access: write-only, Width: 1, Offset: 8
    /// Set Bus error flag clear
    pub fn berrcf(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Stop detection flag clear
    /// Access: write-only, Width: 1, Offset: 5
    /// Set Stop detection flag clear
    pub fn stopcf(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Not Acknowledge flag clear
    /// Access: write-only, Width: 1, Offset: 4
    /// Set Not Acknowledge flag clear
    pub fn nackcf(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Address Matched flag clear
    /// Access: write-only, Width: 1, Offset: 3
    /// Set Address Matched flag clear
    pub fn addrcf(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x1Cu32) as *mut u32, value) };
    }
}
/// PEC register
pub mod pecr {
    /// Packet error checking register
    /// Access: read-only, Width: 8, Offset: 0
    /// Get Packet error checking register
    pub fn pec() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x20u32) as *mut u32) };
        let value = value & (0b11111111 << 0);
        value as u8
    }
}
/// Receive data register
pub mod rxdr {
    /// 8-bit receive data
    /// Access: read-only, Width: 8, Offset: 0
    /// Get 8-bit receive data
    pub fn rxdata() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x24u32) as *mut u32) };
        let value = value & (0b11111111 << 0);
        value as u8
    }
}
/// Transmit data register
pub mod txdr {
    pub struct ReadonlyCache {
        /// 8-bit transmit data
        pub txdata: u8,
    }
    pub struct Cache {
        /// 8-bit transmit data
        pub txdata: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x28u32) as *mut u32) };
        ReadonlyCache {
            txdata: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005400u32 + 0x28u32) as *mut u32) };
        Cache {
            txdata: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.txdata as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005400u32 + 0x28u32) as *mut u32, value) };
        }
    }
}
/// I2C1 event interrupt and EXTI Line23 interrupt
pub const INTERRUPT_I2C1_EV_EXTI23: u32 = 31;
/// I2C1 error interrupt
pub const INTERRUPT_I2C1_ER: u32 = 32;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40005400</baseAddress>
  <description>Inter-integrated circuit</description>
  <groupName>I2C</groupName>
  <interrupt>
    <description>
                    I2C1 event interrupt and EXTI Line23
                    interrupt
                </description>
    <name>I2C1_EV_EXTI23</name>
    <value>31</value>
  </interrupt>
  <interrupt>
    <description>I2C1 error interrupt</description>
    <name>I2C1_ER</name>
    <value>32</value>
  </interrupt>
  <name>I2C1</name>
  <registers>
    <register>
      <addressOffset>0x0</addressOffset>
      <description>Control register 1</description>
      <displayName>CR1</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Peripheral enable</description>
          <name>PE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TX Interrupt enable</description>
          <name>TXIE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RX Interrupt enable</description>
          <name>RXIE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Address match interrupt enable (slave
                                only)
                            </description>
          <name>ADDRIE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Not acknowledge received interrupt
                                enable
                            </description>
          <name>NACKIE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                STOP detection Interrupt
                                enable
                            </description>
          <name>STOPIE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer Complete interrupt
                                enable
                            </description>
          <name>TCIE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Error interrupts enable</description>
          <name>ERRIE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Digital noise filter</description>
          <name>DNF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog noise filter OFF</description>
          <name>ANFOFF</name>
        </field>
        <field>
          <access>write-only</access>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software reset</description>
          <name>SWRST</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DMA transmission requests
                                enable
                            </description>
          <name>TXDMAEN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DMA reception requests
                                enable
                            </description>
          <name>RXDMAEN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Slave byte control</description>
          <name>SBC</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clock stretching disable</description>
          <name>NOSTRETCH</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Wakeup from STOP enable</description>
          <name>WUPEN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>General call enable</description>
          <name>GCEN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SMBus Host address enable</description>
          <name>SMBHEN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                SMBus Device Default address
                                enable
                            </description>
          <name>SMBDEN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SMBUS alert enable</description>
          <name>ALERTEN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PEC enable</description>
          <name>PECEN</name>
        </field>
      </fields>
      <name>CR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>Control register 2</description>
      <displayName>CR2</displayName>
      <fields>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Packet error checking byte</description>
          <name>PECBYTE</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Automatic end mode (master
                                mode)
                            </description>
          <name>AUTOEND</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>NBYTES reload mode</description>
          <name>RELOAD</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Number of bytes</description>
          <name>NBYTES</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                NACK generation (slave
                                mode)
                            </description>
          <name>NACK</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Stop generation (master
                                mode)
                            </description>
          <name>STOP</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Start generation</description>
          <name>START</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                10-bit address header only read
                                direction (master receiver mode)
                            </description>
          <name>HEAD10R</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                10-bit addressing mode (master
                                mode)
                            </description>
          <name>ADD10</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer direction (master
                                mode)
                            </description>
          <name>RD_WRN</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Slave address bit 9:8 (master
                                mode)
                            </description>
          <name>SADD8</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>7</bitWidth>
          <description>
                                Slave address bit 7:1 (master
                                mode)
                            </description>
          <name>SADD1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Slave address bit 0 (master
                                mode)
                            </description>
          <name>SADD0</name>
        </field>
      </fields>
      <name>CR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>Own address register 1</description>
      <displayName>OAR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interface address</description>
          <name>OA1_0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>7</bitWidth>
          <description>Interface address</description>
          <name>OA1_1</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Interface address</description>
          <name>OA1_8</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Own Address 1 10-bit mode</description>
          <name>OA1MODE</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Own Address 1 enable</description>
          <name>OA1EN</name>
        </field>
      </fields>
      <name>OAR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>Own address register 2</description>
      <displayName>OAR2</displayName>
      <fields>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>7</bitWidth>
          <description>Interface address</description>
          <name>OA2</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Own Address 2 masks</description>
          <name>OA2MSK</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Own Address 2 enable</description>
          <name>OA2EN</name>
        </field>
      </fields>
      <name>OAR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>Timing register</description>
      <displayName>TIMINGR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>
                                SCL low period (master
                                mode)
                            </description>
          <name>SCLL</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>
                                SCL high period (master
                                mode)
                            </description>
          <name>SCLH</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Data hold time</description>
          <name>SDADEL</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Data setup time</description>
          <name>SCLDEL</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Timing prescaler</description>
          <name>PRESC</name>
        </field>
      </fields>
      <name>TIMINGR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>Status register 1</description>
      <displayName>TIMEOUTR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>Bus timeout A</description>
          <name>TIMEOUTA</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Idle clock timeout
                                detection
                            </description>
          <name>TIDLE</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clock timeout enable</description>
          <name>TIMOUTEN</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>12</bitWidth>
          <description>Bus timeout B</description>
          <name>TIMEOUTB</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Extended clock timeout
                                enable
                            </description>
          <name>TEXTEN</name>
        </field>
      </fields>
      <name>TIMEOUTR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x18</addressOffset>
      <description>Interrupt and Status register</description>
      <displayName>ISR</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>17</bitOffset>
          <bitWidth>7</bitWidth>
          <description>
                                Address match code (Slave
                                mode)
                            </description>
          <name>ADDCODE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer direction (Slave
                                mode)
                            </description>
          <name>DIR</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Bus busy</description>
          <name>BUSY</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SMBus alert</description>
          <name>ALERT</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Timeout or t_low detection
                                flag
                            </description>
          <name>TIMEOUT</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PEC Error in reception</description>
          <name>PECERR</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Overrun/Underrun (slave
                                mode)
                            </description>
          <name>OVR</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Arbitration lost</description>
          <name>ARLO</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Bus error</description>
          <name>BERR</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Transfer Complete Reload</description>
          <name>TCR</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer Complete (master
                                mode)
                            </description>
          <name>TC</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Stop detection flag</description>
          <name>STOPF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Not acknowledge received
                                flag
                            </description>
          <name>NACKF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Address matched (slave
                                mode)
                            </description>
          <name>ADDR</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Receive data register not empty
                                (receivers)
                            </description>
          <name>RXNE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transmit interrupt status
                                (transmitters)
                            </description>
          <name>TXIS</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transmit data register empty
                                (transmitters)
                            </description>
          <name>TXE</name>
        </field>
      </fields>
      <name>ISR</name>
      <resetValue>0x00000001</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x1C</addressOffset>
      <description>Interrupt clear register</description>
      <displayName>ICR</displayName>
      <fields>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alert flag clear</description>
          <name>ALERTCF</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Timeout detection flag
                                clear
                            </description>
          <name>TIMOUTCF</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PEC Error flag clear</description>
          <name>PECCF</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Overrun/Underrun flag
                                clear
                            </description>
          <name>OVRCF</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Arbitration lost flag
                                clear
                            </description>
          <name>ARLOCF</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Bus error flag clear</description>
          <name>BERRCF</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Stop detection flag clear</description>
          <name>STOPCF</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Not Acknowledge flag clear</description>
          <name>NACKCF</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Address Matched flag clear</description>
          <name>ADDRCF</name>
        </field>
      </fields>
      <name>ICR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x20</addressOffset>
      <description>PEC register</description>
      <displayName>PECR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>
                                Packet error checking
                                register
                            </description>
          <name>PEC</name>
        </field>
      </fields>
      <name>PECR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x24</addressOffset>
      <description>Receive data register</description>
      <displayName>RXDR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>8-bit receive data</description>
          <name>RXDATA</name>
        </field>
      </fields>
      <name>RXDR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28</addressOffset>
      <description>Transmit data register</description>
      <displayName>TXDR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>8-bit transmit data</description>
          <name>TXDATA</name>
        </field>
      </fields>
      <name>TXDR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
