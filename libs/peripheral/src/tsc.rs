/// control register
pub mod cr {
    pub struct ReadonlyCache {
        /// Charge transfer pulse high
        pub ctph: u8,
        /// Charge transfer pulse low
        pub ctpl: u8,
        /// Spread spectrum deviation
        pub ssd: u8,
        /// Spread spectrum enable
        pub sse: u8,
        /// Spread spectrum prescaler
        pub sspsc: u8,
        /// pulse generator prescaler
        pub pgpsc: u8,
        /// Max count value
        pub mcv: u8,
        /// I/O Default mode
        pub iodef: u8,
        /// Synchronization pin polarity
        pub syncpol: u8,
        /// Acquisition mode
        pub am: u8,
        /// Start a new acquisition
        pub start: u8,
        /// Touch sensing controller enable
        pub tsce: u8,
    }
    pub struct Cache {
        /// Charge transfer pulse high
        pub ctph: u8,
        /// Charge transfer pulse low
        pub ctpl: u8,
        /// Spread spectrum deviation
        pub ssd: u8,
        /// Spread spectrum enable
        pub sse: u8,
        /// Spread spectrum prescaler
        pub sspsc: u8,
        /// pulse generator prescaler
        pub pgpsc: u8,
        /// Max count value
        pub mcv: u8,
        /// I/O Default mode
        pub iodef: u8,
        /// Synchronization pin polarity
        pub syncpol: u8,
        /// Acquisition mode
        pub am: u8,
        /// Start a new acquisition
        pub start: u8,
        /// Touch sensing controller enable
        pub tsce: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            ctph: ((value >> 28) & 0b1111) as u8,
            ctpl: ((value >> 24) & 0b1111) as u8,
            ssd: ((value >> 17) & 0b1111) as u8,
            sse: ((value >> 16) & 0b1111) as u8,
            sspsc: ((value >> 15) & 0b1111) as u8,
            pgpsc: ((value >> 12) & 0b1111) as u8,
            mcv: ((value >> 5) & 0b1111) as u8,
            iodef: ((value >> 4) & 0b1111) as u8,
            syncpol: ((value >> 3) & 0b1111) as u8,
            am: ((value >> 2) & 0b1111) as u8,
            start: ((value >> 1) & 0b1111) as u8,
            tsce: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x0u32) as *mut u32) };
        Cache {
            ctph: ((value >> 28) & 0b1111) as u8,
            ctpl: ((value >> 24) & 0b1111) as u8,
            ssd: ((value >> 17) & 0b1111) as u8,
            sse: ((value >> 16) & 0b1111) as u8,
            sspsc: ((value >> 15) & 0b1111) as u8,
            pgpsc: ((value >> 12) & 0b1111) as u8,
            mcv: ((value >> 5) & 0b1111) as u8,
            iodef: ((value >> 4) & 0b1111) as u8,
            syncpol: ((value >> 3) & 0b1111) as u8,
            am: ((value >> 2) & 0b1111) as u8,
            start: ((value >> 1) & 0b1111) as u8,
            tsce: ((value >> 0) & 0b1111) as u8,
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
                | ((self.ctph as u32) << 28)
                | ((self.ctpl as u32) << 24)
                | ((self.ssd as u32) << 17)
                | ((self.sse as u32) << 16)
                | ((self.sspsc as u32) << 15)
                | ((self.pgpsc as u32) << 12)
                | ((self.mcv as u32) << 5)
                | ((self.iodef as u32) << 4)
                | ((self.syncpol as u32) << 3)
                | ((self.am as u32) << 2)
                | ((self.start as u32) << 1)
                | ((self.tsce as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40024000u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// interrupt enable register
pub mod ier {
    pub struct ReadonlyCache {
        /// Max count error interrupt enable
        pub mceie: bool,
        /// End of acquisition interrupt enable
        pub eoaie: bool,
    }
    pub struct Cache {
        /// Max count error interrupt enable
        pub mceie: bool,
        /// End of acquisition interrupt enable
        pub eoaie: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            mceie: ((value >> 1) & 0b1) > 0,
            eoaie: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x4u32) as *mut u32) };
        Cache {
            mceie: ((value >> 1) & 0b1) > 0,
            eoaie: ((value >> 0) & 0b1) > 0,
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
                | ((self.mceie as u32) << 1)
                | ((self.eoaie as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40024000u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// interrupt clear register
pub mod icr {
    pub struct ReadonlyCache {
        /// Max count error interrupt clear
        pub mceic: bool,
        /// End of acquisition interrupt clear
        pub eoaic: bool,
    }
    pub struct Cache {
        /// Max count error interrupt clear
        pub mceic: bool,
        /// End of acquisition interrupt clear
        pub eoaic: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            mceic: ((value >> 1) & 0b1) > 0,
            eoaic: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x8u32) as *mut u32) };
        Cache {
            mceic: ((value >> 1) & 0b1) > 0,
            eoaic: ((value >> 0) & 0b1) > 0,
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
                | ((self.mceic as u32) << 1)
                | ((self.eoaic as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40024000u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// interrupt status register
pub mod isr {
    pub struct ReadonlyCache {
        /// Max count error flag
        pub mcef: bool,
        /// End of acquisition flag
        pub eoaf: bool,
    }
    pub struct Cache {
        /// Max count error flag
        pub mcef: bool,
        /// End of acquisition flag
        pub eoaf: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            mcef: ((value >> 1) & 0b1) > 0,
            eoaf: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0xCu32) as *mut u32) };
        Cache {
            mcef: ((value >> 1) & 0b1) > 0,
            eoaf: ((value >> 0) & 0b1) > 0,
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
                | ((self.mcef as u32) << 1)
                | ((self.eoaf as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40024000u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// I/O hysteresis control register
pub mod iohcr {
    pub struct ReadonlyCache {
        /// G1_IO1 Schmitt trigger hysteresis mode
        pub g1_io1: bool,
        /// G1_IO2 Schmitt trigger hysteresis mode
        pub g1_io2: bool,
        /// G1_IO3 Schmitt trigger hysteresis mode
        pub g1_io3: bool,
        /// G1_IO4 Schmitt trigger hysteresis mode
        pub g1_io4: bool,
        /// G2_IO1 Schmitt trigger hysteresis mode
        pub g2_io1: bool,
        /// G2_IO2 Schmitt trigger hysteresis mode
        pub g2_io2: bool,
        /// G2_IO3 Schmitt trigger hysteresis mode
        pub g2_io3: bool,
        /// G2_IO4 Schmitt trigger hysteresis mode
        pub g2_io4: bool,
        /// G3_IO1 Schmitt trigger hysteresis mode
        pub g3_io1: bool,
        /// G3_IO2 Schmitt trigger hysteresis mode
        pub g3_io2: bool,
        /// G3_IO3 Schmitt trigger hysteresis mode
        pub g3_io3: bool,
        /// G3_IO4 Schmitt trigger hysteresis mode
        pub g3_io4: bool,
        /// G4_IO1 Schmitt trigger hysteresis mode
        pub g4_io1: bool,
        /// G4_IO2 Schmitt trigger hysteresis mode
        pub g4_io2: bool,
        /// G4_IO3 Schmitt trigger hysteresis mode
        pub g4_io3: bool,
        /// G4_IO4 Schmitt trigger hysteresis mode
        pub g4_io4: bool,
        /// G5_IO1 Schmitt trigger hysteresis mode
        pub g5_io1: bool,
        /// G5_IO2 Schmitt trigger hysteresis mode
        pub g5_io2: bool,
        /// G5_IO3 Schmitt trigger hysteresis mode
        pub g5_io3: bool,
        /// G5_IO4 Schmitt trigger hysteresis mode
        pub g5_io4: bool,
        /// G6_IO1 Schmitt trigger hysteresis mode
        pub g6_io1: bool,
        /// G6_IO2 Schmitt trigger hysteresis mode
        pub g6_io2: bool,
        /// G6_IO3 Schmitt trigger hysteresis mode
        pub g6_io3: bool,
        /// G6_IO4 Schmitt trigger hysteresis mode
        pub g6_io4: bool,
        /// G7_IO1 Schmitt trigger hysteresis mode
        pub g7_io1: bool,
        /// G7_IO2 Schmitt trigger hysteresis mode
        pub g7_io2: bool,
        /// G7_IO3 Schmitt trigger hysteresis mode
        pub g7_io3: bool,
        /// G7_IO4 Schmitt trigger hysteresis mode
        pub g7_io4: bool,
        /// G8_IO1 Schmitt trigger hysteresis mode
        pub g8_io1: bool,
        /// G8_IO2 Schmitt trigger hysteresis mode
        pub g8_io2: bool,
        /// G8_IO3 Schmitt trigger hysteresis mode
        pub g8_io3: bool,
        /// G8_IO4 Schmitt trigger hysteresis mode
        pub g8_io4: bool,
    }
    pub struct Cache {
        /// G1_IO1 Schmitt trigger hysteresis mode
        pub g1_io1: bool,
        /// G1_IO2 Schmitt trigger hysteresis mode
        pub g1_io2: bool,
        /// G1_IO3 Schmitt trigger hysteresis mode
        pub g1_io3: bool,
        /// G1_IO4 Schmitt trigger hysteresis mode
        pub g1_io4: bool,
        /// G2_IO1 Schmitt trigger hysteresis mode
        pub g2_io1: bool,
        /// G2_IO2 Schmitt trigger hysteresis mode
        pub g2_io2: bool,
        /// G2_IO3 Schmitt trigger hysteresis mode
        pub g2_io3: bool,
        /// G2_IO4 Schmitt trigger hysteresis mode
        pub g2_io4: bool,
        /// G3_IO1 Schmitt trigger hysteresis mode
        pub g3_io1: bool,
        /// G3_IO2 Schmitt trigger hysteresis mode
        pub g3_io2: bool,
        /// G3_IO3 Schmitt trigger hysteresis mode
        pub g3_io3: bool,
        /// G3_IO4 Schmitt trigger hysteresis mode
        pub g3_io4: bool,
        /// G4_IO1 Schmitt trigger hysteresis mode
        pub g4_io1: bool,
        /// G4_IO2 Schmitt trigger hysteresis mode
        pub g4_io2: bool,
        /// G4_IO3 Schmitt trigger hysteresis mode
        pub g4_io3: bool,
        /// G4_IO4 Schmitt trigger hysteresis mode
        pub g4_io4: bool,
        /// G5_IO1 Schmitt trigger hysteresis mode
        pub g5_io1: bool,
        /// G5_IO2 Schmitt trigger hysteresis mode
        pub g5_io2: bool,
        /// G5_IO3 Schmitt trigger hysteresis mode
        pub g5_io3: bool,
        /// G5_IO4 Schmitt trigger hysteresis mode
        pub g5_io4: bool,
        /// G6_IO1 Schmitt trigger hysteresis mode
        pub g6_io1: bool,
        /// G6_IO2 Schmitt trigger hysteresis mode
        pub g6_io2: bool,
        /// G6_IO3 Schmitt trigger hysteresis mode
        pub g6_io3: bool,
        /// G6_IO4 Schmitt trigger hysteresis mode
        pub g6_io4: bool,
        /// G7_IO1 Schmitt trigger hysteresis mode
        pub g7_io1: bool,
        /// G7_IO2 Schmitt trigger hysteresis mode
        pub g7_io2: bool,
        /// G7_IO3 Schmitt trigger hysteresis mode
        pub g7_io3: bool,
        /// G7_IO4 Schmitt trigger hysteresis mode
        pub g7_io4: bool,
        /// G8_IO1 Schmitt trigger hysteresis mode
        pub g8_io1: bool,
        /// G8_IO2 Schmitt trigger hysteresis mode
        pub g8_io2: bool,
        /// G8_IO3 Schmitt trigger hysteresis mode
        pub g8_io3: bool,
        /// G8_IO4 Schmitt trigger hysteresis mode
        pub g8_io4: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            g1_io1: ((value >> 0) & 0b1) > 0,
            g1_io2: ((value >> 1) & 0b1) > 0,
            g1_io3: ((value >> 2) & 0b1) > 0,
            g1_io4: ((value >> 3) & 0b1) > 0,
            g2_io1: ((value >> 4) & 0b1) > 0,
            g2_io2: ((value >> 5) & 0b1) > 0,
            g2_io3: ((value >> 6) & 0b1) > 0,
            g2_io4: ((value >> 7) & 0b1) > 0,
            g3_io1: ((value >> 8) & 0b1) > 0,
            g3_io2: ((value >> 9) & 0b1) > 0,
            g3_io3: ((value >> 10) & 0b1) > 0,
            g3_io4: ((value >> 11) & 0b1) > 0,
            g4_io1: ((value >> 12) & 0b1) > 0,
            g4_io2: ((value >> 13) & 0b1) > 0,
            g4_io3: ((value >> 14) & 0b1) > 0,
            g4_io4: ((value >> 15) & 0b1) > 0,
            g5_io1: ((value >> 16) & 0b1) > 0,
            g5_io2: ((value >> 17) & 0b1) > 0,
            g5_io3: ((value >> 18) & 0b1) > 0,
            g5_io4: ((value >> 19) & 0b1) > 0,
            g6_io1: ((value >> 20) & 0b1) > 0,
            g6_io2: ((value >> 21) & 0b1) > 0,
            g6_io3: ((value >> 22) & 0b1) > 0,
            g6_io4: ((value >> 23) & 0b1) > 0,
            g7_io1: ((value >> 24) & 0b1) > 0,
            g7_io2: ((value >> 25) & 0b1) > 0,
            g7_io3: ((value >> 26) & 0b1) > 0,
            g7_io4: ((value >> 27) & 0b1) > 0,
            g8_io1: ((value >> 28) & 0b1) > 0,
            g8_io2: ((value >> 29) & 0b1) > 0,
            g8_io3: ((value >> 30) & 0b1) > 0,
            g8_io4: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x10u32) as *mut u32) };
        Cache {
            g1_io1: ((value >> 0) & 0b1) > 0,
            g1_io2: ((value >> 1) & 0b1) > 0,
            g1_io3: ((value >> 2) & 0b1) > 0,
            g1_io4: ((value >> 3) & 0b1) > 0,
            g2_io1: ((value >> 4) & 0b1) > 0,
            g2_io2: ((value >> 5) & 0b1) > 0,
            g2_io3: ((value >> 6) & 0b1) > 0,
            g2_io4: ((value >> 7) & 0b1) > 0,
            g3_io1: ((value >> 8) & 0b1) > 0,
            g3_io2: ((value >> 9) & 0b1) > 0,
            g3_io3: ((value >> 10) & 0b1) > 0,
            g3_io4: ((value >> 11) & 0b1) > 0,
            g4_io1: ((value >> 12) & 0b1) > 0,
            g4_io2: ((value >> 13) & 0b1) > 0,
            g4_io3: ((value >> 14) & 0b1) > 0,
            g4_io4: ((value >> 15) & 0b1) > 0,
            g5_io1: ((value >> 16) & 0b1) > 0,
            g5_io2: ((value >> 17) & 0b1) > 0,
            g5_io3: ((value >> 18) & 0b1) > 0,
            g5_io4: ((value >> 19) & 0b1) > 0,
            g6_io1: ((value >> 20) & 0b1) > 0,
            g6_io2: ((value >> 21) & 0b1) > 0,
            g6_io3: ((value >> 22) & 0b1) > 0,
            g6_io4: ((value >> 23) & 0b1) > 0,
            g7_io1: ((value >> 24) & 0b1) > 0,
            g7_io2: ((value >> 25) & 0b1) > 0,
            g7_io3: ((value >> 26) & 0b1) > 0,
            g7_io4: ((value >> 27) & 0b1) > 0,
            g8_io1: ((value >> 28) & 0b1) > 0,
            g8_io2: ((value >> 29) & 0b1) > 0,
            g8_io3: ((value >> 30) & 0b1) > 0,
            g8_io4: ((value >> 31) & 0b1) > 0,
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
                | ((self.g1_io1 as u32) << 0)
                | ((self.g1_io2 as u32) << 1)
                | ((self.g1_io3 as u32) << 2)
                | ((self.g1_io4 as u32) << 3)
                | ((self.g2_io1 as u32) << 4)
                | ((self.g2_io2 as u32) << 5)
                | ((self.g2_io3 as u32) << 6)
                | ((self.g2_io4 as u32) << 7)
                | ((self.g3_io1 as u32) << 8)
                | ((self.g3_io2 as u32) << 9)
                | ((self.g3_io3 as u32) << 10)
                | ((self.g3_io4 as u32) << 11)
                | ((self.g4_io1 as u32) << 12)
                | ((self.g4_io2 as u32) << 13)
                | ((self.g4_io3 as u32) << 14)
                | ((self.g4_io4 as u32) << 15)
                | ((self.g5_io1 as u32) << 16)
                | ((self.g5_io2 as u32) << 17)
                | ((self.g5_io3 as u32) << 18)
                | ((self.g5_io4 as u32) << 19)
                | ((self.g6_io1 as u32) << 20)
                | ((self.g6_io2 as u32) << 21)
                | ((self.g6_io3 as u32) << 22)
                | ((self.g6_io4 as u32) << 23)
                | ((self.g7_io1 as u32) << 24)
                | ((self.g7_io2 as u32) << 25)
                | ((self.g7_io3 as u32) << 26)
                | ((self.g7_io4 as u32) << 27)
                | ((self.g8_io1 as u32) << 28)
                | ((self.g8_io2 as u32) << 29)
                | ((self.g8_io3 as u32) << 30)
                | ((self.g8_io4 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40024000u32 + 0x10u32) as *mut u32, value) };
        }
    }
}
/// I/O analog switch control register
pub mod ioascr {
    pub struct ReadonlyCache {
        /// G1_IO1 analog switch enable
        pub g1_io1: bool,
        /// G1_IO2 analog switch enable
        pub g1_io2: bool,
        /// G1_IO3 analog switch enable
        pub g1_io3: bool,
        /// G1_IO4 analog switch enable
        pub g1_io4: bool,
        /// G2_IO1 analog switch enable
        pub g2_io1: bool,
        /// G2_IO2 analog switch enable
        pub g2_io2: bool,
        /// G2_IO3 analog switch enable
        pub g2_io3: bool,
        /// G2_IO4 analog switch enable
        pub g2_io4: bool,
        /// G3_IO1 analog switch enable
        pub g3_io1: bool,
        /// G3_IO2 analog switch enable
        pub g3_io2: bool,
        /// G3_IO3 analog switch enable
        pub g3_io3: bool,
        /// G3_IO4 analog switch enable
        pub g3_io4: bool,
        /// G4_IO1 analog switch enable
        pub g4_io1: bool,
        /// G4_IO2 analog switch enable
        pub g4_io2: bool,
        /// G4_IO3 analog switch enable
        pub g4_io3: bool,
        /// G4_IO4 analog switch enable
        pub g4_io4: bool,
        /// G5_IO1 analog switch enable
        pub g5_io1: bool,
        /// G5_IO2 analog switch enable
        pub g5_io2: bool,
        /// G5_IO3 analog switch enable
        pub g5_io3: bool,
        /// G5_IO4 analog switch enable
        pub g5_io4: bool,
        /// G6_IO1 analog switch enable
        pub g6_io1: bool,
        /// G6_IO2 analog switch enable
        pub g6_io2: bool,
        /// G6_IO3 analog switch enable
        pub g6_io3: bool,
        /// G6_IO4 analog switch enable
        pub g6_io4: bool,
        /// G7_IO1 analog switch enable
        pub g7_io1: bool,
        /// G7_IO2 analog switch enable
        pub g7_io2: bool,
        /// G7_IO3 analog switch enable
        pub g7_io3: bool,
        /// G7_IO4 analog switch enable
        pub g7_io4: bool,
        /// G8_IO1 analog switch enable
        pub g8_io1: bool,
        /// G8_IO2 analog switch enable
        pub g8_io2: bool,
        /// G8_IO3 analog switch enable
        pub g8_io3: bool,
        /// G8_IO4 analog switch enable
        pub g8_io4: bool,
    }
    pub struct Cache {
        /// G1_IO1 analog switch enable
        pub g1_io1: bool,
        /// G1_IO2 analog switch enable
        pub g1_io2: bool,
        /// G1_IO3 analog switch enable
        pub g1_io3: bool,
        /// G1_IO4 analog switch enable
        pub g1_io4: bool,
        /// G2_IO1 analog switch enable
        pub g2_io1: bool,
        /// G2_IO2 analog switch enable
        pub g2_io2: bool,
        /// G2_IO3 analog switch enable
        pub g2_io3: bool,
        /// G2_IO4 analog switch enable
        pub g2_io4: bool,
        /// G3_IO1 analog switch enable
        pub g3_io1: bool,
        /// G3_IO2 analog switch enable
        pub g3_io2: bool,
        /// G3_IO3 analog switch enable
        pub g3_io3: bool,
        /// G3_IO4 analog switch enable
        pub g3_io4: bool,
        /// G4_IO1 analog switch enable
        pub g4_io1: bool,
        /// G4_IO2 analog switch enable
        pub g4_io2: bool,
        /// G4_IO3 analog switch enable
        pub g4_io3: bool,
        /// G4_IO4 analog switch enable
        pub g4_io4: bool,
        /// G5_IO1 analog switch enable
        pub g5_io1: bool,
        /// G5_IO2 analog switch enable
        pub g5_io2: bool,
        /// G5_IO3 analog switch enable
        pub g5_io3: bool,
        /// G5_IO4 analog switch enable
        pub g5_io4: bool,
        /// G6_IO1 analog switch enable
        pub g6_io1: bool,
        /// G6_IO2 analog switch enable
        pub g6_io2: bool,
        /// G6_IO3 analog switch enable
        pub g6_io3: bool,
        /// G6_IO4 analog switch enable
        pub g6_io4: bool,
        /// G7_IO1 analog switch enable
        pub g7_io1: bool,
        /// G7_IO2 analog switch enable
        pub g7_io2: bool,
        /// G7_IO3 analog switch enable
        pub g7_io3: bool,
        /// G7_IO4 analog switch enable
        pub g7_io4: bool,
        /// G8_IO1 analog switch enable
        pub g8_io1: bool,
        /// G8_IO2 analog switch enable
        pub g8_io2: bool,
        /// G8_IO3 analog switch enable
        pub g8_io3: bool,
        /// G8_IO4 analog switch enable
        pub g8_io4: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x18u32) as *mut u32) };
        ReadonlyCache {
            g1_io1: ((value >> 0) & 0b1) > 0,
            g1_io2: ((value >> 1) & 0b1) > 0,
            g1_io3: ((value >> 2) & 0b1) > 0,
            g1_io4: ((value >> 3) & 0b1) > 0,
            g2_io1: ((value >> 4) & 0b1) > 0,
            g2_io2: ((value >> 5) & 0b1) > 0,
            g2_io3: ((value >> 6) & 0b1) > 0,
            g2_io4: ((value >> 7) & 0b1) > 0,
            g3_io1: ((value >> 8) & 0b1) > 0,
            g3_io2: ((value >> 9) & 0b1) > 0,
            g3_io3: ((value >> 10) & 0b1) > 0,
            g3_io4: ((value >> 11) & 0b1) > 0,
            g4_io1: ((value >> 12) & 0b1) > 0,
            g4_io2: ((value >> 13) & 0b1) > 0,
            g4_io3: ((value >> 14) & 0b1) > 0,
            g4_io4: ((value >> 15) & 0b1) > 0,
            g5_io1: ((value >> 16) & 0b1) > 0,
            g5_io2: ((value >> 17) & 0b1) > 0,
            g5_io3: ((value >> 18) & 0b1) > 0,
            g5_io4: ((value >> 19) & 0b1) > 0,
            g6_io1: ((value >> 20) & 0b1) > 0,
            g6_io2: ((value >> 21) & 0b1) > 0,
            g6_io3: ((value >> 22) & 0b1) > 0,
            g6_io4: ((value >> 23) & 0b1) > 0,
            g7_io1: ((value >> 24) & 0b1) > 0,
            g7_io2: ((value >> 25) & 0b1) > 0,
            g7_io3: ((value >> 26) & 0b1) > 0,
            g7_io4: ((value >> 27) & 0b1) > 0,
            g8_io1: ((value >> 28) & 0b1) > 0,
            g8_io2: ((value >> 29) & 0b1) > 0,
            g8_io3: ((value >> 30) & 0b1) > 0,
            g8_io4: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x18u32) as *mut u32) };
        Cache {
            g1_io1: ((value >> 0) & 0b1) > 0,
            g1_io2: ((value >> 1) & 0b1) > 0,
            g1_io3: ((value >> 2) & 0b1) > 0,
            g1_io4: ((value >> 3) & 0b1) > 0,
            g2_io1: ((value >> 4) & 0b1) > 0,
            g2_io2: ((value >> 5) & 0b1) > 0,
            g2_io3: ((value >> 6) & 0b1) > 0,
            g2_io4: ((value >> 7) & 0b1) > 0,
            g3_io1: ((value >> 8) & 0b1) > 0,
            g3_io2: ((value >> 9) & 0b1) > 0,
            g3_io3: ((value >> 10) & 0b1) > 0,
            g3_io4: ((value >> 11) & 0b1) > 0,
            g4_io1: ((value >> 12) & 0b1) > 0,
            g4_io2: ((value >> 13) & 0b1) > 0,
            g4_io3: ((value >> 14) & 0b1) > 0,
            g4_io4: ((value >> 15) & 0b1) > 0,
            g5_io1: ((value >> 16) & 0b1) > 0,
            g5_io2: ((value >> 17) & 0b1) > 0,
            g5_io3: ((value >> 18) & 0b1) > 0,
            g5_io4: ((value >> 19) & 0b1) > 0,
            g6_io1: ((value >> 20) & 0b1) > 0,
            g6_io2: ((value >> 21) & 0b1) > 0,
            g6_io3: ((value >> 22) & 0b1) > 0,
            g6_io4: ((value >> 23) & 0b1) > 0,
            g7_io1: ((value >> 24) & 0b1) > 0,
            g7_io2: ((value >> 25) & 0b1) > 0,
            g7_io3: ((value >> 26) & 0b1) > 0,
            g7_io4: ((value >> 27) & 0b1) > 0,
            g8_io1: ((value >> 28) & 0b1) > 0,
            g8_io2: ((value >> 29) & 0b1) > 0,
            g8_io3: ((value >> 30) & 0b1) > 0,
            g8_io4: ((value >> 31) & 0b1) > 0,
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
                | ((self.g1_io1 as u32) << 0)
                | ((self.g1_io2 as u32) << 1)
                | ((self.g1_io3 as u32) << 2)
                | ((self.g1_io4 as u32) << 3)
                | ((self.g2_io1 as u32) << 4)
                | ((self.g2_io2 as u32) << 5)
                | ((self.g2_io3 as u32) << 6)
                | ((self.g2_io4 as u32) << 7)
                | ((self.g3_io1 as u32) << 8)
                | ((self.g3_io2 as u32) << 9)
                | ((self.g3_io3 as u32) << 10)
                | ((self.g3_io4 as u32) << 11)
                | ((self.g4_io1 as u32) << 12)
                | ((self.g4_io2 as u32) << 13)
                | ((self.g4_io3 as u32) << 14)
                | ((self.g4_io4 as u32) << 15)
                | ((self.g5_io1 as u32) << 16)
                | ((self.g5_io2 as u32) << 17)
                | ((self.g5_io3 as u32) << 18)
                | ((self.g5_io4 as u32) << 19)
                | ((self.g6_io1 as u32) << 20)
                | ((self.g6_io2 as u32) << 21)
                | ((self.g6_io3 as u32) << 22)
                | ((self.g6_io4 as u32) << 23)
                | ((self.g7_io1 as u32) << 24)
                | ((self.g7_io2 as u32) << 25)
                | ((self.g7_io3 as u32) << 26)
                | ((self.g7_io4 as u32) << 27)
                | ((self.g8_io1 as u32) << 28)
                | ((self.g8_io2 as u32) << 29)
                | ((self.g8_io3 as u32) << 30)
                | ((self.g8_io4 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40024000u32 + 0x18u32) as *mut u32, value) };
        }
    }
}
/// I/O sampling control register
pub mod ioscr {
    pub struct ReadonlyCache {
        /// G1_IO1 sampling mode
        pub g1_io1: bool,
        /// G1_IO2 sampling mode
        pub g1_io2: bool,
        /// G1_IO3 sampling mode
        pub g1_io3: bool,
        /// G1_IO4 sampling mode
        pub g1_io4: bool,
        /// G2_IO1 sampling mode
        pub g2_io1: bool,
        /// G2_IO2 sampling mode
        pub g2_io2: bool,
        /// G2_IO3 sampling mode
        pub g2_io3: bool,
        /// G2_IO4 sampling mode
        pub g2_io4: bool,
        /// G3_IO1 sampling mode
        pub g3_io1: bool,
        /// G3_IO2 sampling mode
        pub g3_io2: bool,
        /// G3_IO3 sampling mode
        pub g3_io3: bool,
        /// G3_IO4 sampling mode
        pub g3_io4: bool,
        /// G4_IO1 sampling mode
        pub g4_io1: bool,
        /// G4_IO2 sampling mode
        pub g4_io2: bool,
        /// G4_IO3 sampling mode
        pub g4_io3: bool,
        /// G4_IO4 sampling mode
        pub g4_io4: bool,
        /// G5_IO1 sampling mode
        pub g5_io1: bool,
        /// G5_IO2 sampling mode
        pub g5_io2: bool,
        /// G5_IO3 sampling mode
        pub g5_io3: bool,
        /// G5_IO4 sampling mode
        pub g5_io4: bool,
        /// G6_IO1 sampling mode
        pub g6_io1: bool,
        /// G6_IO2 sampling mode
        pub g6_io2: bool,
        /// G6_IO3 sampling mode
        pub g6_io3: bool,
        /// G6_IO4 sampling mode
        pub g6_io4: bool,
        /// G7_IO1 sampling mode
        pub g7_io1: bool,
        /// G7_IO2 sampling mode
        pub g7_io2: bool,
        /// G7_IO3 sampling mode
        pub g7_io3: bool,
        /// G7_IO4 sampling mode
        pub g7_io4: bool,
        /// G8_IO1 sampling mode
        pub g8_io1: bool,
        /// G8_IO2 sampling mode
        pub g8_io2: bool,
        /// G8_IO3 sampling mode
        pub g8_io3: bool,
        /// G8_IO4 sampling mode
        pub g8_io4: bool,
    }
    pub struct Cache {
        /// G1_IO1 sampling mode
        pub g1_io1: bool,
        /// G1_IO2 sampling mode
        pub g1_io2: bool,
        /// G1_IO3 sampling mode
        pub g1_io3: bool,
        /// G1_IO4 sampling mode
        pub g1_io4: bool,
        /// G2_IO1 sampling mode
        pub g2_io1: bool,
        /// G2_IO2 sampling mode
        pub g2_io2: bool,
        /// G2_IO3 sampling mode
        pub g2_io3: bool,
        /// G2_IO4 sampling mode
        pub g2_io4: bool,
        /// G3_IO1 sampling mode
        pub g3_io1: bool,
        /// G3_IO2 sampling mode
        pub g3_io2: bool,
        /// G3_IO3 sampling mode
        pub g3_io3: bool,
        /// G3_IO4 sampling mode
        pub g3_io4: bool,
        /// G4_IO1 sampling mode
        pub g4_io1: bool,
        /// G4_IO2 sampling mode
        pub g4_io2: bool,
        /// G4_IO3 sampling mode
        pub g4_io3: bool,
        /// G4_IO4 sampling mode
        pub g4_io4: bool,
        /// G5_IO1 sampling mode
        pub g5_io1: bool,
        /// G5_IO2 sampling mode
        pub g5_io2: bool,
        /// G5_IO3 sampling mode
        pub g5_io3: bool,
        /// G5_IO4 sampling mode
        pub g5_io4: bool,
        /// G6_IO1 sampling mode
        pub g6_io1: bool,
        /// G6_IO2 sampling mode
        pub g6_io2: bool,
        /// G6_IO3 sampling mode
        pub g6_io3: bool,
        /// G6_IO4 sampling mode
        pub g6_io4: bool,
        /// G7_IO1 sampling mode
        pub g7_io1: bool,
        /// G7_IO2 sampling mode
        pub g7_io2: bool,
        /// G7_IO3 sampling mode
        pub g7_io3: bool,
        /// G7_IO4 sampling mode
        pub g7_io4: bool,
        /// G8_IO1 sampling mode
        pub g8_io1: bool,
        /// G8_IO2 sampling mode
        pub g8_io2: bool,
        /// G8_IO3 sampling mode
        pub g8_io3: bool,
        /// G8_IO4 sampling mode
        pub g8_io4: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x20u32) as *mut u32) };
        ReadonlyCache {
            g1_io1: ((value >> 0) & 0b1) > 0,
            g1_io2: ((value >> 1) & 0b1) > 0,
            g1_io3: ((value >> 2) & 0b1) > 0,
            g1_io4: ((value >> 3) & 0b1) > 0,
            g2_io1: ((value >> 4) & 0b1) > 0,
            g2_io2: ((value >> 5) & 0b1) > 0,
            g2_io3: ((value >> 6) & 0b1) > 0,
            g2_io4: ((value >> 7) & 0b1) > 0,
            g3_io1: ((value >> 8) & 0b1) > 0,
            g3_io2: ((value >> 9) & 0b1) > 0,
            g3_io3: ((value >> 10) & 0b1) > 0,
            g3_io4: ((value >> 11) & 0b1) > 0,
            g4_io1: ((value >> 12) & 0b1) > 0,
            g4_io2: ((value >> 13) & 0b1) > 0,
            g4_io3: ((value >> 14) & 0b1) > 0,
            g4_io4: ((value >> 15) & 0b1) > 0,
            g5_io1: ((value >> 16) & 0b1) > 0,
            g5_io2: ((value >> 17) & 0b1) > 0,
            g5_io3: ((value >> 18) & 0b1) > 0,
            g5_io4: ((value >> 19) & 0b1) > 0,
            g6_io1: ((value >> 20) & 0b1) > 0,
            g6_io2: ((value >> 21) & 0b1) > 0,
            g6_io3: ((value >> 22) & 0b1) > 0,
            g6_io4: ((value >> 23) & 0b1) > 0,
            g7_io1: ((value >> 24) & 0b1) > 0,
            g7_io2: ((value >> 25) & 0b1) > 0,
            g7_io3: ((value >> 26) & 0b1) > 0,
            g7_io4: ((value >> 27) & 0b1) > 0,
            g8_io1: ((value >> 28) & 0b1) > 0,
            g8_io2: ((value >> 29) & 0b1) > 0,
            g8_io3: ((value >> 30) & 0b1) > 0,
            g8_io4: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x20u32) as *mut u32) };
        Cache {
            g1_io1: ((value >> 0) & 0b1) > 0,
            g1_io2: ((value >> 1) & 0b1) > 0,
            g1_io3: ((value >> 2) & 0b1) > 0,
            g1_io4: ((value >> 3) & 0b1) > 0,
            g2_io1: ((value >> 4) & 0b1) > 0,
            g2_io2: ((value >> 5) & 0b1) > 0,
            g2_io3: ((value >> 6) & 0b1) > 0,
            g2_io4: ((value >> 7) & 0b1) > 0,
            g3_io1: ((value >> 8) & 0b1) > 0,
            g3_io2: ((value >> 9) & 0b1) > 0,
            g3_io3: ((value >> 10) & 0b1) > 0,
            g3_io4: ((value >> 11) & 0b1) > 0,
            g4_io1: ((value >> 12) & 0b1) > 0,
            g4_io2: ((value >> 13) & 0b1) > 0,
            g4_io3: ((value >> 14) & 0b1) > 0,
            g4_io4: ((value >> 15) & 0b1) > 0,
            g5_io1: ((value >> 16) & 0b1) > 0,
            g5_io2: ((value >> 17) & 0b1) > 0,
            g5_io3: ((value >> 18) & 0b1) > 0,
            g5_io4: ((value >> 19) & 0b1) > 0,
            g6_io1: ((value >> 20) & 0b1) > 0,
            g6_io2: ((value >> 21) & 0b1) > 0,
            g6_io3: ((value >> 22) & 0b1) > 0,
            g6_io4: ((value >> 23) & 0b1) > 0,
            g7_io1: ((value >> 24) & 0b1) > 0,
            g7_io2: ((value >> 25) & 0b1) > 0,
            g7_io3: ((value >> 26) & 0b1) > 0,
            g7_io4: ((value >> 27) & 0b1) > 0,
            g8_io1: ((value >> 28) & 0b1) > 0,
            g8_io2: ((value >> 29) & 0b1) > 0,
            g8_io3: ((value >> 30) & 0b1) > 0,
            g8_io4: ((value >> 31) & 0b1) > 0,
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
                | ((self.g1_io1 as u32) << 0)
                | ((self.g1_io2 as u32) << 1)
                | ((self.g1_io3 as u32) << 2)
                | ((self.g1_io4 as u32) << 3)
                | ((self.g2_io1 as u32) << 4)
                | ((self.g2_io2 as u32) << 5)
                | ((self.g2_io3 as u32) << 6)
                | ((self.g2_io4 as u32) << 7)
                | ((self.g3_io1 as u32) << 8)
                | ((self.g3_io2 as u32) << 9)
                | ((self.g3_io3 as u32) << 10)
                | ((self.g3_io4 as u32) << 11)
                | ((self.g4_io1 as u32) << 12)
                | ((self.g4_io2 as u32) << 13)
                | ((self.g4_io3 as u32) << 14)
                | ((self.g4_io4 as u32) << 15)
                | ((self.g5_io1 as u32) << 16)
                | ((self.g5_io2 as u32) << 17)
                | ((self.g5_io3 as u32) << 18)
                | ((self.g5_io4 as u32) << 19)
                | ((self.g6_io1 as u32) << 20)
                | ((self.g6_io2 as u32) << 21)
                | ((self.g6_io3 as u32) << 22)
                | ((self.g6_io4 as u32) << 23)
                | ((self.g7_io1 as u32) << 24)
                | ((self.g7_io2 as u32) << 25)
                | ((self.g7_io3 as u32) << 26)
                | ((self.g7_io4 as u32) << 27)
                | ((self.g8_io1 as u32) << 28)
                | ((self.g8_io2 as u32) << 29)
                | ((self.g8_io3 as u32) << 30)
                | ((self.g8_io4 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40024000u32 + 0x20u32) as *mut u32, value) };
        }
    }
}
/// I/O channel control register
pub mod ioccr {
    pub struct ReadonlyCache {
        /// G1_IO1 channel mode
        pub g1_io1: bool,
        /// G1_IO2 channel mode
        pub g1_io2: bool,
        /// G1_IO3 channel mode
        pub g1_io3: bool,
        /// G1_IO4 channel mode
        pub g1_io4: bool,
        /// G2_IO1 channel mode
        pub g2_io1: bool,
        /// G2_IO2 channel mode
        pub g2_io2: bool,
        /// G2_IO3 channel mode
        pub g2_io3: bool,
        /// G2_IO4 channel mode
        pub g2_io4: bool,
        /// G3_IO1 channel mode
        pub g3_io1: bool,
        /// G3_IO2 channel mode
        pub g3_io2: bool,
        /// G3_IO3 channel mode
        pub g3_io3: bool,
        /// G3_IO4 channel mode
        pub g3_io4: bool,
        /// G4_IO1 channel mode
        pub g4_io1: bool,
        /// G4_IO2 channel mode
        pub g4_io2: bool,
        /// G4_IO3 channel mode
        pub g4_io3: bool,
        /// G4_IO4 channel mode
        pub g4_io4: bool,
        /// G5_IO1 channel mode
        pub g5_io1: bool,
        /// G5_IO2 channel mode
        pub g5_io2: bool,
        /// G5_IO3 channel mode
        pub g5_io3: bool,
        /// G5_IO4 channel mode
        pub g5_io4: bool,
        /// G6_IO1 channel mode
        pub g6_io1: bool,
        /// G6_IO2 channel mode
        pub g6_io2: bool,
        /// G6_IO3 channel mode
        pub g6_io3: bool,
        /// G6_IO4 channel mode
        pub g6_io4: bool,
        /// G7_IO1 channel mode
        pub g7_io1: bool,
        /// G7_IO2 channel mode
        pub g7_io2: bool,
        /// G7_IO3 channel mode
        pub g7_io3: bool,
        /// G7_IO4 channel mode
        pub g7_io4: bool,
        /// G8_IO1 channel mode
        pub g8_io1: bool,
        /// G8_IO2 channel mode
        pub g8_io2: bool,
        /// G8_IO3 channel mode
        pub g8_io3: bool,
        /// G8_IO4 channel mode
        pub g8_io4: bool,
    }
    pub struct Cache {
        /// G1_IO1 channel mode
        pub g1_io1: bool,
        /// G1_IO2 channel mode
        pub g1_io2: bool,
        /// G1_IO3 channel mode
        pub g1_io3: bool,
        /// G1_IO4 channel mode
        pub g1_io4: bool,
        /// G2_IO1 channel mode
        pub g2_io1: bool,
        /// G2_IO2 channel mode
        pub g2_io2: bool,
        /// G2_IO3 channel mode
        pub g2_io3: bool,
        /// G2_IO4 channel mode
        pub g2_io4: bool,
        /// G3_IO1 channel mode
        pub g3_io1: bool,
        /// G3_IO2 channel mode
        pub g3_io2: bool,
        /// G3_IO3 channel mode
        pub g3_io3: bool,
        /// G3_IO4 channel mode
        pub g3_io4: bool,
        /// G4_IO1 channel mode
        pub g4_io1: bool,
        /// G4_IO2 channel mode
        pub g4_io2: bool,
        /// G4_IO3 channel mode
        pub g4_io3: bool,
        /// G4_IO4 channel mode
        pub g4_io4: bool,
        /// G5_IO1 channel mode
        pub g5_io1: bool,
        /// G5_IO2 channel mode
        pub g5_io2: bool,
        /// G5_IO3 channel mode
        pub g5_io3: bool,
        /// G5_IO4 channel mode
        pub g5_io4: bool,
        /// G6_IO1 channel mode
        pub g6_io1: bool,
        /// G6_IO2 channel mode
        pub g6_io2: bool,
        /// G6_IO3 channel mode
        pub g6_io3: bool,
        /// G6_IO4 channel mode
        pub g6_io4: bool,
        /// G7_IO1 channel mode
        pub g7_io1: bool,
        /// G7_IO2 channel mode
        pub g7_io2: bool,
        /// G7_IO3 channel mode
        pub g7_io3: bool,
        /// G7_IO4 channel mode
        pub g7_io4: bool,
        /// G8_IO1 channel mode
        pub g8_io1: bool,
        /// G8_IO2 channel mode
        pub g8_io2: bool,
        /// G8_IO3 channel mode
        pub g8_io3: bool,
        /// G8_IO4 channel mode
        pub g8_io4: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x28u32) as *mut u32) };
        ReadonlyCache {
            g1_io1: ((value >> 0) & 0b1) > 0,
            g1_io2: ((value >> 1) & 0b1) > 0,
            g1_io3: ((value >> 2) & 0b1) > 0,
            g1_io4: ((value >> 3) & 0b1) > 0,
            g2_io1: ((value >> 4) & 0b1) > 0,
            g2_io2: ((value >> 5) & 0b1) > 0,
            g2_io3: ((value >> 6) & 0b1) > 0,
            g2_io4: ((value >> 7) & 0b1) > 0,
            g3_io1: ((value >> 8) & 0b1) > 0,
            g3_io2: ((value >> 9) & 0b1) > 0,
            g3_io3: ((value >> 10) & 0b1) > 0,
            g3_io4: ((value >> 11) & 0b1) > 0,
            g4_io1: ((value >> 12) & 0b1) > 0,
            g4_io2: ((value >> 13) & 0b1) > 0,
            g4_io3: ((value >> 14) & 0b1) > 0,
            g4_io4: ((value >> 15) & 0b1) > 0,
            g5_io1: ((value >> 16) & 0b1) > 0,
            g5_io2: ((value >> 17) & 0b1) > 0,
            g5_io3: ((value >> 18) & 0b1) > 0,
            g5_io4: ((value >> 19) & 0b1) > 0,
            g6_io1: ((value >> 20) & 0b1) > 0,
            g6_io2: ((value >> 21) & 0b1) > 0,
            g6_io3: ((value >> 22) & 0b1) > 0,
            g6_io4: ((value >> 23) & 0b1) > 0,
            g7_io1: ((value >> 24) & 0b1) > 0,
            g7_io2: ((value >> 25) & 0b1) > 0,
            g7_io3: ((value >> 26) & 0b1) > 0,
            g7_io4: ((value >> 27) & 0b1) > 0,
            g8_io1: ((value >> 28) & 0b1) > 0,
            g8_io2: ((value >> 29) & 0b1) > 0,
            g8_io3: ((value >> 30) & 0b1) > 0,
            g8_io4: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x28u32) as *mut u32) };
        Cache {
            g1_io1: ((value >> 0) & 0b1) > 0,
            g1_io2: ((value >> 1) & 0b1) > 0,
            g1_io3: ((value >> 2) & 0b1) > 0,
            g1_io4: ((value >> 3) & 0b1) > 0,
            g2_io1: ((value >> 4) & 0b1) > 0,
            g2_io2: ((value >> 5) & 0b1) > 0,
            g2_io3: ((value >> 6) & 0b1) > 0,
            g2_io4: ((value >> 7) & 0b1) > 0,
            g3_io1: ((value >> 8) & 0b1) > 0,
            g3_io2: ((value >> 9) & 0b1) > 0,
            g3_io3: ((value >> 10) & 0b1) > 0,
            g3_io4: ((value >> 11) & 0b1) > 0,
            g4_io1: ((value >> 12) & 0b1) > 0,
            g4_io2: ((value >> 13) & 0b1) > 0,
            g4_io3: ((value >> 14) & 0b1) > 0,
            g4_io4: ((value >> 15) & 0b1) > 0,
            g5_io1: ((value >> 16) & 0b1) > 0,
            g5_io2: ((value >> 17) & 0b1) > 0,
            g5_io3: ((value >> 18) & 0b1) > 0,
            g5_io4: ((value >> 19) & 0b1) > 0,
            g6_io1: ((value >> 20) & 0b1) > 0,
            g6_io2: ((value >> 21) & 0b1) > 0,
            g6_io3: ((value >> 22) & 0b1) > 0,
            g6_io4: ((value >> 23) & 0b1) > 0,
            g7_io1: ((value >> 24) & 0b1) > 0,
            g7_io2: ((value >> 25) & 0b1) > 0,
            g7_io3: ((value >> 26) & 0b1) > 0,
            g7_io4: ((value >> 27) & 0b1) > 0,
            g8_io1: ((value >> 28) & 0b1) > 0,
            g8_io2: ((value >> 29) & 0b1) > 0,
            g8_io3: ((value >> 30) & 0b1) > 0,
            g8_io4: ((value >> 31) & 0b1) > 0,
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
                | ((self.g1_io1 as u32) << 0)
                | ((self.g1_io2 as u32) << 1)
                | ((self.g1_io3 as u32) << 2)
                | ((self.g1_io4 as u32) << 3)
                | ((self.g2_io1 as u32) << 4)
                | ((self.g2_io2 as u32) << 5)
                | ((self.g2_io3 as u32) << 6)
                | ((self.g2_io4 as u32) << 7)
                | ((self.g3_io1 as u32) << 8)
                | ((self.g3_io2 as u32) << 9)
                | ((self.g3_io3 as u32) << 10)
                | ((self.g3_io4 as u32) << 11)
                | ((self.g4_io1 as u32) << 12)
                | ((self.g4_io2 as u32) << 13)
                | ((self.g4_io3 as u32) << 14)
                | ((self.g4_io4 as u32) << 15)
                | ((self.g5_io1 as u32) << 16)
                | ((self.g5_io2 as u32) << 17)
                | ((self.g5_io3 as u32) << 18)
                | ((self.g5_io4 as u32) << 19)
                | ((self.g6_io1 as u32) << 20)
                | ((self.g6_io2 as u32) << 21)
                | ((self.g6_io3 as u32) << 22)
                | ((self.g6_io4 as u32) << 23)
                | ((self.g7_io1 as u32) << 24)
                | ((self.g7_io2 as u32) << 25)
                | ((self.g7_io3 as u32) << 26)
                | ((self.g7_io4 as u32) << 27)
                | ((self.g8_io1 as u32) << 28)
                | ((self.g8_io2 as u32) << 29)
                | ((self.g8_io3 as u32) << 30)
                | ((self.g8_io4 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40024000u32 + 0x28u32) as *mut u32, value) };
        }
    }
}
/// I/O group control status register
pub mod iogcsr {
    /// Set Analog I/O group x status
    pub fn set_gs(index: u8, value: bool) {
        debug_assert!(index < 8, "set_gs out of range");
        let value = value as u32;
        let value = value << (16 + index * 1);
        unsafe { ::core::ptr::write_volatile((0x40024000u32 + 0x30u32) as *mut u32, value) };
    }
    /// Get Analog I/O group x status
    pub fn get_gs(index: u8) -> bool {
        debug_assert!(index < 8, "get_gs out of range");
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x30u32) as *mut u32) };
        let value = value & (0b1 << (16 + index * 1));
        value > 0
    }
    /// Set Analog I/O group x enable
    pub fn set_ge(index: u8, value: bool) {
        debug_assert!(index < 8, "set_ge out of range");
        let value = value as u32;
        let value = value << (0 + index * 1);
        unsafe { ::core::ptr::write_volatile((0x40024000u32 + 0x30u32) as *mut u32, value) };
    }
    /// Get Analog I/O group x enable
    pub fn get_ge(index: u8) -> bool {
        debug_assert!(index < 8, "get_ge out of range");
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x30u32) as *mut u32) };
        let value = value & (0b1 << (0 + index * 1));
        value > 0
    }
}
/// I/O group x counter register
pub mod iog1cr {
    /// Counter value
    /// Access: read-only, Width: 14, Offset: 0
    /// Get Counter value
    pub fn cnt() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x34u32) as *mut u32) };
        let value = value & (0b11111111111111 << 0);
        value as u16
    }
}
/// I/O group x counter register
pub mod iog2cr {
    /// Counter value
    /// Access: read-only, Width: 14, Offset: 0
    /// Get Counter value
    pub fn cnt() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x38u32) as *mut u32) };
        let value = value & (0b11111111111111 << 0);
        value as u16
    }
}
/// I/O group x counter register
pub mod iog3cr {
    /// Counter value
    /// Access: read-only, Width: 14, Offset: 0
    /// Get Counter value
    pub fn cnt() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x3Cu32) as *mut u32) };
        let value = value & (0b11111111111111 << 0);
        value as u16
    }
}
/// I/O group x counter register
pub mod iog4cr {
    /// Counter value
    /// Access: read-only, Width: 14, Offset: 0
    /// Get Counter value
    pub fn cnt() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x40u32) as *mut u32) };
        let value = value & (0b11111111111111 << 0);
        value as u16
    }
}
/// I/O group x counter register
pub mod iog5cr {
    /// Counter value
    /// Access: read-only, Width: 14, Offset: 0
    /// Get Counter value
    pub fn cnt() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x44u32) as *mut u32) };
        let value = value & (0b11111111111111 << 0);
        value as u16
    }
}
/// I/O group x counter register
pub mod iog6cr {
    /// Counter value
    /// Access: read-only, Width: 14, Offset: 0
    /// Get Counter value
    pub fn cnt() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x48u32) as *mut u32) };
        let value = value & (0b11111111111111 << 0);
        value as u16
    }
}
/// I/O group x counter register
pub mod iog7cr {
    /// Counter value
    /// Access: read-only, Width: 14, Offset: 0
    /// Get Counter value
    pub fn cnt() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x4Cu32) as *mut u32) };
        let value = value & (0b11111111111111 << 0);
        value as u16
    }
}
/// I/O group x counter register
pub mod iog8cr {
    /// Counter value
    /// Access: read-only, Width: 14, Offset: 0
    /// Get Counter value
    pub fn cnt() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40024000u32 + 0x50u32) as *mut u32) };
        let value = value & (0b11111111111111 << 0);
        value as u16
    }
}
/// EXTI Line2 and Touch sensing interrupts
pub const INTERRUPT_EXTI2_TSC: u32 = 8;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40024000</baseAddress>
  <description>Touch sensing controller</description>
  <groupName>TSC</groupName>
  <interrupt>
    <description>
                    EXTI Line2 and Touch sensing
                    interrupts
                </description>
    <name>EXTI2_TSC</name>
    <value>8</value>
  </interrupt>
  <name>TSC</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>control register</description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Charge transfer pulse high</description>
          <name>CTPH</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Charge transfer pulse low</description>
          <name>CTPL</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>7</bitWidth>
          <description>Spread spectrum deviation</description>
          <name>SSD</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Spread spectrum enable</description>
          <name>SSE</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Spread spectrum prescaler</description>
          <name>SSPSC</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>pulse generator prescaler</description>
          <name>PGPSC</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Max count value</description>
          <name>MCV</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I/O Default mode</description>
          <name>IODEF</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Synchronization pin
                                polarity
                            </description>
          <name>SYNCPOL</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Acquisition mode</description>
          <name>AM</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Start a new acquisition</description>
          <name>START</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Touch sensing controller
                                enable
                            </description>
          <name>TSCE</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>interrupt enable register</description>
      <displayName>IER</displayName>
      <fields>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Max count error interrupt
                                enable
                            </description>
          <name>MCEIE</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                End of acquisition interrupt
                                enable
                            </description>
          <name>EOAIE</name>
        </field>
      </fields>
      <name>IER</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>interrupt clear register</description>
      <displayName>ICR</displayName>
      <fields>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Max count error interrupt
                                clear
                            </description>
          <name>MCEIC</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                End of acquisition interrupt
                                clear
                            </description>
          <name>EOAIC</name>
        </field>
      </fields>
      <name>ICR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>interrupt status register</description>
      <displayName>ISR</displayName>
      <fields>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Max count error flag</description>
          <name>MCEF</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>End of acquisition flag</description>
          <name>EOAF</name>
        </field>
      </fields>
      <name>ISR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>
                        I/O hysteresis control
                        register
                    </description>
      <displayName>IOHCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G1_IO1 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G1_IO1</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G1_IO2 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G1_IO2</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G1_IO3 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G1_IO3</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G1_IO4 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G1_IO4</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G2_IO1 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G2_IO1</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G2_IO2 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G2_IO2</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G2_IO3 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G2_IO3</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G2_IO4 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G2_IO4</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G3_IO1 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G3_IO1</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G3_IO2 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G3_IO2</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G3_IO3 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G3_IO3</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G3_IO4 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G3_IO4</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G4_IO1 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G4_IO1</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G4_IO2 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G4_IO2</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G4_IO3 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G4_IO3</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G4_IO4 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G4_IO4</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G5_IO1 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G5_IO1</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G5_IO2 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G5_IO2</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G5_IO3 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G5_IO3</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G5_IO4 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G5_IO4</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G6_IO1 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G6_IO1</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G6_IO2 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G6_IO2</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G6_IO3 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G6_IO3</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G6_IO4 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G6_IO4</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G7_IO1 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G7_IO1</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G7_IO2 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G7_IO2</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G7_IO3 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G7_IO3</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G7_IO4 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G7_IO4</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G8_IO1 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G8_IO1</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G8_IO2 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G8_IO2</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G8_IO3 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G8_IO3</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G8_IO4 Schmitt trigger hysteresis
                                mode
                            </description>
          <name>G8_IO4</name>
        </field>
      </fields>
      <name>IOHCR</name>
      <resetValue>0xFFFFFFFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>
                        I/O analog switch control
                        register
                    </description>
      <displayName>IOASCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G1_IO1 analog switch
                                enable
                            </description>
          <name>G1_IO1</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G1_IO2 analog switch
                                enable
                            </description>
          <name>G1_IO2</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G1_IO3 analog switch
                                enable
                            </description>
          <name>G1_IO3</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G1_IO4 analog switch
                                enable
                            </description>
          <name>G1_IO4</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G2_IO1 analog switch
                                enable
                            </description>
          <name>G2_IO1</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G2_IO2 analog switch
                                enable
                            </description>
          <name>G2_IO2</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G2_IO3 analog switch
                                enable
                            </description>
          <name>G2_IO3</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G2_IO4 analog switch
                                enable
                            </description>
          <name>G2_IO4</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G3_IO1 analog switch
                                enable
                            </description>
          <name>G3_IO1</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G3_IO2 analog switch
                                enable
                            </description>
          <name>G3_IO2</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G3_IO3 analog switch
                                enable
                            </description>
          <name>G3_IO3</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G3_IO4 analog switch
                                enable
                            </description>
          <name>G3_IO4</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G4_IO1 analog switch
                                enable
                            </description>
          <name>G4_IO1</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G4_IO2 analog switch
                                enable
                            </description>
          <name>G4_IO2</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G4_IO3 analog switch
                                enable
                            </description>
          <name>G4_IO3</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G4_IO4 analog switch
                                enable
                            </description>
          <name>G4_IO4</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G5_IO1 analog switch
                                enable
                            </description>
          <name>G5_IO1</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G5_IO2 analog switch
                                enable
                            </description>
          <name>G5_IO2</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G5_IO3 analog switch
                                enable
                            </description>
          <name>G5_IO3</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G5_IO4 analog switch
                                enable
                            </description>
          <name>G5_IO4</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G6_IO1 analog switch
                                enable
                            </description>
          <name>G6_IO1</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G6_IO2 analog switch
                                enable
                            </description>
          <name>G6_IO2</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G6_IO3 analog switch
                                enable
                            </description>
          <name>G6_IO3</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G6_IO4 analog switch
                                enable
                            </description>
          <name>G6_IO4</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G7_IO1 analog switch
                                enable
                            </description>
          <name>G7_IO1</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G7_IO2 analog switch
                                enable
                            </description>
          <name>G7_IO2</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G7_IO3 analog switch
                                enable
                            </description>
          <name>G7_IO3</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G7_IO4 analog switch
                                enable
                            </description>
          <name>G7_IO4</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G8_IO1 analog switch
                                enable
                            </description>
          <name>G8_IO1</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G8_IO2 analog switch
                                enable
                            </description>
          <name>G8_IO2</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G8_IO3 analog switch
                                enable
                            </description>
          <name>G8_IO3</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                G8_IO4 analog switch
                                enable
                            </description>
          <name>G8_IO4</name>
        </field>
      </fields>
      <name>IOASCR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>I/O sampling control register</description>
      <displayName>IOSCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G1_IO1 sampling mode</description>
          <name>G1_IO1</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G1_IO2 sampling mode</description>
          <name>G1_IO2</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G1_IO3 sampling mode</description>
          <name>G1_IO3</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G1_IO4 sampling mode</description>
          <name>G1_IO4</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G2_IO1 sampling mode</description>
          <name>G2_IO1</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G2_IO2 sampling mode</description>
          <name>G2_IO2</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G2_IO3 sampling mode</description>
          <name>G2_IO3</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G2_IO4 sampling mode</description>
          <name>G2_IO4</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G3_IO1 sampling mode</description>
          <name>G3_IO1</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G3_IO2 sampling mode</description>
          <name>G3_IO2</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G3_IO3 sampling mode</description>
          <name>G3_IO3</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G3_IO4 sampling mode</description>
          <name>G3_IO4</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G4_IO1 sampling mode</description>
          <name>G4_IO1</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G4_IO2 sampling mode</description>
          <name>G4_IO2</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G4_IO3 sampling mode</description>
          <name>G4_IO3</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G4_IO4 sampling mode</description>
          <name>G4_IO4</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G5_IO1 sampling mode</description>
          <name>G5_IO1</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G5_IO2 sampling mode</description>
          <name>G5_IO2</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G5_IO3 sampling mode</description>
          <name>G5_IO3</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G5_IO4 sampling mode</description>
          <name>G5_IO4</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G6_IO1 sampling mode</description>
          <name>G6_IO1</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G6_IO2 sampling mode</description>
          <name>G6_IO2</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G6_IO3 sampling mode</description>
          <name>G6_IO3</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G6_IO4 sampling mode</description>
          <name>G6_IO4</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G7_IO1 sampling mode</description>
          <name>G7_IO1</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G7_IO2 sampling mode</description>
          <name>G7_IO2</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G7_IO3 sampling mode</description>
          <name>G7_IO3</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G7_IO4 sampling mode</description>
          <name>G7_IO4</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G8_IO1 sampling mode</description>
          <name>G8_IO1</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G8_IO2 sampling mode</description>
          <name>G8_IO2</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G8_IO3 sampling mode</description>
          <name>G8_IO3</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G8_IO4 sampling mode</description>
          <name>G8_IO4</name>
        </field>
      </fields>
      <name>IOSCR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28</addressOffset>
      <description>I/O channel control register</description>
      <displayName>IOCCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G1_IO1 channel mode</description>
          <name>G1_IO1</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G1_IO2 channel mode</description>
          <name>G1_IO2</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G1_IO3 channel mode</description>
          <name>G1_IO3</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G1_IO4 channel mode</description>
          <name>G1_IO4</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G2_IO1 channel mode</description>
          <name>G2_IO1</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G2_IO2 channel mode</description>
          <name>G2_IO2</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G2_IO3 channel mode</description>
          <name>G2_IO3</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G2_IO4 channel mode</description>
          <name>G2_IO4</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G3_IO1 channel mode</description>
          <name>G3_IO1</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G3_IO2 channel mode</description>
          <name>G3_IO2</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G3_IO3 channel mode</description>
          <name>G3_IO3</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G3_IO4 channel mode</description>
          <name>G3_IO4</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G4_IO1 channel mode</description>
          <name>G4_IO1</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G4_IO2 channel mode</description>
          <name>G4_IO2</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G4_IO3 channel mode</description>
          <name>G4_IO3</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G4_IO4 channel mode</description>
          <name>G4_IO4</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G5_IO1 channel mode</description>
          <name>G5_IO1</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G5_IO2 channel mode</description>
          <name>G5_IO2</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G5_IO3 channel mode</description>
          <name>G5_IO3</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G5_IO4 channel mode</description>
          <name>G5_IO4</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G6_IO1 channel mode</description>
          <name>G6_IO1</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G6_IO2 channel mode</description>
          <name>G6_IO2</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G6_IO3 channel mode</description>
          <name>G6_IO3</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G6_IO4 channel mode</description>
          <name>G6_IO4</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G7_IO1 channel mode</description>
          <name>G7_IO1</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G7_IO2 channel mode</description>
          <name>G7_IO2</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G7_IO3 channel mode</description>
          <name>G7_IO3</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G7_IO4 channel mode</description>
          <name>G7_IO4</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G8_IO1 channel mode</description>
          <name>G8_IO1</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G8_IO2 channel mode</description>
          <name>G8_IO2</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G8_IO3 channel mode</description>
          <name>G8_IO3</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>G8_IO4 channel mode</description>
          <name>G8_IO4</name>
        </field>
      </fields>
      <name>IOCCR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x30</addressOffset>
      <description>
                        I/O group control status
                        register
                    </description>
      <displayName>IOGCSR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x status</description>
          <name>G8S</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x status</description>
          <name>G7S</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x status</description>
          <name>G6S</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x status</description>
          <name>G5S</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x status</description>
          <name>G4S</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x status</description>
          <name>G3S</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x status</description>
          <name>G2S</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x status</description>
          <name>G1S</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x enable</description>
          <name>G8E</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x enable</description>
          <name>G7E</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x enable</description>
          <name>G6E</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x enable</description>
          <name>G5E</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x enable</description>
          <name>G4E</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x enable</description>
          <name>G3E</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x enable</description>
          <name>G2E</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog I/O group x enable</description>
          <name>G1E</name>
        </field>
      </fields>
      <name>IOGCSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x34</addressOffset>
      <description>I/O group x counter register</description>
      <displayName>IOG1CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
          <description>Counter value</description>
          <name>CNT</name>
        </field>
      </fields>
      <name>IOG1CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x38</addressOffset>
      <description>I/O group x counter register</description>
      <displayName>IOG2CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
          <description>Counter value</description>
          <name>CNT</name>
        </field>
      </fields>
      <name>IOG2CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x3C</addressOffset>
      <description>I/O group x counter register</description>
      <displayName>IOG3CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
          <description>Counter value</description>
          <name>CNT</name>
        </field>
      </fields>
      <name>IOG3CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x40</addressOffset>
      <description>I/O group x counter register</description>
      <displayName>IOG4CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
          <description>Counter value</description>
          <name>CNT</name>
        </field>
      </fields>
      <name>IOG4CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x44</addressOffset>
      <description>I/O group x counter register</description>
      <displayName>IOG5CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
          <description>Counter value</description>
          <name>CNT</name>
        </field>
      </fields>
      <name>IOG5CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x48</addressOffset>
      <description>I/O group x counter register</description>
      <displayName>IOG6CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
          <description>Counter value</description>
          <name>CNT</name>
        </field>
      </fields>
      <name>IOG6CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x4C</addressOffset>
      <description>I/O group x counter register</description>
      <displayName>IOG7CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
          <description>Counter value</description>
          <name>CNT</name>
        </field>
      </fields>
      <name>IOG7CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x50</addressOffset>
      <description>I/O group x counter register</description>
      <displayName>IOG8CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
          <description>Counter value</description>
          <name>CNT</name>
        </field>
      </fields>
      <name>IOG8CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
