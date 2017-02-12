/// MOD WWDG
/// Window watchdog
const BASE_ADDRESS: u32 = 0x40002C00;
/// Control register
/// Size: 0x20 bits
pub mod cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const T_BIT_OFFSET: u8 = 0;
	const T_BIT_WIDTH: u8 = 7;
	/// 7-bit counter (Width: 7, Offset: 0)
	pub fn get_t() -> u8 { ::read(REGISTER_ADDRESS, T_BIT_OFFSET, T_BIT_WIDTH) as u8 }
	/// 7-bit counter (Width: 7, Offset: 0)
	pub fn set_t(value: u8) { ::write(REGISTER_ADDRESS, T_BIT_OFFSET, T_BIT_WIDTH, value as u32); }

	const WDGA_BIT_OFFSET: u8 = 7;
	const WDGA_BIT_WIDTH: u8 = 1;
	/// Activation bit (Width: 1, Offset: 7)
	pub fn get_wdga() -> u8 { ::read(REGISTER_ADDRESS, WDGA_BIT_OFFSET, WDGA_BIT_WIDTH) as u8 }
	/// Activation bit (Width: 1, Offset: 7)
	pub fn set_wdga(value: u8) { ::write(REGISTER_ADDRESS, WDGA_BIT_OFFSET, WDGA_BIT_WIDTH, value as u32); }
}
/// Configuration register
/// Size: 0x20 bits
pub mod cfr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EWI_BIT_OFFSET: u8 = 9;
	const EWI_BIT_WIDTH: u8 = 1;
	/// Early wakeup interrupt (Width: 1, Offset: 9)
	pub fn get_ewi() -> u8 { ::read(REGISTER_ADDRESS, EWI_BIT_OFFSET, EWI_BIT_WIDTH) as u8 }
	/// Early wakeup interrupt (Width: 1, Offset: 9)
	pub fn set_ewi(value: u8) { ::write(REGISTER_ADDRESS, EWI_BIT_OFFSET, EWI_BIT_WIDTH, value as u32); }

	const WDGTB_BIT_OFFSET: u8 = 7;
	const WDGTB_BIT_WIDTH: u8 = 2;
	/// Timer base (Width: 2, Offset: 7)
	pub fn get_wdgtb() -> u8 { ::read(REGISTER_ADDRESS, WDGTB_BIT_OFFSET, WDGTB_BIT_WIDTH) as u8 }
	/// Timer base (Width: 2, Offset: 7)
	pub fn set_wdgtb(value: u8) { ::write(REGISTER_ADDRESS, WDGTB_BIT_OFFSET, WDGTB_BIT_WIDTH, value as u32); }

	const W_BIT_OFFSET: u8 = 0;
	const W_BIT_WIDTH: u8 = 7;
	/// 7-bit window value (Width: 7, Offset: 0)
	pub fn get_w() -> u8 { ::read(REGISTER_ADDRESS, W_BIT_OFFSET, W_BIT_WIDTH) as u8 }
	/// 7-bit window value (Width: 7, Offset: 0)
	pub fn set_w(value: u8) { ::write(REGISTER_ADDRESS, W_BIT_OFFSET, W_BIT_WIDTH, value as u32); }
}
/// Status register
/// Size: 0x20 bits
pub mod sr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EWIF_BIT_OFFSET: u8 = 0;
	const EWIF_BIT_WIDTH: u8 = 1;
	/// Early wakeup interrupt flag (Width: 1, Offset: 0)
	pub fn get_ewif() -> u8 { ::read(REGISTER_ADDRESS, EWIF_BIT_OFFSET, EWIF_BIT_WIDTH) as u8 }
	/// Early wakeup interrupt flag (Width: 1, Offset: 0)
	pub fn set_ewif(value: u8) { ::write(REGISTER_ADDRESS, EWIF_BIT_OFFSET, EWIF_BIT_WIDTH, value as u32); }
}
/// Window Watchdog interrupt
pub const INTERRUPT_WWDG: u32 = 0;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>WWDG</name>
  <description>Window watchdog</description>
  <groupName>WWDG</groupName>
  <baseAddress>0x40002C00</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>CR</name>
      <displayName>CR</displayName>
      <description>Control register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000007F</resetValue>
      <fields>
        <field>
          <name>T</name>
          <description>7-bit counter</description>
          <bitOffset>0</bitOffset>
          <bitWidth>7</bitWidth>
        </field>
        <field>
          <name>WDGA</name>
          <description>Activation bit</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CFR</name>
      <displayName>CFR</displayName>
      <description>Configuration register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000007F</resetValue>
      <fields>
        <field>
          <name>EWI</name>
          <description>Early wakeup interrupt</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>WDGTB</name>
          <description>Timer base</description>
          <bitOffset>7</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>W</name>
          <description>7-bit window value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>7</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SR</name>
      <displayName>SR</displayName>
      <description>Status register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EWIF</name>
          <description>Early wakeup interrupt
              flag</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>WWDG</name>
    <description>Window Watchdog interrupt</description>
    <value>0</value>
  </interrupt>
</peripheral>*/
