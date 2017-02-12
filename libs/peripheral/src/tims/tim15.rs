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
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x0u32) as *mut u32) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x0u32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x0u32) as *mut u32, value) };
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
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            ccpc: ((value >> 0) & 0b1) > 0,
            ccus: ((value >> 2) & 0b1) > 0,
            ccds: ((value >> 3) & 0b1) > 0,
            mms: ((value >> 4) & 0b1) > 0,
            ti1s: ((value >> 7) & 0b1) > 0,
            ois1: ((value >> 8) & 0b1) > 0,
            ois1n: ((value >> 9) & 0b1) > 0,
            ois2: ((value >> 10) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x4u32) as *mut u32) };
        Cache {
            ccpc: ((value >> 0) & 0b1) > 0,
            ccus: ((value >> 2) & 0b1) > 0,
            ccds: ((value >> 3) & 0b1) > 0,
            mms: ((value >> 4) & 0b1) > 0,
            ti1s: ((value >> 7) & 0b1) > 0,
            ois1: ((value >> 8) & 0b1) > 0,
            ois1n: ((value >> 9) & 0b1) > 0,
            ois2: ((value >> 10) & 0b1) > 0,
        }
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
            ;
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// slave mode control register
pub mod smcr {
    pub struct ReadonlyCache {
        /// Slave mode selection
        pub sms: u8,
        /// Trigger selection
        pub ts: u8,
        /// Master/Slave mode
        pub msm: u8,
        /// Slave mode selection bit 3
        pub sms_3: u8,
    }
    pub struct Cache {
        /// Slave mode selection
        pub sms: u8,
        /// Trigger selection
        pub ts: u8,
        /// Master/Slave mode
        pub msm: u8,
        /// Slave mode selection bit 3
        pub sms_3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            sms: ((value >> 0) & 0b111) as u8,
            ts: ((value >> 4) & 0b111) as u8,
            msm: ((value >> 7) & 0b111) as u8,
            sms_3: ((value >> 16) & 0b111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x8u32) as *mut u32) };
        Cache {
            sms: ((value >> 0) & 0b111) as u8,
            ts: ((value >> 4) & 0b111) as u8,
            msm: ((value >> 7) & 0b111) as u8,
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
                | ((self.ts as u32) << 4)
                | ((self.msm as u32) << 7)
                | ((self.sms_3 as u32) << 16)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x8u32) as *mut u32, value) };
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
        /// Capture/Compare 2 interrupt enable
        pub cc2ie: bool,
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
        /// Capture/Compare 2 DMA request enable
        pub cc2de: bool,
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
        /// Capture/Compare 2 interrupt enable
        pub cc2ie: bool,
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
        /// Capture/Compare 2 DMA request enable
        pub cc2de: bool,
        /// COM DMA request enable
        pub comde: bool,
        /// Trigger DMA request enable
        pub tde: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            uie: ((value >> 0) & 0b1) > 0,
            cc1ie: ((value >> 1) & 0b1) > 0,
            cc2ie: ((value >> 2) & 0b1) > 0,
            comie: ((value >> 5) & 0b1) > 0,
            tie: ((value >> 6) & 0b1) > 0,
            bie: ((value >> 7) & 0b1) > 0,
            ude: ((value >> 8) & 0b1) > 0,
            cc1de: ((value >> 9) & 0b1) > 0,
            cc2de: ((value >> 10) & 0b1) > 0,
            comde: ((value >> 13) & 0b1) > 0,
            tde: ((value >> 14) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0xCu32) as *mut u32) };
        Cache {
            uie: ((value >> 0) & 0b1) > 0,
            cc1ie: ((value >> 1) & 0b1) > 0,
            cc2ie: ((value >> 2) & 0b1) > 0,
            comie: ((value >> 5) & 0b1) > 0,
            tie: ((value >> 6) & 0b1) > 0,
            bie: ((value >> 7) & 0b1) > 0,
            ude: ((value >> 8) & 0b1) > 0,
            cc1de: ((value >> 9) & 0b1) > 0,
            cc2de: ((value >> 10) & 0b1) > 0,
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
                | ((self.cc2ie as u32) << 2)
                | ((self.comie as u32) << 5)
                | ((self.tie as u32) << 6)
                | ((self.bie as u32) << 7)
                | ((self.ude as u32) << 8)
                | ((self.cc1de as u32) << 9)
                | ((self.cc2de as u32) << 10)
                | ((self.comde as u32) << 13)
                | ((self.tde as u32) << 14)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// status register
pub mod sr {
    pub struct ReadonlyCache {
        /// Capture/compare 2 overcapture flag
        pub cc2of: bool,
        /// Capture/Compare 1 overcapture flag
        pub cc1of: bool,
        /// Break interrupt flag
        pub bif: bool,
        /// Trigger interrupt flag
        pub tif: bool,
        /// COM interrupt flag
        pub comif: bool,
        /// Capture/Compare 2 interrupt flag
        pub cc2if: bool,
        /// Capture/compare 1 interrupt flag
        pub cc1if: bool,
        /// Update interrupt flag
        pub uif: bool,
    }
    pub struct Cache {
        /// Capture/compare 2 overcapture flag
        pub cc2of: bool,
        /// Capture/Compare 1 overcapture flag
        pub cc1of: bool,
        /// Break interrupt flag
        pub bif: bool,
        /// Trigger interrupt flag
        pub tif: bool,
        /// COM interrupt flag
        pub comif: bool,
        /// Capture/Compare 2 interrupt flag
        pub cc2if: bool,
        /// Capture/compare 1 interrupt flag
        pub cc1if: bool,
        /// Update interrupt flag
        pub uif: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            cc2of: ((value >> 10) & 0b1) > 0,
            cc1of: ((value >> 9) & 0b1) > 0,
            bif: ((value >> 7) & 0b1) > 0,
            tif: ((value >> 6) & 0b1) > 0,
            comif: ((value >> 5) & 0b1) > 0,
            cc2if: ((value >> 2) & 0b1) > 0,
            cc1if: ((value >> 1) & 0b1) > 0,
            uif: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x10u32) as *mut u32) };
        Cache {
            cc2of: ((value >> 10) & 0b1) > 0,
            cc1of: ((value >> 9) & 0b1) > 0,
            bif: ((value >> 7) & 0b1) > 0,
            tif: ((value >> 6) & 0b1) > 0,
            comif: ((value >> 5) & 0b1) > 0,
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
                | ((self.cc2of as u32) << 10)
                | ((self.cc1of as u32) << 9)
                | ((self.bif as u32) << 7)
                | ((self.tif as u32) << 6)
                | ((self.comif as u32) << 5)
                | ((self.cc2if as u32) << 2)
                | ((self.cc1if as u32) << 1)
                | ((self.uif as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x10u32) as *mut u32, value) };
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
        unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x14u32) as *mut u32, value) };
    }
    /// Trigger generation
    /// Access: write-only, Width: 1, Offset: 6
    /// Set Trigger generation
    pub fn tg(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x14u32) as *mut u32, value) };
    }
    /// Capture/Compare control update generation
    /// Access: write-only, Width: 1, Offset: 5
    /// Set Capture/Compare control update generation
    pub fn comg(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x14u32) as *mut u32, value) };
    }
    /// Capture/compare 2 generation
    /// Access: write-only, Width: 1, Offset: 2
    /// Set Capture/compare 2 generation
    pub fn cc2g(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x14u32) as *mut u32, value) };
    }
    /// Capture/compare 1 generation
    /// Access: write-only, Width: 1, Offset: 1
    /// Set Capture/compare 1 generation
    pub fn cc1g(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x14u32) as *mut u32, value) };
    }
    /// Update generation
    /// Access: write-only, Width: 1, Offset: 0
    /// Set Update generation
    pub fn ug(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x14u32) as *mut u32, value) };
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
        /// Capture/Compare 2 selection
        pub cc2s: u8,
        /// Output Compare 2 fast enable
        pub oc2fe: u8,
        /// Output Compare 2 preload enable
        pub oc2pe: u8,
        /// Output Compare 2 mode
        pub oc2m: u8,
        /// Output Compare 1 mode bit 3
        pub oc1m_3: u8,
        /// Output Compare 2 mode bit 3
        pub oc2m_3: u8,
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
        /// Capture/Compare 2 selection
        pub cc2s: u8,
        /// Output Compare 2 fast enable
        pub oc2fe: u8,
        /// Output Compare 2 preload enable
        pub oc2pe: u8,
        /// Output Compare 2 mode
        pub oc2m: u8,
        /// Output Compare 1 mode bit 3
        pub oc1m_3: u8,
        /// Output Compare 2 mode bit 3
        pub oc2m_3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x18u32) as *mut u32) };
        ReadonlyCache {
            cc1s: ((value >> 0) & 0b11) as u8,
            oc1fe: ((value >> 2) & 0b11) as u8,
            oc1pe: ((value >> 3) & 0b11) as u8,
            oc1m: ((value >> 4) & 0b11) as u8,
            cc2s: ((value >> 8) & 0b11) as u8,
            oc2fe: ((value >> 10) & 0b11) as u8,
            oc2pe: ((value >> 11) & 0b11) as u8,
            oc2m: ((value >> 12) & 0b11) as u8,
            oc1m_3: ((value >> 16) & 0b11) as u8,
            oc2m_3: ((value >> 24) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x18u32) as *mut u32) };
        Cache {
            cc1s: ((value >> 0) & 0b11) as u8,
            oc1fe: ((value >> 2) & 0b11) as u8,
            oc1pe: ((value >> 3) & 0b11) as u8,
            oc1m: ((value >> 4) & 0b11) as u8,
            cc2s: ((value >> 8) & 0b11) as u8,
            oc2fe: ((value >> 10) & 0b11) as u8,
            oc2pe: ((value >> 11) & 0b11) as u8,
            oc2m: ((value >> 12) & 0b11) as u8,
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
                | ((self.cc2s as u32) << 8)
                | ((self.oc2fe as u32) << 10)
                | ((self.oc2pe as u32) << 11)
                | ((self.oc2m as u32) << 12)
                | ((self.oc1m_3 as u32) << 16)
                | ((self.oc2m_3 as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x18u32) as *mut u32, value) };
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
        /// Capture/Compare 2 selection
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
        /// Capture/Compare 2 selection
        pub cc2s: u8,
        /// Input capture 1 filter
        pub ic1f: u8,
        /// Input capture 1 prescaler
        pub ic1psc: u8,
        /// Capture/Compare 1 selection
        pub cc1s: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x18u32) as *mut u32) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x18u32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x18u32) as *mut u32, value) };
        }
    }
}
/// capture/compare enable register
pub mod ccer {
    pub struct ReadonlyCache {
        /// Capture/Compare 2 output Polarity
        pub cc2np: bool,
        /// Capture/Compare 2 output Polarity
        pub cc2p: bool,
        /// Capture/Compare 2 output enable
        pub cc2e: bool,
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
        /// Capture/Compare 2 output Polarity
        pub cc2np: bool,
        /// Capture/Compare 2 output Polarity
        pub cc2p: bool,
        /// Capture/Compare 2 output enable
        pub cc2e: bool,
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
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x20u32) as *mut u32) };
        ReadonlyCache {
            cc2np: ((value >> 7) & 0b1) > 0,
            cc2p: ((value >> 5) & 0b1) > 0,
            cc2e: ((value >> 4) & 0b1) > 0,
            cc1np: ((value >> 3) & 0b1) > 0,
            cc1ne: ((value >> 2) & 0b1) > 0,
            cc1p: ((value >> 1) & 0b1) > 0,
            cc1e: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x20u32) as *mut u32) };
        Cache {
            cc2np: ((value >> 7) & 0b1) > 0,
            cc2p: ((value >> 5) & 0b1) > 0,
            cc2e: ((value >> 4) & 0b1) > 0,
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
                | ((self.cc2np as u32) << 7)
                | ((self.cc2p as u32) << 5)
                | ((self.cc2e as u32) << 4)
                | ((self.cc1np as u32) << 3)
                | ((self.cc1ne as u32) << 2)
                | ((self.cc1p as u32) << 1)
                | ((self.cc1e as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x20u32) as *mut u32, value) };
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
        unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x24u32) as *mut u32, value) };
    }
    /// Get counter value
    pub fn get_cnt() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
    /// UIF copy
    /// Access: read-only, Width: 1, Offset: 31
    /// Get UIF copy
    pub fn uifcpy() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x24u32) as *mut u32) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x28u32) as *mut u32) };
        ReadonlyCache {
            psc: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x28u32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x28u32) as *mut u32, value) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x2Cu32) as *mut u32) };
        ReadonlyCache {
            arr: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x2Cu32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x2Cu32) as *mut u32, value) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x30u32) as *mut u32) };
        ReadonlyCache {
            rep: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x30u32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x30u32) as *mut u32, value) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x34u32) as *mut u32) };
        ReadonlyCache {
            ccr1: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x34u32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x34u32) as *mut u32, value) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x38u32) as *mut u32) };
        ReadonlyCache {
            ccr2: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x38u32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x38u32) as *mut u32, value) };
        }
    }
}
/// break and dead-time register
pub mod bdtr {
    pub struct ReadonlyCache {
        /// Main output enable
        pub moe: bool,
        /// Automatic output enable
        pub aoe: bool,
        /// Break polarity
        pub bkp: bool,
        /// Break enable
        pub bke: bool,
        /// Off-state selection for Run mode
        pub ossr: bool,
        /// Off-state selection for Idle mode
        pub ossi: bool,
        /// Lock configuration
        pub lock: bool,
        /// Dead-time generator setup
        pub dtg: bool,
        /// Break filter
        pub bkf: bool,
    }
    pub struct Cache {
        /// Main output enable
        pub moe: bool,
        /// Automatic output enable
        pub aoe: bool,
        /// Break polarity
        pub bkp: bool,
        /// Break enable
        pub bke: bool,
        /// Off-state selection for Run mode
        pub ossr: bool,
        /// Off-state selection for Idle mode
        pub ossi: bool,
        /// Lock configuration
        pub lock: bool,
        /// Dead-time generator setup
        pub dtg: bool,
        /// Break filter
        pub bkf: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x44u32) as *mut u32) };
        ReadonlyCache {
            moe: ((value >> 15) & 0b1) > 0,
            aoe: ((value >> 14) & 0b1) > 0,
            bkp: ((value >> 13) & 0b1) > 0,
            bke: ((value >> 12) & 0b1) > 0,
            ossr: ((value >> 11) & 0b1) > 0,
            ossi: ((value >> 10) & 0b1) > 0,
            lock: ((value >> 8) & 0b1) > 0,
            dtg: ((value >> 0) & 0b1) > 0,
            bkf: ((value >> 16) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x44u32) as *mut u32) };
        Cache {
            moe: ((value >> 15) & 0b1) > 0,
            aoe: ((value >> 14) & 0b1) > 0,
            bkp: ((value >> 13) & 0b1) > 0,
            bke: ((value >> 12) & 0b1) > 0,
            ossr: ((value >> 11) & 0b1) > 0,
            ossi: ((value >> 10) & 0b1) > 0,
            lock: ((value >> 8) & 0b1) > 0,
            dtg: ((value >> 0) & 0b1) > 0,
            bkf: ((value >> 16) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.moe as u32) << 15)
                | ((self.aoe as u32) << 14)
                | ((self.bkp as u32) << 13)
                | ((self.bke as u32) << 12)
                | ((self.ossr as u32) << 11)
                | ((self.ossi as u32) << 10)
                | ((self.lock as u32) << 8)
                | ((self.dtg as u32) << 0)
                | ((self.bkf as u32) << 16)
            ;
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x44u32) as *mut u32, value) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x48u32) as *mut u32) };
        ReadonlyCache {
            dbl: ((value >> 8) & 0b11111) as u8,
            dba: ((value >> 0) & 0b11111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x48u32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x48u32) as *mut u32, value) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x4Cu32) as *mut u32) };
        ReadonlyCache {
            dmab: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40014000u32 + 0x4Cu32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40014000u32 + 0x4Cu32) as *mut u32, value) };
        }
    }
}
/// TIM1 Break/TIM15 global interruts
pub const INTERRUPT_TIM1_BRK_TIM15: u32 = 24;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40014000</baseAddress>
  <description>General purpose timers</description>
  <groupName>TIMs</groupName>
  <interrupt>
    <description>
                    TIM1 Break/TIM15 global
                    interruts
                </description>
    <name>TIM1_BRK_TIM15</name>
    <value>24</value>
  </interrupt>
  <name>TIM15</name>
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
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Slave mode selection bit 3</description>
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
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 2 interrupt
                                enable
                            </description>
          <name>CC2IE</name>
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
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 2 DMA request
                                enable
                            </description>
          <name>CC2DE</name>
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
                                Output Compare 2 fast
                                enable
                            </description>
          <name>OC2FE</name>
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
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Output Compare 2 mode</description>
          <name>OC2M</name>
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
          <name>IC2PSC</name>
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
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 2 output
                                Polarity
                            </description>
          <name>CC2NP</name>
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
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Capture/Compare 2 output
                                enable
                            </description>
          <name>CC2E</name>
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
      <addressOffset>0x44</addressOffset>
      <description>break and dead-time register</description>
      <displayName>BDTR</displayName>
      <fields>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Main output enable</description>
          <name>MOE</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Automatic output enable</description>
          <name>AOE</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Break polarity</description>
          <name>BKP</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Break enable</description>
          <name>BKE</name>
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
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Off-state selection for Idle
                                mode
                            </description>
          <name>OSSI</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Lock configuration</description>
          <name>LOCK</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Dead-time generator setup</description>
          <name>DTG</name>
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
  </registers>
</peripheral>
*/
