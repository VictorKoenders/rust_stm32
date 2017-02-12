/// MOD IWDG
/// Independent watchdog
const BASE_ADDRESS: u32 = 0x40003000;
/// Key register
/// Size: 0x20 bits
pub mod kr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const KEY_BIT_OFFSET: u8 = 0;
	const KEY_BIT_WIDTH: u8 = 16;
	/// Key value (Width: 16, Offset: 0)
	pub fn set_key(value: u16) { ::write(REGISTER_ADDRESS, KEY_BIT_OFFSET, KEY_BIT_WIDTH, value as u32); }
}
/// Prescaler register
/// Size: 0x20 bits
pub mod pr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PR_BIT_OFFSET: u8 = 0;
	const PR_BIT_WIDTH: u8 = 3;
	/// Prescaler divider (Width: 3, Offset: 0)
	pub fn get_pr() -> u8 { ::read(REGISTER_ADDRESS, PR_BIT_OFFSET, PR_BIT_WIDTH) as u8 }
	/// Prescaler divider (Width: 3, Offset: 0)
	pub fn set_pr(value: u8) { ::write(REGISTER_ADDRESS, PR_BIT_OFFSET, PR_BIT_WIDTH, value as u32); }
}
/// Reload register
/// Size: 0x20 bits
pub mod rlr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const RL_BIT_OFFSET: u8 = 0;
	const RL_BIT_WIDTH: u8 = 12;
	/// Watchdog counter reload value (Width: 12, Offset: 0)
	pub fn get_rl() -> u16 { ::read(REGISTER_ADDRESS, RL_BIT_OFFSET, RL_BIT_WIDTH) as u16 }
	/// Watchdog counter reload value (Width: 12, Offset: 0)
	pub fn set_rl(value: u16) { ::write(REGISTER_ADDRESS, RL_BIT_OFFSET, RL_BIT_WIDTH, value as u32); }
}
/// Status register
/// Size: 0x20 bits
pub mod sr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PVU_BIT_OFFSET: u8 = 0;
	const PVU_BIT_WIDTH: u8 = 1;
	/// Watchdog prescaler value update (Width: 1, Offset: 0)
	pub fn get_pvu() -> u8 { ::read(REGISTER_ADDRESS, PVU_BIT_OFFSET, PVU_BIT_WIDTH) as u8 }

	const RVU_BIT_OFFSET: u8 = 1;
	const RVU_BIT_WIDTH: u8 = 1;
	/// Watchdog counter reload value update (Width: 1, Offset: 1)
	pub fn get_rvu() -> u8 { ::read(REGISTER_ADDRESS, RVU_BIT_OFFSET, RVU_BIT_WIDTH) as u8 }

	const WVU_BIT_OFFSET: u8 = 2;
	const WVU_BIT_WIDTH: u8 = 1;
	/// Watchdog counter window value update (Width: 1, Offset: 2)
	pub fn get_wvu() -> u8 { ::read(REGISTER_ADDRESS, WVU_BIT_OFFSET, WVU_BIT_WIDTH) as u8 }
}
/// Window register
/// Size: 0x20 bits
pub mod winr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const WIN_BIT_OFFSET: u8 = 0;
	const WIN_BIT_WIDTH: u8 = 12;
	/// Watchdog counter window value (Width: 12, Offset: 0)
	pub fn get_win() -> u16 { ::read(REGISTER_ADDRESS, WIN_BIT_OFFSET, WIN_BIT_WIDTH) as u16 }
	/// Watchdog counter window value (Width: 12, Offset: 0)
	pub fn set_win(value: u16) { ::write(REGISTER_ADDRESS, WIN_BIT_OFFSET, WIN_BIT_WIDTH, value as u32); }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>IWDG</name>
  <description>Independent watchdog</description>
  <groupName>IWDG</groupName>
  <baseAddress>0x40003000</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>KR</name>
      <displayName>KR</displayName>
      <description>Key register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>write-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>KEY</name>
          <description>Key value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>PR</name>
      <displayName>PR</displayName>
      <description>Prescaler register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PR</name>
          <description>Prescaler divider</description>
          <bitOffset>0</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>RLR</name>
      <displayName>RLR</displayName>
      <description>Reload register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000FFF</resetValue>
      <fields>
        <field>
          <name>RL</name>
          <description>Watchdog counter reload
              value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SR</name>
      <displayName>SR</displayName>
      <description>Status register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PVU</name>
          <description>Watchdog prescaler value
              update</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RVU</name>
          <description>Watchdog counter reload value
              update</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>WVU</name>
          <description>Watchdog counter window value
              update</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>WINR</name>
      <displayName>WINR</displayName>
      <description>Window register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000FFF</resetValue>
      <fields>
        <field>
          <name>WIN</name>
          <description>Watchdog counter window
              value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
</peripheral>*/
