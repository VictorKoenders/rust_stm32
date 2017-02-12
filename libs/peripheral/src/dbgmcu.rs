/// MOD DBGMCU
/// Debug support
const BASE_ADDRESS: u32 = 0xE0042000;
/// MCU Device ID Code Register
/// Size: 0x20 bits
pub mod idcode {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DEV_ID_BIT_OFFSET: u8 = 0;
	const DEV_ID_BIT_WIDTH: u8 = 12;
	/// Device Identifier (Width: 12, Offset: 0)
	pub fn get_dev_id() -> u16 { ::read(REGISTER_ADDRESS, DEV_ID_BIT_OFFSET, DEV_ID_BIT_WIDTH) as u16 }

	const REV_ID_BIT_OFFSET: u8 = 16;
	const REV_ID_BIT_WIDTH: u8 = 16;
	/// Revision Identifier (Width: 16, Offset: 16)
	pub fn get_rev_id() -> u16 { ::read(REGISTER_ADDRESS, REV_ID_BIT_OFFSET, REV_ID_BIT_WIDTH) as u16 }
}
/// Debug MCU Configuration Register
/// Size: 0x20 bits
pub mod cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DBG_SLEEP_BIT_OFFSET: u8 = 0;
	const DBG_SLEEP_BIT_WIDTH: u8 = 1;
	/// Debug Sleep mode (Width: 1, Offset: 0)
	pub fn get_dbg_sleep() -> u8 { ::read(REGISTER_ADDRESS, DBG_SLEEP_BIT_OFFSET, DBG_SLEEP_BIT_WIDTH) as u8 }
	/// Debug Sleep mode (Width: 1, Offset: 0)
	pub fn set_dbg_sleep(value: u8) { ::write(REGISTER_ADDRESS, DBG_SLEEP_BIT_OFFSET, DBG_SLEEP_BIT_WIDTH, value as u32); }

	const DBG_STOP_BIT_OFFSET: u8 = 1;
	const DBG_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Stop Mode (Width: 1, Offset: 1)
	pub fn get_dbg_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_STOP_BIT_OFFSET, DBG_STOP_BIT_WIDTH) as u8 }
	/// Debug Stop Mode (Width: 1, Offset: 1)
	pub fn set_dbg_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_STOP_BIT_OFFSET, DBG_STOP_BIT_WIDTH, value as u32); }

	const DBG_STANDBY_BIT_OFFSET: u8 = 2;
	const DBG_STANDBY_BIT_WIDTH: u8 = 1;
	/// Debug Standby Mode (Width: 1, Offset: 2)
	pub fn get_dbg_standby() -> u8 { ::read(REGISTER_ADDRESS, DBG_STANDBY_BIT_OFFSET, DBG_STANDBY_BIT_WIDTH) as u8 }
	/// Debug Standby Mode (Width: 1, Offset: 2)
	pub fn set_dbg_standby(value: u8) { ::write(REGISTER_ADDRESS, DBG_STANDBY_BIT_OFFSET, DBG_STANDBY_BIT_WIDTH, value as u32); }

	const TRACE_IOEN_BIT_OFFSET: u8 = 5;
	const TRACE_IOEN_BIT_WIDTH: u8 = 1;
	/// Trace pin assignment control (Width: 1, Offset: 5)
	pub fn get_trace_ioen() -> u8 { ::read(REGISTER_ADDRESS, TRACE_IOEN_BIT_OFFSET, TRACE_IOEN_BIT_WIDTH) as u8 }
	/// Trace pin assignment control (Width: 1, Offset: 5)
	pub fn set_trace_ioen(value: u8) { ::write(REGISTER_ADDRESS, TRACE_IOEN_BIT_OFFSET, TRACE_IOEN_BIT_WIDTH, value as u32); }

	const TRACE_MODE_BIT_OFFSET: u8 = 6;
	const TRACE_MODE_BIT_WIDTH: u8 = 2;
	/// Trace pin assignment control (Width: 2, Offset: 6)
	pub fn get_trace_mode() -> u8 { ::read(REGISTER_ADDRESS, TRACE_MODE_BIT_OFFSET, TRACE_MODE_BIT_WIDTH) as u8 }
	/// Trace pin assignment control (Width: 2, Offset: 6)
	pub fn set_trace_mode(value: u8) { ::write(REGISTER_ADDRESS, TRACE_MODE_BIT_OFFSET, TRACE_MODE_BIT_WIDTH, value as u32); }
}
/// APB Low Freeze Register
/// Size: 0x20 bits
pub mod apb1fz {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DBG_TIM2_STOP_BIT_OFFSET: u8 = 0;
	const DBG_TIM2_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Timer 2 stopped when Core is halted (Width: 1, Offset: 0)
	pub fn get_dbg_tim2_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_TIM2_STOP_BIT_OFFSET, DBG_TIM2_STOP_BIT_WIDTH) as u8 }
	/// Debug Timer 2 stopped when Core is halted (Width: 1, Offset: 0)
	pub fn set_dbg_tim2_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_TIM2_STOP_BIT_OFFSET, DBG_TIM2_STOP_BIT_WIDTH, value as u32); }

	const DBG_TIM3_STOP_BIT_OFFSET: u8 = 1;
	const DBG_TIM3_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Timer 3 stopped when Core is halted (Width: 1, Offset: 1)
	pub fn get_dbg_tim3_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_TIM3_STOP_BIT_OFFSET, DBG_TIM3_STOP_BIT_WIDTH) as u8 }
	/// Debug Timer 3 stopped when Core is halted (Width: 1, Offset: 1)
	pub fn set_dbg_tim3_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_TIM3_STOP_BIT_OFFSET, DBG_TIM3_STOP_BIT_WIDTH, value as u32); }

	const DBG_TIM4_STOP_BIT_OFFSET: u8 = 2;
	const DBG_TIM4_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Timer 4 stopped when Core is halted (Width: 1, Offset: 2)
	pub fn get_dbg_tim4_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_TIM4_STOP_BIT_OFFSET, DBG_TIM4_STOP_BIT_WIDTH) as u8 }
	/// Debug Timer 4 stopped when Core is halted (Width: 1, Offset: 2)
	pub fn set_dbg_tim4_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_TIM4_STOP_BIT_OFFSET, DBG_TIM4_STOP_BIT_WIDTH, value as u32); }

	const DBG_TIM5_STOP_BIT_OFFSET: u8 = 3;
	const DBG_TIM5_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Timer 5 stopped when Core is halted (Width: 1, Offset: 3)
	pub fn get_dbg_tim5_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_TIM5_STOP_BIT_OFFSET, DBG_TIM5_STOP_BIT_WIDTH) as u8 }
	/// Debug Timer 5 stopped when Core is halted (Width: 1, Offset: 3)
	pub fn set_dbg_tim5_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_TIM5_STOP_BIT_OFFSET, DBG_TIM5_STOP_BIT_WIDTH, value as u32); }

	const DBG_TIM6_STOP_BIT_OFFSET: u8 = 4;
	const DBG_TIM6_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Timer 6 stopped when Core is halted (Width: 1, Offset: 4)
	pub fn get_dbg_tim6_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_TIM6_STOP_BIT_OFFSET, DBG_TIM6_STOP_BIT_WIDTH) as u8 }
	/// Debug Timer 6 stopped when Core is halted (Width: 1, Offset: 4)
	pub fn set_dbg_tim6_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_TIM6_STOP_BIT_OFFSET, DBG_TIM6_STOP_BIT_WIDTH, value as u32); }

	const DBG_TIM7_STOP_BIT_OFFSET: u8 = 5;
	const DBG_TIM7_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Timer 7 stopped when Core is halted (Width: 1, Offset: 5)
	pub fn get_dbg_tim7_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_TIM7_STOP_BIT_OFFSET, DBG_TIM7_STOP_BIT_WIDTH) as u8 }
	/// Debug Timer 7 stopped when Core is halted (Width: 1, Offset: 5)
	pub fn set_dbg_tim7_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_TIM7_STOP_BIT_OFFSET, DBG_TIM7_STOP_BIT_WIDTH, value as u32); }

	const DBG_TIM12_STOP_BIT_OFFSET: u8 = 6;
	const DBG_TIM12_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Timer 12 stopped when Core is halted (Width: 1, Offset: 6)
	pub fn get_dbg_tim12_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_TIM12_STOP_BIT_OFFSET, DBG_TIM12_STOP_BIT_WIDTH) as u8 }
	/// Debug Timer 12 stopped when Core is halted (Width: 1, Offset: 6)
	pub fn set_dbg_tim12_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_TIM12_STOP_BIT_OFFSET, DBG_TIM12_STOP_BIT_WIDTH, value as u32); }

	const DBG_TIM13_STOP_BIT_OFFSET: u8 = 7;
	const DBG_TIM13_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Timer 13 stopped when Core is halted (Width: 1, Offset: 7)
	pub fn get_dbg_tim13_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_TIM13_STOP_BIT_OFFSET, DBG_TIM13_STOP_BIT_WIDTH) as u8 }
	/// Debug Timer 13 stopped when Core is halted (Width: 1, Offset: 7)
	pub fn set_dbg_tim13_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_TIM13_STOP_BIT_OFFSET, DBG_TIM13_STOP_BIT_WIDTH, value as u32); }

	const DBG_TIMER14_STOP_BIT_OFFSET: u8 = 8;
	const DBG_TIMER14_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Timer 14 stopped when Core is halted (Width: 1, Offset: 8)
	pub fn get_dbg_timer14_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_TIMER14_STOP_BIT_OFFSET, DBG_TIMER14_STOP_BIT_WIDTH) as u8 }
	/// Debug Timer 14 stopped when Core is halted (Width: 1, Offset: 8)
	pub fn set_dbg_timer14_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_TIMER14_STOP_BIT_OFFSET, DBG_TIMER14_STOP_BIT_WIDTH, value as u32); }

	const DBG_TIM18_STOP_BIT_OFFSET: u8 = 9;
	const DBG_TIM18_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Timer 18 stopped when Core is halted (Width: 1, Offset: 9)
	pub fn get_dbg_tim18_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_TIM18_STOP_BIT_OFFSET, DBG_TIM18_STOP_BIT_WIDTH) as u8 }
	/// Debug Timer 18 stopped when Core is halted (Width: 1, Offset: 9)
	pub fn set_dbg_tim18_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_TIM18_STOP_BIT_OFFSET, DBG_TIM18_STOP_BIT_WIDTH, value as u32); }

	const DBG_RTC_STOP_BIT_OFFSET: u8 = 10;
	const DBG_RTC_STOP_BIT_WIDTH: u8 = 1;
	/// Debug RTC stopped when Core is halted (Width: 1, Offset: 10)
	pub fn get_dbg_rtc_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_RTC_STOP_BIT_OFFSET, DBG_RTC_STOP_BIT_WIDTH) as u8 }
	/// Debug RTC stopped when Core is halted (Width: 1, Offset: 10)
	pub fn set_dbg_rtc_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_RTC_STOP_BIT_OFFSET, DBG_RTC_STOP_BIT_WIDTH, value as u32); }

	const DBG_WWDG_STOP_BIT_OFFSET: u8 = 11;
	const DBG_WWDG_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Window Wachdog stopped when Core is halted (Width: 1, Offset: 11)
	pub fn get_dbg_wwdg_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_WWDG_STOP_BIT_OFFSET, DBG_WWDG_STOP_BIT_WIDTH) as u8 }
	/// Debug Window Wachdog stopped when Core is halted (Width: 1, Offset: 11)
	pub fn set_dbg_wwdg_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_WWDG_STOP_BIT_OFFSET, DBG_WWDG_STOP_BIT_WIDTH, value as u32); }

	const DBG_IWDG_STOP_BIT_OFFSET: u8 = 12;
	const DBG_IWDG_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Independent Wachdog stopped when Core is halted (Width: 1, Offset: 12)
	pub fn get_dbg_iwdg_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_IWDG_STOP_BIT_OFFSET, DBG_IWDG_STOP_BIT_WIDTH) as u8 }
	/// Debug Independent Wachdog stopped when Core is halted (Width: 1, Offset: 12)
	pub fn set_dbg_iwdg_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_IWDG_STOP_BIT_OFFSET, DBG_IWDG_STOP_BIT_WIDTH, value as u32); }

	const I2C1_SMBUS_TIMEOUT_BIT_OFFSET: u8 = 21;
	const I2C1_SMBUS_TIMEOUT_BIT_WIDTH: u8 = 1;
	/// SMBUS timeout mode stopped when Core is halted (Width: 1, Offset: 21)
	pub fn get_i2c1_smbus_timeout() -> u8 { ::read(REGISTER_ADDRESS, I2C1_SMBUS_TIMEOUT_BIT_OFFSET, I2C1_SMBUS_TIMEOUT_BIT_WIDTH) as u8 }
	/// SMBUS timeout mode stopped when Core is halted (Width: 1, Offset: 21)
	pub fn set_i2c1_smbus_timeout(value: u8) { ::write(REGISTER_ADDRESS, I2C1_SMBUS_TIMEOUT_BIT_OFFSET, I2C1_SMBUS_TIMEOUT_BIT_WIDTH, value as u32); }

	const I2C2_SMBUS_TIMEOUT_BIT_OFFSET: u8 = 22;
	const I2C2_SMBUS_TIMEOUT_BIT_WIDTH: u8 = 1;
	/// SMBUS timeout mode stopped when Core is halted (Width: 1, Offset: 22)
	pub fn get_i2c2_smbus_timeout() -> u8 { ::read(REGISTER_ADDRESS, I2C2_SMBUS_TIMEOUT_BIT_OFFSET, I2C2_SMBUS_TIMEOUT_BIT_WIDTH) as u8 }
	/// SMBUS timeout mode stopped when Core is halted (Width: 1, Offset: 22)
	pub fn set_i2c2_smbus_timeout(value: u8) { ::write(REGISTER_ADDRESS, I2C2_SMBUS_TIMEOUT_BIT_OFFSET, I2C2_SMBUS_TIMEOUT_BIT_WIDTH, value as u32); }

	const DBG_CAN_STOP_BIT_OFFSET: u8 = 25;
	const DBG_CAN_STOP_BIT_WIDTH: u8 = 1;
	/// Debug CAN stopped when core is halted (Width: 1, Offset: 25)
	pub fn get_dbg_can_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_CAN_STOP_BIT_OFFSET, DBG_CAN_STOP_BIT_WIDTH) as u8 }
	/// Debug CAN stopped when core is halted (Width: 1, Offset: 25)
	pub fn set_dbg_can_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_CAN_STOP_BIT_OFFSET, DBG_CAN_STOP_BIT_WIDTH, value as u32); }
}
/// APB High Freeze Register
/// Size: 0x20 bits
pub mod apb2fz {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DBG_TIM15_STOP_BIT_OFFSET: u8 = 2;
	const DBG_TIM15_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Timer 15 stopped when Core is halted (Width: 1, Offset: 2)
	pub fn get_dbg_tim15_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_TIM15_STOP_BIT_OFFSET, DBG_TIM15_STOP_BIT_WIDTH) as u8 }
	/// Debug Timer 15 stopped when Core is halted (Width: 1, Offset: 2)
	pub fn set_dbg_tim15_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_TIM15_STOP_BIT_OFFSET, DBG_TIM15_STOP_BIT_WIDTH, value as u32); }

	const DBG_TIM16_STOP_BIT_OFFSET: u8 = 3;
	const DBG_TIM16_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Timer 16 stopped when Core is halted (Width: 1, Offset: 3)
	pub fn get_dbg_tim16_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_TIM16_STOP_BIT_OFFSET, DBG_TIM16_STOP_BIT_WIDTH) as u8 }
	/// Debug Timer 16 stopped when Core is halted (Width: 1, Offset: 3)
	pub fn set_dbg_tim16_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_TIM16_STOP_BIT_OFFSET, DBG_TIM16_STOP_BIT_WIDTH, value as u32); }

	const DBG_TIM17_STO_BIT_OFFSET: u8 = 4;
	const DBG_TIM17_STO_BIT_WIDTH: u8 = 1;
	/// Debug Timer 17 stopped when Core is halted (Width: 1, Offset: 4)
	pub fn get_dbg_tim17_sto() -> u8 { ::read(REGISTER_ADDRESS, DBG_TIM17_STO_BIT_OFFSET, DBG_TIM17_STO_BIT_WIDTH) as u8 }
	/// Debug Timer 17 stopped when Core is halted (Width: 1, Offset: 4)
	pub fn set_dbg_tim17_sto(value: u8) { ::write(REGISTER_ADDRESS, DBG_TIM17_STO_BIT_OFFSET, DBG_TIM17_STO_BIT_WIDTH, value as u32); }

	const DBG_TIM19_STOP_BIT_OFFSET: u8 = 5;
	const DBG_TIM19_STOP_BIT_WIDTH: u8 = 1;
	/// Debug Timer 19 stopped when Core is halted (Width: 1, Offset: 5)
	pub fn get_dbg_tim19_stop() -> u8 { ::read(REGISTER_ADDRESS, DBG_TIM19_STOP_BIT_OFFSET, DBG_TIM19_STOP_BIT_WIDTH) as u8 }
	/// Debug Timer 19 stopped when Core is halted (Width: 1, Offset: 5)
	pub fn set_dbg_tim19_stop(value: u8) { ::write(REGISTER_ADDRESS, DBG_TIM19_STOP_BIT_OFFSET, DBG_TIM19_STOP_BIT_WIDTH, value as u32); }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>DBGMCU</name>
  <description>Debug support</description>
  <groupName>DBGMCU</groupName>
  <baseAddress>0xE0042000</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>IDCODE</name>
      <displayName>IDCODE</displayName>
      <description>MCU Device ID Code Register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x0</resetValue>
      <fields>
        <field>
          <name>DEV_ID</name>
          <description>Device Identifier</description>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
        <field>
          <name>REV_ID</name>
          <description>Revision Identifier</description>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CR</name>
      <displayName>CR</displayName>
      <description>Debug MCU Configuration
          Register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0</resetValue>
      <fields>
        <field>
          <name>DBG_SLEEP</name>
          <description>Debug Sleep mode</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_STOP</name>
          <description>Debug Stop Mode</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_STANDBY</name>
          <description>Debug Standby Mode</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TRACE_IOEN</name>
          <description>Trace pin assignment
              control</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TRACE_MODE</name>
          <description>Trace pin assignment
              control</description>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>APB1FZ</name>
      <displayName>APB1FZ</displayName>
      <description>APB Low Freeze Register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0</resetValue>
      <fields>
        <field>
          <name>DBG_TIM2_STOP</name>
          <description>Debug Timer 2 stopped when Core is
              halted</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_TIM3_STOP</name>
          <description>Debug Timer 3 stopped when Core is
              halted</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_TIM4_STOP</name>
          <description>Debug Timer 4 stopped when Core is
              halted</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_TIM5_STOP</name>
          <description>Debug Timer 5 stopped when Core is
              halted</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_TIM6_STOP</name>
          <description>Debug Timer 6 stopped when Core is
              halted</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_TIM7_STOP</name>
          <description>Debug Timer 7 stopped when Core is
              halted</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_TIM12_STOP</name>
          <description>Debug Timer 12 stopped when Core is
              halted</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_TIM13_STOP</name>
          <description>Debug Timer 13 stopped when Core is
              halted</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_TIMER14_STOP</name>
          <description>Debug Timer 14 stopped when Core is
              halted</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_TIM18_STOP</name>
          <description>Debug Timer 18 stopped when Core is
              halted</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_RTC_STOP</name>
          <description>Debug RTC stopped when Core is
              halted</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_WWDG_STOP</name>
          <description>Debug Window Wachdog stopped when Core
              is halted</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_IWDG_STOP</name>
          <description>Debug Independent Wachdog stopped when
              Core is halted</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2C1_SMBUS_TIMEOUT</name>
          <description>SMBUS timeout mode stopped when Core is
              halted</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2C2_SMBUS_TIMEOUT</name>
          <description>SMBUS timeout mode stopped when Core is
              halted</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_CAN_STOP</name>
          <description>Debug CAN stopped when core is
              halted</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>APB2FZ</name>
      <displayName>APB2FZ</displayName>
      <description>APB High Freeze Register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0</resetValue>
      <fields>
        <field>
          <name>DBG_TIM15_STOP</name>
          <description>Debug Timer 15 stopped when Core is
              halted</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_TIM16_STOP</name>
          <description>Debug Timer 16 stopped when Core is
              halted</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_TIM17_STO</name>
          <description>Debug Timer 17 stopped when Core is
              halted</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DBG_TIM19_STOP</name>
          <description>Debug Timer 19 stopped when Core is
              halted</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
</peripheral>*/
