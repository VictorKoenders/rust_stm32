/// Interrupt mask register
pub mod imr1 {
    pub struct ReadonlyCache {
        /// Interrupt Mask on line 0
        pub mr0: bool,
        /// Interrupt Mask on line 1
        pub mr1: bool,
        /// Interrupt Mask on line 2
        pub mr2: bool,
        /// Interrupt Mask on line 3
        pub mr3: bool,
        /// Interrupt Mask on line 4
        pub mr4: bool,
        /// Interrupt Mask on line 5
        pub mr5: bool,
        /// Interrupt Mask on line 6
        pub mr6: bool,
        /// Interrupt Mask on line 7
        pub mr7: bool,
        /// Interrupt Mask on line 8
        pub mr8: bool,
        /// Interrupt Mask on line 9
        pub mr9: bool,
        /// Interrupt Mask on line 10
        pub mr10: bool,
        /// Interrupt Mask on line 11
        pub mr11: bool,
        /// Interrupt Mask on line 12
        pub mr12: bool,
        /// Interrupt Mask on line 13
        pub mr13: bool,
        /// Interrupt Mask on line 14
        pub mr14: bool,
        /// Interrupt Mask on line 15
        pub mr15: bool,
        /// Interrupt Mask on line 16
        pub mr16: bool,
        /// Interrupt Mask on line 17
        pub mr17: bool,
        /// Interrupt Mask on line 18
        pub mr18: bool,
        /// Interrupt Mask on line 19
        pub mr19: bool,
        /// Interrupt Mask on line 20
        pub mr20: bool,
        /// Interrupt Mask on line 21
        pub mr21: bool,
        /// Interrupt Mask on line 22
        pub mr22: bool,
        /// Interrupt Mask on line 23
        pub mr23: bool,
        /// Interrupt Mask on line 24
        pub mr24: bool,
        /// Interrupt Mask on line 25
        pub mr25: bool,
        /// Interrupt Mask on line 26
        pub mr26: bool,
        /// Interrupt Mask on line 27
        pub mr27: bool,
        /// Interrupt Mask on line 28
        pub mr28: bool,
        /// Interrupt Mask on line 29
        pub mr29: bool,
        /// Interrupt Mask on line 30
        pub mr30: bool,
        /// Interrupt Mask on line 31
        pub mr31: bool,
    }
    pub struct Cache {
        /// Interrupt Mask on line 0
        pub mr0: bool,
        /// Interrupt Mask on line 1
        pub mr1: bool,
        /// Interrupt Mask on line 2
        pub mr2: bool,
        /// Interrupt Mask on line 3
        pub mr3: bool,
        /// Interrupt Mask on line 4
        pub mr4: bool,
        /// Interrupt Mask on line 5
        pub mr5: bool,
        /// Interrupt Mask on line 6
        pub mr6: bool,
        /// Interrupt Mask on line 7
        pub mr7: bool,
        /// Interrupt Mask on line 8
        pub mr8: bool,
        /// Interrupt Mask on line 9
        pub mr9: bool,
        /// Interrupt Mask on line 10
        pub mr10: bool,
        /// Interrupt Mask on line 11
        pub mr11: bool,
        /// Interrupt Mask on line 12
        pub mr12: bool,
        /// Interrupt Mask on line 13
        pub mr13: bool,
        /// Interrupt Mask on line 14
        pub mr14: bool,
        /// Interrupt Mask on line 15
        pub mr15: bool,
        /// Interrupt Mask on line 16
        pub mr16: bool,
        /// Interrupt Mask on line 17
        pub mr17: bool,
        /// Interrupt Mask on line 18
        pub mr18: bool,
        /// Interrupt Mask on line 19
        pub mr19: bool,
        /// Interrupt Mask on line 20
        pub mr20: bool,
        /// Interrupt Mask on line 21
        pub mr21: bool,
        /// Interrupt Mask on line 22
        pub mr22: bool,
        /// Interrupt Mask on line 23
        pub mr23: bool,
        /// Interrupt Mask on line 24
        pub mr24: bool,
        /// Interrupt Mask on line 25
        pub mr25: bool,
        /// Interrupt Mask on line 26
        pub mr26: bool,
        /// Interrupt Mask on line 27
        pub mr27: bool,
        /// Interrupt Mask on line 28
        pub mr28: bool,
        /// Interrupt Mask on line 29
        pub mr29: bool,
        /// Interrupt Mask on line 30
        pub mr30: bool,
        /// Interrupt Mask on line 31
        pub mr31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            mr0: ((value >> 0) & 0b1) > 0,
            mr1: ((value >> 1) & 0b1) > 0,
            mr2: ((value >> 2) & 0b1) > 0,
            mr3: ((value >> 3) & 0b1) > 0,
            mr4: ((value >> 4) & 0b1) > 0,
            mr5: ((value >> 5) & 0b1) > 0,
            mr6: ((value >> 6) & 0b1) > 0,
            mr7: ((value >> 7) & 0b1) > 0,
            mr8: ((value >> 8) & 0b1) > 0,
            mr9: ((value >> 9) & 0b1) > 0,
            mr10: ((value >> 10) & 0b1) > 0,
            mr11: ((value >> 11) & 0b1) > 0,
            mr12: ((value >> 12) & 0b1) > 0,
            mr13: ((value >> 13) & 0b1) > 0,
            mr14: ((value >> 14) & 0b1) > 0,
            mr15: ((value >> 15) & 0b1) > 0,
            mr16: ((value >> 16) & 0b1) > 0,
            mr17: ((value >> 17) & 0b1) > 0,
            mr18: ((value >> 18) & 0b1) > 0,
            mr19: ((value >> 19) & 0b1) > 0,
            mr20: ((value >> 20) & 0b1) > 0,
            mr21: ((value >> 21) & 0b1) > 0,
            mr22: ((value >> 22) & 0b1) > 0,
            mr23: ((value >> 23) & 0b1) > 0,
            mr24: ((value >> 24) & 0b1) > 0,
            mr25: ((value >> 25) & 0b1) > 0,
            mr26: ((value >> 26) & 0b1) > 0,
            mr27: ((value >> 27) & 0b1) > 0,
            mr28: ((value >> 28) & 0b1) > 0,
            mr29: ((value >> 29) & 0b1) > 0,
            mr30: ((value >> 30) & 0b1) > 0,
            mr31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x0u32) as *mut u32) };
        Cache {
            mr0: ((value >> 0) & 0b1) > 0,
            mr1: ((value >> 1) & 0b1) > 0,
            mr2: ((value >> 2) & 0b1) > 0,
            mr3: ((value >> 3) & 0b1) > 0,
            mr4: ((value >> 4) & 0b1) > 0,
            mr5: ((value >> 5) & 0b1) > 0,
            mr6: ((value >> 6) & 0b1) > 0,
            mr7: ((value >> 7) & 0b1) > 0,
            mr8: ((value >> 8) & 0b1) > 0,
            mr9: ((value >> 9) & 0b1) > 0,
            mr10: ((value >> 10) & 0b1) > 0,
            mr11: ((value >> 11) & 0b1) > 0,
            mr12: ((value >> 12) & 0b1) > 0,
            mr13: ((value >> 13) & 0b1) > 0,
            mr14: ((value >> 14) & 0b1) > 0,
            mr15: ((value >> 15) & 0b1) > 0,
            mr16: ((value >> 16) & 0b1) > 0,
            mr17: ((value >> 17) & 0b1) > 0,
            mr18: ((value >> 18) & 0b1) > 0,
            mr19: ((value >> 19) & 0b1) > 0,
            mr20: ((value >> 20) & 0b1) > 0,
            mr21: ((value >> 21) & 0b1) > 0,
            mr22: ((value >> 22) & 0b1) > 0,
            mr23: ((value >> 23) & 0b1) > 0,
            mr24: ((value >> 24) & 0b1) > 0,
            mr25: ((value >> 25) & 0b1) > 0,
            mr26: ((value >> 26) & 0b1) > 0,
            mr27: ((value >> 27) & 0b1) > 0,
            mr28: ((value >> 28) & 0b1) > 0,
            mr29: ((value >> 29) & 0b1) > 0,
            mr30: ((value >> 30) & 0b1) > 0,
            mr31: ((value >> 31) & 0b1) > 0,
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
                | ((self.mr0 as u32) << 0)
                | ((self.mr1 as u32) << 1)
                | ((self.mr2 as u32) << 2)
                | ((self.mr3 as u32) << 3)
                | ((self.mr4 as u32) << 4)
                | ((self.mr5 as u32) << 5)
                | ((self.mr6 as u32) << 6)
                | ((self.mr7 as u32) << 7)
                | ((self.mr8 as u32) << 8)
                | ((self.mr9 as u32) << 9)
                | ((self.mr10 as u32) << 10)
                | ((self.mr11 as u32) << 11)
                | ((self.mr12 as u32) << 12)
                | ((self.mr13 as u32) << 13)
                | ((self.mr14 as u32) << 14)
                | ((self.mr15 as u32) << 15)
                | ((self.mr16 as u32) << 16)
                | ((self.mr17 as u32) << 17)
                | ((self.mr18 as u32) << 18)
                | ((self.mr19 as u32) << 19)
                | ((self.mr20 as u32) << 20)
                | ((self.mr21 as u32) << 21)
                | ((self.mr22 as u32) << 22)
                | ((self.mr23 as u32) << 23)
                | ((self.mr24 as u32) << 24)
                | ((self.mr25 as u32) << 25)
                | ((self.mr26 as u32) << 26)
                | ((self.mr27 as u32) << 27)
                | ((self.mr28 as u32) << 28)
                | ((self.mr29 as u32) << 29)
                | ((self.mr30 as u32) << 30)
                | ((self.mr31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010400u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// Event mask register
pub mod emr1 {
    pub struct ReadonlyCache {
        /// Event Mask on line 0
        pub mr0: bool,
        /// Event Mask on line 1
        pub mr1: bool,
        /// Event Mask on line 2
        pub mr2: bool,
        /// Event Mask on line 3
        pub mr3: bool,
        /// Event Mask on line 4
        pub mr4: bool,
        /// Event Mask on line 5
        pub mr5: bool,
        /// Event Mask on line 6
        pub mr6: bool,
        /// Event Mask on line 7
        pub mr7: bool,
        /// Event Mask on line 8
        pub mr8: bool,
        /// Event Mask on line 9
        pub mr9: bool,
        /// Event Mask on line 10
        pub mr10: bool,
        /// Event Mask on line 11
        pub mr11: bool,
        /// Event Mask on line 12
        pub mr12: bool,
        /// Event Mask on line 13
        pub mr13: bool,
        /// Event Mask on line 14
        pub mr14: bool,
        /// Event Mask on line 15
        pub mr15: bool,
        /// Event Mask on line 16
        pub mr16: bool,
        /// Event Mask on line 17
        pub mr17: bool,
        /// Event Mask on line 18
        pub mr18: bool,
        /// Event Mask on line 19
        pub mr19: bool,
        /// Event Mask on line 20
        pub mr20: bool,
        /// Event Mask on line 21
        pub mr21: bool,
        /// Event Mask on line 22
        pub mr22: bool,
        /// Event Mask on line 23
        pub mr23: bool,
        /// Event Mask on line 24
        pub mr24: bool,
        /// Event Mask on line 25
        pub mr25: bool,
        /// Event Mask on line 26
        pub mr26: bool,
        /// Event Mask on line 27
        pub mr27: bool,
        /// Event Mask on line 28
        pub mr28: bool,
        /// Event Mask on line 29
        pub mr29: bool,
        /// Event Mask on line 30
        pub mr30: bool,
        /// Event Mask on line 31
        pub mr31: bool,
    }
    pub struct Cache {
        /// Event Mask on line 0
        pub mr0: bool,
        /// Event Mask on line 1
        pub mr1: bool,
        /// Event Mask on line 2
        pub mr2: bool,
        /// Event Mask on line 3
        pub mr3: bool,
        /// Event Mask on line 4
        pub mr4: bool,
        /// Event Mask on line 5
        pub mr5: bool,
        /// Event Mask on line 6
        pub mr6: bool,
        /// Event Mask on line 7
        pub mr7: bool,
        /// Event Mask on line 8
        pub mr8: bool,
        /// Event Mask on line 9
        pub mr9: bool,
        /// Event Mask on line 10
        pub mr10: bool,
        /// Event Mask on line 11
        pub mr11: bool,
        /// Event Mask on line 12
        pub mr12: bool,
        /// Event Mask on line 13
        pub mr13: bool,
        /// Event Mask on line 14
        pub mr14: bool,
        /// Event Mask on line 15
        pub mr15: bool,
        /// Event Mask on line 16
        pub mr16: bool,
        /// Event Mask on line 17
        pub mr17: bool,
        /// Event Mask on line 18
        pub mr18: bool,
        /// Event Mask on line 19
        pub mr19: bool,
        /// Event Mask on line 20
        pub mr20: bool,
        /// Event Mask on line 21
        pub mr21: bool,
        /// Event Mask on line 22
        pub mr22: bool,
        /// Event Mask on line 23
        pub mr23: bool,
        /// Event Mask on line 24
        pub mr24: bool,
        /// Event Mask on line 25
        pub mr25: bool,
        /// Event Mask on line 26
        pub mr26: bool,
        /// Event Mask on line 27
        pub mr27: bool,
        /// Event Mask on line 28
        pub mr28: bool,
        /// Event Mask on line 29
        pub mr29: bool,
        /// Event Mask on line 30
        pub mr30: bool,
        /// Event Mask on line 31
        pub mr31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            mr0: ((value >> 0) & 0b1) > 0,
            mr1: ((value >> 1) & 0b1) > 0,
            mr2: ((value >> 2) & 0b1) > 0,
            mr3: ((value >> 3) & 0b1) > 0,
            mr4: ((value >> 4) & 0b1) > 0,
            mr5: ((value >> 5) & 0b1) > 0,
            mr6: ((value >> 6) & 0b1) > 0,
            mr7: ((value >> 7) & 0b1) > 0,
            mr8: ((value >> 8) & 0b1) > 0,
            mr9: ((value >> 9) & 0b1) > 0,
            mr10: ((value >> 10) & 0b1) > 0,
            mr11: ((value >> 11) & 0b1) > 0,
            mr12: ((value >> 12) & 0b1) > 0,
            mr13: ((value >> 13) & 0b1) > 0,
            mr14: ((value >> 14) & 0b1) > 0,
            mr15: ((value >> 15) & 0b1) > 0,
            mr16: ((value >> 16) & 0b1) > 0,
            mr17: ((value >> 17) & 0b1) > 0,
            mr18: ((value >> 18) & 0b1) > 0,
            mr19: ((value >> 19) & 0b1) > 0,
            mr20: ((value >> 20) & 0b1) > 0,
            mr21: ((value >> 21) & 0b1) > 0,
            mr22: ((value >> 22) & 0b1) > 0,
            mr23: ((value >> 23) & 0b1) > 0,
            mr24: ((value >> 24) & 0b1) > 0,
            mr25: ((value >> 25) & 0b1) > 0,
            mr26: ((value >> 26) & 0b1) > 0,
            mr27: ((value >> 27) & 0b1) > 0,
            mr28: ((value >> 28) & 0b1) > 0,
            mr29: ((value >> 29) & 0b1) > 0,
            mr30: ((value >> 30) & 0b1) > 0,
            mr31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x4u32) as *mut u32) };
        Cache {
            mr0: ((value >> 0) & 0b1) > 0,
            mr1: ((value >> 1) & 0b1) > 0,
            mr2: ((value >> 2) & 0b1) > 0,
            mr3: ((value >> 3) & 0b1) > 0,
            mr4: ((value >> 4) & 0b1) > 0,
            mr5: ((value >> 5) & 0b1) > 0,
            mr6: ((value >> 6) & 0b1) > 0,
            mr7: ((value >> 7) & 0b1) > 0,
            mr8: ((value >> 8) & 0b1) > 0,
            mr9: ((value >> 9) & 0b1) > 0,
            mr10: ((value >> 10) & 0b1) > 0,
            mr11: ((value >> 11) & 0b1) > 0,
            mr12: ((value >> 12) & 0b1) > 0,
            mr13: ((value >> 13) & 0b1) > 0,
            mr14: ((value >> 14) & 0b1) > 0,
            mr15: ((value >> 15) & 0b1) > 0,
            mr16: ((value >> 16) & 0b1) > 0,
            mr17: ((value >> 17) & 0b1) > 0,
            mr18: ((value >> 18) & 0b1) > 0,
            mr19: ((value >> 19) & 0b1) > 0,
            mr20: ((value >> 20) & 0b1) > 0,
            mr21: ((value >> 21) & 0b1) > 0,
            mr22: ((value >> 22) & 0b1) > 0,
            mr23: ((value >> 23) & 0b1) > 0,
            mr24: ((value >> 24) & 0b1) > 0,
            mr25: ((value >> 25) & 0b1) > 0,
            mr26: ((value >> 26) & 0b1) > 0,
            mr27: ((value >> 27) & 0b1) > 0,
            mr28: ((value >> 28) & 0b1) > 0,
            mr29: ((value >> 29) & 0b1) > 0,
            mr30: ((value >> 30) & 0b1) > 0,
            mr31: ((value >> 31) & 0b1) > 0,
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
                | ((self.mr0 as u32) << 0)
                | ((self.mr1 as u32) << 1)
                | ((self.mr2 as u32) << 2)
                | ((self.mr3 as u32) << 3)
                | ((self.mr4 as u32) << 4)
                | ((self.mr5 as u32) << 5)
                | ((self.mr6 as u32) << 6)
                | ((self.mr7 as u32) << 7)
                | ((self.mr8 as u32) << 8)
                | ((self.mr9 as u32) << 9)
                | ((self.mr10 as u32) << 10)
                | ((self.mr11 as u32) << 11)
                | ((self.mr12 as u32) << 12)
                | ((self.mr13 as u32) << 13)
                | ((self.mr14 as u32) << 14)
                | ((self.mr15 as u32) << 15)
                | ((self.mr16 as u32) << 16)
                | ((self.mr17 as u32) << 17)
                | ((self.mr18 as u32) << 18)
                | ((self.mr19 as u32) << 19)
                | ((self.mr20 as u32) << 20)
                | ((self.mr21 as u32) << 21)
                | ((self.mr22 as u32) << 22)
                | ((self.mr23 as u32) << 23)
                | ((self.mr24 as u32) << 24)
                | ((self.mr25 as u32) << 25)
                | ((self.mr26 as u32) << 26)
                | ((self.mr27 as u32) << 27)
                | ((self.mr28 as u32) << 28)
                | ((self.mr29 as u32) << 29)
                | ((self.mr30 as u32) << 30)
                | ((self.mr31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010400u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// Rising Trigger selection register
pub mod rtsr1 {
    pub struct ReadonlyCache {
        /// Rising trigger event configuration of line 0
        pub tr0: bool,
        /// Rising trigger event configuration of line 1
        pub tr1: bool,
        /// Rising trigger event configuration of line 2
        pub tr2: bool,
        /// Rising trigger event configuration of line 3
        pub tr3: bool,
        /// Rising trigger event configuration of line 4
        pub tr4: bool,
        /// Rising trigger event configuration of line 5
        pub tr5: bool,
        /// Rising trigger event configuration of line 6
        pub tr6: bool,
        /// Rising trigger event configuration of line 7
        pub tr7: bool,
        /// Rising trigger event configuration of line 8
        pub tr8: bool,
        /// Rising trigger event configuration of line 9
        pub tr9: bool,
        /// Rising trigger event configuration of line 10
        pub tr10: bool,
        /// Rising trigger event configuration of line 11
        pub tr11: bool,
        /// Rising trigger event configuration of line 12
        pub tr12: bool,
        /// Rising trigger event configuration of line 13
        pub tr13: bool,
        /// Rising trigger event configuration of line 14
        pub tr14: bool,
        /// Rising trigger event configuration of line 15
        pub tr15: bool,
        /// Rising trigger event configuration of line 16
        pub tr16: bool,
        /// Rising trigger event configuration of line 17
        pub tr17: bool,
        /// Rising trigger event configuration of line 18
        pub tr18: bool,
        /// Rising trigger event configuration of line 19
        pub tr19: bool,
        /// Rising trigger event configuration of line 20
        pub tr20: bool,
        /// Rising trigger event configuration of line 21
        pub tr21: bool,
        /// Rising trigger event configuration of line 22
        pub tr22: bool,
        /// Rising trigger event configuration of line 29
        pub tr29: bool,
        /// Rising trigger event configuration of line 30
        pub tr30: bool,
        /// Rising trigger event configuration of line 31
        pub tr31: bool,
    }
    pub struct Cache {
        /// Rising trigger event configuration of line 0
        pub tr0: bool,
        /// Rising trigger event configuration of line 1
        pub tr1: bool,
        /// Rising trigger event configuration of line 2
        pub tr2: bool,
        /// Rising trigger event configuration of line 3
        pub tr3: bool,
        /// Rising trigger event configuration of line 4
        pub tr4: bool,
        /// Rising trigger event configuration of line 5
        pub tr5: bool,
        /// Rising trigger event configuration of line 6
        pub tr6: bool,
        /// Rising trigger event configuration of line 7
        pub tr7: bool,
        /// Rising trigger event configuration of line 8
        pub tr8: bool,
        /// Rising trigger event configuration of line 9
        pub tr9: bool,
        /// Rising trigger event configuration of line 10
        pub tr10: bool,
        /// Rising trigger event configuration of line 11
        pub tr11: bool,
        /// Rising trigger event configuration of line 12
        pub tr12: bool,
        /// Rising trigger event configuration of line 13
        pub tr13: bool,
        /// Rising trigger event configuration of line 14
        pub tr14: bool,
        /// Rising trigger event configuration of line 15
        pub tr15: bool,
        /// Rising trigger event configuration of line 16
        pub tr16: bool,
        /// Rising trigger event configuration of line 17
        pub tr17: bool,
        /// Rising trigger event configuration of line 18
        pub tr18: bool,
        /// Rising trigger event configuration of line 19
        pub tr19: bool,
        /// Rising trigger event configuration of line 20
        pub tr20: bool,
        /// Rising trigger event configuration of line 21
        pub tr21: bool,
        /// Rising trigger event configuration of line 22
        pub tr22: bool,
        /// Rising trigger event configuration of line 29
        pub tr29: bool,
        /// Rising trigger event configuration of line 30
        pub tr30: bool,
        /// Rising trigger event configuration of line 31
        pub tr31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            tr0: ((value >> 0) & 0b1) > 0,
            tr1: ((value >> 1) & 0b1) > 0,
            tr2: ((value >> 2) & 0b1) > 0,
            tr3: ((value >> 3) & 0b1) > 0,
            tr4: ((value >> 4) & 0b1) > 0,
            tr5: ((value >> 5) & 0b1) > 0,
            tr6: ((value >> 6) & 0b1) > 0,
            tr7: ((value >> 7) & 0b1) > 0,
            tr8: ((value >> 8) & 0b1) > 0,
            tr9: ((value >> 9) & 0b1) > 0,
            tr10: ((value >> 10) & 0b1) > 0,
            tr11: ((value >> 11) & 0b1) > 0,
            tr12: ((value >> 12) & 0b1) > 0,
            tr13: ((value >> 13) & 0b1) > 0,
            tr14: ((value >> 14) & 0b1) > 0,
            tr15: ((value >> 15) & 0b1) > 0,
            tr16: ((value >> 16) & 0b1) > 0,
            tr17: ((value >> 17) & 0b1) > 0,
            tr18: ((value >> 18) & 0b1) > 0,
            tr19: ((value >> 19) & 0b1) > 0,
            tr20: ((value >> 20) & 0b1) > 0,
            tr21: ((value >> 21) & 0b1) > 0,
            tr22: ((value >> 22) & 0b1) > 0,
            tr29: ((value >> 29) & 0b1) > 0,
            tr30: ((value >> 30) & 0b1) > 0,
            tr31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x8u32) as *mut u32) };
        Cache {
            tr0: ((value >> 0) & 0b1) > 0,
            tr1: ((value >> 1) & 0b1) > 0,
            tr2: ((value >> 2) & 0b1) > 0,
            tr3: ((value >> 3) & 0b1) > 0,
            tr4: ((value >> 4) & 0b1) > 0,
            tr5: ((value >> 5) & 0b1) > 0,
            tr6: ((value >> 6) & 0b1) > 0,
            tr7: ((value >> 7) & 0b1) > 0,
            tr8: ((value >> 8) & 0b1) > 0,
            tr9: ((value >> 9) & 0b1) > 0,
            tr10: ((value >> 10) & 0b1) > 0,
            tr11: ((value >> 11) & 0b1) > 0,
            tr12: ((value >> 12) & 0b1) > 0,
            tr13: ((value >> 13) & 0b1) > 0,
            tr14: ((value >> 14) & 0b1) > 0,
            tr15: ((value >> 15) & 0b1) > 0,
            tr16: ((value >> 16) & 0b1) > 0,
            tr17: ((value >> 17) & 0b1) > 0,
            tr18: ((value >> 18) & 0b1) > 0,
            tr19: ((value >> 19) & 0b1) > 0,
            tr20: ((value >> 20) & 0b1) > 0,
            tr21: ((value >> 21) & 0b1) > 0,
            tr22: ((value >> 22) & 0b1) > 0,
            tr29: ((value >> 29) & 0b1) > 0,
            tr30: ((value >> 30) & 0b1) > 0,
            tr31: ((value >> 31) & 0b1) > 0,
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
                | ((self.tr0 as u32) << 0)
                | ((self.tr1 as u32) << 1)
                | ((self.tr2 as u32) << 2)
                | ((self.tr3 as u32) << 3)
                | ((self.tr4 as u32) << 4)
                | ((self.tr5 as u32) << 5)
                | ((self.tr6 as u32) << 6)
                | ((self.tr7 as u32) << 7)
                | ((self.tr8 as u32) << 8)
                | ((self.tr9 as u32) << 9)
                | ((self.tr10 as u32) << 10)
                | ((self.tr11 as u32) << 11)
                | ((self.tr12 as u32) << 12)
                | ((self.tr13 as u32) << 13)
                | ((self.tr14 as u32) << 14)
                | ((self.tr15 as u32) << 15)
                | ((self.tr16 as u32) << 16)
                | ((self.tr17 as u32) << 17)
                | ((self.tr18 as u32) << 18)
                | ((self.tr19 as u32) << 19)
                | ((self.tr20 as u32) << 20)
                | ((self.tr21 as u32) << 21)
                | ((self.tr22 as u32) << 22)
                | ((self.tr29 as u32) << 29)
                | ((self.tr30 as u32) << 30)
                | ((self.tr31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010400u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// Falling Trigger selection register
pub mod ftsr1 {
    pub struct ReadonlyCache {
        /// Falling trigger event configuration of line 0
        pub tr0: bool,
        /// Falling trigger event configuration of line 1
        pub tr1: bool,
        /// Falling trigger event configuration of line 2
        pub tr2: bool,
        /// Falling trigger event configuration of line 3
        pub tr3: bool,
        /// Falling trigger event configuration of line 4
        pub tr4: bool,
        /// Falling trigger event configuration of line 5
        pub tr5: bool,
        /// Falling trigger event configuration of line 6
        pub tr6: bool,
        /// Falling trigger event configuration of line 7
        pub tr7: bool,
        /// Falling trigger event configuration of line 8
        pub tr8: bool,
        /// Falling trigger event configuration of line 9
        pub tr9: bool,
        /// Falling trigger event configuration of line 10
        pub tr10: bool,
        /// Falling trigger event configuration of line 11
        pub tr11: bool,
        /// Falling trigger event configuration of line 12
        pub tr12: bool,
        /// Falling trigger event configuration of line 13
        pub tr13: bool,
        /// Falling trigger event configuration of line 14
        pub tr14: bool,
        /// Falling trigger event configuration of line 15
        pub tr15: bool,
        /// Falling trigger event configuration of line 16
        pub tr16: bool,
        /// Falling trigger event configuration of line 17
        pub tr17: bool,
        /// Falling trigger event configuration of line 18
        pub tr18: bool,
        /// Falling trigger event configuration of line 19
        pub tr19: bool,
        /// Falling trigger event configuration of line 20
        pub tr20: bool,
        /// Falling trigger event configuration of line 21
        pub tr21: bool,
        /// Falling trigger event configuration of line 22
        pub tr22: bool,
        /// Falling trigger event configuration of line 29
        pub tr29: bool,
        /// Falling trigger event configuration of line 30.
        pub tr30: bool,
        /// Falling trigger event configuration of line 31
        pub tr31: bool,
    }
    pub struct Cache {
        /// Falling trigger event configuration of line 0
        pub tr0: bool,
        /// Falling trigger event configuration of line 1
        pub tr1: bool,
        /// Falling trigger event configuration of line 2
        pub tr2: bool,
        /// Falling trigger event configuration of line 3
        pub tr3: bool,
        /// Falling trigger event configuration of line 4
        pub tr4: bool,
        /// Falling trigger event configuration of line 5
        pub tr5: bool,
        /// Falling trigger event configuration of line 6
        pub tr6: bool,
        /// Falling trigger event configuration of line 7
        pub tr7: bool,
        /// Falling trigger event configuration of line 8
        pub tr8: bool,
        /// Falling trigger event configuration of line 9
        pub tr9: bool,
        /// Falling trigger event configuration of line 10
        pub tr10: bool,
        /// Falling trigger event configuration of line 11
        pub tr11: bool,
        /// Falling trigger event configuration of line 12
        pub tr12: bool,
        /// Falling trigger event configuration of line 13
        pub tr13: bool,
        /// Falling trigger event configuration of line 14
        pub tr14: bool,
        /// Falling trigger event configuration of line 15
        pub tr15: bool,
        /// Falling trigger event configuration of line 16
        pub tr16: bool,
        /// Falling trigger event configuration of line 17
        pub tr17: bool,
        /// Falling trigger event configuration of line 18
        pub tr18: bool,
        /// Falling trigger event configuration of line 19
        pub tr19: bool,
        /// Falling trigger event configuration of line 20
        pub tr20: bool,
        /// Falling trigger event configuration of line 21
        pub tr21: bool,
        /// Falling trigger event configuration of line 22
        pub tr22: bool,
        /// Falling trigger event configuration of line 29
        pub tr29: bool,
        /// Falling trigger event configuration of line 30.
        pub tr30: bool,
        /// Falling trigger event configuration of line 31
        pub tr31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            tr0: ((value >> 0) & 0b1) > 0,
            tr1: ((value >> 1) & 0b1) > 0,
            tr2: ((value >> 2) & 0b1) > 0,
            tr3: ((value >> 3) & 0b1) > 0,
            tr4: ((value >> 4) & 0b1) > 0,
            tr5: ((value >> 5) & 0b1) > 0,
            tr6: ((value >> 6) & 0b1) > 0,
            tr7: ((value >> 7) & 0b1) > 0,
            tr8: ((value >> 8) & 0b1) > 0,
            tr9: ((value >> 9) & 0b1) > 0,
            tr10: ((value >> 10) & 0b1) > 0,
            tr11: ((value >> 11) & 0b1) > 0,
            tr12: ((value >> 12) & 0b1) > 0,
            tr13: ((value >> 13) & 0b1) > 0,
            tr14: ((value >> 14) & 0b1) > 0,
            tr15: ((value >> 15) & 0b1) > 0,
            tr16: ((value >> 16) & 0b1) > 0,
            tr17: ((value >> 17) & 0b1) > 0,
            tr18: ((value >> 18) & 0b1) > 0,
            tr19: ((value >> 19) & 0b1) > 0,
            tr20: ((value >> 20) & 0b1) > 0,
            tr21: ((value >> 21) & 0b1) > 0,
            tr22: ((value >> 22) & 0b1) > 0,
            tr29: ((value >> 29) & 0b1) > 0,
            tr30: ((value >> 30) & 0b1) > 0,
            tr31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0xCu32) as *mut u32) };
        Cache {
            tr0: ((value >> 0) & 0b1) > 0,
            tr1: ((value >> 1) & 0b1) > 0,
            tr2: ((value >> 2) & 0b1) > 0,
            tr3: ((value >> 3) & 0b1) > 0,
            tr4: ((value >> 4) & 0b1) > 0,
            tr5: ((value >> 5) & 0b1) > 0,
            tr6: ((value >> 6) & 0b1) > 0,
            tr7: ((value >> 7) & 0b1) > 0,
            tr8: ((value >> 8) & 0b1) > 0,
            tr9: ((value >> 9) & 0b1) > 0,
            tr10: ((value >> 10) & 0b1) > 0,
            tr11: ((value >> 11) & 0b1) > 0,
            tr12: ((value >> 12) & 0b1) > 0,
            tr13: ((value >> 13) & 0b1) > 0,
            tr14: ((value >> 14) & 0b1) > 0,
            tr15: ((value >> 15) & 0b1) > 0,
            tr16: ((value >> 16) & 0b1) > 0,
            tr17: ((value >> 17) & 0b1) > 0,
            tr18: ((value >> 18) & 0b1) > 0,
            tr19: ((value >> 19) & 0b1) > 0,
            tr20: ((value >> 20) & 0b1) > 0,
            tr21: ((value >> 21) & 0b1) > 0,
            tr22: ((value >> 22) & 0b1) > 0,
            tr29: ((value >> 29) & 0b1) > 0,
            tr30: ((value >> 30) & 0b1) > 0,
            tr31: ((value >> 31) & 0b1) > 0,
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
                | ((self.tr0 as u32) << 0)
                | ((self.tr1 as u32) << 1)
                | ((self.tr2 as u32) << 2)
                | ((self.tr3 as u32) << 3)
                | ((self.tr4 as u32) << 4)
                | ((self.tr5 as u32) << 5)
                | ((self.tr6 as u32) << 6)
                | ((self.tr7 as u32) << 7)
                | ((self.tr8 as u32) << 8)
                | ((self.tr9 as u32) << 9)
                | ((self.tr10 as u32) << 10)
                | ((self.tr11 as u32) << 11)
                | ((self.tr12 as u32) << 12)
                | ((self.tr13 as u32) << 13)
                | ((self.tr14 as u32) << 14)
                | ((self.tr15 as u32) << 15)
                | ((self.tr16 as u32) << 16)
                | ((self.tr17 as u32) << 17)
                | ((self.tr18 as u32) << 18)
                | ((self.tr19 as u32) << 19)
                | ((self.tr20 as u32) << 20)
                | ((self.tr21 as u32) << 21)
                | ((self.tr22 as u32) << 22)
                | ((self.tr29 as u32) << 29)
                | ((self.tr30 as u32) << 30)
                | ((self.tr31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010400u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// Software interrupt event register
pub mod swier1 {
    pub struct ReadonlyCache {
        /// Software Interrupt on line 0
        pub swier0: bool,
        /// Software Interrupt on line 1
        pub swier1: bool,
        /// Software Interrupt on line 2
        pub swier2: bool,
        /// Software Interrupt on line 3
        pub swier3: bool,
        /// Software Interrupt on line 4
        pub swier4: bool,
        /// Software Interrupt on line 5
        pub swier5: bool,
        /// Software Interrupt on line 6
        pub swier6: bool,
        /// Software Interrupt on line 7
        pub swier7: bool,
        /// Software Interrupt on line 8
        pub swier8: bool,
        /// Software Interrupt on line 9
        pub swier9: bool,
        /// Software Interrupt on line 10
        pub swier10: bool,
        /// Software Interrupt on line 11
        pub swier11: bool,
        /// Software Interrupt on line 12
        pub swier12: bool,
        /// Software Interrupt on line 13
        pub swier13: bool,
        /// Software Interrupt on line 14
        pub swier14: bool,
        /// Software Interrupt on line 15
        pub swier15: bool,
        /// Software Interrupt on line 16
        pub swier16: bool,
        /// Software Interrupt on line 17
        pub swier17: bool,
        /// Software Interrupt on line 18
        pub swier18: bool,
        /// Software Interrupt on line 19
        pub swier19: bool,
        /// Software Interrupt on line 20
        pub swier20: bool,
        /// Software Interrupt on line 21
        pub swier21: bool,
        /// Software Interrupt on line 22
        pub swier22: bool,
        /// Software Interrupt on line 29
        pub swier29: bool,
        /// Software Interrupt on line 309
        pub swier30: bool,
        /// Software Interrupt on line 319
        pub swier31: bool,
    }
    pub struct Cache {
        /// Software Interrupt on line 0
        pub swier0: bool,
        /// Software Interrupt on line 1
        pub swier1: bool,
        /// Software Interrupt on line 2
        pub swier2: bool,
        /// Software Interrupt on line 3
        pub swier3: bool,
        /// Software Interrupt on line 4
        pub swier4: bool,
        /// Software Interrupt on line 5
        pub swier5: bool,
        /// Software Interrupt on line 6
        pub swier6: bool,
        /// Software Interrupt on line 7
        pub swier7: bool,
        /// Software Interrupt on line 8
        pub swier8: bool,
        /// Software Interrupt on line 9
        pub swier9: bool,
        /// Software Interrupt on line 10
        pub swier10: bool,
        /// Software Interrupt on line 11
        pub swier11: bool,
        /// Software Interrupt on line 12
        pub swier12: bool,
        /// Software Interrupt on line 13
        pub swier13: bool,
        /// Software Interrupt on line 14
        pub swier14: bool,
        /// Software Interrupt on line 15
        pub swier15: bool,
        /// Software Interrupt on line 16
        pub swier16: bool,
        /// Software Interrupt on line 17
        pub swier17: bool,
        /// Software Interrupt on line 18
        pub swier18: bool,
        /// Software Interrupt on line 19
        pub swier19: bool,
        /// Software Interrupt on line 20
        pub swier20: bool,
        /// Software Interrupt on line 21
        pub swier21: bool,
        /// Software Interrupt on line 22
        pub swier22: bool,
        /// Software Interrupt on line 29
        pub swier29: bool,
        /// Software Interrupt on line 309
        pub swier30: bool,
        /// Software Interrupt on line 319
        pub swier31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            swier0: ((value >> 0) & 0b1) > 0,
            swier1: ((value >> 1) & 0b1) > 0,
            swier2: ((value >> 2) & 0b1) > 0,
            swier3: ((value >> 3) & 0b1) > 0,
            swier4: ((value >> 4) & 0b1) > 0,
            swier5: ((value >> 5) & 0b1) > 0,
            swier6: ((value >> 6) & 0b1) > 0,
            swier7: ((value >> 7) & 0b1) > 0,
            swier8: ((value >> 8) & 0b1) > 0,
            swier9: ((value >> 9) & 0b1) > 0,
            swier10: ((value >> 10) & 0b1) > 0,
            swier11: ((value >> 11) & 0b1) > 0,
            swier12: ((value >> 12) & 0b1) > 0,
            swier13: ((value >> 13) & 0b1) > 0,
            swier14: ((value >> 14) & 0b1) > 0,
            swier15: ((value >> 15) & 0b1) > 0,
            swier16: ((value >> 16) & 0b1) > 0,
            swier17: ((value >> 17) & 0b1) > 0,
            swier18: ((value >> 18) & 0b1) > 0,
            swier19: ((value >> 19) & 0b1) > 0,
            swier20: ((value >> 20) & 0b1) > 0,
            swier21: ((value >> 21) & 0b1) > 0,
            swier22: ((value >> 22) & 0b1) > 0,
            swier29: ((value >> 29) & 0b1) > 0,
            swier30: ((value >> 30) & 0b1) > 0,
            swier31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x10u32) as *mut u32) };
        Cache {
            swier0: ((value >> 0) & 0b1) > 0,
            swier1: ((value >> 1) & 0b1) > 0,
            swier2: ((value >> 2) & 0b1) > 0,
            swier3: ((value >> 3) & 0b1) > 0,
            swier4: ((value >> 4) & 0b1) > 0,
            swier5: ((value >> 5) & 0b1) > 0,
            swier6: ((value >> 6) & 0b1) > 0,
            swier7: ((value >> 7) & 0b1) > 0,
            swier8: ((value >> 8) & 0b1) > 0,
            swier9: ((value >> 9) & 0b1) > 0,
            swier10: ((value >> 10) & 0b1) > 0,
            swier11: ((value >> 11) & 0b1) > 0,
            swier12: ((value >> 12) & 0b1) > 0,
            swier13: ((value >> 13) & 0b1) > 0,
            swier14: ((value >> 14) & 0b1) > 0,
            swier15: ((value >> 15) & 0b1) > 0,
            swier16: ((value >> 16) & 0b1) > 0,
            swier17: ((value >> 17) & 0b1) > 0,
            swier18: ((value >> 18) & 0b1) > 0,
            swier19: ((value >> 19) & 0b1) > 0,
            swier20: ((value >> 20) & 0b1) > 0,
            swier21: ((value >> 21) & 0b1) > 0,
            swier22: ((value >> 22) & 0b1) > 0,
            swier29: ((value >> 29) & 0b1) > 0,
            swier30: ((value >> 30) & 0b1) > 0,
            swier31: ((value >> 31) & 0b1) > 0,
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
                | ((self.swier0 as u32) << 0)
                | ((self.swier1 as u32) << 1)
                | ((self.swier2 as u32) << 2)
                | ((self.swier3 as u32) << 3)
                | ((self.swier4 as u32) << 4)
                | ((self.swier5 as u32) << 5)
                | ((self.swier6 as u32) << 6)
                | ((self.swier7 as u32) << 7)
                | ((self.swier8 as u32) << 8)
                | ((self.swier9 as u32) << 9)
                | ((self.swier10 as u32) << 10)
                | ((self.swier11 as u32) << 11)
                | ((self.swier12 as u32) << 12)
                | ((self.swier13 as u32) << 13)
                | ((self.swier14 as u32) << 14)
                | ((self.swier15 as u32) << 15)
                | ((self.swier16 as u32) << 16)
                | ((self.swier17 as u32) << 17)
                | ((self.swier18 as u32) << 18)
                | ((self.swier19 as u32) << 19)
                | ((self.swier20 as u32) << 20)
                | ((self.swier21 as u32) << 21)
                | ((self.swier22 as u32) << 22)
                | ((self.swier29 as u32) << 29)
                | ((self.swier30 as u32) << 30)
                | ((self.swier31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010400u32 + 0x10u32) as *mut u32, value) };
        }
    }
}
/// Pending register
pub mod pr1 {
    pub struct ReadonlyCache {
        /// Pending bit 0
        pub pr0: bool,
        /// Pending bit 1
        pub pr1: bool,
        /// Pending bit 2
        pub pr2: bool,
        /// Pending bit 3
        pub pr3: bool,
        /// Pending bit 4
        pub pr4: bool,
        /// Pending bit 5
        pub pr5: bool,
        /// Pending bit 6
        pub pr6: bool,
        /// Pending bit 7
        pub pr7: bool,
        /// Pending bit 8
        pub pr8: bool,
        /// Pending bit 9
        pub pr9: bool,
        /// Pending bit 10
        pub pr10: bool,
        /// Pending bit 11
        pub pr11: bool,
        /// Pending bit 12
        pub pr12: bool,
        /// Pending bit 13
        pub pr13: bool,
        /// Pending bit 14
        pub pr14: bool,
        /// Pending bit 15
        pub pr15: bool,
        /// Pending bit 16
        pub pr16: bool,
        /// Pending bit 17
        pub pr17: bool,
        /// Pending bit 18
        pub pr18: bool,
        /// Pending bit 19
        pub pr19: bool,
        /// Pending bit 20
        pub pr20: bool,
        /// Pending bit 21
        pub pr21: bool,
        /// Pending bit 22
        pub pr22: bool,
        /// Pending bit 29
        pub pr29: bool,
        /// Pending bit 30
        pub pr30: bool,
        /// Pending bit 31
        pub pr31: bool,
    }
    pub struct Cache {
        /// Pending bit 0
        pub pr0: bool,
        /// Pending bit 1
        pub pr1: bool,
        /// Pending bit 2
        pub pr2: bool,
        /// Pending bit 3
        pub pr3: bool,
        /// Pending bit 4
        pub pr4: bool,
        /// Pending bit 5
        pub pr5: bool,
        /// Pending bit 6
        pub pr6: bool,
        /// Pending bit 7
        pub pr7: bool,
        /// Pending bit 8
        pub pr8: bool,
        /// Pending bit 9
        pub pr9: bool,
        /// Pending bit 10
        pub pr10: bool,
        /// Pending bit 11
        pub pr11: bool,
        /// Pending bit 12
        pub pr12: bool,
        /// Pending bit 13
        pub pr13: bool,
        /// Pending bit 14
        pub pr14: bool,
        /// Pending bit 15
        pub pr15: bool,
        /// Pending bit 16
        pub pr16: bool,
        /// Pending bit 17
        pub pr17: bool,
        /// Pending bit 18
        pub pr18: bool,
        /// Pending bit 19
        pub pr19: bool,
        /// Pending bit 20
        pub pr20: bool,
        /// Pending bit 21
        pub pr21: bool,
        /// Pending bit 22
        pub pr22: bool,
        /// Pending bit 29
        pub pr29: bool,
        /// Pending bit 30
        pub pr30: bool,
        /// Pending bit 31
        pub pr31: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x14u32) as *mut u32) };
        ReadonlyCache {
            pr0: ((value >> 0) & 0b1) > 0,
            pr1: ((value >> 1) & 0b1) > 0,
            pr2: ((value >> 2) & 0b1) > 0,
            pr3: ((value >> 3) & 0b1) > 0,
            pr4: ((value >> 4) & 0b1) > 0,
            pr5: ((value >> 5) & 0b1) > 0,
            pr6: ((value >> 6) & 0b1) > 0,
            pr7: ((value >> 7) & 0b1) > 0,
            pr8: ((value >> 8) & 0b1) > 0,
            pr9: ((value >> 9) & 0b1) > 0,
            pr10: ((value >> 10) & 0b1) > 0,
            pr11: ((value >> 11) & 0b1) > 0,
            pr12: ((value >> 12) & 0b1) > 0,
            pr13: ((value >> 13) & 0b1) > 0,
            pr14: ((value >> 14) & 0b1) > 0,
            pr15: ((value >> 15) & 0b1) > 0,
            pr16: ((value >> 16) & 0b1) > 0,
            pr17: ((value >> 17) & 0b1) > 0,
            pr18: ((value >> 18) & 0b1) > 0,
            pr19: ((value >> 19) & 0b1) > 0,
            pr20: ((value >> 20) & 0b1) > 0,
            pr21: ((value >> 21) & 0b1) > 0,
            pr22: ((value >> 22) & 0b1) > 0,
            pr29: ((value >> 29) & 0b1) > 0,
            pr30: ((value >> 30) & 0b1) > 0,
            pr31: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x14u32) as *mut u32) };
        Cache {
            pr0: ((value >> 0) & 0b1) > 0,
            pr1: ((value >> 1) & 0b1) > 0,
            pr2: ((value >> 2) & 0b1) > 0,
            pr3: ((value >> 3) & 0b1) > 0,
            pr4: ((value >> 4) & 0b1) > 0,
            pr5: ((value >> 5) & 0b1) > 0,
            pr6: ((value >> 6) & 0b1) > 0,
            pr7: ((value >> 7) & 0b1) > 0,
            pr8: ((value >> 8) & 0b1) > 0,
            pr9: ((value >> 9) & 0b1) > 0,
            pr10: ((value >> 10) & 0b1) > 0,
            pr11: ((value >> 11) & 0b1) > 0,
            pr12: ((value >> 12) & 0b1) > 0,
            pr13: ((value >> 13) & 0b1) > 0,
            pr14: ((value >> 14) & 0b1) > 0,
            pr15: ((value >> 15) & 0b1) > 0,
            pr16: ((value >> 16) & 0b1) > 0,
            pr17: ((value >> 17) & 0b1) > 0,
            pr18: ((value >> 18) & 0b1) > 0,
            pr19: ((value >> 19) & 0b1) > 0,
            pr20: ((value >> 20) & 0b1) > 0,
            pr21: ((value >> 21) & 0b1) > 0,
            pr22: ((value >> 22) & 0b1) > 0,
            pr29: ((value >> 29) & 0b1) > 0,
            pr30: ((value >> 30) & 0b1) > 0,
            pr31: ((value >> 31) & 0b1) > 0,
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
                | ((self.pr0 as u32) << 0)
                | ((self.pr1 as u32) << 1)
                | ((self.pr2 as u32) << 2)
                | ((self.pr3 as u32) << 3)
                | ((self.pr4 as u32) << 4)
                | ((self.pr5 as u32) << 5)
                | ((self.pr6 as u32) << 6)
                | ((self.pr7 as u32) << 7)
                | ((self.pr8 as u32) << 8)
                | ((self.pr9 as u32) << 9)
                | ((self.pr10 as u32) << 10)
                | ((self.pr11 as u32) << 11)
                | ((self.pr12 as u32) << 12)
                | ((self.pr13 as u32) << 13)
                | ((self.pr14 as u32) << 14)
                | ((self.pr15 as u32) << 15)
                | ((self.pr16 as u32) << 16)
                | ((self.pr17 as u32) << 17)
                | ((self.pr18 as u32) << 18)
                | ((self.pr19 as u32) << 19)
                | ((self.pr20 as u32) << 20)
                | ((self.pr21 as u32) << 21)
                | ((self.pr22 as u32) << 22)
                | ((self.pr29 as u32) << 29)
                | ((self.pr30 as u32) << 30)
                | ((self.pr31 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010400u32 + 0x14u32) as *mut u32, value) };
        }
    }
}
/// Interrupt mask register
pub mod imr2 {
    pub struct ReadonlyCache {
        /// Interrupt Mask on external/internal line 32
        pub mr32: bool,
        /// Interrupt Mask on external/internal line 33
        pub mr33: bool,
        /// Interrupt Mask on external/internal line 34
        pub mr34: bool,
        /// Interrupt Mask on external/internal line 35
        pub mr35: bool,
    }
    pub struct Cache {
        /// Interrupt Mask on external/internal line 32
        pub mr32: bool,
        /// Interrupt Mask on external/internal line 33
        pub mr33: bool,
        /// Interrupt Mask on external/internal line 34
        pub mr34: bool,
        /// Interrupt Mask on external/internal line 35
        pub mr35: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x18u32) as *mut u32) };
        ReadonlyCache {
            mr32: ((value >> 0) & 0b1) > 0,
            mr33: ((value >> 1) & 0b1) > 0,
            mr34: ((value >> 2) & 0b1) > 0,
            mr35: ((value >> 3) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x18u32) as *mut u32) };
        Cache {
            mr32: ((value >> 0) & 0b1) > 0,
            mr33: ((value >> 1) & 0b1) > 0,
            mr34: ((value >> 2) & 0b1) > 0,
            mr35: ((value >> 3) & 0b1) > 0,
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
                | ((self.mr32 as u32) << 0)
                | ((self.mr33 as u32) << 1)
                | ((self.mr34 as u32) << 2)
                | ((self.mr35 as u32) << 3)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010400u32 + 0x18u32) as *mut u32, value) };
        }
    }
}
/// Event mask register
pub mod emr2 {
    pub struct ReadonlyCache {
        /// Event mask on external/internal line 32
        pub mr32: bool,
        /// Event mask on external/internal line 33
        pub mr33: bool,
        /// Event mask on external/internal line 34
        pub mr34: bool,
        /// Event mask on external/internal line 35
        pub mr35: bool,
    }
    pub struct Cache {
        /// Event mask on external/internal line 32
        pub mr32: bool,
        /// Event mask on external/internal line 33
        pub mr33: bool,
        /// Event mask on external/internal line 34
        pub mr34: bool,
        /// Event mask on external/internal line 35
        pub mr35: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x1Cu32) as *mut u32) };
        ReadonlyCache {
            mr32: ((value >> 0) & 0b1) > 0,
            mr33: ((value >> 1) & 0b1) > 0,
            mr34: ((value >> 2) & 0b1) > 0,
            mr35: ((value >> 3) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x1Cu32) as *mut u32) };
        Cache {
            mr32: ((value >> 0) & 0b1) > 0,
            mr33: ((value >> 1) & 0b1) > 0,
            mr34: ((value >> 2) & 0b1) > 0,
            mr35: ((value >> 3) & 0b1) > 0,
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
                | ((self.mr32 as u32) << 0)
                | ((self.mr33 as u32) << 1)
                | ((self.mr34 as u32) << 2)
                | ((self.mr35 as u32) << 3)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010400u32 + 0x1Cu32) as *mut u32, value) };
        }
    }
}
/// Rising Trigger selection register
pub mod rtsr2 {
    pub struct ReadonlyCache {
        /// Rising trigger event configuration bit of line 32
        pub tr32: bool,
        /// Rising trigger event configuration bit of line 33
        pub tr33: bool,
    }
    pub struct Cache {
        /// Rising trigger event configuration bit of line 32
        pub tr32: bool,
        /// Rising trigger event configuration bit of line 33
        pub tr33: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x20u32) as *mut u32) };
        ReadonlyCache {
            tr32: ((value >> 0) & 0b1) > 0,
            tr33: ((value >> 1) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x20u32) as *mut u32) };
        Cache {
            tr32: ((value >> 0) & 0b1) > 0,
            tr33: ((value >> 1) & 0b1) > 0,
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
                | ((self.tr32 as u32) << 0)
                | ((self.tr33 as u32) << 1)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010400u32 + 0x20u32) as *mut u32, value) };
        }
    }
}
/// Falling Trigger selection register
pub mod ftsr2 {
    pub struct ReadonlyCache {
        /// Falling trigger event configuration bit of line 32
        pub tr32: bool,
        /// Falling trigger event configuration bit of line 33
        pub tr33: bool,
    }
    pub struct Cache {
        /// Falling trigger event configuration bit of line 32
        pub tr32: bool,
        /// Falling trigger event configuration bit of line 33
        pub tr33: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x24u32) as *mut u32) };
        ReadonlyCache {
            tr32: ((value >> 0) & 0b1) > 0,
            tr33: ((value >> 1) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x24u32) as *mut u32) };
        Cache {
            tr32: ((value >> 0) & 0b1) > 0,
            tr33: ((value >> 1) & 0b1) > 0,
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
                | ((self.tr32 as u32) << 0)
                | ((self.tr33 as u32) << 1)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010400u32 + 0x24u32) as *mut u32, value) };
        }
    }
}
/// Software interrupt event register
pub mod swier2 {
    pub struct ReadonlyCache {
        /// Software interrupt on line 32
        pub swier32: bool,
        /// Software interrupt on line 33
        pub swier33: bool,
    }
    pub struct Cache {
        /// Software interrupt on line 32
        pub swier32: bool,
        /// Software interrupt on line 33
        pub swier33: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x28u32) as *mut u32) };
        ReadonlyCache {
            swier32: ((value >> 0) & 0b1) > 0,
            swier33: ((value >> 1) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x28u32) as *mut u32) };
        Cache {
            swier32: ((value >> 0) & 0b1) > 0,
            swier33: ((value >> 1) & 0b1) > 0,
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
                | ((self.swier32 as u32) << 0)
                | ((self.swier33 as u32) << 1)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010400u32 + 0x28u32) as *mut u32, value) };
        }
    }
}
/// Pending register
pub mod pr2 {
    pub struct ReadonlyCache {
        /// Pending bit on line 32
        pub pr32: bool,
        /// Pending bit on line 33
        pub pr33: bool,
    }
    pub struct Cache {
        /// Pending bit on line 32
        pub pr32: bool,
        /// Pending bit on line 33
        pub pr33: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x2Cu32) as *mut u32) };
        ReadonlyCache {
            pr32: ((value >> 0) & 0b1) > 0,
            pr33: ((value >> 1) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010400u32 + 0x2Cu32) as *mut u32) };
        Cache {
            pr32: ((value >> 0) & 0b1) > 0,
            pr33: ((value >> 1) & 0b1) > 0,
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
                | ((self.pr32 as u32) << 0)
                | ((self.pr33 as u32) << 1)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010400u32 + 0x2Cu32) as *mut u32, value) };
        }
    }
}
/// Tamper and TimeStamp interrupts
pub const INTERRUPT_TAMP_STAMP: u32 = 2;
/// EXTI Line0 interrupt
pub const INTERRUPT_EXTI0: u32 = 6;
/// EXTI Line3 interrupt
pub const INTERRUPT_EXTI1: u32 = 7;
/// EXTI Line2 and Touch sensing interrupts
pub const INTERRUPT_EXTI2_TSC: u32 = 8;
/// EXTI Line3 interrupt
pub const INTERRUPT_EXTI3: u32 = 9;
/// EXTI Line4 interrupt
pub const INTERRUPT_EXTI4: u32 = 10;
/// EXTI Line5 to Line9 interrupts
pub const INTERRUPT_EXTI9_5: u32 = 23;
/// I2C1 event interrupt and EXTI Line23 interrupt
pub const INTERRUPT_I2C1_EV_EXTI23: u32 = 31;
/// USART1 global interrupt and EXTI Line 25 interrupt
pub const INTERRUPT_USART1_EXTI25: u32 = 37;
/// USART2 global interrupt and EXTI Line 26 interrupt
pub const INTERRUPT_USART2_EXTI26: u32 = 38;
/// USART3 global interrupt and EXTI Line 28 interrupt
pub const INTERRUPT_USART3_EXTI28: u32 = 39;
/// EXTI Line15 to Line10 interrupts
pub const INTERRUPT_EXTI15_10: u32 = 40;
/// UART4 global and EXTI Line 34 interrupts
pub const INTERRUPT_UART4_EXTI34: u32 = 52;
/// UART5 global and EXTI Line 35 interrupts
pub const INTERRUPT_UART5_EXTI35: u32 = 53;
/// USB wakeup from Suspend and EXTI Line 18
pub const INTERRUPT_USB_WKUP_EXTI: u32 = 76;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40010400</baseAddress>
  <description>
                External interrupt/event
                controller
            </description>
  <groupName>EXTI</groupName>
  <interrupt>
    <description>Tamper and TimeStamp interrupts</description>
    <name>TAMP_STAMP</name>
    <value>2</value>
  </interrupt>
  <interrupt>
    <description>EXTI Line0 interrupt</description>
    <name>EXTI0</name>
    <value>6</value>
  </interrupt>
  <interrupt>
    <description>EXTI Line3 interrupt</description>
    <name>EXTI1</name>
    <value>7</value>
  </interrupt>
  <interrupt>
    <description>
                    EXTI Line2 and Touch sensing
                    interrupts
                </description>
    <name>EXTI2_TSC</name>
    <value>8</value>
  </interrupt>
  <interrupt>
    <description>EXTI Line3 interrupt</description>
    <name>EXTI3</name>
    <value>9</value>
  </interrupt>
  <interrupt>
    <description>EXTI Line4 interrupt</description>
    <name>EXTI4</name>
    <value>10</value>
  </interrupt>
  <interrupt>
    <description>EXTI Line5 to Line9 interrupts</description>
    <name>EXTI9_5</name>
    <value>23</value>
  </interrupt>
  <interrupt>
    <description>
                    I2C1 event interrupt and EXTI Line23
                    interrupt
                </description>
    <name>I2C1_EV_EXTI23</name>
    <value>31</value>
  </interrupt>
  <interrupt>
    <description>
                    USART1 global interrupt and EXTI Line 25
                    interrupt
                </description>
    <name>USART1_EXTI25</name>
    <value>37</value>
  </interrupt>
  <interrupt>
    <description>
                    USART2 global interrupt and EXTI Line 26
                    interrupt
                </description>
    <name>USART2_EXTI26</name>
    <value>38</value>
  </interrupt>
  <interrupt>
    <description>
                    USART3 global interrupt and EXTI Line 28
                    interrupt
                </description>
    <name>USART3_EXTI28</name>
    <value>39</value>
  </interrupt>
  <interrupt>
    <description>EXTI Line15 to Line10 interrupts</description>
    <name>EXTI15_10</name>
    <value>40</value>
  </interrupt>
  <interrupt>
    <description>
                    UART4 global and EXTI Line 34
                    interrupts
                </description>
    <name>UART4_EXTI34</name>
    <value>52</value>
  </interrupt>
  <interrupt>
    <description>
                    UART5 global and EXTI Line 35
                    interrupts
                </description>
    <name>UART5_EXTI35</name>
    <value>53</value>
  </interrupt>
  <interrupt>
    <description>
                    USB wakeup from Suspend and EXTI Line
                    18
                </description>
    <name>USB_WKUP_EXTI</name>
    <value>76</value>
  </interrupt>
  <name>EXTI</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>Interrupt mask register</description>
      <displayName>IMR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 0</description>
          <name>MR0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 1</description>
          <name>MR1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 2</description>
          <name>MR2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 3</description>
          <name>MR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 4</description>
          <name>MR4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 5</description>
          <name>MR5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 6</description>
          <name>MR6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 7</description>
          <name>MR7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 8</description>
          <name>MR8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 9</description>
          <name>MR9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 10</description>
          <name>MR10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 11</description>
          <name>MR11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 12</description>
          <name>MR12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 13</description>
          <name>MR13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 14</description>
          <name>MR14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 15</description>
          <name>MR15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 16</description>
          <name>MR16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 17</description>
          <name>MR17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 18</description>
          <name>MR18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 19</description>
          <name>MR19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 20</description>
          <name>MR20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 21</description>
          <name>MR21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 22</description>
          <name>MR22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 23</description>
          <name>MR23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 24</description>
          <name>MR24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 25</description>
          <name>MR25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 26</description>
          <name>MR26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 27</description>
          <name>MR27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 28</description>
          <name>MR28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 29</description>
          <name>MR29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 30</description>
          <name>MR30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 31</description>
          <name>MR31</name>
        </field>
      </fields>
      <name>IMR1</name>
      <resetValue>0x1F800000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>Event mask register</description>
      <displayName>EMR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 0</description>
          <name>MR0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 1</description>
          <name>MR1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 2</description>
          <name>MR2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 3</description>
          <name>MR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 4</description>
          <name>MR4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 5</description>
          <name>MR5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 6</description>
          <name>MR6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 7</description>
          <name>MR7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 8</description>
          <name>MR8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 9</description>
          <name>MR9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 10</description>
          <name>MR10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 11</description>
          <name>MR11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 12</description>
          <name>MR12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 13</description>
          <name>MR13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 14</description>
          <name>MR14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 15</description>
          <name>MR15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 16</description>
          <name>MR16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 17</description>
          <name>MR17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 18</description>
          <name>MR18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 19</description>
          <name>MR19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 20</description>
          <name>MR20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 21</description>
          <name>MR21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 22</description>
          <name>MR22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 23</description>
          <name>MR23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 24</description>
          <name>MR24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 25</description>
          <name>MR25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 26</description>
          <name>MR26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 27</description>
          <name>MR27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 28</description>
          <name>MR28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 29</description>
          <name>MR29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 30</description>
          <name>MR30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 31</description>
          <name>MR31</name>
        </field>
      </fields>
      <name>EMR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>
                        Rising Trigger selection
                        register
                    </description>
      <displayName>RTSR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 0
                            </description>
          <name>TR0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 1
                            </description>
          <name>TR1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 2
                            </description>
          <name>TR2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 3
                            </description>
          <name>TR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 4
                            </description>
          <name>TR4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 5
                            </description>
          <name>TR5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 6
                            </description>
          <name>TR6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 7
                            </description>
          <name>TR7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 8
                            </description>
          <name>TR8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 9
                            </description>
          <name>TR9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 10
                            </description>
          <name>TR10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 11
                            </description>
          <name>TR11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 12
                            </description>
          <name>TR12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 13
                            </description>
          <name>TR13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 14
                            </description>
          <name>TR14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 15
                            </description>
          <name>TR15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 16
                            </description>
          <name>TR16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 17
                            </description>
          <name>TR17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 18
                            </description>
          <name>TR18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 19
                            </description>
          <name>TR19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 20
                            </description>
          <name>TR20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 21
                            </description>
          <name>TR21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 22
                            </description>
          <name>TR22</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 29
                            </description>
          <name>TR29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 30
                            </description>
          <name>TR30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration of
                                line 31
                            </description>
          <name>TR31</name>
        </field>
      </fields>
      <name>RTSR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>
                        Falling Trigger selection
                        register
                    </description>
      <displayName>FTSR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 0
                            </description>
          <name>TR0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 1
                            </description>
          <name>TR1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 2
                            </description>
          <name>TR2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 3
                            </description>
          <name>TR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 4
                            </description>
          <name>TR4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 5
                            </description>
          <name>TR5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 6
                            </description>
          <name>TR6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 7
                            </description>
          <name>TR7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 8
                            </description>
          <name>TR8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 9
                            </description>
          <name>TR9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 10
                            </description>
          <name>TR10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 11
                            </description>
          <name>TR11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 12
                            </description>
          <name>TR12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 13
                            </description>
          <name>TR13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 14
                            </description>
          <name>TR14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 15
                            </description>
          <name>TR15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 16
                            </description>
          <name>TR16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 17
                            </description>
          <name>TR17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 18
                            </description>
          <name>TR18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 19
                            </description>
          <name>TR19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 20
                            </description>
          <name>TR20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 21
                            </description>
          <name>TR21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 22
                            </description>
          <name>TR22</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 29
                            </description>
          <name>TR29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 30.
                            </description>
          <name>TR30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration of
                                line 31
                            </description>
          <name>TR31</name>
        </field>
      </fields>
      <name>FTSR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>
                        Software interrupt event
                        register
                    </description>
      <displayName>SWIER1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                0
                            </description>
          <name>SWIER0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                1
                            </description>
          <name>SWIER1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                2
                            </description>
          <name>SWIER2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                3
                            </description>
          <name>SWIER3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                4
                            </description>
          <name>SWIER4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                5
                            </description>
          <name>SWIER5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                6
                            </description>
          <name>SWIER6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                7
                            </description>
          <name>SWIER7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                8
                            </description>
          <name>SWIER8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                9
                            </description>
          <name>SWIER9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                10
                            </description>
          <name>SWIER10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                11
                            </description>
          <name>SWIER11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                12
                            </description>
          <name>SWIER12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                13
                            </description>
          <name>SWIER13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                14
                            </description>
          <name>SWIER14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                15
                            </description>
          <name>SWIER15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                16
                            </description>
          <name>SWIER16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                17
                            </description>
          <name>SWIER17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                18
                            </description>
          <name>SWIER18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                19
                            </description>
          <name>SWIER19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                20
                            </description>
          <name>SWIER20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                21
                            </description>
          <name>SWIER21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                22
                            </description>
          <name>SWIER22</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                29
                            </description>
          <name>SWIER29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                309
                            </description>
          <name>SWIER30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software Interrupt on line
                                319
                            </description>
          <name>SWIER31</name>
        </field>
      </fields>
      <name>SWIER1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>Pending register</description>
      <displayName>PR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 0</description>
          <name>PR0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 1</description>
          <name>PR1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 2</description>
          <name>PR2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 3</description>
          <name>PR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 4</description>
          <name>PR4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 5</description>
          <name>PR5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 6</description>
          <name>PR6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 7</description>
          <name>PR7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 8</description>
          <name>PR8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 9</description>
          <name>PR9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 10</description>
          <name>PR10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 11</description>
          <name>PR11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 12</description>
          <name>PR12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 13</description>
          <name>PR13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 14</description>
          <name>PR14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 15</description>
          <name>PR15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 16</description>
          <name>PR16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 17</description>
          <name>PR17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 18</description>
          <name>PR18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 19</description>
          <name>PR19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 20</description>
          <name>PR20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 21</description>
          <name>PR21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 22</description>
          <name>PR22</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 29</description>
          <name>PR29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 30</description>
          <name>PR30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 31</description>
          <name>PR31</name>
        </field>
      </fields>
      <name>PR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>Interrupt mask register</description>
      <displayName>IMR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Interrupt Mask on external/internal line
                                32
                            </description>
          <name>MR32</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Interrupt Mask on external/internal line
                                33
                            </description>
          <name>MR33</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Interrupt Mask on external/internal line
                                34
                            </description>
          <name>MR34</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Interrupt Mask on external/internal line
                                35
                            </description>
          <name>MR35</name>
        </field>
      </fields>
      <name>IMR2</name>
      <resetValue>0xFFFFFFFC</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>Event mask register</description>
      <displayName>EMR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Event mask on external/internal line
                                32
                            </description>
          <name>MR32</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Event mask on external/internal line
                                33
                            </description>
          <name>MR33</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Event mask on external/internal line
                                34
                            </description>
          <name>MR34</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Event mask on external/internal line
                                35
                            </description>
          <name>MR35</name>
        </field>
      </fields>
      <name>EMR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>
                        Rising Trigger selection
                        register
                    </description>
      <displayName>RTSR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration bit
                                of line 32
                            </description>
          <name>TR32</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Rising trigger event configuration bit
                                of line 33
                            </description>
          <name>TR33</name>
        </field>
      </fields>
      <name>RTSR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x24</addressOffset>
      <description>
                        Falling Trigger selection
                        register
                    </description>
      <displayName>FTSR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration bit
                                of line 32
                            </description>
          <name>TR32</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Falling trigger event configuration bit
                                of line 33
                            </description>
          <name>TR33</name>
        </field>
      </fields>
      <name>FTSR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28</addressOffset>
      <description>
                        Software interrupt event
                        register
                    </description>
      <displayName>SWIER2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software interrupt on line
                                32
                            </description>
          <name>SWIER32</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Software interrupt on line
                                33
                            </description>
          <name>SWIER33</name>
        </field>
      </fields>
      <name>SWIER2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2C</addressOffset>
      <description>Pending register</description>
      <displayName>PR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit on line 32</description>
          <name>PR32</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit on line 33</description>
          <name>PR33</name>
        </field>
      </fields>
      <name>PR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
