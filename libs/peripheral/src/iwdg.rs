/// Key register
pub mod kr {
    /// Key value
    /// Access: write-only, Width: 16, Offset: 0
    /// Set Key value
    pub fn key(value: u16) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40003000u32 + 0x0u32) as *mut u32, value) };
    }
}
/// Prescaler register
pub mod pr {
    pub struct ReadonlyCache {
        /// Prescaler divider
        pub pr: u8,
    }
    pub struct Cache {
        /// Prescaler divider
        pub pr: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40003000u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            pr: ((value >> 0) & 0b111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40003000u32 + 0x4u32) as *mut u32) };
        Cache {
            pr: ((value >> 0) & 0b111) as u8,
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
                | ((self.pr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40003000u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// Reload register
pub mod rlr {
    pub struct ReadonlyCache {
        /// Watchdog counter reload value
        pub rl: u16,
    }
    pub struct Cache {
        /// Watchdog counter reload value
        pub rl: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40003000u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            rl: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40003000u32 + 0x8u32) as *mut u32) };
        Cache {
            rl: ((value >> 0) & 0b111111111111) as u16,
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
                | ((self.rl as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40003000u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// Status register
pub mod sr {
    /// Watchdog prescaler value update
    /// Access: read-only, Width: 1, Offset: 0
    /// Get Watchdog prescaler value update
    pub fn pvu() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40003000u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Watchdog counter reload value update
    /// Access: read-only, Width: 1, Offset: 1
    /// Get Watchdog counter reload value update
    pub fn rvu() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40003000u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// Watchdog counter window value update
    /// Access: read-only, Width: 1, Offset: 2
    /// Get Watchdog counter window value update
    pub fn wvu() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40003000u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
}
/// Window register
pub mod winr {
    pub struct ReadonlyCache {
        /// Watchdog counter window value
        pub win: u16,
    }
    pub struct Cache {
        /// Watchdog counter window value
        pub win: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40003000u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            win: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40003000u32 + 0x10u32) as *mut u32) };
        Cache {
            win: ((value >> 0) & 0b111111111111) as u16,
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
                | ((self.win as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40003000u32 + 0x10u32) as *mut u32, value) };
        }
    }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40003000</baseAddress>
  <description>Independent watchdog</description>
  <groupName>IWDG</groupName>
  <name>IWDG</name>
  <registers>
    <register>
      <access>write-only</access>
      <addressOffset>0x0</addressOffset>
      <description>Key register</description>
      <displayName>KR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Key value</description>
          <name>KEY</name>
        </field>
      </fields>
      <name>KR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>Prescaler register</description>
      <displayName>PR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Prescaler divider</description>
          <name>PR</name>
        </field>
      </fields>
      <name>PR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>Reload register</description>
      <displayName>RLR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>
                                Watchdog counter reload
                                value
                            </description>
          <name>RL</name>
        </field>
      </fields>
      <name>RLR</name>
      <resetValue>0x00000FFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0xC</addressOffset>
      <description>Status register</description>
      <displayName>SR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Watchdog prescaler value
                                update
                            </description>
          <name>PVU</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Watchdog counter reload value
                                update
                            </description>
          <name>RVU</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Watchdog counter window value
                                update
                            </description>
          <name>WVU</name>
        </field>
      </fields>
      <name>SR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>Window register</description>
      <displayName>WINR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>
                                Watchdog counter window
                                value
                            </description>
          <name>WIN</name>
        </field>
      </fields>
      <name>WINR</name>
      <resetValue>0x00000FFF</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
