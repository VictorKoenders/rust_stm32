/// GPIO port mode register
pub mod moder {
    pub struct ReadonlyCache {
        /// Port x configuration bits (y = 0..15)
        pub moder15: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder14: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder13: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder12: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder11: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder10: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder9: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder8: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder7: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder6: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder5: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder4: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder3: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder2: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder1: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder0: ::ModerType,
    }
    pub struct Cache {
        /// Port x configuration bits (y = 0..15)
        pub moder15: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder14: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder13: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder12: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder11: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder10: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder9: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder8: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder7: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder6: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder5: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder4: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder3: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder2: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder1: ::ModerType,
        /// Port x configuration bits (y = 0..15)
        pub moder0: ::ModerType,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            moder15: ((value >> 30) & 0b11) .into(),
            moder14: ((value >> 28) & 0b11) .into(),
            moder13: ((value >> 26) & 0b11) .into(),
            moder12: ((value >> 24) & 0b11) .into(),
            moder11: ((value >> 22) & 0b11) .into(),
            moder10: ((value >> 20) & 0b11) .into(),
            moder9: ((value >> 18) & 0b11) .into(),
            moder8: ((value >> 16) & 0b11) .into(),
            moder7: ((value >> 14) & 0b11) .into(),
            moder6: ((value >> 12) & 0b11) .into(),
            moder5: ((value >> 10) & 0b11) .into(),
            moder4: ((value >> 8) & 0b11) .into(),
            moder3: ((value >> 6) & 0b11) .into(),
            moder2: ((value >> 4) & 0b11) .into(),
            moder1: ((value >> 2) & 0b11) .into(),
            moder0: ((value >> 0) & 0b11) .into(),
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x0u32) as *mut u32) };
        Cache {
            moder15: ((value >> 30) & 0b11) .into(),
            moder14: ((value >> 28) & 0b11) .into(),
            moder13: ((value >> 26) & 0b11) .into(),
            moder12: ((value >> 24) & 0b11) .into(),
            moder11: ((value >> 22) & 0b11) .into(),
            moder10: ((value >> 20) & 0b11) .into(),
            moder9: ((value >> 18) & 0b11) .into(),
            moder8: ((value >> 16) & 0b11) .into(),
            moder7: ((value >> 14) & 0b11) .into(),
            moder6: ((value >> 12) & 0b11) .into(),
            moder5: ((value >> 10) & 0b11) .into(),
            moder4: ((value >> 8) & 0b11) .into(),
            moder3: ((value >> 6) & 0b11) .into(),
            moder2: ((value >> 4) & 0b11) .into(),
            moder1: ((value >> 2) & 0b11) .into(),
            moder0: ((value >> 0) & 0b11) .into(),
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
                | ((self.moder15 as u32) << 30)
                | ((self.moder14 as u32) << 28)
                | ((self.moder13 as u32) << 26)
                | ((self.moder12 as u32) << 24)
                | ((self.moder11 as u32) << 22)
                | ((self.moder10 as u32) << 20)
                | ((self.moder9 as u32) << 18)
                | ((self.moder8 as u32) << 16)
                | ((self.moder7 as u32) << 14)
                | ((self.moder6 as u32) << 12)
                | ((self.moder5 as u32) << 10)
                | ((self.moder4 as u32) << 8)
                | ((self.moder3 as u32) << 6)
                | ((self.moder2 as u32) << 4)
                | ((self.moder1 as u32) << 2)
                | ((self.moder0 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x48001000u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// GPIO port output type register
pub mod otyper {
    pub struct ReadonlyCache {
        /// Port x configuration bit 15
        pub ot15: bool,
        /// Port x configuration bit 14
        pub ot14: bool,
        /// Port x configuration bit 13
        pub ot13: bool,
        /// Port x configuration bit 12
        pub ot12: bool,
        /// Port x configuration bit 11
        pub ot11: bool,
        /// Port x configuration bit 10
        pub ot10: bool,
        /// Port x configuration bit 9
        pub ot9: bool,
        /// Port x configuration bit 8
        pub ot8: bool,
        /// Port x configuration bit 7
        pub ot7: bool,
        /// Port x configuration bit 6
        pub ot6: bool,
        /// Port x configuration bit 5
        pub ot5: bool,
        /// Port x configuration bit 4
        pub ot4: bool,
        /// Port x configuration bit 3
        pub ot3: bool,
        /// Port x configuration bit 2
        pub ot2: bool,
        /// Port x configuration bit 1
        pub ot1: bool,
        /// Port x configuration bit 0
        pub ot0: bool,
    }
    pub struct Cache {
        /// Port x configuration bit 15
        pub ot15: bool,
        /// Port x configuration bit 14
        pub ot14: bool,
        /// Port x configuration bit 13
        pub ot13: bool,
        /// Port x configuration bit 12
        pub ot12: bool,
        /// Port x configuration bit 11
        pub ot11: bool,
        /// Port x configuration bit 10
        pub ot10: bool,
        /// Port x configuration bit 9
        pub ot9: bool,
        /// Port x configuration bit 8
        pub ot8: bool,
        /// Port x configuration bit 7
        pub ot7: bool,
        /// Port x configuration bit 6
        pub ot6: bool,
        /// Port x configuration bit 5
        pub ot5: bool,
        /// Port x configuration bit 4
        pub ot4: bool,
        /// Port x configuration bit 3
        pub ot3: bool,
        /// Port x configuration bit 2
        pub ot2: bool,
        /// Port x configuration bit 1
        pub ot1: bool,
        /// Port x configuration bit 0
        pub ot0: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            ot15: ((value >> 15) & 0b1) > 0,
            ot14: ((value >> 14) & 0b1) > 0,
            ot13: ((value >> 13) & 0b1) > 0,
            ot12: ((value >> 12) & 0b1) > 0,
            ot11: ((value >> 11) & 0b1) > 0,
            ot10: ((value >> 10) & 0b1) > 0,
            ot9: ((value >> 9) & 0b1) > 0,
            ot8: ((value >> 8) & 0b1) > 0,
            ot7: ((value >> 7) & 0b1) > 0,
            ot6: ((value >> 6) & 0b1) > 0,
            ot5: ((value >> 5) & 0b1) > 0,
            ot4: ((value >> 4) & 0b1) > 0,
            ot3: ((value >> 3) & 0b1) > 0,
            ot2: ((value >> 2) & 0b1) > 0,
            ot1: ((value >> 1) & 0b1) > 0,
            ot0: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x4u32) as *mut u32) };
        Cache {
            ot15: ((value >> 15) & 0b1) > 0,
            ot14: ((value >> 14) & 0b1) > 0,
            ot13: ((value >> 13) & 0b1) > 0,
            ot12: ((value >> 12) & 0b1) > 0,
            ot11: ((value >> 11) & 0b1) > 0,
            ot10: ((value >> 10) & 0b1) > 0,
            ot9: ((value >> 9) & 0b1) > 0,
            ot8: ((value >> 8) & 0b1) > 0,
            ot7: ((value >> 7) & 0b1) > 0,
            ot6: ((value >> 6) & 0b1) > 0,
            ot5: ((value >> 5) & 0b1) > 0,
            ot4: ((value >> 4) & 0b1) > 0,
            ot3: ((value >> 3) & 0b1) > 0,
            ot2: ((value >> 2) & 0b1) > 0,
            ot1: ((value >> 1) & 0b1) > 0,
            ot0: ((value >> 0) & 0b1) > 0,
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
                | ((self.ot15 as u32) << 15)
                | ((self.ot14 as u32) << 14)
                | ((self.ot13 as u32) << 13)
                | ((self.ot12 as u32) << 12)
                | ((self.ot11 as u32) << 11)
                | ((self.ot10 as u32) << 10)
                | ((self.ot9 as u32) << 9)
                | ((self.ot8 as u32) << 8)
                | ((self.ot7 as u32) << 7)
                | ((self.ot6 as u32) << 6)
                | ((self.ot5 as u32) << 5)
                | ((self.ot4 as u32) << 4)
                | ((self.ot3 as u32) << 3)
                | ((self.ot2 as u32) << 2)
                | ((self.ot1 as u32) << 1)
                | ((self.ot0 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x48001000u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// GPIO port output speed register
pub mod ospeedr {
    pub struct ReadonlyCache {
        /// Port x configuration bits (y = 0..15)
        pub ospeedr15: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr14: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr13: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr12: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr11: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr10: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr9: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr8: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr7: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr6: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr5: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr4: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr3: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr2: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr1: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr0: u8,
    }
    pub struct Cache {
        /// Port x configuration bits (y = 0..15)
        pub ospeedr15: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr14: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr13: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr12: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr11: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr10: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr9: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr8: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr7: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr6: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr5: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr4: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr3: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr2: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr1: u8,
        /// Port x configuration bits (y = 0..15)
        pub ospeedr0: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            ospeedr15: ((value >> 30) & 0b11) as u8,
            ospeedr14: ((value >> 28) & 0b11) as u8,
            ospeedr13: ((value >> 26) & 0b11) as u8,
            ospeedr12: ((value >> 24) & 0b11) as u8,
            ospeedr11: ((value >> 22) & 0b11) as u8,
            ospeedr10: ((value >> 20) & 0b11) as u8,
            ospeedr9: ((value >> 18) & 0b11) as u8,
            ospeedr8: ((value >> 16) & 0b11) as u8,
            ospeedr7: ((value >> 14) & 0b11) as u8,
            ospeedr6: ((value >> 12) & 0b11) as u8,
            ospeedr5: ((value >> 10) & 0b11) as u8,
            ospeedr4: ((value >> 8) & 0b11) as u8,
            ospeedr3: ((value >> 6) & 0b11) as u8,
            ospeedr2: ((value >> 4) & 0b11) as u8,
            ospeedr1: ((value >> 2) & 0b11) as u8,
            ospeedr0: ((value >> 0) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x8u32) as *mut u32) };
        Cache {
            ospeedr15: ((value >> 30) & 0b11) as u8,
            ospeedr14: ((value >> 28) & 0b11) as u8,
            ospeedr13: ((value >> 26) & 0b11) as u8,
            ospeedr12: ((value >> 24) & 0b11) as u8,
            ospeedr11: ((value >> 22) & 0b11) as u8,
            ospeedr10: ((value >> 20) & 0b11) as u8,
            ospeedr9: ((value >> 18) & 0b11) as u8,
            ospeedr8: ((value >> 16) & 0b11) as u8,
            ospeedr7: ((value >> 14) & 0b11) as u8,
            ospeedr6: ((value >> 12) & 0b11) as u8,
            ospeedr5: ((value >> 10) & 0b11) as u8,
            ospeedr4: ((value >> 8) & 0b11) as u8,
            ospeedr3: ((value >> 6) & 0b11) as u8,
            ospeedr2: ((value >> 4) & 0b11) as u8,
            ospeedr1: ((value >> 2) & 0b11) as u8,
            ospeedr0: ((value >> 0) & 0b11) as u8,
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
                | ((self.ospeedr15 as u32) << 30)
                | ((self.ospeedr14 as u32) << 28)
                | ((self.ospeedr13 as u32) << 26)
                | ((self.ospeedr12 as u32) << 24)
                | ((self.ospeedr11 as u32) << 22)
                | ((self.ospeedr10 as u32) << 20)
                | ((self.ospeedr9 as u32) << 18)
                | ((self.ospeedr8 as u32) << 16)
                | ((self.ospeedr7 as u32) << 14)
                | ((self.ospeedr6 as u32) << 12)
                | ((self.ospeedr5 as u32) << 10)
                | ((self.ospeedr4 as u32) << 8)
                | ((self.ospeedr3 as u32) << 6)
                | ((self.ospeedr2 as u32) << 4)
                | ((self.ospeedr1 as u32) << 2)
                | ((self.ospeedr0 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x48001000u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// GPIO port pull-up/pull-down register
pub mod pupdr {
    pub struct ReadonlyCache {
        /// Port x configuration bits (y = 0..15)
        pub pupdr15: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr14: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr13: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr12: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr11: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr10: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr9: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr8: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr7: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr6: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr5: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr4: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr3: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr2: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr1: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr0: u8,
    }
    pub struct Cache {
        /// Port x configuration bits (y = 0..15)
        pub pupdr15: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr14: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr13: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr12: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr11: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr10: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr9: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr8: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr7: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr6: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr5: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr4: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr3: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr2: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr1: u8,
        /// Port x configuration bits (y = 0..15)
        pub pupdr0: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            pupdr15: ((value >> 30) & 0b11) as u8,
            pupdr14: ((value >> 28) & 0b11) as u8,
            pupdr13: ((value >> 26) & 0b11) as u8,
            pupdr12: ((value >> 24) & 0b11) as u8,
            pupdr11: ((value >> 22) & 0b11) as u8,
            pupdr10: ((value >> 20) & 0b11) as u8,
            pupdr9: ((value >> 18) & 0b11) as u8,
            pupdr8: ((value >> 16) & 0b11) as u8,
            pupdr7: ((value >> 14) & 0b11) as u8,
            pupdr6: ((value >> 12) & 0b11) as u8,
            pupdr5: ((value >> 10) & 0b11) as u8,
            pupdr4: ((value >> 8) & 0b11) as u8,
            pupdr3: ((value >> 6) & 0b11) as u8,
            pupdr2: ((value >> 4) & 0b11) as u8,
            pupdr1: ((value >> 2) & 0b11) as u8,
            pupdr0: ((value >> 0) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0xCu32) as *mut u32) };
        Cache {
            pupdr15: ((value >> 30) & 0b11) as u8,
            pupdr14: ((value >> 28) & 0b11) as u8,
            pupdr13: ((value >> 26) & 0b11) as u8,
            pupdr12: ((value >> 24) & 0b11) as u8,
            pupdr11: ((value >> 22) & 0b11) as u8,
            pupdr10: ((value >> 20) & 0b11) as u8,
            pupdr9: ((value >> 18) & 0b11) as u8,
            pupdr8: ((value >> 16) & 0b11) as u8,
            pupdr7: ((value >> 14) & 0b11) as u8,
            pupdr6: ((value >> 12) & 0b11) as u8,
            pupdr5: ((value >> 10) & 0b11) as u8,
            pupdr4: ((value >> 8) & 0b11) as u8,
            pupdr3: ((value >> 6) & 0b11) as u8,
            pupdr2: ((value >> 4) & 0b11) as u8,
            pupdr1: ((value >> 2) & 0b11) as u8,
            pupdr0: ((value >> 0) & 0b11) as u8,
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
                | ((self.pupdr15 as u32) << 30)
                | ((self.pupdr14 as u32) << 28)
                | ((self.pupdr13 as u32) << 26)
                | ((self.pupdr12 as u32) << 24)
                | ((self.pupdr11 as u32) << 22)
                | ((self.pupdr10 as u32) << 20)
                | ((self.pupdr9 as u32) << 18)
                | ((self.pupdr8 as u32) << 16)
                | ((self.pupdr7 as u32) << 14)
                | ((self.pupdr6 as u32) << 12)
                | ((self.pupdr5 as u32) << 10)
                | ((self.pupdr4 as u32) << 8)
                | ((self.pupdr3 as u32) << 6)
                | ((self.pupdr2 as u32) << 4)
                | ((self.pupdr1 as u32) << 2)
                | ((self.pupdr0 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x48001000u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// GPIO port input data register
pub mod idr {
    /// Get Port input data (y = 0..15)
    pub fn idr(index: u8) -> bool {
        debug_assert!(index < 16, "idr out of range");
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << (0 + index * 1));
        value > 0
    }
}
/// GPIO port output data register
pub mod odr {
    pub struct ReadonlyCache {
        /// Port output data (y = 0..15)
        pub odr15: bool,
        /// Port output data (y = 0..15)
        pub odr14: bool,
        /// Port output data (y = 0..15)
        pub odr13: bool,
        /// Port output data (y = 0..15)
        pub odr12: bool,
        /// Port output data (y = 0..15)
        pub odr11: bool,
        /// Port output data (y = 0..15)
        pub odr10: bool,
        /// Port output data (y = 0..15)
        pub odr9: bool,
        /// Port output data (y = 0..15)
        pub odr8: bool,
        /// Port output data (y = 0..15)
        pub odr7: bool,
        /// Port output data (y = 0..15)
        pub odr6: bool,
        /// Port output data (y = 0..15)
        pub odr5: bool,
        /// Port output data (y = 0..15)
        pub odr4: bool,
        /// Port output data (y = 0..15)
        pub odr3: bool,
        /// Port output data (y = 0..15)
        pub odr2: bool,
        /// Port output data (y = 0..15)
        pub odr1: bool,
        /// Port output data (y = 0..15)
        pub odr0: bool,
    }
    pub struct Cache {
        /// Port output data (y = 0..15)
        pub odr15: bool,
        /// Port output data (y = 0..15)
        pub odr14: bool,
        /// Port output data (y = 0..15)
        pub odr13: bool,
        /// Port output data (y = 0..15)
        pub odr12: bool,
        /// Port output data (y = 0..15)
        pub odr11: bool,
        /// Port output data (y = 0..15)
        pub odr10: bool,
        /// Port output data (y = 0..15)
        pub odr9: bool,
        /// Port output data (y = 0..15)
        pub odr8: bool,
        /// Port output data (y = 0..15)
        pub odr7: bool,
        /// Port output data (y = 0..15)
        pub odr6: bool,
        /// Port output data (y = 0..15)
        pub odr5: bool,
        /// Port output data (y = 0..15)
        pub odr4: bool,
        /// Port output data (y = 0..15)
        pub odr3: bool,
        /// Port output data (y = 0..15)
        pub odr2: bool,
        /// Port output data (y = 0..15)
        pub odr1: bool,
        /// Port output data (y = 0..15)
        pub odr0: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x14u32) as *mut u32) };
        ReadonlyCache {
            odr15: ((value >> 15) & 0b1) > 0,
            odr14: ((value >> 14) & 0b1) > 0,
            odr13: ((value >> 13) & 0b1) > 0,
            odr12: ((value >> 12) & 0b1) > 0,
            odr11: ((value >> 11) & 0b1) > 0,
            odr10: ((value >> 10) & 0b1) > 0,
            odr9: ((value >> 9) & 0b1) > 0,
            odr8: ((value >> 8) & 0b1) > 0,
            odr7: ((value >> 7) & 0b1) > 0,
            odr6: ((value >> 6) & 0b1) > 0,
            odr5: ((value >> 5) & 0b1) > 0,
            odr4: ((value >> 4) & 0b1) > 0,
            odr3: ((value >> 3) & 0b1) > 0,
            odr2: ((value >> 2) & 0b1) > 0,
            odr1: ((value >> 1) & 0b1) > 0,
            odr0: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x14u32) as *mut u32) };
        Cache {
            odr15: ((value >> 15) & 0b1) > 0,
            odr14: ((value >> 14) & 0b1) > 0,
            odr13: ((value >> 13) & 0b1) > 0,
            odr12: ((value >> 12) & 0b1) > 0,
            odr11: ((value >> 11) & 0b1) > 0,
            odr10: ((value >> 10) & 0b1) > 0,
            odr9: ((value >> 9) & 0b1) > 0,
            odr8: ((value >> 8) & 0b1) > 0,
            odr7: ((value >> 7) & 0b1) > 0,
            odr6: ((value >> 6) & 0b1) > 0,
            odr5: ((value >> 5) & 0b1) > 0,
            odr4: ((value >> 4) & 0b1) > 0,
            odr3: ((value >> 3) & 0b1) > 0,
            odr2: ((value >> 2) & 0b1) > 0,
            odr1: ((value >> 1) & 0b1) > 0,
            odr0: ((value >> 0) & 0b1) > 0,
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
                | ((self.odr15 as u32) << 15)
                | ((self.odr14 as u32) << 14)
                | ((self.odr13 as u32) << 13)
                | ((self.odr12 as u32) << 12)
                | ((self.odr11 as u32) << 11)
                | ((self.odr10 as u32) << 10)
                | ((self.odr9 as u32) << 9)
                | ((self.odr8 as u32) << 8)
                | ((self.odr7 as u32) << 7)
                | ((self.odr6 as u32) << 6)
                | ((self.odr5 as u32) << 5)
                | ((self.odr4 as u32) << 4)
                | ((self.odr3 as u32) << 3)
                | ((self.odr2 as u32) << 2)
                | ((self.odr1 as u32) << 1)
                | ((self.odr0 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x48001000u32 + 0x14u32) as *mut u32, value) };
        }
    }
}
/// GPIO port bit set/reset register
pub mod bsrr {
    /// Set Port x reset bit y (y = 0..15)
    pub fn br(index: u8, value: bool) {
        debug_assert!(index < 16, "br out of range");
        let value = value as u32;
        let value = value << (16 + index * 1);
        unsafe { ::core::ptr::write_volatile((0x48001000u32 + 0x18u32) as *mut u32, value) };
    }
    /// Set Port x set bit y (y= 0..15)
    pub fn bs(index: u8, value: bool) {
        debug_assert!(index < 16, "bs out of range");
        let value = value as u32;
        let value = value << (0 + index * 1);
        unsafe { ::core::ptr::write_volatile((0x48001000u32 + 0x18u32) as *mut u32, value) };
    }
}
/// GPIO port configuration lock register
pub mod lckr {
    pub struct ReadonlyCache {
        /// Lok Key
        pub lckk: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck15: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck14: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck13: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck12: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck11: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck10: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck9: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck8: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck7: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck6: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck5: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck4: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck3: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck2: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck1: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck0: bool,
    }
    pub struct Cache {
        /// Lok Key
        pub lckk: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck15: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck14: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck13: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck12: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck11: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck10: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck9: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck8: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck7: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck6: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck5: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck4: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck3: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck2: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck1: bool,
        /// Port x lock bit y (y= 0..15)
        pub lck0: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x1Cu32) as *mut u32) };
        ReadonlyCache {
            lckk: ((value >> 16) & 0b1) > 0,
            lck15: ((value >> 15) & 0b1) > 0,
            lck14: ((value >> 14) & 0b1) > 0,
            lck13: ((value >> 13) & 0b1) > 0,
            lck12: ((value >> 12) & 0b1) > 0,
            lck11: ((value >> 11) & 0b1) > 0,
            lck10: ((value >> 10) & 0b1) > 0,
            lck9: ((value >> 9) & 0b1) > 0,
            lck8: ((value >> 8) & 0b1) > 0,
            lck7: ((value >> 7) & 0b1) > 0,
            lck6: ((value >> 6) & 0b1) > 0,
            lck5: ((value >> 5) & 0b1) > 0,
            lck4: ((value >> 4) & 0b1) > 0,
            lck3: ((value >> 3) & 0b1) > 0,
            lck2: ((value >> 2) & 0b1) > 0,
            lck1: ((value >> 1) & 0b1) > 0,
            lck0: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x1Cu32) as *mut u32) };
        Cache {
            lckk: ((value >> 16) & 0b1) > 0,
            lck15: ((value >> 15) & 0b1) > 0,
            lck14: ((value >> 14) & 0b1) > 0,
            lck13: ((value >> 13) & 0b1) > 0,
            lck12: ((value >> 12) & 0b1) > 0,
            lck11: ((value >> 11) & 0b1) > 0,
            lck10: ((value >> 10) & 0b1) > 0,
            lck9: ((value >> 9) & 0b1) > 0,
            lck8: ((value >> 8) & 0b1) > 0,
            lck7: ((value >> 7) & 0b1) > 0,
            lck6: ((value >> 6) & 0b1) > 0,
            lck5: ((value >> 5) & 0b1) > 0,
            lck4: ((value >> 4) & 0b1) > 0,
            lck3: ((value >> 3) & 0b1) > 0,
            lck2: ((value >> 2) & 0b1) > 0,
            lck1: ((value >> 1) & 0b1) > 0,
            lck0: ((value >> 0) & 0b1) > 0,
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
                | ((self.lckk as u32) << 16)
                | ((self.lck15 as u32) << 15)
                | ((self.lck14 as u32) << 14)
                | ((self.lck13 as u32) << 13)
                | ((self.lck12 as u32) << 12)
                | ((self.lck11 as u32) << 11)
                | ((self.lck10 as u32) << 10)
                | ((self.lck9 as u32) << 9)
                | ((self.lck8 as u32) << 8)
                | ((self.lck7 as u32) << 7)
                | ((self.lck6 as u32) << 6)
                | ((self.lck5 as u32) << 5)
                | ((self.lck4 as u32) << 4)
                | ((self.lck3 as u32) << 3)
                | ((self.lck2 as u32) << 2)
                | ((self.lck1 as u32) << 1)
                | ((self.lck0 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x48001000u32 + 0x1Cu32) as *mut u32, value) };
        }
    }
}
/// GPIO alternate function low register
pub mod afrl {
    pub struct ReadonlyCache {
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl7: u8,
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl6: u8,
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl5: u8,
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl4: u8,
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl3: u8,
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl2: u8,
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl1: u8,
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl0: u8,
    }
    pub struct Cache {
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl7: u8,
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl6: u8,
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl5: u8,
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl4: u8,
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl3: u8,
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl2: u8,
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl1: u8,
        /// Alternate function selection for port x bit y (y = 0..7)
        pub afrl0: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x20u32) as *mut u32) };
        ReadonlyCache {
            afrl7: ((value >> 28) & 0b1111) as u8,
            afrl6: ((value >> 24) & 0b1111) as u8,
            afrl5: ((value >> 20) & 0b1111) as u8,
            afrl4: ((value >> 16) & 0b1111) as u8,
            afrl3: ((value >> 12) & 0b1111) as u8,
            afrl2: ((value >> 8) & 0b1111) as u8,
            afrl1: ((value >> 4) & 0b1111) as u8,
            afrl0: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x20u32) as *mut u32) };
        Cache {
            afrl7: ((value >> 28) & 0b1111) as u8,
            afrl6: ((value >> 24) & 0b1111) as u8,
            afrl5: ((value >> 20) & 0b1111) as u8,
            afrl4: ((value >> 16) & 0b1111) as u8,
            afrl3: ((value >> 12) & 0b1111) as u8,
            afrl2: ((value >> 8) & 0b1111) as u8,
            afrl1: ((value >> 4) & 0b1111) as u8,
            afrl0: ((value >> 0) & 0b1111) as u8,
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
                | ((self.afrl7 as u32) << 28)
                | ((self.afrl6 as u32) << 24)
                | ((self.afrl5 as u32) << 20)
                | ((self.afrl4 as u32) << 16)
                | ((self.afrl3 as u32) << 12)
                | ((self.afrl2 as u32) << 8)
                | ((self.afrl1 as u32) << 4)
                | ((self.afrl0 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x48001000u32 + 0x20u32) as *mut u32, value) };
        }
    }
}
/// GPIO alternate function high register
pub mod afrh {
    pub struct ReadonlyCache {
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh15: u8,
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh14: u8,
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh13: u8,
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh12: u8,
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh11: u8,
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh10: u8,
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh9: u8,
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh8: u8,
    }
    pub struct Cache {
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh15: u8,
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh14: u8,
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh13: u8,
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh12: u8,
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh11: u8,
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh10: u8,
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh9: u8,
        /// Alternate function selection for port x bit y (y = 8..15)
        pub afrh8: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x24u32) as *mut u32) };
        ReadonlyCache {
            afrh15: ((value >> 28) & 0b1111) as u8,
            afrh14: ((value >> 24) & 0b1111) as u8,
            afrh13: ((value >> 20) & 0b1111) as u8,
            afrh12: ((value >> 16) & 0b1111) as u8,
            afrh11: ((value >> 12) & 0b1111) as u8,
            afrh10: ((value >> 8) & 0b1111) as u8,
            afrh9: ((value >> 4) & 0b1111) as u8,
            afrh8: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x48001000u32 + 0x24u32) as *mut u32) };
        Cache {
            afrh15: ((value >> 28) & 0b1111) as u8,
            afrh14: ((value >> 24) & 0b1111) as u8,
            afrh13: ((value >> 20) & 0b1111) as u8,
            afrh12: ((value >> 16) & 0b1111) as u8,
            afrh11: ((value >> 12) & 0b1111) as u8,
            afrh10: ((value >> 8) & 0b1111) as u8,
            afrh9: ((value >> 4) & 0b1111) as u8,
            afrh8: ((value >> 0) & 0b1111) as u8,
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
                | ((self.afrh15 as u32) << 28)
                | ((self.afrh14 as u32) << 24)
                | ((self.afrh13 as u32) << 20)
                | ((self.afrh12 as u32) << 16)
                | ((self.afrh11 as u32) << 12)
                | ((self.afrh10 as u32) << 8)
                | ((self.afrh9 as u32) << 4)
                | ((self.afrh8 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x48001000u32 + 0x24u32) as *mut u32, value) };
        }
    }
}
/// Port bit reset register
pub mod brr {
    /// Set Port x Reset bit y
    pub fn br(index: u8, value: bool) {
        debug_assert!(index < 16, "br out of range");
        let value = value as u32;
        let value = value << (0 + index * 1);
        unsafe { ::core::ptr::write_volatile((0x48001000u32 + 0x28u32) as *mut u32, value) };
    }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" derivedFrom="GPIOB">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x48001000</baseAddress>
  <description>General-purpose I/Os</description>
  <groupName>GPIO</groupName>
  <name>GPIOE</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>GPIO port mode register</description>
      <displayName>MODER</displayName>
      <fields>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER15</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER14</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER13</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER12</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER11</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER10</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER9</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER8</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER7</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER6</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER5</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER4</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER2</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>MODER0</name>
        </field>
      </fields>
      <name>MODER</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>GPIO port output type register</description>
      <displayName>OTYPER</displayName>
      <fields>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x configuration bit
                                15
                            </description>
          <name>OT15</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x configuration bit
                                14
                            </description>
          <name>OT14</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x configuration bit
                                13
                            </description>
          <name>OT13</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x configuration bit
                                12
                            </description>
          <name>OT12</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x configuration bit
                                11
                            </description>
          <name>OT11</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x configuration bit
                                10
                            </description>
          <name>OT10</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x configuration bit 9</description>
          <name>OT9</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x configuration bit 8</description>
          <name>OT8</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x configuration bit 7</description>
          <name>OT7</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x configuration bit 6</description>
          <name>OT6</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x configuration bit 5</description>
          <name>OT5</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x configuration bit 4</description>
          <name>OT4</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x configuration bit 3</description>
          <name>OT3</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x configuration bit 2</description>
          <name>OT2</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x configuration bit 1</description>
          <name>OT1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x configuration bit 0</description>
          <name>OT0</name>
        </field>
      </fields>
      <name>OTYPER</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>
                        GPIO port output speed
                        register
                    </description>
      <displayName>OSPEEDR</displayName>
      <fields>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR15</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR14</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR13</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR12</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR11</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR10</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR9</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR8</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR7</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR6</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR5</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR4</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR2</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>OSPEEDR0</name>
        </field>
      </fields>
      <name>OSPEEDR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>
                        GPIO port pull-up/pull-down
                        register
                    </description>
      <displayName>PUPDR</displayName>
      <fields>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR15</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR14</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR13</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR12</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR11</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR10</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR9</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR8</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR7</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR6</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR5</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR4</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR2</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Port x configuration bits (y =
                                0..15)
                            </description>
          <name>PUPDR0</name>
        </field>
      </fields>
      <name>PUPDR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x10</addressOffset>
      <description>GPIO port input data register</description>
      <displayName>IDR</displayName>
      <fields>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR15</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR14</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR13</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR12</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR11</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR10</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR9</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR8</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR7</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR6</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR5</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR4</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR3</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR2</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port input data (y =
                                0..15)
                            </description>
          <name>IDR0</name>
        </field>
      </fields>
      <name>IDR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>GPIO port output data register</description>
      <displayName>ODR</displayName>
      <fields>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR15</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR14</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR13</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR12</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR11</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR10</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR9</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR8</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR7</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR6</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR5</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR4</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR3</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR2</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port output data (y =
                                0..15)
                            </description>
          <name>ODR0</name>
        </field>
      </fields>
      <name>ODR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x18</addressOffset>
      <description>
                        GPIO port bit set/reset
                        register
                    </description>
      <displayName>BSRR</displayName>
      <fields>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR15</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR14</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR13</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR12</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR11</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR10</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR9</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR8</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR7</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR6</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR5</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR4</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR3</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR2</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x reset bit y (y =
                                0..15)
                            </description>
          <name>BR1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BR0</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS15</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS14</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS13</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS12</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS11</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS10</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS9</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS8</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS7</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS6</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS5</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS4</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS3</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS2</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x set bit y (y=
                                0..15)
                            </description>
          <name>BS0</name>
        </field>
      </fields>
      <name>BSRR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>
                        GPIO port configuration lock
                        register
                    </description>
      <displayName>LCKR</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Lok Key</description>
          <name>LCKK</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK15</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK14</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK13</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK12</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK11</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK10</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK9</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK8</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK7</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK6</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK5</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK4</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK3</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK2</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Port x lock bit y (y=
                                0..15)
                            </description>
          <name>LCK0</name>
        </field>
      </fields>
      <name>LCKR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>
                        GPIO alternate function low
                        register
                    </description>
      <displayName>AFRL</displayName>
      <fields>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 0..7)
                            </description>
          <name>AFRL7</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 0..7)
                            </description>
          <name>AFRL6</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 0..7)
                            </description>
          <name>AFRL5</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 0..7)
                            </description>
          <name>AFRL4</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 0..7)
                            </description>
          <name>AFRL3</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 0..7)
                            </description>
          <name>AFRL2</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 0..7)
                            </description>
          <name>AFRL1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 0..7)
                            </description>
          <name>AFRL0</name>
        </field>
      </fields>
      <name>AFRL</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x24</addressOffset>
      <description>
                        GPIO alternate function high
                        register
                    </description>
      <displayName>AFRH</displayName>
      <fields>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 8..15)
                            </description>
          <name>AFRH15</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 8..15)
                            </description>
          <name>AFRH14</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 8..15)
                            </description>
          <name>AFRH13</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 8..15)
                            </description>
          <name>AFRH12</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 8..15)
                            </description>
          <name>AFRH11</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 8..15)
                            </description>
          <name>AFRH10</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 8..15)
                            </description>
          <name>AFRH9</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Alternate function selection for port x
                                bit y (y = 8..15)
                            </description>
          <name>AFRH8</name>
        </field>
      </fields>
      <name>AFRH</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x28</addressOffset>
      <description>Port bit reset register</description>
      <displayName>BRR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port x Reset bit y</description>
          <name>BR15</name>
        </field>
      </fields>
      <name>BRR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
