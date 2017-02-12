/// Control register
pub mod cr {
    pub struct ReadonlyCache {
        /// 7-bit counter
        pub t: u8,
        /// Activation bit
        pub wdga: u8,
    }
    pub struct Cache {
        /// 7-bit counter
        pub t: u8,
        /// Activation bit
        pub wdga: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002C00u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            t: ((value >> 0) & 0b1111111) as u8,
            wdga: ((value >> 7) & 0b1111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002C00u32 + 0x0u32) as *mut u32) };
        Cache {
            t: ((value >> 0) & 0b1111111) as u8,
            wdga: ((value >> 7) & 0b1111111) as u8,
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
                | ((self.t as u32) << 0)
                | ((self.wdga as u32) << 7)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002C00u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// Configuration register
pub mod cfr {
    pub struct ReadonlyCache {
        /// Early wakeup interrupt
        pub ewi: bool,
        /// Timer base
        pub wdgtb: bool,
        /// 7-bit window value
        pub w: bool,
    }
    pub struct Cache {
        /// Early wakeup interrupt
        pub ewi: bool,
        /// Timer base
        pub wdgtb: bool,
        /// 7-bit window value
        pub w: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002C00u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            ewi: ((value >> 9) & 0b1) > 0,
            wdgtb: ((value >> 7) & 0b1) > 0,
            w: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002C00u32 + 0x4u32) as *mut u32) };
        Cache {
            ewi: ((value >> 9) & 0b1) > 0,
            wdgtb: ((value >> 7) & 0b1) > 0,
            w: ((value >> 0) & 0b1) > 0,
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
                | ((self.ewi as u32) << 9)
                | ((self.wdgtb as u32) << 7)
                | ((self.w as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002C00u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// Status register
pub mod sr {
    pub struct ReadonlyCache {
        /// Early wakeup interrupt flag
        pub ewif: bool,
    }
    pub struct Cache {
        /// Early wakeup interrupt flag
        pub ewif: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002C00u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            ewif: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40002C00u32 + 0x8u32) as *mut u32) };
        Cache {
            ewif: ((value >> 0) & 0b1) > 0,
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
                | ((self.ewif as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40002C00u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// Window Watchdog interrupt
pub const INTERRUPT_WWDG: u32 = 0;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40002C00</baseAddress>
  <description>Window watchdog</description>
  <groupName>WWDG</groupName>
  <interrupt>
    <description>Window Watchdog interrupt</description>
    <name>WWDG</name>
    <value>0</value>
  </interrupt>
  <name>WWDG</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>Control register</description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>7</bitWidth>
          <description>7-bit counter</description>
          <name>T</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Activation bit</description>
          <name>WDGA</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x0000007F</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>Configuration register</description>
      <displayName>CFR</displayName>
      <fields>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Early wakeup interrupt</description>
          <name>EWI</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Timer base</description>
          <name>WDGTB</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>7</bitWidth>
          <description>7-bit window value</description>
          <name>W</name>
        </field>
      </fields>
      <name>CFR</name>
      <resetValue>0x0000007F</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>Status register</description>
      <displayName>SR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Early wakeup interrupt
                                flag
                            </description>
          <name>EWIF</name>
        </field>
      </fields>
      <name>SR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
