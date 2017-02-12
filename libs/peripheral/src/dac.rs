/// MOD DAC
/// Digital-to-analog converter
const BASE_ADDRESS: u32 = 0x40007400;
/// control register
/// Size: 0x20 bits
pub mod cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DMAUDRIE2_BIT_OFFSET: u8 = 29;
	const DMAUDRIE2_BIT_WIDTH: u8 = 1;
	/// DAC channel2 DMA underrun interrupt enable (Width: 1, Offset: 29)
	pub fn get_dmaudrie2() -> u8 { ::read(REGISTER_ADDRESS, DMAUDRIE2_BIT_OFFSET, DMAUDRIE2_BIT_WIDTH) as u8 }
	/// DAC channel2 DMA underrun interrupt enable (Width: 1, Offset: 29)
	pub fn set_dmaudrie2(value: u8) { ::write(REGISTER_ADDRESS, DMAUDRIE2_BIT_OFFSET, DMAUDRIE2_BIT_WIDTH, value as u32); }

	const DMAEN2_BIT_OFFSET: u8 = 28;
	const DMAEN2_BIT_WIDTH: u8 = 1;
	/// DAC channel2 DMA enable (Width: 1, Offset: 28)
	pub fn get_dmaen2() -> u8 { ::read(REGISTER_ADDRESS, DMAEN2_BIT_OFFSET, DMAEN2_BIT_WIDTH) as u8 }
	/// DAC channel2 DMA enable (Width: 1, Offset: 28)
	pub fn set_dmaen2(value: u8) { ::write(REGISTER_ADDRESS, DMAEN2_BIT_OFFSET, DMAEN2_BIT_WIDTH, value as u32); }

	const MAMP2_BIT_OFFSET: u8 = 24;
	const MAMP2_BIT_WIDTH: u8 = 4;
	/// DAC channel2 mask/amplitude selector (Width: 4, Offset: 24)
	pub fn get_mamp2() -> u8 { ::read(REGISTER_ADDRESS, MAMP2_BIT_OFFSET, MAMP2_BIT_WIDTH) as u8 }
	/// DAC channel2 mask/amplitude selector (Width: 4, Offset: 24)
	pub fn set_mamp2(value: u8) { ::write(REGISTER_ADDRESS, MAMP2_BIT_OFFSET, MAMP2_BIT_WIDTH, value as u32); }

	const WAVE2_BIT_OFFSET: u8 = 22;
	const WAVE2_BIT_WIDTH: u8 = 2;
	/// DAC channel2 noise/triangle wave generation enable (Width: 2, Offset: 22)
	pub fn get_wave2() -> u8 { ::read(REGISTER_ADDRESS, WAVE2_BIT_OFFSET, WAVE2_BIT_WIDTH) as u8 }
	/// DAC channel2 noise/triangle wave generation enable (Width: 2, Offset: 22)
	pub fn set_wave2(value: u8) { ::write(REGISTER_ADDRESS, WAVE2_BIT_OFFSET, WAVE2_BIT_WIDTH, value as u32); }

	const TSEL2_BIT_OFFSET: u8 = 19;
	const TSEL2_BIT_WIDTH: u8 = 3;
	/// DAC channel2 trigger selection (Width: 3, Offset: 19)
	pub fn get_tsel2() -> u8 { ::read(REGISTER_ADDRESS, TSEL2_BIT_OFFSET, TSEL2_BIT_WIDTH) as u8 }
	/// DAC channel2 trigger selection (Width: 3, Offset: 19)
	pub fn set_tsel2(value: u8) { ::write(REGISTER_ADDRESS, TSEL2_BIT_OFFSET, TSEL2_BIT_WIDTH, value as u32); }

	const TEN2_BIT_OFFSET: u8 = 18;
	const TEN2_BIT_WIDTH: u8 = 1;
	/// DAC channel2 trigger enable (Width: 1, Offset: 18)
	pub fn get_ten2() -> u8 { ::read(REGISTER_ADDRESS, TEN2_BIT_OFFSET, TEN2_BIT_WIDTH) as u8 }
	/// DAC channel2 trigger enable (Width: 1, Offset: 18)
	pub fn set_ten2(value: u8) { ::write(REGISTER_ADDRESS, TEN2_BIT_OFFSET, TEN2_BIT_WIDTH, value as u32); }

	const BOFF2_BIT_OFFSET: u8 = 17;
	const BOFF2_BIT_WIDTH: u8 = 1;
	/// DAC channel2 output buffer disable (Width: 1, Offset: 17)
	pub fn get_boff2() -> u8 { ::read(REGISTER_ADDRESS, BOFF2_BIT_OFFSET, BOFF2_BIT_WIDTH) as u8 }
	/// DAC channel2 output buffer disable (Width: 1, Offset: 17)
	pub fn set_boff2(value: u8) { ::write(REGISTER_ADDRESS, BOFF2_BIT_OFFSET, BOFF2_BIT_WIDTH, value as u32); }

	const EN2_BIT_OFFSET: u8 = 16;
	const EN2_BIT_WIDTH: u8 = 1;
	/// DAC channel2 enable (Width: 1, Offset: 16)
	pub fn get_en2() -> u8 { ::read(REGISTER_ADDRESS, EN2_BIT_OFFSET, EN2_BIT_WIDTH) as u8 }
	/// DAC channel2 enable (Width: 1, Offset: 16)
	pub fn set_en2(value: u8) { ::write(REGISTER_ADDRESS, EN2_BIT_OFFSET, EN2_BIT_WIDTH, value as u32); }

	const DMAUDRIE1_BIT_OFFSET: u8 = 13;
	const DMAUDRIE1_BIT_WIDTH: u8 = 1;
	/// DAC channel1 DMA Underrun Interrupt enable (Width: 1, Offset: 13)
	pub fn get_dmaudrie1() -> u8 { ::read(REGISTER_ADDRESS, DMAUDRIE1_BIT_OFFSET, DMAUDRIE1_BIT_WIDTH) as u8 }
	/// DAC channel1 DMA Underrun Interrupt enable (Width: 1, Offset: 13)
	pub fn set_dmaudrie1(value: u8) { ::write(REGISTER_ADDRESS, DMAUDRIE1_BIT_OFFSET, DMAUDRIE1_BIT_WIDTH, value as u32); }

	const DMAEN1_BIT_OFFSET: u8 = 12;
	const DMAEN1_BIT_WIDTH: u8 = 1;
	/// DAC channel1 DMA enable (Width: 1, Offset: 12)
	pub fn get_dmaen1() -> u8 { ::read(REGISTER_ADDRESS, DMAEN1_BIT_OFFSET, DMAEN1_BIT_WIDTH) as u8 }
	/// DAC channel1 DMA enable (Width: 1, Offset: 12)
	pub fn set_dmaen1(value: u8) { ::write(REGISTER_ADDRESS, DMAEN1_BIT_OFFSET, DMAEN1_BIT_WIDTH, value as u32); }

	const MAMP1_BIT_OFFSET: u8 = 8;
	const MAMP1_BIT_WIDTH: u8 = 4;
	/// DAC channel1 mask/amplitude selector (Width: 4, Offset: 8)
	pub fn get_mamp1() -> u8 { ::read(REGISTER_ADDRESS, MAMP1_BIT_OFFSET, MAMP1_BIT_WIDTH) as u8 }
	/// DAC channel1 mask/amplitude selector (Width: 4, Offset: 8)
	pub fn set_mamp1(value: u8) { ::write(REGISTER_ADDRESS, MAMP1_BIT_OFFSET, MAMP1_BIT_WIDTH, value as u32); }

	const WAVE1_BIT_OFFSET: u8 = 6;
	const WAVE1_BIT_WIDTH: u8 = 2;
	/// DAC channel1 noise/triangle wave generation enable (Width: 2, Offset: 6)
	pub fn get_wave1() -> u8 { ::read(REGISTER_ADDRESS, WAVE1_BIT_OFFSET, WAVE1_BIT_WIDTH) as u8 }
	/// DAC channel1 noise/triangle wave generation enable (Width: 2, Offset: 6)
	pub fn set_wave1(value: u8) { ::write(REGISTER_ADDRESS, WAVE1_BIT_OFFSET, WAVE1_BIT_WIDTH, value as u32); }

	const TSEL1_BIT_OFFSET: u8 = 3;
	const TSEL1_BIT_WIDTH: u8 = 3;
	/// DAC channel1 trigger selection (Width: 3, Offset: 3)
	pub fn get_tsel1() -> u8 { ::read(REGISTER_ADDRESS, TSEL1_BIT_OFFSET, TSEL1_BIT_WIDTH) as u8 }
	/// DAC channel1 trigger selection (Width: 3, Offset: 3)
	pub fn set_tsel1(value: u8) { ::write(REGISTER_ADDRESS, TSEL1_BIT_OFFSET, TSEL1_BIT_WIDTH, value as u32); }

	const TEN1_BIT_OFFSET: u8 = 2;
	const TEN1_BIT_WIDTH: u8 = 1;
	/// DAC channel1 trigger enable (Width: 1, Offset: 2)
	pub fn get_ten1() -> u8 { ::read(REGISTER_ADDRESS, TEN1_BIT_OFFSET, TEN1_BIT_WIDTH) as u8 }
	/// DAC channel1 trigger enable (Width: 1, Offset: 2)
	pub fn set_ten1(value: u8) { ::write(REGISTER_ADDRESS, TEN1_BIT_OFFSET, TEN1_BIT_WIDTH, value as u32); }

	const BOFF1_BIT_OFFSET: u8 = 1;
	const BOFF1_BIT_WIDTH: u8 = 1;
	/// DAC channel1 output buffer disable (Width: 1, Offset: 1)
	pub fn get_boff1() -> u8 { ::read(REGISTER_ADDRESS, BOFF1_BIT_OFFSET, BOFF1_BIT_WIDTH) as u8 }
	/// DAC channel1 output buffer disable (Width: 1, Offset: 1)
	pub fn set_boff1(value: u8) { ::write(REGISTER_ADDRESS, BOFF1_BIT_OFFSET, BOFF1_BIT_WIDTH, value as u32); }

	const EN1_BIT_OFFSET: u8 = 0;
	const EN1_BIT_WIDTH: u8 = 1;
	/// DAC channel1 enable (Width: 1, Offset: 0)
	pub fn get_en1() -> u8 { ::read(REGISTER_ADDRESS, EN1_BIT_OFFSET, EN1_BIT_WIDTH) as u8 }
	/// DAC channel1 enable (Width: 1, Offset: 0)
	pub fn set_en1(value: u8) { ::write(REGISTER_ADDRESS, EN1_BIT_OFFSET, EN1_BIT_WIDTH, value as u32); }
}
/// software trigger register
/// Size: 0x20 bits
pub mod swtrigr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SWTRIG2_BIT_OFFSET: u8 = 1;
	const SWTRIG2_BIT_WIDTH: u8 = 1;
	/// DAC channel2 software trigger (Width: 1, Offset: 1)
	pub fn set_swtrig2(value: u8) { ::write(REGISTER_ADDRESS, SWTRIG2_BIT_OFFSET, SWTRIG2_BIT_WIDTH, value as u32); }

	const SWTRIG1_BIT_OFFSET: u8 = 0;
	const SWTRIG1_BIT_WIDTH: u8 = 1;
	/// DAC channel1 software trigger (Width: 1, Offset: 0)
	pub fn set_swtrig1(value: u8) { ::write(REGISTER_ADDRESS, SWTRIG1_BIT_OFFSET, SWTRIG1_BIT_WIDTH, value as u32); }
}
/// channel1 12-bit right-aligned data holding register
/// Size: 0x20 bits
pub mod dhr12r1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DACC1DHR_BIT_OFFSET: u8 = 0;
	const DACC1DHR_BIT_WIDTH: u8 = 12;
	/// DAC channel1 12-bit right-aligned data (Width: 12, Offset: 0)
	pub fn get_dacc1dhr() -> u16 { ::read(REGISTER_ADDRESS, DACC1DHR_BIT_OFFSET, DACC1DHR_BIT_WIDTH) as u16 }
	/// DAC channel1 12-bit right-aligned data (Width: 12, Offset: 0)
	pub fn set_dacc1dhr(value: u16) { ::write(REGISTER_ADDRESS, DACC1DHR_BIT_OFFSET, DACC1DHR_BIT_WIDTH, value as u32); }
}
/// channel1 12-bit left aligned data holding register
/// Size: 0x20 bits
pub mod dhr12l1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DACC1DHR_BIT_OFFSET: u8 = 4;
	const DACC1DHR_BIT_WIDTH: u8 = 12;
	/// DAC channel1 12-bit left-aligned data (Width: 12, Offset: 4)
	pub fn get_dacc1dhr() -> u16 { ::read(REGISTER_ADDRESS, DACC1DHR_BIT_OFFSET, DACC1DHR_BIT_WIDTH) as u16 }
	/// DAC channel1 12-bit left-aligned data (Width: 12, Offset: 4)
	pub fn set_dacc1dhr(value: u16) { ::write(REGISTER_ADDRESS, DACC1DHR_BIT_OFFSET, DACC1DHR_BIT_WIDTH, value as u32); }
}
/// channel1 8-bit right aligned data holding register
/// Size: 0x20 bits
pub mod dhr8r1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DACC1DHR_BIT_OFFSET: u8 = 0;
	const DACC1DHR_BIT_WIDTH: u8 = 8;
	/// DAC channel1 8-bit right-aligned data (Width: 8, Offset: 0)
	pub fn get_dacc1dhr() -> u8 { ::read(REGISTER_ADDRESS, DACC1DHR_BIT_OFFSET, DACC1DHR_BIT_WIDTH) as u8 }
	/// DAC channel1 8-bit right-aligned data (Width: 8, Offset: 0)
	pub fn set_dacc1dhr(value: u8) { ::write(REGISTER_ADDRESS, DACC1DHR_BIT_OFFSET, DACC1DHR_BIT_WIDTH, value as u32); }
}
/// channel2 12-bit right aligned data holding register
/// Size: 0x20 bits
pub mod dhr12r2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DACC2DHR_BIT_OFFSET: u8 = 0;
	const DACC2DHR_BIT_WIDTH: u8 = 12;
	/// DAC channel2 12-bit right-aligned data (Width: 12, Offset: 0)
	pub fn get_dacc2dhr() -> u16 { ::read(REGISTER_ADDRESS, DACC2DHR_BIT_OFFSET, DACC2DHR_BIT_WIDTH) as u16 }
	/// DAC channel2 12-bit right-aligned data (Width: 12, Offset: 0)
	pub fn set_dacc2dhr(value: u16) { ::write(REGISTER_ADDRESS, DACC2DHR_BIT_OFFSET, DACC2DHR_BIT_WIDTH, value as u32); }
}
/// channel2 12-bit left aligned data holding register
/// Size: 0x20 bits
pub mod dhr12l2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DACC2DHR_BIT_OFFSET: u8 = 4;
	const DACC2DHR_BIT_WIDTH: u8 = 12;
	/// DAC channel2 12-bit left-aligned data (Width: 12, Offset: 4)
	pub fn get_dacc2dhr() -> u16 { ::read(REGISTER_ADDRESS, DACC2DHR_BIT_OFFSET, DACC2DHR_BIT_WIDTH) as u16 }
	/// DAC channel2 12-bit left-aligned data (Width: 12, Offset: 4)
	pub fn set_dacc2dhr(value: u16) { ::write(REGISTER_ADDRESS, DACC2DHR_BIT_OFFSET, DACC2DHR_BIT_WIDTH, value as u32); }
}
/// channel2 8-bit right-aligned data holding register
/// Size: 0x20 bits
pub mod dhr8r2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DACC2DHR_BIT_OFFSET: u8 = 0;
	const DACC2DHR_BIT_WIDTH: u8 = 8;
	/// DAC channel2 8-bit right-aligned data (Width: 8, Offset: 0)
	pub fn get_dacc2dhr() -> u8 { ::read(REGISTER_ADDRESS, DACC2DHR_BIT_OFFSET, DACC2DHR_BIT_WIDTH) as u8 }
	/// DAC channel2 8-bit right-aligned data (Width: 8, Offset: 0)
	pub fn set_dacc2dhr(value: u8) { ::write(REGISTER_ADDRESS, DACC2DHR_BIT_OFFSET, DACC2DHR_BIT_WIDTH, value as u32); }
}
/// Dual DAC 12-bit right-aligned data holding register
/// Size: 0x20 bits
pub mod dhr12rd {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x20;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DACC2DHR_BIT_OFFSET: u8 = 16;
	const DACC2DHR_BIT_WIDTH: u8 = 12;
	/// DAC channel2 12-bit right-aligned data (Width: 12, Offset: 16)
	pub fn get_dacc2dhr() -> u16 { ::read(REGISTER_ADDRESS, DACC2DHR_BIT_OFFSET, DACC2DHR_BIT_WIDTH) as u16 }
	/// DAC channel2 12-bit right-aligned data (Width: 12, Offset: 16)
	pub fn set_dacc2dhr(value: u16) { ::write(REGISTER_ADDRESS, DACC2DHR_BIT_OFFSET, DACC2DHR_BIT_WIDTH, value as u32); }

	const DACC1DHR_BIT_OFFSET: u8 = 0;
	const DACC1DHR_BIT_WIDTH: u8 = 12;
	/// DAC channel1 12-bit right-aligned data (Width: 12, Offset: 0)
	pub fn get_dacc1dhr() -> u16 { ::read(REGISTER_ADDRESS, DACC1DHR_BIT_OFFSET, DACC1DHR_BIT_WIDTH) as u16 }
	/// DAC channel1 12-bit right-aligned data (Width: 12, Offset: 0)
	pub fn set_dacc1dhr(value: u16) { ::write(REGISTER_ADDRESS, DACC1DHR_BIT_OFFSET, DACC1DHR_BIT_WIDTH, value as u32); }
}
/// DUAL DAC 12-bit left aligned data holding register
/// Size: 0x20 bits
pub mod dhr12ld {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x24;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DACC2DHR_BIT_OFFSET: u8 = 20;
	const DACC2DHR_BIT_WIDTH: u8 = 12;
	/// DAC channel2 12-bit left-aligned data (Width: 12, Offset: 20)
	pub fn get_dacc2dhr() -> u16 { ::read(REGISTER_ADDRESS, DACC2DHR_BIT_OFFSET, DACC2DHR_BIT_WIDTH) as u16 }
	/// DAC channel2 12-bit left-aligned data (Width: 12, Offset: 20)
	pub fn set_dacc2dhr(value: u16) { ::write(REGISTER_ADDRESS, DACC2DHR_BIT_OFFSET, DACC2DHR_BIT_WIDTH, value as u32); }

	const DACC1DHR_BIT_OFFSET: u8 = 4;
	const DACC1DHR_BIT_WIDTH: u8 = 12;
	/// DAC channel1 12-bit left-aligned data (Width: 12, Offset: 4)
	pub fn get_dacc1dhr() -> u16 { ::read(REGISTER_ADDRESS, DACC1DHR_BIT_OFFSET, DACC1DHR_BIT_WIDTH) as u16 }
	/// DAC channel1 12-bit left-aligned data (Width: 12, Offset: 4)
	pub fn set_dacc1dhr(value: u16) { ::write(REGISTER_ADDRESS, DACC1DHR_BIT_OFFSET, DACC1DHR_BIT_WIDTH, value as u32); }
}
/// DUAL DAC 8-bit right aligned data holding register
/// Size: 0x20 bits
pub mod dhr8rd {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x28;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DACC2DHR_BIT_OFFSET: u8 = 8;
	const DACC2DHR_BIT_WIDTH: u8 = 8;
	/// DAC channel2 8-bit right-aligned data (Width: 8, Offset: 8)
	pub fn get_dacc2dhr() -> u8 { ::read(REGISTER_ADDRESS, DACC2DHR_BIT_OFFSET, DACC2DHR_BIT_WIDTH) as u8 }
	/// DAC channel2 8-bit right-aligned data (Width: 8, Offset: 8)
	pub fn set_dacc2dhr(value: u8) { ::write(REGISTER_ADDRESS, DACC2DHR_BIT_OFFSET, DACC2DHR_BIT_WIDTH, value as u32); }

	const DACC1DHR_BIT_OFFSET: u8 = 0;
	const DACC1DHR_BIT_WIDTH: u8 = 8;
	/// DAC channel1 8-bit right-aligned data (Width: 8, Offset: 0)
	pub fn get_dacc1dhr() -> u8 { ::read(REGISTER_ADDRESS, DACC1DHR_BIT_OFFSET, DACC1DHR_BIT_WIDTH) as u8 }
	/// DAC channel1 8-bit right-aligned data (Width: 8, Offset: 0)
	pub fn set_dacc1dhr(value: u8) { ::write(REGISTER_ADDRESS, DACC1DHR_BIT_OFFSET, DACC1DHR_BIT_WIDTH, value as u32); }
}
/// channel1 data output register
/// Size: 0x20 bits
pub mod dor1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x2C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DACC1DOR_BIT_OFFSET: u8 = 0;
	const DACC1DOR_BIT_WIDTH: u8 = 12;
	/// DAC channel1 data output (Width: 12, Offset: 0)
	pub fn get_dacc1dor() -> u16 { ::read(REGISTER_ADDRESS, DACC1DOR_BIT_OFFSET, DACC1DOR_BIT_WIDTH) as u16 }
}
/// channel2 data output register
/// Size: 0x20 bits
pub mod dor2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x30;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DACC2DOR_BIT_OFFSET: u8 = 0;
	const DACC2DOR_BIT_WIDTH: u8 = 12;
	/// DAC channel2 data output (Width: 12, Offset: 0)
	pub fn get_dacc2dor() -> u16 { ::read(REGISTER_ADDRESS, DACC2DOR_BIT_OFFSET, DACC2DOR_BIT_WIDTH) as u16 }
}
/// status register
/// Size: 0x20 bits
pub mod sr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x34;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DMAUDR2_BIT_OFFSET: u8 = 29;
	const DMAUDR2_BIT_WIDTH: u8 = 1;
	/// DAC channel2 DMA underrun flag (Width: 1, Offset: 29)
	pub fn get_dmaudr2() -> u8 { ::read(REGISTER_ADDRESS, DMAUDR2_BIT_OFFSET, DMAUDR2_BIT_WIDTH) as u8 }
	/// DAC channel2 DMA underrun flag (Width: 1, Offset: 29)
	pub fn set_dmaudr2(value: u8) { ::write(REGISTER_ADDRESS, DMAUDR2_BIT_OFFSET, DMAUDR2_BIT_WIDTH, value as u32); }

	const DMAUDR1_BIT_OFFSET: u8 = 13;
	const DMAUDR1_BIT_WIDTH: u8 = 1;
	/// DAC channel1 DMA underrun flag (Width: 1, Offset: 13)
	pub fn get_dmaudr1() -> u8 { ::read(REGISTER_ADDRESS, DMAUDR1_BIT_OFFSET, DMAUDR1_BIT_WIDTH) as u8 }
	/// DAC channel1 DMA underrun flag (Width: 1, Offset: 13)
	pub fn set_dmaudr1(value: u8) { ::write(REGISTER_ADDRESS, DMAUDR1_BIT_OFFSET, DMAUDR1_BIT_WIDTH, value as u32); }
}
/// TIM6 global and DAC12 underrun interrupts
pub const INTERRUPT_TIM6_DACUNDER: u32 = 54;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>DAC</name>
  <description>Digital-to-analog converter</description>
  <groupName>DAC</groupName>
  <baseAddress>0x40007400</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>CR</name>
      <displayName>CR</displayName>
      <description>control register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DMAUDRIE2</name>
          <description>DAC channel2 DMA underrun interrupt
              enable</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DMAEN2</name>
          <description>DAC channel2 DMA enable</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MAMP2</name>
          <description>DAC channel2 mask/amplitude
              selector</description>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>WAVE2</name>
          <description>DAC channel2 noise/triangle wave
              generation enable</description>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>TSEL2</name>
          <description>DAC channel2 trigger
              selection</description>
          <bitOffset>19</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>TEN2</name>
          <description>DAC channel2 trigger
              enable</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BOFF2</name>
          <description>DAC channel2 output buffer
              disable</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EN2</name>
          <description>DAC channel2 enable</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DMAUDRIE1</name>
          <description>DAC channel1 DMA Underrun Interrupt
              enable</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DMAEN1</name>
          <description>DAC channel1 DMA enable</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MAMP1</name>
          <description>DAC channel1 mask/amplitude
              selector</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>WAVE1</name>
          <description>DAC channel1 noise/triangle wave
              generation enable</description>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>TSEL1</name>
          <description>DAC channel1 trigger
              selection</description>
          <bitOffset>3</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>TEN1</name>
          <description>DAC channel1 trigger
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BOFF1</name>
          <description>DAC channel1 output buffer
              disable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EN1</name>
          <description>DAC channel1 enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SWTRIGR</name>
      <displayName>SWTRIGR</displayName>
      <description>software trigger register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>write-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SWTRIG2</name>
          <description>DAC channel2 software
              trigger</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWTRIG1</name>
          <description>DAC channel1 software
              trigger</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DHR12R1</name>
      <displayName>DHR12R1</displayName>
      <description>channel1 12-bit right-aligned data holding
          register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DACC1DHR</name>
          <description>DAC channel1 12-bit right-aligned
              data</description>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DHR12L1</name>
      <displayName>DHR12L1</displayName>
      <description>channel1 12-bit left aligned data holding
          register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DACC1DHR</name>
          <description>DAC channel1 12-bit left-aligned
              data</description>
          <bitOffset>4</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DHR8R1</name>
      <displayName>DHR8R1</displayName>
      <description>channel1 8-bit right aligned data holding
          register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DACC1DHR</name>
          <description>DAC channel1 8-bit right-aligned
              data</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DHR12R2</name>
      <displayName>DHR12R2</displayName>
      <description>channel2 12-bit right aligned data holding
          register</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DACC2DHR</name>
          <description>DAC channel2 12-bit right-aligned
              data</description>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DHR12L2</name>
      <displayName>DHR12L2</displayName>
      <description>channel2 12-bit left aligned data holding
          register</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DACC2DHR</name>
          <description>DAC channel2 12-bit left-aligned
              data</description>
          <bitOffset>4</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DHR8R2</name>
      <displayName>DHR8R2</displayName>
      <description>channel2 8-bit right-aligned data holding
          register</description>
      <addressOffset>0x1C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DACC2DHR</name>
          <description>DAC channel2 8-bit right-aligned
              data</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DHR12RD</name>
      <displayName>DHR12RD</displayName>
      <description>Dual DAC 12-bit right-aligned data holding
          register</description>
      <addressOffset>0x20</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DACC2DHR</name>
          <description>DAC channel2 12-bit right-aligned
              data</description>
          <bitOffset>16</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
        <field>
          <name>DACC1DHR</name>
          <description>DAC channel1 12-bit right-aligned
              data</description>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DHR12LD</name>
      <displayName>DHR12LD</displayName>
      <description>DUAL DAC 12-bit left aligned data holding
          register</description>
      <addressOffset>0x24</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DACC2DHR</name>
          <description>DAC channel2 12-bit left-aligned
              data</description>
          <bitOffset>20</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
        <field>
          <name>DACC1DHR</name>
          <description>DAC channel1 12-bit left-aligned
              data</description>
          <bitOffset>4</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DHR8RD</name>
      <displayName>DHR8RD</displayName>
      <description>DUAL DAC 8-bit right aligned data holding
          register</description>
      <addressOffset>0x28</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DACC2DHR</name>
          <description>DAC channel2 8-bit right-aligned
              data</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>DACC1DHR</name>
          <description>DAC channel1 8-bit right-aligned
              data</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DOR1</name>
      <displayName>DOR1</displayName>
      <description>channel1 data output register</description>
      <addressOffset>0x2C</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DACC1DOR</name>
          <description>DAC channel1 data output</description>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DOR2</name>
      <displayName>DOR2</displayName>
      <description>channel2 data output register</description>
      <addressOffset>0x30</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DACC2DOR</name>
          <description>DAC channel2 data output</description>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SR</name>
      <displayName>SR</displayName>
      <description>status register</description>
      <addressOffset>0x34</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DMAUDR2</name>
          <description>DAC channel2 DMA underrun
              flag</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DMAUDR1</name>
          <description>DAC channel1 DMA underrun
              flag</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
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
