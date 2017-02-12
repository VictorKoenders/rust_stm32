/// DMA interrupt status register (DMA_ISR)
pub mod isr {
    /// Channel 1 Global interrupt flag
    /// Access: read-only, Width: 1, Offset: 0
    /// Get Channel 1 Global interrupt flag
    pub fn gif1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Channel 1 Transfer Complete flag
    /// Access: read-only, Width: 1, Offset: 1
    /// Get Channel 1 Transfer Complete flag
    pub fn tcif1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// Channel 1 Half Transfer Complete flag
    /// Access: read-only, Width: 1, Offset: 2
    /// Get Channel 1 Half Transfer Complete flag
    pub fn htif1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// Channel 1 Transfer Error flag
    /// Access: read-only, Width: 1, Offset: 3
    /// Get Channel 1 Transfer Error flag
    pub fn teif1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// Channel 2 Global interrupt flag
    /// Access: read-only, Width: 1, Offset: 4
    /// Get Channel 2 Global interrupt flag
    pub fn gif2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// Channel 2 Transfer Complete flag
    /// Access: read-only, Width: 1, Offset: 5
    /// Get Channel 2 Transfer Complete flag
    pub fn tcif2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// Channel 2 Half Transfer Complete flag
    /// Access: read-only, Width: 1, Offset: 6
    /// Get Channel 2 Half Transfer Complete flag
    pub fn htif2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Channel 2 Transfer Error flag
    /// Access: read-only, Width: 1, Offset: 7
    /// Get Channel 2 Transfer Error flag
    pub fn teif2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Channel 3 Global interrupt flag
    /// Access: read-only, Width: 1, Offset: 8
    /// Get Channel 3 Global interrupt flag
    pub fn gif3() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Channel 3 Transfer Complete flag
    /// Access: read-only, Width: 1, Offset: 9
    /// Get Channel 3 Transfer Complete flag
    pub fn tcif3() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// Channel 3 Half Transfer Complete flag
    /// Access: read-only, Width: 1, Offset: 10
    /// Get Channel 3 Half Transfer Complete flag
    pub fn htif3() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 10);
        value > 0
    }
    /// Channel 3 Transfer Error flag
    /// Access: read-only, Width: 1, Offset: 11
    /// Get Channel 3 Transfer Error flag
    pub fn teif3() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Channel 4 Global interrupt flag
    /// Access: read-only, Width: 1, Offset: 12
    /// Get Channel 4 Global interrupt flag
    pub fn gif4() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 12);
        value > 0
    }
    /// Channel 4 Transfer Complete flag
    /// Access: read-only, Width: 1, Offset: 13
    /// Get Channel 4 Transfer Complete flag
    pub fn tcif4() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 13);
        value > 0
    }
    /// Channel 4 Half Transfer Complete flag
    /// Access: read-only, Width: 1, Offset: 14
    /// Get Channel 4 Half Transfer Complete flag
    pub fn htif4() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// Channel 4 Transfer Error flag
    /// Access: read-only, Width: 1, Offset: 15
    /// Get Channel 4 Transfer Error flag
    pub fn teif4() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// Channel 5 Global interrupt flag
    /// Access: read-only, Width: 1, Offset: 16
    /// Get Channel 5 Global interrupt flag
    pub fn gif5() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 16);
        value > 0
    }
    /// Channel 5 Transfer Complete flag
    /// Access: read-only, Width: 1, Offset: 17
    /// Get Channel 5 Transfer Complete flag
    pub fn tcif5() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 17);
        value > 0
    }
    /// Channel 5 Half Transfer Complete flag
    /// Access: read-only, Width: 1, Offset: 18
    /// Get Channel 5 Half Transfer Complete flag
    pub fn htif5() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 18);
        value > 0
    }
    /// Channel 5 Transfer Error flag
    /// Access: read-only, Width: 1, Offset: 19
    /// Get Channel 5 Transfer Error flag
    pub fn teif5() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 19);
        value > 0
    }
    /// Channel 6 Global interrupt flag
    /// Access: read-only, Width: 1, Offset: 20
    /// Get Channel 6 Global interrupt flag
    pub fn gif6() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 20);
        value > 0
    }
    /// Channel 6 Transfer Complete flag
    /// Access: read-only, Width: 1, Offset: 21
    /// Get Channel 6 Transfer Complete flag
    pub fn tcif6() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 21);
        value > 0
    }
    /// Channel 6 Half Transfer Complete flag
    /// Access: read-only, Width: 1, Offset: 22
    /// Get Channel 6 Half Transfer Complete flag
    pub fn htif6() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 22);
        value > 0
    }
    /// Channel 6 Transfer Error flag
    /// Access: read-only, Width: 1, Offset: 23
    /// Get Channel 6 Transfer Error flag
    pub fn teif6() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 23);
        value > 0
    }
    /// Channel 7 Global interrupt flag
    /// Access: read-only, Width: 1, Offset: 24
    /// Get Channel 7 Global interrupt flag
    pub fn gif7() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 24);
        value > 0
    }
    /// Channel 7 Transfer Complete flag
    /// Access: read-only, Width: 1, Offset: 25
    /// Get Channel 7 Transfer Complete flag
    pub fn tcif7() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 25);
        value > 0
    }
    /// Channel 7 Half Transfer Complete flag
    /// Access: read-only, Width: 1, Offset: 26
    /// Get Channel 7 Half Transfer Complete flag
    pub fn htif7() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 26);
        value > 0
    }
    /// Channel 7 Transfer Error flag
    /// Access: read-only, Width: 1, Offset: 27
    /// Get Channel 7 Transfer Error flag
    pub fn teif7() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 27);
        value > 0
    }
}
/// DMA interrupt flag clear register (DMA_IFCR)
pub mod ifcr {
    /// Channel 1 Global interrupt clear
    /// Access: write-only, Width: 1, Offset: 0
    /// Set Channel 1 Global interrupt clear
    pub fn cgif1(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 1 Transfer Complete clear
    /// Access: write-only, Width: 1, Offset: 1
    /// Set Channel 1 Transfer Complete clear
    pub fn ctcif1(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 1 Half Transfer clear
    /// Access: write-only, Width: 1, Offset: 2
    /// Set Channel 1 Half Transfer clear
    pub fn chtif1(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 1 Transfer Error clear
    /// Access: write-only, Width: 1, Offset: 3
    /// Set Channel 1 Transfer Error clear
    pub fn cteif1(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 2 Global interrupt clear
    /// Access: write-only, Width: 1, Offset: 4
    /// Set Channel 2 Global interrupt clear
    pub fn cgif2(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 2 Transfer Complete clear
    /// Access: write-only, Width: 1, Offset: 5
    /// Set Channel 2 Transfer Complete clear
    pub fn ctcif2(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 2 Half Transfer clear
    /// Access: write-only, Width: 1, Offset: 6
    /// Set Channel 2 Half Transfer clear
    pub fn chtif2(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 2 Transfer Error clear
    /// Access: write-only, Width: 1, Offset: 7
    /// Set Channel 2 Transfer Error clear
    pub fn cteif2(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 3 Global interrupt clear
    /// Access: write-only, Width: 1, Offset: 8
    /// Set Channel 3 Global interrupt clear
    pub fn cgif3(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 3 Transfer Complete clear
    /// Access: write-only, Width: 1, Offset: 9
    /// Set Channel 3 Transfer Complete clear
    pub fn ctcif3(value: bool) {
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 3 Half Transfer clear
    /// Access: write-only, Width: 1, Offset: 10
    /// Set Channel 3 Half Transfer clear
    pub fn chtif3(value: bool) {
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 3 Transfer Error clear
    /// Access: write-only, Width: 1, Offset: 11
    /// Set Channel 3 Transfer Error clear
    pub fn cteif3(value: bool) {
        let value = value as u32;
        let value = value << 11;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 4 Global interrupt clear
    /// Access: write-only, Width: 1, Offset: 12
    /// Set Channel 4 Global interrupt clear
    pub fn cgif4(value: bool) {
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 4 Transfer Complete clear
    /// Access: write-only, Width: 1, Offset: 13
    /// Set Channel 4 Transfer Complete clear
    pub fn ctcif4(value: bool) {
        let value = value as u32;
        let value = value << 13;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 4 Half Transfer clear
    /// Access: write-only, Width: 1, Offset: 14
    /// Set Channel 4 Half Transfer clear
    pub fn chtif4(value: bool) {
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 4 Transfer Error clear
    /// Access: write-only, Width: 1, Offset: 15
    /// Set Channel 4 Transfer Error clear
    pub fn cteif4(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 5 Global interrupt clear
    /// Access: write-only, Width: 1, Offset: 16
    /// Set Channel 5 Global interrupt clear
    pub fn cgif5(value: bool) {
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 5 Transfer Complete clear
    /// Access: write-only, Width: 1, Offset: 17
    /// Set Channel 5 Transfer Complete clear
    pub fn ctcif5(value: bool) {
        let value = value as u32;
        let value = value << 17;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 5 Half Transfer clear
    /// Access: write-only, Width: 1, Offset: 18
    /// Set Channel 5 Half Transfer clear
    pub fn chtif5(value: bool) {
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 5 Transfer Error clear
    /// Access: write-only, Width: 1, Offset: 19
    /// Set Channel 5 Transfer Error clear
    pub fn cteif5(value: bool) {
        let value = value as u32;
        let value = value << 19;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 6 Global interrupt clear
    /// Access: write-only, Width: 1, Offset: 20
    /// Set Channel 6 Global interrupt clear
    pub fn cgif6(value: bool) {
        let value = value as u32;
        let value = value << 20;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 6 Transfer Complete clear
    /// Access: write-only, Width: 1, Offset: 21
    /// Set Channel 6 Transfer Complete clear
    pub fn ctcif6(value: bool) {
        let value = value as u32;
        let value = value << 21;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 6 Half Transfer clear
    /// Access: write-only, Width: 1, Offset: 22
    /// Set Channel 6 Half Transfer clear
    pub fn chtif6(value: bool) {
        let value = value as u32;
        let value = value << 22;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 6 Transfer Error clear
    /// Access: write-only, Width: 1, Offset: 23
    /// Set Channel 6 Transfer Error clear
    pub fn cteif6(value: bool) {
        let value = value as u32;
        let value = value << 23;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 7 Global interrupt clear
    /// Access: write-only, Width: 1, Offset: 24
    /// Set Channel 7 Global interrupt clear
    pub fn cgif7(value: bool) {
        let value = value as u32;
        let value = value << 24;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 7 Transfer Complete clear
    /// Access: write-only, Width: 1, Offset: 25
    /// Set Channel 7 Transfer Complete clear
    pub fn ctcif7(value: bool) {
        let value = value as u32;
        let value = value << 25;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 7 Half Transfer clear
    /// Access: write-only, Width: 1, Offset: 26
    /// Set Channel 7 Half Transfer clear
    pub fn chtif7(value: bool) {
        let value = value as u32;
        let value = value << 26;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
    /// Channel 7 Transfer Error clear
    /// Access: write-only, Width: 1, Offset: 27
    /// Set Channel 7 Transfer Error clear
    pub fn cteif7(value: bool) {
        let value = value as u32;
        let value = value << 27;
        unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4u32) as *mut u32, value) };
    }
}
/// DMA channel configuration register (DMA_CCR)
pub mod ccr1 {
    pub struct ReadonlyCache {
        /// Channel enable
        pub en: bool,
        /// Transfer complete interrupt enable
        pub tcie: bool,
        /// Half Transfer interrupt enable
        pub htie: bool,
        /// Transfer error interrupt enable
        pub teie: bool,
        /// Data transfer direction
        pub dir: bool,
        /// Circular mode
        pub circ: bool,
        /// Peripheral increment mode
        pub pinc: bool,
        /// Memory increment mode
        pub minc: bool,
        /// Peripheral size
        pub psize: bool,
        /// Memory size
        pub msize: bool,
        /// Channel Priority level
        pub pl: bool,
        /// Memory to memory mode
        pub mem2mem: bool,
    }
    pub struct Cache {
        /// Channel enable
        pub en: bool,
        /// Transfer complete interrupt enable
        pub tcie: bool,
        /// Half Transfer interrupt enable
        pub htie: bool,
        /// Transfer error interrupt enable
        pub teie: bool,
        /// Data transfer direction
        pub dir: bool,
        /// Circular mode
        pub circ: bool,
        /// Peripheral increment mode
        pub pinc: bool,
        /// Memory increment mode
        pub minc: bool,
        /// Peripheral size
        pub psize: bool,
        /// Memory size
        pub msize: bool,
        /// Channel Priority level
        pub pl: bool,
        /// Memory to memory mode
        pub mem2mem: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            en: ((value >> 0) & 0b1) > 0,
            tcie: ((value >> 1) & 0b1) > 0,
            htie: ((value >> 2) & 0b1) > 0,
            teie: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            circ: ((value >> 5) & 0b1) > 0,
            pinc: ((value >> 6) & 0b1) > 0,
            minc: ((value >> 7) & 0b1) > 0,
            psize: ((value >> 8) & 0b1) > 0,
            msize: ((value >> 10) & 0b1) > 0,
            pl: ((value >> 12) & 0b1) > 0,
            mem2mem: ((value >> 14) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x8u32) as *mut u32) };
        Cache {
            en: ((value >> 0) & 0b1) > 0,
            tcie: ((value >> 1) & 0b1) > 0,
            htie: ((value >> 2) & 0b1) > 0,
            teie: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            circ: ((value >> 5) & 0b1) > 0,
            pinc: ((value >> 6) & 0b1) > 0,
            minc: ((value >> 7) & 0b1) > 0,
            psize: ((value >> 8) & 0b1) > 0,
            msize: ((value >> 10) & 0b1) > 0,
            pl: ((value >> 12) & 0b1) > 0,
            mem2mem: ((value >> 14) & 0b1) > 0,
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
                | ((self.en as u32) << 0)
                | ((self.tcie as u32) << 1)
                | ((self.htie as u32) << 2)
                | ((self.teie as u32) << 3)
                | ((self.dir as u32) << 4)
                | ((self.circ as u32) << 5)
                | ((self.pinc as u32) << 6)
                | ((self.minc as u32) << 7)
                | ((self.psize as u32) << 8)
                | ((self.msize as u32) << 10)
                | ((self.pl as u32) << 12)
                | ((self.mem2mem as u32) << 14)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 1 number of data register
pub mod cndtr1 {
    pub struct ReadonlyCache {
        /// Number of data to transfer
        pub ndt: u16,
    }
    pub struct Cache {
        /// Number of data to transfer
        pub ndt: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            ndt: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0xCu32) as *mut u32) };
        Cache {
            ndt: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.ndt as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// DMA channel 1 peripheral address register
pub mod cpar1 {
    pub struct ReadonlyCache {
        /// Peripheral address
        pub pa: u32,
    }
    pub struct Cache {
        /// Peripheral address
        pub pa: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            pa: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x10u32) as *mut u32) };
        Cache {
            pa: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.pa as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x10u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 1 memory address register
pub mod cmar1 {
    pub struct ReadonlyCache {
        /// Memory address
        pub ma: u32,
    }
    pub struct Cache {
        /// Memory address
        pub ma: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x14u32) as *mut u32) };
        ReadonlyCache {
            ma: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x14u32) as *mut u32) };
        Cache {
            ma: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.ma as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x14u32) as *mut u32, value) };
        }
    }
}
/// DMA channel configuration register (DMA_CCR)
pub mod ccr2 {
    pub struct ReadonlyCache {
        /// Channel enable
        pub en: bool,
        /// Transfer complete interrupt enable
        pub tcie: bool,
        /// Half Transfer interrupt enable
        pub htie: bool,
        /// Transfer error interrupt enable
        pub teie: bool,
        /// Data transfer direction
        pub dir: bool,
        /// Circular mode
        pub circ: bool,
        /// Peripheral increment mode
        pub pinc: bool,
        /// Memory increment mode
        pub minc: bool,
        /// Peripheral size
        pub psize: bool,
        /// Memory size
        pub msize: bool,
        /// Channel Priority level
        pub pl: bool,
        /// Memory to memory mode
        pub mem2mem: bool,
    }
    pub struct Cache {
        /// Channel enable
        pub en: bool,
        /// Transfer complete interrupt enable
        pub tcie: bool,
        /// Half Transfer interrupt enable
        pub htie: bool,
        /// Transfer error interrupt enable
        pub teie: bool,
        /// Data transfer direction
        pub dir: bool,
        /// Circular mode
        pub circ: bool,
        /// Peripheral increment mode
        pub pinc: bool,
        /// Memory increment mode
        pub minc: bool,
        /// Peripheral size
        pub psize: bool,
        /// Memory size
        pub msize: bool,
        /// Channel Priority level
        pub pl: bool,
        /// Memory to memory mode
        pub mem2mem: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x1Cu32) as *mut u32) };
        ReadonlyCache {
            en: ((value >> 0) & 0b1) > 0,
            tcie: ((value >> 1) & 0b1) > 0,
            htie: ((value >> 2) & 0b1) > 0,
            teie: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            circ: ((value >> 5) & 0b1) > 0,
            pinc: ((value >> 6) & 0b1) > 0,
            minc: ((value >> 7) & 0b1) > 0,
            psize: ((value >> 8) & 0b1) > 0,
            msize: ((value >> 10) & 0b1) > 0,
            pl: ((value >> 12) & 0b1) > 0,
            mem2mem: ((value >> 14) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x1Cu32) as *mut u32) };
        Cache {
            en: ((value >> 0) & 0b1) > 0,
            tcie: ((value >> 1) & 0b1) > 0,
            htie: ((value >> 2) & 0b1) > 0,
            teie: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            circ: ((value >> 5) & 0b1) > 0,
            pinc: ((value >> 6) & 0b1) > 0,
            minc: ((value >> 7) & 0b1) > 0,
            psize: ((value >> 8) & 0b1) > 0,
            msize: ((value >> 10) & 0b1) > 0,
            pl: ((value >> 12) & 0b1) > 0,
            mem2mem: ((value >> 14) & 0b1) > 0,
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
                | ((self.en as u32) << 0)
                | ((self.tcie as u32) << 1)
                | ((self.htie as u32) << 2)
                | ((self.teie as u32) << 3)
                | ((self.dir as u32) << 4)
                | ((self.circ as u32) << 5)
                | ((self.pinc as u32) << 6)
                | ((self.minc as u32) << 7)
                | ((self.psize as u32) << 8)
                | ((self.msize as u32) << 10)
                | ((self.pl as u32) << 12)
                | ((self.mem2mem as u32) << 14)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x1Cu32) as *mut u32, value) };
        }
    }
}
/// DMA channel 2 number of data register
pub mod cndtr2 {
    pub struct ReadonlyCache {
        /// Number of data to transfer
        pub ndt: u16,
    }
    pub struct Cache {
        /// Number of data to transfer
        pub ndt: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x20u32) as *mut u32) };
        ReadonlyCache {
            ndt: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x20u32) as *mut u32) };
        Cache {
            ndt: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.ndt as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x20u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 2 peripheral address register
pub mod cpar2 {
    pub struct ReadonlyCache {
        /// Peripheral address
        pub pa: u32,
    }
    pub struct Cache {
        /// Peripheral address
        pub pa: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x24u32) as *mut u32) };
        ReadonlyCache {
            pa: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x24u32) as *mut u32) };
        Cache {
            pa: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.pa as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x24u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 2 memory address register
pub mod cmar2 {
    pub struct ReadonlyCache {
        /// Memory address
        pub ma: u32,
    }
    pub struct Cache {
        /// Memory address
        pub ma: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x28u32) as *mut u32) };
        ReadonlyCache {
            ma: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x28u32) as *mut u32) };
        Cache {
            ma: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.ma as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x28u32) as *mut u32, value) };
        }
    }
}
/// DMA channel configuration register (DMA_CCR)
pub mod ccr3 {
    pub struct ReadonlyCache {
        /// Channel enable
        pub en: bool,
        /// Transfer complete interrupt enable
        pub tcie: bool,
        /// Half Transfer interrupt enable
        pub htie: bool,
        /// Transfer error interrupt enable
        pub teie: bool,
        /// Data transfer direction
        pub dir: bool,
        /// Circular mode
        pub circ: bool,
        /// Peripheral increment mode
        pub pinc: bool,
        /// Memory increment mode
        pub minc: bool,
        /// Peripheral size
        pub psize: bool,
        /// Memory size
        pub msize: bool,
        /// Channel Priority level
        pub pl: bool,
        /// Memory to memory mode
        pub mem2mem: bool,
    }
    pub struct Cache {
        /// Channel enable
        pub en: bool,
        /// Transfer complete interrupt enable
        pub tcie: bool,
        /// Half Transfer interrupt enable
        pub htie: bool,
        /// Transfer error interrupt enable
        pub teie: bool,
        /// Data transfer direction
        pub dir: bool,
        /// Circular mode
        pub circ: bool,
        /// Peripheral increment mode
        pub pinc: bool,
        /// Memory increment mode
        pub minc: bool,
        /// Peripheral size
        pub psize: bool,
        /// Memory size
        pub msize: bool,
        /// Channel Priority level
        pub pl: bool,
        /// Memory to memory mode
        pub mem2mem: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x30u32) as *mut u32) };
        ReadonlyCache {
            en: ((value >> 0) & 0b1) > 0,
            tcie: ((value >> 1) & 0b1) > 0,
            htie: ((value >> 2) & 0b1) > 0,
            teie: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            circ: ((value >> 5) & 0b1) > 0,
            pinc: ((value >> 6) & 0b1) > 0,
            minc: ((value >> 7) & 0b1) > 0,
            psize: ((value >> 8) & 0b1) > 0,
            msize: ((value >> 10) & 0b1) > 0,
            pl: ((value >> 12) & 0b1) > 0,
            mem2mem: ((value >> 14) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x30u32) as *mut u32) };
        Cache {
            en: ((value >> 0) & 0b1) > 0,
            tcie: ((value >> 1) & 0b1) > 0,
            htie: ((value >> 2) & 0b1) > 0,
            teie: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            circ: ((value >> 5) & 0b1) > 0,
            pinc: ((value >> 6) & 0b1) > 0,
            minc: ((value >> 7) & 0b1) > 0,
            psize: ((value >> 8) & 0b1) > 0,
            msize: ((value >> 10) & 0b1) > 0,
            pl: ((value >> 12) & 0b1) > 0,
            mem2mem: ((value >> 14) & 0b1) > 0,
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
                | ((self.en as u32) << 0)
                | ((self.tcie as u32) << 1)
                | ((self.htie as u32) << 2)
                | ((self.teie as u32) << 3)
                | ((self.dir as u32) << 4)
                | ((self.circ as u32) << 5)
                | ((self.pinc as u32) << 6)
                | ((self.minc as u32) << 7)
                | ((self.psize as u32) << 8)
                | ((self.msize as u32) << 10)
                | ((self.pl as u32) << 12)
                | ((self.mem2mem as u32) << 14)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x30u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 3 number of data register
pub mod cndtr3 {
    pub struct ReadonlyCache {
        /// Number of data to transfer
        pub ndt: u16,
    }
    pub struct Cache {
        /// Number of data to transfer
        pub ndt: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x34u32) as *mut u32) };
        ReadonlyCache {
            ndt: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x34u32) as *mut u32) };
        Cache {
            ndt: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.ndt as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x34u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 3 peripheral address register
pub mod cpar3 {
    pub struct ReadonlyCache {
        /// Peripheral address
        pub pa: u32,
    }
    pub struct Cache {
        /// Peripheral address
        pub pa: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x38u32) as *mut u32) };
        ReadonlyCache {
            pa: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x38u32) as *mut u32) };
        Cache {
            pa: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.pa as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x38u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 3 memory address register
pub mod cmar3 {
    pub struct ReadonlyCache {
        /// Memory address
        pub ma: u32,
    }
    pub struct Cache {
        /// Memory address
        pub ma: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x3Cu32) as *mut u32) };
        ReadonlyCache {
            ma: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x3Cu32) as *mut u32) };
        Cache {
            ma: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.ma as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x3Cu32) as *mut u32, value) };
        }
    }
}
/// DMA channel configuration register (DMA_CCR)
pub mod ccr4 {
    pub struct ReadonlyCache {
        /// Channel enable
        pub en: bool,
        /// Transfer complete interrupt enable
        pub tcie: bool,
        /// Half Transfer interrupt enable
        pub htie: bool,
        /// Transfer error interrupt enable
        pub teie: bool,
        /// Data transfer direction
        pub dir: bool,
        /// Circular mode
        pub circ: bool,
        /// Peripheral increment mode
        pub pinc: bool,
        /// Memory increment mode
        pub minc: bool,
        /// Peripheral size
        pub psize: bool,
        /// Memory size
        pub msize: bool,
        /// Channel Priority level
        pub pl: bool,
        /// Memory to memory mode
        pub mem2mem: bool,
    }
    pub struct Cache {
        /// Channel enable
        pub en: bool,
        /// Transfer complete interrupt enable
        pub tcie: bool,
        /// Half Transfer interrupt enable
        pub htie: bool,
        /// Transfer error interrupt enable
        pub teie: bool,
        /// Data transfer direction
        pub dir: bool,
        /// Circular mode
        pub circ: bool,
        /// Peripheral increment mode
        pub pinc: bool,
        /// Memory increment mode
        pub minc: bool,
        /// Peripheral size
        pub psize: bool,
        /// Memory size
        pub msize: bool,
        /// Channel Priority level
        pub pl: bool,
        /// Memory to memory mode
        pub mem2mem: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x44u32) as *mut u32) };
        ReadonlyCache {
            en: ((value >> 0) & 0b1) > 0,
            tcie: ((value >> 1) & 0b1) > 0,
            htie: ((value >> 2) & 0b1) > 0,
            teie: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            circ: ((value >> 5) & 0b1) > 0,
            pinc: ((value >> 6) & 0b1) > 0,
            minc: ((value >> 7) & 0b1) > 0,
            psize: ((value >> 8) & 0b1) > 0,
            msize: ((value >> 10) & 0b1) > 0,
            pl: ((value >> 12) & 0b1) > 0,
            mem2mem: ((value >> 14) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x44u32) as *mut u32) };
        Cache {
            en: ((value >> 0) & 0b1) > 0,
            tcie: ((value >> 1) & 0b1) > 0,
            htie: ((value >> 2) & 0b1) > 0,
            teie: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            circ: ((value >> 5) & 0b1) > 0,
            pinc: ((value >> 6) & 0b1) > 0,
            minc: ((value >> 7) & 0b1) > 0,
            psize: ((value >> 8) & 0b1) > 0,
            msize: ((value >> 10) & 0b1) > 0,
            pl: ((value >> 12) & 0b1) > 0,
            mem2mem: ((value >> 14) & 0b1) > 0,
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
                | ((self.en as u32) << 0)
                | ((self.tcie as u32) << 1)
                | ((self.htie as u32) << 2)
                | ((self.teie as u32) << 3)
                | ((self.dir as u32) << 4)
                | ((self.circ as u32) << 5)
                | ((self.pinc as u32) << 6)
                | ((self.minc as u32) << 7)
                | ((self.psize as u32) << 8)
                | ((self.msize as u32) << 10)
                | ((self.pl as u32) << 12)
                | ((self.mem2mem as u32) << 14)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x44u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 4 number of data register
pub mod cndtr4 {
    pub struct ReadonlyCache {
        /// Number of data to transfer
        pub ndt: u16,
    }
    pub struct Cache {
        /// Number of data to transfer
        pub ndt: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x48u32) as *mut u32) };
        ReadonlyCache {
            ndt: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x48u32) as *mut u32) };
        Cache {
            ndt: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.ndt as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x48u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 4 peripheral address register
pub mod cpar4 {
    pub struct ReadonlyCache {
        /// Peripheral address
        pub pa: u32,
    }
    pub struct Cache {
        /// Peripheral address
        pub pa: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x4Cu32) as *mut u32) };
        ReadonlyCache {
            pa: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x4Cu32) as *mut u32) };
        Cache {
            pa: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.pa as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x4Cu32) as *mut u32, value) };
        }
    }
}
/// DMA channel 4 memory address register
pub mod cmar4 {
    pub struct ReadonlyCache {
        /// Memory address
        pub ma: u32,
    }
    pub struct Cache {
        /// Memory address
        pub ma: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x50u32) as *mut u32) };
        ReadonlyCache {
            ma: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x50u32) as *mut u32) };
        Cache {
            ma: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.ma as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x50u32) as *mut u32, value) };
        }
    }
}
/// DMA channel configuration register (DMA_CCR)
pub mod ccr5 {
    pub struct ReadonlyCache {
        /// Channel enable
        pub en: bool,
        /// Transfer complete interrupt enable
        pub tcie: bool,
        /// Half Transfer interrupt enable
        pub htie: bool,
        /// Transfer error interrupt enable
        pub teie: bool,
        /// Data transfer direction
        pub dir: bool,
        /// Circular mode
        pub circ: bool,
        /// Peripheral increment mode
        pub pinc: bool,
        /// Memory increment mode
        pub minc: bool,
        /// Peripheral size
        pub psize: bool,
        /// Memory size
        pub msize: bool,
        /// Channel Priority level
        pub pl: bool,
        /// Memory to memory mode
        pub mem2mem: bool,
    }
    pub struct Cache {
        /// Channel enable
        pub en: bool,
        /// Transfer complete interrupt enable
        pub tcie: bool,
        /// Half Transfer interrupt enable
        pub htie: bool,
        /// Transfer error interrupt enable
        pub teie: bool,
        /// Data transfer direction
        pub dir: bool,
        /// Circular mode
        pub circ: bool,
        /// Peripheral increment mode
        pub pinc: bool,
        /// Memory increment mode
        pub minc: bool,
        /// Peripheral size
        pub psize: bool,
        /// Memory size
        pub msize: bool,
        /// Channel Priority level
        pub pl: bool,
        /// Memory to memory mode
        pub mem2mem: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x58u32) as *mut u32) };
        ReadonlyCache {
            en: ((value >> 0) & 0b1) > 0,
            tcie: ((value >> 1) & 0b1) > 0,
            htie: ((value >> 2) & 0b1) > 0,
            teie: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            circ: ((value >> 5) & 0b1) > 0,
            pinc: ((value >> 6) & 0b1) > 0,
            minc: ((value >> 7) & 0b1) > 0,
            psize: ((value >> 8) & 0b1) > 0,
            msize: ((value >> 10) & 0b1) > 0,
            pl: ((value >> 12) & 0b1) > 0,
            mem2mem: ((value >> 14) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x58u32) as *mut u32) };
        Cache {
            en: ((value >> 0) & 0b1) > 0,
            tcie: ((value >> 1) & 0b1) > 0,
            htie: ((value >> 2) & 0b1) > 0,
            teie: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            circ: ((value >> 5) & 0b1) > 0,
            pinc: ((value >> 6) & 0b1) > 0,
            minc: ((value >> 7) & 0b1) > 0,
            psize: ((value >> 8) & 0b1) > 0,
            msize: ((value >> 10) & 0b1) > 0,
            pl: ((value >> 12) & 0b1) > 0,
            mem2mem: ((value >> 14) & 0b1) > 0,
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
                | ((self.en as u32) << 0)
                | ((self.tcie as u32) << 1)
                | ((self.htie as u32) << 2)
                | ((self.teie as u32) << 3)
                | ((self.dir as u32) << 4)
                | ((self.circ as u32) << 5)
                | ((self.pinc as u32) << 6)
                | ((self.minc as u32) << 7)
                | ((self.psize as u32) << 8)
                | ((self.msize as u32) << 10)
                | ((self.pl as u32) << 12)
                | ((self.mem2mem as u32) << 14)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x58u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 5 number of data register
pub mod cndtr5 {
    pub struct ReadonlyCache {
        /// Number of data to transfer
        pub ndt: u16,
    }
    pub struct Cache {
        /// Number of data to transfer
        pub ndt: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x5Cu32) as *mut u32) };
        ReadonlyCache {
            ndt: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x5Cu32) as *mut u32) };
        Cache {
            ndt: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.ndt as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x5Cu32) as *mut u32, value) };
        }
    }
}
/// DMA channel 5 peripheral address register
pub mod cpar5 {
    pub struct ReadonlyCache {
        /// Peripheral address
        pub pa: u32,
    }
    pub struct Cache {
        /// Peripheral address
        pub pa: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x60u32) as *mut u32) };
        ReadonlyCache {
            pa: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x60u32) as *mut u32) };
        Cache {
            pa: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.pa as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x60u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 5 memory address register
pub mod cmar5 {
    pub struct ReadonlyCache {
        /// Memory address
        pub ma: u32,
    }
    pub struct Cache {
        /// Memory address
        pub ma: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x64u32) as *mut u32) };
        ReadonlyCache {
            ma: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x64u32) as *mut u32) };
        Cache {
            ma: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.ma as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x64u32) as *mut u32, value) };
        }
    }
}
/// DMA channel configuration register (DMA_CCR)
pub mod ccr6 {
    pub struct ReadonlyCache {
        /// Channel enable
        pub en: bool,
        /// Transfer complete interrupt enable
        pub tcie: bool,
        /// Half Transfer interrupt enable
        pub htie: bool,
        /// Transfer error interrupt enable
        pub teie: bool,
        /// Data transfer direction
        pub dir: bool,
        /// Circular mode
        pub circ: bool,
        /// Peripheral increment mode
        pub pinc: bool,
        /// Memory increment mode
        pub minc: bool,
        /// Peripheral size
        pub psize: bool,
        /// Memory size
        pub msize: bool,
        /// Channel Priority level
        pub pl: bool,
        /// Memory to memory mode
        pub mem2mem: bool,
    }
    pub struct Cache {
        /// Channel enable
        pub en: bool,
        /// Transfer complete interrupt enable
        pub tcie: bool,
        /// Half Transfer interrupt enable
        pub htie: bool,
        /// Transfer error interrupt enable
        pub teie: bool,
        /// Data transfer direction
        pub dir: bool,
        /// Circular mode
        pub circ: bool,
        /// Peripheral increment mode
        pub pinc: bool,
        /// Memory increment mode
        pub minc: bool,
        /// Peripheral size
        pub psize: bool,
        /// Memory size
        pub msize: bool,
        /// Channel Priority level
        pub pl: bool,
        /// Memory to memory mode
        pub mem2mem: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x6Cu32) as *mut u32) };
        ReadonlyCache {
            en: ((value >> 0) & 0b1) > 0,
            tcie: ((value >> 1) & 0b1) > 0,
            htie: ((value >> 2) & 0b1) > 0,
            teie: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            circ: ((value >> 5) & 0b1) > 0,
            pinc: ((value >> 6) & 0b1) > 0,
            minc: ((value >> 7) & 0b1) > 0,
            psize: ((value >> 8) & 0b1) > 0,
            msize: ((value >> 10) & 0b1) > 0,
            pl: ((value >> 12) & 0b1) > 0,
            mem2mem: ((value >> 14) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x6Cu32) as *mut u32) };
        Cache {
            en: ((value >> 0) & 0b1) > 0,
            tcie: ((value >> 1) & 0b1) > 0,
            htie: ((value >> 2) & 0b1) > 0,
            teie: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            circ: ((value >> 5) & 0b1) > 0,
            pinc: ((value >> 6) & 0b1) > 0,
            minc: ((value >> 7) & 0b1) > 0,
            psize: ((value >> 8) & 0b1) > 0,
            msize: ((value >> 10) & 0b1) > 0,
            pl: ((value >> 12) & 0b1) > 0,
            mem2mem: ((value >> 14) & 0b1) > 0,
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
                | ((self.en as u32) << 0)
                | ((self.tcie as u32) << 1)
                | ((self.htie as u32) << 2)
                | ((self.teie as u32) << 3)
                | ((self.dir as u32) << 4)
                | ((self.circ as u32) << 5)
                | ((self.pinc as u32) << 6)
                | ((self.minc as u32) << 7)
                | ((self.psize as u32) << 8)
                | ((self.msize as u32) << 10)
                | ((self.pl as u32) << 12)
                | ((self.mem2mem as u32) << 14)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x6Cu32) as *mut u32, value) };
        }
    }
}
/// DMA channel 6 number of data register
pub mod cndtr6 {
    pub struct ReadonlyCache {
        /// Number of data to transfer
        pub ndt: u16,
    }
    pub struct Cache {
        /// Number of data to transfer
        pub ndt: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x70u32) as *mut u32) };
        ReadonlyCache {
            ndt: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x70u32) as *mut u32) };
        Cache {
            ndt: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.ndt as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x70u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 6 peripheral address register
pub mod cpar6 {
    pub struct ReadonlyCache {
        /// Peripheral address
        pub pa: u32,
    }
    pub struct Cache {
        /// Peripheral address
        pub pa: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x74u32) as *mut u32) };
        ReadonlyCache {
            pa: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x74u32) as *mut u32) };
        Cache {
            pa: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.pa as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x74u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 6 memory address register
pub mod cmar6 {
    pub struct ReadonlyCache {
        /// Memory address
        pub ma: u32,
    }
    pub struct Cache {
        /// Memory address
        pub ma: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x78u32) as *mut u32) };
        ReadonlyCache {
            ma: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x78u32) as *mut u32) };
        Cache {
            ma: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.ma as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x78u32) as *mut u32, value) };
        }
    }
}
/// DMA channel configuration register (DMA_CCR)
pub mod ccr7 {
    pub struct ReadonlyCache {
        /// Channel enable
        pub en: bool,
        /// Transfer complete interrupt enable
        pub tcie: bool,
        /// Half Transfer interrupt enable
        pub htie: bool,
        /// Transfer error interrupt enable
        pub teie: bool,
        /// Data transfer direction
        pub dir: bool,
        /// Circular mode
        pub circ: bool,
        /// Peripheral increment mode
        pub pinc: bool,
        /// Memory increment mode
        pub minc: bool,
        /// Peripheral size
        pub psize: bool,
        /// Memory size
        pub msize: bool,
        /// Channel Priority level
        pub pl: bool,
        /// Memory to memory mode
        pub mem2mem: bool,
    }
    pub struct Cache {
        /// Channel enable
        pub en: bool,
        /// Transfer complete interrupt enable
        pub tcie: bool,
        /// Half Transfer interrupt enable
        pub htie: bool,
        /// Transfer error interrupt enable
        pub teie: bool,
        /// Data transfer direction
        pub dir: bool,
        /// Circular mode
        pub circ: bool,
        /// Peripheral increment mode
        pub pinc: bool,
        /// Memory increment mode
        pub minc: bool,
        /// Peripheral size
        pub psize: bool,
        /// Memory size
        pub msize: bool,
        /// Channel Priority level
        pub pl: bool,
        /// Memory to memory mode
        pub mem2mem: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x80u32) as *mut u32) };
        ReadonlyCache {
            en: ((value >> 0) & 0b1) > 0,
            tcie: ((value >> 1) & 0b1) > 0,
            htie: ((value >> 2) & 0b1) > 0,
            teie: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            circ: ((value >> 5) & 0b1) > 0,
            pinc: ((value >> 6) & 0b1) > 0,
            minc: ((value >> 7) & 0b1) > 0,
            psize: ((value >> 8) & 0b1) > 0,
            msize: ((value >> 10) & 0b1) > 0,
            pl: ((value >> 12) & 0b1) > 0,
            mem2mem: ((value >> 14) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x80u32) as *mut u32) };
        Cache {
            en: ((value >> 0) & 0b1) > 0,
            tcie: ((value >> 1) & 0b1) > 0,
            htie: ((value >> 2) & 0b1) > 0,
            teie: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            circ: ((value >> 5) & 0b1) > 0,
            pinc: ((value >> 6) & 0b1) > 0,
            minc: ((value >> 7) & 0b1) > 0,
            psize: ((value >> 8) & 0b1) > 0,
            msize: ((value >> 10) & 0b1) > 0,
            pl: ((value >> 12) & 0b1) > 0,
            mem2mem: ((value >> 14) & 0b1) > 0,
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
                | ((self.en as u32) << 0)
                | ((self.tcie as u32) << 1)
                | ((self.htie as u32) << 2)
                | ((self.teie as u32) << 3)
                | ((self.dir as u32) << 4)
                | ((self.circ as u32) << 5)
                | ((self.pinc as u32) << 6)
                | ((self.minc as u32) << 7)
                | ((self.psize as u32) << 8)
                | ((self.msize as u32) << 10)
                | ((self.pl as u32) << 12)
                | ((self.mem2mem as u32) << 14)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x80u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 7 number of data register
pub mod cndtr7 {
    pub struct ReadonlyCache {
        /// Number of data to transfer
        pub ndt: u16,
    }
    pub struct Cache {
        /// Number of data to transfer
        pub ndt: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x84u32) as *mut u32) };
        ReadonlyCache {
            ndt: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x84u32) as *mut u32) };
        Cache {
            ndt: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.ndt as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x84u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 7 peripheral address register
pub mod cpar7 {
    pub struct ReadonlyCache {
        /// Peripheral address
        pub pa: u32,
    }
    pub struct Cache {
        /// Peripheral address
        pub pa: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x88u32) as *mut u32) };
        ReadonlyCache {
            pa: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x88u32) as *mut u32) };
        Cache {
            pa: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.pa as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x88u32) as *mut u32, value) };
        }
    }
}
/// DMA channel 7 memory address register
pub mod cmar7 {
    pub struct ReadonlyCache {
        /// Memory address
        pub ma: u32,
    }
    pub struct Cache {
        /// Memory address
        pub ma: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x8Cu32) as *mut u32) };
        ReadonlyCache {
            ma: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40020400u32 + 0x8Cu32) as *mut u32) };
        Cache {
            ma: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.ma as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40020400u32 + 0x8Cu32) as *mut u32, value) };
        }
    }
}
/// DMA2 channel1 global interrupt
pub const INTERRUPT_DMA2_CH1: u32 = 56;
/// DMA2 channel2 global interrupt
pub const INTERRUPT_DMA2_CH2: u32 = 57;
/// DMA2 channel3 global interrupt
pub const INTERRUPT_DMA2_CH3: u32 = 58;
/// DMA2 channel4 global interrupt
pub const INTERRUPT_DMA2_CH4: u32 = 59;
/// DMA2 channel5 global interrupt
pub const INTERRUPT_DMA2_CH5: u32 = 60;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" derivedFrom="DMA1">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40020400</baseAddress>
  <description>DMA controller 1</description>
  <groupName>DMA</groupName>
  <interrupt>
    <description>DMA2 channel1 global interrupt</description>
    <name>DMA2_CH1</name>
    <value>56</value>
  </interrupt>
  <interrupt>
    <description>DMA2 channel2 global interrupt</description>
    <name>DMA2_CH2</name>
    <value>57</value>
  </interrupt>
  <interrupt>
    <description>DMA2 channel3 global interrupt</description>
    <name>DMA2_CH3</name>
    <value>58</value>
  </interrupt>
  <interrupt>
    <description>DMA2 channel4 global interrupt</description>
    <name>DMA2_CH4</name>
    <value>59</value>
  </interrupt>
  <interrupt>
    <description>DMA2 channel5 global interrupt</description>
    <name>DMA2_CH5</name>
    <value>60</value>
  </interrupt>
  <name>DMA2</name>
  <registers>
    <register>
      <access>read-only</access>
      <addressOffset>0x0</addressOffset>
      <description>
                        DMA interrupt status register
                        (DMA_ISR)
                    </description>
      <displayName>ISR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 1 Global interrupt
                                flag
                            </description>
          <name>GIF1</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 1 Transfer Complete
                                flag
                            </description>
          <name>TCIF1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 1 Half Transfer Complete
                                flag
                            </description>
          <name>HTIF1</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 1 Transfer Error
                                flag
                            </description>
          <name>TEIF1</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 2 Global interrupt
                                flag
                            </description>
          <name>GIF2</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 2 Transfer Complete
                                flag
                            </description>
          <name>TCIF2</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 2 Half Transfer Complete
                                flag
                            </description>
          <name>HTIF2</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 2 Transfer Error
                                flag
                            </description>
          <name>TEIF2</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 3 Global interrupt
                                flag
                            </description>
          <name>GIF3</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 3 Transfer Complete
                                flag
                            </description>
          <name>TCIF3</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 3 Half Transfer Complete
                                flag
                            </description>
          <name>HTIF3</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 3 Transfer Error
                                flag
                            </description>
          <name>TEIF3</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 4 Global interrupt
                                flag
                            </description>
          <name>GIF4</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 4 Transfer Complete
                                flag
                            </description>
          <name>TCIF4</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 4 Half Transfer Complete
                                flag
                            </description>
          <name>HTIF4</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 4 Transfer Error
                                flag
                            </description>
          <name>TEIF4</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 5 Global interrupt
                                flag
                            </description>
          <name>GIF5</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 5 Transfer Complete
                                flag
                            </description>
          <name>TCIF5</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 5 Half Transfer Complete
                                flag
                            </description>
          <name>HTIF5</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 5 Transfer Error
                                flag
                            </description>
          <name>TEIF5</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 6 Global interrupt
                                flag
                            </description>
          <name>GIF6</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 6 Transfer Complete
                                flag
                            </description>
          <name>TCIF6</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 6 Half Transfer Complete
                                flag
                            </description>
          <name>HTIF6</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 6 Transfer Error
                                flag
                            </description>
          <name>TEIF6</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 7 Global interrupt
                                flag
                            </description>
          <name>GIF7</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 7 Transfer Complete
                                flag
                            </description>
          <name>TCIF7</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 7 Half Transfer Complete
                                flag
                            </description>
          <name>HTIF7</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 7 Transfer Error
                                flag
                            </description>
          <name>TEIF7</name>
        </field>
      </fields>
      <name>ISR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x4</addressOffset>
      <description>
                        DMA interrupt flag clear register
                        (DMA_IFCR)
                    </description>
      <displayName>IFCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 1 Global interrupt
                                clear
                            </description>
          <name>CGIF1</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 1 Transfer Complete
                                clear
                            </description>
          <name>CTCIF1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 1 Half Transfer
                                clear
                            </description>
          <name>CHTIF1</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 1 Transfer Error
                                clear
                            </description>
          <name>CTEIF1</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 2 Global interrupt
                                clear
                            </description>
          <name>CGIF2</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 2 Transfer Complete
                                clear
                            </description>
          <name>CTCIF2</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 2 Half Transfer
                                clear
                            </description>
          <name>CHTIF2</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 2 Transfer Error
                                clear
                            </description>
          <name>CTEIF2</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 3 Global interrupt
                                clear
                            </description>
          <name>CGIF3</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 3 Transfer Complete
                                clear
                            </description>
          <name>CTCIF3</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 3 Half Transfer
                                clear
                            </description>
          <name>CHTIF3</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 3 Transfer Error
                                clear
                            </description>
          <name>CTEIF3</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 4 Global interrupt
                                clear
                            </description>
          <name>CGIF4</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 4 Transfer Complete
                                clear
                            </description>
          <name>CTCIF4</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 4 Half Transfer
                                clear
                            </description>
          <name>CHTIF4</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 4 Transfer Error
                                clear
                            </description>
          <name>CTEIF4</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 5 Global interrupt
                                clear
                            </description>
          <name>CGIF5</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 5 Transfer Complete
                                clear
                            </description>
          <name>CTCIF5</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 5 Half Transfer
                                clear
                            </description>
          <name>CHTIF5</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 5 Transfer Error
                                clear
                            </description>
          <name>CTEIF5</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 6 Global interrupt
                                clear
                            </description>
          <name>CGIF6</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 6 Transfer Complete
                                clear
                            </description>
          <name>CTCIF6</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 6 Half Transfer
                                clear
                            </description>
          <name>CHTIF6</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 6 Transfer Error
                                clear
                            </description>
          <name>CTEIF6</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 7 Global interrupt
                                clear
                            </description>
          <name>CGIF7</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 7 Transfer Complete
                                clear
                            </description>
          <name>CTCIF7</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 7 Half Transfer
                                clear
                            </description>
          <name>CHTIF7</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Channel 7 Transfer Error
                                clear
                            </description>
          <name>CTEIF7</name>
        </field>
      </fields>
      <name>IFCR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>
                        DMA channel configuration register
                        (DMA_CCR)
                    </description>
      <displayName>CCR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Channel enable</description>
          <name>EN</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer complete interrupt
                                enable
                            </description>
          <name>TCIE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Half Transfer interrupt
                                enable
                            </description>
          <name>HTIE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer error interrupt
                                enable
                            </description>
          <name>TEIE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data transfer direction</description>
          <name>DIR</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Circular mode</description>
          <name>CIRC</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Peripheral increment mode</description>
          <name>PINC</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Memory increment mode</description>
          <name>MINC</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Peripheral size</description>
          <name>PSIZE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Memory size</description>
          <name>MSIZE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Channel Priority level</description>
          <name>PL</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Memory to memory mode</description>
          <name>MEM2MEM</name>
        </field>
      </fields>
      <name>CCR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>
                        DMA channel 1 number of data
                        register
                    </description>
      <displayName>CNDTR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Number of data to transfer</description>
          <name>NDT</name>
        </field>
      </fields>
      <name>CNDTR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>
                        DMA channel 1 peripheral address
                        register
                    </description>
      <displayName>CPAR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Peripheral address</description>
          <name>PA</name>
        </field>
      </fields>
      <name>CPAR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>
                        DMA channel 1 memory address
                        register
                    </description>
      <displayName>CMAR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Memory address</description>
          <name>MA</name>
        </field>
      </fields>
      <name>CMAR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>
                        DMA channel configuration register
                        (DMA_CCR)
                    </description>
      <displayName>CCR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Channel enable</description>
          <name>EN</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer complete interrupt
                                enable
                            </description>
          <name>TCIE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Half Transfer interrupt
                                enable
                            </description>
          <name>HTIE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer error interrupt
                                enable
                            </description>
          <name>TEIE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data transfer direction</description>
          <name>DIR</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Circular mode</description>
          <name>CIRC</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Peripheral increment mode</description>
          <name>PINC</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Memory increment mode</description>
          <name>MINC</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Peripheral size</description>
          <name>PSIZE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Memory size</description>
          <name>MSIZE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Channel Priority level</description>
          <name>PL</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Memory to memory mode</description>
          <name>MEM2MEM</name>
        </field>
      </fields>
      <name>CCR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>
                        DMA channel 2 number of data
                        register
                    </description>
      <displayName>CNDTR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Number of data to transfer</description>
          <name>NDT</name>
        </field>
      </fields>
      <name>CNDTR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x24</addressOffset>
      <description>
                        DMA channel 2 peripheral address
                        register
                    </description>
      <displayName>CPAR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Peripheral address</description>
          <name>PA</name>
        </field>
      </fields>
      <name>CPAR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28</addressOffset>
      <description>
                        DMA channel 2 memory address
                        register
                    </description>
      <displayName>CMAR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Memory address</description>
          <name>MA</name>
        </field>
      </fields>
      <name>CMAR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x30</addressOffset>
      <description>
                        DMA channel configuration register
                        (DMA_CCR)
                    </description>
      <displayName>CCR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Channel enable</description>
          <name>EN</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer complete interrupt
                                enable
                            </description>
          <name>TCIE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Half Transfer interrupt
                                enable
                            </description>
          <name>HTIE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer error interrupt
                                enable
                            </description>
          <name>TEIE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data transfer direction</description>
          <name>DIR</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Circular mode</description>
          <name>CIRC</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Peripheral increment mode</description>
          <name>PINC</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Memory increment mode</description>
          <name>MINC</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Peripheral size</description>
          <name>PSIZE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Memory size</description>
          <name>MSIZE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Channel Priority level</description>
          <name>PL</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Memory to memory mode</description>
          <name>MEM2MEM</name>
        </field>
      </fields>
      <name>CCR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x34</addressOffset>
      <description>
                        DMA channel 3 number of data
                        register
                    </description>
      <displayName>CNDTR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Number of data to transfer</description>
          <name>NDT</name>
        </field>
      </fields>
      <name>CNDTR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x38</addressOffset>
      <description>
                        DMA channel 3 peripheral address
                        register
                    </description>
      <displayName>CPAR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Peripheral address</description>
          <name>PA</name>
        </field>
      </fields>
      <name>CPAR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x3C</addressOffset>
      <description>
                        DMA channel 3 memory address
                        register
                    </description>
      <displayName>CMAR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Memory address</description>
          <name>MA</name>
        </field>
      </fields>
      <name>CMAR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x44</addressOffset>
      <description>
                        DMA channel configuration register
                        (DMA_CCR)
                    </description>
      <displayName>CCR4</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Channel enable</description>
          <name>EN</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer complete interrupt
                                enable
                            </description>
          <name>TCIE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Half Transfer interrupt
                                enable
                            </description>
          <name>HTIE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer error interrupt
                                enable
                            </description>
          <name>TEIE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data transfer direction</description>
          <name>DIR</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Circular mode</description>
          <name>CIRC</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Peripheral increment mode</description>
          <name>PINC</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Memory increment mode</description>
          <name>MINC</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Peripheral size</description>
          <name>PSIZE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Memory size</description>
          <name>MSIZE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Channel Priority level</description>
          <name>PL</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Memory to memory mode</description>
          <name>MEM2MEM</name>
        </field>
      </fields>
      <name>CCR4</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x48</addressOffset>
      <description>
                        DMA channel 4 number of data
                        register
                    </description>
      <displayName>CNDTR4</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Number of data to transfer</description>
          <name>NDT</name>
        </field>
      </fields>
      <name>CNDTR4</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4C</addressOffset>
      <description>
                        DMA channel 4 peripheral address
                        register
                    </description>
      <displayName>CPAR4</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Peripheral address</description>
          <name>PA</name>
        </field>
      </fields>
      <name>CPAR4</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x50</addressOffset>
      <description>
                        DMA channel 4 memory address
                        register
                    </description>
      <displayName>CMAR4</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Memory address</description>
          <name>MA</name>
        </field>
      </fields>
      <name>CMAR4</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x58</addressOffset>
      <description>
                        DMA channel configuration register
                        (DMA_CCR)
                    </description>
      <displayName>CCR5</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Channel enable</description>
          <name>EN</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer complete interrupt
                                enable
                            </description>
          <name>TCIE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Half Transfer interrupt
                                enable
                            </description>
          <name>HTIE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer error interrupt
                                enable
                            </description>
          <name>TEIE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data transfer direction</description>
          <name>DIR</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Circular mode</description>
          <name>CIRC</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Peripheral increment mode</description>
          <name>PINC</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Memory increment mode</description>
          <name>MINC</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Peripheral size</description>
          <name>PSIZE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Memory size</description>
          <name>MSIZE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Channel Priority level</description>
          <name>PL</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Memory to memory mode</description>
          <name>MEM2MEM</name>
        </field>
      </fields>
      <name>CCR5</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x5C</addressOffset>
      <description>
                        DMA channel 5 number of data
                        register
                    </description>
      <displayName>CNDTR5</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Number of data to transfer</description>
          <name>NDT</name>
        </field>
      </fields>
      <name>CNDTR5</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x60</addressOffset>
      <description>
                        DMA channel 5 peripheral address
                        register
                    </description>
      <displayName>CPAR5</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Peripheral address</description>
          <name>PA</name>
        </field>
      </fields>
      <name>CPAR5</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x64</addressOffset>
      <description>
                        DMA channel 5 memory address
                        register
                    </description>
      <displayName>CMAR5</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Memory address</description>
          <name>MA</name>
        </field>
      </fields>
      <name>CMAR5</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x6C</addressOffset>
      <description>
                        DMA channel configuration register
                        (DMA_CCR)
                    </description>
      <displayName>CCR6</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Channel enable</description>
          <name>EN</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer complete interrupt
                                enable
                            </description>
          <name>TCIE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Half Transfer interrupt
                                enable
                            </description>
          <name>HTIE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer error interrupt
                                enable
                            </description>
          <name>TEIE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data transfer direction</description>
          <name>DIR</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Circular mode</description>
          <name>CIRC</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Peripheral increment mode</description>
          <name>PINC</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Memory increment mode</description>
          <name>MINC</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Peripheral size</description>
          <name>PSIZE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Memory size</description>
          <name>MSIZE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Channel Priority level</description>
          <name>PL</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Memory to memory mode</description>
          <name>MEM2MEM</name>
        </field>
      </fields>
      <name>CCR6</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x70</addressOffset>
      <description>
                        DMA channel 6 number of data
                        register
                    </description>
      <displayName>CNDTR6</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Number of data to transfer</description>
          <name>NDT</name>
        </field>
      </fields>
      <name>CNDTR6</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x74</addressOffset>
      <description>
                        DMA channel 6 peripheral address
                        register
                    </description>
      <displayName>CPAR6</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Peripheral address</description>
          <name>PA</name>
        </field>
      </fields>
      <name>CPAR6</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x78</addressOffset>
      <description>
                        DMA channel 6 memory address
                        register
                    </description>
      <displayName>CMAR6</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Memory address</description>
          <name>MA</name>
        </field>
      </fields>
      <name>CMAR6</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x80</addressOffset>
      <description>
                        DMA channel configuration register
                        (DMA_CCR)
                    </description>
      <displayName>CCR7</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Channel enable</description>
          <name>EN</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer complete interrupt
                                enable
                            </description>
          <name>TCIE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Half Transfer interrupt
                                enable
                            </description>
          <name>HTIE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transfer error interrupt
                                enable
                            </description>
          <name>TEIE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data transfer direction</description>
          <name>DIR</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Circular mode</description>
          <name>CIRC</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Peripheral increment mode</description>
          <name>PINC</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Memory increment mode</description>
          <name>MINC</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Peripheral size</description>
          <name>PSIZE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Memory size</description>
          <name>MSIZE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Channel Priority level</description>
          <name>PL</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Memory to memory mode</description>
          <name>MEM2MEM</name>
        </field>
      </fields>
      <name>CCR7</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x84</addressOffset>
      <description>
                        DMA channel 7 number of data
                        register
                    </description>
      <displayName>CNDTR7</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Number of data to transfer</description>
          <name>NDT</name>
        </field>
      </fields>
      <name>CNDTR7</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x88</addressOffset>
      <description>
                        DMA channel 7 peripheral address
                        register
                    </description>
      <displayName>CPAR7</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Peripheral address</description>
          <name>PA</name>
        </field>
      </fields>
      <name>CPAR7</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8C</addressOffset>
      <description>
                        DMA channel 7 memory address
                        register
                    </description>
      <displayName>CMAR7</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Memory address</description>
          <name>MA</name>
        </field>
      </fields>
      <name>CMAR7</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
