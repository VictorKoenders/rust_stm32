/// MOD ADC1_2
/// Analog-to-Digital Converter
const BASE_ADDRESS: u32 = 0x50000300;
/// ADC Common status register
/// Size: 0x20 bits
pub mod csr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ADDRDY_MST_BIT_OFFSET: u8 = 0;
	const ADDRDY_MST_BIT_WIDTH: u8 = 1;
	/// ADDRDY_MST (Width: 1, Offset: 0)
	pub fn get_addrdy_mst() -> u8 { ::read(REGISTER_ADDRESS, ADDRDY_MST_BIT_OFFSET, ADDRDY_MST_BIT_WIDTH) as u8 }

	const EOSMP_MST_BIT_OFFSET: u8 = 1;
	const EOSMP_MST_BIT_WIDTH: u8 = 1;
	/// EOSMP_MST (Width: 1, Offset: 1)
	pub fn get_eosmp_mst() -> u8 { ::read(REGISTER_ADDRESS, EOSMP_MST_BIT_OFFSET, EOSMP_MST_BIT_WIDTH) as u8 }

	const EOC_MST_BIT_OFFSET: u8 = 2;
	const EOC_MST_BIT_WIDTH: u8 = 1;
	/// EOC_MST (Width: 1, Offset: 2)
	pub fn get_eoc_mst() -> u8 { ::read(REGISTER_ADDRESS, EOC_MST_BIT_OFFSET, EOC_MST_BIT_WIDTH) as u8 }

	const EOS_MST_BIT_OFFSET: u8 = 3;
	const EOS_MST_BIT_WIDTH: u8 = 1;
	/// EOS_MST (Width: 1, Offset: 3)
	pub fn get_eos_mst() -> u8 { ::read(REGISTER_ADDRESS, EOS_MST_BIT_OFFSET, EOS_MST_BIT_WIDTH) as u8 }

	const OVR_MST_BIT_OFFSET: u8 = 4;
	const OVR_MST_BIT_WIDTH: u8 = 1;
	/// OVR_MST (Width: 1, Offset: 4)
	pub fn get_ovr_mst() -> u8 { ::read(REGISTER_ADDRESS, OVR_MST_BIT_OFFSET, OVR_MST_BIT_WIDTH) as u8 }

	const JEOC_MST_BIT_OFFSET: u8 = 5;
	const JEOC_MST_BIT_WIDTH: u8 = 1;
	/// JEOC_MST (Width: 1, Offset: 5)
	pub fn get_jeoc_mst() -> u8 { ::read(REGISTER_ADDRESS, JEOC_MST_BIT_OFFSET, JEOC_MST_BIT_WIDTH) as u8 }

	const JEOS_MST_BIT_OFFSET: u8 = 6;
	const JEOS_MST_BIT_WIDTH: u8 = 1;
	/// JEOS_MST (Width: 1, Offset: 6)
	pub fn get_jeos_mst() -> u8 { ::read(REGISTER_ADDRESS, JEOS_MST_BIT_OFFSET, JEOS_MST_BIT_WIDTH) as u8 }

	const AWD1_MST_BIT_OFFSET: u8 = 7;
	const AWD1_MST_BIT_WIDTH: u8 = 1;
	/// AWD1_MST (Width: 1, Offset: 7)
	pub fn get_awd1_mst() -> u8 { ::read(REGISTER_ADDRESS, AWD1_MST_BIT_OFFSET, AWD1_MST_BIT_WIDTH) as u8 }

	const AWD2_MST_BIT_OFFSET: u8 = 8;
	const AWD2_MST_BIT_WIDTH: u8 = 1;
	/// AWD2_MST (Width: 1, Offset: 8)
	pub fn get_awd2_mst() -> u8 { ::read(REGISTER_ADDRESS, AWD2_MST_BIT_OFFSET, AWD2_MST_BIT_WIDTH) as u8 }

	const AWD3_MST_BIT_OFFSET: u8 = 9;
	const AWD3_MST_BIT_WIDTH: u8 = 1;
	/// AWD3_MST (Width: 1, Offset: 9)
	pub fn get_awd3_mst() -> u8 { ::read(REGISTER_ADDRESS, AWD3_MST_BIT_OFFSET, AWD3_MST_BIT_WIDTH) as u8 }

	const JQOVF_MST_BIT_OFFSET: u8 = 10;
	const JQOVF_MST_BIT_WIDTH: u8 = 1;
	/// JQOVF_MST (Width: 1, Offset: 10)
	pub fn get_jqovf_mst() -> u8 { ::read(REGISTER_ADDRESS, JQOVF_MST_BIT_OFFSET, JQOVF_MST_BIT_WIDTH) as u8 }

	const ADRDY_SLV_BIT_OFFSET: u8 = 16;
	const ADRDY_SLV_BIT_WIDTH: u8 = 1;
	/// ADRDY_SLV (Width: 1, Offset: 16)
	pub fn get_adrdy_slv() -> u8 { ::read(REGISTER_ADDRESS, ADRDY_SLV_BIT_OFFSET, ADRDY_SLV_BIT_WIDTH) as u8 }

	const EOSMP_SLV_BIT_OFFSET: u8 = 17;
	const EOSMP_SLV_BIT_WIDTH: u8 = 1;
	/// EOSMP_SLV (Width: 1, Offset: 17)
	pub fn get_eosmp_slv() -> u8 { ::read(REGISTER_ADDRESS, EOSMP_SLV_BIT_OFFSET, EOSMP_SLV_BIT_WIDTH) as u8 }

	const EOC_SLV_BIT_OFFSET: u8 = 18;
	const EOC_SLV_BIT_WIDTH: u8 = 1;
	/// End of regular conversion of the slave ADC (Width: 1, Offset: 18)
	pub fn get_eoc_slv() -> u8 { ::read(REGISTER_ADDRESS, EOC_SLV_BIT_OFFSET, EOC_SLV_BIT_WIDTH) as u8 }

	const EOS_SLV_BIT_OFFSET: u8 = 19;
	const EOS_SLV_BIT_WIDTH: u8 = 1;
	/// End of regular sequence flag of the slave ADC (Width: 1, Offset: 19)
	pub fn get_eos_slv() -> u8 { ::read(REGISTER_ADDRESS, EOS_SLV_BIT_OFFSET, EOS_SLV_BIT_WIDTH) as u8 }

	const OVR_SLV_BIT_OFFSET: u8 = 20;
	const OVR_SLV_BIT_WIDTH: u8 = 1;
	/// Overrun flag of the slave ADC (Width: 1, Offset: 20)
	pub fn get_ovr_slv() -> u8 { ::read(REGISTER_ADDRESS, OVR_SLV_BIT_OFFSET, OVR_SLV_BIT_WIDTH) as u8 }

	const JEOC_SLV_BIT_OFFSET: u8 = 21;
	const JEOC_SLV_BIT_WIDTH: u8 = 1;
	/// End of injected conversion flag of the slave ADC (Width: 1, Offset: 21)
	pub fn get_jeoc_slv() -> u8 { ::read(REGISTER_ADDRESS, JEOC_SLV_BIT_OFFSET, JEOC_SLV_BIT_WIDTH) as u8 }

	const JEOS_SLV_BIT_OFFSET: u8 = 22;
	const JEOS_SLV_BIT_WIDTH: u8 = 1;
	/// End of injected sequence flag of the slave ADC (Width: 1, Offset: 22)
	pub fn get_jeos_slv() -> u8 { ::read(REGISTER_ADDRESS, JEOS_SLV_BIT_OFFSET, JEOS_SLV_BIT_WIDTH) as u8 }

	const AWD1_SLV_BIT_OFFSET: u8 = 23;
	const AWD1_SLV_BIT_WIDTH: u8 = 1;
	/// Analog watchdog 1 flag of the slave ADC (Width: 1, Offset: 23)
	pub fn get_awd1_slv() -> u8 { ::read(REGISTER_ADDRESS, AWD1_SLV_BIT_OFFSET, AWD1_SLV_BIT_WIDTH) as u8 }

	const AWD2_SLV_BIT_OFFSET: u8 = 24;
	const AWD2_SLV_BIT_WIDTH: u8 = 1;
	/// Analog watchdog 2 flag of the slave ADC (Width: 1, Offset: 24)
	pub fn get_awd2_slv() -> u8 { ::read(REGISTER_ADDRESS, AWD2_SLV_BIT_OFFSET, AWD2_SLV_BIT_WIDTH) as u8 }

	const AWD3_SLV_BIT_OFFSET: u8 = 25;
	const AWD3_SLV_BIT_WIDTH: u8 = 1;
	/// Analog watchdog 3 flag of the slave ADC (Width: 1, Offset: 25)
	pub fn get_awd3_slv() -> u8 { ::read(REGISTER_ADDRESS, AWD3_SLV_BIT_OFFSET, AWD3_SLV_BIT_WIDTH) as u8 }

	const JQOVF_SLV_BIT_OFFSET: u8 = 26;
	const JQOVF_SLV_BIT_WIDTH: u8 = 1;
	/// Injected Context Queue Overflow flag of the slave ADC (Width: 1, Offset: 26)
	pub fn get_jqovf_slv() -> u8 { ::read(REGISTER_ADDRESS, JQOVF_SLV_BIT_OFFSET, JQOVF_SLV_BIT_WIDTH) as u8 }
}
/// ADC common control register
/// Size: 0x20 bits
pub mod ccr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MULT_BIT_OFFSET: u8 = 0;
	const MULT_BIT_WIDTH: u8 = 5;
	/// Multi ADC mode selection (Width: 5, Offset: 0)
	pub fn get_mult() -> u8 { ::read(REGISTER_ADDRESS, MULT_BIT_OFFSET, MULT_BIT_WIDTH) as u8 }
	/// Multi ADC mode selection (Width: 5, Offset: 0)
	pub fn set_mult(value: u8) { ::write(REGISTER_ADDRESS, MULT_BIT_OFFSET, MULT_BIT_WIDTH, value as u32); }

	const DELAY_BIT_OFFSET: u8 = 8;
	const DELAY_BIT_WIDTH: u8 = 4;
	/// Delay between 2 sampling phases (Width: 4, Offset: 8)
	pub fn get_delay() -> u8 { ::read(REGISTER_ADDRESS, DELAY_BIT_OFFSET, DELAY_BIT_WIDTH) as u8 }
	/// Delay between 2 sampling phases (Width: 4, Offset: 8)
	pub fn set_delay(value: u8) { ::write(REGISTER_ADDRESS, DELAY_BIT_OFFSET, DELAY_BIT_WIDTH, value as u32); }

	const DMACFG_BIT_OFFSET: u8 = 13;
	const DMACFG_BIT_WIDTH: u8 = 1;
	/// DMA configuration (for multi-ADC mode) (Width: 1, Offset: 13)
	pub fn get_dmacfg() -> u8 { ::read(REGISTER_ADDRESS, DMACFG_BIT_OFFSET, DMACFG_BIT_WIDTH) as u8 }
	/// DMA configuration (for multi-ADC mode) (Width: 1, Offset: 13)
	pub fn set_dmacfg(value: u8) { ::write(REGISTER_ADDRESS, DMACFG_BIT_OFFSET, DMACFG_BIT_WIDTH, value as u32); }

	const MDMA_BIT_OFFSET: u8 = 14;
	const MDMA_BIT_WIDTH: u8 = 2;
	/// Direct memory access mode for multi ADC mode (Width: 2, Offset: 14)
	pub fn get_mdma() -> u8 { ::read(REGISTER_ADDRESS, MDMA_BIT_OFFSET, MDMA_BIT_WIDTH) as u8 }
	/// Direct memory access mode for multi ADC mode (Width: 2, Offset: 14)
	pub fn set_mdma(value: u8) { ::write(REGISTER_ADDRESS, MDMA_BIT_OFFSET, MDMA_BIT_WIDTH, value as u32); }

	const CKMODE_BIT_OFFSET: u8 = 16;
	const CKMODE_BIT_WIDTH: u8 = 2;
	/// ADC clock mode (Width: 2, Offset: 16)
	pub fn get_ckmode() -> u8 { ::read(REGISTER_ADDRESS, CKMODE_BIT_OFFSET, CKMODE_BIT_WIDTH) as u8 }
	/// ADC clock mode (Width: 2, Offset: 16)
	pub fn set_ckmode(value: u8) { ::write(REGISTER_ADDRESS, CKMODE_BIT_OFFSET, CKMODE_BIT_WIDTH, value as u32); }

	const VREFEN_BIT_OFFSET: u8 = 22;
	const VREFEN_BIT_WIDTH: u8 = 1;
	/// VREFINT enable (Width: 1, Offset: 22)
	pub fn get_vrefen() -> u8 { ::read(REGISTER_ADDRESS, VREFEN_BIT_OFFSET, VREFEN_BIT_WIDTH) as u8 }
	/// VREFINT enable (Width: 1, Offset: 22)
	pub fn set_vrefen(value: u8) { ::write(REGISTER_ADDRESS, VREFEN_BIT_OFFSET, VREFEN_BIT_WIDTH, value as u32); }

	const TSEN_BIT_OFFSET: u8 = 23;
	const TSEN_BIT_WIDTH: u8 = 1;
	/// Temperature sensor enable (Width: 1, Offset: 23)
	pub fn get_tsen() -> u8 { ::read(REGISTER_ADDRESS, TSEN_BIT_OFFSET, TSEN_BIT_WIDTH) as u8 }
	/// Temperature sensor enable (Width: 1, Offset: 23)
	pub fn set_tsen(value: u8) { ::write(REGISTER_ADDRESS, TSEN_BIT_OFFSET, TSEN_BIT_WIDTH, value as u32); }

	const VBATEN_BIT_OFFSET: u8 = 24;
	const VBATEN_BIT_WIDTH: u8 = 1;
	/// VBAT enable (Width: 1, Offset: 24)
	pub fn get_vbaten() -> u8 { ::read(REGISTER_ADDRESS, VBATEN_BIT_OFFSET, VBATEN_BIT_WIDTH) as u8 }
	/// VBAT enable (Width: 1, Offset: 24)
	pub fn set_vbaten(value: u8) { ::write(REGISTER_ADDRESS, VBATEN_BIT_OFFSET, VBATEN_BIT_WIDTH, value as u32); }
}
/// ADC common regular data register for dual and triple modes
/// Size: 0x20 bits
pub mod cdr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const RDATA_SLV_BIT_OFFSET: u8 = 16;
	const RDATA_SLV_BIT_WIDTH: u8 = 16;
	/// Regular data of the slave ADC (Width: 16, Offset: 16)
	pub fn get_rdata_slv() -> u16 { ::read(REGISTER_ADDRESS, RDATA_SLV_BIT_OFFSET, RDATA_SLV_BIT_WIDTH) as u16 }

	const RDATA_MST_BIT_OFFSET: u8 = 0;
	const RDATA_MST_BIT_WIDTH: u8 = 16;
	/// Regular data of the master ADC (Width: 16, Offset: 0)
	pub fn get_rdata_mst() -> u16 { ::read(REGISTER_ADDRESS, RDATA_MST_BIT_OFFSET, RDATA_MST_BIT_WIDTH) as u16 }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>ADC1_2</name>
  <description>Analog-to-Digital Converter</description>
  <groupName>ADC</groupName>
  <baseAddress>0x50000300</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0xD</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>CSR</name>
      <displayName>CSR</displayName>
      <description>ADC Common status register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>ADDRDY_MST</name>
          <description>ADDRDY_MST</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOSMP_MST</name>
          <description>EOSMP_MST</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOC_MST</name>
          <description>EOC_MST</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOS_MST</name>
          <description>EOS_MST</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OVR_MST</name>
          <description>OVR_MST</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JEOC_MST</name>
          <description>JEOC_MST</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JEOS_MST</name>
          <description>JEOS_MST</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AWD1_MST</name>
          <description>AWD1_MST</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AWD2_MST</name>
          <description>AWD2_MST</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AWD3_MST</name>
          <description>AWD3_MST</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JQOVF_MST</name>
          <description>JQOVF_MST</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADRDY_SLV</name>
          <description>ADRDY_SLV</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOSMP_SLV</name>
          <description>EOSMP_SLV</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOC_SLV</name>
          <description>End of regular conversion of the slave
              ADC</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOS_SLV</name>
          <description>End of regular sequence flag of the
              slave ADC</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OVR_SLV</name>
          <description>Overrun flag of the slave
              ADC</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JEOC_SLV</name>
          <description>End of injected conversion flag of the
              slave ADC</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JEOS_SLV</name>
          <description>End of injected sequence flag of the
              slave ADC</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AWD1_SLV</name>
          <description>Analog watchdog 1 flag of the slave
              ADC</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AWD2_SLV</name>
          <description>Analog watchdog 2 flag of the slave
              ADC</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AWD3_SLV</name>
          <description>Analog watchdog 3 flag of the slave
              ADC</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JQOVF_SLV</name>
          <description>Injected Context Queue Overflow flag of
              the slave ADC</description>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCR</name>
      <displayName>CCR</displayName>
      <description>ADC common control register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MULT</name>
          <description>Multi ADC mode selection</description>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>DELAY</name>
          <description>Delay between 2 sampling
              phases</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>DMACFG</name>
          <description>DMA configuration (for multi-ADC
              mode)</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MDMA</name>
          <description>Direct memory access mode for multi ADC
              mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>CKMODE</name>
          <description>ADC clock mode</description>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>VREFEN</name>
          <description>VREFINT enable</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TSEN</name>
          <description>Temperature sensor enable</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>VBATEN</name>
          <description>VBAT enable</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CDR</name>
      <displayName>CDR</displayName>
      <description>ADC common regular data register for dual
          and triple modes</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>RDATA_SLV</name>
          <description>Regular data of the slave
              ADC</description>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
        <field>
          <name>RDATA_MST</name>
          <description>Regular data of the master
              ADC</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
</peripheral>*/
