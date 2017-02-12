/// control register
pub mod cr {
    pub struct ReadonlyCache {
        /// DAC channel2 DMA underrun interrupt enable
        pub dmaudrie2: bool,
        /// DAC channel2 DMA enable
        pub dmaen2: bool,
        /// DAC channel2 mask/amplitude selector
        pub mamp2: bool,
        /// DAC channel2 noise/triangle wave generation enable
        pub wave2: bool,
        /// DAC channel2 trigger selection
        pub tsel2: bool,
        /// DAC channel2 trigger enable
        pub ten2: bool,
        /// DAC channel2 output buffer disable
        pub boff2: bool,
        /// DAC channel2 enable
        pub en2: bool,
        /// DAC channel1 DMA Underrun Interrupt enable
        pub dmaudrie1: bool,
        /// DAC channel1 DMA enable
        pub dmaen1: bool,
        /// DAC channel1 mask/amplitude selector
        pub mamp1: bool,
        /// DAC channel1 noise/triangle wave generation enable
        pub wave1: bool,
        /// DAC channel1 trigger selection
        pub tsel1: bool,
        /// DAC channel1 trigger enable
        pub ten1: bool,
        /// DAC channel1 output buffer disable
        pub boff1: bool,
        /// DAC channel1 enable
        pub en1: bool,
    }
    pub struct Cache {
        /// DAC channel2 DMA underrun interrupt enable
        pub dmaudrie2: bool,
        /// DAC channel2 DMA enable
        pub dmaen2: bool,
        /// DAC channel2 mask/amplitude selector
        pub mamp2: bool,
        /// DAC channel2 noise/triangle wave generation enable
        pub wave2: bool,
        /// DAC channel2 trigger selection
        pub tsel2: bool,
        /// DAC channel2 trigger enable
        pub ten2: bool,
        /// DAC channel2 output buffer disable
        pub boff2: bool,
        /// DAC channel2 enable
        pub en2: bool,
        /// DAC channel1 DMA Underrun Interrupt enable
        pub dmaudrie1: bool,
        /// DAC channel1 DMA enable
        pub dmaen1: bool,
        /// DAC channel1 mask/amplitude selector
        pub mamp1: bool,
        /// DAC channel1 noise/triangle wave generation enable
        pub wave1: bool,
        /// DAC channel1 trigger selection
        pub tsel1: bool,
        /// DAC channel1 trigger enable
        pub ten1: bool,
        /// DAC channel1 output buffer disable
        pub boff1: bool,
        /// DAC channel1 enable
        pub en1: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            dmaudrie2: ((value >> 29) & 0b1) > 0,
            dmaen2: ((value >> 28) & 0b1) > 0,
            mamp2: ((value >> 24) & 0b1) > 0,
            wave2: ((value >> 22) & 0b1) > 0,
            tsel2: ((value >> 19) & 0b1) > 0,
            ten2: ((value >> 18) & 0b1) > 0,
            boff2: ((value >> 17) & 0b1) > 0,
            en2: ((value >> 16) & 0b1) > 0,
            dmaudrie1: ((value >> 13) & 0b1) > 0,
            dmaen1: ((value >> 12) & 0b1) > 0,
            mamp1: ((value >> 8) & 0b1) > 0,
            wave1: ((value >> 6) & 0b1) > 0,
            tsel1: ((value >> 3) & 0b1) > 0,
            ten1: ((value >> 2) & 0b1) > 0,
            boff1: ((value >> 1) & 0b1) > 0,
            en1: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x0u32) as *mut u32) };
        Cache {
            dmaudrie2: ((value >> 29) & 0b1) > 0,
            dmaen2: ((value >> 28) & 0b1) > 0,
            mamp2: ((value >> 24) & 0b1) > 0,
            wave2: ((value >> 22) & 0b1) > 0,
            tsel2: ((value >> 19) & 0b1) > 0,
            ten2: ((value >> 18) & 0b1) > 0,
            boff2: ((value >> 17) & 0b1) > 0,
            en2: ((value >> 16) & 0b1) > 0,
            dmaudrie1: ((value >> 13) & 0b1) > 0,
            dmaen1: ((value >> 12) & 0b1) > 0,
            mamp1: ((value >> 8) & 0b1) > 0,
            wave1: ((value >> 6) & 0b1) > 0,
            tsel1: ((value >> 3) & 0b1) > 0,
            ten1: ((value >> 2) & 0b1) > 0,
            boff1: ((value >> 1) & 0b1) > 0,
            en1: ((value >> 0) & 0b1) > 0,
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
                | ((self.dmaudrie2 as u32) << 29)
                | ((self.dmaen2 as u32) << 28)
                | ((self.mamp2 as u32) << 24)
                | ((self.wave2 as u32) << 22)
                | ((self.tsel2 as u32) << 19)
                | ((self.ten2 as u32) << 18)
                | ((self.boff2 as u32) << 17)
                | ((self.en2 as u32) << 16)
                | ((self.dmaudrie1 as u32) << 13)
                | ((self.dmaen1 as u32) << 12)
                | ((self.mamp1 as u32) << 8)
                | ((self.wave1 as u32) << 6)
                | ((self.tsel1 as u32) << 3)
                | ((self.ten1 as u32) << 2)
                | ((self.boff1 as u32) << 1)
                | ((self.en1 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40007400u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// software trigger register
pub mod swtrigr {
    /// Set DAC channel2 software trigger
    pub fn swtrig(index: u8, value: bool) {
        debug_assert!(index < 2, "swtrig out of range");
        let value = value as u32;
        let value = value << (0 + index * 1);
        unsafe { ::core::ptr::write_volatile((0x40007400u32 + 0x4u32) as *mut u32, value) };
    }
}
/// channel1 12-bit right-aligned data holding register
pub mod dhr12r1 {
    pub struct ReadonlyCache {
        /// DAC channel1 12-bit right-aligned data
        pub dacc1dhr: u16,
    }
    pub struct Cache {
        /// DAC channel1 12-bit right-aligned data
        pub dacc1dhr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            dacc1dhr: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x8u32) as *mut u32) };
        Cache {
            dacc1dhr: ((value >> 0) & 0b111111111111) as u16,
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
                | ((self.dacc1dhr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40007400u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// channel1 12-bit left aligned data holding register
pub mod dhr12l1 {
    pub struct ReadonlyCache {
        /// DAC channel1 12-bit left-aligned data
        pub dacc1dhr: u16,
    }
    pub struct Cache {
        /// DAC channel1 12-bit left-aligned data
        pub dacc1dhr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            dacc1dhr: ((value >> 4) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0xCu32) as *mut u32) };
        Cache {
            dacc1dhr: ((value >> 4) & 0b111111111111) as u16,
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
                | ((self.dacc1dhr as u32) << 4)
            ;
            unsafe { ::core::ptr::write_volatile((0x40007400u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// channel1 8-bit right aligned data holding register
pub mod dhr8r1 {
    pub struct ReadonlyCache {
        /// DAC channel1 8-bit right-aligned data
        pub dacc1dhr: u8,
    }
    pub struct Cache {
        /// DAC channel1 8-bit right-aligned data
        pub dacc1dhr: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            dacc1dhr: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x10u32) as *mut u32) };
        Cache {
            dacc1dhr: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.dacc1dhr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40007400u32 + 0x10u32) as *mut u32, value) };
        }
    }
}
/// channel2 12-bit right aligned data holding register
pub mod dhr12r2 {
    pub struct ReadonlyCache {
        /// DAC channel2 12-bit right-aligned data
        pub dacc2dhr: u16,
    }
    pub struct Cache {
        /// DAC channel2 12-bit right-aligned data
        pub dacc2dhr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x14u32) as *mut u32) };
        ReadonlyCache {
            dacc2dhr: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x14u32) as *mut u32) };
        Cache {
            dacc2dhr: ((value >> 0) & 0b111111111111) as u16,
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
                | ((self.dacc2dhr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40007400u32 + 0x14u32) as *mut u32, value) };
        }
    }
}
/// channel2 12-bit left aligned data holding register
pub mod dhr12l2 {
    pub struct ReadonlyCache {
        /// DAC channel2 12-bit left-aligned data
        pub dacc2dhr: u16,
    }
    pub struct Cache {
        /// DAC channel2 12-bit left-aligned data
        pub dacc2dhr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x18u32) as *mut u32) };
        ReadonlyCache {
            dacc2dhr: ((value >> 4) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x18u32) as *mut u32) };
        Cache {
            dacc2dhr: ((value >> 4) & 0b111111111111) as u16,
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
                | ((self.dacc2dhr as u32) << 4)
            ;
            unsafe { ::core::ptr::write_volatile((0x40007400u32 + 0x18u32) as *mut u32, value) };
        }
    }
}
/// channel2 8-bit right-aligned data holding register
pub mod dhr8r2 {
    pub struct ReadonlyCache {
        /// DAC channel2 8-bit right-aligned data
        pub dacc2dhr: u8,
    }
    pub struct Cache {
        /// DAC channel2 8-bit right-aligned data
        pub dacc2dhr: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x1Cu32) as *mut u32) };
        ReadonlyCache {
            dacc2dhr: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x1Cu32) as *mut u32) };
        Cache {
            dacc2dhr: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.dacc2dhr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40007400u32 + 0x1Cu32) as *mut u32, value) };
        }
    }
}
/// Dual DAC 12-bit right-aligned data holding register
pub mod dhr12rd {
    pub struct ReadonlyCache {
        /// DAC channel2 12-bit right-aligned data
        pub dacc2dhr: u16,
        /// DAC channel1 12-bit right-aligned data
        pub dacc1dhr: u16,
    }
    pub struct Cache {
        /// DAC channel2 12-bit right-aligned data
        pub dacc2dhr: u16,
        /// DAC channel1 12-bit right-aligned data
        pub dacc1dhr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x20u32) as *mut u32) };
        ReadonlyCache {
            dacc2dhr: ((value >> 16) & 0b111111111111) as u16,
            dacc1dhr: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x20u32) as *mut u32) };
        Cache {
            dacc2dhr: ((value >> 16) & 0b111111111111) as u16,
            dacc1dhr: ((value >> 0) & 0b111111111111) as u16,
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
                | ((self.dacc2dhr as u32) << 16)
                | ((self.dacc1dhr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40007400u32 + 0x20u32) as *mut u32, value) };
        }
    }
}
/// DUAL DAC 12-bit left aligned data holding register
pub mod dhr12ld {
    pub struct ReadonlyCache {
        /// DAC channel2 12-bit left-aligned data
        pub dacc2dhr: u16,
        /// DAC channel1 12-bit left-aligned data
        pub dacc1dhr: u16,
    }
    pub struct Cache {
        /// DAC channel2 12-bit left-aligned data
        pub dacc2dhr: u16,
        /// DAC channel1 12-bit left-aligned data
        pub dacc1dhr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x24u32) as *mut u32) };
        ReadonlyCache {
            dacc2dhr: ((value >> 20) & 0b111111111111) as u16,
            dacc1dhr: ((value >> 4) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x24u32) as *mut u32) };
        Cache {
            dacc2dhr: ((value >> 20) & 0b111111111111) as u16,
            dacc1dhr: ((value >> 4) & 0b111111111111) as u16,
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
                | ((self.dacc2dhr as u32) << 20)
                | ((self.dacc1dhr as u32) << 4)
            ;
            unsafe { ::core::ptr::write_volatile((0x40007400u32 + 0x24u32) as *mut u32, value) };
        }
    }
}
/// DUAL DAC 8-bit right aligned data holding register
pub mod dhr8rd {
    pub struct ReadonlyCache {
        /// DAC channel2 8-bit right-aligned data
        pub dacc2dhr: u8,
        /// DAC channel1 8-bit right-aligned data
        pub dacc1dhr: u8,
    }
    pub struct Cache {
        /// DAC channel2 8-bit right-aligned data
        pub dacc2dhr: u8,
        /// DAC channel1 8-bit right-aligned data
        pub dacc1dhr: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x28u32) as *mut u32) };
        ReadonlyCache {
            dacc2dhr: ((value >> 8) & 0b11111111) as u8,
            dacc1dhr: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x28u32) as *mut u32) };
        Cache {
            dacc2dhr: ((value >> 8) & 0b11111111) as u8,
            dacc1dhr: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.dacc2dhr as u32) << 8)
                | ((self.dacc1dhr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40007400u32 + 0x28u32) as *mut u32, value) };
        }
    }
}
/// channel1 data output register
pub mod dor1 {
    /// DAC channel1 data output
    /// Access: read-only, Width: 12, Offset: 0
    /// Get DAC channel1 data output
    pub fn dacc1dor() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x2Cu32) as *mut u32) };
        let value = value & (0b111111111111 << 0);
        value as u16
    }
}
/// channel2 data output register
pub mod dor2 {
    /// DAC channel2 data output
    /// Access: read-only, Width: 12, Offset: 0
    /// Get DAC channel2 data output
    pub fn dacc2dor() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x30u32) as *mut u32) };
        let value = value & (0b111111111111 << 0);
        value as u16
    }
}
/// status register
pub mod sr {
    pub struct ReadonlyCache {
        /// DAC channel2 DMA underrun flag
        pub dmaudr2: bool,
        /// DAC channel1 DMA underrun flag
        pub dmaudr1: bool,
    }
    pub struct Cache {
        /// DAC channel2 DMA underrun flag
        pub dmaudr2: bool,
        /// DAC channel1 DMA underrun flag
        pub dmaudr1: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x34u32) as *mut u32) };
        ReadonlyCache {
            dmaudr2: ((value >> 29) & 0b1) > 0,
            dmaudr1: ((value >> 13) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007400u32 + 0x34u32) as *mut u32) };
        Cache {
            dmaudr2: ((value >> 29) & 0b1) > 0,
            dmaudr1: ((value >> 13) & 0b1) > 0,
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
                | ((self.dmaudr2 as u32) << 29)
                | ((self.dmaudr1 as u32) << 13)
            ;
            unsafe { ::core::ptr::write_volatile((0x40007400u32 + 0x34u32) as *mut u32, value) };
        }
    }
}
/// TIM6 global and DAC12 underrun interrupts
pub const INTERRUPT_TIM6_DACUNDER: u32 = 54;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40007400</baseAddress>
  <description>Digital-to-analog converter</description>
  <groupName>DAC</groupName>
  <interrupt>
    <description>
                    TIM6 global and DAC12 underrun
                    interrupts
                </description>
    <name>TIM6_DACUNDER</name>
    <value>54</value>
  </interrupt>
  <name>DAC</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>control register</description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DAC channel2 DMA underrun interrupt
                                enable
                            </description>
          <name>DMAUDRIE2</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC channel2 DMA enable</description>
          <name>DMAEN2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                DAC channel2 mask/amplitude
                                selector
                            </description>
          <name>MAMP2</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                DAC channel2 noise/triangle wave
                                generation enable
                            </description>
          <name>WAVE2</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                DAC channel2 trigger
                                selection
                            </description>
          <name>TSEL2</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DAC channel2 trigger
                                enable
                            </description>
          <name>TEN2</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DAC channel2 output buffer
                                disable
                            </description>
          <name>BOFF2</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC channel2 enable</description>
          <name>EN2</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DAC channel1 DMA Underrun Interrupt
                                enable
                            </description>
          <name>DMAUDRIE1</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC channel1 DMA enable</description>
          <name>DMAEN1</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                DAC channel1 mask/amplitude
                                selector
                            </description>
          <name>MAMP1</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                DAC channel1 noise/triangle wave
                                generation enable
                            </description>
          <name>WAVE1</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                DAC channel1 trigger
                                selection
                            </description>
          <name>TSEL1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DAC channel1 trigger
                                enable
                            </description>
          <name>TEN1</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DAC channel1 output buffer
                                disable
                            </description>
          <name>BOFF1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC channel1 enable</description>
          <name>EN1</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x4</addressOffset>
      <description>software trigger register</description>
      <displayName>SWTRIGR</displayName>
      <fields>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DAC channel2 software
                                trigger
                            </description>
          <name>SWTRIG2</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DAC channel1 software
                                trigger
                            </description>
          <name>SWTRIG1</name>
        </field>
      </fields>
      <name>SWTRIGR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>
                        channel1 12-bit right-aligned data holding
                        register
                    </description>
      <displayName>DHR12R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>
                                DAC channel1 12-bit right-aligned
                                data
                            </description>
          <name>DACC1DHR</name>
        </field>
      </fields>
      <name>DHR12R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>
                        channel1 12-bit left aligned data holding
                        register
                    </description>
      <displayName>DHR12L1</displayName>
      <fields>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>12</bitWidth>
          <description>
                                DAC channel1 12-bit left-aligned
                                data
                            </description>
          <name>DACC1DHR</name>
        </field>
      </fields>
      <name>DHR12L1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>
                        channel1 8-bit right aligned data holding
                        register
                    </description>
      <displayName>DHR8R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>
                                DAC channel1 8-bit right-aligned
                                data
                            </description>
          <name>DACC1DHR</name>
        </field>
      </fields>
      <name>DHR8R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>
                        channel2 12-bit right aligned data holding
                        register
                    </description>
      <displayName>DHR12R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>
                                DAC channel2 12-bit right-aligned
                                data
                            </description>
          <name>DACC2DHR</name>
        </field>
      </fields>
      <name>DHR12R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>
                        channel2 12-bit left aligned data holding
                        register
                    </description>
      <displayName>DHR12L2</displayName>
      <fields>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>12</bitWidth>
          <description>
                                DAC channel2 12-bit left-aligned
                                data
                            </description>
          <name>DACC2DHR</name>
        </field>
      </fields>
      <name>DHR12L2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>
                        channel2 8-bit right-aligned data holding
                        register
                    </description>
      <displayName>DHR8R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>
                                DAC channel2 8-bit right-aligned
                                data
                            </description>
          <name>DACC2DHR</name>
        </field>
      </fields>
      <name>DHR8R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>
                        Dual DAC 12-bit right-aligned data holding
                        register
                    </description>
      <displayName>DHR12RD</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>12</bitWidth>
          <description>
                                DAC channel2 12-bit right-aligned
                                data
                            </description>
          <name>DACC2DHR</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>
                                DAC channel1 12-bit right-aligned
                                data
                            </description>
          <name>DACC1DHR</name>
        </field>
      </fields>
      <name>DHR12RD</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x24</addressOffset>
      <description>
                        DUAL DAC 12-bit left aligned data holding
                        register
                    </description>
      <displayName>DHR12LD</displayName>
      <fields>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>12</bitWidth>
          <description>
                                DAC channel2 12-bit left-aligned
                                data
                            </description>
          <name>DACC2DHR</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>12</bitWidth>
          <description>
                                DAC channel1 12-bit left-aligned
                                data
                            </description>
          <name>DACC1DHR</name>
        </field>
      </fields>
      <name>DHR12LD</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28</addressOffset>
      <description>
                        DUAL DAC 8-bit right aligned data holding
                        register
                    </description>
      <displayName>DHR8RD</displayName>
      <fields>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>
                                DAC channel2 8-bit right-aligned
                                data
                            </description>
          <name>DACC2DHR</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>
                                DAC channel1 8-bit right-aligned
                                data
                            </description>
          <name>DACC1DHR</name>
        </field>
      </fields>
      <name>DHR8RD</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x2C</addressOffset>
      <description>channel1 data output register</description>
      <displayName>DOR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>DAC channel1 data output</description>
          <name>DACC1DOR</name>
        </field>
      </fields>
      <name>DOR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x30</addressOffset>
      <description>channel2 data output register</description>
      <displayName>DOR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>DAC channel2 data output</description>
          <name>DACC2DOR</name>
        </field>
      </fields>
      <name>DOR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x34</addressOffset>
      <description>status register</description>
      <displayName>SR</displayName>
      <fields>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DAC channel2 DMA underrun
                                flag
                            </description>
          <name>DMAUDR2</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DAC channel1 DMA underrun
                                flag
                            </description>
          <name>DMAUDR1</name>
        </field>
      </fields>
      <name>SR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
