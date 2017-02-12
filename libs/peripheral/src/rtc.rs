/// time register
pub mod tr {
    pub struct ReadonlyCache {
        /// AM/PM notation
        pub pm: bool,
        /// Hour tens in BCD format
        pub ht: bool,
        /// Hour units in BCD format
        pub hu: bool,
        /// Minute tens in BCD format
        pub mnt: bool,
        /// Minute units in BCD format
        pub mnu: bool,
        /// Second tens in BCD format
        pub st: bool,
        /// Second units in BCD format
        pub su: bool,
    }
    pub struct Cache {
        /// AM/PM notation
        pub pm: bool,
        /// Hour tens in BCD format
        pub ht: bool,
        /// Hour units in BCD format
        pub hu: bool,
        /// Minute tens in BCD format
        pub mnt: bool,
        /// Minute units in BCD format
        pub mnu: bool,
        /// Second tens in BCD format
        pub st: bool,
        /// Second units in BCD format
        pub su: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            pm: ((value >> 22) & 0b1) > 0,
            ht: ((value >> 20) & 0b1) > 0,
            hu: ((value >> 16) & 0b1) > 0,
            mnt: ((value >> 12) & 0b1) > 0,
            mnu: ((value >> 8) & 0b1) > 0,
            st: ((value >> 4) & 0b1) > 0,
            su: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x0u32) as *mut u32) };
        Cache {
            pm: ((value >> 22) & 0b1) > 0,
            ht: ((value >> 20) & 0b1) > 0,
            hu: ((value >> 16) & 0b1) > 0,
            mnt: ((value >> 12) & 0b1) > 0,
            mnu: ((value >> 8) & 0b1) > 0,
            st: ((value >> 4) & 0b1) > 0,
            su: ((value >> 0) & 0b1) > 0,
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
                | ((self.pm as u32) << 22)
                | ((self.ht as u32) << 20)
                | ((self.hu as u32) << 16)
                | ((self.mnt as u32) << 12)
                | ((self.mnu as u32) << 8)
                | ((self.st as u32) << 4)
                | ((self.su as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// date register
pub mod dr {
    pub struct ReadonlyCache {
        /// Year tens in BCD format
        pub yt: u8,
        /// Year units in BCD format
        pub yu: u8,
        /// Week day units
        pub wdu: u8,
        /// Month tens in BCD format
        pub mt: u8,
        /// Month units in BCD format
        pub mu: u8,
        /// Date tens in BCD format
        pub dt: u8,
        /// Date units in BCD format
        pub du: u8,
    }
    pub struct Cache {
        /// Year tens in BCD format
        pub yt: u8,
        /// Year units in BCD format
        pub yu: u8,
        /// Week day units
        pub wdu: u8,
        /// Month tens in BCD format
        pub mt: u8,
        /// Month units in BCD format
        pub mu: u8,
        /// Date tens in BCD format
        pub dt: u8,
        /// Date units in BCD format
        pub du: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            yt: ((value >> 20) & 0b1111) as u8,
            yu: ((value >> 16) & 0b1111) as u8,
            wdu: ((value >> 13) & 0b1111) as u8,
            mt: ((value >> 12) & 0b1111) as u8,
            mu: ((value >> 8) & 0b1111) as u8,
            dt: ((value >> 4) & 0b1111) as u8,
            du: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x4u32) as *mut u32) };
        Cache {
            yt: ((value >> 20) & 0b1111) as u8,
            yu: ((value >> 16) & 0b1111) as u8,
            wdu: ((value >> 13) & 0b1111) as u8,
            mt: ((value >> 12) & 0b1111) as u8,
            mu: ((value >> 8) & 0b1111) as u8,
            dt: ((value >> 4) & 0b1111) as u8,
            du: ((value >> 0) & 0b1111) as u8,
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
                | ((self.yt as u32) << 20)
                | ((self.yu as u32) << 16)
                | ((self.wdu as u32) << 13)
                | ((self.mt as u32) << 12)
                | ((self.mu as u32) << 8)
                | ((self.dt as u32) << 4)
                | ((self.du as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// control register
pub mod cr {
    pub struct ReadonlyCache {
        /// Wakeup clock selection
        pub wcksel: u8,
        /// Time-stamp event active edge
        pub tsedge: u8,
        /// Reference clock detection enable (50 or 60 Hz)
        pub refckon: u8,
        /// Bypass the shadow registers
        pub bypshad: u8,
        /// Hour format
        pub fmt: u8,
        /// Alarm A enable
        pub alrae: u8,
        /// Alarm B enable
        pub alrbe: u8,
        /// Wakeup timer enable
        pub wute: u8,
        /// Time stamp enable
        pub tse: u8,
        /// Alarm A interrupt enable
        pub alraie: u8,
        /// Alarm B interrupt enable
        pub alrbie: u8,
        /// Wakeup timer interrupt enable
        pub wutie: u8,
        /// Time-stamp interrupt enable
        pub tsie: u8,
        /// Add 1 hour (summer time change)
        pub add1h: u8,
        /// Subtract 1 hour (winter time change)
        pub sub1h: u8,
        /// Backup
        pub bkp: u8,
        /// Calibration output selection
        pub cosel: u8,
        /// Output polarity
        pub pol: u8,
        /// Output selection
        pub osel: u8,
        /// Calibration output enable
        pub coe: u8,
    }
    pub struct Cache {
        /// Wakeup clock selection
        pub wcksel: u8,
        /// Time-stamp event active edge
        pub tsedge: u8,
        /// Reference clock detection enable (50 or 60 Hz)
        pub refckon: u8,
        /// Bypass the shadow registers
        pub bypshad: u8,
        /// Hour format
        pub fmt: u8,
        /// Alarm A enable
        pub alrae: u8,
        /// Alarm B enable
        pub alrbe: u8,
        /// Wakeup timer enable
        pub wute: u8,
        /// Time stamp enable
        pub tse: u8,
        /// Alarm A interrupt enable
        pub alraie: u8,
        /// Alarm B interrupt enable
        pub alrbie: u8,
        /// Wakeup timer interrupt enable
        pub wutie: u8,
        /// Time-stamp interrupt enable
        pub tsie: u8,
        /// Add 1 hour (summer time change)
        pub add1h: u8,
        /// Subtract 1 hour (winter time change)
        pub sub1h: u8,
        /// Backup
        pub bkp: u8,
        /// Calibration output selection
        pub cosel: u8,
        /// Output polarity
        pub pol: u8,
        /// Output selection
        pub osel: u8,
        /// Calibration output enable
        pub coe: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            wcksel: ((value >> 0) & 0b111) as u8,
            tsedge: ((value >> 3) & 0b111) as u8,
            refckon: ((value >> 4) & 0b111) as u8,
            bypshad: ((value >> 5) & 0b111) as u8,
            fmt: ((value >> 6) & 0b111) as u8,
            alrae: ((value >> 8) & 0b111) as u8,
            alrbe: ((value >> 9) & 0b111) as u8,
            wute: ((value >> 10) & 0b111) as u8,
            tse: ((value >> 11) & 0b111) as u8,
            alraie: ((value >> 12) & 0b111) as u8,
            alrbie: ((value >> 13) & 0b111) as u8,
            wutie: ((value >> 14) & 0b111) as u8,
            tsie: ((value >> 15) & 0b111) as u8,
            add1h: ((value >> 16) & 0b111) as u8,
            sub1h: ((value >> 17) & 0b111) as u8,
            bkp: ((value >> 18) & 0b111) as u8,
            cosel: ((value >> 19) & 0b111) as u8,
            pol: ((value >> 20) & 0b111) as u8,
            osel: ((value >> 21) & 0b111) as u8,
            coe: ((value >> 23) & 0b111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x8u32) as *mut u32) };
        Cache {
            wcksel: ((value >> 0) & 0b111) as u8,
            tsedge: ((value >> 3) & 0b111) as u8,
            refckon: ((value >> 4) & 0b111) as u8,
            bypshad: ((value >> 5) & 0b111) as u8,
            fmt: ((value >> 6) & 0b111) as u8,
            alrae: ((value >> 8) & 0b111) as u8,
            alrbe: ((value >> 9) & 0b111) as u8,
            wute: ((value >> 10) & 0b111) as u8,
            tse: ((value >> 11) & 0b111) as u8,
            alraie: ((value >> 12) & 0b111) as u8,
            alrbie: ((value >> 13) & 0b111) as u8,
            wutie: ((value >> 14) & 0b111) as u8,
            tsie: ((value >> 15) & 0b111) as u8,
            add1h: ((value >> 16) & 0b111) as u8,
            sub1h: ((value >> 17) & 0b111) as u8,
            bkp: ((value >> 18) & 0b111) as u8,
            cosel: ((value >> 19) & 0b111) as u8,
            pol: ((value >> 20) & 0b111) as u8,
            osel: ((value >> 21) & 0b111) as u8,
            coe: ((value >> 23) & 0b111) as u8,
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
                | ((self.wcksel as u32) << 0)
                | ((self.tsedge as u32) << 3)
                | ((self.refckon as u32) << 4)
                | ((self.bypshad as u32) << 5)
                | ((self.fmt as u32) << 6)
                | ((self.alrae as u32) << 8)
                | ((self.alrbe as u32) << 9)
                | ((self.wute as u32) << 10)
                | ((self.tse as u32) << 11)
                | ((self.alraie as u32) << 12)
                | ((self.alrbie as u32) << 13)
                | ((self.wutie as u32) << 14)
                | ((self.tsie as u32) << 15)
                | ((self.add1h as u32) << 16)
                | ((self.sub1h as u32) << 17)
                | ((self.bkp as u32) << 18)
                | ((self.cosel as u32) << 19)
                | ((self.pol as u32) << 20)
                | ((self.osel as u32) << 21)
                | ((self.coe as u32) << 23)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// initialization and status register
pub mod isr {
    /// Alarm A write flag
    /// Access: read-only, Width: 1, Offset: 0
    /// Get Alarm A write flag
    pub fn alrawf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Alarm B write flag
    /// Access: read-only, Width: 1, Offset: 1
    /// Get Alarm B write flag
    pub fn alrbwf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// Wakeup timer write flag
    /// Access: read-only, Width: 1, Offset: 2
    /// Get Wakeup timer write flag
    pub fn wutwf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// Shift operation pending
    /// Access: read-write, Width: 1, Offset: 3
    /// Set Shift operation pending
    pub fn set_shpf(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Shift operation pending
    pub fn get_shpf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// Initialization status flag
    /// Access: read-only, Width: 1, Offset: 4
    /// Get Initialization status flag
    pub fn inits() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// Registers synchronization flag
    /// Access: read-write, Width: 1, Offset: 5
    /// Set Registers synchronization flag
    pub fn set_rsf(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Registers synchronization flag
    pub fn get_rsf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// Initialization flag
    /// Access: read-only, Width: 1, Offset: 6
    /// Get Initialization flag
    pub fn initf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Initialization mode
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Initialization mode
    pub fn set_init(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Initialization mode
    pub fn get_init() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Alarm A flag
    /// Access: read-write, Width: 1, Offset: 8
    /// Set Alarm A flag
    pub fn set_alraf(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Alarm A flag
    pub fn get_alraf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Alarm B flag
    /// Access: read-write, Width: 1, Offset: 9
    /// Set Alarm B flag
    pub fn set_alrbf(value: bool) {
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Alarm B flag
    pub fn get_alrbf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// Wakeup timer flag
    /// Access: read-write, Width: 1, Offset: 10
    /// Set Wakeup timer flag
    pub fn set_wutf(value: bool) {
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Wakeup timer flag
    pub fn get_wutf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 10);
        value > 0
    }
    /// Time-stamp flag
    /// Access: read-write, Width: 1, Offset: 11
    /// Set Time-stamp flag
    pub fn set_tsf(value: bool) {
        let value = value as u32;
        let value = value << 11;
        unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Time-stamp flag
    pub fn get_tsf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Time-stamp overflow flag
    /// Access: read-write, Width: 1, Offset: 12
    /// Set Time-stamp overflow flag
    pub fn set_tsovf(value: bool) {
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Time-stamp overflow flag
    pub fn get_tsovf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 12);
        value > 0
    }
    /// Tamper detection flag
    /// Access: read-write, Width: 1, Offset: 13
    /// Set Tamper detection flag
    pub fn set_tamp1f(value: bool) {
        let value = value as u32;
        let value = value << 13;
        unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Tamper detection flag
    pub fn get_tamp1f() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 13);
        value > 0
    }
    /// RTC_TAMP2 detection flag
    /// Access: read-write, Width: 1, Offset: 14
    /// Set RTC_TAMP2 detection flag
    pub fn set_tamp2f(value: bool) {
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get RTC_TAMP2 detection flag
    pub fn get_tamp2f() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// RTC_TAMP3 detection flag
    /// Access: read-write, Width: 1, Offset: 15
    /// Set RTC_TAMP3 detection flag
    pub fn set_tamp3f(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get RTC_TAMP3 detection flag
    pub fn get_tamp3f() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// Recalibration pending Flag
    /// Access: read-only, Width: 1, Offset: 16
    /// Get Recalibration pending Flag
    pub fn recalpf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 16);
        value > 0
    }
}
/// prescaler register
pub mod prer {
    pub struct ReadonlyCache {
        /// Asynchronous prescaler factor
        pub prediv_a: u8,
        /// Synchronous prescaler factor
        pub prediv_s: u8,
    }
    pub struct Cache {
        /// Asynchronous prescaler factor
        pub prediv_a: u8,
        /// Synchronous prescaler factor
        pub prediv_s: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            prediv_a: ((value >> 16) & 0b1111111) as u8,
            prediv_s: ((value >> 0) & 0b1111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x10u32) as *mut u32) };
        Cache {
            prediv_a: ((value >> 16) & 0b1111111) as u8,
            prediv_s: ((value >> 0) & 0b1111111) as u8,
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
                | ((self.prediv_a as u32) << 16)
                | ((self.prediv_s as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x10u32) as *mut u32, value) };
        }
    }
}
/// wakeup timer register
pub mod wutr {
    pub struct ReadonlyCache {
        /// Wakeup auto-reload value bits
        pub wut: u16,
    }
    pub struct Cache {
        /// Wakeup auto-reload value bits
        pub wut: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x14u32) as *mut u32) };
        ReadonlyCache {
            wut: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x14u32) as *mut u32) };
        Cache {
            wut: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.wut as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x14u32) as *mut u32, value) };
        }
    }
}
/// alarm A register
pub mod alrmar {
    pub struct ReadonlyCache {
        /// Alarm A date mask
        pub msk4: bool,
        /// Week day selection
        pub wdsel: bool,
        /// Date tens in BCD format
        pub dt: bool,
        /// Date units or day in BCD format
        pub du: bool,
        /// Alarm A hours mask
        pub msk3: bool,
        /// AM/PM notation
        pub pm: bool,
        /// Hour tens in BCD format
        pub ht: bool,
        /// Hour units in BCD format
        pub hu: bool,
        /// Alarm A minutes mask
        pub msk2: bool,
        /// Minute tens in BCD format
        pub mnt: bool,
        /// Minute units in BCD format
        pub mnu: bool,
        /// Alarm A seconds mask
        pub msk1: bool,
        /// Second tens in BCD format
        pub st: bool,
        /// Second units in BCD format
        pub su: bool,
    }
    pub struct Cache {
        /// Alarm A date mask
        pub msk4: bool,
        /// Week day selection
        pub wdsel: bool,
        /// Date tens in BCD format
        pub dt: bool,
        /// Date units or day in BCD format
        pub du: bool,
        /// Alarm A hours mask
        pub msk3: bool,
        /// AM/PM notation
        pub pm: bool,
        /// Hour tens in BCD format
        pub ht: bool,
        /// Hour units in BCD format
        pub hu: bool,
        /// Alarm A minutes mask
        pub msk2: bool,
        /// Minute tens in BCD format
        pub mnt: bool,
        /// Minute units in BCD format
        pub mnu: bool,
        /// Alarm A seconds mask
        pub msk1: bool,
        /// Second tens in BCD format
        pub st: bool,
        /// Second units in BCD format
        pub su: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x1Cu32) as *mut u32) };
        ReadonlyCache {
            msk4: ((value >> 31) & 0b1) > 0,
            wdsel: ((value >> 30) & 0b1) > 0,
            dt: ((value >> 28) & 0b1) > 0,
            du: ((value >> 24) & 0b1) > 0,
            msk3: ((value >> 23) & 0b1) > 0,
            pm: ((value >> 22) & 0b1) > 0,
            ht: ((value >> 20) & 0b1) > 0,
            hu: ((value >> 16) & 0b1) > 0,
            msk2: ((value >> 15) & 0b1) > 0,
            mnt: ((value >> 12) & 0b1) > 0,
            mnu: ((value >> 8) & 0b1) > 0,
            msk1: ((value >> 7) & 0b1) > 0,
            st: ((value >> 4) & 0b1) > 0,
            su: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x1Cu32) as *mut u32) };
        Cache {
            msk4: ((value >> 31) & 0b1) > 0,
            wdsel: ((value >> 30) & 0b1) > 0,
            dt: ((value >> 28) & 0b1) > 0,
            du: ((value >> 24) & 0b1) > 0,
            msk3: ((value >> 23) & 0b1) > 0,
            pm: ((value >> 22) & 0b1) > 0,
            ht: ((value >> 20) & 0b1) > 0,
            hu: ((value >> 16) & 0b1) > 0,
            msk2: ((value >> 15) & 0b1) > 0,
            mnt: ((value >> 12) & 0b1) > 0,
            mnu: ((value >> 8) & 0b1) > 0,
            msk1: ((value >> 7) & 0b1) > 0,
            st: ((value >> 4) & 0b1) > 0,
            su: ((value >> 0) & 0b1) > 0,
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
                | ((self.msk4 as u32) << 31)
                | ((self.wdsel as u32) << 30)
                | ((self.dt as u32) << 28)
                | ((self.du as u32) << 24)
                | ((self.msk3 as u32) << 23)
                | ((self.pm as u32) << 22)
                | ((self.ht as u32) << 20)
                | ((self.hu as u32) << 16)
                | ((self.msk2 as u32) << 15)
                | ((self.mnt as u32) << 12)
                | ((self.mnu as u32) << 8)
                | ((self.msk1 as u32) << 7)
                | ((self.st as u32) << 4)
                | ((self.su as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x1Cu32) as *mut u32, value) };
        }
    }
}
/// alarm B register
pub mod alrmbr {
    pub struct ReadonlyCache {
        /// Alarm B date mask
        pub msk4: bool,
        /// Week day selection
        pub wdsel: bool,
        /// Date tens in BCD format
        pub dt: bool,
        /// Date units or day in BCD format
        pub du: bool,
        /// Alarm B hours mask
        pub msk3: bool,
        /// AM/PM notation
        pub pm: bool,
        /// Hour tens in BCD format
        pub ht: bool,
        /// Hour units in BCD format
        pub hu: bool,
        /// Alarm B minutes mask
        pub msk2: bool,
        /// Minute tens in BCD format
        pub mnt: bool,
        /// Minute units in BCD format
        pub mnu: bool,
        /// Alarm B seconds mask
        pub msk1: bool,
        /// Second tens in BCD format
        pub st: bool,
        /// Second units in BCD format
        pub su: bool,
    }
    pub struct Cache {
        /// Alarm B date mask
        pub msk4: bool,
        /// Week day selection
        pub wdsel: bool,
        /// Date tens in BCD format
        pub dt: bool,
        /// Date units or day in BCD format
        pub du: bool,
        /// Alarm B hours mask
        pub msk3: bool,
        /// AM/PM notation
        pub pm: bool,
        /// Hour tens in BCD format
        pub ht: bool,
        /// Hour units in BCD format
        pub hu: bool,
        /// Alarm B minutes mask
        pub msk2: bool,
        /// Minute tens in BCD format
        pub mnt: bool,
        /// Minute units in BCD format
        pub mnu: bool,
        /// Alarm B seconds mask
        pub msk1: bool,
        /// Second tens in BCD format
        pub st: bool,
        /// Second units in BCD format
        pub su: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x20u32) as *mut u32) };
        ReadonlyCache {
            msk4: ((value >> 31) & 0b1) > 0,
            wdsel: ((value >> 30) & 0b1) > 0,
            dt: ((value >> 28) & 0b1) > 0,
            du: ((value >> 24) & 0b1) > 0,
            msk3: ((value >> 23) & 0b1) > 0,
            pm: ((value >> 22) & 0b1) > 0,
            ht: ((value >> 20) & 0b1) > 0,
            hu: ((value >> 16) & 0b1) > 0,
            msk2: ((value >> 15) & 0b1) > 0,
            mnt: ((value >> 12) & 0b1) > 0,
            mnu: ((value >> 8) & 0b1) > 0,
            msk1: ((value >> 7) & 0b1) > 0,
            st: ((value >> 4) & 0b1) > 0,
            su: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x20u32) as *mut u32) };
        Cache {
            msk4: ((value >> 31) & 0b1) > 0,
            wdsel: ((value >> 30) & 0b1) > 0,
            dt: ((value >> 28) & 0b1) > 0,
            du: ((value >> 24) & 0b1) > 0,
            msk3: ((value >> 23) & 0b1) > 0,
            pm: ((value >> 22) & 0b1) > 0,
            ht: ((value >> 20) & 0b1) > 0,
            hu: ((value >> 16) & 0b1) > 0,
            msk2: ((value >> 15) & 0b1) > 0,
            mnt: ((value >> 12) & 0b1) > 0,
            mnu: ((value >> 8) & 0b1) > 0,
            msk1: ((value >> 7) & 0b1) > 0,
            st: ((value >> 4) & 0b1) > 0,
            su: ((value >> 0) & 0b1) > 0,
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
                | ((self.msk4 as u32) << 31)
                | ((self.wdsel as u32) << 30)
                | ((self.dt as u32) << 28)
                | ((self.du as u32) << 24)
                | ((self.msk3 as u32) << 23)
                | ((self.pm as u32) << 22)
                | ((self.ht as u32) << 20)
                | ((self.hu as u32) << 16)
                | ((self.msk2 as u32) << 15)
                | ((self.mnt as u32) << 12)
                | ((self.mnu as u32) << 8)
                | ((self.msk1 as u32) << 7)
                | ((self.st as u32) << 4)
                | ((self.su as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x20u32) as *mut u32, value) };
        }
    }
}
/// write protection register
pub mod wpr {
    /// Write protection key
    /// Access: write-only, Width: 8, Offset: 0
    /// Set Write protection key
    pub fn key(value: u8) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x24u32) as *mut u32, value) };
    }
}
/// sub second register
pub mod ssr {
    /// Sub second value
    /// Access: read-only, Width: 16, Offset: 0
    /// Get Sub second value
    pub fn ss() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x28u32) as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// shift control register
pub mod shiftr {
    /// Add one second
    /// Access: write-only, Width: 1, Offset: 31
    /// Set Add one second
    pub fn add1s(value: bool) {
        let value = value as u32;
        let value = value << 31;
        unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x2Cu32) as *mut u32, value) };
    }
    /// Subtract a fraction of a second
    /// Access: write-only, Width: 15, Offset: 0
    /// Set Subtract a fraction of a second
    pub fn subfs(value: u16) {
        debug_assert!(value <= 0b111111111111111, "subfs out of range");
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x2Cu32) as *mut u32, value) };
    }
}
/// time stamp time register
pub mod tstr {
    /// Second units in BCD format
    /// Access: read-only, Width: 4, Offset: 0
    /// Get Second units in BCD format
    pub fn su() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x30u32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
    /// Second tens in BCD format
    /// Access: read-only, Width: 3, Offset: 4
    /// Get Second tens in BCD format
    pub fn st() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x30u32) as *mut u32) };
        let value = value & (0b111 << 4);
        value as u8
    }
    /// Minute units in BCD format
    /// Access: read-only, Width: 4, Offset: 8
    /// Get Minute units in BCD format
    pub fn mnu() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x30u32) as *mut u32) };
        let value = value & (0b1111 << 8);
        value as u8
    }
    /// Minute tens in BCD format
    /// Access: read-only, Width: 3, Offset: 12
    /// Get Minute tens in BCD format
    pub fn mnt() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x30u32) as *mut u32) };
        let value = value & (0b111 << 12);
        value as u8
    }
    /// Hour units in BCD format
    /// Access: read-only, Width: 4, Offset: 16
    /// Get Hour units in BCD format
    pub fn hu() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x30u32) as *mut u32) };
        let value = value & (0b1111 << 16);
        value as u8
    }
    /// Hour tens in BCD format
    /// Access: read-only, Width: 2, Offset: 20
    /// Get Hour tens in BCD format
    pub fn ht() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x30u32) as *mut u32) };
        let value = value & (0b11 << 20);
        value as u8
    }
    /// AM/PM notation
    /// Access: read-only, Width: 1, Offset: 22
    /// Get AM/PM notation
    pub fn pm() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x30u32) as *mut u32) };
        let value = value & (0b1 << 22);
        value > 0
    }
}
/// time stamp date register
pub mod tsdr {
    /// Week day units
    /// Access: read-only, Width: 3, Offset: 13
    /// Get Week day units
    pub fn wdu() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x34u32) as *mut u32) };
        let value = value & (0b111 << 13);
        value as u8
    }
    /// Month tens in BCD format
    /// Access: read-only, Width: 1, Offset: 12
    /// Get Month tens in BCD format
    pub fn mt() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x34u32) as *mut u32) };
        let value = value & (0b1 << 12);
        value > 0
    }
    /// Month units in BCD format
    /// Access: read-only, Width: 4, Offset: 8
    /// Get Month units in BCD format
    pub fn mu() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x34u32) as *mut u32) };
        let value = value & (0b1111 << 8);
        value as u8
    }
    /// Date tens in BCD format
    /// Access: read-only, Width: 2, Offset: 4
    /// Get Date tens in BCD format
    pub fn dt() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x34u32) as *mut u32) };
        let value = value & (0b11 << 4);
        value as u8
    }
    /// Date units in BCD format
    /// Access: read-only, Width: 4, Offset: 0
    /// Get Date units in BCD format
    pub fn du() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x34u32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
}
/// timestamp sub second register
pub mod tsssr {
    /// Sub second value
    /// Access: read-only, Width: 16, Offset: 0
    /// Get Sub second value
    pub fn ss() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x38u32) as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// calibration register
pub mod calr {
    pub struct ReadonlyCache {
        /// Increase frequency of RTC by 488.5 ppm
        pub calp: bool,
        /// Use an 8-second calibration cycle period
        pub calw8: bool,
        /// Use a 16-second calibration cycle period
        pub calw16: bool,
        /// Calibration minus
        pub calm: bool,
    }
    pub struct Cache {
        /// Increase frequency of RTC by 488.5 ppm
        pub calp: bool,
        /// Use an 8-second calibration cycle period
        pub calw8: bool,
        /// Use a 16-second calibration cycle period
        pub calw16: bool,
        /// Calibration minus
        pub calm: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x3Cu32) as *mut u32) };
        ReadonlyCache {
            calp: ((value >> 15) & 0b1) > 0,
            calw8: ((value >> 14) & 0b1) > 0,
            calw16: ((value >> 13) & 0b1) > 0,
            calm: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x3Cu32) as *mut u32) };
        Cache {
            calp: ((value >> 15) & 0b1) > 0,
            calw8: ((value >> 14) & 0b1) > 0,
            calw16: ((value >> 13) & 0b1) > 0,
            calm: ((value >> 0) & 0b1) > 0,
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
                | ((self.calp as u32) << 15)
                | ((self.calw8 as u32) << 14)
                | ((self.calw16 as u32) << 13)
                | ((self.calm as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x3Cu32) as *mut u32, value) };
        }
    }
}
/// tamper and alternate function configuration register
pub mod tafcr {
    pub struct ReadonlyCache {
        /// Tamper 1 detection enable
        pub tamp1e: bool,
        /// Active level for tamper 1
        pub tamp1trg: bool,
        /// Tamper interrupt enable
        pub tampie: bool,
        /// Tamper 2 detection enable
        pub tamp2e: bool,
        /// Active level for tamper 2
        pub tamp2trg: bool,
        /// Tamper 3 detection enable
        pub tamp3e: bool,
        /// Active level for tamper 3
        pub tamp3trg: bool,
        /// Activate timestamp on tamper detection event
        pub tampts: bool,
        /// Tamper sampling frequency
        pub tampfreq: bool,
        /// Tamper filter count
        pub tampflt: bool,
        /// Tamper precharge duration
        pub tampprch: bool,
        /// TAMPER pull-up disable
        pub tamppudis: bool,
        /// PC13 value
        pub pc13value: bool,
        /// PC13 mode
        pub pc13mode: bool,
        /// PC14 value
        pub pc14value: bool,
        /// PC 14 mode
        pub pc14mode: bool,
        /// PC15 value
        pub pc15value: bool,
        /// PC15 mode
        pub pc15mode: bool,
    }
    pub struct Cache {
        /// Tamper 1 detection enable
        pub tamp1e: bool,
        /// Active level for tamper 1
        pub tamp1trg: bool,
        /// Tamper interrupt enable
        pub tampie: bool,
        /// Tamper 2 detection enable
        pub tamp2e: bool,
        /// Active level for tamper 2
        pub tamp2trg: bool,
        /// Tamper 3 detection enable
        pub tamp3e: bool,
        /// Active level for tamper 3
        pub tamp3trg: bool,
        /// Activate timestamp on tamper detection event
        pub tampts: bool,
        /// Tamper sampling frequency
        pub tampfreq: bool,
        /// Tamper filter count
        pub tampflt: bool,
        /// Tamper precharge duration
        pub tampprch: bool,
        /// TAMPER pull-up disable
        pub tamppudis: bool,
        /// PC13 value
        pub pc13value: bool,
        /// PC13 mode
        pub pc13mode: bool,
        /// PC14 value
        pub pc14value: bool,
        /// PC 14 mode
        pub pc14mode: bool,
        /// PC15 value
        pub pc15value: bool,
        /// PC15 mode
        pub pc15mode: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x40u32) as *mut u32) };
        ReadonlyCache {
            tamp1e: ((value >> 0) & 0b1) > 0,
            tamp1trg: ((value >> 1) & 0b1) > 0,
            tampie: ((value >> 2) & 0b1) > 0,
            tamp2e: ((value >> 3) & 0b1) > 0,
            tamp2trg: ((value >> 4) & 0b1) > 0,
            tamp3e: ((value >> 5) & 0b1) > 0,
            tamp3trg: ((value >> 6) & 0b1) > 0,
            tampts: ((value >> 7) & 0b1) > 0,
            tampfreq: ((value >> 8) & 0b1) > 0,
            tampflt: ((value >> 11) & 0b1) > 0,
            tampprch: ((value >> 13) & 0b1) > 0,
            tamppudis: ((value >> 15) & 0b1) > 0,
            pc13value: ((value >> 18) & 0b1) > 0,
            pc13mode: ((value >> 19) & 0b1) > 0,
            pc14value: ((value >> 20) & 0b1) > 0,
            pc14mode: ((value >> 21) & 0b1) > 0,
            pc15value: ((value >> 22) & 0b1) > 0,
            pc15mode: ((value >> 23) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x40u32) as *mut u32) };
        Cache {
            tamp1e: ((value >> 0) & 0b1) > 0,
            tamp1trg: ((value >> 1) & 0b1) > 0,
            tampie: ((value >> 2) & 0b1) > 0,
            tamp2e: ((value >> 3) & 0b1) > 0,
            tamp2trg: ((value >> 4) & 0b1) > 0,
            tamp3e: ((value >> 5) & 0b1) > 0,
            tamp3trg: ((value >> 6) & 0b1) > 0,
            tampts: ((value >> 7) & 0b1) > 0,
            tampfreq: ((value >> 8) & 0b1) > 0,
            tampflt: ((value >> 11) & 0b1) > 0,
            tampprch: ((value >> 13) & 0b1) > 0,
            tamppudis: ((value >> 15) & 0b1) > 0,
            pc13value: ((value >> 18) & 0b1) > 0,
            pc13mode: ((value >> 19) & 0b1) > 0,
            pc14value: ((value >> 20) & 0b1) > 0,
            pc14mode: ((value >> 21) & 0b1) > 0,
            pc15value: ((value >> 22) & 0b1) > 0,
            pc15mode: ((value >> 23) & 0b1) > 0,
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
                | ((self.tamp1e as u32) << 0)
                | ((self.tamp1trg as u32) << 1)
                | ((self.tampie as u32) << 2)
                | ((self.tamp2e as u32) << 3)
                | ((self.tamp2trg as u32) << 4)
                | ((self.tamp3e as u32) << 5)
                | ((self.tamp3trg as u32) << 6)
                | ((self.tampts as u32) << 7)
                | ((self.tampfreq as u32) << 8)
                | ((self.tampflt as u32) << 11)
                | ((self.tampprch as u32) << 13)
                | ((self.tamppudis as u32) << 15)
                | ((self.pc13value as u32) << 18)
                | ((self.pc13mode as u32) << 19)
                | ((self.pc14value as u32) << 20)
                | ((self.pc14mode as u32) << 21)
                | ((self.pc15value as u32) << 22)
                | ((self.pc15mode as u32) << 23)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x40u32) as *mut u32, value) };
        }
    }
}
/// alarm A sub second register
pub mod alrmassr {
    pub struct ReadonlyCache {
        /// Mask the most-significant bits starting at this bit
        pub maskss: u8,
        /// Sub seconds value
        pub ss: u8,
    }
    pub struct Cache {
        /// Mask the most-significant bits starting at this bit
        pub maskss: u8,
        /// Sub seconds value
        pub ss: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x44u32) as *mut u32) };
        ReadonlyCache {
            maskss: ((value >> 24) & 0b1111) as u8,
            ss: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x44u32) as *mut u32) };
        Cache {
            maskss: ((value >> 24) & 0b1111) as u8,
            ss: ((value >> 0) & 0b1111) as u8,
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
                | ((self.maskss as u32) << 24)
                | ((self.ss as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x44u32) as *mut u32, value) };
        }
    }
}
/// alarm B sub second register
pub mod alrmbssr {
    pub struct ReadonlyCache {
        /// Mask the most-significant bits starting at this bit
        pub maskss: u8,
        /// Sub seconds value
        pub ss: u8,
    }
    pub struct Cache {
        /// Mask the most-significant bits starting at this bit
        pub maskss: u8,
        /// Sub seconds value
        pub ss: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x48u32) as *mut u32) };
        ReadonlyCache {
            maskss: ((value >> 24) & 0b1111) as u8,
            ss: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x48u32) as *mut u32) };
        Cache {
            maskss: ((value >> 24) & 0b1111) as u8,
            ss: ((value >> 0) & 0b1111) as u8,
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
                | ((self.maskss as u32) << 24)
                | ((self.ss as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x48u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp0r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x50u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x50u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x50u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp1r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x54u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x54u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x54u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp2r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x58u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x58u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x58u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp3r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x5Cu32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x5Cu32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x5Cu32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp4r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x60u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x60u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x60u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp5r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x64u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x64u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x64u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp6r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x68u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x68u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x68u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp7r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x6Cu32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x6Cu32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x6Cu32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp8r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x70u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x70u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x70u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp9r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x74u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x74u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x74u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp10r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x78u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x78u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x78u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp11r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x7Cu32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x7Cu32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x7Cu32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp12r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x80u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x80u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x80u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp13r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x84u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x84u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x84u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp14r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x88u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x88u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x88u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp15r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x8Cu32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x8Cu32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x8Cu32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp16r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x90u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x90u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x90u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp17r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x94u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x94u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x94u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp18r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x98u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x98u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x98u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp19r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x9Cu32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0x9Cu32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0x9Cu32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp20r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xA0u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xA0u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xA0u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp21r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xA4u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xA4u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xA4u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp22r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xA8u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xA8u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xA8u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp23r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xACu32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xACu32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xACu32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp24r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xB0u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xB0u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xB0u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp25r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xB4u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xB4u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xB4u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp26r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xB8u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xB8u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xB8u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp27r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xBCu32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xBCu32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xBCu32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp28r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xC0u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xC0u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xC0u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp29r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xC4u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xC4u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xC4u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp30r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xC8u32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xC8u32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xC8u32) as *mut u32, value) };
        }
    }
}
/// backup register
pub mod bkp31r {
    pub struct ReadonlyCache {
        /// BKP
        pub bkp: u32,
    }
    pub struct Cache {
        /// BKP
        pub bkp: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCCu32) as *mut u32) };
        ReadonlyCache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002800u32 + 0xCCu32) as *mut u32) };
        Cache {
            bkp: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.bkp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002800u32 + 0xCCu32) as *mut u32, value) };
        }
    }
}
/// RTC Wakeup interrupt through the EXTI line
pub const INTERRUPT_RTC_WKUP: u32 = 3;
/// RTC alarm interrupt
pub const INTERRUPT_RTCALARM: u32 = 41;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40002800</baseAddress>
  <description>Real-time clock</description>
  <groupName>RTC</groupName>
  <interrupt>
    <description>
                    RTC Wakeup interrupt through the EXTI
                    line
                </description>
    <name>RTC_WKUP</name>
    <value>3</value>
  </interrupt>
  <interrupt>
    <description>RTC alarm interrupt</description>
    <name>RTCAlarm</name>
    <value>41</value>
  </interrupt>
  <name>RTC</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>time register</description>
      <displayName>TR</displayName>
      <fields>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AM/PM notation</description>
          <name>PM</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Hour tens in BCD format</description>
          <name>HT</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Hour units in BCD format</description>
          <name>HU</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Minute tens in BCD format</description>
          <name>MNT</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Minute units in BCD format</description>
          <name>MNU</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Second tens in BCD format</description>
          <name>ST</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Second units in BCD format</description>
          <name>SU</name>
        </field>
      </fields>
      <name>TR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>date register</description>
      <displayName>DR</displayName>
      <fields>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Year tens in BCD format</description>
          <name>YT</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Year units in BCD format</description>
          <name>YU</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Week day units</description>
          <name>WDU</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Month tens in BCD format</description>
          <name>MT</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Month units in BCD format</description>
          <name>MU</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Date tens in BCD format</description>
          <name>DT</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Date units in BCD format</description>
          <name>DU</name>
        </field>
      </fields>
      <name>DR</name>
      <resetValue>0x00002101</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>control register</description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Wakeup clock selection</description>
          <name>WCKSEL</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Time-stamp event active
                                edge
                            </description>
          <name>TSEDGE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Reference clock detection enable (50 or
                                60 Hz)
                            </description>
          <name>REFCKON</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Bypass the shadow
                                registers
                            </description>
          <name>BYPSHAD</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Hour format</description>
          <name>FMT</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm A enable</description>
          <name>ALRAE</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm B enable</description>
          <name>ALRBE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Wakeup timer enable</description>
          <name>WUTE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Time stamp enable</description>
          <name>TSE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm A interrupt enable</description>
          <name>ALRAIE</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm B interrupt enable</description>
          <name>ALRBIE</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Wakeup timer interrupt
                                enable
                            </description>
          <name>WUTIE</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Time-stamp interrupt
                                enable
                            </description>
          <name>TSIE</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Add 1 hour (summer time
                                change)
                            </description>
          <name>ADD1H</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Subtract 1 hour (winter time
                                change)
                            </description>
          <name>SUB1H</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Backup</description>
          <name>BKP</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Calibration output
                                selection
                            </description>
          <name>COSEL</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output polarity</description>
          <name>POL</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Output selection</description>
          <name>OSEL</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Calibration output enable</description>
          <name>COE</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0xC</addressOffset>
      <description>
                        initialization and status
                        register
                    </description>
      <displayName>ISR</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm A write flag</description>
          <name>ALRAWF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm B write flag</description>
          <name>ALRBWF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Wakeup timer write flag</description>
          <name>WUTWF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Shift operation pending</description>
          <name>SHPF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Initialization status flag</description>
          <name>INITS</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Registers synchronization
                                flag
                            </description>
          <name>RSF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Initialization flag</description>
          <name>INITF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Initialization mode</description>
          <name>INIT</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm A flag</description>
          <name>ALRAF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm B flag</description>
          <name>ALRBF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Wakeup timer flag</description>
          <name>WUTF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Time-stamp flag</description>
          <name>TSF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Time-stamp overflow flag</description>
          <name>TSOVF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Tamper detection flag</description>
          <name>TAMP1F</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTC_TAMP2 detection flag</description>
          <name>TAMP2F</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTC_TAMP3 detection flag</description>
          <name>TAMP3F</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Recalibration pending Flag</description>
          <name>RECALPF</name>
        </field>
      </fields>
      <name>ISR</name>
      <resetValue>0x00000007</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>prescaler register</description>
      <displayName>PRER</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>7</bitWidth>
          <description>
                                Asynchronous prescaler
                                factor
                            </description>
          <name>PREDIV_A</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>15</bitWidth>
          <description>
                                Synchronous prescaler
                                factor
                            </description>
          <name>PREDIV_S</name>
        </field>
      </fields>
      <name>PRER</name>
      <resetValue>0x007F00FF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>wakeup timer register</description>
      <displayName>WUTR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>
                                Wakeup auto-reload value
                                bits
                            </description>
          <name>WUT</name>
        </field>
      </fields>
      <name>WUTR</name>
      <resetValue>0x0000FFFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>alarm A register</description>
      <displayName>ALRMAR</displayName>
      <fields>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm A date mask</description>
          <name>MSK4</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Week day selection</description>
          <name>WDSEL</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Date tens in BCD format</description>
          <name>DT</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Date units or day in BCD
                                format
                            </description>
          <name>DU</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm A hours mask</description>
          <name>MSK3</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AM/PM notation</description>
          <name>PM</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Hour tens in BCD format</description>
          <name>HT</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Hour units in BCD format</description>
          <name>HU</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm A minutes mask</description>
          <name>MSK2</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Minute tens in BCD format</description>
          <name>MNT</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Minute units in BCD format</description>
          <name>MNU</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm A seconds mask</description>
          <name>MSK1</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Second tens in BCD format</description>
          <name>ST</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Second units in BCD format</description>
          <name>SU</name>
        </field>
      </fields>
      <name>ALRMAR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>alarm B register</description>
      <displayName>ALRMBR</displayName>
      <fields>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm B date mask</description>
          <name>MSK4</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Week day selection</description>
          <name>WDSEL</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Date tens in BCD format</description>
          <name>DT</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Date units or day in BCD
                                format
                            </description>
          <name>DU</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm B hours mask</description>
          <name>MSK3</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AM/PM notation</description>
          <name>PM</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Hour tens in BCD format</description>
          <name>HT</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Hour units in BCD format</description>
          <name>HU</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm B minutes mask</description>
          <name>MSK2</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Minute tens in BCD format</description>
          <name>MNT</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Minute units in BCD format</description>
          <name>MNU</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm B seconds mask</description>
          <name>MSK1</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Second tens in BCD format</description>
          <name>ST</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Second units in BCD format</description>
          <name>SU</name>
        </field>
      </fields>
      <name>ALRMBR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x24</addressOffset>
      <description>write protection register</description>
      <displayName>WPR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Write protection key</description>
          <name>KEY</name>
        </field>
      </fields>
      <name>WPR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x28</addressOffset>
      <description>sub second register</description>
      <displayName>SSR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Sub second value</description>
          <name>SS</name>
        </field>
      </fields>
      <name>SSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x2C</addressOffset>
      <description>shift control register</description>
      <displayName>SHIFTR</displayName>
      <fields>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Add one second</description>
          <name>ADD1S</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>15</bitWidth>
          <description>
                                Subtract a fraction of a
                                second
                            </description>
          <name>SUBFS</name>
        </field>
      </fields>
      <name>SHIFTR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x30</addressOffset>
      <description>time stamp time register</description>
      <displayName>TSTR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Second units in BCD format</description>
          <name>SU</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Second tens in BCD format</description>
          <name>ST</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Minute units in BCD format</description>
          <name>MNU</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Minute tens in BCD format</description>
          <name>MNT</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Hour units in BCD format</description>
          <name>HU</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Hour tens in BCD format</description>
          <name>HT</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AM/PM notation</description>
          <name>PM</name>
        </field>
      </fields>
      <name>TSTR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x34</addressOffset>
      <description>time stamp date register</description>
      <displayName>TSDR</displayName>
      <fields>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Week day units</description>
          <name>WDU</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Month tens in BCD format</description>
          <name>MT</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Month units in BCD format</description>
          <name>MU</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Date tens in BCD format</description>
          <name>DT</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Date units in BCD format</description>
          <name>DU</name>
        </field>
      </fields>
      <name>TSDR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x38</addressOffset>
      <description>timestamp sub second register</description>
      <displayName>TSSSR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Sub second value</description>
          <name>SS</name>
        </field>
      </fields>
      <name>TSSSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x3C</addressOffset>
      <description>calibration register</description>
      <displayName>CALR</displayName>
      <fields>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Increase frequency of RTC by 488.5
                                ppm
                            </description>
          <name>CALP</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Use an 8-second calibration cycle
                                period
                            </description>
          <name>CALW8</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Use a 16-second calibration cycle
                                period
                            </description>
          <name>CALW16</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>9</bitWidth>
          <description>Calibration minus</description>
          <name>CALM</name>
        </field>
      </fields>
      <name>CALR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x40</addressOffset>
      <description>
                        tamper and alternate function configuration
                        register
                    </description>
      <displayName>TAFCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Tamper 1 detection enable</description>
          <name>TAMP1E</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Active level for tamper 1</description>
          <name>TAMP1TRG</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Tamper interrupt enable</description>
          <name>TAMPIE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Tamper 2 detection enable</description>
          <name>TAMP2E</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Active level for tamper 2</description>
          <name>TAMP2TRG</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Tamper 3 detection enable</description>
          <name>TAMP3E</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Active level for tamper 3</description>
          <name>TAMP3TRG</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Activate timestamp on tamper detection
                                event
                            </description>
          <name>TAMPTS</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Tamper sampling frequency</description>
          <name>TAMPFREQ</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Tamper filter count</description>
          <name>TAMPFLT</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Tamper precharge duration</description>
          <name>TAMPPRCH</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TAMPER pull-up disable</description>
          <name>TAMPPUDIS</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PC13 value</description>
          <name>PC13VALUE</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PC13 mode</description>
          <name>PC13MODE</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PC14 value</description>
          <name>PC14VALUE</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PC 14 mode</description>
          <name>PC14MODE</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PC15 value</description>
          <name>PC15VALUE</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PC15 mode</description>
          <name>PC15MODE</name>
        </field>
      </fields>
      <name>TAFCR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x44</addressOffset>
      <description>alarm A sub second register</description>
      <displayName>ALRMASSR</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Mask the most-significant bits starting
                                at this bit
                            </description>
          <name>MASKSS</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>15</bitWidth>
          <description>Sub seconds value</description>
          <name>SS</name>
        </field>
      </fields>
      <name>ALRMASSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x48</addressOffset>
      <description>alarm B sub second register</description>
      <displayName>ALRMBSSR</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Mask the most-significant bits starting
                                at this bit
                            </description>
          <name>MASKSS</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>15</bitWidth>
          <description>Sub seconds value</description>
          <name>SS</name>
        </field>
      </fields>
      <name>ALRMBSSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x50</addressOffset>
      <description>backup register</description>
      <displayName>BKP0R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x54</addressOffset>
      <description>backup register</description>
      <displayName>BKP1R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x58</addressOffset>
      <description>backup register</description>
      <displayName>BKP2R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP2R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x5C</addressOffset>
      <description>backup register</description>
      <displayName>BKP3R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP3R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x60</addressOffset>
      <description>backup register</description>
      <displayName>BKP4R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP4R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x64</addressOffset>
      <description>backup register</description>
      <displayName>BKP5R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP5R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x68</addressOffset>
      <description>backup register</description>
      <displayName>BKP6R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP6R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x6C</addressOffset>
      <description>backup register</description>
      <displayName>BKP7R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP7R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x70</addressOffset>
      <description>backup register</description>
      <displayName>BKP8R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP8R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x74</addressOffset>
      <description>backup register</description>
      <displayName>BKP9R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP9R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x78</addressOffset>
      <description>backup register</description>
      <displayName>BKP10R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP10R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x7C</addressOffset>
      <description>backup register</description>
      <displayName>BKP11R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP11R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x80</addressOffset>
      <description>backup register</description>
      <displayName>BKP12R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP12R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x84</addressOffset>
      <description>backup register</description>
      <displayName>BKP13R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP13R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x88</addressOffset>
      <description>backup register</description>
      <displayName>BKP14R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP14R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8C</addressOffset>
      <description>backup register</description>
      <displayName>BKP15R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP15R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x90</addressOffset>
      <description>backup register</description>
      <displayName>BKP16R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP16R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x94</addressOffset>
      <description>backup register</description>
      <displayName>BKP17R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP17R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x98</addressOffset>
      <description>backup register</description>
      <displayName>BKP18R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP18R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x9C</addressOffset>
      <description>backup register</description>
      <displayName>BKP19R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP19R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xA0</addressOffset>
      <description>backup register</description>
      <displayName>BKP20R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP20R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xA4</addressOffset>
      <description>backup register</description>
      <displayName>BKP21R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP21R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xA8</addressOffset>
      <description>backup register</description>
      <displayName>BKP22R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP22R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xAC</addressOffset>
      <description>backup register</description>
      <displayName>BKP23R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP23R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xB0</addressOffset>
      <description>backup register</description>
      <displayName>BKP24R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP24R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xB4</addressOffset>
      <description>backup register</description>
      <displayName>BKP25R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP25R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xB8</addressOffset>
      <description>backup register</description>
      <displayName>BKP26R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP26R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xBC</addressOffset>
      <description>backup register</description>
      <displayName>BKP27R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP27R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC0</addressOffset>
      <description>backup register</description>
      <displayName>BKP28R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP28R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC4</addressOffset>
      <description>backup register</description>
      <displayName>BKP29R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP29R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC8</addressOffset>
      <description>backup register</description>
      <displayName>BKP30R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP30R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xCC</addressOffset>
      <description>backup register</description>
      <displayName>BKP31R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>BKP</description>
          <name>BKP</name>
        </field>
      </fields>
      <name>BKP31R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
