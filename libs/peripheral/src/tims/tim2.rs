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
        /// Direction
        pub dir: bool,
        /// Center-aligned mode selection
        pub cms: bool,
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
        /// Direction
        pub dir: bool,
        /// Center-aligned mode selection
        pub cms: bool,
        /// Auto-reload preload enable
        pub arpe: bool,
        /// Clock division
        pub ckd: bool,
        /// UIF status bit remapping
        pub uifremap: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            cen: ((value >> 0) & 0b1) > 0,
            udis: ((value >> 1) & 0b1) > 0,
            urs: ((value >> 2) & 0b1) > 0,
            opm: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            cms: ((value >> 5) & 0b1) > 0,
            arpe: ((value >> 7) & 0b1) > 0,
            ckd: ((value >> 8) & 0b1) > 0,
            uifremap: ((value >> 11) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x0u32) as *mut u32) };
        Cache {
            cen: ((value >> 0) & 0b1) > 0,
            udis: ((value >> 1) & 0b1) > 0,
            urs: ((value >> 2) & 0b1) > 0,
            opm: ((value >> 3) & 0b1) > 0,
            dir: ((value >> 4) & 0b1) > 0,
            cms: ((value >> 5) & 0b1) > 0,
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
                | ((self.dir as u32) << 4)
                | ((self.cms as u32) << 5)
                | ((self.arpe as u32) << 7)
                | ((self.ckd as u32) << 8)
                | ((self.uifremap as u32) << 11)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// control register 2
pub mod cr2 {
    pub struct ReadonlyCache {
        /// TI1 selection
        pub ti1s: bool,
        /// Master mode selection
        pub mms: bool,
        /// Capture/compare DMA selection
        pub ccds: bool,
    }
    pub struct Cache {
        /// TI1 selection
        pub ti1s: bool,
        /// Master mode selection
        pub mms: bool,
        /// Capture/compare DMA selection
        pub ccds: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            ti1s: ((value >> 7) & 0b1) > 0,
            mms: ((value >> 4) & 0b1) > 0,
            ccds: ((value >> 3) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x4u32) as *mut u32) };
        Cache {
            ti1s: ((value >> 7) & 0b1) > 0,
            mms: ((value >> 4) & 0b1) > 0,
            ccds: ((value >> 3) & 0b1) > 0,
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
                | ((self.ti1s as u32) << 7)
                | ((self.mms as u32) << 4)
                | ((self.ccds as u32) << 3)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// slave mode control register
pub mod smcr {
    pub struct ReadonlyCache {
        /// Slave mode selection
        pub sms: u8,
        /// OCREF clear selection
        pub occs: u8,
        /// Trigger selection
        pub ts: u8,
        /// Master/Slave mode
        pub msm: u8,
        /// External trigger filter
        pub etf: u8,
        /// External trigger prescaler
        pub etps: u8,
        /// External clock enable
        pub ece: u8,
        /// External trigger polarity
        pub etp: u8,
        /// Slave mode selection bit3
        pub sms_3: u8,
    }
    pub struct Cache {
        /// Slave mode selection
        pub sms: u8,
        /// OCREF clear selection
        pub occs: u8,
        /// Trigger selection
        pub ts: u8,
        /// Master/Slave mode
        pub msm: u8,
        /// External trigger filter
        pub etf: u8,
        /// External trigger prescaler
        pub etps: u8,
        /// External clock enable
        pub ece: u8,
        /// External trigger polarity
        pub etp: u8,
        /// Slave mode selection bit3
        pub sms_3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            sms: ((value >> 0) & 0b111) as u8,
            occs: ((value >> 3) & 0b111) as u8,
            ts: ((value >> 4) & 0b111) as u8,
            msm: ((value >> 7) & 0b111) as u8,
            etf: ((value >> 8) & 0b111) as u8,
            etps: ((value >> 12) & 0b111) as u8,
            ece: ((value >> 14) & 0b111) as u8,
            etp: ((value >> 15) & 0b111) as u8,
            sms_3: ((value >> 16) & 0b111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x8u32) as *mut u32) };
        Cache {
            sms: ((value >> 0) & 0b111) as u8,
            occs: ((value >> 3) & 0b111) as u8,
            ts: ((value >> 4) & 0b111) as u8,
            msm: ((value >> 7) & 0b111) as u8,
            etf: ((value >> 8) & 0b111) as u8,
            etps: ((value >> 12) & 0b111) as u8,
            ece: ((value >> 14) & 0b111) as u8,
            etp: ((value >> 15) & 0b111) as u8,
            sms_3: ((value >> 16) & 0b111) as u8,
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
                | ((self.sms as u32) << 0)
                | ((self.occs as u32) << 3)
                | ((self.ts as u32) << 4)
                | ((self.msm as u32) << 7)
                | ((self.etf as u32) << 8)
                | ((self.etps as u32) << 12)
                | ((self.ece as u32) << 14)
                | ((self.etp as u32) << 15)
                | ((self.sms_3 as u32) << 16)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// DMA/Interrupt enable register
pub mod dier {
    pub struct ReadonlyCache {
        /// Trigger DMA request enable
        pub tde: bool,
        /// Capture/Compare 4 DMA request enable
        pub cc4de: bool,
        /// Capture/Compare 3 DMA request enable
        pub cc3de: bool,
        /// Capture/Compare 2 DMA request enable
        pub cc2de: bool,
        /// Capture/Compare 1 DMA request enable
        pub cc1de: bool,
        /// Update DMA request enable
        pub ude: bool,
        /// Trigger interrupt enable
        pub tie: bool,
        /// Capture/Compare 4 interrupt enable
        pub cc4ie: bool,
        /// Capture/Compare 3 interrupt enable
        pub cc3ie: bool,
        /// Capture/Compare 2 interrupt enable
        pub cc2ie: bool,
        /// Capture/Compare 1 interrupt enable
        pub cc1ie: bool,
        /// Update interrupt enable
        pub uie: bool,
    }
    pub struct Cache {
        /// Trigger DMA request enable
        pub tde: bool,
        /// Capture/Compare 4 DMA request enable
        pub cc4de: bool,
        /// Capture/Compare 3 DMA request enable
        pub cc3de: bool,
        /// Capture/Compare 2 DMA request enable
        pub cc2de: bool,
        /// Capture/Compare 1 DMA request enable
        pub cc1de: bool,
        /// Update DMA request enable
        pub ude: bool,
        /// Trigger interrupt enable
        pub tie: bool,
        /// Capture/Compare 4 interrupt enable
        pub cc4ie: bool,
        /// Capture/Compare 3 interrupt enable
        pub cc3ie: bool,
        /// Capture/Compare 2 interrupt enable
        pub cc2ie: bool,
        /// Capture/Compare 1 interrupt enable
        pub cc1ie: bool,
        /// Update interrupt enable
        pub uie: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            tde: ((value >> 14) & 0b1) > 0,
            cc4de: ((value >> 12) & 0b1) > 0,
            cc3de: ((value >> 11) & 0b1) > 0,
            cc2de: ((value >> 10) & 0b1) > 0,
            cc1de: ((value >> 9) & 0b1) > 0,
            ude: ((value >> 8) & 0b1) > 0,
            tie: ((value >> 6) & 0b1) > 0,
            cc4ie: ((value >> 4) & 0b1) > 0,
            cc3ie: ((value >> 3) & 0b1) > 0,
            cc2ie: ((value >> 2) & 0b1) > 0,
            cc1ie: ((value >> 1) & 0b1) > 0,
            uie: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0xCu32) as *mut u32) };
        Cache {
            tde: ((value >> 14) & 0b1) > 0,
            cc4de: ((value >> 12) & 0b1) > 0,
            cc3de: ((value >> 11) & 0b1) > 0,
            cc2de: ((value >> 10) & 0b1) > 0,
            cc1de: ((value >> 9) & 0b1) > 0,
            ude: ((value >> 8) & 0b1) > 0,
            tie: ((value >> 6) & 0b1) > 0,
            cc4ie: ((value >> 4) & 0b1) > 0,
            cc3ie: ((value >> 3) & 0b1) > 0,
            cc2ie: ((value >> 2) & 0b1) > 0,
            cc1ie: ((value >> 1) & 0b1) > 0,
            uie: ((value >> 0) & 0b1) > 0,
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
                | ((self.tde as u32) << 14)
                | ((self.cc4de as u32) << 12)
                | ((self.cc3de as u32) << 11)
                | ((self.cc2de as u32) << 10)
                | ((self.cc1de as u32) << 9)
                | ((self.ude as u32) << 8)
                | ((self.tie as u32) << 6)
                | ((self.cc4ie as u32) << 4)
                | ((self.cc3ie as u32) << 3)
                | ((self.cc2ie as u32) << 2)
                | ((self.cc1ie as u32) << 1)
                | ((self.uie as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// status register
pub mod sr {
    pub struct ReadonlyCache {
        /// Capture/Compare 4 overcapture flag
        pub cc4of: bool,
        /// Capture/Compare 3 overcapture flag
        pub cc3of: bool,
        /// Capture/compare 2 overcapture flag
        pub cc2of: bool,
        /// Capture/Compare 1 overcapture flag
        pub cc1of: bool,
        /// Trigger interrupt flag
        pub tif: bool,
        /// Capture/Compare 4 interrupt flag
        pub cc4if: bool,
        /// Capture/Compare 3 interrupt flag
        pub cc3if: bool,
        /// Capture/Compare 2 interrupt flag
        pub cc2if: bool,
        /// Capture/compare 1 interrupt flag
        pub cc1if: bool,
        /// Update interrupt flag
        pub uif: bool,
    }
    pub struct Cache {
        /// Capture/Compare 4 overcapture flag
        pub cc4of: bool,
        /// Capture/Compare 3 overcapture flag
        pub cc3of: bool,
        /// Capture/compare 2 overcapture flag
        pub cc2of: bool,
        /// Capture/Compare 1 overcapture flag
        pub cc1of: bool,
        /// Trigger interrupt flag
        pub tif: bool,
        /// Capture/Compare 4 interrupt flag
        pub cc4if: bool,
        /// Capture/Compare 3 interrupt flag
        pub cc3if: bool,
        /// Capture/Compare 2 interrupt flag
        pub cc2if: bool,
        /// Capture/compare 1 interrupt flag
        pub cc1if: bool,
        /// Update interrupt flag
        pub uif: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            cc4of: ((value >> 12) & 0b1) > 0,
            cc3of: ((value >> 11) & 0b1) > 0,
            cc2of: ((value >> 10) & 0b1) > 0,
            cc1of: ((value >> 9) & 0b1) > 0,
            tif: ((value >> 6) & 0b1) > 0,
            cc4if: ((value >> 4) & 0b1) > 0,
            cc3if: ((value >> 3) & 0b1) > 0,
            cc2if: ((value >> 2) & 0b1) > 0,
            cc1if: ((value >> 1) & 0b1) > 0,
            uif: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x10u32) as *mut u32) };
        Cache {
            cc4of: ((value >> 12) & 0b1) > 0,
            cc3of: ((value >> 11) & 0b1) > 0,
            cc2of: ((value >> 10) & 0b1) > 0,
            cc1of: ((value >> 9) & 0b1) > 0,
            tif: ((value >> 6) & 0b1) > 0,
            cc4if: ((value >> 4) & 0b1) > 0,
            cc3if: ((value >> 3) & 0b1) > 0,
            cc2if: ((value >> 2) & 0b1) > 0,
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
                | ((self.cc4of as u32) << 12)
                | ((self.cc3of as u32) << 11)
                | ((self.cc2of as u32) << 10)
                | ((self.cc1of as u32) << 9)
                | ((self.tif as u32) << 6)
                | ((self.cc4if as u32) << 4)
                | ((self.cc3if as u32) << 3)
                | ((self.cc2if as u32) << 2)
                | ((self.cc1if as u32) << 1)
                | ((self.uif as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x10u32) as *mut u32, value) };
        }
    }
}
/// event generation register
pub mod egr {
    /// Trigger generation
    /// Access: write-only, Width: 1, Offset: 6
    /// Set Trigger generation
    pub fn tg(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x14u32) as *mut u32, value) };
    }
    /// Capture/compare 4 generation
    /// Access: write-only, Width: 1, Offset: 4
    /// Set Capture/compare 4 generation
    pub fn cc4g(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x14u32) as *mut u32, value) };
    }
    /// Capture/compare 3 generation
    /// Access: write-only, Width: 1, Offset: 3
    /// Set Capture/compare 3 generation
    pub fn cc3g(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x14u32) as *mut u32, value) };
    }
    /// Capture/compare 2 generation
    /// Access: write-only, Width: 1, Offset: 2
    /// Set Capture/compare 2 generation
    pub fn cc2g(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x14u32) as *mut u32, value) };
    }
    /// Capture/compare 1 generation
    /// Access: write-only, Width: 1, Offset: 1
    /// Set Capture/compare 1 generation
    pub fn cc1g(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x14u32) as *mut u32, value) };
    }
    /// Update generation
    /// Access: write-only, Width: 1, Offset: 0
    /// Set Update generation
    pub fn ug(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x14u32) as *mut u32, value) };
    }
}
/// capture/compare mode register 1 (output mode)
pub mod ccmr1_output {
    pub struct ReadonlyCache {
        /// Capture/Compare 1 selection
        pub cc1s: u8,
        /// Output compare 1 fast enable
        pub oc1fe: u8,
        /// Output compare 1 preload enable
        pub oc1pe: u8,
        /// Output compare 1 mode
        pub oc1m: u8,
        /// Output compare 1 clear enable
        pub oc1ce: u8,
        /// Capture/Compare 2 selection
        pub cc2s: u8,
        /// Output compare 2 fast enable
        pub oc2fe: u8,
        /// Output compare 2 preload enable
        pub oc2pe: u8,
        /// Output compare 2 mode
        pub oc2m: u8,
        /// Output compare 2 clear enable
        pub oc2ce: u8,
        /// Output compare 1 mode bit 3
        pub oc1m_3: u8,
        /// Output compare 2 mode bit 3
        pub oc2m_3: u8,
    }
    pub struct Cache {
        /// Capture/Compare 1 selection
        pub cc1s: u8,
        /// Output compare 1 fast enable
        pub oc1fe: u8,
        /// Output compare 1 preload enable
        pub oc1pe: u8,
        /// Output compare 1 mode
        pub oc1m: u8,
        /// Output compare 1 clear enable
        pub oc1ce: u8,
        /// Capture/Compare 2 selection
        pub cc2s: u8,
        /// Output compare 2 fast enable
        pub oc2fe: u8,
        /// Output compare 2 preload enable
        pub oc2pe: u8,
        /// Output compare 2 mode
        pub oc2m: u8,
        /// Output compare 2 clear enable
        pub oc2ce: u8,
        /// Output compare 1 mode bit 3
        pub oc1m_3: u8,
        /// Output compare 2 mode bit 3
        pub oc2m_3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x18u32) as *mut u32) };
        ReadonlyCache {
            cc1s: ((value >> 0) & 0b11) as u8,
            oc1fe: ((value >> 2) & 0b11) as u8,
            oc1pe: ((value >> 3) & 0b11) as u8,
            oc1m: ((value >> 4) & 0b11) as u8,
            oc1ce: ((value >> 7) & 0b11) as u8,
            cc2s: ((value >> 8) & 0b11) as u8,
            oc2fe: ((value >> 10) & 0b11) as u8,
            oc2pe: ((value >> 11) & 0b11) as u8,
            oc2m: ((value >> 12) & 0b11) as u8,
            oc2ce: ((value >> 15) & 0b11) as u8,
            oc1m_3: ((value >> 16) & 0b11) as u8,
            oc2m_3: ((value >> 24) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x18u32) as *mut u32) };
        Cache {
            cc1s: ((value >> 0) & 0b11) as u8,
            oc1fe: ((value >> 2) & 0b11) as u8,
            oc1pe: ((value >> 3) & 0b11) as u8,
            oc1m: ((value >> 4) & 0b11) as u8,
            oc1ce: ((value >> 7) & 0b11) as u8,
            cc2s: ((value >> 8) & 0b11) as u8,
            oc2fe: ((value >> 10) & 0b11) as u8,
            oc2pe: ((value >> 11) & 0b11) as u8,
            oc2m: ((value >> 12) & 0b11) as u8,
            oc2ce: ((value >> 15) & 0b11) as u8,
            oc1m_3: ((value >> 16) & 0b11) as u8,
            oc2m_3: ((value >> 24) & 0b11) as u8,
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
                | ((self.oc1ce as u32) << 7)
                | ((self.cc2s as u32) << 8)
                | ((self.oc2fe as u32) << 10)
                | ((self.oc2pe as u32) << 11)
                | ((self.oc2m as u32) << 12)
                | ((self.oc2ce as u32) << 15)
                | ((self.oc1m_3 as u32) << 16)
                | ((self.oc2m_3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x18u32) as *mut u32, value) };
        }
    }
}
/// capture/compare mode register 1 (input mode)
pub mod ccmr1_input {
    pub struct ReadonlyCache {
        /// Input capture 2 filter
        pub ic2f: u8,
        /// Input capture 2 prescaler
        pub ic2psc: u8,
        /// Capture/compare 2 selection
        pub cc2s: u8,
        /// Input capture 1 filter
        pub ic1f: u8,
        /// Input capture 1 prescaler
        pub ic1psc: u8,
        /// Capture/Compare 1 selection
        pub cc1s: u8,
    }
    pub struct Cache {
        /// Input capture 2 filter
        pub ic2f: u8,
        /// Input capture 2 prescaler
        pub ic2psc: u8,
        /// Capture/compare 2 selection
        pub cc2s: u8,
        /// Input capture 1 filter
        pub ic1f: u8,
        /// Input capture 1 prescaler
        pub ic1psc: u8,
        /// Capture/Compare 1 selection
        pub cc1s: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x18u32) as *mut u32) };
        ReadonlyCache {
            ic2f: ((value >> 12) & 0b1111) as u8,
            ic2psc: ((value >> 10) & 0b1111) as u8,
            cc2s: ((value >> 8) & 0b1111) as u8,
            ic1f: ((value >> 4) & 0b1111) as u8,
            ic1psc: ((value >> 2) & 0b1111) as u8,
            cc1s: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x18u32) as *mut u32) };
        Cache {
            ic2f: ((value >> 12) & 0b1111) as u8,
            ic2psc: ((value >> 10) & 0b1111) as u8,
            cc2s: ((value >> 8) & 0b1111) as u8,
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
                | ((self.ic2f as u32) << 12)
                | ((self.ic2psc as u32) << 10)
                | ((self.cc2s as u32) << 8)
                | ((self.ic1f as u32) << 4)
                | ((self.ic1psc as u32) << 2)
                | ((self.cc1s as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x18u32) as *mut u32, value) };
        }
    }
}
/// capture/compare mode register 2 (output mode)
pub mod ccmr2_output {
    pub struct ReadonlyCache {
        /// Capture/Compare 3 selection
        pub cc3s: u8,
        /// Output compare 3 fast enable
        pub oc3fe: u8,
        /// Output compare 3 preload enable
        pub oc3pe: u8,
        /// Output compare 3 mode
        pub oc3m: u8,
        /// Output compare 3 clear enable
        pub oc3ce: u8,
        /// Capture/Compare 4 selection
        pub cc4s: u8,
        /// Output compare 4 fast enable
        pub oc4fe: u8,
        /// Output compare 4 preload enable
        pub oc4pe: u8,
        /// Output compare 4 mode
        pub oc4m: u8,
        /// Output compare 4 clear enable
        pub o24ce: u8,
        /// Output compare 3 mode bit3
        pub oc3m_3: u8,
        /// Output compare 4 mode bit3
        pub oc4m_3: u8,
    }
    pub struct Cache {
        /// Capture/Compare 3 selection
        pub cc3s: u8,
        /// Output compare 3 fast enable
        pub oc3fe: u8,
        /// Output compare 3 preload enable
        pub oc3pe: u8,
        /// Output compare 3 mode
        pub oc3m: u8,
        /// Output compare 3 clear enable
        pub oc3ce: u8,
        /// Capture/Compare 4 selection
        pub cc4s: u8,
        /// Output compare 4 fast enable
        pub oc4fe: u8,
        /// Output compare 4 preload enable
        pub oc4pe: u8,
        /// Output compare 4 mode
        pub oc4m: u8,
        /// Output compare 4 clear enable
        pub o24ce: u8,
        /// Output compare 3 mode bit3
        pub oc3m_3: u8,
        /// Output compare 4 mode bit3
        pub oc4m_3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x1Cu32) as *mut u32) };
        ReadonlyCache {
            cc3s: ((value >> 0) & 0b11) as u8,
            oc3fe: ((value >> 2) & 0b11) as u8,
            oc3pe: ((value >> 3) & 0b11) as u8,
            oc3m: ((value >> 4) & 0b11) as u8,
            oc3ce: ((value >> 7) & 0b11) as u8,
            cc4s: ((value >> 8) & 0b11) as u8,
            oc4fe: ((value >> 10) & 0b11) as u8,
            oc4pe: ((value >> 11) & 0b11) as u8,
            oc4m: ((value >> 12) & 0b11) as u8,
            o24ce: ((value >> 15) & 0b11) as u8,
            oc3m_3: ((value >> 16) & 0b11) as u8,
            oc4m_3: ((value >> 24) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x1Cu32) as *mut u32) };
        Cache {
            cc3s: ((value >> 0) & 0b11) as u8,
            oc3fe: ((value >> 2) & 0b11) as u8,
            oc3pe: ((value >> 3) & 0b11) as u8,
            oc3m: ((value >> 4) & 0b11) as u8,
            oc3ce: ((value >> 7) & 0b11) as u8,
            cc4s: ((value >> 8) & 0b11) as u8,
            oc4fe: ((value >> 10) & 0b11) as u8,
            oc4pe: ((value >> 11) & 0b11) as u8,
            oc4m: ((value >> 12) & 0b11) as u8,
            o24ce: ((value >> 15) & 0b11) as u8,
            oc3m_3: ((value >> 16) & 0b11) as u8,
            oc4m_3: ((value >> 24) & 0b11) as u8,
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
                | ((self.cc3s as u32) << 0)
                | ((self.oc3fe as u32) << 2)
                | ((self.oc3pe as u32) << 3)
                | ((self.oc3m as u32) << 4)
                | ((self.oc3ce as u32) << 7)
                | ((self.cc4s as u32) << 8)
                | ((self.oc4fe as u32) << 10)
                | ((self.oc4pe as u32) << 11)
                | ((self.oc4m as u32) << 12)
                | ((self.o24ce as u32) << 15)
                | ((self.oc3m_3 as u32) << 16)
                | ((self.oc4m_3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x1Cu32) as *mut u32, value) };
        }
    }
}
/// capture/compare mode register 2 (input mode)
pub mod ccmr2_input {
    pub struct ReadonlyCache {
        /// Input capture 4 filter
        pub ic4f: u8,
        /// Input capture 4 prescaler
        pub ic4psc: u8,
        /// Capture/Compare 4 selection
        pub cc4s: u8,
        /// Input capture 3 filter
        pub ic3f: u8,
        /// Input capture 3 prescaler
        pub ic3psc: u8,
        /// Capture/Compare 3 selection
        pub cc3s: u8,
    }
    pub struct Cache {
        /// Input capture 4 filter
        pub ic4f: u8,
        /// Input capture 4 prescaler
        pub ic4psc: u8,
        /// Capture/Compare 4 selection
        pub cc4s: u8,
        /// Input capture 3 filter
        pub ic3f: u8,
        /// Input capture 3 prescaler
        pub ic3psc: u8,
        /// Capture/Compare 3 selection
        pub cc3s: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x1Cu32) as *mut u32) };
        ReadonlyCache {
            ic4f: ((value >> 12) & 0b1111) as u8,
            ic4psc: ((value >> 10) & 0b1111) as u8,
            cc4s: ((value >> 8) & 0b1111) as u8,
            ic3f: ((value >> 4) & 0b1111) as u8,
            ic3psc: ((value >> 2) & 0b1111) as u8,
            cc3s: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x1Cu32) as *mut u32) };
        Cache {
            ic4f: ((value >> 12) & 0b1111) as u8,
            ic4psc: ((value >> 10) & 0b1111) as u8,
            cc4s: ((value >> 8) & 0b1111) as u8,
            ic3f: ((value >> 4) & 0b1111) as u8,
            ic3psc: ((value >> 2) & 0b1111) as u8,
            cc3s: ((value >> 0) & 0b1111) as u8,
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
                | ((self.ic4f as u32) << 12)
                | ((self.ic4psc as u32) << 10)
                | ((self.cc4s as u32) << 8)
                | ((self.ic3f as u32) << 4)
                | ((self.ic3psc as u32) << 2)
                | ((self.cc3s as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x1Cu32) as *mut u32, value) };
        }
    }
}
/// capture/compare enable register
pub mod ccer {
    pub struct ReadonlyCache {
        /// Capture/Compare 1 output enable
        pub cc1e: bool,
        /// Capture/Compare 1 output Polarity
        pub cc1p: bool,
        /// Capture/Compare 1 output Polarity
        pub cc1np: bool,
        /// Capture/Compare 2 output enable
        pub cc2e: bool,
        /// Capture/Compare 2 output Polarity
        pub cc2p: bool,
        /// Capture/Compare 2 output Polarity
        pub cc2np: bool,
        /// Capture/Compare 3 output enable
        pub cc3e: bool,
        /// Capture/Compare 3 output Polarity
        pub cc3p: bool,
        /// Capture/Compare 3 output Polarity
        pub cc3np: bool,
        /// Capture/Compare 4 output enable
        pub cc4e: bool,
        /// Capture/Compare 3 output Polarity
        pub cc4p: bool,
        /// Capture/Compare 3 output Polarity
        pub cc4np: bool,
    }
    pub struct Cache {
        /// Capture/Compare 1 output enable
        pub cc1e: bool,
        /// Capture/Compare 1 output Polarity
        pub cc1p: bool,
        /// Capture/Compare 1 output Polarity
        pub cc1np: bool,
        /// Capture/Compare 2 output enable
        pub cc2e: bool,
        /// Capture/Compare 2 output Polarity
        pub cc2p: bool,
        /// Capture/Compare 2 output Polarity
        pub cc2np: bool,
        /// Capture/Compare 3 output enable
        pub cc3e: bool,
        /// Capture/Compare 3 output Polarity
        pub cc3p: bool,
        /// Capture/Compare 3 output Polarity
        pub cc3np: bool,
        /// Capture/Compare 4 output enable
        pub cc4e: bool,
        /// Capture/Compare 3 output Polarity
        pub cc4p: bool,
        /// Capture/Compare 3 output Polarity
        pub cc4np: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x20u32) as *mut u32) };
        ReadonlyCache {
            cc1e: ((value >> 0) & 0b1) > 0,
            cc1p: ((value >> 1) & 0b1) > 0,
            cc1np: ((value >> 3) & 0b1) > 0,
            cc2e: ((value >> 4) & 0b1) > 0,
            cc2p: ((value >> 5) & 0b1) > 0,
            cc2np: ((value >> 7) & 0b1) > 0,
            cc3e: ((value >> 8) & 0b1) > 0,
            cc3p: ((value >> 9) & 0b1) > 0,
            cc3np: ((value >> 11) & 0b1) > 0,
            cc4e: ((value >> 12) & 0b1) > 0,
            cc4p: ((value >> 13) & 0b1) > 0,
            cc4np: ((value >> 15) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x20u32) as *mut u32) };
        Cache {
            cc1e: ((value >> 0) & 0b1) > 0,
            cc1p: ((value >> 1) & 0b1) > 0,
            cc1np: ((value >> 3) & 0b1) > 0,
            cc2e: ((value >> 4) & 0b1) > 0,
            cc2p: ((value >> 5) & 0b1) > 0,
            cc2np: ((value >> 7) & 0b1) > 0,
            cc3e: ((value >> 8) & 0b1) > 0,
            cc3p: ((value >> 9) & 0b1) > 0,
            cc3np: ((value >> 11) & 0b1) > 0,
            cc4e: ((value >> 12) & 0b1) > 0,
            cc4p: ((value >> 13) & 0b1) > 0,
            cc4np: ((value >> 15) & 0b1) > 0,
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
                | ((self.cc1e as u32) << 0)
                | ((self.cc1p as u32) << 1)
                | ((self.cc1np as u32) << 3)
                | ((self.cc2e as u32) << 4)
                | ((self.cc2p as u32) << 5)
                | ((self.cc2np as u32) << 7)
                | ((self.cc3e as u32) << 8)
                | ((self.cc3p as u32) << 9)
                | ((self.cc3np as u32) << 11)
                | ((self.cc4e as u32) << 12)
                | ((self.cc4p as u32) << 13)
                | ((self.cc4np as u32) << 15)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x20u32) as *mut u32, value) };
        }
    }
}
/// counter
pub mod cnt {
    pub struct ReadonlyCache {
        /// Low counter value
        pub cntl: u16,
        /// High counter value
        pub cnth: u16,
        /// if IUFREMAP=0 than CNT with read write access else UIFCPY with read only access
        pub cnt_or_uifcpy: u16,
    }
    pub struct Cache {
        /// Low counter value
        pub cntl: u16,
        /// High counter value
        pub cnth: u16,
        /// if IUFREMAP=0 than CNT with read write access else UIFCPY with read only access
        pub cnt_or_uifcpy: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x24u32) as *mut u32) };
        ReadonlyCache {
            cntl: ((value >> 0) & 0b1111111111111111) as u16,
            cnth: ((value >> 16) & 0b1111111111111111) as u16,
            cnt_or_uifcpy: ((value >> 31) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x24u32) as *mut u32) };
        Cache {
            cntl: ((value >> 0) & 0b1111111111111111) as u16,
            cnth: ((value >> 16) & 0b1111111111111111) as u16,
            cnt_or_uifcpy: ((value >> 31) & 0b1111111111111111) as u16,
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
                | ((self.cntl as u32) << 0)
                | ((self.cnth as u32) << 16)
                | ((self.cnt_or_uifcpy as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x24u32) as *mut u32, value) };
        }
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
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x28u32) as *mut u32) };
        ReadonlyCache {
            psc: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x28u32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x28u32) as *mut u32, value) };
        }
    }
}
/// auto-reload register
pub mod arr {
    pub struct ReadonlyCache {
        /// Low Auto-reload value
        pub arrl: u16,
        /// High Auto-reload value
        pub arrh: u16,
    }
    pub struct Cache {
        /// Low Auto-reload value
        pub arrl: u16,
        /// High Auto-reload value
        pub arrh: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x2Cu32) as *mut u32) };
        ReadonlyCache {
            arrl: ((value >> 0) & 0b1111111111111111) as u16,
            arrh: ((value >> 16) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x2Cu32) as *mut u32) };
        Cache {
            arrl: ((value >> 0) & 0b1111111111111111) as u16,
            arrh: ((value >> 16) & 0b1111111111111111) as u16,
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
                | ((self.arrl as u32) << 0)
                | ((self.arrh as u32) << 16)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x2Cu32) as *mut u32, value) };
        }
    }
}
/// capture/compare register 1
pub mod ccr1 {
    pub struct ReadonlyCache {
        /// Low Capture/Compare 1 value
        pub ccr1l: u16,
        /// High Capture/Compare 1 value (on TIM2)
        pub ccr1h: u16,
    }
    pub struct Cache {
        /// Low Capture/Compare 1 value
        pub ccr1l: u16,
        /// High Capture/Compare 1 value (on TIM2)
        pub ccr1h: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x34u32) as *mut u32) };
        ReadonlyCache {
            ccr1l: ((value >> 0) & 0b1111111111111111) as u16,
            ccr1h: ((value >> 16) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x34u32) as *mut u32) };
        Cache {
            ccr1l: ((value >> 0) & 0b1111111111111111) as u16,
            ccr1h: ((value >> 16) & 0b1111111111111111) as u16,
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
                | ((self.ccr1l as u32) << 0)
                | ((self.ccr1h as u32) << 16)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x34u32) as *mut u32, value) };
        }
    }
}
/// capture/compare register 2
pub mod ccr2 {
    pub struct ReadonlyCache {
        /// Low Capture/Compare 2 value
        pub ccr2l: u16,
        /// High Capture/Compare 2 value (on TIM2)
        pub ccr2h: u16,
    }
    pub struct Cache {
        /// Low Capture/Compare 2 value
        pub ccr2l: u16,
        /// High Capture/Compare 2 value (on TIM2)
        pub ccr2h: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x38u32) as *mut u32) };
        ReadonlyCache {
            ccr2l: ((value >> 0) & 0b1111111111111111) as u16,
            ccr2h: ((value >> 16) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x38u32) as *mut u32) };
        Cache {
            ccr2l: ((value >> 0) & 0b1111111111111111) as u16,
            ccr2h: ((value >> 16) & 0b1111111111111111) as u16,
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
                | ((self.ccr2l as u32) << 0)
                | ((self.ccr2h as u32) << 16)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x38u32) as *mut u32, value) };
        }
    }
}
/// capture/compare register 3
pub mod ccr3 {
    pub struct ReadonlyCache {
        /// Low Capture/Compare value
        pub ccr3l: u16,
        /// High Capture/Compare value (on TIM2)
        pub ccr3h: u16,
    }
    pub struct Cache {
        /// Low Capture/Compare value
        pub ccr3l: u16,
        /// High Capture/Compare value (on TIM2)
        pub ccr3h: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x3Cu32) as *mut u32) };
        ReadonlyCache {
            ccr3l: ((value >> 0) & 0b1111111111111111) as u16,
            ccr3h: ((value >> 16) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x3Cu32) as *mut u32) };
        Cache {
            ccr3l: ((value >> 0) & 0b1111111111111111) as u16,
            ccr3h: ((value >> 16) & 0b1111111111111111) as u16,
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
                | ((self.ccr3l as u32) << 0)
                | ((self.ccr3h as u32) << 16)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x3Cu32) as *mut u32, value) };
        }
    }
}
/// capture/compare register 4
pub mod ccr4 {
    pub struct ReadonlyCache {
        /// Low Capture/Compare value
        pub ccr4l: u16,
        /// High Capture/Compare value (on TIM2)
        pub ccr4h: u16,
    }
    pub struct Cache {
        /// Low Capture/Compare value
        pub ccr4l: u16,
        /// High Capture/Compare value (on TIM2)
        pub ccr4h: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x40u32) as *mut u32) };
        ReadonlyCache {
            ccr4l: ((value >> 0) & 0b1111111111111111) as u16,
            ccr4h: ((value >> 16) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x40u32) as *mut u32) };
        Cache {
            ccr4l: ((value >> 0) & 0b1111111111111111) as u16,
            ccr4h: ((value >> 16) & 0b1111111111111111) as u16,
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
                | ((self.ccr4l as u32) << 0)
                | ((self.ccr4h as u32) << 16)
            ;
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x40u32) as *mut u32, value) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x48u32) as *mut u32) };
        ReadonlyCache {
            dbl: ((value >> 8) & 0b11111) as u8,
            dba: ((value >> 0) & 0b11111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x48u32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x48u32) as *mut u32, value) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x4Cu32) as *mut u32) };
        ReadonlyCache {
            dmab: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40000000u32 + 0x4Cu32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40000000u32 + 0x4Cu32) as *mut u32, value) };
        }
    }
}
/// TIM2 global interrupt
pub const INTERRUPT_TIM2: u32 = 28;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40000000</baseAddress>
  <description>General purpose timer</description>
  <groupName>TIMs</groupName>
  <interrupt>
    <description>TIM2 global interrupt</description>
    <name>TIM2</name>
    <value>28</value>
  </interrupt>
  <name>TIM2</name>
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
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Direction</description>
          <name>DIR</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Center-aligned mode
                                selection
                            </description>
          <name>CMS</name>
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
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TI1 selection</description>
          <name>TI1S</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Master mode selection</description>
          <name>MMS</name>
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
      </fields>
      <name>CR2</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>slave mode control register</description>
      <displayName>SMCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Slave mode selection</description>
          <name>SMS</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OCREF clear selection</description>
          <name>OCCS</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Trigger selection</description>
          <name>TS</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Master/Slave mode</description>
          <name>MSM</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>External trigger filter</description>
          <name>ETF</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>External trigger prescaler</description>
          <name>ETPS</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>External clock enable</description>
          <name>ECE</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>External trigger polarity</description>
          <name>ETP</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Slave mode selection bit3</description>
          <name>SMS_3</name>
        </field>
      </fields>
      <name>SMCR</name>
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
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Trigger DMA request enable</description>
          <name>TDE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 4 DMA request
                                enable
                            </description>
          <name>CC4DE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 3 DMA request
                                enable
                            </description>
          <name>CC3DE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 2 DMA request
                                enable
                            </description>
          <name>CC2DE</name>
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
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update DMA request enable</description>
          <name>UDE</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Trigger interrupt enable</description>
          <name>TIE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 4 interrupt
                                enable
                            </description>
          <name>CC4IE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 3 interrupt
                                enable
                            </description>
          <name>CC3IE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 2 interrupt
                                enable
                            </description>
          <name>CC2IE</name>
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
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update interrupt enable</description>
          <name>UIE</name>
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
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 4 overcapture
                                flag
                            </description>
          <name>CC4OF</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 3 overcapture
                                flag
                            </description>
          <name>CC3OF</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/compare 2 overcapture
                                flag
                            </description>
          <name>CC2OF</name>
        </field>
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
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Trigger interrupt flag</description>
          <name>TIF</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 4 interrupt
                                flag
                            </description>
          <name>CC4IF</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 3 interrupt
                                flag
                            </description>
          <name>CC3IF</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 2 interrupt
                                flag
                            </description>
          <name>CC2IF</name>
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
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Trigger generation</description>
          <name>TG</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/compare 4
                                generation
                            </description>
          <name>CC4G</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/compare 3
                                generation
                            </description>
          <name>CC3G</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/compare 2
                                generation
                            </description>
          <name>CC2G</name>
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
                        capture/compare mode register 1 (output
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
                                Output compare 1 fast
                                enable
                            </description>
          <name>OC1FE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 1 preload
                                enable
                            </description>
          <name>OC1PE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Output compare 1 mode</description>
          <name>OC1M</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 1 clear
                                enable
                            </description>
          <name>OC1CE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Capture/Compare 2
                                selection
                            </description>
          <name>CC2S</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 2 fast
                                enable
                            </description>
          <name>OC2FE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 2 preload
                                enable
                            </description>
          <name>OC2PE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Output compare 2 mode</description>
          <name>OC2M</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 2 clear
                                enable
                            </description>
          <name>OC2CE</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 1 mode bit
                                3
                            </description>
          <name>OC1M_3</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 2 mode bit
                                3
                            </description>
          <name>OC2M_3</name>
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
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Input capture 2 filter</description>
          <name>IC2F</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Input capture 2 prescaler</description>
          <name>IC2PSC</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Capture/compare 2
                                selection
                            </description>
          <name>CC2S</name>
        </field>
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
      <addressOffset>0x1C</addressOffset>
      <description>
                        capture/compare mode register 2 (output
                        mode)
                    </description>
      <displayName>CCMR2_Output</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Capture/Compare 3
                                selection
                            </description>
          <name>CC3S</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 3 fast
                                enable
                            </description>
          <name>OC3FE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 3 preload
                                enable
                            </description>
          <name>OC3PE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Output compare 3 mode</description>
          <name>OC3M</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 3 clear
                                enable
                            </description>
          <name>OC3CE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Capture/Compare 4
                                selection
                            </description>
          <name>CC4S</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 4 fast
                                enable
                            </description>
          <name>OC4FE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 4 preload
                                enable
                            </description>
          <name>OC4PE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Output compare 4 mode</description>
          <name>OC4M</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 4 clear
                                enable
                            </description>
          <name>O24CE</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output compare 3 mode bit3</description>
          <name>OC3M_3</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output compare 4 mode bit3</description>
          <name>OC4M_3</name>
        </field>
      </fields>
      <name>CCMR2_Output</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <alternateRegister>CCMR2_Output</alternateRegister>
      <description>
                        capture/compare mode register 2 (input
                        mode)
                    </description>
      <displayName>CCMR2_Input</displayName>
      <fields>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Input capture 4 filter</description>
          <name>IC4F</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Input capture 4 prescaler</description>
          <name>IC4PSC</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Capture/Compare 4
                                selection
                            </description>
          <name>CC4S</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Input capture 3 filter</description>
          <name>IC3F</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Input capture 3 prescaler</description>
          <name>IC3PSC</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Capture/Compare 3
                                selection
                            </description>
          <name>CC3S</name>
        </field>
      </fields>
      <name>CCMR2_Input</name>
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
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 1 output
                                enable
                            </description>
          <name>CC1E</name>
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
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 1 output
                                Polarity
                            </description>
          <name>CC1NP</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 2 output
                                enable
                            </description>
          <name>CC2E</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 2 output
                                Polarity
                            </description>
          <name>CC2P</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 2 output
                                Polarity
                            </description>
          <name>CC2NP</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 3 output
                                enable
                            </description>
          <name>CC3E</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 3 output
                                Polarity
                            </description>
          <name>CC3P</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 3 output
                                Polarity
                            </description>
          <name>CC3NP</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 4 output
                                enable
                            </description>
          <name>CC4E</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 3 output
                                Polarity
                            </description>
          <name>CC4P</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 3 output
                                Polarity
                            </description>
          <name>CC4NP</name>
        </field>
      </fields>
      <name>CCER</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x24</addressOffset>
      <description>counter</description>
      <displayName>CNT</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Low counter value</description>
          <name>CNTL</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>15</bitWidth>
          <description>High counter value</description>
          <name>CNTH</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                if IUFREMAP=0 than CNT with read write
                                access else UIFCPY with read only
                                access
                            </description>
          <name>CNT_or_UIFCPY</name>
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
          <description>Low Auto-reload value</description>
          <name>ARRL</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>High Auto-reload value</description>
          <name>ARRH</name>
        </field>
      </fields>
      <name>ARR</name>
      <resetValue>0x00000000</resetValue>
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
          <description>
                                Low Capture/Compare 1
                                value
                            </description>
          <name>CCR1L</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>
                                High Capture/Compare 1 value (on
                                TIM2)
                            </description>
          <name>CCR1H</name>
        </field>
      </fields>
      <name>CCR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x38</addressOffset>
      <description>capture/compare register 2</description>
      <displayName>CCR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>
                                Low Capture/Compare 2
                                value
                            </description>
          <name>CCR2L</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>
                                High Capture/Compare 2 value (on
                                TIM2)
                            </description>
          <name>CCR2H</name>
        </field>
      </fields>
      <name>CCR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x3C</addressOffset>
      <description>capture/compare register 3</description>
      <displayName>CCR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Low Capture/Compare value</description>
          <name>CCR3L</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>
                                High Capture/Compare value (on
                                TIM2)
                            </description>
          <name>CCR3H</name>
        </field>
      </fields>
      <name>CCR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x40</addressOffset>
      <description>capture/compare register 4</description>
      <displayName>CCR4</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Low Capture/Compare value</description>
          <name>CCR4L</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>
                                High Capture/Compare value (on
                                TIM2)
                            </description>
          <name>CCR4H</name>
        </field>
      </fields>
      <name>CCR4</name>
      <resetValue>0x00000000</resetValue>
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
  </registers>
</peripheral>
*/
