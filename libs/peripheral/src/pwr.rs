/// power control register
pub mod cr {
    pub struct ReadonlyCache {
        /// Low-power deep sleep
        pub lpds: bool,
        /// Power down deepsleep
        pub pdds: bool,
        /// Clear wakeup flag
        pub cwuf: bool,
        /// Clear standby flag
        pub csbf: bool,
        /// Power voltage detector enable
        pub pvde: bool,
        /// PVD level selection
        pub pls: bool,
        /// Disable backup domain write protection
        pub dbp: bool,
    }
    pub struct Cache {
        /// Low-power deep sleep
        pub lpds: bool,
        /// Power down deepsleep
        pub pdds: bool,
        /// Clear wakeup flag
        pub cwuf: bool,
        /// Clear standby flag
        pub csbf: bool,
        /// Power voltage detector enable
        pub pvde: bool,
        /// PVD level selection
        pub pls: bool,
        /// Disable backup domain write protection
        pub dbp: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007000u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            lpds: ((value >> 0) & 0b1) > 0,
            pdds: ((value >> 1) & 0b1) > 0,
            cwuf: ((value >> 2) & 0b1) > 0,
            csbf: ((value >> 3) & 0b1) > 0,
            pvde: ((value >> 4) & 0b1) > 0,
            pls: ((value >> 5) & 0b1) > 0,
            dbp: ((value >> 8) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40007000u32 + 0x0u32) as *mut u32) };
        Cache {
            lpds: ((value >> 0) & 0b1) > 0,
            pdds: ((value >> 1) & 0b1) > 0,
            cwuf: ((value >> 2) & 0b1) > 0,
            csbf: ((value >> 3) & 0b1) > 0,
            pvde: ((value >> 4) & 0b1) > 0,
            pls: ((value >> 5) & 0b1) > 0,
            dbp: ((value >> 8) & 0b1) > 0,
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
                | ((self.lpds as u32) << 0)
                | ((self.pdds as u32) << 1)
                | ((self.cwuf as u32) << 2)
                | ((self.csbf as u32) << 3)
                | ((self.pvde as u32) << 4)
                | ((self.pls as u32) << 5)
                | ((self.dbp as u32) << 8)
            ;
            unsafe { ::core::ptr::write_volatile((0x40007000u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// power control/status register
pub mod csr {
    /// Wakeup flag
    /// Access: read-only, Width: 1, Offset: 0
    /// Get Wakeup flag
    pub fn wuf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40007000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Standby flag
    /// Access: read-only, Width: 1, Offset: 1
    /// Get Standby flag
    pub fn sbf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40007000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// PVD output
    /// Access: read-only, Width: 1, Offset: 2
    /// Get PVD output
    pub fn pvdo() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40007000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// Enable WKUP1 pin
    /// Access: read-write, Width: 1, Offset: 8
    /// Set Enable WKUP1 pin
    pub fn set_ewup1(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40007000u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Enable WKUP1 pin
    pub fn get_ewup1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40007000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Enable WKUP2 pin
    /// Access: read-write, Width: 1, Offset: 9
    /// Set Enable WKUP2 pin
    pub fn set_ewup2(value: bool) {
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40007000u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Enable WKUP2 pin
    pub fn get_ewup2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40007000u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
}
/// PVD through EXTI line detection interrupt
pub const INTERRUPT_PVD: u32 = 1;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40007000</baseAddress>
  <description>Power control</description>
  <groupName>PWR</groupName>
  <interrupt>
    <description>
                    PVD through EXTI line detection
                    interrupt
                </description>
    <name>PVD</name>
    <value>1</value>
  </interrupt>
  <name>PWR</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>power control register</description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Low-power deep sleep</description>
          <name>LPDS</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Power down deepsleep</description>
          <name>PDDS</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clear wakeup flag</description>
          <name>CWUF</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clear standby flag</description>
          <name>CSBF</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Power voltage detector
                                enable
                            </description>
          <name>PVDE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>3</bitWidth>
          <description>PVD level selection</description>
          <name>PLS</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Disable backup domain write
                                protection
                            </description>
          <name>DBP</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x4</addressOffset>
      <description>power control/status register</description>
      <displayName>CSR</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Wakeup flag</description>
          <name>WUF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Standby flag</description>
          <name>SBF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PVD output</description>
          <name>PVDO</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Enable WKUP1 pin</description>
          <name>EWUP1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Enable WKUP2 pin</description>
          <name>EWUP2</name>
        </field>
      </fields>
      <name>CSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
