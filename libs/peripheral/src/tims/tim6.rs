/// MOD TIM6
/// Basic timers
const BASE_ADDRESS: u32 = 0x40001000;
/// control register 1
/// Size: 0x20 bits
pub mod cr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CEN_BIT_OFFSET: u8 = 0;
	const CEN_BIT_WIDTH: u8 = 1;
	/// Counter enable (Width: 1, Offset: 0)
	pub fn get_cen() -> u8 { ::read(REGISTER_ADDRESS, CEN_BIT_OFFSET, CEN_BIT_WIDTH) as u8 }
	/// Counter enable (Width: 1, Offset: 0)
	pub fn set_cen(value: u8) { ::write(REGISTER_ADDRESS, CEN_BIT_OFFSET, CEN_BIT_WIDTH, value as u32); }

	const UDIS_BIT_OFFSET: u8 = 1;
	const UDIS_BIT_WIDTH: u8 = 1;
	/// Update disable (Width: 1, Offset: 1)
	pub fn get_udis() -> u8 { ::read(REGISTER_ADDRESS, UDIS_BIT_OFFSET, UDIS_BIT_WIDTH) as u8 }
	/// Update disable (Width: 1, Offset: 1)
	pub fn set_udis(value: u8) { ::write(REGISTER_ADDRESS, UDIS_BIT_OFFSET, UDIS_BIT_WIDTH, value as u32); }

	const URS_BIT_OFFSET: u8 = 2;
	const URS_BIT_WIDTH: u8 = 1;
	/// Update request source (Width: 1, Offset: 2)
	pub fn get_urs() -> u8 { ::read(REGISTER_ADDRESS, URS_BIT_OFFSET, URS_BIT_WIDTH) as u8 }
	/// Update request source (Width: 1, Offset: 2)
	pub fn set_urs(value: u8) { ::write(REGISTER_ADDRESS, URS_BIT_OFFSET, URS_BIT_WIDTH, value as u32); }

	const OPM_BIT_OFFSET: u8 = 3;
	const OPM_BIT_WIDTH: u8 = 1;
	/// One-pulse mode (Width: 1, Offset: 3)
	pub fn get_opm() -> u8 { ::read(REGISTER_ADDRESS, OPM_BIT_OFFSET, OPM_BIT_WIDTH) as u8 }
	/// One-pulse mode (Width: 1, Offset: 3)
	pub fn set_opm(value: u8) { ::write(REGISTER_ADDRESS, OPM_BIT_OFFSET, OPM_BIT_WIDTH, value as u32); }

	const ARPE_BIT_OFFSET: u8 = 7;
	const ARPE_BIT_WIDTH: u8 = 1;
	/// Auto-reload preload enable (Width: 1, Offset: 7)
	pub fn get_arpe() -> u8 { ::read(REGISTER_ADDRESS, ARPE_BIT_OFFSET, ARPE_BIT_WIDTH) as u8 }
	/// Auto-reload preload enable (Width: 1, Offset: 7)
	pub fn set_arpe(value: u8) { ::write(REGISTER_ADDRESS, ARPE_BIT_OFFSET, ARPE_BIT_WIDTH, value as u32); }

	const UIFREMAP_BIT_OFFSET: u8 = 11;
	const UIFREMAP_BIT_WIDTH: u8 = 1;
	/// UIF status bit remapping (Width: 1, Offset: 11)
	pub fn get_uifremap() -> u8 { ::read(REGISTER_ADDRESS, UIFREMAP_BIT_OFFSET, UIFREMAP_BIT_WIDTH) as u8 }
	/// UIF status bit remapping (Width: 1, Offset: 11)
	pub fn set_uifremap(value: u8) { ::write(REGISTER_ADDRESS, UIFREMAP_BIT_OFFSET, UIFREMAP_BIT_WIDTH, value as u32); }
}
/// control register 2
/// Size: 0x20 bits
pub mod cr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MMS_BIT_OFFSET: u8 = 4;
	const MMS_BIT_WIDTH: u8 = 3;
	/// Master mode selection (Width: 3, Offset: 4)
	pub fn get_mms() -> u8 { ::read(REGISTER_ADDRESS, MMS_BIT_OFFSET, MMS_BIT_WIDTH) as u8 }
	/// Master mode selection (Width: 3, Offset: 4)
	pub fn set_mms(value: u8) { ::write(REGISTER_ADDRESS, MMS_BIT_OFFSET, MMS_BIT_WIDTH, value as u32); }
}
/// DMA/Interrupt enable register
/// Size: 0x20 bits
pub mod dier {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const UDE_BIT_OFFSET: u8 = 8;
	const UDE_BIT_WIDTH: u8 = 1;
	/// Update DMA request enable (Width: 1, Offset: 8)
	pub fn get_ude() -> u8 { ::read(REGISTER_ADDRESS, UDE_BIT_OFFSET, UDE_BIT_WIDTH) as u8 }
	/// Update DMA request enable (Width: 1, Offset: 8)
	pub fn set_ude(value: u8) { ::write(REGISTER_ADDRESS, UDE_BIT_OFFSET, UDE_BIT_WIDTH, value as u32); }

	const UIE_BIT_OFFSET: u8 = 0;
	const UIE_BIT_WIDTH: u8 = 1;
	/// Update interrupt enable (Width: 1, Offset: 0)
	pub fn get_uie() -> u8 { ::read(REGISTER_ADDRESS, UIE_BIT_OFFSET, UIE_BIT_WIDTH) as u8 }
	/// Update interrupt enable (Width: 1, Offset: 0)
	pub fn set_uie(value: u8) { ::write(REGISTER_ADDRESS, UIE_BIT_OFFSET, UIE_BIT_WIDTH, value as u32); }
}
/// status register
/// Size: 0x20 bits
pub mod sr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const UIF_BIT_OFFSET: u8 = 0;
	const UIF_BIT_WIDTH: u8 = 1;
	/// Update interrupt flag (Width: 1, Offset: 0)
	pub fn get_uif() -> u8 { ::read(REGISTER_ADDRESS, UIF_BIT_OFFSET, UIF_BIT_WIDTH) as u8 }
	/// Update interrupt flag (Width: 1, Offset: 0)
	pub fn set_uif(value: u8) { ::write(REGISTER_ADDRESS, UIF_BIT_OFFSET, UIF_BIT_WIDTH, value as u32); }
}
/// event generation register
/// Size: 0x20 bits
pub mod egr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const UG_BIT_OFFSET: u8 = 0;
	const UG_BIT_WIDTH: u8 = 1;
	/// Update generation (Width: 1, Offset: 0)
	pub fn set_ug(value: u8) { ::write(REGISTER_ADDRESS, UG_BIT_OFFSET, UG_BIT_WIDTH, value as u32); }
}
/// counter
/// Size: 0x20 bits
pub mod cnt {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x24;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CNT_BIT_OFFSET: u8 = 0;
	const CNT_BIT_WIDTH: u8 = 16;
	/// Low counter value (Width: 16, Offset: 0)
	pub fn get_cnt() -> u16 { ::read(REGISTER_ADDRESS, CNT_BIT_OFFSET, CNT_BIT_WIDTH) as u16 }
	/// Low counter value (Width: 16, Offset: 0)
	pub fn set_cnt(value: u16) { ::write(REGISTER_ADDRESS, CNT_BIT_OFFSET, CNT_BIT_WIDTH, value as u32); }

	const UIFCPY_BIT_OFFSET: u8 = 31;
	const UIFCPY_BIT_WIDTH: u8 = 1;
	/// UIF Copy (Width: 1, Offset: 31)
	pub fn get_uifcpy() -> u8 { ::read(REGISTER_ADDRESS, UIFCPY_BIT_OFFSET, UIFCPY_BIT_WIDTH) as u8 }
}
/// prescaler
/// Size: 0x20 bits
pub mod psc {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x28;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PSC_BIT_OFFSET: u8 = 0;
	const PSC_BIT_WIDTH: u8 = 16;
	/// Prescaler value (Width: 16, Offset: 0)
	pub fn get_psc() -> u16 { ::read(REGISTER_ADDRESS, PSC_BIT_OFFSET, PSC_BIT_WIDTH) as u16 }
	/// Prescaler value (Width: 16, Offset: 0)
	pub fn set_psc(value: u16) { ::write(REGISTER_ADDRESS, PSC_BIT_OFFSET, PSC_BIT_WIDTH, value as u32); }
}
/// auto-reload register
/// Size: 0x20 bits
pub mod arr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x2C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ARR_BIT_OFFSET: u8 = 0;
	const ARR_BIT_WIDTH: u8 = 16;
	/// Low Auto-reload value (Width: 16, Offset: 0)
	pub fn get_arr() -> u16 { ::read(REGISTER_ADDRESS, ARR_BIT_OFFSET, ARR_BIT_WIDTH) as u16 }
	/// Low Auto-reload value (Width: 16, Offset: 0)
	pub fn set_arr(value: u16) { ::write(REGISTER_ADDRESS, ARR_BIT_OFFSET, ARR_BIT_WIDTH, value as u32); }
}
/// TIM6 global and DAC12 underrun interrupts
pub const INTERRUPT_TIM6_DACUNDER: u32 = 54;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>TIM6</name>
  <description>Basic timers</description>
  <groupName>TIMs</groupName>
  <baseAddress>0x40001000</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>CR1</name>
      <displayName>CR1</displayName>
      <description>control register 1</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>CEN</name>
          <description>Counter enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>UDIS</name>
          <description>Update disable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>URS</name>
          <description>Update request source</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OPM</name>
          <description>One-pulse mode</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ARPE</name>
          <description>Auto-reload preload enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>UIFREMAP</name>
          <description>UIF status bit remapping</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CR2</name>
      <displayName>CR2</displayName>
      <description>control register 2</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>MMS</name>
          <description>Master mode selection</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DIER</name>
      <displayName>DIER</displayName>
      <description>DMA/Interrupt enable register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>UDE</name>
          <description>Update DMA request enable</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>UIE</name>
          <description>Update interrupt enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SR</name>
      <displayName>SR</displayName>
      <description>status register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>UIF</name>
          <description>Update interrupt flag</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>EGR</name>
      <displayName>EGR</displayName>
      <description>event generation register</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>write-only</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>UG</name>
          <description>Update generation</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CNT</name>
      <displayName>CNT</displayName>
      <description>counter</description>
      <addressOffset>0x24</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CNT</name>
          <description>Low counter value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>UIFCPY</name>
          <description>UIF Copy</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
      </fields>
    </register>
    <register>
      <name>PSC</name>
      <displayName>PSC</displayName>
      <description>prescaler</description>
      <addressOffset>0x28</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>PSC</name>
          <description>Prescaler value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ARR</name>
      <displayName>ARR</displayName>
      <description>auto-reload register</description>
      <addressOffset>0x2C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>ARR</name>
          <description>Low Auto-reload value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>TIM6_DACUNDER</name>
    <description>TIM6 global and DAC12 underrun
        interrupts</description>
    <value>54</value>
  </interrupt>
</peripheral>*/
