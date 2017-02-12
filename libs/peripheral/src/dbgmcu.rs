/// MCU Device ID Code Register
pub mod idcode {
    /// Device Identifier
    /// Access: read-only, Width: 12, Offset: 0
    /// Get Device Identifier
    pub fn dev_id() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0xE0042000u32 + 0x0u32) as *mut u32) };
        let value = value & (0b111111111111 << 0);
        value as u16
    }
    /// Revision Identifier
    /// Access: read-only, Width: 16, Offset: 16
    /// Get Revision Identifier
    pub fn rev_id() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0xE0042000u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1111111111111111 << 16);
        value as u16
    }
}
/// Debug MCU Configuration Register
pub mod cr {
    pub struct ReadonlyCache {
        /// Debug Sleep mode
        pub dbg_sleep: bool,
        /// Debug Stop Mode
        pub dbg_stop: bool,
        /// Debug Standby Mode
        pub dbg_standby: bool,
        /// Trace pin assignment control
        pub trace_ioen: bool,
        /// Trace pin assignment control
        pub trace_mode: bool,
    }
    pub struct Cache {
        /// Debug Sleep mode
        pub dbg_sleep: bool,
        /// Debug Stop Mode
        pub dbg_stop: bool,
        /// Debug Standby Mode
        pub dbg_standby: bool,
        /// Trace pin assignment control
        pub trace_ioen: bool,
        /// Trace pin assignment control
        pub trace_mode: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE0042000u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            dbg_sleep: ((value >> 0) & 0b1) > 0,
            dbg_stop: ((value >> 1) & 0b1) > 0,
            dbg_standby: ((value >> 2) & 0b1) > 0,
            trace_ioen: ((value >> 5) & 0b1) > 0,
            trace_mode: ((value >> 6) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE0042000u32 + 0x4u32) as *mut u32) };
        Cache {
            dbg_sleep: ((value >> 0) & 0b1) > 0,
            dbg_stop: ((value >> 1) & 0b1) > 0,
            dbg_standby: ((value >> 2) & 0b1) > 0,
            trace_ioen: ((value >> 5) & 0b1) > 0,
            trace_mode: ((value >> 6) & 0b1) > 0,
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
                | ((self.dbg_sleep as u32) << 0)
                | ((self.dbg_stop as u32) << 1)
                | ((self.dbg_standby as u32) << 2)
                | ((self.trace_ioen as u32) << 5)
                | ((self.trace_mode as u32) << 6)
            ;
            unsafe { ::core::ptr::write_volatile((0xE0042000u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// APB Low Freeze Register
pub mod apb1fz {
    pub struct ReadonlyCache {
        /// Debug Timer 2 stopped when Core is halted
        pub dbg_tim2_stop: bool,
        /// Debug Timer 3 stopped when Core is halted
        pub dbg_tim3_stop: bool,
        /// Debug Timer 4 stopped when Core is halted
        pub dbg_tim4_stop: bool,
        /// Debug Timer 5 stopped when Core is halted
        pub dbg_tim5_stop: bool,
        /// Debug Timer 6 stopped when Core is halted
        pub dbg_tim6_stop: bool,
        /// Debug Timer 7 stopped when Core is halted
        pub dbg_tim7_stop: bool,
        /// Debug Timer 12 stopped when Core is halted
        pub dbg_tim12_stop: bool,
        /// Debug Timer 13 stopped when Core is halted
        pub dbg_tim13_stop: bool,
        /// Debug Timer 14 stopped when Core is halted
        pub dbg_timer14_stop: bool,
        /// Debug Timer 18 stopped when Core is halted
        pub dbg_tim18_stop: bool,
        /// Debug RTC stopped when Core is halted
        pub dbg_rtc_stop: bool,
        /// Debug Window Wachdog stopped when Core is halted
        pub dbg_wwdg_stop: bool,
        /// Debug Independent Wachdog stopped when Core is halted
        pub dbg_iwdg_stop: bool,
        /// SMBUS timeout mode stopped when Core is halted
        pub i2c1_smbus_timeout: bool,
        /// SMBUS timeout mode stopped when Core is halted
        pub i2c2_smbus_timeout: bool,
        /// Debug CAN stopped when core is halted
        pub dbg_can_stop: bool,
    }
    pub struct Cache {
        /// Debug Timer 2 stopped when Core is halted
        pub dbg_tim2_stop: bool,
        /// Debug Timer 3 stopped when Core is halted
        pub dbg_tim3_stop: bool,
        /// Debug Timer 4 stopped when Core is halted
        pub dbg_tim4_stop: bool,
        /// Debug Timer 5 stopped when Core is halted
        pub dbg_tim5_stop: bool,
        /// Debug Timer 6 stopped when Core is halted
        pub dbg_tim6_stop: bool,
        /// Debug Timer 7 stopped when Core is halted
        pub dbg_tim7_stop: bool,
        /// Debug Timer 12 stopped when Core is halted
        pub dbg_tim12_stop: bool,
        /// Debug Timer 13 stopped when Core is halted
        pub dbg_tim13_stop: bool,
        /// Debug Timer 14 stopped when Core is halted
        pub dbg_timer14_stop: bool,
        /// Debug Timer 18 stopped when Core is halted
        pub dbg_tim18_stop: bool,
        /// Debug RTC stopped when Core is halted
        pub dbg_rtc_stop: bool,
        /// Debug Window Wachdog stopped when Core is halted
        pub dbg_wwdg_stop: bool,
        /// Debug Independent Wachdog stopped when Core is halted
        pub dbg_iwdg_stop: bool,
        /// SMBUS timeout mode stopped when Core is halted
        pub i2c1_smbus_timeout: bool,
        /// SMBUS timeout mode stopped when Core is halted
        pub i2c2_smbus_timeout: bool,
        /// Debug CAN stopped when core is halted
        pub dbg_can_stop: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE0042000u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            dbg_tim2_stop: ((value >> 0) & 0b1) > 0,
            dbg_tim3_stop: ((value >> 1) & 0b1) > 0,
            dbg_tim4_stop: ((value >> 2) & 0b1) > 0,
            dbg_tim5_stop: ((value >> 3) & 0b1) > 0,
            dbg_tim6_stop: ((value >> 4) & 0b1) > 0,
            dbg_tim7_stop: ((value >> 5) & 0b1) > 0,
            dbg_tim12_stop: ((value >> 6) & 0b1) > 0,
            dbg_tim13_stop: ((value >> 7) & 0b1) > 0,
            dbg_timer14_stop: ((value >> 8) & 0b1) > 0,
            dbg_tim18_stop: ((value >> 9) & 0b1) > 0,
            dbg_rtc_stop: ((value >> 10) & 0b1) > 0,
            dbg_wwdg_stop: ((value >> 11) & 0b1) > 0,
            dbg_iwdg_stop: ((value >> 12) & 0b1) > 0,
            i2c1_smbus_timeout: ((value >> 21) & 0b1) > 0,
            i2c2_smbus_timeout: ((value >> 22) & 0b1) > 0,
            dbg_can_stop: ((value >> 25) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE0042000u32 + 0x8u32) as *mut u32) };
        Cache {
            dbg_tim2_stop: ((value >> 0) & 0b1) > 0,
            dbg_tim3_stop: ((value >> 1) & 0b1) > 0,
            dbg_tim4_stop: ((value >> 2) & 0b1) > 0,
            dbg_tim5_stop: ((value >> 3) & 0b1) > 0,
            dbg_tim6_stop: ((value >> 4) & 0b1) > 0,
            dbg_tim7_stop: ((value >> 5) & 0b1) > 0,
            dbg_tim12_stop: ((value >> 6) & 0b1) > 0,
            dbg_tim13_stop: ((value >> 7) & 0b1) > 0,
            dbg_timer14_stop: ((value >> 8) & 0b1) > 0,
            dbg_tim18_stop: ((value >> 9) & 0b1) > 0,
            dbg_rtc_stop: ((value >> 10) & 0b1) > 0,
            dbg_wwdg_stop: ((value >> 11) & 0b1) > 0,
            dbg_iwdg_stop: ((value >> 12) & 0b1) > 0,
            i2c1_smbus_timeout: ((value >> 21) & 0b1) > 0,
            i2c2_smbus_timeout: ((value >> 22) & 0b1) > 0,
            dbg_can_stop: ((value >> 25) & 0b1) > 0,
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
                | ((self.dbg_tim2_stop as u32) << 0)
                | ((self.dbg_tim3_stop as u32) << 1)
                | ((self.dbg_tim4_stop as u32) << 2)
                | ((self.dbg_tim5_stop as u32) << 3)
                | ((self.dbg_tim6_stop as u32) << 4)
                | ((self.dbg_tim7_stop as u32) << 5)
                | ((self.dbg_tim12_stop as u32) << 6)
                | ((self.dbg_tim13_stop as u32) << 7)
                | ((self.dbg_timer14_stop as u32) << 8)
                | ((self.dbg_tim18_stop as u32) << 9)
                | ((self.dbg_rtc_stop as u32) << 10)
                | ((self.dbg_wwdg_stop as u32) << 11)
                | ((self.dbg_iwdg_stop as u32) << 12)
                | ((self.i2c1_smbus_timeout as u32) << 21)
                | ((self.i2c2_smbus_timeout as u32) << 22)
                | ((self.dbg_can_stop as u32) << 25)
            ;
            unsafe { ::core::ptr::write_volatile((0xE0042000u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// APB High Freeze Register
pub mod apb2fz {
    pub struct ReadonlyCache {
        /// Debug Timer 15 stopped when Core is halted
        pub dbg_tim15_stop: bool,
        /// Debug Timer 16 stopped when Core is halted
        pub dbg_tim16_stop: bool,
        /// Debug Timer 17 stopped when Core is halted
        pub dbg_tim17_sto: bool,
        /// Debug Timer 19 stopped when Core is halted
        pub dbg_tim19_stop: bool,
    }
    pub struct Cache {
        /// Debug Timer 15 stopped when Core is halted
        pub dbg_tim15_stop: bool,
        /// Debug Timer 16 stopped when Core is halted
        pub dbg_tim16_stop: bool,
        /// Debug Timer 17 stopped when Core is halted
        pub dbg_tim17_sto: bool,
        /// Debug Timer 19 stopped when Core is halted
        pub dbg_tim19_stop: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE0042000u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            dbg_tim15_stop: ((value >> 2) & 0b1) > 0,
            dbg_tim16_stop: ((value >> 3) & 0b1) > 0,
            dbg_tim17_sto: ((value >> 4) & 0b1) > 0,
            dbg_tim19_stop: ((value >> 5) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE0042000u32 + 0xCu32) as *mut u32) };
        Cache {
            dbg_tim15_stop: ((value >> 2) & 0b1) > 0,
            dbg_tim16_stop: ((value >> 3) & 0b1) > 0,
            dbg_tim17_sto: ((value >> 4) & 0b1) > 0,
            dbg_tim19_stop: ((value >> 5) & 0b1) > 0,
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
                | ((self.dbg_tim15_stop as u32) << 2)
                | ((self.dbg_tim16_stop as u32) << 3)
                | ((self.dbg_tim17_sto as u32) << 4)
                | ((self.dbg_tim19_stop as u32) << 5)
            ;
            unsafe { ::core::ptr::write_volatile((0xE0042000u32 + 0xCu32) as *mut u32, value) };
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
  <baseAddress>0xE0042000</baseAddress>
  <description>Debug support</description>
  <groupName>DBGMCU</groupName>
  <name>DBGMCU</name>
  <registers>
    <register>
      <access>read-only</access>
      <addressOffset>0x0</addressOffset>
      <description>MCU Device ID Code Register</description>
      <displayName>IDCODE</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>Device Identifier</description>
          <name>DEV_ID</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Revision Identifier</description>
          <name>REV_ID</name>
        </field>
      </fields>
      <name>IDCODE</name>
      <resetValue>0x0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>
                        Debug MCU Configuration
                        Register
                    </description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Debug Sleep mode</description>
          <name>DBG_SLEEP</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Debug Stop Mode</description>
          <name>DBG_STOP</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Debug Standby Mode</description>
          <name>DBG_STANDBY</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Trace pin assignment
                                control
                            </description>
          <name>TRACE_IOEN</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Trace pin assignment
                                control
                            </description>
          <name>TRACE_MODE</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>APB Low Freeze Register</description>
      <displayName>APB1FZ</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Timer 2 stopped when Core is
                                halted
                            </description>
          <name>DBG_TIM2_STOP</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Timer 3 stopped when Core is
                                halted
                            </description>
          <name>DBG_TIM3_STOP</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Timer 4 stopped when Core is
                                halted
                            </description>
          <name>DBG_TIM4_STOP</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Timer 5 stopped when Core is
                                halted
                            </description>
          <name>DBG_TIM5_STOP</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Timer 6 stopped when Core is
                                halted
                            </description>
          <name>DBG_TIM6_STOP</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Timer 7 stopped when Core is
                                halted
                            </description>
          <name>DBG_TIM7_STOP</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Timer 12 stopped when Core is
                                halted
                            </description>
          <name>DBG_TIM12_STOP</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Timer 13 stopped when Core is
                                halted
                            </description>
          <name>DBG_TIM13_STOP</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Timer 14 stopped when Core is
                                halted
                            </description>
          <name>DBG_TIMER14_STOP</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Timer 18 stopped when Core is
                                halted
                            </description>
          <name>DBG_TIM18_STOP</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug RTC stopped when Core is
                                halted
                            </description>
          <name>DBG_RTC_STOP</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Window Wachdog stopped when Core
                                is halted
                            </description>
          <name>DBG_WWDG_STOP</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Independent Wachdog stopped when
                                Core is halted
                            </description>
          <name>DBG_IWDG_STOP</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                SMBUS timeout mode stopped when Core is
                                halted
                            </description>
          <name>I2C1_SMBUS_TIMEOUT</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                SMBUS timeout mode stopped when Core is
                                halted
                            </description>
          <name>I2C2_SMBUS_TIMEOUT</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug CAN stopped when core is
                                halted
                            </description>
          <name>DBG_CAN_STOP</name>
        </field>
      </fields>
      <name>APB1FZ</name>
      <resetValue>0x0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>APB High Freeze Register</description>
      <displayName>APB2FZ</displayName>
      <fields>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Timer 15 stopped when Core is
                                halted
                            </description>
          <name>DBG_TIM15_STOP</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Timer 16 stopped when Core is
                                halted
                            </description>
          <name>DBG_TIM16_STOP</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Timer 17 stopped when Core is
                                halted
                            </description>
          <name>DBG_TIM17_STO</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Debug Timer 19 stopped when Core is
                                halted
                            </description>
          <name>DBG_TIM19_STOP</name>
        </field>
      </fields>
      <name>APB2FZ</name>
      <resetValue>0x0</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
