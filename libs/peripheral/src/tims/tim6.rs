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
        /// UIF status bit remapping
        pub uifremap: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40001000u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            cen: ((value >> 0) & 0b1) > 0,
            udis: ((value >> 1) & 0b1) > 0,
            urs: ((value >> 2) & 0b1) > 0,
            opm: ((value >> 3) & 0b1) > 0,
            arpe: ((value >> 7) & 0b1) > 0,
            uifremap: ((value >> 11) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40001000u32 + 0x0u32) as *mut u32) };
        Cache {
            cen: ((value >> 0) & 0b1) > 0,
            udis: ((value >> 1) & 0b1) > 0,
            urs: ((value >> 2) & 0b1) > 0,
            opm: ((value >> 3) & 0b1) > 0,
            arpe: ((value >> 7) & 0b1) > 0,
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
                | ((self.uifremap as u32) << 11)
            ;
            unsafe { ::core::ptr::write_volatile((0x40001000u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// control register 2
pub mod cr2 {
    pub struct ReadonlyCache {
        /// Master mode selection
        pub mms: u8,
    }
    pub struct Cache {
        /// Master mode selection
        pub mms: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40001000u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            mms: ((value >> 4) & 0b111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40001000u32 + 0x4u32) as *mut u32) };
        Cache {
            mms: ((value >> 4) & 0b111) as u8,
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
                | ((self.mms as u32) << 4)
            ;
            unsafe { ::core::ptr::write_volatile((0x40001000u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// DMA/Interrupt enable register
pub mod dier {
    pub struct ReadonlyCache {
        /// Update DMA request enable
        pub ude: bool,
        /// Update interrupt enable
        pub uie: bool,
    }
    pub struct Cache {
        /// Update DMA request enable
        pub ude: bool,
        /// Update interrupt enable
        pub uie: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40001000u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            ude: ((value >> 8) & 0b1) > 0,
            uie: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40001000u32 + 0xCu32) as *mut u32) };
        Cache {
            ude: ((value >> 8) & 0b1) > 0,
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
                | ((self.ude as u32) << 8)
                | ((self.uie as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40001000u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// status register
pub mod sr {
    pub struct ReadonlyCache {
        /// Update interrupt flag
        pub uif: bool,
    }
    pub struct Cache {
        /// Update interrupt flag
        pub uif: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40001000u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            uif: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40001000u32 + 0x10u32) as *mut u32) };
        Cache {
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
                | ((self.uif as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40001000u32 + 0x10u32) as *mut u32, value) };
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
        unsafe { ::core::ptr::write_volatile((0x40001000u32 + 0x14u32) as *mut u32, value) };
    }
}
/// counter
pub mod cnt {
    /// Low counter value
    /// Access: read-write, Width: 16, Offset: 0
    /// Set Low counter value
    pub fn set_cnt(value: u16) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40001000u32 + 0x24u32) as *mut u32, value) };
    }
    /// Get Low counter value
    pub fn get_cnt() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40001000u32 + 0x24u32) as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
    /// UIF Copy
    /// Access: read-only, Width: 1, Offset: 31
    /// Get UIF Copy
    pub fn uifcpy() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40001000u32 + 0x24u32) as *mut u32) };
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
        let value = unsafe { ::core::ptr::read_volatile((0x40001000u32 + 0x28u32) as *mut u32) };
        ReadonlyCache {
            psc: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40001000u32 + 0x28u32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40001000u32 + 0x28u32) as *mut u32, value) };
        }
    }
}
/// auto-reload register
pub mod arr {
    pub struct ReadonlyCache {
        /// Low Auto-reload value
        pub arr: u16,
    }
    pub struct Cache {
        /// Low Auto-reload value
        pub arr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40001000u32 + 0x2Cu32) as *mut u32) };
        ReadonlyCache {
            arr: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40001000u32 + 0x2Cu32) as *mut u32) };
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
            unsafe { ::core::ptr::write_volatile((0x40001000u32 + 0x2Cu32) as *mut u32, value) };
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
  <baseAddress>0x40001000</baseAddress>
  <description>Basic timers</description>
  <groupName>TIMs</groupName>
  <interrupt>
    <description>
                    TIM6 global and DAC12 underrun
                    interrupts
                </description>
    <name>TIM6_DACUNDER</name>
    <value>54</value>
  </interrupt>
  <name>TIM6</name>
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
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Master mode selection</description>
          <name>MMS</name>
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
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update DMA request enable</description>
          <name>UDE</name>
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
      </fields>
      <name>EGR</name>
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
          <description>Low counter value</description>
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
          <description>Low Auto-reload value</description>
          <name>ARR</name>
        </field>
      </fields>
      <name>ARR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
