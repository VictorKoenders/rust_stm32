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
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x0u32) as *mut u32) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x0u32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// control register 2
pub mod cr2 {
    pub struct ReadonlyCache {
        /// Capture/compare preloaded control
        pub ccpc: bool,
        /// Capture/compare control update selection
        pub ccus: bool,
        /// Capture/compare DMA selection
        pub ccds: bool,
        /// Master mode selection
        pub mms: bool,
        /// TI1 selection
        pub ti1s: bool,
        /// Output Idle state 1
        pub ois1: bool,
        /// Output Idle state 1
        pub ois1n: bool,
        /// Output Idle state 2
        pub ois2: bool,
        /// Output Idle state 2
        pub ois2n: bool,
        /// Output Idle state 3
        pub ois3: bool,
        /// Output Idle state 3
        pub ois3n: bool,
        /// Output Idle state 4
        pub ois4: bool,
        /// Output Idle state 5
        pub ois5: bool,
        /// Output Idle state 6
        pub ois6: bool,
        /// Master mode selection 2
        pub mms2: bool,
    }
    pub struct Cache {
        /// Capture/compare preloaded control
        pub ccpc: bool,
        /// Capture/compare control update selection
        pub ccus: bool,
        /// Capture/compare DMA selection
        pub ccds: bool,
        /// Master mode selection
        pub mms: bool,
        /// TI1 selection
        pub ti1s: bool,
        /// Output Idle state 1
        pub ois1: bool,
        /// Output Idle state 1
        pub ois1n: bool,
        /// Output Idle state 2
        pub ois2: bool,
        /// Output Idle state 2
        pub ois2n: bool,
        /// Output Idle state 3
        pub ois3: bool,
        /// Output Idle state 3
        pub ois3n: bool,
        /// Output Idle state 4
        pub ois4: bool,
        /// Output Idle state 5
        pub ois5: bool,
        /// Output Idle state 6
        pub ois6: bool,
        /// Master mode selection 2
        pub mms2: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            ccpc: ((value >> 0) & 0b1) > 0,
            ccus: ((value >> 2) & 0b1) > 0,
            ccds: ((value >> 3) & 0b1) > 0,
            mms: ((value >> 4) & 0b1) > 0,
            ti1s: ((value >> 7) & 0b1) > 0,
            ois1: ((value >> 8) & 0b1) > 0,
            ois1n: ((value >> 9) & 0b1) > 0,
            ois2: ((value >> 10) & 0b1) > 0,
            ois2n: ((value >> 11) & 0b1) > 0,
            ois3: ((value >> 12) & 0b1) > 0,
            ois3n: ((value >> 13) & 0b1) > 0,
            ois4: ((value >> 14) & 0b1) > 0,
            ois5: ((value >> 16) & 0b1) > 0,
            ois6: ((value >> 18) & 0b1) > 0,
            mms2: ((value >> 20) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x4u32) as *mut u32) };
        Cache {
            ccpc: ((value >> 0) & 0b1) > 0,
            ccus: ((value >> 2) & 0b1) > 0,
            ccds: ((value >> 3) & 0b1) > 0,
            mms: ((value >> 4) & 0b1) > 0,
            ti1s: ((value >> 7) & 0b1) > 0,
            ois1: ((value >> 8) & 0b1) > 0,
            ois1n: ((value >> 9) & 0b1) > 0,
            ois2: ((value >> 10) & 0b1) > 0,
            ois2n: ((value >> 11) & 0b1) > 0,
            ois3: ((value >> 12) & 0b1) > 0,
            ois3n: ((value >> 13) & 0b1) > 0,
            ois4: ((value >> 14) & 0b1) > 0,
            ois5: ((value >> 16) & 0b1) > 0,
            ois6: ((value >> 18) & 0b1) > 0,
            mms2: ((value >> 20) & 0b1) > 0,
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
                | ((self.ccpc as u32) << 0)
                | ((self.ccus as u32) << 2)
                | ((self.ccds as u32) << 3)
                | ((self.mms as u32) << 4)
                | ((self.ti1s as u32) << 7)
                | ((self.ois1 as u32) << 8)
                | ((self.ois1n as u32) << 9)
                | ((self.ois2 as u32) << 10)
                | ((self.ois2n as u32) << 11)
                | ((self.ois3 as u32) << 12)
                | ((self.ois3n as u32) << 13)
                | ((self.ois4 as u32) << 14)
                | ((self.ois5 as u32) << 16)
                | ((self.ois6 as u32) << 18)
                | ((self.mms2 as u32) << 20)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x4u32) as *mut u32, value) };
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
        /// Slave mode selection bit 3
        pub sms3: u8,
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
        /// Slave mode selection bit 3
        pub sms3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            sms: ((value >> 0) & 0b111) as u8,
            occs: ((value >> 3) & 0b111) as u8,
            ts: ((value >> 4) & 0b111) as u8,
            msm: ((value >> 7) & 0b111) as u8,
            etf: ((value >> 8) & 0b111) as u8,
            etps: ((value >> 12) & 0b111) as u8,
            ece: ((value >> 14) & 0b111) as u8,
            etp: ((value >> 15) & 0b111) as u8,
            sms3: ((value >> 16) & 0b111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x8u32) as *mut u32) };
        Cache {
            sms: ((value >> 0) & 0b111) as u8,
            occs: ((value >> 3) & 0b111) as u8,
            ts: ((value >> 4) & 0b111) as u8,
            msm: ((value >> 7) & 0b111) as u8,
            etf: ((value >> 8) & 0b111) as u8,
            etps: ((value >> 12) & 0b111) as u8,
            ece: ((value >> 14) & 0b111) as u8,
            etp: ((value >> 15) & 0b111) as u8,
            sms3: ((value >> 16) & 0b111) as u8,
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
                | ((self.sms3 as u32) << 16)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// DMA/Interrupt enable register
pub mod dier {
    pub struct ReadonlyCache {
        /// Trigger DMA request enable
        pub tde: bool,
        /// Reserved
        pub comde: bool,
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
        /// Break interrupt enable
        pub bie: bool,
        /// Trigger interrupt enable
        pub tie: bool,
        /// COM interrupt enable
        pub comie: bool,
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
        /// Reserved
        pub comde: bool,
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
        /// Break interrupt enable
        pub bie: bool,
        /// Trigger interrupt enable
        pub tie: bool,
        /// COM interrupt enable
        pub comie: bool,
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
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            tde: ((value >> 14) & 0b1) > 0,
            comde: ((value >> 13) & 0b1) > 0,
            cc4de: ((value >> 12) & 0b1) > 0,
            cc3de: ((value >> 11) & 0b1) > 0,
            cc2de: ((value >> 10) & 0b1) > 0,
            cc1de: ((value >> 9) & 0b1) > 0,
            ude: ((value >> 8) & 0b1) > 0,
            bie: ((value >> 7) & 0b1) > 0,
            tie: ((value >> 6) & 0b1) > 0,
            comie: ((value >> 5) & 0b1) > 0,
            cc4ie: ((value >> 4) & 0b1) > 0,
            cc3ie: ((value >> 3) & 0b1) > 0,
            cc2ie: ((value >> 2) & 0b1) > 0,
            cc1ie: ((value >> 1) & 0b1) > 0,
            uie: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0xCu32) as *mut u32) };
        Cache {
            tde: ((value >> 14) & 0b1) > 0,
            comde: ((value >> 13) & 0b1) > 0,
            cc4de: ((value >> 12) & 0b1) > 0,
            cc3de: ((value >> 11) & 0b1) > 0,
            cc2de: ((value >> 10) & 0b1) > 0,
            cc1de: ((value >> 9) & 0b1) > 0,
            ude: ((value >> 8) & 0b1) > 0,
            bie: ((value >> 7) & 0b1) > 0,
            tie: ((value >> 6) & 0b1) > 0,
            comie: ((value >> 5) & 0b1) > 0,
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
                | ((self.comde as u32) << 13)
                | ((self.cc4de as u32) << 12)
                | ((self.cc3de as u32) << 11)
                | ((self.cc2de as u32) << 10)
                | ((self.cc1de as u32) << 9)
                | ((self.ude as u32) << 8)
                | ((self.bie as u32) << 7)
                | ((self.tie as u32) << 6)
                | ((self.comie as u32) << 5)
                | ((self.cc4ie as u32) << 4)
                | ((self.cc3ie as u32) << 3)
                | ((self.cc2ie as u32) << 2)
                | ((self.cc1ie as u32) << 1)
                | ((self.uie as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// status register
pub mod sr {
    pub struct ReadonlyCache {
        /// Update interrupt flag
        pub uif: bool,
        /// Capture/compare 1 interrupt flag
        pub cc1if: bool,
        /// Capture/Compare 2 interrupt flag
        pub cc2if: bool,
        /// Capture/Compare 3 interrupt flag
        pub cc3if: bool,
        /// Capture/Compare 4 interrupt flag
        pub cc4if: bool,
        /// COM interrupt flag
        pub comif: bool,
        /// Trigger interrupt flag
        pub tif: bool,
        /// Break interrupt flag
        pub bif: bool,
        /// Break 2 interrupt flag
        pub b2if: bool,
        /// Capture/Compare 1 overcapture flag
        pub cc1of: bool,
        /// Capture/compare 2 overcapture flag
        pub cc2of: bool,
        /// Capture/Compare 3 overcapture flag
        pub cc3of: bool,
        /// Capture/Compare 4 overcapture flag
        pub cc4of: bool,
        /// Capture/Compare 5 interrupt flag
        pub c5if: bool,
        /// Capture/Compare 6 interrupt flag
        pub c6if: bool,
    }
    pub struct Cache {
        /// Update interrupt flag
        pub uif: bool,
        /// Capture/compare 1 interrupt flag
        pub cc1if: bool,
        /// Capture/Compare 2 interrupt flag
        pub cc2if: bool,
        /// Capture/Compare 3 interrupt flag
        pub cc3if: bool,
        /// Capture/Compare 4 interrupt flag
        pub cc4if: bool,
        /// COM interrupt flag
        pub comif: bool,
        /// Trigger interrupt flag
        pub tif: bool,
        /// Break interrupt flag
        pub bif: bool,
        /// Break 2 interrupt flag
        pub b2if: bool,
        /// Capture/Compare 1 overcapture flag
        pub cc1of: bool,
        /// Capture/compare 2 overcapture flag
        pub cc2of: bool,
        /// Capture/Compare 3 overcapture flag
        pub cc3of: bool,
        /// Capture/Compare 4 overcapture flag
        pub cc4of: bool,
        /// Capture/Compare 5 interrupt flag
        pub c5if: bool,
        /// Capture/Compare 6 interrupt flag
        pub c6if: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            uif: ((value >> 0) & 0b1) > 0,
            cc1if: ((value >> 1) & 0b1) > 0,
            cc2if: ((value >> 2) & 0b1) > 0,
            cc3if: ((value >> 3) & 0b1) > 0,
            cc4if: ((value >> 4) & 0b1) > 0,
            comif: ((value >> 5) & 0b1) > 0,
            tif: ((value >> 6) & 0b1) > 0,
            bif: ((value >> 7) & 0b1) > 0,
            b2if: ((value >> 8) & 0b1) > 0,
            cc1of: ((value >> 9) & 0b1) > 0,
            cc2of: ((value >> 10) & 0b1) > 0,
            cc3of: ((value >> 11) & 0b1) > 0,
            cc4of: ((value >> 12) & 0b1) > 0,
            c5if: ((value >> 16) & 0b1) > 0,
            c6if: ((value >> 17) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x10u32) as *mut u32) };
        Cache {
            uif: ((value >> 0) & 0b1) > 0,
            cc1if: ((value >> 1) & 0b1) > 0,
            cc2if: ((value >> 2) & 0b1) > 0,
            cc3if: ((value >> 3) & 0b1) > 0,
            cc4if: ((value >> 4) & 0b1) > 0,
            comif: ((value >> 5) & 0b1) > 0,
            tif: ((value >> 6) & 0b1) > 0,
            bif: ((value >> 7) & 0b1) > 0,
            b2if: ((value >> 8) & 0b1) > 0,
            cc1of: ((value >> 9) & 0b1) > 0,
            cc2of: ((value >> 10) & 0b1) > 0,
            cc3of: ((value >> 11) & 0b1) > 0,
            cc4of: ((value >> 12) & 0b1) > 0,
            c5if: ((value >> 16) & 0b1) > 0,
            c6if: ((value >> 17) & 0b1) > 0,
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
                | ((self.uif as u32) << 0)
                | ((self.cc1if as u32) << 1)
                | ((self.cc2if as u32) << 2)
                | ((self.cc3if as u32) << 3)
                | ((self.cc4if as u32) << 4)
                | ((self.comif as u32) << 5)
                | ((self.tif as u32) << 6)
                | ((self.bif as u32) << 7)
                | ((self.b2if as u32) << 8)
                | ((self.cc1of as u32) << 9)
                | ((self.cc2of as u32) << 10)
                | ((self.cc3of as u32) << 11)
                | ((self.cc4of as u32) << 12)
                | ((self.c5if as u32) << 16)
                | ((self.c6if as u32) << 17)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x10u32) as *mut u32, value) };
        }
    }
}
/// event generation register
pub mod egr {
    /// Update generation
    /// Access: write-only, Width: 1, Offset: 0
    /// Set Update generation
    pub fn ug(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Capture/compare 1 generation
    /// Access: write-only, Width: 1, Offset: 1
    /// Set Capture/compare 1 generation
    pub fn cc1g(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Capture/compare 2 generation
    /// Access: write-only, Width: 1, Offset: 2
    /// Set Capture/compare 2 generation
    pub fn cc2g(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Capture/compare 3 generation
    /// Access: write-only, Width: 1, Offset: 3
    /// Set Capture/compare 3 generation
    pub fn cc3g(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Capture/compare 4 generation
    /// Access: write-only, Width: 1, Offset: 4
    /// Set Capture/compare 4 generation
    pub fn cc4g(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Capture/Compare control update generation
    /// Access: write-only, Width: 1, Offset: 5
    /// Set Capture/Compare control update generation
    pub fn comg(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Trigger generation
    /// Access: write-only, Width: 1, Offset: 6
    /// Set Trigger generation
    pub fn tg(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Break generation
    /// Access: write-only, Width: 1, Offset: 7
    /// Set Break generation
    pub fn bg(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Break 2 generation
    /// Access: write-only, Width: 1, Offset: 8
    /// Set Break 2 generation
    pub fn b2g(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x14u32) as *mut u32, value) };
    }
}
/// capture/compare mode register (output mode)
pub mod ccmr1_output {
    pub struct ReadonlyCache {
        /// Output Compare 2 clear enable
        pub oc2ce: bool,
        /// Output Compare 2 mode
        pub oc2m: bool,
        /// Output Compare 2 preload enable
        pub oc2pe: bool,
        /// Output Compare 2 fast enable
        pub oc2fe: bool,
        /// Capture/Compare 2 selection
        pub cc2s: bool,
        /// Output Compare 1 clear enable
        pub oc1ce: bool,
        /// Output Compare 1 mode
        pub oc1m: bool,
        /// Output Compare 1 preload enable
        pub oc1pe: bool,
        /// Output Compare 1 fast enable
        pub oc1fe: bool,
        /// Capture/Compare 1 selection
        pub cc1s: bool,
        /// Output Compare 1 mode bit 3
        pub oc1m_3: bool,
        /// Output Compare 2 mode bit 3
        pub oc2m_3: bool,
    }
    pub struct Cache {
        /// Output Compare 2 clear enable
        pub oc2ce: bool,
        /// Output Compare 2 mode
        pub oc2m: bool,
        /// Output Compare 2 preload enable
        pub oc2pe: bool,
        /// Output Compare 2 fast enable
        pub oc2fe: bool,
        /// Capture/Compare 2 selection
        pub cc2s: bool,
        /// Output Compare 1 clear enable
        pub oc1ce: bool,
        /// Output Compare 1 mode
        pub oc1m: bool,
        /// Output Compare 1 preload enable
        pub oc1pe: bool,
        /// Output Compare 1 fast enable
        pub oc1fe: bool,
        /// Capture/Compare 1 selection
        pub cc1s: bool,
        /// Output Compare 1 mode bit 3
        pub oc1m_3: bool,
        /// Output Compare 2 mode bit 3
        pub oc2m_3: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x18u32) as *mut u32) };
        ReadonlyCache {
            oc2ce: ((value >> 15) & 0b1) > 0,
            oc2m: ((value >> 12) & 0b1) > 0,
            oc2pe: ((value >> 11) & 0b1) > 0,
            oc2fe: ((value >> 10) & 0b1) > 0,
            cc2s: ((value >> 8) & 0b1) > 0,
            oc1ce: ((value >> 7) & 0b1) > 0,
            oc1m: ((value >> 4) & 0b1) > 0,
            oc1pe: ((value >> 3) & 0b1) > 0,
            oc1fe: ((value >> 2) & 0b1) > 0,
            cc1s: ((value >> 0) & 0b1) > 0,
            oc1m_3: ((value >> 16) & 0b1) > 0,
            oc2m_3: ((value >> 24) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x18u32) as *mut u32) };
        Cache {
            oc2ce: ((value >> 15) & 0b1) > 0,
            oc2m: ((value >> 12) & 0b1) > 0,
            oc2pe: ((value >> 11) & 0b1) > 0,
            oc2fe: ((value >> 10) & 0b1) > 0,
            cc2s: ((value >> 8) & 0b1) > 0,
            oc1ce: ((value >> 7) & 0b1) > 0,
            oc1m: ((value >> 4) & 0b1) > 0,
            oc1pe: ((value >> 3) & 0b1) > 0,
            oc1fe: ((value >> 2) & 0b1) > 0,
            cc1s: ((value >> 0) & 0b1) > 0,
            oc1m_3: ((value >> 16) & 0b1) > 0,
            oc2m_3: ((value >> 24) & 0b1) > 0,
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
                | ((self.oc2ce as u32) << 15)
                | ((self.oc2m as u32) << 12)
                | ((self.oc2pe as u32) << 11)
                | ((self.oc2fe as u32) << 10)
                | ((self.cc2s as u32) << 8)
                | ((self.oc1ce as u32) << 7)
                | ((self.oc1m as u32) << 4)
                | ((self.oc1pe as u32) << 3)
                | ((self.oc1fe as u32) << 2)
                | ((self.cc1s as u32) << 0)
                | ((self.oc1m_3 as u32) << 16)
                | ((self.oc2m_3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x18u32) as *mut u32, value) };
        }
    }
}
/// capture/compare mode register 1 (input mode)
pub mod ccmr1_input {
    pub struct ReadonlyCache {
        /// Input capture 2 filter
        pub ic2f: u8,
        /// Input capture 2 prescaler
        pub ic2pcs: u8,
        /// Capture/Compare 2 selection
        pub cc2s: u8,
        /// Input capture 1 filter
        pub ic1f: u8,
        /// Input capture 1 prescaler
        pub ic1pcs: u8,
        /// Capture/Compare 1 selection
        pub cc1s: u8,
    }
    pub struct Cache {
        /// Input capture 2 filter
        pub ic2f: u8,
        /// Input capture 2 prescaler
        pub ic2pcs: u8,
        /// Capture/Compare 2 selection
        pub cc2s: u8,
        /// Input capture 1 filter
        pub ic1f: u8,
        /// Input capture 1 prescaler
        pub ic1pcs: u8,
        /// Capture/Compare 1 selection
        pub cc1s: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x18u32) as *mut u32) };
        ReadonlyCache {
            ic2f: ((value >> 12) & 0b1111) as u8,
            ic2pcs: ((value >> 10) & 0b1111) as u8,
            cc2s: ((value >> 8) & 0b1111) as u8,
            ic1f: ((value >> 4) & 0b1111) as u8,
            ic1pcs: ((value >> 2) & 0b1111) as u8,
            cc1s: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x18u32) as *mut u32) };
        Cache {
            ic2f: ((value >> 12) & 0b1111) as u8,
            ic2pcs: ((value >> 10) & 0b1111) as u8,
            cc2s: ((value >> 8) & 0b1111) as u8,
            ic1f: ((value >> 4) & 0b1111) as u8,
            ic1pcs: ((value >> 2) & 0b1111) as u8,
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
                | ((self.ic2pcs as u32) << 10)
                | ((self.cc2s as u32) << 8)
                | ((self.ic1f as u32) << 4)
                | ((self.ic1pcs as u32) << 2)
                | ((self.cc1s as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x18u32) as *mut u32, value) };
        }
    }
}
/// capture/compare mode register (output mode)
pub mod ccmr2_output {
    pub struct ReadonlyCache {
        /// Output compare 4 clear enable
        pub oc4ce: bool,
        /// Output compare 4 mode
        pub oc4m: bool,
        /// Output compare 4 preload enable
        pub oc4pe: bool,
        /// Output compare 4 fast enable
        pub oc4fe: bool,
        /// Capture/Compare 4 selection
        pub cc4s: bool,
        /// Output compare 3 clear enable
        pub oc3ce: bool,
        /// Output compare 3 mode
        pub oc3m: bool,
        /// Output compare 3 preload enable
        pub oc3pe: bool,
        /// Output compare 3 fast enable
        pub oc3fe: bool,
        /// Capture/Compare 3 selection
        pub cc3s: bool,
        /// Output Compare 3 mode bit 3
        pub oc3m_3: bool,
        /// Output Compare 4 mode bit 3
        pub oc4m_3: bool,
    }
    pub struct Cache {
        /// Output compare 4 clear enable
        pub oc4ce: bool,
        /// Output compare 4 mode
        pub oc4m: bool,
        /// Output compare 4 preload enable
        pub oc4pe: bool,
        /// Output compare 4 fast enable
        pub oc4fe: bool,
        /// Capture/Compare 4 selection
        pub cc4s: bool,
        /// Output compare 3 clear enable
        pub oc3ce: bool,
        /// Output compare 3 mode
        pub oc3m: bool,
        /// Output compare 3 preload enable
        pub oc3pe: bool,
        /// Output compare 3 fast enable
        pub oc3fe: bool,
        /// Capture/Compare 3 selection
        pub cc3s: bool,
        /// Output Compare 3 mode bit 3
        pub oc3m_3: bool,
        /// Output Compare 4 mode bit 3
        pub oc4m_3: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x1Cu32) as *mut u32) };
        ReadonlyCache {
            oc4ce: ((value >> 15) & 0b1) > 0,
            oc4m: ((value >> 12) & 0b1) > 0,
            oc4pe: ((value >> 11) & 0b1) > 0,
            oc4fe: ((value >> 10) & 0b1) > 0,
            cc4s: ((value >> 8) & 0b1) > 0,
            oc3ce: ((value >> 7) & 0b1) > 0,
            oc3m: ((value >> 4) & 0b1) > 0,
            oc3pe: ((value >> 3) & 0b1) > 0,
            oc3fe: ((value >> 2) & 0b1) > 0,
            cc3s: ((value >> 0) & 0b1) > 0,
            oc3m_3: ((value >> 16) & 0b1) > 0,
            oc4m_3: ((value >> 24) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x1Cu32) as *mut u32) };
        Cache {
            oc4ce: ((value >> 15) & 0b1) > 0,
            oc4m: ((value >> 12) & 0b1) > 0,
            oc4pe: ((value >> 11) & 0b1) > 0,
            oc4fe: ((value >> 10) & 0b1) > 0,
            cc4s: ((value >> 8) & 0b1) > 0,
            oc3ce: ((value >> 7) & 0b1) > 0,
            oc3m: ((value >> 4) & 0b1) > 0,
            oc3pe: ((value >> 3) & 0b1) > 0,
            oc3fe: ((value >> 2) & 0b1) > 0,
            cc3s: ((value >> 0) & 0b1) > 0,
            oc3m_3: ((value >> 16) & 0b1) > 0,
            oc4m_3: ((value >> 24) & 0b1) > 0,
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
                | ((self.oc4ce as u32) << 15)
                | ((self.oc4m as u32) << 12)
                | ((self.oc4pe as u32) << 11)
                | ((self.oc4fe as u32) << 10)
                | ((self.cc4s as u32) << 8)
                | ((self.oc3ce as u32) << 7)
                | ((self.oc3m as u32) << 4)
                | ((self.oc3pe as u32) << 3)
                | ((self.oc3fe as u32) << 2)
                | ((self.cc3s as u32) << 0)
                | ((self.oc3m_3 as u32) << 16)
                | ((self.oc4m_3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x1Cu32) as *mut u32, value) };
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
        /// Capture/compare 3 selection
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
        /// Capture/compare 3 selection
        pub cc3s: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x1Cu32) as *mut u32) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x1Cu32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x1Cu32) as *mut u32, value) };
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
        /// Capture/Compare 1 complementary output enable
        pub cc1ne: bool,
        /// Capture/Compare 1 output Polarity
        pub cc1np: bool,
        /// Capture/Compare 2 output enable
        pub cc2e: bool,
        /// Capture/Compare 2 output Polarity
        pub cc2p: bool,
        /// Capture/Compare 2 complementary output enable
        pub cc2ne: bool,
        /// Capture/Compare 2 output Polarity
        pub cc2np: bool,
        /// Capture/Compare 3 output enable
        pub cc3e: bool,
        /// Capture/Compare 3 output Polarity
        pub cc3p: bool,
        /// Capture/Compare 3 complementary output enable
        pub cc3ne: bool,
        /// Capture/Compare 3 output Polarity
        pub cc3np: bool,
        /// Capture/Compare 4 output enable
        pub cc4e: bool,
        /// Capture/Compare 3 output Polarity
        pub cc4p: bool,
        /// Capture/Compare 4 output Polarity
        pub cc4np: bool,
        /// Capture/Compare 5 output enable
        pub cc5e: bool,
        /// Capture/Compare 5 output Polarity
        pub cc5p: bool,
        /// Capture/Compare 6 output enable
        pub cc6e: bool,
        /// Capture/Compare 6 output Polarity
        pub cc6p: bool,
    }
    pub struct Cache {
        /// Capture/Compare 1 output enable
        pub cc1e: bool,
        /// Capture/Compare 1 output Polarity
        pub cc1p: bool,
        /// Capture/Compare 1 complementary output enable
        pub cc1ne: bool,
        /// Capture/Compare 1 output Polarity
        pub cc1np: bool,
        /// Capture/Compare 2 output enable
        pub cc2e: bool,
        /// Capture/Compare 2 output Polarity
        pub cc2p: bool,
        /// Capture/Compare 2 complementary output enable
        pub cc2ne: bool,
        /// Capture/Compare 2 output Polarity
        pub cc2np: bool,
        /// Capture/Compare 3 output enable
        pub cc3e: bool,
        /// Capture/Compare 3 output Polarity
        pub cc3p: bool,
        /// Capture/Compare 3 complementary output enable
        pub cc3ne: bool,
        /// Capture/Compare 3 output Polarity
        pub cc3np: bool,
        /// Capture/Compare 4 output enable
        pub cc4e: bool,
        /// Capture/Compare 3 output Polarity
        pub cc4p: bool,
        /// Capture/Compare 4 output Polarity
        pub cc4np: bool,
        /// Capture/Compare 5 output enable
        pub cc5e: bool,
        /// Capture/Compare 5 output Polarity
        pub cc5p: bool,
        /// Capture/Compare 6 output enable
        pub cc6e: bool,
        /// Capture/Compare 6 output Polarity
        pub cc6p: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x20u32) as *mut u32) };
        ReadonlyCache {
            cc1e: ((value >> 0) & 0b1) > 0,
            cc1p: ((value >> 1) & 0b1) > 0,
            cc1ne: ((value >> 2) & 0b1) > 0,
            cc1np: ((value >> 3) & 0b1) > 0,
            cc2e: ((value >> 4) & 0b1) > 0,
            cc2p: ((value >> 5) & 0b1) > 0,
            cc2ne: ((value >> 6) & 0b1) > 0,
            cc2np: ((value >> 7) & 0b1) > 0,
            cc3e: ((value >> 8) & 0b1) > 0,
            cc3p: ((value >> 9) & 0b1) > 0,
            cc3ne: ((value >> 10) & 0b1) > 0,
            cc3np: ((value >> 11) & 0b1) > 0,
            cc4e: ((value >> 12) & 0b1) > 0,
            cc4p: ((value >> 13) & 0b1) > 0,
            cc4np: ((value >> 15) & 0b1) > 0,
            cc5e: ((value >> 16) & 0b1) > 0,
            cc5p: ((value >> 17) & 0b1) > 0,
            cc6e: ((value >> 20) & 0b1) > 0,
            cc6p: ((value >> 21) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x20u32) as *mut u32) };
        Cache {
            cc1e: ((value >> 0) & 0b1) > 0,
            cc1p: ((value >> 1) & 0b1) > 0,
            cc1ne: ((value >> 2) & 0b1) > 0,
            cc1np: ((value >> 3) & 0b1) > 0,
            cc2e: ((value >> 4) & 0b1) > 0,
            cc2p: ((value >> 5) & 0b1) > 0,
            cc2ne: ((value >> 6) & 0b1) > 0,
            cc2np: ((value >> 7) & 0b1) > 0,
            cc3e: ((value >> 8) & 0b1) > 0,
            cc3p: ((value >> 9) & 0b1) > 0,
            cc3ne: ((value >> 10) & 0b1) > 0,
            cc3np: ((value >> 11) & 0b1) > 0,
            cc4e: ((value >> 12) & 0b1) > 0,
            cc4p: ((value >> 13) & 0b1) > 0,
            cc4np: ((value >> 15) & 0b1) > 0,
            cc5e: ((value >> 16) & 0b1) > 0,
            cc5p: ((value >> 17) & 0b1) > 0,
            cc6e: ((value >> 20) & 0b1) > 0,
            cc6p: ((value >> 21) & 0b1) > 0,
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
                | ((self.cc1ne as u32) << 2)
                | ((self.cc1np as u32) << 3)
                | ((self.cc2e as u32) << 4)
                | ((self.cc2p as u32) << 5)
                | ((self.cc2ne as u32) << 6)
                | ((self.cc2np as u32) << 7)
                | ((self.cc3e as u32) << 8)
                | ((self.cc3p as u32) << 9)
                | ((self.cc3ne as u32) << 10)
                | ((self.cc3np as u32) << 11)
                | ((self.cc4e as u32) << 12)
                | ((self.cc4p as u32) << 13)
                | ((self.cc4np as u32) << 15)
                | ((self.cc5e as u32) << 16)
                | ((self.cc5p as u32) << 17)
                | ((self.cc6e as u32) << 20)
                | ((self.cc6p as u32) << 21)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x20u32) as *mut u32, value) };
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
        unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x24u32) as *mut u32, value) };
    }
    /// Get counter value
    pub fn get_cnt() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
    /// UIF copy
    /// Access: read-only, Width: 1, Offset: 31
    /// Get UIF copy
    pub fn uifcpy() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x24u32) as *mut u32) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x28u32) as *mut u32) };
        ReadonlyCache {
            psc: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x28u32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x28u32) as *mut u32, value) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x2Cu32) as *mut u32) };
        ReadonlyCache {
            arr: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x2Cu32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x2Cu32) as *mut u32, value) };
        }
    }
}
/// repetition counter register
pub mod rcr {
    pub struct ReadonlyCache {
        /// Repetition counter value
        pub rep: u16,
    }
    pub struct Cache {
        /// Repetition counter value
        pub rep: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x30u32) as *mut u32) };
        ReadonlyCache {
            rep: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x30u32) as *mut u32) };
        Cache {
            rep: ((value >> 0) & 0b1111111111111111) as u16,
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
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x30u32) as *mut u32, value) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x34u32) as *mut u32) };
        ReadonlyCache {
            ccr1: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x34u32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x34u32) as *mut u32, value) };
        }
    }
}
/// capture/compare register 2
pub mod ccr2 {
    pub struct ReadonlyCache {
        /// Capture/Compare 2 value
        pub ccr2: u16,
    }
    pub struct Cache {
        /// Capture/Compare 2 value
        pub ccr2: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x38u32) as *mut u32) };
        ReadonlyCache {
            ccr2: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x38u32) as *mut u32) };
        Cache {
            ccr2: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.ccr2 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x38u32) as *mut u32, value) };
        }
    }
}
/// capture/compare register 3
pub mod ccr3 {
    pub struct ReadonlyCache {
        /// Capture/Compare 3 value
        pub ccr3: u16,
    }
    pub struct Cache {
        /// Capture/Compare 3 value
        pub ccr3: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x3Cu32) as *mut u32) };
        ReadonlyCache {
            ccr3: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x3Cu32) as *mut u32) };
        Cache {
            ccr3: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.ccr3 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x3Cu32) as *mut u32, value) };
        }
    }
}
/// capture/compare register 4
pub mod ccr4 {
    pub struct ReadonlyCache {
        /// Capture/Compare 3 value
        pub ccr4: u16,
    }
    pub struct Cache {
        /// Capture/Compare 3 value
        pub ccr4: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x40u32) as *mut u32) };
        ReadonlyCache {
            ccr4: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x40u32) as *mut u32) };
        Cache {
            ccr4: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.ccr4 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x40u32) as *mut u32, value) };
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
        /// Break 2 filter
        pub bk2f: u8,
        /// Break 2 enable
        pub bk2e: u8,
        /// Break 2 polarity
        pub bk2p: u8,
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
        /// Break 2 filter
        pub bk2f: u8,
        /// Break 2 enable
        pub bk2e: u8,
        /// Break 2 polarity
        pub bk2p: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x44u32) as *mut u32) };
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
            bk2f: ((value >> 20) & 0b11111111) as u8,
            bk2e: ((value >> 24) & 0b11111111) as u8,
            bk2p: ((value >> 25) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x44u32) as *mut u32) };
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
            bk2f: ((value >> 20) & 0b11111111) as u8,
            bk2e: ((value >> 24) & 0b11111111) as u8,
            bk2p: ((value >> 25) & 0b11111111) as u8,
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
                | ((self.bk2f as u32) << 20)
                | ((self.bk2e as u32) << 24)
                | ((self.bk2p as u32) << 25)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x44u32) as *mut u32, value) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x48u32) as *mut u32) };
        ReadonlyCache {
            dbl: ((value >> 8) & 0b11111) as u8,
            dba: ((value >> 0) & 0b11111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x48u32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x48u32) as *mut u32, value) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x4Cu32) as *mut u32) };
        ReadonlyCache {
            dmab: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x4Cu32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x4Cu32) as *mut u32, value) };
        }
    }
}
/// capture/compare mode register 3 (output mode)
pub mod ccmr3_output {
    pub struct ReadonlyCache {
        /// Output compare 5 fast enable
        pub oc5fe: bool,
        /// Output compare 5 preload enable
        pub oc5pe: bool,
        /// Output compare 5 mode
        pub oc5m: bool,
        /// Output compare 5 clear enable
        pub oc5ce: bool,
        /// Output compare 6 fast enable
        pub oc6fe: bool,
        /// Output compare 6 preload enable
        pub oc6pe: bool,
        /// Output compare 6 mode
        pub oc6m: bool,
        /// Output compare 6 clear enable
        pub oc6ce: bool,
        /// Outout Compare 5 mode bit 3
        pub oc5m_3: bool,
        /// Outout Compare 6 mode bit 3
        pub oc6m_3: bool,
    }
    pub struct Cache {
        /// Output compare 5 fast enable
        pub oc5fe: bool,
        /// Output compare 5 preload enable
        pub oc5pe: bool,
        /// Output compare 5 mode
        pub oc5m: bool,
        /// Output compare 5 clear enable
        pub oc5ce: bool,
        /// Output compare 6 fast enable
        pub oc6fe: bool,
        /// Output compare 6 preload enable
        pub oc6pe: bool,
        /// Output compare 6 mode
        pub oc6m: bool,
        /// Output compare 6 clear enable
        pub oc6ce: bool,
        /// Outout Compare 5 mode bit 3
        pub oc5m_3: bool,
        /// Outout Compare 6 mode bit 3
        pub oc6m_3: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x54u32) as *mut u32) };
        ReadonlyCache {
            oc5fe: ((value >> 2) & 0b1) > 0,
            oc5pe: ((value >> 3) & 0b1) > 0,
            oc5m: ((value >> 4) & 0b1) > 0,
            oc5ce: ((value >> 7) & 0b1) > 0,
            oc6fe: ((value >> 10) & 0b1) > 0,
            oc6pe: ((value >> 11) & 0b1) > 0,
            oc6m: ((value >> 12) & 0b1) > 0,
            oc6ce: ((value >> 15) & 0b1) > 0,
            oc5m_3: ((value >> 16) & 0b1) > 0,
            oc6m_3: ((value >> 24) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x54u32) as *mut u32) };
        Cache {
            oc5fe: ((value >> 2) & 0b1) > 0,
            oc5pe: ((value >> 3) & 0b1) > 0,
            oc5m: ((value >> 4) & 0b1) > 0,
            oc5ce: ((value >> 7) & 0b1) > 0,
            oc6fe: ((value >> 10) & 0b1) > 0,
            oc6pe: ((value >> 11) & 0b1) > 0,
            oc6m: ((value >> 12) & 0b1) > 0,
            oc6ce: ((value >> 15) & 0b1) > 0,
            oc5m_3: ((value >> 16) & 0b1) > 0,
            oc6m_3: ((value >> 24) & 0b1) > 0,
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
                | ((self.oc5fe as u32) << 2)
                | ((self.oc5pe as u32) << 3)
                | ((self.oc5m as u32) << 4)
                | ((self.oc5ce as u32) << 7)
                | ((self.oc6fe as u32) << 10)
                | ((self.oc6pe as u32) << 11)
                | ((self.oc6m as u32) << 12)
                | ((self.oc6ce as u32) << 15)
                | ((self.oc5m_3 as u32) << 16)
                | ((self.oc6m_3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x54u32) as *mut u32, value) };
        }
    }
}
/// capture/compare register 5
pub mod ccr5 {
    pub struct ReadonlyCache {
        /// Capture/Compare 5 value
        pub ccr5: u16,
        /// Group Channel 5 and Channel 1
        pub gc5c1: u16,
        /// Group Channel 5 and Channel 2
        pub gc5c2: u16,
        /// Group Channel 5 and Channel 3
        pub gc5c3: u16,
    }
    pub struct Cache {
        /// Capture/Compare 5 value
        pub ccr5: u16,
        /// Group Channel 5 and Channel 1
        pub gc5c1: u16,
        /// Group Channel 5 and Channel 2
        pub gc5c2: u16,
        /// Group Channel 5 and Channel 3
        pub gc5c3: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x58u32) as *mut u32) };
        ReadonlyCache {
            ccr5: ((value >> 0) & 0b1111111111111111) as u16,
            gc5c1: ((value >> 29) & 0b1111111111111111) as u16,
            gc5c2: ((value >> 30) & 0b1111111111111111) as u16,
            gc5c3: ((value >> 31) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x58u32) as *mut u32) };
        Cache {
            ccr5: ((value >> 0) & 0b1111111111111111) as u16,
            gc5c1: ((value >> 29) & 0b1111111111111111) as u16,
            gc5c2: ((value >> 30) & 0b1111111111111111) as u16,
            gc5c3: ((value >> 31) & 0b1111111111111111) as u16,
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
                | ((self.ccr5 as u32) << 0)
                | ((self.gc5c1 as u32) << 29)
                | ((self.gc5c2 as u32) << 30)
                | ((self.gc5c3 as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x58u32) as *mut u32, value) };
        }
    }
}
/// capture/compare register 6
pub mod ccr6 {
    pub struct ReadonlyCache {
        /// Capture/Compare 6 value
        pub ccr6: u16,
    }
    pub struct Cache {
        /// Capture/Compare 6 value
        pub ccr6: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x5Cu32) as *mut u32) };
        ReadonlyCache {
            ccr6: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x5Cu32) as *mut u32) };
        Cache {
            ccr6: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.ccr6 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x5Cu32) as *mut u32, value) };
        }
    }
}
/// option registers
pub mod or {
    pub struct ReadonlyCache {
        /// TIM1_ETR_ADC1 remapping capability
        pub tim1_etr_adc1_rmp: u8,
        /// TIM1_ETR_ADC4 remapping capability
        pub tim1_etr_adc4_rmp: u8,
    }
    pub struct Cache {
        /// TIM1_ETR_ADC1 remapping capability
        pub tim1_etr_adc1_rmp: u8,
        /// TIM1_ETR_ADC4 remapping capability
        pub tim1_etr_adc4_rmp: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x60u32) as *mut u32) };
        ReadonlyCache {
            tim1_etr_adc1_rmp: ((value >> 0) & 0b11) as u8,
            tim1_etr_adc4_rmp: ((value >> 2) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40012C00u32 + 0x60u32) as *mut u32) };
        Cache {
            tim1_etr_adc1_rmp: ((value >> 0) & 0b11) as u8,
            tim1_etr_adc4_rmp: ((value >> 2) & 0b11) as u8,
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
                | ((self.tim1_etr_adc1_rmp as u32) << 0)
                | ((self.tim1_etr_adc4_rmp as u32) << 2)
            ;
            unsafe { ::core::ptr::write_volatile((0x40012C00u32 + 0x60u32) as *mut u32, value) };
        }
    }
}
/// TIM1 capture compare interrupt
pub const INTERRUPT_TIM1_CC: u32 = 27;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40012C00</baseAddress>
  <description>Advanced timer</description>
  <groupName>TIMs</groupName>
  <interrupt>
    <description>TIM1 capture compare interrupt</description>
    <name>TIM1_CC</name>
    <value>27</value>
  </interrupt>
  <name>TIM1</name>
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
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/compare preloaded
                                control
                            </description>
          <name>CCPC</name>
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
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/compare DMA
                                selection
                            </description>
          <name>CCDS</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Master mode selection</description>
          <name>MMS</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TI1 selection</description>
          <name>TI1S</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Idle state 1</description>
          <name>OIS1</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Idle state 1</description>
          <name>OIS1N</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Idle state 2</description>
          <name>OIS2</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Idle state 2</description>
          <name>OIS2N</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Idle state 3</description>
          <name>OIS3</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Idle state 3</description>
          <name>OIS3N</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Idle state 4</description>
          <name>OIS4</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Idle state 5</description>
          <name>OIS5</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Idle state 6</description>
          <name>OIS6</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Master mode selection 2</description>
          <name>MMS2</name>
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
          <description>Slave mode selection bit 3</description>
          <name>SMS3</name>
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
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reserved</description>
          <name>COMDE</name>
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
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Break interrupt enable</description>
          <name>BIE</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Trigger interrupt enable</description>
          <name>TIE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>COM interrupt enable</description>
          <name>COMIE</name>
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
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update interrupt flag</description>
          <name>UIF</name>
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
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 2 interrupt
                                flag
                            </description>
          <name>CC2IF</name>
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
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 4 interrupt
                                flag
                            </description>
          <name>CC4IF</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>COM interrupt flag</description>
          <name>COMIF</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Trigger interrupt flag</description>
          <name>TIF</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Break interrupt flag</description>
          <name>BIF</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Break 2 interrupt flag</description>
          <name>B2IF</name>
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
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/compare 2 overcapture
                                flag
                            </description>
          <name>CC2OF</name>
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
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 4 overcapture
                                flag
                            </description>
          <name>CC4OF</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 5 interrupt
                                flag
                            </description>
          <name>C5IF</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 6 interrupt
                                flag
                            </description>
          <name>C6IF</name>
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
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update generation</description>
          <name>UG</name>
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
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/compare 2
                                generation
                            </description>
          <name>CC2G</name>
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
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/compare 4
                                generation
                            </description>
          <name>CC4G</name>
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
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Trigger generation</description>
          <name>TG</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Break generation</description>
          <name>BG</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Break 2 generation</description>
          <name>B2G</name>
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
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output Compare 2 clear
                                enable
                            </description>
          <name>OC2CE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Output Compare 2 mode</description>
          <name>OC2M</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output Compare 2 preload
                                enable
                            </description>
          <name>OC2PE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output Compare 2 fast
                                enable
                            </description>
          <name>OC2FE</name>
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
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output Compare 1 clear
                                enable
                            </description>
          <name>OC1CE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Output Compare 1 mode</description>
          <name>OC1M</name>
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
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output Compare 1 fast
                                enable
                            </description>
          <name>OC1FE</name>
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
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output Compare 1 mode bit
                                3
                            </description>
          <name>OC1M_3</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output Compare 2 mode bit
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
          <name>IC2PCS</name>
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
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Input capture 1 filter</description>
          <name>IC1F</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Input capture 1 prescaler</description>
          <name>IC1PCS</name>
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
                        capture/compare mode register (output
                        mode)
                    </description>
      <displayName>CCMR2_Output</displayName>
      <fields>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 4 clear
                                enable
                            </description>
          <name>OC4CE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Output compare 4 mode</description>
          <name>OC4M</name>
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
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 4 fast
                                enable
                            </description>
          <name>OC4FE</name>
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
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 3 clear
                                enable
                            </description>
          <name>OC3CE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Output compare 3 mode</description>
          <name>OC3M</name>
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
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 3 fast
                                enable
                            </description>
          <name>OC3FE</name>
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
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output Compare 3 mode bit
                                3
                            </description>
          <name>OC3M_3</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output Compare 4 mode bit
                                3
                            </description>
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
                                Capture/compare 3
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
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 1 complementary output
                                enable
                            </description>
          <name>CC1NE</name>
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
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 2 complementary output
                                enable
                            </description>
          <name>CC2NE</name>
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
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 3 complementary output
                                enable
                            </description>
          <name>CC3NE</name>
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
                                Capture/Compare 4 output
                                Polarity
                            </description>
          <name>CC4NP</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 5 output
                                enable
                            </description>
          <name>CC5E</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 5 output
                                Polarity
                            </description>
          <name>CC5P</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 6 output
                                enable
                            </description>
          <name>CC6E</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 6 output
                                Polarity
                            </description>
          <name>CC6P</name>
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
          <description>UIF copy</description>
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
          <bitWidth>16</bitWidth>
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
      <addressOffset>0x38</addressOffset>
      <description>capture/compare register 2</description>
      <displayName>CCR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Capture/Compare 2 value</description>
          <name>CCR2</name>
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
          <description>Capture/Compare 3 value</description>
          <name>CCR3</name>
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
          <description>Capture/Compare 3 value</description>
          <name>CCR4</name>
        </field>
      </fields>
      <name>CCR4</name>
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
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Break 2 filter</description>
          <name>BK2F</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Break 2 enable</description>
          <name>BK2E</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Break 2 polarity</description>
          <name>BK2P</name>
        </field>
      </fields>
      <name>BDTR</name>
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
      <resetValue>0x00000000</resetValue>
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
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x54</addressOffset>
      <description>
                        capture/compare mode register 3 (output
                        mode)
                    </description>
      <displayName>CCMR3_Output</displayName>
      <fields>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 5 fast
                                enable
                            </description>
          <name>OC5FE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 5 preload
                                enable
                            </description>
          <name>OC5PE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Output compare 5 mode</description>
          <name>OC5M</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 5 clear
                                enable
                            </description>
          <name>OC5CE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 6 fast
                                enable
                            </description>
          <name>OC6FE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 6 preload
                                enable
                            </description>
          <name>OC6PE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Output compare 6 mode</description>
          <name>OC6M</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Output compare 6 clear
                                enable
                            </description>
          <name>OC6CE</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Outout Compare 5 mode bit
                                3
                            </description>
          <name>OC5M_3</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Outout Compare 6 mode bit
                                3
                            </description>
          <name>OC6M_3</name>
        </field>
      </fields>
      <name>CCMR3_Output</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x58</addressOffset>
      <description>capture/compare register 5</description>
      <displayName>CCR5</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Capture/Compare 5 value</description>
          <name>CCR5</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Group Channel 5 and Channel
                                1
                            </description>
          <name>GC5C1</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Group Channel 5 and Channel
                                2
                            </description>
          <name>GC5C2</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Group Channel 5 and Channel
                                3
                            </description>
          <name>GC5C3</name>
        </field>
      </fields>
      <name>CCR5</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x5C</addressOffset>
      <description>capture/compare register 6</description>
      <displayName>CCR6</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Capture/Compare 6 value</description>
          <name>CCR6</name>
        </field>
      </fields>
      <name>CCR6</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x60</addressOffset>
      <description>option registers</description>
      <displayName>OR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                TIM1_ETR_ADC1 remapping
                                capability
                            </description>
          <name>TIM1_ETR_ADC1_RMP</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                TIM1_ETR_ADC4 remapping
                                capability
                            </description>
          <name>TIM1_ETR_ADC4_RMP</name>
        </field>
      </fields>
      <name>OR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
