/// MOD CRC
/// cyclic redundancy check calculation unit
const BASE_ADDRESS: u32 = 0x40023000;
/// Data register
/// Size: 0x20 bits
pub mod dr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DR_BIT_OFFSET: u8 = 0;
	const DR_BIT_WIDTH: u8 = 32;
	/// Data register bits (Width: 32, Offset: 0)
	pub fn get_dr() -> u32 { ::read(REGISTER_ADDRESS, DR_BIT_OFFSET, DR_BIT_WIDTH) as u32 }
	/// Data register bits (Width: 32, Offset: 0)
	pub fn set_dr(value: u32) { ::write(REGISTER_ADDRESS, DR_BIT_OFFSET, DR_BIT_WIDTH, value as u32); }
}
/// Independent data register
/// Size: 0x20 bits
pub mod idr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IDR_BIT_OFFSET: u8 = 0;
	const IDR_BIT_WIDTH: u8 = 8;
	/// General-purpose 8-bit data register bits (Width: 8, Offset: 0)
	pub fn get_idr() -> u8 { ::read(REGISTER_ADDRESS, IDR_BIT_OFFSET, IDR_BIT_WIDTH) as u8 }
	/// General-purpose 8-bit data register bits (Width: 8, Offset: 0)
	pub fn set_idr(value: u8) { ::write(REGISTER_ADDRESS, IDR_BIT_OFFSET, IDR_BIT_WIDTH, value as u32); }
}
/// Control register
/// Size: 0x20 bits
pub mod cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const RESET_BIT_OFFSET: u8 = 0;
	const RESET_BIT_WIDTH: u8 = 1;
	/// reset bit (Width: 1, Offset: 0)
	pub fn get_reset() -> u8 { ::read(REGISTER_ADDRESS, RESET_BIT_OFFSET, RESET_BIT_WIDTH) as u8 }
	/// reset bit (Width: 1, Offset: 0)
	pub fn set_reset(value: u8) { ::write(REGISTER_ADDRESS, RESET_BIT_OFFSET, RESET_BIT_WIDTH, value as u32); }

	const POLYSIZE_BIT_OFFSET: u8 = 3;
	const POLYSIZE_BIT_WIDTH: u8 = 2;
	/// Polynomial size (Width: 2, Offset: 3)
	pub fn get_polysize() -> u8 { ::read(REGISTER_ADDRESS, POLYSIZE_BIT_OFFSET, POLYSIZE_BIT_WIDTH) as u8 }
	/// Polynomial size (Width: 2, Offset: 3)
	pub fn set_polysize(value: u8) { ::write(REGISTER_ADDRESS, POLYSIZE_BIT_OFFSET, POLYSIZE_BIT_WIDTH, value as u32); }

	const REV_IN_BIT_OFFSET: u8 = 5;
	const REV_IN_BIT_WIDTH: u8 = 2;
	/// Reverse input data (Width: 2, Offset: 5)
	pub fn get_rev_in() -> u8 { ::read(REGISTER_ADDRESS, REV_IN_BIT_OFFSET, REV_IN_BIT_WIDTH) as u8 }
	/// Reverse input data (Width: 2, Offset: 5)
	pub fn set_rev_in(value: u8) { ::write(REGISTER_ADDRESS, REV_IN_BIT_OFFSET, REV_IN_BIT_WIDTH, value as u32); }

	const REV_OUT_BIT_OFFSET: u8 = 7;
	const REV_OUT_BIT_WIDTH: u8 = 1;
	/// Reverse output data (Width: 1, Offset: 7)
	pub fn get_rev_out() -> u8 { ::read(REGISTER_ADDRESS, REV_OUT_BIT_OFFSET, REV_OUT_BIT_WIDTH) as u8 }
	/// Reverse output data (Width: 1, Offset: 7)
	pub fn set_rev_out(value: u8) { ::write(REGISTER_ADDRESS, REV_OUT_BIT_OFFSET, REV_OUT_BIT_WIDTH, value as u32); }
}
/// Initial CRC value
/// Size: 0x20 bits
pub mod init {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const INIT_BIT_OFFSET: u8 = 0;
	const INIT_BIT_WIDTH: u8 = 32;
	/// Programmable initial CRC value (Width: 32, Offset: 0)
	pub fn get_init() -> u32 { ::read(REGISTER_ADDRESS, INIT_BIT_OFFSET, INIT_BIT_WIDTH) as u32 }
	/// Programmable initial CRC value (Width: 32, Offset: 0)
	pub fn set_init(value: u32) { ::write(REGISTER_ADDRESS, INIT_BIT_OFFSET, INIT_BIT_WIDTH, value as u32); }
}
/// CRC polynomial
/// Size: 0x20 bits
pub mod pol {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const POL_BIT_OFFSET: u8 = 0;
	const POL_BIT_WIDTH: u8 = 32;
	/// Programmable polynomial (Width: 32, Offset: 0)
	pub fn get_pol() -> u32 { ::read(REGISTER_ADDRESS, POL_BIT_OFFSET, POL_BIT_WIDTH) as u32 }
	/// Programmable polynomial (Width: 32, Offset: 0)
	pub fn set_pol(value: u32) { ::write(REGISTER_ADDRESS, POL_BIT_OFFSET, POL_BIT_WIDTH, value as u32); }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>CRC</name>
  <description>cyclic redundancy check calculation
      unit</description>
  <groupName>CRC</groupName>
  <baseAddress>0x40023000</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>DR</name>
      <displayName>DR</displayName>
      <description>Data register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0xFFFFFFFF</resetValue>
      <fields>
        <field>
          <name>DR</name>
          <description>Data register bits</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IDR</name>
      <displayName>IDR</displayName>
      <description>Independent data register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IDR</name>
          <description>General-purpose 8-bit data register
              bits</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CR</name>
      <displayName>CR</displayName>
      <description>Control register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>RESET</name>
          <description>reset bit</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>POLYSIZE</name>
          <description>Polynomial size</description>
          <bitOffset>3</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>REV_IN</name>
          <description>Reverse input data</description>
          <bitOffset>5</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>REV_OUT</name>
          <description>Reverse output data</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>INIT</name>
      <displayName>INIT</displayName>
      <description>Initial CRC value</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0xFFFFFFFF</resetValue>
      <fields>
        <field>
          <name>INIT</name>
          <description>Programmable initial CRC
              value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>POL</name>
      <displayName>POL</displayName>
      <description>CRC polynomial</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x04C11DB7</resetValue>
      <fields>
        <field>
          <name>POL</name>
          <description>Programmable polynomial</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
</peripheral>*/
