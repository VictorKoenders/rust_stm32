/// MOD Flash
/// Flash
const BASE_ADDRESS: u32 = 0x40022000;
/// Flash access control register
/// Size: 0x20 bits
pub mod acr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const LATENCY_BIT_OFFSET: u8 = 0;
	const LATENCY_BIT_WIDTH: u8 = 3;
	/// LATENCY (Width: 3, Offset: 0)
	pub fn get_latency() -> u8 { ::read(REGISTER_ADDRESS, LATENCY_BIT_OFFSET, LATENCY_BIT_WIDTH) as u8 }
	/// LATENCY (Width: 3, Offset: 0)
	pub fn set_latency(value: u8) { ::write(REGISTER_ADDRESS, LATENCY_BIT_OFFSET, LATENCY_BIT_WIDTH, value as u32); }

	const PRFTBE_BIT_OFFSET: u8 = 4;
	const PRFTBE_BIT_WIDTH: u8 = 1;
	/// PRFTBE (Width: 1, Offset: 4)
	pub fn get_prftbe() -> u8 { ::read(REGISTER_ADDRESS, PRFTBE_BIT_OFFSET, PRFTBE_BIT_WIDTH) as u8 }
	/// PRFTBE (Width: 1, Offset: 4)
	pub fn set_prftbe(value: u8) { ::write(REGISTER_ADDRESS, PRFTBE_BIT_OFFSET, PRFTBE_BIT_WIDTH, value as u32); }

	const PRFTBS_BIT_OFFSET: u8 = 5;
	const PRFTBS_BIT_WIDTH: u8 = 1;
	/// PRFTBS (Width: 1, Offset: 5)
	pub fn get_prftbs() -> u8 { ::read(REGISTER_ADDRESS, PRFTBS_BIT_OFFSET, PRFTBS_BIT_WIDTH) as u8 }
}
/// Flash key register
/// Size: 0x20 bits
pub mod keyr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const FKEYR_BIT_OFFSET: u8 = 0;
	const FKEYR_BIT_WIDTH: u8 = 32;
	/// Flash Key (Width: 32, Offset: 0)
	pub fn set_fkeyr(value: u32) { ::write(REGISTER_ADDRESS, FKEYR_BIT_OFFSET, FKEYR_BIT_WIDTH, value as u32); }
}
/// Flash option key register
/// Size: 0x20 bits
pub mod optkeyr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OPTKEYR_BIT_OFFSET: u8 = 0;
	const OPTKEYR_BIT_WIDTH: u8 = 32;
	/// Option byte key (Width: 32, Offset: 0)
	pub fn set_optkeyr(value: u32) { ::write(REGISTER_ADDRESS, OPTKEYR_BIT_OFFSET, OPTKEYR_BIT_WIDTH, value as u32); }
}
/// Flash status register
/// Size: 0x20 bits
pub mod sr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EOP_BIT_OFFSET: u8 = 5;
	const EOP_BIT_WIDTH: u8 = 1;
	/// End of operation (Width: 1, Offset: 5)
	pub fn get_eop() -> u8 { ::read(REGISTER_ADDRESS, EOP_BIT_OFFSET, EOP_BIT_WIDTH) as u8 }
	/// End of operation (Width: 1, Offset: 5)
	pub fn set_eop(value: u8) { ::write(REGISTER_ADDRESS, EOP_BIT_OFFSET, EOP_BIT_WIDTH, value as u32); }

	const WRPRT_BIT_OFFSET: u8 = 4;
	const WRPRT_BIT_WIDTH: u8 = 1;
	/// Write protection error (Width: 1, Offset: 4)
	pub fn get_wrprt() -> u8 { ::read(REGISTER_ADDRESS, WRPRT_BIT_OFFSET, WRPRT_BIT_WIDTH) as u8 }
	/// Write protection error (Width: 1, Offset: 4)
	pub fn set_wrprt(value: u8) { ::write(REGISTER_ADDRESS, WRPRT_BIT_OFFSET, WRPRT_BIT_WIDTH, value as u32); }

	const PGERR_BIT_OFFSET: u8 = 2;
	const PGERR_BIT_WIDTH: u8 = 1;
	/// Programming error (Width: 1, Offset: 2)
	pub fn get_pgerr() -> u8 { ::read(REGISTER_ADDRESS, PGERR_BIT_OFFSET, PGERR_BIT_WIDTH) as u8 }
	/// Programming error (Width: 1, Offset: 2)
	pub fn set_pgerr(value: u8) { ::write(REGISTER_ADDRESS, PGERR_BIT_OFFSET, PGERR_BIT_WIDTH, value as u32); }

	const BSY_BIT_OFFSET: u8 = 0;
	const BSY_BIT_WIDTH: u8 = 1;
	/// Busy (Width: 1, Offset: 0)
	pub fn get_bsy() -> u8 { ::read(REGISTER_ADDRESS, BSY_BIT_OFFSET, BSY_BIT_WIDTH) as u8 }
}
/// Flash control register
/// Size: 0x20 bits
pub mod cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const FORCE_OPTLOAD_BIT_OFFSET: u8 = 13;
	const FORCE_OPTLOAD_BIT_WIDTH: u8 = 1;
	/// Force option byte loading (Width: 1, Offset: 13)
	pub fn get_force_optload() -> u8 { ::read(REGISTER_ADDRESS, FORCE_OPTLOAD_BIT_OFFSET, FORCE_OPTLOAD_BIT_WIDTH) as u8 }
	/// Force option byte loading (Width: 1, Offset: 13)
	pub fn set_force_optload(value: u8) { ::write(REGISTER_ADDRESS, FORCE_OPTLOAD_BIT_OFFSET, FORCE_OPTLOAD_BIT_WIDTH, value as u32); }

	const EOPIE_BIT_OFFSET: u8 = 12;
	const EOPIE_BIT_WIDTH: u8 = 1;
	/// End of operation interrupt enable (Width: 1, Offset: 12)
	pub fn get_eopie() -> u8 { ::read(REGISTER_ADDRESS, EOPIE_BIT_OFFSET, EOPIE_BIT_WIDTH) as u8 }
	/// End of operation interrupt enable (Width: 1, Offset: 12)
	pub fn set_eopie(value: u8) { ::write(REGISTER_ADDRESS, EOPIE_BIT_OFFSET, EOPIE_BIT_WIDTH, value as u32); }

	const ERRIE_BIT_OFFSET: u8 = 10;
	const ERRIE_BIT_WIDTH: u8 = 1;
	/// Error interrupt enable (Width: 1, Offset: 10)
	pub fn get_errie() -> u8 { ::read(REGISTER_ADDRESS, ERRIE_BIT_OFFSET, ERRIE_BIT_WIDTH) as u8 }
	/// Error interrupt enable (Width: 1, Offset: 10)
	pub fn set_errie(value: u8) { ::write(REGISTER_ADDRESS, ERRIE_BIT_OFFSET, ERRIE_BIT_WIDTH, value as u32); }

	const OPTWRE_BIT_OFFSET: u8 = 9;
	const OPTWRE_BIT_WIDTH: u8 = 1;
	/// Option bytes write enable (Width: 1, Offset: 9)
	pub fn get_optwre() -> u8 { ::read(REGISTER_ADDRESS, OPTWRE_BIT_OFFSET, OPTWRE_BIT_WIDTH) as u8 }
	/// Option bytes write enable (Width: 1, Offset: 9)
	pub fn set_optwre(value: u8) { ::write(REGISTER_ADDRESS, OPTWRE_BIT_OFFSET, OPTWRE_BIT_WIDTH, value as u32); }

	const LOCK_BIT_OFFSET: u8 = 7;
	const LOCK_BIT_WIDTH: u8 = 1;
	/// Lock (Width: 1, Offset: 7)
	pub fn get_lock() -> u8 { ::read(REGISTER_ADDRESS, LOCK_BIT_OFFSET, LOCK_BIT_WIDTH) as u8 }
	/// Lock (Width: 1, Offset: 7)
	pub fn set_lock(value: u8) { ::write(REGISTER_ADDRESS, LOCK_BIT_OFFSET, LOCK_BIT_WIDTH, value as u32); }

	const STRT_BIT_OFFSET: u8 = 6;
	const STRT_BIT_WIDTH: u8 = 1;
	/// Start (Width: 1, Offset: 6)
	pub fn get_strt() -> u8 { ::read(REGISTER_ADDRESS, STRT_BIT_OFFSET, STRT_BIT_WIDTH) as u8 }
	/// Start (Width: 1, Offset: 6)
	pub fn set_strt(value: u8) { ::write(REGISTER_ADDRESS, STRT_BIT_OFFSET, STRT_BIT_WIDTH, value as u32); }

	const OPTER_BIT_OFFSET: u8 = 5;
	const OPTER_BIT_WIDTH: u8 = 1;
	/// Option byte erase (Width: 1, Offset: 5)
	pub fn get_opter() -> u8 { ::read(REGISTER_ADDRESS, OPTER_BIT_OFFSET, OPTER_BIT_WIDTH) as u8 }
	/// Option byte erase (Width: 1, Offset: 5)
	pub fn set_opter(value: u8) { ::write(REGISTER_ADDRESS, OPTER_BIT_OFFSET, OPTER_BIT_WIDTH, value as u32); }

	const OPTPG_BIT_OFFSET: u8 = 4;
	const OPTPG_BIT_WIDTH: u8 = 1;
	/// Option byte programming (Width: 1, Offset: 4)
	pub fn get_optpg() -> u8 { ::read(REGISTER_ADDRESS, OPTPG_BIT_OFFSET, OPTPG_BIT_WIDTH) as u8 }
	/// Option byte programming (Width: 1, Offset: 4)
	pub fn set_optpg(value: u8) { ::write(REGISTER_ADDRESS, OPTPG_BIT_OFFSET, OPTPG_BIT_WIDTH, value as u32); }

	const MER_BIT_OFFSET: u8 = 2;
	const MER_BIT_WIDTH: u8 = 1;
	/// Mass erase (Width: 1, Offset: 2)
	pub fn get_mer() -> u8 { ::read(REGISTER_ADDRESS, MER_BIT_OFFSET, MER_BIT_WIDTH) as u8 }
	/// Mass erase (Width: 1, Offset: 2)
	pub fn set_mer(value: u8) { ::write(REGISTER_ADDRESS, MER_BIT_OFFSET, MER_BIT_WIDTH, value as u32); }

	const PER_BIT_OFFSET: u8 = 1;
	const PER_BIT_WIDTH: u8 = 1;
	/// Page erase (Width: 1, Offset: 1)
	pub fn get_per() -> u8 { ::read(REGISTER_ADDRESS, PER_BIT_OFFSET, PER_BIT_WIDTH) as u8 }
	/// Page erase (Width: 1, Offset: 1)
	pub fn set_per(value: u8) { ::write(REGISTER_ADDRESS, PER_BIT_OFFSET, PER_BIT_WIDTH, value as u32); }

	const PG_BIT_OFFSET: u8 = 0;
	const PG_BIT_WIDTH: u8 = 1;
	/// Programming (Width: 1, Offset: 0)
	pub fn get_pg() -> u8 { ::read(REGISTER_ADDRESS, PG_BIT_OFFSET, PG_BIT_WIDTH) as u8 }
	/// Programming (Width: 1, Offset: 0)
	pub fn set_pg(value: u8) { ::write(REGISTER_ADDRESS, PG_BIT_OFFSET, PG_BIT_WIDTH, value as u32); }
}
/// Flash address register
/// Size: 0x20 bits
pub mod ar {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const FAR_BIT_OFFSET: u8 = 0;
	const FAR_BIT_WIDTH: u8 = 32;
	/// Flash address (Width: 32, Offset: 0)
	pub fn set_far(value: u32) { ::write(REGISTER_ADDRESS, FAR_BIT_OFFSET, FAR_BIT_WIDTH, value as u32); }
}
/// Option byte register
/// Size: 0x20 bits
pub mod obr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OPTERR_BIT_OFFSET: u8 = 0;
	const OPTERR_BIT_WIDTH: u8 = 1;
	/// Option byte error (Width: 1, Offset: 0)
	pub fn get_opterr() -> u8 { ::read(REGISTER_ADDRESS, OPTERR_BIT_OFFSET, OPTERR_BIT_WIDTH) as u8 }

	const LEVEL1_PROT_BIT_OFFSET: u8 = 1;
	const LEVEL1_PROT_BIT_WIDTH: u8 = 1;
	/// Level 1 protection status (Width: 1, Offset: 1)
	pub fn get_level1_prot() -> u8 { ::read(REGISTER_ADDRESS, LEVEL1_PROT_BIT_OFFSET, LEVEL1_PROT_BIT_WIDTH) as u8 }

	const LEVEL2_PROT_BIT_OFFSET: u8 = 2;
	const LEVEL2_PROT_BIT_WIDTH: u8 = 1;
	/// Level 2 protection status (Width: 1, Offset: 2)
	pub fn get_level2_prot() -> u8 { ::read(REGISTER_ADDRESS, LEVEL2_PROT_BIT_OFFSET, LEVEL2_PROT_BIT_WIDTH) as u8 }

	const WDG_SW_BIT_OFFSET: u8 = 8;
	const WDG_SW_BIT_WIDTH: u8 = 1;
	/// WDG_SW (Width: 1, Offset: 8)
	pub fn get_wdg_sw() -> u8 { ::read(REGISTER_ADDRESS, WDG_SW_BIT_OFFSET, WDG_SW_BIT_WIDTH) as u8 }

	const nRST_STOP_BIT_OFFSET: u8 = 9;
	const nRST_STOP_BIT_WIDTH: u8 = 1;
	/// nRST_STOP (Width: 1, Offset: 9)
	pub fn get_nrst_stop() -> u8 { ::read(REGISTER_ADDRESS, nRST_STOP_BIT_OFFSET, nRST_STOP_BIT_WIDTH) as u8 }

	const nRST_STDBY_BIT_OFFSET: u8 = 10;
	const nRST_STDBY_BIT_WIDTH: u8 = 1;
	/// nRST_STDBY (Width: 1, Offset: 10)
	pub fn get_nrst_stdby() -> u8 { ::read(REGISTER_ADDRESS, nRST_STDBY_BIT_OFFSET, nRST_STDBY_BIT_WIDTH) as u8 }

	const BOOT1_BIT_OFFSET: u8 = 12;
	const BOOT1_BIT_WIDTH: u8 = 1;
	/// BOOT1 (Width: 1, Offset: 12)
	pub fn get_boot1() -> u8 { ::read(REGISTER_ADDRESS, BOOT1_BIT_OFFSET, BOOT1_BIT_WIDTH) as u8 }

	const VDDA_MONITOR_BIT_OFFSET: u8 = 13;
	const VDDA_MONITOR_BIT_WIDTH: u8 = 1;
	/// VDDA_MONITOR (Width: 1, Offset: 13)
	pub fn get_vdda_monitor() -> u8 { ::read(REGISTER_ADDRESS, VDDA_MONITOR_BIT_OFFSET, VDDA_MONITOR_BIT_WIDTH) as u8 }

	const SRAM_PARITY_CHECK_BIT_OFFSET: u8 = 14;
	const SRAM_PARITY_CHECK_BIT_WIDTH: u8 = 1;
	/// SRAM_PARITY_CHECK (Width: 1, Offset: 14)
	pub fn get_sram_parity_check() -> u8 { ::read(REGISTER_ADDRESS, SRAM_PARITY_CHECK_BIT_OFFSET, SRAM_PARITY_CHECK_BIT_WIDTH) as u8 }

	const Data0_BIT_OFFSET: u8 = 16;
	const Data0_BIT_WIDTH: u8 = 8;
	/// Data0 (Width: 8, Offset: 16)
	pub fn get_data0() -> u8 { ::read(REGISTER_ADDRESS, Data0_BIT_OFFSET, Data0_BIT_WIDTH) as u8 }

	const Data1_BIT_OFFSET: u8 = 24;
	const Data1_BIT_WIDTH: u8 = 8;
	/// Data1 (Width: 8, Offset: 24)
	pub fn get_data1() -> u8 { ::read(REGISTER_ADDRESS, Data1_BIT_OFFSET, Data1_BIT_WIDTH) as u8 }
}
/// Write protection register
/// Size: 0x20 bits
pub mod wrpr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x20;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const WRP_BIT_OFFSET: u8 = 0;
	const WRP_BIT_WIDTH: u8 = 32;
	/// Write protect (Width: 32, Offset: 0)
	pub fn get_wrp() -> u32 { ::read(REGISTER_ADDRESS, WRP_BIT_OFFSET, WRP_BIT_WIDTH) as u32 }
}
/// Flash global interrupt
pub const INTERRUPT_FLASH: u32 = 4;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>Flash</name>
  <description>Flash</description>
  <groupName>Flash</groupName>
  <baseAddress>0x40022000</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>ACR</name>
      <displayName>ACR</displayName>
      <description>Flash access control register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000030</resetValue>
      <fields>
        <field>
          <name>LATENCY</name>
          <description>LATENCY</description>
          <bitOffset>0</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PRFTBE</name>
          <description>PRFTBE</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PRFTBS</name>
          <description>PRFTBS</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
      </fields>
    </register>
    <register>
      <name>KEYR</name>
      <displayName>KEYR</displayName>
      <description>Flash key register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>write-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>FKEYR</name>
          <description>Flash Key</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>OPTKEYR</name>
      <displayName>OPTKEYR</displayName>
      <description>Flash option key register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>write-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OPTKEYR</name>
          <description>Option byte key</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SR</name>
      <displayName>SR</displayName>
      <description>Flash status register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EOP</name>
          <description>End of operation</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>WRPRT</name>
          <description>Write protection error</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PGERR</name>
          <description>Programming error</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>BSY</name>
          <description>Busy</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
      </fields>
    </register>
    <register>
      <name>CR</name>
      <displayName>CR</displayName>
      <description>Flash control register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000080</resetValue>
      <fields>
        <field>
          <name>FORCE_OPTLOAD</name>
          <description>Force option byte loading</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOPIE</name>
          <description>End of operation interrupt
              enable</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ERRIE</name>
          <description>Error interrupt enable</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OPTWRE</name>
          <description>Option bytes write enable</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LOCK</name>
          <description>Lock</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>STRT</name>
          <description>Start</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OPTER</name>
          <description>Option byte erase</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OPTPG</name>
          <description>Option byte programming</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MER</name>
          <description>Mass erase</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PER</name>
          <description>Page erase</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PG</name>
          <description>Programming</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>AR</name>
      <displayName>AR</displayName>
      <description>Flash address register</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>write-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>FAR</name>
          <description>Flash address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>OBR</name>
      <displayName>OBR</displayName>
      <description>Option byte register</description>
      <addressOffset>0x1C</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0xFFFFFF02</resetValue>
      <fields>
        <field>
          <name>OPTERR</name>
          <description>Option byte error</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LEVEL1_PROT</name>
          <description>Level 1 protection status</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LEVEL2_PROT</name>
          <description>Level 2 protection status</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>WDG_SW</name>
          <description>WDG_SW</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>nRST_STOP</name>
          <description>nRST_STOP</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>nRST_STDBY</name>
          <description>nRST_STDBY</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BOOT1</name>
          <description>BOOT1</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>VDDA_MONITOR</name>
          <description>VDDA_MONITOR</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SRAM_PARITY_CHECK</name>
          <description>SRAM_PARITY_CHECK</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>Data0</name>
          <description>Data0</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>Data1</name>
          <description>Data1</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>WRPR</name>
      <displayName>WRPR</displayName>
      <description>Write protection register</description>
      <addressOffset>0x20</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0xFFFFFFFF</resetValue>
      <fields>
        <field>
          <name>WRP</name>
          <description>Write protect</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>FLASH</name>
    <description>Flash global interrupt</description>
    <value>4</value>
  </interrupt>
</peripheral>*/
