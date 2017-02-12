/// Interrupt Controller Type Register
pub mod ictr {
    /// Total number of interrupt lines in groups
    /// Access: read-only, Width: 4, Offset: 0
    /// Get Total number of interrupt lines in groups
    pub fn intlinesnum() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
}
/// Software Triggered Interrupt Register
pub mod stir {
    /// interrupt to be triggered
    /// Access: write-only, Width: 9, Offset: 0
    /// Set interrupt to be triggered
    pub fn intid(value: u16) {
        debug_assert!(value <= 0b111111111, "intid out of range");
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0xF00u32) as *mut u32, value) };
    }
}
/// Interrupt Set-Enable Register
pub mod iser0 {
    pub struct ReadonlyCache {
        /// SETENA
        pub setena: u32,
    }
    pub struct Cache {
        /// SETENA
        pub setena: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x100u32) as *mut u32) };
        ReadonlyCache {
            setena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x100u32) as *mut u32) };
        Cache {
            setena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.setena as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x100u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Set-Enable Register
pub mod iser1 {
    pub struct ReadonlyCache {
        /// SETENA
        pub setena: u32,
    }
    pub struct Cache {
        /// SETENA
        pub setena: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x104u32) as *mut u32) };
        ReadonlyCache {
            setena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x104u32) as *mut u32) };
        Cache {
            setena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.setena as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x104u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Set-Enable Register
pub mod iser2 {
    pub struct ReadonlyCache {
        /// SETENA
        pub setena: u32,
    }
    pub struct Cache {
        /// SETENA
        pub setena: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x108u32) as *mut u32) };
        ReadonlyCache {
            setena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x108u32) as *mut u32) };
        Cache {
            setena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.setena as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x108u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Clear-Enable Register
pub mod icer0 {
    pub struct ReadonlyCache {
        /// CLRENA
        pub clrena: u32,
    }
    pub struct Cache {
        /// CLRENA
        pub clrena: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x180u32) as *mut u32) };
        ReadonlyCache {
            clrena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x180u32) as *mut u32) };
        Cache {
            clrena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.clrena as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x180u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Clear-Enable Register
pub mod icer1 {
    pub struct ReadonlyCache {
        /// CLRENA
        pub clrena: u32,
    }
    pub struct Cache {
        /// CLRENA
        pub clrena: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x184u32) as *mut u32) };
        ReadonlyCache {
            clrena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x184u32) as *mut u32) };
        Cache {
            clrena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.clrena as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x184u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Clear-Enable Register
pub mod icer2 {
    pub struct ReadonlyCache {
        /// CLRENA
        pub clrena: u32,
    }
    pub struct Cache {
        /// CLRENA
        pub clrena: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x188u32) as *mut u32) };
        ReadonlyCache {
            clrena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x188u32) as *mut u32) };
        Cache {
            clrena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.clrena as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x188u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Set-Pending Register
pub mod ispr0 {
    pub struct ReadonlyCache {
        /// SETPEND
        pub setpend: u32,
    }
    pub struct Cache {
        /// SETPEND
        pub setpend: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x200u32) as *mut u32) };
        ReadonlyCache {
            setpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x200u32) as *mut u32) };
        Cache {
            setpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.setpend as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x200u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Set-Pending Register
pub mod ispr1 {
    pub struct ReadonlyCache {
        /// SETPEND
        pub setpend: u32,
    }
    pub struct Cache {
        /// SETPEND
        pub setpend: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x204u32) as *mut u32) };
        ReadonlyCache {
            setpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x204u32) as *mut u32) };
        Cache {
            setpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.setpend as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x204u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Set-Pending Register
pub mod ispr2 {
    pub struct ReadonlyCache {
        /// SETPEND
        pub setpend: u32,
    }
    pub struct Cache {
        /// SETPEND
        pub setpend: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x208u32) as *mut u32) };
        ReadonlyCache {
            setpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x208u32) as *mut u32) };
        Cache {
            setpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.setpend as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x208u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Clear-Pending Register
pub mod icpr0 {
    pub struct ReadonlyCache {
        /// CLRPEND
        pub clrpend: u32,
    }
    pub struct Cache {
        /// CLRPEND
        pub clrpend: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x280u32) as *mut u32) };
        ReadonlyCache {
            clrpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x280u32) as *mut u32) };
        Cache {
            clrpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.clrpend as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x280u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Clear-Pending Register
pub mod icpr1 {
    pub struct ReadonlyCache {
        /// CLRPEND
        pub clrpend: u32,
    }
    pub struct Cache {
        /// CLRPEND
        pub clrpend: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x284u32) as *mut u32) };
        ReadonlyCache {
            clrpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x284u32) as *mut u32) };
        Cache {
            clrpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.clrpend as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x284u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Clear-Pending Register
pub mod icpr2 {
    pub struct ReadonlyCache {
        /// CLRPEND
        pub clrpend: u32,
    }
    pub struct Cache {
        /// CLRPEND
        pub clrpend: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x288u32) as *mut u32) };
        ReadonlyCache {
            clrpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x288u32) as *mut u32) };
        Cache {
            clrpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.clrpend as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x288u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Active Bit Register
pub mod iabr0 {
    /// ACTIVE
    /// Access: read-only, Width: 32, Offset: 0
    /// Get ACTIVE
    pub fn active() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x300u32) as *mut u32) };
        let value = value & (0b11111111111111111111111111111111 << 0);
        value as u32
    }
}
/// Interrupt Active Bit Register
pub mod iabr1 {
    /// ACTIVE
    /// Access: read-only, Width: 32, Offset: 0
    /// Get ACTIVE
    pub fn active() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x304u32) as *mut u32) };
        let value = value & (0b11111111111111111111111111111111 << 0);
        value as u32
    }
}
/// Interrupt Active Bit Register
pub mod iabr2 {
    /// ACTIVE
    /// Access: read-only, Width: 32, Offset: 0
    /// Get ACTIVE
    pub fn active() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x308u32) as *mut u32) };
        let value = value & (0b11111111111111111111111111111111 << 0);
        value as u32
    }
}
/// Interrupt Priority Register
pub mod ipr0 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x400u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x400u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x400u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr1 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x404u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x404u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x404u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr2 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x408u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x408u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x408u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr3 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x40Cu32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x40Cu32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x40Cu32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr4 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x410u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x410u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x410u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr5 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x414u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x414u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x414u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr6 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x418u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x418u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x418u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr7 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x41Cu32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x41Cu32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x41Cu32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr8 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x420u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x420u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x420u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr9 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x424u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x424u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x424u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr10 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x428u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x428u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x428u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr11 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x42Cu32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x42Cu32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x42Cu32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr12 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x430u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x430u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x430u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr13 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x434u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x434u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x434u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr14 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x438u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x438u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x438u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr15 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x43Cu32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x43Cu32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x43Cu32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr16 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x440u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x440u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x440u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr17 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x444u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x444u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x444u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr18 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x448u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x448u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x448u32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr19 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x44Cu32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x44Cu32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x44Cu32) as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr20 {
    pub struct ReadonlyCache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub struct Cache {
        /// IPR_N0
        pub ipr_n0: u8,
        /// IPR_N1
        pub ipr_n1: u8,
        /// IPR_N2
        pub ipr_n2: u8,
        /// IPR_N3
        pub ipr_n3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x450u32) as *mut u32) };
        ReadonlyCache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000E000u32 + 0x450u32) as *mut u32) };
        Cache {
            ipr_n0: ((value >> 0) & 0b11111111) as u8,
            ipr_n1: ((value >> 8) & 0b11111111) as u8,
            ipr_n2: ((value >> 16) & 0b11111111) as u8,
            ipr_n3: ((value >> 24) & 0b11111111) as u8,
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
                | ((self.ipr_n0 as u32) << 0)
                | ((self.ipr_n1 as u32) << 8)
                | ((self.ipr_n2 as u32) << 16)
                | ((self.ipr_n3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000E000u32 + 0x450u32) as *mut u32, value) };
        }
    }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x1001</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0xE000E000</baseAddress>
  <description>
                Nested Vectored Interrupt
                Controller
            </description>
  <groupName>NVIC</groupName>
  <name>NVIC</name>
  <registers>
    <register>
      <access>read-only</access>
      <addressOffset>0x4</addressOffset>
      <description>
                        Interrupt Controller Type
                        Register
                    </description>
      <displayName>ICTR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Total number of interrupt lines in
                                groups
                            </description>
          <name>INTLINESNUM</name>
        </field>
      </fields>
      <name>ICTR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0xF00</addressOffset>
      <description>
                        Software Triggered Interrupt
                        Register
                    </description>
      <displayName>STIR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>9</bitWidth>
          <description>interrupt to be triggered</description>
          <name>INTID</name>
        </field>
      </fields>
      <name>STIR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x100</addressOffset>
      <description>Interrupt Set-Enable Register</description>
      <displayName>ISER0</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>SETENA</description>
          <name>SETENA</name>
        </field>
      </fields>
      <name>ISER0</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x104</addressOffset>
      <description>Interrupt Set-Enable Register</description>
      <displayName>ISER1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>SETENA</description>
          <name>SETENA</name>
        </field>
      </fields>
      <name>ISER1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x108</addressOffset>
      <description>Interrupt Set-Enable Register</description>
      <displayName>ISER2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>SETENA</description>
          <name>SETENA</name>
        </field>
      </fields>
      <name>ISER2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x180</addressOffset>
      <description>
                        Interrupt Clear-Enable
                        Register
                    </description>
      <displayName>ICER0</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>CLRENA</description>
          <name>CLRENA</name>
        </field>
      </fields>
      <name>ICER0</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x184</addressOffset>
      <description>
                        Interrupt Clear-Enable
                        Register
                    </description>
      <displayName>ICER1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>CLRENA</description>
          <name>CLRENA</name>
        </field>
      </fields>
      <name>ICER1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x188</addressOffset>
      <description>
                        Interrupt Clear-Enable
                        Register
                    </description>
      <displayName>ICER2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>CLRENA</description>
          <name>CLRENA</name>
        </field>
      </fields>
      <name>ICER2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x200</addressOffset>
      <description>Interrupt Set-Pending Register</description>
      <displayName>ISPR0</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>SETPEND</description>
          <name>SETPEND</name>
        </field>
      </fields>
      <name>ISPR0</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x204</addressOffset>
      <description>Interrupt Set-Pending Register</description>
      <displayName>ISPR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>SETPEND</description>
          <name>SETPEND</name>
        </field>
      </fields>
      <name>ISPR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x208</addressOffset>
      <description>Interrupt Set-Pending Register</description>
      <displayName>ISPR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>SETPEND</description>
          <name>SETPEND</name>
        </field>
      </fields>
      <name>ISPR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x280</addressOffset>
      <description>
                        Interrupt Clear-Pending
                        Register
                    </description>
      <displayName>ICPR0</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>CLRPEND</description>
          <name>CLRPEND</name>
        </field>
      </fields>
      <name>ICPR0</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x284</addressOffset>
      <description>
                        Interrupt Clear-Pending
                        Register
                    </description>
      <displayName>ICPR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>CLRPEND</description>
          <name>CLRPEND</name>
        </field>
      </fields>
      <name>ICPR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x288</addressOffset>
      <description>
                        Interrupt Clear-Pending
                        Register
                    </description>
      <displayName>ICPR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>CLRPEND</description>
          <name>CLRPEND</name>
        </field>
      </fields>
      <name>ICPR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x300</addressOffset>
      <description>Interrupt Active Bit Register</description>
      <displayName>IABR0</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>ACTIVE</description>
          <name>ACTIVE</name>
        </field>
      </fields>
      <name>IABR0</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x304</addressOffset>
      <description>Interrupt Active Bit Register</description>
      <displayName>IABR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>ACTIVE</description>
          <name>ACTIVE</name>
        </field>
      </fields>
      <name>IABR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x308</addressOffset>
      <description>Interrupt Active Bit Register</description>
      <displayName>IABR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>ACTIVE</description>
          <name>ACTIVE</name>
        </field>
      </fields>
      <name>IABR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x400</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR0</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR0</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x404</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x408</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x40C</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x410</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR4</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR4</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x414</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR5</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR5</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x418</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR6</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR6</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x41C</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR7</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR7</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x420</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR8</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR8</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x424</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR9</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR9</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x428</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR10</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR10</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x42C</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR11</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR11</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x430</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR12</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR12</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x434</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR13</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR13</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x438</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR14</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR14</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x43C</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR15</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR15</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x440</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR16</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR16</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x444</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR17</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR17</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x448</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR18</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR18</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x44C</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR19</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR19</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x450</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR20</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR20</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
