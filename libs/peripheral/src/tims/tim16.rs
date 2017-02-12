/// control register 1
pub mod cr1 {
    pub struct ReadonlyCache {
        /// Counter enable
        pub cen: bool,
        /// Update disable
        pub udis: bool,
        /// Update request source
        pub urs: bool,
        /// One-pulse mode
        pub opm: bool,
        /// Auto-reload preload enable
        pub arpe: bool,
        /// Clock division
        pub ckd: bool,
        /// UIF status bit remapping
        pub uifremap: bool,
    }
    pub struct Cache {
        /// Counter enable
        pub cen: bool,
        /// Update disable
        pub udis: bool,
        /// Update request source
        pub urs: bool,
        /// One-pulse mode
        pub opm: bool,
        /// Auto-reload preload enable
        pub arpe: bool,
        /// Clock division
        pub ckd: bool,
        /// UIF status bit remapping
        pub uifremap: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            cen: ((value >> 0) & 0b1) > 0,
            udis: ((value >> 1) & 0b1) > 0,
            urs: ((value >> 2) & 0b1) > 0,
            opm: ((value >> 3) & 0b1) > 0,
            arpe: ((value >> 7) & 0b1) > 0,
            ckd: ((value >> 8) & 0b1) > 0,
            uifremap: ((value >> 11) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x0u32) as *mut u32) };
        Cache {
            cen: ((value >> 0) & 0b1) > 0,
            udis: ((value >> 1) & 0b1) > 0,
            urs: ((value >> 2) & 0b1) > 0,
            opm: ((value >> 3) & 0b1) > 0,
            arpe: ((value >> 7) & 0b1) > 0,
            ckd: ((value >> 8) & 0b1) > 0,
            uifremap: ((value >> 11) & 0b1) > 0,
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
                | ((self.cen as u32) << 0)
                | ((self.udis as u32) << 1)
                | ((self.urs as u32) << 2)
                | ((self.opm as u32) << 3)
                | ((self.arpe as u32) << 7)
                | ((self.ckd as u32) << 8)
                | ((self.uifremap as u32) << 11)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// control register 2
pub mod cr2 {
    pub struct ReadonlyCache {
        /// Output Idle state 1
        pub ois1n: bool,
        /// Output Idle state 1
        pub ois1: bool,
        /// Capture/compare DMA selection
        pub ccds: bool,
        /// Capture/compare control update selection
        pub ccus: bool,
        /// Capture/compare preloaded control
        pub ccpc: bool,
    }
    pub struct Cache {
        /// Output Idle state 1
        pub ois1n: bool,
        /// Output Idle state 1
        pub ois1: bool,
        /// Capture/compare DMA selection
        pub ccds: bool,
        /// Capture/compare control update selection
        pub ccus: bool,
        /// Capture/compare preloaded control
        pub ccpc: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            ois1n: ((value >> 9) & 0b1) > 0,
            ois1: ((value >> 8) & 0b1) > 0,
            ccds: ((value >> 3) & 0b1) > 0,
            ccus: ((value >> 2) & 0b1) > 0,
            ccpc: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x4u32) as *mut u32) };
        Cache {
            ois1n: ((value >> 9) & 0b1) > 0,
            ois1: ((value >> 8) & 0b1) > 0,
            ccds: ((value >> 3) & 0b1) > 0,
            ccus: ((value >> 2) & 0b1) > 0,
            ccpc: ((value >> 0) & 0b1) > 0,
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
                | ((self.ois1n as u32) << 9)
                | ((self.ois1 as u32) << 8)
                | ((self.ccds as u32) << 3)
                | ((self.ccus as u32) << 2)
                | ((self.ccpc as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// DMA/Interrupt enable register
pub mod dier {
    pub struct ReadonlyCache {
        /// Update interrupt enable
        pub uie: bool,
        /// Capture/Compare 1 interrupt enable
        pub cc1ie: bool,
        /// COM interrupt enable
        pub comie: bool,
        /// Trigger interrupt enable
        pub tie: bool,
        /// Break interrupt enable
        pub bie: bool,
        /// Update DMA request enable
        pub ude: bool,
        /// Capture/Compare 1 DMA request enable
        pub cc1de: bool,
        /// COM DMA request enable
        pub comde: bool,
        /// Trigger DMA request enable
        pub tde: bool,
    }
    pub struct Cache {
        /// Update interrupt enable
        pub uie: bool,
        /// Capture/Compare 1 interrupt enable
        pub cc1ie: bool,
        /// COM interrupt enable
        pub comie: bool,
        /// Trigger interrupt enable
        pub tie: bool,
        /// Break interrupt enable
        pub bie: bool,
        /// Update DMA request enable
        pub ude: bool,
        /// Capture/Compare 1 DMA request enable
        pub cc1de: bool,
        /// COM DMA request enable
        pub comde: bool,
        /// Trigger DMA request enable
        pub tde: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            uie: ((value >> 0) & 0b1) > 0,
            cc1ie: ((value >> 1) & 0b1) > 0,
            comie: ((value >> 5) & 0b1) > 0,
            tie: ((value >> 6) & 0b1) > 0,
            bie: ((value >> 7) & 0b1) > 0,
            ude: ((value >> 8) & 0b1) > 0,
            cc1de: ((value >> 9) & 0b1) > 0,
            comde: ((value >> 13) & 0b1) > 0,
            tde: ((value >> 14) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0xCu32) as *mut u32) };
        Cache {
            uie: ((value >> 0) & 0b1) > 0,
            cc1ie: ((value >> 1) & 0b1) > 0,
            comie: ((value >> 5) & 0b1) > 0,
            tie: ((value >> 6) & 0b1) > 0,
            bie: ((value >> 7) & 0b1) > 0,
            ude: ((value >> 8) & 0b1) > 0,
            cc1de: ((value >> 9) & 0b1) > 0,
            comde: ((value >> 13) & 0b1) > 0,
            tde: ((value >> 14) & 0b1) > 0,
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
                | ((self.uie as u32) << 0)
                | ((self.cc1ie as u32) << 1)
                | ((self.comie as u32) << 5)
                | ((self.tie as u32) << 6)
                | ((self.bie as u32) << 7)
                | ((self.ude as u32) << 8)
                | ((self.cc1de as u32) << 9)
                | ((self.comde as u32) << 13)
                | ((self.tde as u32) << 14)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// status register
pub mod sr {
    pub struct ReadonlyCache {
        /// Capture/Compare 1 overcapture flag
        pub cc1of: bool,
        /// Break interrupt flag
        pub bif: bool,
        /// Trigger interrupt flag
        pub tif: bool,
        /// COM interrupt flag
        pub comif: bool,
        /// Capture/compare 1 interrupt flag
        pub cc1if: bool,
        /// Update interrupt flag
        pub uif: bool,
    }
    pub struct Cache {
        /// Capture/Compare 1 overcapture flag
        pub cc1of: bool,
        /// Break interrupt flag
        pub bif: bool,
        /// Trigger interrupt flag
        pub tif: bool,
        /// COM interrupt flag
        pub comif: bool,
        /// Capture/compare 1 interrupt flag
        pub cc1if: bool,
        /// Update interrupt flag
        pub uif: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            cc1of: ((value >> 9) & 0b1) > 0,
            bif: ((value >> 7) & 0b1) > 0,
            tif: ((value >> 6) & 0b1) > 0,
            comif: ((value >> 5) & 0b1) > 0,
            cc1if: ((value >> 1) & 0b1) > 0,
            uif: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x10u32) as *mut u32) };
        Cache {
            cc1of: ((value >> 9) & 0b1) > 0,
            bif: ((value >> 7) & 0b1) > 0,
            tif: ((value >> 6) & 0b1) > 0,
            comif: ((value >> 5) & 0b1) > 0,
            cc1if: ((value >> 1) & 0b1) > 0,
            uif: ((value >> 0) & 0b1) > 0,
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
                | ((self.cc1of as u32) << 9)
                | ((self.bif as u32) << 7)
                | ((self.tif as u32) << 6)
                | ((self.comif as u32) << 5)
                | ((self.cc1if as u32) << 1)
                | ((self.uif as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x10u32) as *mut u32, value) };
        }
    }
}
/// event generation register
pub mod egr {
    /// Break generation
    /// Access: write-only, Width: 1, Offset: 7
    /// Set Break generation
    pub fn bg(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x14u32) as *mut u32, value) };
    }
    /// Trigger generation
    /// Access: write-only, Width: 1, Offset: 6
    /// Set Trigger generation
    pub fn tg(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x14u32) as *mut u32, value) };
    }
    /// Capture/Compare control update generation
    /// Access: write-only, Width: 1, Offset: 5
    /// Set Capture/Compare control update generation
    pub fn comg(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x14u32) as *mut u32, value) };
    }
    /// Capture/compare 1 generation
    /// Access: write-only, Width: 1, Offset: 1
    /// Set Capture/compare 1 generation
    pub fn cc1g(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x14u32) as *mut u32, value) };
    }
    /// Update generation
    /// Access: write-only, Width: 1, Offset: 0
    /// Set Update generation
    pub fn ug(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x14u32) as *mut u32, value) };
    }
}
/// capture/compare mode register (output mode)
pub mod ccmr1_output {
    pub struct ReadonlyCache {
        /// Capture/Compare 1 selection
        pub cc1s: u8,
        /// Output Compare 1 fast enable
        pub oc1fe: u8,
        /// Output Compare 1 preload enable
        pub oc1pe: u8,
        /// Output Compare 1 mode
        pub oc1m: u8,
        /// Output Compare 1 mode
        pub oc1m_3: u8,
    }
    pub struct Cache {
        /// Capture/Compare 1 selection
        pub cc1s: u8,
        /// Output Compare 1 fast enable
        pub oc1fe: u8,
        /// Output Compare 1 preload enable
        pub oc1pe: u8,
        /// Output Compare 1 mode
        pub oc1m: u8,
        /// Output Compare 1 mode
        pub oc1m_3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x18u32) as *mut u32) };
        ReadonlyCache {
            cc1s: ((value >> 0) & 0b11) as u8,
            oc1fe: ((value >> 2) & 0b11) as u8,
            oc1pe: ((value >> 3) & 0b11) as u8,
            oc1m: ((value >> 4) & 0b11) as u8,
            oc1m_3: ((value >> 16) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x18u32) as *mut u32) };
        Cache {
            cc1s: ((value >> 0) & 0b11) as u8,
            oc1fe: ((value >> 2) & 0b11) as u8,
            oc1pe: ((value >> 3) & 0b11) as u8,
            oc1m: ((value >> 4) & 0b11) as u8,
            oc1m_3: ((value >> 16) & 0b11) as u8,
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
                | ((self.cc1s as u32) << 0)
                | ((self.oc1fe as u32) << 2)
                | ((self.oc1pe as u32) << 3)
                | ((self.oc1m as u32) << 4)
                | ((self.oc1m_3 as u32) << 16)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x18u32) as *mut u32, value) };
        }
    }
}
/// capture/compare mode register 1 (input mode)
pub mod ccmr1_input {
    pub struct ReadonlyCache {
        /// Input capture 1 filter
        pub ic1f: u8,
        /// Input capture 1 prescaler
        pub ic1psc: u8,
        /// Capture/Compare 1 selection
        pub cc1s: u8,
    }
    pub struct Cache {
        /// Input capture 1 filter
        pub ic1f: u8,
        /// Input capture 1 prescaler
        pub ic1psc: u8,
        /// Capture/Compare 1 selection
        pub cc1s: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x18u32) as *mut u32) };
        ReadonlyCache {
            ic1f: ((value >> 4) & 0b1111) as u8,
            ic1psc: ((value >> 2) & 0b1111) as u8,
            cc1s: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x18u32) as *mut u32) };
        Cache {
            ic1f: ((value >> 4) & 0b1111) as u8,
            ic1psc: ((value >> 2) & 0b1111) as u8,
            cc1s: ((value >> 0) & 0b1111) as u8,
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
                | ((self.ic1f as u32) << 4)
                | ((self.ic1psc as u32) << 2)
                | ((self.cc1s as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x18u32) as *mut u32, value) };
        }
    }
}
/// capture/compare enable register
pub mod ccer {
    pub struct ReadonlyCache {
        /// Capture/Compare 1 output Polarity
        pub cc1np: bool,
        /// Capture/Compare 1 complementary output enable
        pub cc1ne: bool,
        /// Capture/Compare 1 output Polarity
        pub cc1p: bool,
        /// Capture/Compare 1 output enable
        pub cc1e: bool,
    }
    pub struct Cache {
        /// Capture/Compare 1 output Polarity
        pub cc1np: bool,
        /// Capture/Compare 1 complementary output enable
        pub cc1ne: bool,
        /// Capture/Compare 1 output Polarity
        pub cc1p: bool,
        /// Capture/Compare 1 output enable
        pub cc1e: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x20u32) as *mut u32) };
        ReadonlyCache {
            cc1np: ((value >> 3) & 0b1) > 0,
            cc1ne: ((value >> 2) & 0b1) > 0,
            cc1p: ((value >> 1) & 0b1) > 0,
            cc1e: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x20u32) as *mut u32) };
        Cache {
            cc1np: ((value >> 3) & 0b1) > 0,
            cc1ne: ((value >> 2) & 0b1) > 0,
            cc1p: ((value >> 1) & 0b1) > 0,
            cc1e: ((value >> 0) & 0b1) > 0,
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
                | ((self.cc1np as u32) << 3)
                | ((self.cc1ne as u32) << 2)
                | ((self.cc1p as u32) << 1)
                | ((self.cc1e as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x20u32) as *mut u32, value) };
        }
    }
}
/// counter
pub mod cnt {
    /// counter value
    /// Access: read-write, Width: 16, Offset: 0
    /// Set counter value
    pub fn set_cnt(value: u16) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x24u32) as *mut u32, value) };
    }
    /// Get counter value
    pub fn get_cnt() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
    /// UIF Copy
    /// Access: read-only, Width: 1, Offset: 31
    /// Get UIF Copy
    pub fn uifcpy() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
}
/// prescaler
pub mod psc {
    pub struct ReadonlyCache {
        /// Prescaler value
        pub psc: u16,
    }
    pub struct Cache {
        /// Prescaler value
        pub psc: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x28u32) as *mut u32) };
        ReadonlyCache {
            psc: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x28u32) as *mut u32) };
        Cache {
            psc: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.psc as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x28u32) as *mut u32, value) };
        }
    }
}
/// auto-reload register
pub mod arr {
    pub struct ReadonlyCache {
        /// Auto-reload value
        pub arr: u16,
    }
    pub struct Cache {
        /// Auto-reload value
        pub arr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x2Cu32) as *mut u32) };
        ReadonlyCache {
            arr: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x2Cu32) as *mut u32) };
        Cache {
            arr: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.arr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x2Cu32) as *mut u32, value) };
        }
    }
}
/// repetition counter register
pub mod rcr {
    pub struct ReadonlyCache {
        /// Repetition counter value
        pub rep: u8,
    }
    pub struct Cache {
        /// Repetition counter value
        pub rep: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x30u32) as *mut u32) };
        ReadonlyCache {
            rep: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x30u32) as *mut u32) };
        Cache {
            rep: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.rep as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x30u32) as *mut u32, value) };
        }
    }
}
/// capture/compare register 1
pub mod ccr1 {
    pub struct ReadonlyCache {
        /// Capture/Compare 1 value
        pub ccr1: u16,
    }
    pub struct Cache {
        /// Capture/Compare 1 value
        pub ccr1: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x34u32) as *mut u32) };
        ReadonlyCache {
            ccr1: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x34u32) as *mut u32) };
        Cache {
            ccr1: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.ccr1 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x34u32) as *mut u32, value) };
        }
    }
}
/// break and dead-time register
pub mod bdtr {
    pub struct ReadonlyCache {
        /// Dead-time generator setup
        pub dtg: u8,
        /// Lock configuration
        pub lock: u8,
        /// Off-state selection for Idle mode
        pub ossi: u8,
        /// Off-state selection for Run mode
        pub ossr: u8,
        /// Break enable
        pub bke: u8,
        /// Break polarity
        pub bkp: u8,
        /// Automatic output enable
        pub aoe: u8,
        /// Main output enable
        pub moe: u8,
        /// Break filter
        pub bkf: u8,
    }
    pub struct Cache {
        /// Dead-time generator setup
        pub dtg: u8,
        /// Lock configuration
        pub lock: u8,
        /// Off-state selection for Idle mode
        pub ossi: u8,
        /// Off-state selection for Run mode
        pub ossr: u8,
        /// Break enable
        pub bke: u8,
        /// Break polarity
        pub bkp: u8,
        /// Automatic output enable
        pub aoe: u8,
        /// Main output enable
        pub moe: u8,
        /// Break filter
        pub bkf: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x44u32) as *mut u32) };
        ReadonlyCache {
            dtg: ((value >> 0) & 0b11111111) as u8,
            lock: ((value >> 8) & 0b11111111) as u8,
            ossi: ((value >> 10) & 0b11111111) as u8,
            ossr: ((value >> 11) & 0b11111111) as u8,
            bke: ((value >> 12) & 0b11111111) as u8,
            bkp: ((value >> 13) & 0b11111111) as u8,
            aoe: ((value >> 14) & 0b11111111) as u8,
            moe: ((value >> 15) & 0b11111111) as u8,
            bkf: ((value >> 16) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x44u32) as *mut u32) };
        Cache {
            dtg: ((value >> 0) & 0b11111111) as u8,
            lock: ((value >> 8) & 0b11111111) as u8,
            ossi: ((value >> 10) & 0b11111111) as u8,
            ossr: ((value >> 11) & 0b11111111) as u8,
            bke: ((value >> 12) & 0b11111111) as u8,
            bkp: ((value >> 13) & 0b11111111) as u8,
            aoe: ((value >> 14) & 0b11111111) as u8,
            moe: ((value >> 15) & 0b11111111) as u8,
            bkf: ((value >> 16) & 0b11111111) as u8,
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
                | ((self.dtg as u32) << 0)
                | ((self.lock as u32) << 8)
                | ((self.ossi as u32) << 10)
                | ((self.ossr as u32) << 11)
                | ((self.bke as u32) << 12)
                | ((self.bkp as u32) << 13)
                | ((self.aoe as u32) << 14)
                | ((self.moe as u32) << 15)
                | ((self.bkf as u32) << 16)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x44u32) as *mut u32, value) };
        }
    }
}
/// DMA control register
pub mod dcr {
    pub struct ReadonlyCache {
        /// DMA burst length
        pub dbl: u8,
        /// DMA base address
        pub dba: u8,
    }
    pub struct Cache {
        /// DMA burst length
        pub dbl: u8,
        /// DMA base address
        pub dba: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x48u32) as *mut u32) };
        ReadonlyCache {
            dbl: ((value >> 8) & 0b11111) as u8,
            dba: ((value >> 0) & 0b11111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x48u32) as *mut u32) };
        Cache {
            dbl: ((value >> 8) & 0b11111) as u8,
            dba: ((value >> 0) & 0b11111) as u8,
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
                | ((self.dbl as u32) << 8)
                | ((self.dba as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x48u32) as *mut u32, value) };
        }
    }
}
/// DMA address for full transfer
pub mod dmar {
    pub struct ReadonlyCache {
        /// DMA register for burst accesses
        pub dmab: u16,
    }
    pub struct Cache {
        /// DMA register for burst accesses
        pub dmab: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x4Cu32) as *mut u32) };
        ReadonlyCache {
            dmab: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014400u32 + 0x4Cu32) as *mut u32) };
        Cache {
            dmab: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.dmab as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014400u32 + 0x4Cu32) as *mut u32, value) };
        }
    }
}
/// option register
pub mod or {
}
/// TIM1 Update/TIM16 global interrupts
pub const INTERRUPT_TIM1_UP_TIM16: u32 = 25;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40014400</baseAddress>
  <description>General-purpose-timers</description>
  <groupName>TIMs</groupName>
  <interrupt>
    <description>
                    TIM1 Update/TIM16 global
                    interrupts
                </description>
    <name>TIM1_UP_TIM16</name>
    <value>25</value>
  </interrupt>
  <name>TIM16</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>control register 1</description>
      <displayName>CR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Counter enable</description>
          <name>CEN</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update disable</description>
          <name>UDIS</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update request source</description>
          <name>URS</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>One-pulse mode</description>
          <name>OPM</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Auto-reload preload enable</description>
          <name>ARPE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Clock division</description>
          <name>CKD</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>UIF status bit remapping</description>
          <name>UIFREMAP</name>
        </field>
      </fields>
      <name>CR1</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>control register 2</description>
      <displayName>CR2</displayName>
      <fields>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Idle state 1</description>
          <name>OIS1N</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Idle state 1</description>
          <name>OIS1</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/compare DMA
                                selection
                            </description>
          <name>CCDS</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/compare control update
                                selection
                            </description>
          <name>CCUS</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/compare preloaded
                                control
                            </description>
          <name>CCPC</name>
        </field>
      </fields>
      <name>CR2</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>DMA/Interrupt enable register</description>
      <displayName>DIER</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update interrupt enable</description>
          <name>UIE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 1 interrupt
                                enable
                            </description>
          <name>CC1IE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>COM interrupt enable</description>
          <name>COMIE</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Trigger interrupt enable</description>
          <name>TIE</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Break interrupt enable</description>
          <name>BIE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update DMA request enable</description>
          <name>UDE</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 1 DMA request
                                enable
                            </description>
          <name>CC1DE</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>COM DMA request enable</description>
          <name>COMDE</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Trigger DMA request enable</description>
          <name>TDE</name>
        </field>
      </fields>
      <name>DIER</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>status register</description>
      <displayName>SR</displayName>
      <fields>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 1 overcapture
                                flag
                            </description>
          <name>CC1OF</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Break interrupt flag</description>
          <name>BIF</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Trigger interrupt flag</description>
          <name>TIF</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>COM interrupt flag</description>
          <name>COMIF</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/compare 1 interrupt
                                flag
                            </description>
          <name>CC1IF</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update interrupt flag</description>
          <name>UIF</name>
        </field>
      </fields>
      <name>SR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x14</addressOffset>
      <description>event generation register</description>
      <displayName>EGR</displayName>
      <fields>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Break generation</description>
          <name>BG</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Trigger generation</description>
          <name>TG</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare control update
                                generation
                            </description>
          <name>COMG</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/compare 1
                                generation
                            </description>
          <name>CC1G</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update generation</description>
          <name>UG</name>
        </field>
      </fields>
      <name>EGR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>
                        capture/compare mode register (output
                        mode)
                    </description>
      <displayName>CCMR1_Output</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Capture/Compare 1
                                selection
                            </description>
          <name>CC1S</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output Compare 1 fast
                                enable
                            </description>
          <name>OC1FE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output Compare 1 preload
                                enable
                            </description>
          <name>OC1PE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Output Compare 1 mode</description>
          <name>OC1M</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Compare 1 mode</description>
          <name>OC1M_3</name>
        </field>
      </fields>
      <name>CCMR1_Output</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <alternateRegister>CCMR1_Output</alternateRegister>
      <description>
                        capture/compare mode register 1 (input
                        mode)
                    </description>
      <displayName>CCMR1_Input</displayName>
      <fields>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Input capture 1 filter</description>
          <name>IC1F</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Input capture 1 prescaler</description>
          <name>IC1PSC</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Capture/Compare 1
                                selection
                            </description>
          <name>CC1S</name>
        </field>
      </fields>
      <name>CCMR1_Input</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>
                        capture/compare enable
                        register
                    </description>
      <displayName>CCER</displayName>
      <fields>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 1 output
                                Polarity
                            </description>
          <name>CC1NP</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 1 complementary output
                                enable
                            </description>
          <name>CC1NE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 1 output
                                Polarity
                            </description>
          <name>CC1P</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 1 output
                                enable
                            </description>
          <name>CC1E</name>
        </field>
      </fields>
      <name>CCER</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x24</addressOffset>
      <description>counter</description>
      <displayName>CNT</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>counter value</description>
          <name>CNT</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>UIF Copy</description>
          <name>UIFCPY</name>
        </field>
      </fields>
      <name>CNT</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28</addressOffset>
      <description>prescaler</description>
      <displayName>PSC</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Prescaler value</description>
          <name>PSC</name>
        </field>
      </fields>
      <name>PSC</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2C</addressOffset>
      <description>auto-reload register</description>
      <displayName>ARR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Auto-reload value</description>
          <name>ARR</name>
        </field>
      </fields>
      <name>ARR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x30</addressOffset>
      <description>repetition counter register</description>
      <displayName>RCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Repetition counter value</description>
          <name>REP</name>
        </field>
      </fields>
      <name>RCR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x34</addressOffset>
      <description>capture/compare register 1</description>
      <displayName>CCR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Capture/Compare 1 value</description>
          <name>CCR1</name>
        </field>
      </fields>
      <name>CCR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x44</addressOffset>
      <description>break and dead-time register</description>
      <displayName>BDTR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Dead-time generator setup</description>
          <name>DTG</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Lock configuration</description>
          <name>LOCK</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Off-state selection for Idle
                                mode
                            </description>
          <name>OSSI</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Off-state selection for Run
                                mode
                            </description>
          <name>OSSR</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Break enable</description>
          <name>BKE</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Break polarity</description>
          <name>BKP</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Automatic output enable</description>
          <name>AOE</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Main output enable</description>
          <name>MOE</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Break filter</description>
          <name>BKF</name>
        </field>
      </fields>
      <name>BDTR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x48</addressOffset>
      <description>DMA control register</description>
      <displayName>DCR</displayName>
      <fields>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>5</bitWidth>
          <description>DMA burst length</description>
          <name>DBL</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
          <description>DMA base address</description>
          <name>DBA</name>
        </field>
      </fields>
      <name>DCR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4C</addressOffset>
      <description>DMA address for full transfer</description>
      <displayName>DMAR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>
                                DMA register for burst
                                accesses
                            </description>
          <name>DMAB</name>
        </field>
      </fields>
      <name>DMAR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x50</addressOffset>
      <description>option register</description>
      <displayName>OR</displayName>
      <name>OR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
