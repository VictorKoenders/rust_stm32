/// ADC Common status register
pub mod csr {
    /// ADDRDY_MST
    /// Access: read-only, Width: 1, Offset: 0
    /// Get ADDRDY_MST
    pub fn addrdy_mst() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// EOSMP_MST
    /// Access: read-only, Width: 1, Offset: 1
    /// Get EOSMP_MST
    pub fn eosmp_mst() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// EOC_MST
    /// Access: read-only, Width: 1, Offset: 2
    /// Get EOC_MST
    pub fn eoc_mst() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// EOS_MST
    /// Access: read-only, Width: 1, Offset: 3
    /// Get EOS_MST
    pub fn eos_mst() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// OVR_MST
    /// Access: read-only, Width: 1, Offset: 4
    /// Get OVR_MST
    pub fn ovr_mst() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// JEOC_MST
    /// Access: read-only, Width: 1, Offset: 5
    /// Get JEOC_MST
    pub fn jeoc_mst() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// JEOS_MST
    /// Access: read-only, Width: 1, Offset: 6
    /// Get JEOS_MST
    pub fn jeos_mst() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// AWD1_MST
    /// Access: read-only, Width: 1, Offset: 7
    /// Get AWD1_MST
    pub fn awd1_mst() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// AWD2_MST
    /// Access: read-only, Width: 1, Offset: 8
    /// Get AWD2_MST
    pub fn awd2_mst() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// AWD3_MST
    /// Access: read-only, Width: 1, Offset: 9
    /// Get AWD3_MST
    pub fn awd3_mst() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// JQOVF_MST
    /// Access: read-only, Width: 1, Offset: 10
    /// Get JQOVF_MST
    pub fn jqovf_mst() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 10);
        value > 0
    }
    /// ADRDY_SLV
    /// Access: read-only, Width: 1, Offset: 16
    /// Get ADRDY_SLV
    pub fn adrdy_slv() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 16);
        value > 0
    }
    /// EOSMP_SLV
    /// Access: read-only, Width: 1, Offset: 17
    /// Get EOSMP_SLV
    pub fn eosmp_slv() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 17);
        value > 0
    }
    /// End of regular conversion of the slave ADC
    /// Access: read-only, Width: 1, Offset: 18
    /// Get End of regular conversion of the slave ADC
    pub fn eoc_slv() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 18);
        value > 0
    }
    /// End of regular sequence flag of the slave ADC
    /// Access: read-only, Width: 1, Offset: 19
    /// Get End of regular sequence flag of the slave ADC
    pub fn eos_slv() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 19);
        value > 0
    }
    /// Overrun flag of the slave ADC
    /// Access: read-only, Width: 1, Offset: 20
    /// Get Overrun flag of the slave ADC
    pub fn ovr_slv() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 20);
        value > 0
    }
    /// End of injected conversion flag of the slave ADC
    /// Access: read-only, Width: 1, Offset: 21
    /// Get End of injected conversion flag of the slave ADC
    pub fn jeoc_slv() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 21);
        value > 0
    }
    /// End of injected sequence flag of the slave ADC
    /// Access: read-only, Width: 1, Offset: 22
    /// Get End of injected sequence flag of the slave ADC
    pub fn jeos_slv() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 22);
        value > 0
    }
    /// Analog watchdog 1 flag of the slave ADC
    /// Access: read-only, Width: 1, Offset: 23
    /// Get Analog watchdog 1 flag of the slave ADC
    pub fn awd1_slv() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 23);
        value > 0
    }
    /// Analog watchdog 2 flag of the slave ADC
    /// Access: read-only, Width: 1, Offset: 24
    /// Get Analog watchdog 2 flag of the slave ADC
    pub fn awd2_slv() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 24);
        value > 0
    }
    /// Analog watchdog 3 flag of the slave ADC
    /// Access: read-only, Width: 1, Offset: 25
    /// Get Analog watchdog 3 flag of the slave ADC
    pub fn awd3_slv() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 25);
        value > 0
    }
    /// Injected Context Queue Overflow flag of the slave ADC
    /// Access: read-only, Width: 1, Offset: 26
    /// Get Injected Context Queue Overflow flag of the slave ADC
    pub fn jqovf_slv() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 26);
        value > 0
    }
}
/// ADC common control register
pub mod ccr {
    pub struct ReadonlyCache {
        /// Multi ADC mode selection
        pub mult: u8,
        /// Delay between 2 sampling phases
        pub delay: u8,
        /// DMA configuration (for multi-ADC mode)
        pub dmacfg: u8,
        /// Direct memory access mode for multi ADC mode
        pub mdma: u8,
        /// ADC clock mode
        pub ckmode: u8,
        /// VREFINT enable
        pub vrefen: u8,
        /// Temperature sensor enable
        pub tsen: u8,
        /// VBAT enable
        pub vbaten: u8,
    }
    pub struct Cache {
        /// Multi ADC mode selection
        pub mult: u8,
        /// Delay between 2 sampling phases
        pub delay: u8,
        /// DMA configuration (for multi-ADC mode)
        pub dmacfg: u8,
        /// Direct memory access mode for multi ADC mode
        pub mdma: u8,
        /// ADC clock mode
        pub ckmode: u8,
        /// VREFINT enable
        pub vrefen: u8,
        /// Temperature sensor enable
        pub tsen: u8,
        /// VBAT enable
        pub vbaten: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            mult: ((value >> 0) & 0b11111) as u8,
            delay: ((value >> 8) & 0b11111) as u8,
            dmacfg: ((value >> 13) & 0b11111) as u8,
            mdma: ((value >> 14) & 0b11111) as u8,
            ckmode: ((value >> 16) & 0b11111) as u8,
            vrefen: ((value >> 22) & 0b11111) as u8,
            tsen: ((value >> 23) & 0b11111) as u8,
            vbaten: ((value >> 24) & 0b11111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0x8u32) as *mut u32) };
        Cache {
            mult: ((value >> 0) & 0b11111) as u8,
            delay: ((value >> 8) & 0b11111) as u8,
            dmacfg: ((value >> 13) & 0b11111) as u8,
            mdma: ((value >> 14) & 0b11111) as u8,
            ckmode: ((value >> 16) & 0b11111) as u8,
            vrefen: ((value >> 22) & 0b11111) as u8,
            tsen: ((value >> 23) & 0b11111) as u8,
            vbaten: ((value >> 24) & 0b11111) as u8,
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
                | ((self.mult as u32) << 0)
                | ((self.delay as u32) << 8)
                | ((self.dmacfg as u32) << 13)
                | ((self.mdma as u32) << 14)
                | ((self.ckmode as u32) << 16)
                | ((self.vrefen as u32) << 22)
                | ((self.tsen as u32) << 23)
                | ((self.vbaten as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000300u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// ADC common regular data register for dual and triple modes
pub mod cdr {
    /// Regular data of the slave ADC
    /// Access: read-only, Width: 16, Offset: 16
    /// Get Regular data of the slave ADC
    pub fn rdata_slv() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1111111111111111 << 16);
        value as u16
    }
    /// Regular data of the master ADC
    /// Access: read-only, Width: 16, Offset: 0
    /// Get Regular data of the master ADC
    pub fn rdata_mst() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x50000300u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0xD</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x50000300</baseAddress>
  <description>Analog-to-Digital Converter</description>
  <groupName>ADC</groupName>
  <name>ADC1_2</name>
  <registers>
    <register>
      <access>read-only</access>
      <addressOffset>0x0</addressOffset>
      <description>ADC Common status register</description>
      <displayName>CSR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADDRDY_MST</description>
          <name>ADDRDY_MST</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EOSMP_MST</description>
          <name>EOSMP_MST</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EOC_MST</description>
          <name>EOC_MST</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EOS_MST</description>
          <name>EOS_MST</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OVR_MST</description>
          <name>OVR_MST</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JEOC_MST</description>
          <name>JEOC_MST</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JEOS_MST</description>
          <name>JEOS_MST</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AWD1_MST</description>
          <name>AWD1_MST</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AWD2_MST</description>
          <name>AWD2_MST</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AWD3_MST</description>
          <name>AWD3_MST</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JQOVF_MST</description>
          <name>JQOVF_MST</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADRDY_SLV</description>
          <name>ADRDY_SLV</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EOSMP_SLV</description>
          <name>EOSMP_SLV</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                End of regular conversion of the slave
                                ADC
                            </description>
          <name>EOC_SLV</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                End of regular sequence flag of the
                                slave ADC
                            </description>
          <name>EOS_SLV</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Overrun flag of the slave
                                ADC
                            </description>
          <name>OVR_SLV</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                End of injected conversion flag of the
                                slave ADC
                            </description>
          <name>JEOC_SLV</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                End of injected sequence flag of the
                                slave ADC
                            </description>
          <name>JEOS_SLV</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Analog watchdog 1 flag of the slave
                                ADC
                            </description>
          <name>AWD1_SLV</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Analog watchdog 2 flag of the slave
                                ADC
                            </description>
          <name>AWD2_SLV</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Analog watchdog 3 flag of the slave
                                ADC
                            </description>
          <name>AWD3_SLV</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Injected Context Queue Overflow flag of
                                the slave ADC
                            </description>
          <name>JQOVF_SLV</name>
        </field>
      </fields>
      <name>CSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>ADC common control register</description>
      <displayName>CCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
          <description>Multi ADC mode selection</description>
          <name>MULT</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Delay between 2 sampling
                                phases
                            </description>
          <name>DELAY</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DMA configuration (for multi-ADC
                                mode)
                            </description>
          <name>DMACFG</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Direct memory access mode for multi ADC
                                mode
                            </description>
          <name>MDMA</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <description>ADC clock mode</description>
          <name>CKMODE</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>VREFINT enable</description>
          <name>VREFEN</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Temperature sensor enable</description>
          <name>TSEN</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>VBAT enable</description>
          <name>VBATEN</name>
        </field>
      </fields>
      <name>CCR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0xC</addressOffset>
      <description>
                        ADC common regular data register for dual
                        and triple modes
                    </description>
      <displayName>CDR</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>
                                Regular data of the slave
                                ADC
                            </description>
          <name>RDATA_SLV</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>
                                Regular data of the master
                                ADC
                            </description>
          <name>RDATA_MST</name>
        </field>
      </fields>
      <name>CDR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
