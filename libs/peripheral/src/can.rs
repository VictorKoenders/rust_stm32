/// master control register
pub mod mcr {
    pub struct ReadonlyCache {
        /// DBF
        pub dbf: bool,
        /// RESET
        pub reset: bool,
        /// TTCM
        pub ttcm: bool,
        /// ABOM
        pub abom: bool,
        /// AWUM
        pub awum: bool,
        /// NART
        pub nart: bool,
        /// RFLM
        pub rflm: bool,
        /// TXFP
        pub txfp: bool,
        /// SLEEP
        pub sleep: bool,
        /// INRQ
        pub inrq: bool,
    }
    pub struct Cache {
        /// DBF
        pub dbf: bool,
        /// RESET
        pub reset: bool,
        /// TTCM
        pub ttcm: bool,
        /// ABOM
        pub abom: bool,
        /// AWUM
        pub awum: bool,
        /// NART
        pub nart: bool,
        /// RFLM
        pub rflm: bool,
        /// TXFP
        pub txfp: bool,
        /// SLEEP
        pub sleep: bool,
        /// INRQ
        pub inrq: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            dbf: ((value >> 16) & 0b1) > 0,
            reset: ((value >> 15) & 0b1) > 0,
            ttcm: ((value >> 7) & 0b1) > 0,
            abom: ((value >> 6) & 0b1) > 0,
            awum: ((value >> 5) & 0b1) > 0,
            nart: ((value >> 4) & 0b1) > 0,
            rflm: ((value >> 3) & 0b1) > 0,
            txfp: ((value >> 2) & 0b1) > 0,
            sleep: ((value >> 1) & 0b1) > 0,
            inrq: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x0u32) as *mut u32) };
        Cache {
            dbf: ((value >> 16) & 0b1) > 0,
            reset: ((value >> 15) & 0b1) > 0,
            ttcm: ((value >> 7) & 0b1) > 0,
            abom: ((value >> 6) & 0b1) > 0,
            awum: ((value >> 5) & 0b1) > 0,
            nart: ((value >> 4) & 0b1) > 0,
            rflm: ((value >> 3) & 0b1) > 0,
            txfp: ((value >> 2) & 0b1) > 0,
            sleep: ((value >> 1) & 0b1) > 0,
            inrq: ((value >> 0) & 0b1) > 0,
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
                | ((self.dbf as u32) << 16)
                | ((self.reset as u32) << 15)
                | ((self.ttcm as u32) << 7)
                | ((self.abom as u32) << 6)
                | ((self.awum as u32) << 5)
                | ((self.nart as u32) << 4)
                | ((self.rflm as u32) << 3)
                | ((self.txfp as u32) << 2)
                | ((self.sleep as u32) << 1)
                | ((self.inrq as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// master status register
pub mod msr {
    /// RX
    /// Access: read-only, Width: 1, Offset: 11
    /// Get RX
    pub fn rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// SAMP
    /// Access: read-only, Width: 1, Offset: 10
    /// Get SAMP
    pub fn samp() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 10);
        value > 0
    }
    /// RXM
    /// Access: read-only, Width: 1, Offset: 9
    /// Get RXM
    pub fn rxm() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// TXM
    /// Access: read-only, Width: 1, Offset: 8
    /// Get TXM
    pub fn txm() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// SLAKI
    /// Access: read-write, Width: 1, Offset: 4
    /// Set SLAKI
    pub fn set_slaki(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get SLAKI
    pub fn get_slaki() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// WKUI
    /// Access: read-write, Width: 1, Offset: 3
    /// Set WKUI
    pub fn set_wkui(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get WKUI
    pub fn get_wkui() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// ERRI
    /// Access: read-write, Width: 1, Offset: 2
    /// Set ERRI
    pub fn set_erri(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get ERRI
    pub fn get_erri() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// SLAK
    /// Access: read-only, Width: 1, Offset: 1
    /// Get SLAK
    pub fn slak() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// INAK
    /// Access: read-only, Width: 1, Offset: 0
    /// Get INAK
    pub fn inak() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// transmit status register
pub mod tsr {
    /// Lowest priority flag for mailbox 2
    /// Access: read-only, Width: 1, Offset: 31
    /// Get Lowest priority flag for mailbox 2
    pub fn low2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
    /// Lowest priority flag for mailbox 1
    /// Access: read-only, Width: 1, Offset: 30
    /// Get Lowest priority flag for mailbox 1
    pub fn low1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 30);
        value > 0
    }
    /// Lowest priority flag for mailbox 0
    /// Access: read-only, Width: 1, Offset: 29
    /// Get Lowest priority flag for mailbox 0
    pub fn low0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 29);
        value > 0
    }
    /// Lowest priority flag for mailbox 2
    /// Access: read-only, Width: 1, Offset: 28
    /// Get Lowest priority flag for mailbox 2
    pub fn tme2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 28);
        value > 0
    }
    /// Lowest priority flag for mailbox 1
    /// Access: read-only, Width: 1, Offset: 27
    /// Get Lowest priority flag for mailbox 1
    pub fn tme1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 27);
        value > 0
    }
    /// Lowest priority flag for mailbox 0
    /// Access: read-only, Width: 1, Offset: 26
    /// Get Lowest priority flag for mailbox 0
    pub fn tme0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 26);
        value > 0
    }
    /// CODE
    /// Access: read-only, Width: 2, Offset: 24
    /// Get CODE
    pub fn code() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b11 << 24);
        value as u8
    }
    /// ABRQ2
    /// Access: read-write, Width: 1, Offset: 23
    /// Set ABRQ2
    pub fn set_abrq2(value: bool) {
        let value = value as u32;
        let value = value << 23;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get ABRQ2
    pub fn get_abrq2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 23);
        value > 0
    }
    /// TERR2
    /// Access: read-write, Width: 1, Offset: 19
    /// Set TERR2
    pub fn set_terr2(value: bool) {
        let value = value as u32;
        let value = value << 19;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get TERR2
    pub fn get_terr2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 19);
        value > 0
    }
    /// ALST2
    /// Access: read-write, Width: 1, Offset: 18
    /// Set ALST2
    pub fn set_alst2(value: bool) {
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get ALST2
    pub fn get_alst2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 18);
        value > 0
    }
    /// TXOK2
    /// Access: read-write, Width: 1, Offset: 17
    /// Set TXOK2
    pub fn set_txok2(value: bool) {
        let value = value as u32;
        let value = value << 17;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get TXOK2
    pub fn get_txok2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 17);
        value > 0
    }
    /// RQCP2
    /// Access: read-write, Width: 1, Offset: 16
    /// Set RQCP2
    pub fn set_rqcp2(value: bool) {
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get RQCP2
    pub fn get_rqcp2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 16);
        value > 0
    }
    /// ABRQ1
    /// Access: read-write, Width: 1, Offset: 15
    /// Set ABRQ1
    pub fn set_abrq1(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get ABRQ1
    pub fn get_abrq1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// TERR1
    /// Access: read-write, Width: 1, Offset: 11
    /// Set TERR1
    pub fn set_terr1(value: bool) {
        let value = value as u32;
        let value = value << 11;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get TERR1
    pub fn get_terr1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// ALST1
    /// Access: read-write, Width: 1, Offset: 10
    /// Set ALST1
    pub fn set_alst1(value: bool) {
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get ALST1
    pub fn get_alst1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 10);
        value > 0
    }
    /// TXOK1
    /// Access: read-write, Width: 1, Offset: 9
    /// Set TXOK1
    pub fn set_txok1(value: bool) {
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get TXOK1
    pub fn get_txok1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// RQCP1
    /// Access: read-write, Width: 1, Offset: 8
    /// Set RQCP1
    pub fn set_rqcp1(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get RQCP1
    pub fn get_rqcp1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// ABRQ0
    /// Access: read-write, Width: 1, Offset: 7
    /// Set ABRQ0
    pub fn set_abrq0(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get ABRQ0
    pub fn get_abrq0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// TERR0
    /// Access: read-write, Width: 1, Offset: 3
    /// Set TERR0
    pub fn set_terr0(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get TERR0
    pub fn get_terr0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// ALST0
    /// Access: read-write, Width: 1, Offset: 2
    /// Set ALST0
    pub fn set_alst0(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get ALST0
    pub fn get_alst0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// TXOK0
    /// Access: read-write, Width: 1, Offset: 1
    /// Set TXOK0
    pub fn set_txok0(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get TXOK0
    pub fn get_txok0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// RQCP0
    /// Access: read-write, Width: 1, Offset: 0
    /// Set RQCP0
    pub fn set_rqcp0(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get RQCP0
    pub fn get_rqcp0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// receive FIFO 0 register
pub mod rf0r {
    /// RFOM0
    /// Access: read-write, Width: 1, Offset: 5
    /// Set RFOM0
    pub fn set_rfom0(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get RFOM0
    pub fn get_rfom0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// FOVR0
    /// Access: read-write, Width: 1, Offset: 4
    /// Set FOVR0
    pub fn set_fovr0(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get FOVR0
    pub fn get_fovr0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// FULL0
    /// Access: read-write, Width: 1, Offset: 3
    /// Set FULL0
    pub fn set_full0(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get FULL0
    pub fn get_full0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// FMP0
    /// Access: read-only, Width: 2, Offset: 0
    /// Get FMP0
    pub fn fmp0() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0xCu32) as *mut u32) };
        let value = value & (0b11 << 0);
        value as u8
    }
}
/// receive FIFO 1 register
pub mod rf1r {
    /// RFOM1
    /// Access: read-write, Width: 1, Offset: 5
    /// Set RFOM1
    pub fn set_rfom1(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x10u32) as *mut u32, value) };
    }
    /// Get RFOM1
    pub fn get_rfom1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// FOVR1
    /// Access: read-write, Width: 1, Offset: 4
    /// Set FOVR1
    pub fn set_fovr1(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x10u32) as *mut u32, value) };
    }
    /// Get FOVR1
    pub fn get_fovr1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// FULL1
    /// Access: read-write, Width: 1, Offset: 3
    /// Set FULL1
    pub fn set_full1(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x10u32) as *mut u32, value) };
    }
    /// Get FULL1
    pub fn get_full1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// FMP1
    /// Access: read-only, Width: 2, Offset: 0
    /// Get FMP1
    pub fn fmp1() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x10u32) as *mut u32) };
        let value = value & (0b11 << 0);
        value as u8
    }
}
/// interrupt enable register
pub mod ier {
    pub struct ReadonlyCache {
        /// SLKIE
        pub slkie: bool,
        /// WKUIE
        pub wkuie: bool,
        /// ERRIE
        pub errie: bool,
        /// LECIE
        pub lecie: bool,
        /// BOFIE
        pub bofie: bool,
        /// EPVIE
        pub epvie: bool,
        /// EWGIE
        pub ewgie: bool,
        /// FOVIE1
        pub fovie1: bool,
        /// FFIE1
        pub ffie1: bool,
        /// FMPIE1
        pub fmpie1: bool,
        /// FOVIE0
        pub fovie0: bool,
        /// FFIE0
        pub ffie0: bool,
        /// FMPIE0
        pub fmpie0: bool,
        /// TMEIE
        pub tmeie: bool,
    }
    pub struct Cache {
        /// SLKIE
        pub slkie: bool,
        /// WKUIE
        pub wkuie: bool,
        /// ERRIE
        pub errie: bool,
        /// LECIE
        pub lecie: bool,
        /// BOFIE
        pub bofie: bool,
        /// EPVIE
        pub epvie: bool,
        /// EWGIE
        pub ewgie: bool,
        /// FOVIE1
        pub fovie1: bool,
        /// FFIE1
        pub ffie1: bool,
        /// FMPIE1
        pub fmpie1: bool,
        /// FOVIE0
        pub fovie0: bool,
        /// FFIE0
        pub ffie0: bool,
        /// FMPIE0
        pub fmpie0: bool,
        /// TMEIE
        pub tmeie: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x14u32) as *mut u32) };
        ReadonlyCache {
            slkie: ((value >> 17) & 0b1) > 0,
            wkuie: ((value >> 16) & 0b1) > 0,
            errie: ((value >> 15) & 0b1) > 0,
            lecie: ((value >> 11) & 0b1) > 0,
            bofie: ((value >> 10) & 0b1) > 0,
            epvie: ((value >> 9) & 0b1) > 0,
            ewgie: ((value >> 8) & 0b1) > 0,
            fovie1: ((value >> 6) & 0b1) > 0,
            ffie1: ((value >> 5) & 0b1) > 0,
            fmpie1: ((value >> 4) & 0b1) > 0,
            fovie0: ((value >> 3) & 0b1) > 0,
            ffie0: ((value >> 2) & 0b1) > 0,
            fmpie0: ((value >> 1) & 0b1) > 0,
            tmeie: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x14u32) as *mut u32) };
        Cache {
            slkie: ((value >> 17) & 0b1) > 0,
            wkuie: ((value >> 16) & 0b1) > 0,
            errie: ((value >> 15) & 0b1) > 0,
            lecie: ((value >> 11) & 0b1) > 0,
            bofie: ((value >> 10) & 0b1) > 0,
            epvie: ((value >> 9) & 0b1) > 0,
            ewgie: ((value >> 8) & 0b1) > 0,
            fovie1: ((value >> 6) & 0b1) > 0,
            ffie1: ((value >> 5) & 0b1) > 0,
            fmpie1: ((value >> 4) & 0b1) > 0,
            fovie0: ((value >> 3) & 0b1) > 0,
            ffie0: ((value >> 2) & 0b1) > 0,
            fmpie0: ((value >> 1) & 0b1) > 0,
            tmeie: ((value >> 0) & 0b1) > 0,
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
                | ((self.slkie as u32) << 17)
                | ((self.wkuie as u32) << 16)
                | ((self.errie as u32) << 15)
                | ((self.lecie as u32) << 11)
                | ((self.bofie as u32) << 10)
                | ((self.epvie as u32) << 9)
                | ((self.ewgie as u32) << 8)
                | ((self.fovie1 as u32) << 6)
                | ((self.ffie1 as u32) << 5)
                | ((self.fmpie1 as u32) << 4)
                | ((self.fovie0 as u32) << 3)
                | ((self.ffie0 as u32) << 2)
                | ((self.fmpie0 as u32) << 1)
                | ((self.tmeie as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x14u32) as *mut u32, value) };
        }
    }
}
/// error status register
pub mod esr {
    /// REC
    /// Access: read-only, Width: 8, Offset: 24
    /// Get REC
    pub fn rec() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b11111111 << 24);
        value as u8
    }
    /// TEC
    /// Access: read-only, Width: 8, Offset: 16
    /// Get TEC
    pub fn tec() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b11111111 << 16);
        value as u8
    }
    /// LEC
    /// Access: read-write, Width: 3, Offset: 4
    /// Set LEC
    pub fn set_lec(value: u8) {
        debug_assert!(value <= 0b111, "set_lec out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x18u32) as *mut u32, value) };
    }
    /// Get LEC
    pub fn get_lec() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b111 << 4);
        value as u8
    }
    /// BOFF
    /// Access: read-only, Width: 1, Offset: 2
    /// Get BOFF
    pub fn boff() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// EPVF
    /// Access: read-only, Width: 1, Offset: 1
    /// Get EPVF
    pub fn epvf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// EWGF
    /// Access: read-only, Width: 1, Offset: 0
    /// Get EWGF
    pub fn ewgf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// bit timing register
pub mod btr {
    pub struct ReadonlyCache {
        /// SILM
        pub silm: bool,
        /// LBKM
        pub lbkm: bool,
        /// SJW
        pub sjw: bool,
        /// TS2
        pub ts2: bool,
        /// TS1
        pub ts1: bool,
        /// BRP
        pub brp: bool,
    }
    pub struct Cache {
        /// SILM
        pub silm: bool,
        /// LBKM
        pub lbkm: bool,
        /// SJW
        pub sjw: bool,
        /// TS2
        pub ts2: bool,
        /// TS1
        pub ts1: bool,
        /// BRP
        pub brp: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1Cu32) as *mut u32) };
        ReadonlyCache {
            silm: ((value >> 31) & 0b1) > 0,
            lbkm: ((value >> 30) & 0b1) > 0,
            sjw: ((value >> 24) & 0b1) > 0,
            ts2: ((value >> 20) & 0b1) > 0,
            ts1: ((value >> 16) & 0b1) > 0,
            brp: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1Cu32) as *mut u32) };
        Cache {
            silm: ((value >> 31) & 0b1) > 0,
            lbkm: ((value >> 30) & 0b1) > 0,
            sjw: ((value >> 24) & 0b1) > 0,
            ts2: ((value >> 20) & 0b1) > 0,
            ts1: ((value >> 16) & 0b1) > 0,
            brp: ((value >> 0) & 0b1) > 0,
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
                | ((self.silm as u32) << 31)
                | ((self.lbkm as u32) << 30)
                | ((self.sjw as u32) << 24)
                | ((self.ts2 as u32) << 20)
                | ((self.ts1 as u32) << 16)
                | ((self.brp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x1Cu32) as *mut u32, value) };
        }
    }
}
/// TX mailbox identifier register
pub mod ti0r {
    pub struct ReadonlyCache {
        /// STID
        pub stid: u16,
        /// EXID
        pub exid: u16,
        /// IDE
        pub ide: u16,
        /// RTR
        pub rtr: u16,
        /// TXRQ
        pub txrq: u16,
    }
    pub struct Cache {
        /// STID
        pub stid: u16,
        /// EXID
        pub exid: u16,
        /// IDE
        pub ide: u16,
        /// RTR
        pub rtr: u16,
        /// TXRQ
        pub txrq: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x180u32) as *mut u32) };
        ReadonlyCache {
            stid: ((value >> 21) & 0b11111111111) as u16,
            exid: ((value >> 3) & 0b11111111111) as u16,
            ide: ((value >> 2) & 0b11111111111) as u16,
            rtr: ((value >> 1) & 0b11111111111) as u16,
            txrq: ((value >> 0) & 0b11111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x180u32) as *mut u32) };
        Cache {
            stid: ((value >> 21) & 0b11111111111) as u16,
            exid: ((value >> 3) & 0b11111111111) as u16,
            ide: ((value >> 2) & 0b11111111111) as u16,
            rtr: ((value >> 1) & 0b11111111111) as u16,
            txrq: ((value >> 0) & 0b11111111111) as u16,
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
                | ((self.stid as u32) << 21)
                | ((self.exid as u32) << 3)
                | ((self.ide as u32) << 2)
                | ((self.rtr as u32) << 1)
                | ((self.txrq as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x180u32) as *mut u32, value) };
        }
    }
}
/// mailbox data length control and time stamp register
pub mod tdt0r {
    pub struct ReadonlyCache {
        /// TIME
        pub time: u16,
        /// TGT
        pub tgt: u16,
        /// DLC
        pub dlc: u16,
    }
    pub struct Cache {
        /// TIME
        pub time: u16,
        /// TGT
        pub tgt: u16,
        /// DLC
        pub dlc: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x184u32) as *mut u32) };
        ReadonlyCache {
            time: ((value >> 16) & 0b1111111111111111) as u16,
            tgt: ((value >> 8) & 0b1111111111111111) as u16,
            dlc: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x184u32) as *mut u32) };
        Cache {
            time: ((value >> 16) & 0b1111111111111111) as u16,
            tgt: ((value >> 8) & 0b1111111111111111) as u16,
            dlc: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.time as u32) << 16)
                | ((self.tgt as u32) << 8)
                | ((self.dlc as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x184u32) as *mut u32, value) };
        }
    }
}
/// mailbox data low register
pub mod tdl0r {
    pub struct ReadonlyCache {
        /// DATA3
        pub data3: u8,
        /// DATA2
        pub data2: u8,
        /// DATA1
        pub data1: u8,
        /// DATA0
        pub data0: u8,
    }
    pub struct Cache {
        /// DATA3
        pub data3: u8,
        /// DATA2
        pub data2: u8,
        /// DATA1
        pub data1: u8,
        /// DATA0
        pub data0: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x188u32) as *mut u32) };
        ReadonlyCache {
            data3: ((value >> 24) & 0b11111111) as u8,
            data2: ((value >> 16) & 0b11111111) as u8,
            data1: ((value >> 8) & 0b11111111) as u8,
            data0: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x188u32) as *mut u32) };
        Cache {
            data3: ((value >> 24) & 0b11111111) as u8,
            data2: ((value >> 16) & 0b11111111) as u8,
            data1: ((value >> 8) & 0b11111111) as u8,
            data0: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.data3 as u32) << 24)
                | ((self.data2 as u32) << 16)
                | ((self.data1 as u32) << 8)
                | ((self.data0 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x188u32) as *mut u32, value) };
        }
    }
}
/// mailbox data high register
pub mod tdh0r {
    pub struct ReadonlyCache {
        /// DATA7
        pub data7: u8,
        /// DATA6
        pub data6: u8,
        /// DATA5
        pub data5: u8,
        /// DATA4
        pub data4: u8,
    }
    pub struct Cache {
        /// DATA7
        pub data7: u8,
        /// DATA6
        pub data6: u8,
        /// DATA5
        pub data5: u8,
        /// DATA4
        pub data4: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x18Cu32) as *mut u32) };
        ReadonlyCache {
            data7: ((value >> 24) & 0b11111111) as u8,
            data6: ((value >> 16) & 0b11111111) as u8,
            data5: ((value >> 8) & 0b11111111) as u8,
            data4: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x18Cu32) as *mut u32) };
        Cache {
            data7: ((value >> 24) & 0b11111111) as u8,
            data6: ((value >> 16) & 0b11111111) as u8,
            data5: ((value >> 8) & 0b11111111) as u8,
            data4: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.data7 as u32) << 24)
                | ((self.data6 as u32) << 16)
                | ((self.data5 as u32) << 8)
                | ((self.data4 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x18Cu32) as *mut u32, value) };
        }
    }
}
/// TX mailbox identifier register
pub mod ti1r {
    pub struct ReadonlyCache {
        /// STID
        pub stid: u16,
        /// EXID
        pub exid: u16,
        /// IDE
        pub ide: u16,
        /// RTR
        pub rtr: u16,
        /// TXRQ
        pub txrq: u16,
    }
    pub struct Cache {
        /// STID
        pub stid: u16,
        /// EXID
        pub exid: u16,
        /// IDE
        pub ide: u16,
        /// RTR
        pub rtr: u16,
        /// TXRQ
        pub txrq: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x190u32) as *mut u32) };
        ReadonlyCache {
            stid: ((value >> 21) & 0b11111111111) as u16,
            exid: ((value >> 3) & 0b11111111111) as u16,
            ide: ((value >> 2) & 0b11111111111) as u16,
            rtr: ((value >> 1) & 0b11111111111) as u16,
            txrq: ((value >> 0) & 0b11111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x190u32) as *mut u32) };
        Cache {
            stid: ((value >> 21) & 0b11111111111) as u16,
            exid: ((value >> 3) & 0b11111111111) as u16,
            ide: ((value >> 2) & 0b11111111111) as u16,
            rtr: ((value >> 1) & 0b11111111111) as u16,
            txrq: ((value >> 0) & 0b11111111111) as u16,
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
                | ((self.stid as u32) << 21)
                | ((self.exid as u32) << 3)
                | ((self.ide as u32) << 2)
                | ((self.rtr as u32) << 1)
                | ((self.txrq as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x190u32) as *mut u32, value) };
        }
    }
}
/// mailbox data length control and time stamp register
pub mod tdt1r {
    pub struct ReadonlyCache {
        /// TIME
        pub time: u16,
        /// TGT
        pub tgt: u16,
        /// DLC
        pub dlc: u16,
    }
    pub struct Cache {
        /// TIME
        pub time: u16,
        /// TGT
        pub tgt: u16,
        /// DLC
        pub dlc: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x194u32) as *mut u32) };
        ReadonlyCache {
            time: ((value >> 16) & 0b1111111111111111) as u16,
            tgt: ((value >> 8) & 0b1111111111111111) as u16,
            dlc: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x194u32) as *mut u32) };
        Cache {
            time: ((value >> 16) & 0b1111111111111111) as u16,
            tgt: ((value >> 8) & 0b1111111111111111) as u16,
            dlc: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.time as u32) << 16)
                | ((self.tgt as u32) << 8)
                | ((self.dlc as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x194u32) as *mut u32, value) };
        }
    }
}
/// mailbox data low register
pub mod tdl1r {
    pub struct ReadonlyCache {
        /// DATA3
        pub data3: u8,
        /// DATA2
        pub data2: u8,
        /// DATA1
        pub data1: u8,
        /// DATA0
        pub data0: u8,
    }
    pub struct Cache {
        /// DATA3
        pub data3: u8,
        /// DATA2
        pub data2: u8,
        /// DATA1
        pub data1: u8,
        /// DATA0
        pub data0: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x198u32) as *mut u32) };
        ReadonlyCache {
            data3: ((value >> 24) & 0b11111111) as u8,
            data2: ((value >> 16) & 0b11111111) as u8,
            data1: ((value >> 8) & 0b11111111) as u8,
            data0: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x198u32) as *mut u32) };
        Cache {
            data3: ((value >> 24) & 0b11111111) as u8,
            data2: ((value >> 16) & 0b11111111) as u8,
            data1: ((value >> 8) & 0b11111111) as u8,
            data0: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.data3 as u32) << 24)
                | ((self.data2 as u32) << 16)
                | ((self.data1 as u32) << 8)
                | ((self.data0 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x198u32) as *mut u32, value) };
        }
    }
}
/// mailbox data high register
pub mod tdh1r {
    pub struct ReadonlyCache {
        /// DATA7
        pub data7: u8,
        /// DATA6
        pub data6: u8,
        /// DATA5
        pub data5: u8,
        /// DATA4
        pub data4: u8,
    }
    pub struct Cache {
        /// DATA7
        pub data7: u8,
        /// DATA6
        pub data6: u8,
        /// DATA5
        pub data5: u8,
        /// DATA4
        pub data4: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x19Cu32) as *mut u32) };
        ReadonlyCache {
            data7: ((value >> 24) & 0b11111111) as u8,
            data6: ((value >> 16) & 0b11111111) as u8,
            data5: ((value >> 8) & 0b11111111) as u8,
            data4: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x19Cu32) as *mut u32) };
        Cache {
            data7: ((value >> 24) & 0b11111111) as u8,
            data6: ((value >> 16) & 0b11111111) as u8,
            data5: ((value >> 8) & 0b11111111) as u8,
            data4: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.data7 as u32) << 24)
                | ((self.data6 as u32) << 16)
                | ((self.data5 as u32) << 8)
                | ((self.data4 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x19Cu32) as *mut u32, value) };
        }
    }
}
/// TX mailbox identifier register
pub mod ti2r {
    pub struct ReadonlyCache {
        /// STID
        pub stid: u16,
        /// EXID
        pub exid: u16,
        /// IDE
        pub ide: u16,
        /// RTR
        pub rtr: u16,
        /// TXRQ
        pub txrq: u16,
    }
    pub struct Cache {
        /// STID
        pub stid: u16,
        /// EXID
        pub exid: u16,
        /// IDE
        pub ide: u16,
        /// RTR
        pub rtr: u16,
        /// TXRQ
        pub txrq: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1A0u32) as *mut u32) };
        ReadonlyCache {
            stid: ((value >> 21) & 0b11111111111) as u16,
            exid: ((value >> 3) & 0b11111111111) as u16,
            ide: ((value >> 2) & 0b11111111111) as u16,
            rtr: ((value >> 1) & 0b11111111111) as u16,
            txrq: ((value >> 0) & 0b11111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1A0u32) as *mut u32) };
        Cache {
            stid: ((value >> 21) & 0b11111111111) as u16,
            exid: ((value >> 3) & 0b11111111111) as u16,
            ide: ((value >> 2) & 0b11111111111) as u16,
            rtr: ((value >> 1) & 0b11111111111) as u16,
            txrq: ((value >> 0) & 0b11111111111) as u16,
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
                | ((self.stid as u32) << 21)
                | ((self.exid as u32) << 3)
                | ((self.ide as u32) << 2)
                | ((self.rtr as u32) << 1)
                | ((self.txrq as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x1A0u32) as *mut u32, value) };
        }
    }
}
/// mailbox data length control and time stamp register
pub mod tdt2r {
    pub struct ReadonlyCache {
        /// TIME
        pub time: u16,
        /// TGT
        pub tgt: u16,
        /// DLC
        pub dlc: u16,
    }
    pub struct Cache {
        /// TIME
        pub time: u16,
        /// TGT
        pub tgt: u16,
        /// DLC
        pub dlc: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1A4u32) as *mut u32) };
        ReadonlyCache {
            time: ((value >> 16) & 0b1111111111111111) as u16,
            tgt: ((value >> 8) & 0b1111111111111111) as u16,
            dlc: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1A4u32) as *mut u32) };
        Cache {
            time: ((value >> 16) & 0b1111111111111111) as u16,
            tgt: ((value >> 8) & 0b1111111111111111) as u16,
            dlc: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.time as u32) << 16)
                | ((self.tgt as u32) << 8)
                | ((self.dlc as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x1A4u32) as *mut u32, value) };
        }
    }
}
/// mailbox data low register
pub mod tdl2r {
    pub struct ReadonlyCache {
        /// DATA3
        pub data3: u8,
        /// DATA2
        pub data2: u8,
        /// DATA1
        pub data1: u8,
        /// DATA0
        pub data0: u8,
    }
    pub struct Cache {
        /// DATA3
        pub data3: u8,
        /// DATA2
        pub data2: u8,
        /// DATA1
        pub data1: u8,
        /// DATA0
        pub data0: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1A8u32) as *mut u32) };
        ReadonlyCache {
            data3: ((value >> 24) & 0b11111111) as u8,
            data2: ((value >> 16) & 0b11111111) as u8,
            data1: ((value >> 8) & 0b11111111) as u8,
            data0: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1A8u32) as *mut u32) };
        Cache {
            data3: ((value >> 24) & 0b11111111) as u8,
            data2: ((value >> 16) & 0b11111111) as u8,
            data1: ((value >> 8) & 0b11111111) as u8,
            data0: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.data3 as u32) << 24)
                | ((self.data2 as u32) << 16)
                | ((self.data1 as u32) << 8)
                | ((self.data0 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x1A8u32) as *mut u32, value) };
        }
    }
}
/// mailbox data high register
pub mod tdh2r {
    pub struct ReadonlyCache {
        /// DATA7
        pub data7: u8,
        /// DATA6
        pub data6: u8,
        /// DATA5
        pub data5: u8,
        /// DATA4
        pub data4: u8,
    }
    pub struct Cache {
        /// DATA7
        pub data7: u8,
        /// DATA6
        pub data6: u8,
        /// DATA5
        pub data5: u8,
        /// DATA4
        pub data4: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1ACu32) as *mut u32) };
        ReadonlyCache {
            data7: ((value >> 24) & 0b11111111) as u8,
            data6: ((value >> 16) & 0b11111111) as u8,
            data5: ((value >> 8) & 0b11111111) as u8,
            data4: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1ACu32) as *mut u32) };
        Cache {
            data7: ((value >> 24) & 0b11111111) as u8,
            data6: ((value >> 16) & 0b11111111) as u8,
            data5: ((value >> 8) & 0b11111111) as u8,
            data4: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.data7 as u32) << 24)
                | ((self.data6 as u32) << 16)
                | ((self.data5 as u32) << 8)
                | ((self.data4 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x1ACu32) as *mut u32, value) };
        }
    }
}
/// receive FIFO mailbox identifier register
pub mod ri0r {
    /// STID
    /// Access: read-only, Width: 11, Offset: 21
    /// Get STID
    pub fn stid() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1B0u32) as *mut u32) };
        let value = value & (0b11111111111 << 21);
        value as u16
    }
    /// EXID
    /// Access: read-only, Width: 18, Offset: 3
    /// Get EXID
    pub fn exid() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1B0u32) as *mut u32) };
        let value = value & (0b111111111111111111 << 3);
        value as u32
    }
    /// IDE
    /// Access: read-only, Width: 1, Offset: 2
    /// Get IDE
    pub fn ide() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1B0u32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// RTR
    /// Access: read-only, Width: 1, Offset: 1
    /// Get RTR
    pub fn rtr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1B0u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
}
/// receive FIFO mailbox data length control and time stamp register
pub mod rdt0r {
    /// TIME
    /// Access: read-only, Width: 16, Offset: 16
    /// Get TIME
    pub fn time() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1B4u32) as *mut u32) };
        let value = value & (0b1111111111111111 << 16);
        value as u16
    }
    /// FMI
    /// Access: read-only, Width: 8, Offset: 8
    /// Get FMI
    pub fn fmi() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1B4u32) as *mut u32) };
        let value = value & (0b11111111 << 8);
        value as u8
    }
    /// DLC
    /// Access: read-only, Width: 4, Offset: 0
    /// Get DLC
    pub fn dlc() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1B4u32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
}
/// receive FIFO mailbox data low register
pub mod rdl0r {
    /// Get DATA3
    pub fn data(index: u8) -> u8 {
        debug_assert!(index < 4, "data out of range");
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1B8u32) as *mut u32) };
        let value = value & (0b11111111 << (0 + index * 8));
        value as u8
    }
}
/// receive FIFO mailbox data high register
pub mod rdh0r {
    /// Get DATA7
    pub fn data(index: u8) -> u8 {
        debug_assert!(index < 4, "data out of range");
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1BCu32) as *mut u32) };
        let value = value & (0b11111111 << (0 + index * 8));
        value as u8
    }
}
/// receive FIFO mailbox identifier register
pub mod ri1r {
    /// STID
    /// Access: read-only, Width: 11, Offset: 21
    /// Get STID
    pub fn stid() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1C0u32) as *mut u32) };
        let value = value & (0b11111111111 << 21);
        value as u16
    }
    /// EXID
    /// Access: read-only, Width: 18, Offset: 3
    /// Get EXID
    pub fn exid() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1C0u32) as *mut u32) };
        let value = value & (0b111111111111111111 << 3);
        value as u32
    }
    /// IDE
    /// Access: read-only, Width: 1, Offset: 2
    /// Get IDE
    pub fn ide() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1C0u32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// RTR
    /// Access: read-only, Width: 1, Offset: 1
    /// Get RTR
    pub fn rtr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1C0u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
}
/// receive FIFO mailbox data length control and time stamp register
pub mod rdt1r {
    /// TIME
    /// Access: read-only, Width: 16, Offset: 16
    /// Get TIME
    pub fn time() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1C4u32) as *mut u32) };
        let value = value & (0b1111111111111111 << 16);
        value as u16
    }
    /// FMI
    /// Access: read-only, Width: 8, Offset: 8
    /// Get FMI
    pub fn fmi() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1C4u32) as *mut u32) };
        let value = value & (0b11111111 << 8);
        value as u8
    }
    /// DLC
    /// Access: read-only, Width: 4, Offset: 0
    /// Get DLC
    pub fn dlc() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1C4u32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
}
/// receive FIFO mailbox data low register
pub mod rdl1r {
    /// Get DATA3
    pub fn data(index: u8) -> u8 {
        debug_assert!(index < 4, "data out of range");
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1C8u32) as *mut u32) };
        let value = value & (0b11111111 << (0 + index * 8));
        value as u8
    }
}
/// receive FIFO mailbox data high register
pub mod rdh1r {
    /// Get DATA7
    pub fn data(index: u8) -> u8 {
        debug_assert!(index < 4, "data out of range");
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x1CCu32) as *mut u32) };
        let value = value & (0b11111111 << (0 + index * 8));
        value as u8
    }
}
/// filter master register
pub mod fmr {
    pub struct ReadonlyCache {
        /// CAN2 start bank
        pub can2sb: u8,
        /// Filter init mode
        pub finit: u8,
    }
    pub struct Cache {
        /// CAN2 start bank
        pub can2sb: u8,
        /// Filter init mode
        pub finit: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x200u32) as *mut u32) };
        ReadonlyCache {
            can2sb: ((value >> 8) & 0b111111) as u8,
            finit: ((value >> 0) & 0b111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x200u32) as *mut u32) };
        Cache {
            can2sb: ((value >> 8) & 0b111111) as u8,
            finit: ((value >> 0) & 0b111111) as u8,
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
                | ((self.can2sb as u32) << 8)
                | ((self.finit as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x200u32) as *mut u32, value) };
        }
    }
}
/// filter mode register
pub mod fm1r {
    pub struct ReadonlyCache {
        /// Filter mode
        pub fbm0: bool,
        /// Filter mode
        pub fbm1: bool,
        /// Filter mode
        pub fbm2: bool,
        /// Filter mode
        pub fbm3: bool,
        /// Filter mode
        pub fbm4: bool,
        /// Filter mode
        pub fbm5: bool,
        /// Filter mode
        pub fbm6: bool,
        /// Filter mode
        pub fbm7: bool,
        /// Filter mode
        pub fbm8: bool,
        /// Filter mode
        pub fbm9: bool,
        /// Filter mode
        pub fbm10: bool,
        /// Filter mode
        pub fbm11: bool,
        /// Filter mode
        pub fbm12: bool,
        /// Filter mode
        pub fbm13: bool,
        /// Filter mode
        pub fbm14: bool,
        /// Filter mode
        pub fbm15: bool,
        /// Filter mode
        pub fbm16: bool,
        /// Filter mode
        pub fbm17: bool,
        /// Filter mode
        pub fbm18: bool,
        /// Filter mode
        pub fbm19: bool,
        /// Filter mode
        pub fbm20: bool,
        /// Filter mode
        pub fbm21: bool,
        /// Filter mode
        pub fbm22: bool,
        /// Filter mode
        pub fbm23: bool,
        /// Filter mode
        pub fbm24: bool,
        /// Filter mode
        pub fbm25: bool,
        /// Filter mode
        pub fbm26: bool,
        /// Filter mode
        pub fbm27: bool,
    }
    pub struct Cache {
        /// Filter mode
        pub fbm0: bool,
        /// Filter mode
        pub fbm1: bool,
        /// Filter mode
        pub fbm2: bool,
        /// Filter mode
        pub fbm3: bool,
        /// Filter mode
        pub fbm4: bool,
        /// Filter mode
        pub fbm5: bool,
        /// Filter mode
        pub fbm6: bool,
        /// Filter mode
        pub fbm7: bool,
        /// Filter mode
        pub fbm8: bool,
        /// Filter mode
        pub fbm9: bool,
        /// Filter mode
        pub fbm10: bool,
        /// Filter mode
        pub fbm11: bool,
        /// Filter mode
        pub fbm12: bool,
        /// Filter mode
        pub fbm13: bool,
        /// Filter mode
        pub fbm14: bool,
        /// Filter mode
        pub fbm15: bool,
        /// Filter mode
        pub fbm16: bool,
        /// Filter mode
        pub fbm17: bool,
        /// Filter mode
        pub fbm18: bool,
        /// Filter mode
        pub fbm19: bool,
        /// Filter mode
        pub fbm20: bool,
        /// Filter mode
        pub fbm21: bool,
        /// Filter mode
        pub fbm22: bool,
        /// Filter mode
        pub fbm23: bool,
        /// Filter mode
        pub fbm24: bool,
        /// Filter mode
        pub fbm25: bool,
        /// Filter mode
        pub fbm26: bool,
        /// Filter mode
        pub fbm27: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x204u32) as *mut u32) };
        ReadonlyCache {
            fbm0: ((value >> 0) & 0b1) > 0,
            fbm1: ((value >> 1) & 0b1) > 0,
            fbm2: ((value >> 2) & 0b1) > 0,
            fbm3: ((value >> 3) & 0b1) > 0,
            fbm4: ((value >> 4) & 0b1) > 0,
            fbm5: ((value >> 5) & 0b1) > 0,
            fbm6: ((value >> 6) & 0b1) > 0,
            fbm7: ((value >> 7) & 0b1) > 0,
            fbm8: ((value >> 8) & 0b1) > 0,
            fbm9: ((value >> 9) & 0b1) > 0,
            fbm10: ((value >> 10) & 0b1) > 0,
            fbm11: ((value >> 11) & 0b1) > 0,
            fbm12: ((value >> 12) & 0b1) > 0,
            fbm13: ((value >> 13) & 0b1) > 0,
            fbm14: ((value >> 14) & 0b1) > 0,
            fbm15: ((value >> 15) & 0b1) > 0,
            fbm16: ((value >> 16) & 0b1) > 0,
            fbm17: ((value >> 17) & 0b1) > 0,
            fbm18: ((value >> 18) & 0b1) > 0,
            fbm19: ((value >> 19) & 0b1) > 0,
            fbm20: ((value >> 20) & 0b1) > 0,
            fbm21: ((value >> 21) & 0b1) > 0,
            fbm22: ((value >> 22) & 0b1) > 0,
            fbm23: ((value >> 23) & 0b1) > 0,
            fbm24: ((value >> 24) & 0b1) > 0,
            fbm25: ((value >> 25) & 0b1) > 0,
            fbm26: ((value >> 26) & 0b1) > 0,
            fbm27: ((value >> 27) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x204u32) as *mut u32) };
        Cache {
            fbm0: ((value >> 0) & 0b1) > 0,
            fbm1: ((value >> 1) & 0b1) > 0,
            fbm2: ((value >> 2) & 0b1) > 0,
            fbm3: ((value >> 3) & 0b1) > 0,
            fbm4: ((value >> 4) & 0b1) > 0,
            fbm5: ((value >> 5) & 0b1) > 0,
            fbm6: ((value >> 6) & 0b1) > 0,
            fbm7: ((value >> 7) & 0b1) > 0,
            fbm8: ((value >> 8) & 0b1) > 0,
            fbm9: ((value >> 9) & 0b1) > 0,
            fbm10: ((value >> 10) & 0b1) > 0,
            fbm11: ((value >> 11) & 0b1) > 0,
            fbm12: ((value >> 12) & 0b1) > 0,
            fbm13: ((value >> 13) & 0b1) > 0,
            fbm14: ((value >> 14) & 0b1) > 0,
            fbm15: ((value >> 15) & 0b1) > 0,
            fbm16: ((value >> 16) & 0b1) > 0,
            fbm17: ((value >> 17) & 0b1) > 0,
            fbm18: ((value >> 18) & 0b1) > 0,
            fbm19: ((value >> 19) & 0b1) > 0,
            fbm20: ((value >> 20) & 0b1) > 0,
            fbm21: ((value >> 21) & 0b1) > 0,
            fbm22: ((value >> 22) & 0b1) > 0,
            fbm23: ((value >> 23) & 0b1) > 0,
            fbm24: ((value >> 24) & 0b1) > 0,
            fbm25: ((value >> 25) & 0b1) > 0,
            fbm26: ((value >> 26) & 0b1) > 0,
            fbm27: ((value >> 27) & 0b1) > 0,
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
                | ((self.fbm0 as u32) << 0)
                | ((self.fbm1 as u32) << 1)
                | ((self.fbm2 as u32) << 2)
                | ((self.fbm3 as u32) << 3)
                | ((self.fbm4 as u32) << 4)
                | ((self.fbm5 as u32) << 5)
                | ((self.fbm6 as u32) << 6)
                | ((self.fbm7 as u32) << 7)
                | ((self.fbm8 as u32) << 8)
                | ((self.fbm9 as u32) << 9)
                | ((self.fbm10 as u32) << 10)
                | ((self.fbm11 as u32) << 11)
                | ((self.fbm12 as u32) << 12)
                | ((self.fbm13 as u32) << 13)
                | ((self.fbm14 as u32) << 14)
                | ((self.fbm15 as u32) << 15)
                | ((self.fbm16 as u32) << 16)
                | ((self.fbm17 as u32) << 17)
                | ((self.fbm18 as u32) << 18)
                | ((self.fbm19 as u32) << 19)
                | ((self.fbm20 as u32) << 20)
                | ((self.fbm21 as u32) << 21)
                | ((self.fbm22 as u32) << 22)
                | ((self.fbm23 as u32) << 23)
                | ((self.fbm24 as u32) << 24)
                | ((self.fbm25 as u32) << 25)
                | ((self.fbm26 as u32) << 26)
                | ((self.fbm27 as u32) << 27)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x204u32) as *mut u32, value) };
        }
    }
}
/// filter scale register
pub mod fs1r {
    pub struct ReadonlyCache {
        /// Filter scale configuration
        pub fsc0: bool,
        /// Filter scale configuration
        pub fsc1: bool,
        /// Filter scale configuration
        pub fsc2: bool,
        /// Filter scale configuration
        pub fsc3: bool,
        /// Filter scale configuration
        pub fsc4: bool,
        /// Filter scale configuration
        pub fsc5: bool,
        /// Filter scale configuration
        pub fsc6: bool,
        /// Filter scale configuration
        pub fsc7: bool,
        /// Filter scale configuration
        pub fsc8: bool,
        /// Filter scale configuration
        pub fsc9: bool,
        /// Filter scale configuration
        pub fsc10: bool,
        /// Filter scale configuration
        pub fsc11: bool,
        /// Filter scale configuration
        pub fsc12: bool,
        /// Filter scale configuration
        pub fsc13: bool,
        /// Filter scale configuration
        pub fsc14: bool,
        /// Filter scale configuration
        pub fsc15: bool,
        /// Filter scale configuration
        pub fsc16: bool,
        /// Filter scale configuration
        pub fsc17: bool,
        /// Filter scale configuration
        pub fsc18: bool,
        /// Filter scale configuration
        pub fsc19: bool,
        /// Filter scale configuration
        pub fsc20: bool,
        /// Filter scale configuration
        pub fsc21: bool,
        /// Filter scale configuration
        pub fsc22: bool,
        /// Filter scale configuration
        pub fsc23: bool,
        /// Filter scale configuration
        pub fsc24: bool,
        /// Filter scale configuration
        pub fsc25: bool,
        /// Filter scale configuration
        pub fsc26: bool,
        /// Filter scale configuration
        pub fsc27: bool,
    }
    pub struct Cache {
        /// Filter scale configuration
        pub fsc0: bool,
        /// Filter scale configuration
        pub fsc1: bool,
        /// Filter scale configuration
        pub fsc2: bool,
        /// Filter scale configuration
        pub fsc3: bool,
        /// Filter scale configuration
        pub fsc4: bool,
        /// Filter scale configuration
        pub fsc5: bool,
        /// Filter scale configuration
        pub fsc6: bool,
        /// Filter scale configuration
        pub fsc7: bool,
        /// Filter scale configuration
        pub fsc8: bool,
        /// Filter scale configuration
        pub fsc9: bool,
        /// Filter scale configuration
        pub fsc10: bool,
        /// Filter scale configuration
        pub fsc11: bool,
        /// Filter scale configuration
        pub fsc12: bool,
        /// Filter scale configuration
        pub fsc13: bool,
        /// Filter scale configuration
        pub fsc14: bool,
        /// Filter scale configuration
        pub fsc15: bool,
        /// Filter scale configuration
        pub fsc16: bool,
        /// Filter scale configuration
        pub fsc17: bool,
        /// Filter scale configuration
        pub fsc18: bool,
        /// Filter scale configuration
        pub fsc19: bool,
        /// Filter scale configuration
        pub fsc20: bool,
        /// Filter scale configuration
        pub fsc21: bool,
        /// Filter scale configuration
        pub fsc22: bool,
        /// Filter scale configuration
        pub fsc23: bool,
        /// Filter scale configuration
        pub fsc24: bool,
        /// Filter scale configuration
        pub fsc25: bool,
        /// Filter scale configuration
        pub fsc26: bool,
        /// Filter scale configuration
        pub fsc27: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x20Cu32) as *mut u32) };
        ReadonlyCache {
            fsc0: ((value >> 0) & 0b1) > 0,
            fsc1: ((value >> 1) & 0b1) > 0,
            fsc2: ((value >> 2) & 0b1) > 0,
            fsc3: ((value >> 3) & 0b1) > 0,
            fsc4: ((value >> 4) & 0b1) > 0,
            fsc5: ((value >> 5) & 0b1) > 0,
            fsc6: ((value >> 6) & 0b1) > 0,
            fsc7: ((value >> 7) & 0b1) > 0,
            fsc8: ((value >> 8) & 0b1) > 0,
            fsc9: ((value >> 9) & 0b1) > 0,
            fsc10: ((value >> 10) & 0b1) > 0,
            fsc11: ((value >> 11) & 0b1) > 0,
            fsc12: ((value >> 12) & 0b1) > 0,
            fsc13: ((value >> 13) & 0b1) > 0,
            fsc14: ((value >> 14) & 0b1) > 0,
            fsc15: ((value >> 15) & 0b1) > 0,
            fsc16: ((value >> 16) & 0b1) > 0,
            fsc17: ((value >> 17) & 0b1) > 0,
            fsc18: ((value >> 18) & 0b1) > 0,
            fsc19: ((value >> 19) & 0b1) > 0,
            fsc20: ((value >> 20) & 0b1) > 0,
            fsc21: ((value >> 21) & 0b1) > 0,
            fsc22: ((value >> 22) & 0b1) > 0,
            fsc23: ((value >> 23) & 0b1) > 0,
            fsc24: ((value >> 24) & 0b1) > 0,
            fsc25: ((value >> 25) & 0b1) > 0,
            fsc26: ((value >> 26) & 0b1) > 0,
            fsc27: ((value >> 27) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x20Cu32) as *mut u32) };
        Cache {
            fsc0: ((value >> 0) & 0b1) > 0,
            fsc1: ((value >> 1) & 0b1) > 0,
            fsc2: ((value >> 2) & 0b1) > 0,
            fsc3: ((value >> 3) & 0b1) > 0,
            fsc4: ((value >> 4) & 0b1) > 0,
            fsc5: ((value >> 5) & 0b1) > 0,
            fsc6: ((value >> 6) & 0b1) > 0,
            fsc7: ((value >> 7) & 0b1) > 0,
            fsc8: ((value >> 8) & 0b1) > 0,
            fsc9: ((value >> 9) & 0b1) > 0,
            fsc10: ((value >> 10) & 0b1) > 0,
            fsc11: ((value >> 11) & 0b1) > 0,
            fsc12: ((value >> 12) & 0b1) > 0,
            fsc13: ((value >> 13) & 0b1) > 0,
            fsc14: ((value >> 14) & 0b1) > 0,
            fsc15: ((value >> 15) & 0b1) > 0,
            fsc16: ((value >> 16) & 0b1) > 0,
            fsc17: ((value >> 17) & 0b1) > 0,
            fsc18: ((value >> 18) & 0b1) > 0,
            fsc19: ((value >> 19) & 0b1) > 0,
            fsc20: ((value >> 20) & 0b1) > 0,
            fsc21: ((value >> 21) & 0b1) > 0,
            fsc22: ((value >> 22) & 0b1) > 0,
            fsc23: ((value >> 23) & 0b1) > 0,
            fsc24: ((value >> 24) & 0b1) > 0,
            fsc25: ((value >> 25) & 0b1) > 0,
            fsc26: ((value >> 26) & 0b1) > 0,
            fsc27: ((value >> 27) & 0b1) > 0,
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
                | ((self.fsc0 as u32) << 0)
                | ((self.fsc1 as u32) << 1)
                | ((self.fsc2 as u32) << 2)
                | ((self.fsc3 as u32) << 3)
                | ((self.fsc4 as u32) << 4)
                | ((self.fsc5 as u32) << 5)
                | ((self.fsc6 as u32) << 6)
                | ((self.fsc7 as u32) << 7)
                | ((self.fsc8 as u32) << 8)
                | ((self.fsc9 as u32) << 9)
                | ((self.fsc10 as u32) << 10)
                | ((self.fsc11 as u32) << 11)
                | ((self.fsc12 as u32) << 12)
                | ((self.fsc13 as u32) << 13)
                | ((self.fsc14 as u32) << 14)
                | ((self.fsc15 as u32) << 15)
                | ((self.fsc16 as u32) << 16)
                | ((self.fsc17 as u32) << 17)
                | ((self.fsc18 as u32) << 18)
                | ((self.fsc19 as u32) << 19)
                | ((self.fsc20 as u32) << 20)
                | ((self.fsc21 as u32) << 21)
                | ((self.fsc22 as u32) << 22)
                | ((self.fsc23 as u32) << 23)
                | ((self.fsc24 as u32) << 24)
                | ((self.fsc25 as u32) << 25)
                | ((self.fsc26 as u32) << 26)
                | ((self.fsc27 as u32) << 27)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x20Cu32) as *mut u32, value) };
        }
    }
}
/// filter FIFO assignment register
pub mod ffa1r {
    pub struct ReadonlyCache {
        /// Filter FIFO assignment for filter 0
        pub ffa0: bool,
        /// Filter FIFO assignment for filter 1
        pub ffa1: bool,
        /// Filter FIFO assignment for filter 2
        pub ffa2: bool,
        /// Filter FIFO assignment for filter 3
        pub ffa3: bool,
        /// Filter FIFO assignment for filter 4
        pub ffa4: bool,
        /// Filter FIFO assignment for filter 5
        pub ffa5: bool,
        /// Filter FIFO assignment for filter 6
        pub ffa6: bool,
        /// Filter FIFO assignment for filter 7
        pub ffa7: bool,
        /// Filter FIFO assignment for filter 8
        pub ffa8: bool,
        /// Filter FIFO assignment for filter 9
        pub ffa9: bool,
        /// Filter FIFO assignment for filter 10
        pub ffa10: bool,
        /// Filter FIFO assignment for filter 11
        pub ffa11: bool,
        /// Filter FIFO assignment for filter 12
        pub ffa12: bool,
        /// Filter FIFO assignment for filter 13
        pub ffa13: bool,
        /// Filter FIFO assignment for filter 14
        pub ffa14: bool,
        /// Filter FIFO assignment for filter 15
        pub ffa15: bool,
        /// Filter FIFO assignment for filter 16
        pub ffa16: bool,
        /// Filter FIFO assignment for filter 17
        pub ffa17: bool,
        /// Filter FIFO assignment for filter 18
        pub ffa18: bool,
        /// Filter FIFO assignment for filter 19
        pub ffa19: bool,
        /// Filter FIFO assignment for filter 20
        pub ffa20: bool,
        /// Filter FIFO assignment for filter 21
        pub ffa21: bool,
        /// Filter FIFO assignment for filter 22
        pub ffa22: bool,
        /// Filter FIFO assignment for filter 23
        pub ffa23: bool,
        /// Filter FIFO assignment for filter 24
        pub ffa24: bool,
        /// Filter FIFO assignment for filter 25
        pub ffa25: bool,
        /// Filter FIFO assignment for filter 26
        pub ffa26: bool,
        /// Filter FIFO assignment for filter 27
        pub ffa27: bool,
    }
    pub struct Cache {
        /// Filter FIFO assignment for filter 0
        pub ffa0: bool,
        /// Filter FIFO assignment for filter 1
        pub ffa1: bool,
        /// Filter FIFO assignment for filter 2
        pub ffa2: bool,
        /// Filter FIFO assignment for filter 3
        pub ffa3: bool,
        /// Filter FIFO assignment for filter 4
        pub ffa4: bool,
        /// Filter FIFO assignment for filter 5
        pub ffa5: bool,
        /// Filter FIFO assignment for filter 6
        pub ffa6: bool,
        /// Filter FIFO assignment for filter 7
        pub ffa7: bool,
        /// Filter FIFO assignment for filter 8
        pub ffa8: bool,
        /// Filter FIFO assignment for filter 9
        pub ffa9: bool,
        /// Filter FIFO assignment for filter 10
        pub ffa10: bool,
        /// Filter FIFO assignment for filter 11
        pub ffa11: bool,
        /// Filter FIFO assignment for filter 12
        pub ffa12: bool,
        /// Filter FIFO assignment for filter 13
        pub ffa13: bool,
        /// Filter FIFO assignment for filter 14
        pub ffa14: bool,
        /// Filter FIFO assignment for filter 15
        pub ffa15: bool,
        /// Filter FIFO assignment for filter 16
        pub ffa16: bool,
        /// Filter FIFO assignment for filter 17
        pub ffa17: bool,
        /// Filter FIFO assignment for filter 18
        pub ffa18: bool,
        /// Filter FIFO assignment for filter 19
        pub ffa19: bool,
        /// Filter FIFO assignment for filter 20
        pub ffa20: bool,
        /// Filter FIFO assignment for filter 21
        pub ffa21: bool,
        /// Filter FIFO assignment for filter 22
        pub ffa22: bool,
        /// Filter FIFO assignment for filter 23
        pub ffa23: bool,
        /// Filter FIFO assignment for filter 24
        pub ffa24: bool,
        /// Filter FIFO assignment for filter 25
        pub ffa25: bool,
        /// Filter FIFO assignment for filter 26
        pub ffa26: bool,
        /// Filter FIFO assignment for filter 27
        pub ffa27: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x214u32) as *mut u32) };
        ReadonlyCache {
            ffa0: ((value >> 0) & 0b1) > 0,
            ffa1: ((value >> 1) & 0b1) > 0,
            ffa2: ((value >> 2) & 0b1) > 0,
            ffa3: ((value >> 3) & 0b1) > 0,
            ffa4: ((value >> 4) & 0b1) > 0,
            ffa5: ((value >> 5) & 0b1) > 0,
            ffa6: ((value >> 6) & 0b1) > 0,
            ffa7: ((value >> 7) & 0b1) > 0,
            ffa8: ((value >> 8) & 0b1) > 0,
            ffa9: ((value >> 9) & 0b1) > 0,
            ffa10: ((value >> 10) & 0b1) > 0,
            ffa11: ((value >> 11) & 0b1) > 0,
            ffa12: ((value >> 12) & 0b1) > 0,
            ffa13: ((value >> 13) & 0b1) > 0,
            ffa14: ((value >> 14) & 0b1) > 0,
            ffa15: ((value >> 15) & 0b1) > 0,
            ffa16: ((value >> 16) & 0b1) > 0,
            ffa17: ((value >> 17) & 0b1) > 0,
            ffa18: ((value >> 18) & 0b1) > 0,
            ffa19: ((value >> 19) & 0b1) > 0,
            ffa20: ((value >> 20) & 0b1) > 0,
            ffa21: ((value >> 21) & 0b1) > 0,
            ffa22: ((value >> 22) & 0b1) > 0,
            ffa23: ((value >> 23) & 0b1) > 0,
            ffa24: ((value >> 24) & 0b1) > 0,
            ffa25: ((value >> 25) & 0b1) > 0,
            ffa26: ((value >> 26) & 0b1) > 0,
            ffa27: ((value >> 27) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x214u32) as *mut u32) };
        Cache {
            ffa0: ((value >> 0) & 0b1) > 0,
            ffa1: ((value >> 1) & 0b1) > 0,
            ffa2: ((value >> 2) & 0b1) > 0,
            ffa3: ((value >> 3) & 0b1) > 0,
            ffa4: ((value >> 4) & 0b1) > 0,
            ffa5: ((value >> 5) & 0b1) > 0,
            ffa6: ((value >> 6) & 0b1) > 0,
            ffa7: ((value >> 7) & 0b1) > 0,
            ffa8: ((value >> 8) & 0b1) > 0,
            ffa9: ((value >> 9) & 0b1) > 0,
            ffa10: ((value >> 10) & 0b1) > 0,
            ffa11: ((value >> 11) & 0b1) > 0,
            ffa12: ((value >> 12) & 0b1) > 0,
            ffa13: ((value >> 13) & 0b1) > 0,
            ffa14: ((value >> 14) & 0b1) > 0,
            ffa15: ((value >> 15) & 0b1) > 0,
            ffa16: ((value >> 16) & 0b1) > 0,
            ffa17: ((value >> 17) & 0b1) > 0,
            ffa18: ((value >> 18) & 0b1) > 0,
            ffa19: ((value >> 19) & 0b1) > 0,
            ffa20: ((value >> 20) & 0b1) > 0,
            ffa21: ((value >> 21) & 0b1) > 0,
            ffa22: ((value >> 22) & 0b1) > 0,
            ffa23: ((value >> 23) & 0b1) > 0,
            ffa24: ((value >> 24) & 0b1) > 0,
            ffa25: ((value >> 25) & 0b1) > 0,
            ffa26: ((value >> 26) & 0b1) > 0,
            ffa27: ((value >> 27) & 0b1) > 0,
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
                | ((self.ffa0 as u32) << 0)
                | ((self.ffa1 as u32) << 1)
                | ((self.ffa2 as u32) << 2)
                | ((self.ffa3 as u32) << 3)
                | ((self.ffa4 as u32) << 4)
                | ((self.ffa5 as u32) << 5)
                | ((self.ffa6 as u32) << 6)
                | ((self.ffa7 as u32) << 7)
                | ((self.ffa8 as u32) << 8)
                | ((self.ffa9 as u32) << 9)
                | ((self.ffa10 as u32) << 10)
                | ((self.ffa11 as u32) << 11)
                | ((self.ffa12 as u32) << 12)
                | ((self.ffa13 as u32) << 13)
                | ((self.ffa14 as u32) << 14)
                | ((self.ffa15 as u32) << 15)
                | ((self.ffa16 as u32) << 16)
                | ((self.ffa17 as u32) << 17)
                | ((self.ffa18 as u32) << 18)
                | ((self.ffa19 as u32) << 19)
                | ((self.ffa20 as u32) << 20)
                | ((self.ffa21 as u32) << 21)
                | ((self.ffa22 as u32) << 22)
                | ((self.ffa23 as u32) << 23)
                | ((self.ffa24 as u32) << 24)
                | ((self.ffa25 as u32) << 25)
                | ((self.ffa26 as u32) << 26)
                | ((self.ffa27 as u32) << 27)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x214u32) as *mut u32, value) };
        }
    }
}
/// CAN filter activation register
pub mod fa1r {
    pub struct ReadonlyCache {
        /// Filter active
        pub fact0: bool,
        /// Filter active
        pub fact1: bool,
        /// Filter active
        pub fact2: bool,
        /// Filter active
        pub fact3: bool,
        /// Filter active
        pub fact4: bool,
        /// Filter active
        pub fact5: bool,
        /// Filter active
        pub fact6: bool,
        /// Filter active
        pub fact7: bool,
        /// Filter active
        pub fact8: bool,
        /// Filter active
        pub fact9: bool,
        /// Filter active
        pub fact10: bool,
        /// Filter active
        pub fact11: bool,
        /// Filter active
        pub fact12: bool,
        /// Filter active
        pub fact13: bool,
        /// Filter active
        pub fact14: bool,
        /// Filter active
        pub fact15: bool,
        /// Filter active
        pub fact16: bool,
        /// Filter active
        pub fact17: bool,
        /// Filter active
        pub fact18: bool,
        /// Filter active
        pub fact19: bool,
        /// Filter active
        pub fact20: bool,
        /// Filter active
        pub fact21: bool,
        /// Filter active
        pub fact22: bool,
        /// Filter active
        pub fact23: bool,
        /// Filter active
        pub fact24: bool,
        /// Filter active
        pub fact25: bool,
        /// Filter active
        pub fact26: bool,
        /// Filter active
        pub fact27: bool,
    }
    pub struct Cache {
        /// Filter active
        pub fact0: bool,
        /// Filter active
        pub fact1: bool,
        /// Filter active
        pub fact2: bool,
        /// Filter active
        pub fact3: bool,
        /// Filter active
        pub fact4: bool,
        /// Filter active
        pub fact5: bool,
        /// Filter active
        pub fact6: bool,
        /// Filter active
        pub fact7: bool,
        /// Filter active
        pub fact8: bool,
        /// Filter active
        pub fact9: bool,
        /// Filter active
        pub fact10: bool,
        /// Filter active
        pub fact11: bool,
        /// Filter active
        pub fact12: bool,
        /// Filter active
        pub fact13: bool,
        /// Filter active
        pub fact14: bool,
        /// Filter active
        pub fact15: bool,
        /// Filter active
        pub fact16: bool,
        /// Filter active
        pub fact17: bool,
        /// Filter active
        pub fact18: bool,
        /// Filter active
        pub fact19: bool,
        /// Filter active
        pub fact20: bool,
        /// Filter active
        pub fact21: bool,
        /// Filter active
        pub fact22: bool,
        /// Filter active
        pub fact23: bool,
        /// Filter active
        pub fact24: bool,
        /// Filter active
        pub fact25: bool,
        /// Filter active
        pub fact26: bool,
        /// Filter active
        pub fact27: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x21Cu32) as *mut u32) };
        ReadonlyCache {
            fact0: ((value >> 0) & 0b1) > 0,
            fact1: ((value >> 1) & 0b1) > 0,
            fact2: ((value >> 2) & 0b1) > 0,
            fact3: ((value >> 3) & 0b1) > 0,
            fact4: ((value >> 4) & 0b1) > 0,
            fact5: ((value >> 5) & 0b1) > 0,
            fact6: ((value >> 6) & 0b1) > 0,
            fact7: ((value >> 7) & 0b1) > 0,
            fact8: ((value >> 8) & 0b1) > 0,
            fact9: ((value >> 9) & 0b1) > 0,
            fact10: ((value >> 10) & 0b1) > 0,
            fact11: ((value >> 11) & 0b1) > 0,
            fact12: ((value >> 12) & 0b1) > 0,
            fact13: ((value >> 13) & 0b1) > 0,
            fact14: ((value >> 14) & 0b1) > 0,
            fact15: ((value >> 15) & 0b1) > 0,
            fact16: ((value >> 16) & 0b1) > 0,
            fact17: ((value >> 17) & 0b1) > 0,
            fact18: ((value >> 18) & 0b1) > 0,
            fact19: ((value >> 19) & 0b1) > 0,
            fact20: ((value >> 20) & 0b1) > 0,
            fact21: ((value >> 21) & 0b1) > 0,
            fact22: ((value >> 22) & 0b1) > 0,
            fact23: ((value >> 23) & 0b1) > 0,
            fact24: ((value >> 24) & 0b1) > 0,
            fact25: ((value >> 25) & 0b1) > 0,
            fact26: ((value >> 26) & 0b1) > 0,
            fact27: ((value >> 27) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x21Cu32) as *mut u32) };
        Cache {
            fact0: ((value >> 0) & 0b1) > 0,
            fact1: ((value >> 1) & 0b1) > 0,
            fact2: ((value >> 2) & 0b1) > 0,
            fact3: ((value >> 3) & 0b1) > 0,
            fact4: ((value >> 4) & 0b1) > 0,
            fact5: ((value >> 5) & 0b1) > 0,
            fact6: ((value >> 6) & 0b1) > 0,
            fact7: ((value >> 7) & 0b1) > 0,
            fact8: ((value >> 8) & 0b1) > 0,
            fact9: ((value >> 9) & 0b1) > 0,
            fact10: ((value >> 10) & 0b1) > 0,
            fact11: ((value >> 11) & 0b1) > 0,
            fact12: ((value >> 12) & 0b1) > 0,
            fact13: ((value >> 13) & 0b1) > 0,
            fact14: ((value >> 14) & 0b1) > 0,
            fact15: ((value >> 15) & 0b1) > 0,
            fact16: ((value >> 16) & 0b1) > 0,
            fact17: ((value >> 17) & 0b1) > 0,
            fact18: ((value >> 18) & 0b1) > 0,
            fact19: ((value >> 19) & 0b1) > 0,
            fact20: ((value >> 20) & 0b1) > 0,
            fact21: ((value >> 21) & 0b1) > 0,
            fact22: ((value >> 22) & 0b1) > 0,
            fact23: ((value >> 23) & 0b1) > 0,
            fact24: ((value >> 24) & 0b1) > 0,
            fact25: ((value >> 25) & 0b1) > 0,
            fact26: ((value >> 26) & 0b1) > 0,
            fact27: ((value >> 27) & 0b1) > 0,
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
                | ((self.fact0 as u32) << 0)
                | ((self.fact1 as u32) << 1)
                | ((self.fact2 as u32) << 2)
                | ((self.fact3 as u32) << 3)
                | ((self.fact4 as u32) << 4)
                | ((self.fact5 as u32) << 5)
                | ((self.fact6 as u32) << 6)
                | ((self.fact7 as u32) << 7)
                | ((self.fact8 as u32) << 8)
                | ((self.fact9 as u32) << 9)
                | ((self.fact10 as u32) << 10)
                | ((self.fact11 as u32) << 11)
                | ((self.fact12 as u32) << 12)
                | ((self.fact13 as u32) << 13)
                | ((self.fact14 as u32) << 14)
                | ((self.fact15 as u32) << 15)
                | ((self.fact16 as u32) << 16)
                | ((self.fact17 as u32) << 17)
                | ((self.fact18 as u32) << 18)
                | ((self.fact19 as u32) << 19)
                | ((self.fact20 as u32) << 20)
                | ((self.fact21 as u32) << 21)
                | ((self.fact22 as u32) << 22)
                | ((self.fact23 as u32) << 23)
                | ((self.fact24 as u32) << 24)
                | ((self.fact25 as u32) << 25)
                | ((self.fact26 as u32) << 26)
                | ((self.fact27 as u32) << 27)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x21Cu32) as *mut u32, value) };
        }
    }
}
/// Filter bank 0 register 1
pub mod f0r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x240u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x240u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x240u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 0 register 2
pub mod f0r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x244u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x244u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x244u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 1 register 1
pub mod f1r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x248u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x248u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x248u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 1 register 2
pub mod f1r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x24Cu32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x24Cu32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x24Cu32) as *mut u32, value) };
        }
    }
}
/// Filter bank 2 register 1
pub mod f2r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x250u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x250u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x250u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 2 register 2
pub mod f2r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x254u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x254u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x254u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 3 register 1
pub mod f3r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x258u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x258u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x258u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 3 register 2
pub mod f3r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x25Cu32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x25Cu32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x25Cu32) as *mut u32, value) };
        }
    }
}
/// Filter bank 4 register 1
pub mod f4r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x260u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x260u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x260u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 4 register 2
pub mod f4r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x264u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x264u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x264u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 5 register 1
pub mod f5r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x268u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x268u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x268u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 5 register 2
pub mod f5r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x26Cu32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x26Cu32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x26Cu32) as *mut u32, value) };
        }
    }
}
/// Filter bank 6 register 1
pub mod f6r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x270u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x270u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x270u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 6 register 2
pub mod f6r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x274u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x274u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x274u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 7 register 1
pub mod f7r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x278u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x278u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x278u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 7 register 2
pub mod f7r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x27Cu32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x27Cu32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x27Cu32) as *mut u32, value) };
        }
    }
}
/// Filter bank 8 register 1
pub mod f8r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x280u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x280u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x280u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 8 register 2
pub mod f8r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x284u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x284u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x284u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 9 register 1
pub mod f9r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x288u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x288u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x288u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 9 register 2
pub mod f9r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x28Cu32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x28Cu32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x28Cu32) as *mut u32, value) };
        }
    }
}
/// Filter bank 10 register 1
pub mod f10r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x290u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x290u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x290u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 10 register 2
pub mod f10r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x294u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x294u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x294u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 11 register 1
pub mod f11r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x298u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x298u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x298u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 11 register 2
pub mod f11r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x29Cu32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x29Cu32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x29Cu32) as *mut u32, value) };
        }
    }
}
/// Filter bank 4 register 1
pub mod f12r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2A0u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2A0u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2A0u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 12 register 2
pub mod f12r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2A4u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2A4u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2A4u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 13 register 1
pub mod f13r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2A8u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2A8u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2A8u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 13 register 2
pub mod f13r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2ACu32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2ACu32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2ACu32) as *mut u32, value) };
        }
    }
}
/// Filter bank 14 register 1
pub mod f14r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2B0u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2B0u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2B0u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 14 register 2
pub mod f14r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2B4u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2B4u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2B4u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 15 register 1
pub mod f15r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2B8u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2B8u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2B8u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 15 register 2
pub mod f15r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2BCu32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2BCu32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2BCu32) as *mut u32, value) };
        }
    }
}
/// Filter bank 16 register 1
pub mod f16r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2C0u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2C0u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2C0u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 16 register 2
pub mod f16r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2C4u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2C4u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2C4u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 17 register 1
pub mod f17r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2C8u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2C8u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2C8u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 17 register 2
pub mod f17r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2CCu32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2CCu32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2CCu32) as *mut u32, value) };
        }
    }
}
/// Filter bank 18 register 1
pub mod f18r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2D0u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2D0u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2D0u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 18 register 2
pub mod f18r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2D4u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2D4u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2D4u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 19 register 1
pub mod f19r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2D8u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2D8u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2D8u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 19 register 2
pub mod f19r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2DCu32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2DCu32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2DCu32) as *mut u32, value) };
        }
    }
}
/// Filter bank 20 register 1
pub mod f20r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2E0u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2E0u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2E0u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 20 register 2
pub mod f20r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2E4u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2E4u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2E4u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 21 register 1
pub mod f21r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2E8u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2E8u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2E8u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 21 register 2
pub mod f21r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2ECu32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2ECu32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2ECu32) as *mut u32, value) };
        }
    }
}
/// Filter bank 22 register 1
pub mod f22r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2F0u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2F0u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2F0u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 22 register 2
pub mod f22r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2F4u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2F4u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2F4u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 23 register 1
pub mod f23r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2F8u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2F8u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2F8u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 23 register 2
pub mod f23r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2FCu32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x2FCu32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x2FCu32) as *mut u32, value) };
        }
    }
}
/// Filter bank 24 register 1
pub mod f24r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x300u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x300u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x300u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 24 register 2
pub mod f24r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x304u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x304u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x304u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 25 register 1
pub mod f25r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x308u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x308u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x308u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 25 register 2
pub mod f25r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x30Cu32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x30Cu32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x30Cu32) as *mut u32, value) };
        }
    }
}
/// Filter bank 26 register 1
pub mod f26r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x310u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x310u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x310u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 26 register 2
pub mod f26r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x314u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x314u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x314u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 27 register 1
pub mod f27r1 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x318u32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x318u32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x318u32) as *mut u32, value) };
        }
    }
}
/// Filter bank 27 register 2
pub mod f27r2 {
    pub struct ReadonlyCache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub struct Cache {
        /// Filter bits
        pub fb0: bool,
        /// Filter bits
        pub fb1: bool,
        /// Filter bits
        pub fb2: bool,
        /// Filter bits
        pub fb3: bool,
        /// Filter bits
        pub fb4: bool,
        /// Filter bits
        pub fb5: bool,
        /// Filter bits
        pub fb6: bool,
        /// Filter bits
        pub fb7: bool,
        /// Filter bits
        pub fb8: bool,
        /// Filter bits
        pub fb9: bool,
        /// Filter bits
        pub fb10: bool,
        /// Filter bits
        pub fb11: bool,
        /// Filter bits
        pub fb12: bool,
        /// Filter bits
        pub fb13: bool,
        /// Filter bits
        pub fb14: bool,
        /// Filter bits
        pub fb15: bool,
        /// Filter bits
        pub fb16: bool,
        /// Filter bits
        pub fb17: bool,
        /// Filter bits
        pub fb18: bool,
        /// Filter bits
        pub fb19: bool,
        /// Filter bits
        pub fb20: bool,
        /// Filter bits
        pub fb21: bool,
        /// Filter bits
        pub fb22: bool,
        /// Filter bits
        pub fb23: bool,
        /// Filter bits
        pub fb24: bool,
        /// Filter bits
        pub fb25: bool,
        /// Filter bits
        pub fb26: bool,
        /// Filter bits
        pub fb27: bool,
        /// Filter bits
        pub fb28: bool,
        /// Filter bits
        pub fb29: bool,
        /// Filter bits
        pub fb30: bool,
        /// Filter bits
        pub fb31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x31Cu32) as *mut u32) };
        ReadonlyCache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40006400u32 + 0x31Cu32) as *mut u32) };
        Cache {
            fb0: ((value >> 0) & 0b1) > 0,
            fb1: ((value >> 1) & 0b1) > 0,
            fb2: ((value >> 2) & 0b1) > 0,
            fb3: ((value >> 3) & 0b1) > 0,
            fb4: ((value >> 4) & 0b1) > 0,
            fb5: ((value >> 5) & 0b1) > 0,
            fb6: ((value >> 6) & 0b1) > 0,
            fb7: ((value >> 7) & 0b1) > 0,
            fb8: ((value >> 8) & 0b1) > 0,
            fb9: ((value >> 9) & 0b1) > 0,
            fb10: ((value >> 10) & 0b1) > 0,
            fb11: ((value >> 11) & 0b1) > 0,
            fb12: ((value >> 12) & 0b1) > 0,
            fb13: ((value >> 13) & 0b1) > 0,
            fb14: ((value >> 14) & 0b1) > 0,
            fb15: ((value >> 15) & 0b1) > 0,
            fb16: ((value >> 16) & 0b1) > 0,
            fb17: ((value >> 17) & 0b1) > 0,
            fb18: ((value >> 18) & 0b1) > 0,
            fb19: ((value >> 19) & 0b1) > 0,
            fb20: ((value >> 20) & 0b1) > 0,
            fb21: ((value >> 21) & 0b1) > 0,
            fb22: ((value >> 22) & 0b1) > 0,
            fb23: ((value >> 23) & 0b1) > 0,
            fb24: ((value >> 24) & 0b1) > 0,
            fb25: ((value >> 25) & 0b1) > 0,
            fb26: ((value >> 26) & 0b1) > 0,
            fb27: ((value >> 27) & 0b1) > 0,
            fb28: ((value >> 28) & 0b1) > 0,
            fb29: ((value >> 29) & 0b1) > 0,
            fb30: ((value >> 30) & 0b1) > 0,
            fb31: ((value >> 31) & 0b1) > 0,
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
                | ((self.fb0 as u32) << 0)
                | ((self.fb1 as u32) << 1)
                | ((self.fb2 as u32) << 2)
                | ((self.fb3 as u32) << 3)
                | ((self.fb4 as u32) << 4)
                | ((self.fb5 as u32) << 5)
                | ((self.fb6 as u32) << 6)
                | ((self.fb7 as u32) << 7)
                | ((self.fb8 as u32) << 8)
                | ((self.fb9 as u32) << 9)
                | ((self.fb10 as u32) << 10)
                | ((self.fb11 as u32) << 11)
                | ((self.fb12 as u32) << 12)
                | ((self.fb13 as u32) << 13)
                | ((self.fb14 as u32) << 14)
                | ((self.fb15 as u32) << 15)
                | ((self.fb16 as u32) << 16)
                | ((self.fb17 as u32) << 17)
                | ((self.fb18 as u32) << 18)
                | ((self.fb19 as u32) << 19)
                | ((self.fb20 as u32) << 20)
                | ((self.fb21 as u32) << 21)
                | ((self.fb22 as u32) << 22)
                | ((self.fb23 as u32) << 23)
                | ((self.fb24 as u32) << 24)
                | ((self.fb25 as u32) << 25)
                | ((self.fb26 as u32) << 26)
                | ((self.fb27 as u32) << 27)
                | ((self.fb28 as u32) << 28)
                | ((self.fb29 as u32) << 29)
                | ((self.fb30 as u32) << 30)
                | ((self.fb31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40006400u32 + 0x31Cu32) as *mut u32, value) };
        }
    }
}
/// USB High Priority/CAN_TX interrupts
pub const INTERRUPT_USB_HP_CAN_TX: u32 = 19;
/// USB Low Priority/CAN_RX0 interrupts
pub const INTERRUPT_USB_LP_CAN_RX0: u32 = 20;
/// CAN_RX1 interrupt
pub const INTERRUPT_CAN_RX1: u32 = 21;
/// CAN_SCE interrupt
pub const INTERRUPT_CAN_SCE: u32 = 22;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40006400</baseAddress>
  <description>Controller area network</description>
  <groupName>CAN</groupName>
  <interrupt>
    <description>
                    USB High Priority/CAN_TX
                    interrupts
                </description>
    <name>USB_HP_CAN_TX</name>
    <value>19</value>
  </interrupt>
  <interrupt>
    <description>
                    USB Low Priority/CAN_RX0
                    interrupts
                </description>
    <name>USB_LP_CAN_RX0</name>
    <value>20</value>
  </interrupt>
  <interrupt>
    <description>CAN_RX1 interrupt</description>
    <name>CAN_RX1</name>
    <value>21</value>
  </interrupt>
  <interrupt>
    <description>CAN_SCE interrupt</description>
    <name>CAN_SCE</name>
    <value>22</value>
  </interrupt>
  <name>CAN</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>master control register</description>
      <displayName>MCR</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBF</description>
          <name>DBF</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RESET</description>
          <name>RESET</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TTCM</description>
          <name>TTCM</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ABOM</description>
          <name>ABOM</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AWUM</description>
          <name>AWUM</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>NART</description>
          <name>NART</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RFLM</description>
          <name>RFLM</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXFP</description>
          <name>TXFP</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SLEEP</description>
          <name>SLEEP</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>INRQ</description>
          <name>INRQ</name>
        </field>
      </fields>
      <name>MCR</name>
      <resetValue>0x00010002</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x4</addressOffset>
      <description>master status register</description>
      <displayName>MSR</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RX</description>
          <name>RX</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SAMP</description>
          <name>SAMP</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXM</description>
          <name>RXM</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXM</description>
          <name>TXM</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SLAKI</description>
          <name>SLAKI</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WKUI</description>
          <name>WKUI</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ERRI</description>
          <name>ERRI</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SLAK</description>
          <name>SLAK</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>INAK</description>
          <name>INAK</name>
        </field>
      </fields>
      <name>MSR</name>
      <resetValue>0x00000C02</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x8</addressOffset>
      <description>transmit status register</description>
      <displayName>TSR</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Lowest priority flag for mailbox
                                2
                            </description>
          <name>LOW2</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Lowest priority flag for mailbox
                                1
                            </description>
          <name>LOW1</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Lowest priority flag for mailbox
                                0
                            </description>
          <name>LOW0</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Lowest priority flag for mailbox
                                2
                            </description>
          <name>TME2</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Lowest priority flag for mailbox
                                1
                            </description>
          <name>TME1</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Lowest priority flag for mailbox
                                0
                            </description>
          <name>TME0</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
          <description>CODE</description>
          <name>CODE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ABRQ2</description>
          <name>ABRQ2</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TERR2</description>
          <name>TERR2</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ALST2</description>
          <name>ALST2</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXOK2</description>
          <name>TXOK2</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RQCP2</description>
          <name>RQCP2</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ABRQ1</description>
          <name>ABRQ1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TERR1</description>
          <name>TERR1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ALST1</description>
          <name>ALST1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXOK1</description>
          <name>TXOK1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RQCP1</description>
          <name>RQCP1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ABRQ0</description>
          <name>ABRQ0</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TERR0</description>
          <name>TERR0</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ALST0</description>
          <name>ALST0</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXOK0</description>
          <name>TXOK0</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RQCP0</description>
          <name>RQCP0</name>
        </field>
      </fields>
      <name>TSR</name>
      <resetValue>0x1C000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0xC</addressOffset>
      <description>receive FIFO 0 register</description>
      <displayName>RF0R</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RFOM0</description>
          <name>RFOM0</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FOVR0</description>
          <name>FOVR0</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FULL0</description>
          <name>FULL0</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>FMP0</description>
          <name>FMP0</name>
        </field>
      </fields>
      <name>RF0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x10</addressOffset>
      <description>receive FIFO 1 register</description>
      <displayName>RF1R</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RFOM1</description>
          <name>RFOM1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FOVR1</description>
          <name>FOVR1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FULL1</description>
          <name>FULL1</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>FMP1</description>
          <name>FMP1</name>
        </field>
      </fields>
      <name>RF1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>interrupt enable register</description>
      <displayName>IER</displayName>
      <fields>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SLKIE</description>
          <name>SLKIE</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WKUIE</description>
          <name>WKUIE</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ERRIE</description>
          <name>ERRIE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LECIE</description>
          <name>LECIE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>BOFIE</description>
          <name>BOFIE</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EPVIE</description>
          <name>EPVIE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EWGIE</description>
          <name>EWGIE</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FOVIE1</description>
          <name>FOVIE1</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FFIE1</description>
          <name>FFIE1</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FMPIE1</description>
          <name>FMPIE1</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FOVIE0</description>
          <name>FOVIE0</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FFIE0</description>
          <name>FFIE0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FMPIE0</description>
          <name>FMPIE0</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TMEIE</description>
          <name>TMEIE</name>
        </field>
      </fields>
      <name>IER</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x18</addressOffset>
      <description>error status register</description>
      <displayName>ESR</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>REC</description>
          <name>REC</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>TEC</description>
          <name>TEC</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>LEC</description>
          <name>LEC</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>BOFF</description>
          <name>BOFF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EPVF</description>
          <name>EPVF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EWGF</description>
          <name>EWGF</name>
        </field>
      </fields>
      <name>ESR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>bit timing register</description>
      <displayName>BTR</displayName>
      <fields>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SILM</description>
          <name>SILM</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LBKM</description>
          <name>LBKM</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
          <description>SJW</description>
          <name>SJW</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>3</bitWidth>
          <description>TS2</description>
          <name>TS2</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>TS1</description>
          <name>TS1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>10</bitWidth>
          <description>BRP</description>
          <name>BRP</name>
        </field>
      </fields>
      <name>BTR</name>
      <resetValue>0x01230000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x180</addressOffset>
      <description>TX mailbox identifier register</description>
      <displayName>TI0R</displayName>
      <fields>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>11</bitWidth>
          <description>STID</description>
          <name>STID</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>18</bitWidth>
          <description>EXID</description>
          <name>EXID</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDE</description>
          <name>IDE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTR</description>
          <name>RTR</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXRQ</description>
          <name>TXRQ</name>
        </field>
      </fields>
      <name>TI0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x184</addressOffset>
      <description>
                        mailbox data length control and time stamp
                        register
                    </description>
      <displayName>TDT0R</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>TIME</description>
          <name>TIME</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TGT</description>
          <name>TGT</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DLC</description>
          <name>DLC</name>
        </field>
      </fields>
      <name>TDT0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x188</addressOffset>
      <description>mailbox data low register</description>
      <displayName>TDL0R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA3</description>
          <name>DATA3</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA2</description>
          <name>DATA2</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA1</description>
          <name>DATA1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA0</description>
          <name>DATA0</name>
        </field>
      </fields>
      <name>TDL0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18C</addressOffset>
      <description>mailbox data high register</description>
      <displayName>TDH0R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA7</description>
          <name>DATA7</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA6</description>
          <name>DATA6</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA5</description>
          <name>DATA5</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA4</description>
          <name>DATA4</name>
        </field>
      </fields>
      <name>TDH0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x190</addressOffset>
      <description>TX mailbox identifier register</description>
      <displayName>TI1R</displayName>
      <fields>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>11</bitWidth>
          <description>STID</description>
          <name>STID</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>18</bitWidth>
          <description>EXID</description>
          <name>EXID</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDE</description>
          <name>IDE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTR</description>
          <name>RTR</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXRQ</description>
          <name>TXRQ</name>
        </field>
      </fields>
      <name>TI1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x194</addressOffset>
      <description>
                        mailbox data length control and time stamp
                        register
                    </description>
      <displayName>TDT1R</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>TIME</description>
          <name>TIME</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TGT</description>
          <name>TGT</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DLC</description>
          <name>DLC</name>
        </field>
      </fields>
      <name>TDT1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x198</addressOffset>
      <description>mailbox data low register</description>
      <displayName>TDL1R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA3</description>
          <name>DATA3</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA2</description>
          <name>DATA2</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA1</description>
          <name>DATA1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA0</description>
          <name>DATA0</name>
        </field>
      </fields>
      <name>TDL1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x19C</addressOffset>
      <description>mailbox data high register</description>
      <displayName>TDH1R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA7</description>
          <name>DATA7</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA6</description>
          <name>DATA6</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA5</description>
          <name>DATA5</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA4</description>
          <name>DATA4</name>
        </field>
      </fields>
      <name>TDH1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1A0</addressOffset>
      <description>TX mailbox identifier register</description>
      <displayName>TI2R</displayName>
      <fields>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>11</bitWidth>
          <description>STID</description>
          <name>STID</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>18</bitWidth>
          <description>EXID</description>
          <name>EXID</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDE</description>
          <name>IDE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTR</description>
          <name>RTR</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXRQ</description>
          <name>TXRQ</name>
        </field>
      </fields>
      <name>TI2R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1A4</addressOffset>
      <description>
                        mailbox data length control and time stamp
                        register
                    </description>
      <displayName>TDT2R</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>TIME</description>
          <name>TIME</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TGT</description>
          <name>TGT</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DLC</description>
          <name>DLC</name>
        </field>
      </fields>
      <name>TDT2R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1A8</addressOffset>
      <description>mailbox data low register</description>
      <displayName>TDL2R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA3</description>
          <name>DATA3</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA2</description>
          <name>DATA2</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA1</description>
          <name>DATA1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA0</description>
          <name>DATA0</name>
        </field>
      </fields>
      <name>TDL2R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1AC</addressOffset>
      <description>mailbox data high register</description>
      <displayName>TDH2R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA7</description>
          <name>DATA7</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA6</description>
          <name>DATA6</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA5</description>
          <name>DATA5</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA4</description>
          <name>DATA4</name>
        </field>
      </fields>
      <name>TDH2R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1B0</addressOffset>
      <description>
                        receive FIFO mailbox identifier
                        register
                    </description>
      <displayName>RI0R</displayName>
      <fields>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>11</bitWidth>
          <description>STID</description>
          <name>STID</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>18</bitWidth>
          <description>EXID</description>
          <name>EXID</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDE</description>
          <name>IDE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTR</description>
          <name>RTR</name>
        </field>
      </fields>
      <name>RI0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1B4</addressOffset>
      <description>
                        receive FIFO mailbox data length control and
                        time stamp register
                    </description>
      <displayName>RDT0R</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>TIME</description>
          <name>TIME</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>FMI</description>
          <name>FMI</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DLC</description>
          <name>DLC</name>
        </field>
      </fields>
      <name>RDT0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1B8</addressOffset>
      <description>
                        receive FIFO mailbox data low
                        register
                    </description>
      <displayName>RDL0R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA3</description>
          <name>DATA3</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA2</description>
          <name>DATA2</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA1</description>
          <name>DATA1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA0</description>
          <name>DATA0</name>
        </field>
      </fields>
      <name>RDL0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1BC</addressOffset>
      <description>
                        receive FIFO mailbox data high
                        register
                    </description>
      <displayName>RDH0R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA7</description>
          <name>DATA7</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA6</description>
          <name>DATA6</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA5</description>
          <name>DATA5</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA4</description>
          <name>DATA4</name>
        </field>
      </fields>
      <name>RDH0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1C0</addressOffset>
      <description>
                        receive FIFO mailbox identifier
                        register
                    </description>
      <displayName>RI1R</displayName>
      <fields>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>11</bitWidth>
          <description>STID</description>
          <name>STID</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>18</bitWidth>
          <description>EXID</description>
          <name>EXID</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDE</description>
          <name>IDE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTR</description>
          <name>RTR</name>
        </field>
      </fields>
      <name>RI1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1C4</addressOffset>
      <description>
                        receive FIFO mailbox data length control and
                        time stamp register
                    </description>
      <displayName>RDT1R</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>TIME</description>
          <name>TIME</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>FMI</description>
          <name>FMI</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DLC</description>
          <name>DLC</name>
        </field>
      </fields>
      <name>RDT1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1C8</addressOffset>
      <description>
                        receive FIFO mailbox data low
                        register
                    </description>
      <displayName>RDL1R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA3</description>
          <name>DATA3</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA2</description>
          <name>DATA2</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA1</description>
          <name>DATA1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA0</description>
          <name>DATA0</name>
        </field>
      </fields>
      <name>RDL1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1CC</addressOffset>
      <description>
                        receive FIFO mailbox data high
                        register
                    </description>
      <displayName>RDH1R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA7</description>
          <name>DATA7</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA6</description>
          <name>DATA6</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA5</description>
          <name>DATA5</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA4</description>
          <name>DATA4</name>
        </field>
      </fields>
      <name>RDH1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x200</addressOffset>
      <description>filter master register</description>
      <displayName>FMR</displayName>
      <fields>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>6</bitWidth>
          <description>CAN2 start bank</description>
          <name>CAN2SB</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter init mode</description>
          <name>FINIT</name>
        </field>
      </fields>
      <name>FMR</name>
      <resetValue>0x2A1C0E01</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x204</addressOffset>
      <description>filter mode register</description>
      <displayName>FM1R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM27</name>
        </field>
      </fields>
      <name>FM1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20C</addressOffset>
      <description>filter scale register</description>
      <displayName>FS1R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC27</name>
        </field>
      </fields>
      <name>FS1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x214</addressOffset>
      <description>
                        filter FIFO assignment
                        register
                    </description>
      <displayName>FFA1R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                0
                            </description>
          <name>FFA0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                1
                            </description>
          <name>FFA1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                2
                            </description>
          <name>FFA2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                3
                            </description>
          <name>FFA3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                4
                            </description>
          <name>FFA4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                5
                            </description>
          <name>FFA5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                6
                            </description>
          <name>FFA6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                7
                            </description>
          <name>FFA7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                8
                            </description>
          <name>FFA8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                9
                            </description>
          <name>FFA9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                10
                            </description>
          <name>FFA10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                11
                            </description>
          <name>FFA11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                12
                            </description>
          <name>FFA12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                13
                            </description>
          <name>FFA13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                14
                            </description>
          <name>FFA14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                15
                            </description>
          <name>FFA15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                16
                            </description>
          <name>FFA16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                17
                            </description>
          <name>FFA17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                18
                            </description>
          <name>FFA18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                19
                            </description>
          <name>FFA19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                20
                            </description>
          <name>FFA20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                21
                            </description>
          <name>FFA21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                22
                            </description>
          <name>FFA22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                23
                            </description>
          <name>FFA23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                24
                            </description>
          <name>FFA24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                25
                            </description>
          <name>FFA25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                26
                            </description>
          <name>FFA26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Filter FIFO assignment for filter
                                27
                            </description>
          <name>FFA27</name>
        </field>
      </fields>
      <name>FFA1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x21C</addressOffset>
      <description>CAN filter activation register</description>
      <displayName>FA1R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT27</name>
        </field>
      </fields>
      <name>FA1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x240</addressOffset>
      <description>Filter bank 0 register 1</description>
      <displayName>F0R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F0R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x244</addressOffset>
      <description>Filter bank 0 register 2</description>
      <displayName>F0R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F0R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x248</addressOffset>
      <description>Filter bank 1 register 1</description>
      <displayName>F1R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F1R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x24C</addressOffset>
      <description>Filter bank 1 register 2</description>
      <displayName>F1R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F1R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x250</addressOffset>
      <description>Filter bank 2 register 1</description>
      <displayName>F2R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F2R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x254</addressOffset>
      <description>Filter bank 2 register 2</description>
      <displayName>F2R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F2R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x258</addressOffset>
      <description>Filter bank 3 register 1</description>
      <displayName>F3R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F3R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x25C</addressOffset>
      <description>Filter bank 3 register 2</description>
      <displayName>F3R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F3R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x260</addressOffset>
      <description>Filter bank 4 register 1</description>
      <displayName>F4R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F4R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x264</addressOffset>
      <description>Filter bank 4 register 2</description>
      <displayName>F4R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F4R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x268</addressOffset>
      <description>Filter bank 5 register 1</description>
      <displayName>F5R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F5R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x26C</addressOffset>
      <description>Filter bank 5 register 2</description>
      <displayName>F5R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F5R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x270</addressOffset>
      <description>Filter bank 6 register 1</description>
      <displayName>F6R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F6R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x274</addressOffset>
      <description>Filter bank 6 register 2</description>
      <displayName>F6R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F6R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x278</addressOffset>
      <description>Filter bank 7 register 1</description>
      <displayName>F7R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F7R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x27C</addressOffset>
      <description>Filter bank 7 register 2</description>
      <displayName>F7R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F7R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x280</addressOffset>
      <description>Filter bank 8 register 1</description>
      <displayName>F8R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F8R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x284</addressOffset>
      <description>Filter bank 8 register 2</description>
      <displayName>F8R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F8R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x288</addressOffset>
      <description>Filter bank 9 register 1</description>
      <displayName>F9R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F9R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28C</addressOffset>
      <description>Filter bank 9 register 2</description>
      <displayName>F9R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F9R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x290</addressOffset>
      <description>Filter bank 10 register 1</description>
      <displayName>F10R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F10R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x294</addressOffset>
      <description>Filter bank 10 register 2</description>
      <displayName>F10R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F10R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x298</addressOffset>
      <description>Filter bank 11 register 1</description>
      <displayName>F11R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F11R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x29C</addressOffset>
      <description>Filter bank 11 register 2</description>
      <displayName>F11R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F11R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2A0</addressOffset>
      <description>Filter bank 4 register 1</description>
      <displayName>F12R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F12R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2A4</addressOffset>
      <description>Filter bank 12 register 2</description>
      <displayName>F12R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F12R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2A8</addressOffset>
      <description>Filter bank 13 register 1</description>
      <displayName>F13R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F13R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2AC</addressOffset>
      <description>Filter bank 13 register 2</description>
      <displayName>F13R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F13R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2B0</addressOffset>
      <description>Filter bank 14 register 1</description>
      <displayName>F14R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F14R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2B4</addressOffset>
      <description>Filter bank 14 register 2</description>
      <displayName>F14R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F14R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2B8</addressOffset>
      <description>Filter bank 15 register 1</description>
      <displayName>F15R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F15R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2BC</addressOffset>
      <description>Filter bank 15 register 2</description>
      <displayName>F15R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F15R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2C0</addressOffset>
      <description>Filter bank 16 register 1</description>
      <displayName>F16R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F16R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2C4</addressOffset>
      <description>Filter bank 16 register 2</description>
      <displayName>F16R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F16R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2C8</addressOffset>
      <description>Filter bank 17 register 1</description>
      <displayName>F17R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F17R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2CC</addressOffset>
      <description>Filter bank 17 register 2</description>
      <displayName>F17R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F17R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2D0</addressOffset>
      <description>Filter bank 18 register 1</description>
      <displayName>F18R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F18R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2D4</addressOffset>
      <description>Filter bank 18 register 2</description>
      <displayName>F18R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F18R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2D8</addressOffset>
      <description>Filter bank 19 register 1</description>
      <displayName>F19R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F19R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2DC</addressOffset>
      <description>Filter bank 19 register 2</description>
      <displayName>F19R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F19R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2E0</addressOffset>
      <description>Filter bank 20 register 1</description>
      <displayName>F20R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F20R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2E4</addressOffset>
      <description>Filter bank 20 register 2</description>
      <displayName>F20R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F20R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2E8</addressOffset>
      <description>Filter bank 21 register 1</description>
      <displayName>F21R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F21R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2EC</addressOffset>
      <description>Filter bank 21 register 2</description>
      <displayName>F21R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F21R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2F0</addressOffset>
      <description>Filter bank 22 register 1</description>
      <displayName>F22R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F22R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2F4</addressOffset>
      <description>Filter bank 22 register 2</description>
      <displayName>F22R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F22R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2F8</addressOffset>
      <description>Filter bank 23 register 1</description>
      <displayName>F23R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F23R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2FC</addressOffset>
      <description>Filter bank 23 register 2</description>
      <displayName>F23R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F23R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x300</addressOffset>
      <description>Filter bank 24 register 1</description>
      <displayName>F24R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F24R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x304</addressOffset>
      <description>Filter bank 24 register 2</description>
      <displayName>F24R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F24R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x308</addressOffset>
      <description>Filter bank 25 register 1</description>
      <displayName>F25R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F25R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x30C</addressOffset>
      <description>Filter bank 25 register 2</description>
      <displayName>F25R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F25R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x310</addressOffset>
      <description>Filter bank 26 register 1</description>
      <displayName>F26R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F26R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x314</addressOffset>
      <description>Filter bank 26 register 2</description>
      <displayName>F26R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F26R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x318</addressOffset>
      <description>Filter bank 27 register 1</description>
      <displayName>F27R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F27R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x31C</addressOffset>
      <description>Filter bank 27 register 2</description>
      <displayName>F27R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F27R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
