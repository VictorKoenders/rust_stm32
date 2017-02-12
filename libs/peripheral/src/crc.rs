/// Data register
pub mod dr {
    pub struct ReadonlyCache {
        /// Data register bits
        pub dr: u32,
    }
    pub struct Cache {
        /// Data register bits
        pub dr: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40023000u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            dr: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40023000u32 + 0x0u32) as *mut u32) };
        Cache {
            dr: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.dr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40023000u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// Independent data register
pub mod idr {
    pub struct ReadonlyCache {
        /// General-purpose 8-bit data register bits
        pub idr: u8,
    }
    pub struct Cache {
        /// General-purpose 8-bit data register bits
        pub idr: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40023000u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            idr: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40023000u32 + 0x4u32) as *mut u32) };
        Cache {
            idr: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.idr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40023000u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// Control register
pub mod cr {
    pub struct ReadonlyCache {
        /// reset bit
        pub reset: bool,
        /// Polynomial size
        pub polysize: bool,
        /// Reverse input data
        pub rev_in: bool,
        /// Reverse output data
        pub rev_out: bool,
    }
    pub struct Cache {
        /// reset bit
        pub reset: bool,
        /// Polynomial size
        pub polysize: bool,
        /// Reverse input data
        pub rev_in: bool,
        /// Reverse output data
        pub rev_out: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40023000u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            reset: ((value >> 0) & 0b1) > 0,
            polysize: ((value >> 3) & 0b1) > 0,
            rev_in: ((value >> 5) & 0b1) > 0,
            rev_out: ((value >> 7) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40023000u32 + 0x8u32) as *mut u32) };
        Cache {
            reset: ((value >> 0) & 0b1) > 0,
            polysize: ((value >> 3) & 0b1) > 0,
            rev_in: ((value >> 5) & 0b1) > 0,
            rev_out: ((value >> 7) & 0b1) > 0,
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
                | ((self.reset as u32) << 0)
                | ((self.polysize as u32) << 3)
                | ((self.rev_in as u32) << 5)
                | ((self.rev_out as u32) << 7)
            ;
            unsafe { ::core::ptr::write_volatile((0x40023000u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// Initial CRC value
pub mod init {
    pub struct ReadonlyCache {
        /// Programmable initial CRC value
        pub init: u32,
    }
    pub struct Cache {
        /// Programmable initial CRC value
        pub init: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40023000u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            init: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40023000u32 + 0x10u32) as *mut u32) };
        Cache {
            init: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.init as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40023000u32 + 0x10u32) as *mut u32, value) };
        }
    }
}
/// CRC polynomial
pub mod pol {
    pub struct ReadonlyCache {
        /// Programmable polynomial
        pub pol: u32,
    }
    pub struct Cache {
        /// Programmable polynomial
        pub pol: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40023000u32 + 0x14u32) as *mut u32) };
        ReadonlyCache {
            pol: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40023000u32 + 0x14u32) as *mut u32) };
        Cache {
            pol: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.pol as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40023000u32 + 0x14u32) as *mut u32, value) };
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
  <baseAddress>0x40023000</baseAddress>
  <description>
                cyclic redundancy check calculation
                unit
            </description>
  <groupName>CRC</groupName>
  <name>CRC</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>Data register</description>
      <displayName>DR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Data register bits</description>
          <name>DR</name>
        </field>
      </fields>
      <name>DR</name>
      <resetValue>0xFFFFFFFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>Independent data register</description>
      <displayName>IDR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>
                                General-purpose 8-bit data register
                                bits
                            </description>
          <name>IDR</name>
        </field>
      </fields>
      <name>IDR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>Control register</description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>reset bit</description>
          <name>RESET</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Polynomial size</description>
          <name>POLYSIZE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Reverse input data</description>
          <name>REV_IN</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reverse output data</description>
          <name>REV_OUT</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>Initial CRC value</description>
      <displayName>INIT</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>
                                Programmable initial CRC
                                value
                            </description>
          <name>INIT</name>
        </field>
      </fields>
      <name>INIT</name>
      <resetValue>0xFFFFFFFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>CRC polynomial</description>
      <displayName>POL</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Programmable polynomial</description>
          <name>POL</name>
        </field>
      </fields>
      <name>POL</name>
      <resetValue>0x04C11DB7</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
