/// MOD PWR
/// Power control
const BASE_ADDRESS: u32 = 0x40007000;
/// power control register
/// Size: 0x20 bits
pub mod cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const LPDS_BIT_OFFSET: u8 = 0;
	const LPDS_BIT_WIDTH: u8 = 1;
	/// Low-power deep sleep (Width: 1, Offset: 0)
	pub fn get_lpds() -> u8 { ::read(REGISTER_ADDRESS, LPDS_BIT_OFFSET, LPDS_BIT_WIDTH) as u8 }
	/// Low-power deep sleep (Width: 1, Offset: 0)
	pub fn set_lpds(value: u8) { ::write(REGISTER_ADDRESS, LPDS_BIT_OFFSET, LPDS_BIT_WIDTH, value as u32); }

	const PDDS_BIT_OFFSET: u8 = 1;
	const PDDS_BIT_WIDTH: u8 = 1;
	/// Power down deepsleep (Width: 1, Offset: 1)
	pub fn get_pdds() -> u8 { ::read(REGISTER_ADDRESS, PDDS_BIT_OFFSET, PDDS_BIT_WIDTH) as u8 }
	/// Power down deepsleep (Width: 1, Offset: 1)
	pub fn set_pdds(value: u8) { ::write(REGISTER_ADDRESS, PDDS_BIT_OFFSET, PDDS_BIT_WIDTH, value as u32); }

	const CWUF_BIT_OFFSET: u8 = 2;
	const CWUF_BIT_WIDTH: u8 = 1;
	/// Clear wakeup flag (Width: 1, Offset: 2)
	pub fn get_cwuf() -> u8 { ::read(REGISTER_ADDRESS, CWUF_BIT_OFFSET, CWUF_BIT_WIDTH) as u8 }
	/// Clear wakeup flag (Width: 1, Offset: 2)
	pub fn set_cwuf(value: u8) { ::write(REGISTER_ADDRESS, CWUF_BIT_OFFSET, CWUF_BIT_WIDTH, value as u32); }

	const CSBF_BIT_OFFSET: u8 = 3;
	const CSBF_BIT_WIDTH: u8 = 1;
	/// Clear standby flag (Width: 1, Offset: 3)
	pub fn get_csbf() -> u8 { ::read(REGISTER_ADDRESS, CSBF_BIT_OFFSET, CSBF_BIT_WIDTH) as u8 }
	/// Clear standby flag (Width: 1, Offset: 3)
	pub fn set_csbf(value: u8) { ::write(REGISTER_ADDRESS, CSBF_BIT_OFFSET, CSBF_BIT_WIDTH, value as u32); }

	const PVDE_BIT_OFFSET: u8 = 4;
	const PVDE_BIT_WIDTH: u8 = 1;
	/// Power voltage detector enable (Width: 1, Offset: 4)
	pub fn get_pvde() -> u8 { ::read(REGISTER_ADDRESS, PVDE_BIT_OFFSET, PVDE_BIT_WIDTH) as u8 }
	/// Power voltage detector enable (Width: 1, Offset: 4)
	pub fn set_pvde(value: u8) { ::write(REGISTER_ADDRESS, PVDE_BIT_OFFSET, PVDE_BIT_WIDTH, value as u32); }

	const PLS_BIT_OFFSET: u8 = 5;
	const PLS_BIT_WIDTH: u8 = 3;
	/// PVD level selection (Width: 3, Offset: 5)
	pub fn get_pls() -> u8 { ::read(REGISTER_ADDRESS, PLS_BIT_OFFSET, PLS_BIT_WIDTH) as u8 }
	/// PVD level selection (Width: 3, Offset: 5)
	pub fn set_pls(value: u8) { ::write(REGISTER_ADDRESS, PLS_BIT_OFFSET, PLS_BIT_WIDTH, value as u32); }

	const DBP_BIT_OFFSET: u8 = 8;
	const DBP_BIT_WIDTH: u8 = 1;
	/// Disable backup domain write protection (Width: 1, Offset: 8)
	pub fn get_dbp() -> u8 { ::read(REGISTER_ADDRESS, DBP_BIT_OFFSET, DBP_BIT_WIDTH) as u8 }
	/// Disable backup domain write protection (Width: 1, Offset: 8)
	pub fn set_dbp(value: u8) { ::write(REGISTER_ADDRESS, DBP_BIT_OFFSET, DBP_BIT_WIDTH, value as u32); }
}
/// power control/status register
/// Size: 0x20 bits
pub mod csr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const WUF_BIT_OFFSET: u8 = 0;
	const WUF_BIT_WIDTH: u8 = 1;
	/// Wakeup flag (Width: 1, Offset: 0)
	pub fn get_wuf() -> u8 { ::read(REGISTER_ADDRESS, WUF_BIT_OFFSET, WUF_BIT_WIDTH) as u8 }

	const SBF_BIT_OFFSET: u8 = 1;
	const SBF_BIT_WIDTH: u8 = 1;
	/// Standby flag (Width: 1, Offset: 1)
	pub fn get_sbf() -> u8 { ::read(REGISTER_ADDRESS, SBF_BIT_OFFSET, SBF_BIT_WIDTH) as u8 }

	const PVDO_BIT_OFFSET: u8 = 2;
	const PVDO_BIT_WIDTH: u8 = 1;
	/// PVD output (Width: 1, Offset: 2)
	pub fn get_pvdo() -> u8 { ::read(REGISTER_ADDRESS, PVDO_BIT_OFFSET, PVDO_BIT_WIDTH) as u8 }

	const EWUP1_BIT_OFFSET: u8 = 8;
	const EWUP1_BIT_WIDTH: u8 = 1;
	/// Enable WKUP1 pin (Width: 1, Offset: 8)
	pub fn get_ewup1() -> u8 { ::read(REGISTER_ADDRESS, EWUP1_BIT_OFFSET, EWUP1_BIT_WIDTH) as u8 }
	/// Enable WKUP1 pin (Width: 1, Offset: 8)
	pub fn set_ewup1(value: u8) { ::write(REGISTER_ADDRESS, EWUP1_BIT_OFFSET, EWUP1_BIT_WIDTH, value as u32); }

	const EWUP2_BIT_OFFSET: u8 = 9;
	const EWUP2_BIT_WIDTH: u8 = 1;
	/// Enable WKUP2 pin (Width: 1, Offset: 9)
	pub fn get_ewup2() -> u8 { ::read(REGISTER_ADDRESS, EWUP2_BIT_OFFSET, EWUP2_BIT_WIDTH) as u8 }
	/// Enable WKUP2 pin (Width: 1, Offset: 9)
	pub fn set_ewup2(value: u8) { ::write(REGISTER_ADDRESS, EWUP2_BIT_OFFSET, EWUP2_BIT_WIDTH, value as u32); }
}
/// PVD through EXTI line detection interrupt
pub const INTERRUPT_PVD: u32 = 1;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>PWR</name>
  <description>Power control</description>
  <groupName>PWR</groupName>
  <baseAddress>0x40007000</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>CR</name>
      <displayName>CR</displayName>
      <description>power control register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>LPDS</name>
          <description>Low-power deep sleep</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PDDS</name>
          <description>Power down deepsleep</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CWUF</name>
          <description>Clear wakeup flag</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CSBF</name>
          <description>Clear standby flag</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PVDE</name>
          <description>Power voltage detector
              enable</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PLS</name>
          <description>PVD level selection</description>
          <bitOffset>5</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>DBP</name>
          <description>Disable backup domain write
              protection</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CSR</name>
      <displayName>CSR</displayName>
      <description>power control/status register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>WUF</name>
          <description>Wakeup flag</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>SBF</name>
          <description>Standby flag</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>PVDO</name>
          <description>PVD output</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>EWUP1</name>
          <description>Enable WKUP1 pin</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EWUP2</name>
          <description>Enable WKUP2 pin</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>PVD</name>
    <description>PVD through EXTI line detection
        interrupt</description>
    <value>1</value>
  </interrupt>
</peripheral>*/
