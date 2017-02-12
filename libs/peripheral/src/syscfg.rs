/// configuration register 1
pub mod cfgr1 {
    pub struct ReadonlyCache {
        /// Memory mapping selection bits
        pub mem_mode: u8,
        /// USB interrupt remap
        pub usb_it_rmp: u8,
        /// Timer 1 ITR3 selection
        pub tim1_itr_rmp: u8,
        /// DAC trigger remap (when TSEL = 001)
        pub dac_trig_rmp: u8,
        /// ADC24 DMA remapping bit
        pub adc24_dma_rmp: u8,
        /// TIM16 DMA request remapping bit
        pub tim16_dma_rmp: u8,
        /// TIM17 DMA request remapping bit
        pub tim17_dma_rmp: u8,
        /// TIM6 and DAC1 DMA request remapping bit
        pub tim6_dac1_dma_rmp: u8,
        /// TIM7 and DAC2 DMA request remapping bit
        pub tim7_dac2_dma_rmp: u8,
        /// Fast Mode Plus (FM+) driving capability activation bits.
        pub i2c_pb6_fm: u8,
        /// Fast Mode Plus (FM+) driving capability activation bits.
        pub i2c_pb7_fm: u8,
        /// Fast Mode Plus (FM+) driving capability activation bits.
        pub i2c_pb8_fm: u8,
        /// Fast Mode Plus (FM+) driving capability activation bits.
        pub i2c_pb9_fm: u8,
        /// I2C1 Fast Mode Plus
        pub i2c1_fm: u8,
        /// I2C2 Fast Mode Plus
        pub i2c2_fm: u8,
        /// Encoder mode
        pub encoder_mode: u8,
        /// Interrupt enable bits from FPU
        pub fpu_it: u8,
    }
    pub struct Cache {
        /// Memory mapping selection bits
        pub mem_mode: u8,
        /// USB interrupt remap
        pub usb_it_rmp: u8,
        /// Timer 1 ITR3 selection
        pub tim1_itr_rmp: u8,
        /// DAC trigger remap (when TSEL = 001)
        pub dac_trig_rmp: u8,
        /// ADC24 DMA remapping bit
        pub adc24_dma_rmp: u8,
        /// TIM16 DMA request remapping bit
        pub tim16_dma_rmp: u8,
        /// TIM17 DMA request remapping bit
        pub tim17_dma_rmp: u8,
        /// TIM6 and DAC1 DMA request remapping bit
        pub tim6_dac1_dma_rmp: u8,
        /// TIM7 and DAC2 DMA request remapping bit
        pub tim7_dac2_dma_rmp: u8,
        /// Fast Mode Plus (FM+) driving capability activation bits.
        pub i2c_pb6_fm: u8,
        /// Fast Mode Plus (FM+) driving capability activation bits.
        pub i2c_pb7_fm: u8,
        /// Fast Mode Plus (FM+) driving capability activation bits.
        pub i2c_pb8_fm: u8,
        /// Fast Mode Plus (FM+) driving capability activation bits.
        pub i2c_pb9_fm: u8,
        /// I2C1 Fast Mode Plus
        pub i2c1_fm: u8,
        /// I2C2 Fast Mode Plus
        pub i2c2_fm: u8,
        /// Encoder mode
        pub encoder_mode: u8,
        /// Interrupt enable bits from FPU
        pub fpu_it: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010000u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            mem_mode: ((value >> 0) & 0b11) as u8,
            usb_it_rmp: ((value >> 5) & 0b11) as u8,
            tim1_itr_rmp: ((value >> 6) & 0b11) as u8,
            dac_trig_rmp: ((value >> 7) & 0b11) as u8,
            adc24_dma_rmp: ((value >> 8) & 0b11) as u8,
            tim16_dma_rmp: ((value >> 11) & 0b11) as u8,
            tim17_dma_rmp: ((value >> 12) & 0b11) as u8,
            tim6_dac1_dma_rmp: ((value >> 13) & 0b11) as u8,
            tim7_dac2_dma_rmp: ((value >> 14) & 0b11) as u8,
            i2c_pb6_fm: ((value >> 16) & 0b11) as u8,
            i2c_pb7_fm: ((value >> 17) & 0b11) as u8,
            i2c_pb8_fm: ((value >> 18) & 0b11) as u8,
            i2c_pb9_fm: ((value >> 19) & 0b11) as u8,
            i2c1_fm: ((value >> 20) & 0b11) as u8,
            i2c2_fm: ((value >> 21) & 0b11) as u8,
            encoder_mode: ((value >> 22) & 0b11) as u8,
            fpu_it: ((value >> 26) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010000u32 + 0x0u32) as *mut u32) };
        Cache {
            mem_mode: ((value >> 0) & 0b11) as u8,
            usb_it_rmp: ((value >> 5) & 0b11) as u8,
            tim1_itr_rmp: ((value >> 6) & 0b11) as u8,
            dac_trig_rmp: ((value >> 7) & 0b11) as u8,
            adc24_dma_rmp: ((value >> 8) & 0b11) as u8,
            tim16_dma_rmp: ((value >> 11) & 0b11) as u8,
            tim17_dma_rmp: ((value >> 12) & 0b11) as u8,
            tim6_dac1_dma_rmp: ((value >> 13) & 0b11) as u8,
            tim7_dac2_dma_rmp: ((value >> 14) & 0b11) as u8,
            i2c_pb6_fm: ((value >> 16) & 0b11) as u8,
            i2c_pb7_fm: ((value >> 17) & 0b11) as u8,
            i2c_pb8_fm: ((value >> 18) & 0b11) as u8,
            i2c_pb9_fm: ((value >> 19) & 0b11) as u8,
            i2c1_fm: ((value >> 20) & 0b11) as u8,
            i2c2_fm: ((value >> 21) & 0b11) as u8,
            encoder_mode: ((value >> 22) & 0b11) as u8,
            fpu_it: ((value >> 26) & 0b11) as u8,
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
                | ((self.mem_mode as u32) << 0)
                | ((self.usb_it_rmp as u32) << 5)
                | ((self.tim1_itr_rmp as u32) << 6)
                | ((self.dac_trig_rmp as u32) << 7)
                | ((self.adc24_dma_rmp as u32) << 8)
                | ((self.tim16_dma_rmp as u32) << 11)
                | ((self.tim17_dma_rmp as u32) << 12)
                | ((self.tim6_dac1_dma_rmp as u32) << 13)
                | ((self.tim7_dac2_dma_rmp as u32) << 14)
                | ((self.i2c_pb6_fm as u32) << 16)
                | ((self.i2c_pb7_fm as u32) << 17)
                | ((self.i2c_pb8_fm as u32) << 18)
                | ((self.i2c_pb9_fm as u32) << 19)
                | ((self.i2c1_fm as u32) << 20)
                | ((self.i2c2_fm as u32) << 21)
                | ((self.encoder_mode as u32) << 22)
                | ((self.fpu_it as u32) << 26)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010000u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// external interrupt configuration register 1
pub mod exticr1 {
    pub struct ReadonlyCache {
        /// EXTI 3 configuration bits
        pub exti3: u8,
        /// EXTI 2 configuration bits
        pub exti2: u8,
        /// EXTI 1 configuration bits
        pub exti1: u8,
        /// EXTI 0 configuration bits
        pub exti0: u8,
    }
    pub struct Cache {
        /// EXTI 3 configuration bits
        pub exti3: u8,
        /// EXTI 2 configuration bits
        pub exti2: u8,
        /// EXTI 1 configuration bits
        pub exti1: u8,
        /// EXTI 0 configuration bits
        pub exti0: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010000u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            exti3: ((value >> 12) & 0b1111) as u8,
            exti2: ((value >> 8) & 0b1111) as u8,
            exti1: ((value >> 4) & 0b1111) as u8,
            exti0: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010000u32 + 0x8u32) as *mut u32) };
        Cache {
            exti3: ((value >> 12) & 0b1111) as u8,
            exti2: ((value >> 8) & 0b1111) as u8,
            exti1: ((value >> 4) & 0b1111) as u8,
            exti0: ((value >> 0) & 0b1111) as u8,
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
                | ((self.exti3 as u32) << 12)
                | ((self.exti2 as u32) << 8)
                | ((self.exti1 as u32) << 4)
                | ((self.exti0 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010000u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// external interrupt configuration register 2
pub mod exticr2 {
    pub struct ReadonlyCache {
        /// EXTI 7 configuration bits
        pub exti7: u8,
        /// EXTI 6 configuration bits
        pub exti6: u8,
        /// EXTI 5 configuration bits
        pub exti5: u8,
        /// EXTI 4 configuration bits
        pub exti4: u8,
    }
    pub struct Cache {
        /// EXTI 7 configuration bits
        pub exti7: u8,
        /// EXTI 6 configuration bits
        pub exti6: u8,
        /// EXTI 5 configuration bits
        pub exti5: u8,
        /// EXTI 4 configuration bits
        pub exti4: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010000u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            exti7: ((value >> 12) & 0b1111) as u8,
            exti6: ((value >> 8) & 0b1111) as u8,
            exti5: ((value >> 4) & 0b1111) as u8,
            exti4: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010000u32 + 0xCu32) as *mut u32) };
        Cache {
            exti7: ((value >> 12) & 0b1111) as u8,
            exti6: ((value >> 8) & 0b1111) as u8,
            exti5: ((value >> 4) & 0b1111) as u8,
            exti4: ((value >> 0) & 0b1111) as u8,
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
                | ((self.exti7 as u32) << 12)
                | ((self.exti6 as u32) << 8)
                | ((self.exti5 as u32) << 4)
                | ((self.exti4 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010000u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// external interrupt configuration register 3
pub mod exticr3 {
    pub struct ReadonlyCache {
        /// EXTI 11 configuration bits
        pub exti11: u8,
        /// EXTI 10 configuration bits
        pub exti10: u8,
        /// EXTI 9 configuration bits
        pub exti9: u8,
        /// EXTI 8 configuration bits
        pub exti8: u8,
    }
    pub struct Cache {
        /// EXTI 11 configuration bits
        pub exti11: u8,
        /// EXTI 10 configuration bits
        pub exti10: u8,
        /// EXTI 9 configuration bits
        pub exti9: u8,
        /// EXTI 8 configuration bits
        pub exti8: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010000u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            exti11: ((value >> 12) & 0b1111) as u8,
            exti10: ((value >> 8) & 0b1111) as u8,
            exti9: ((value >> 4) & 0b1111) as u8,
            exti8: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010000u32 + 0x10u32) as *mut u32) };
        Cache {
            exti11: ((value >> 12) & 0b1111) as u8,
            exti10: ((value >> 8) & 0b1111) as u8,
            exti9: ((value >> 4) & 0b1111) as u8,
            exti8: ((value >> 0) & 0b1111) as u8,
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
                | ((self.exti11 as u32) << 12)
                | ((self.exti10 as u32) << 8)
                | ((self.exti9 as u32) << 4)
                | ((self.exti8 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010000u32 + 0x10u32) as *mut u32, value) };
        }
    }
}
/// external interrupt configuration register 4
pub mod exticr4 {
    pub struct ReadonlyCache {
        /// EXTI 15 configuration bits
        pub exti15: u8,
        /// EXTI 14 configuration bits
        pub exti14: u8,
        /// EXTI 13 configuration bits
        pub exti13: u8,
        /// EXTI 12 configuration bits
        pub exti12: u8,
    }
    pub struct Cache {
        /// EXTI 15 configuration bits
        pub exti15: u8,
        /// EXTI 14 configuration bits
        pub exti14: u8,
        /// EXTI 13 configuration bits
        pub exti13: u8,
        /// EXTI 12 configuration bits
        pub exti12: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010000u32 + 0x14u32) as *mut u32) };
        ReadonlyCache {
            exti15: ((value >> 12) & 0b1111) as u8,
            exti14: ((value >> 8) & 0b1111) as u8,
            exti13: ((value >> 4) & 0b1111) as u8,
            exti12: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010000u32 + 0x14u32) as *mut u32) };
        Cache {
            exti15: ((value >> 12) & 0b1111) as u8,
            exti14: ((value >> 8) & 0b1111) as u8,
            exti13: ((value >> 4) & 0b1111) as u8,
            exti12: ((value >> 0) & 0b1111) as u8,
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
                | ((self.exti15 as u32) << 12)
                | ((self.exti14 as u32) << 8)
                | ((self.exti13 as u32) << 4)
                | ((self.exti12 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010000u32 + 0x14u32) as *mut u32, value) };
        }
    }
}
/// configuration register 2
pub mod cfgr2 {
    pub struct ReadonlyCache {
        /// Cortex-M0 LOCKUP bit enable bit
        pub locup_lock: bool,
        /// SRAM parity lock bit
        pub sram_parity_lock: bool,
        /// PVD lock enable bit
        pub pvd_lock: bool,
        /// Bypass address bit 29 in parity calculation
        pub byp_add_par: bool,
        /// SRAM parity flag
        pub sram_pef: bool,
    }
    pub struct Cache {
        /// Cortex-M0 LOCKUP bit enable bit
        pub locup_lock: bool,
        /// SRAM parity lock bit
        pub sram_parity_lock: bool,
        /// PVD lock enable bit
        pub pvd_lock: bool,
        /// Bypass address bit 29 in parity calculation
        pub byp_add_par: bool,
        /// SRAM parity flag
        pub sram_pef: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010000u32 + 0x18u32) as *mut u32) };
        ReadonlyCache {
            locup_lock: ((value >> 0) & 0b1) > 0,
            sram_parity_lock: ((value >> 1) & 0b1) > 0,
            pvd_lock: ((value >> 2) & 0b1) > 0,
            byp_add_par: ((value >> 4) & 0b1) > 0,
            sram_pef: ((value >> 8) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010000u32 + 0x18u32) as *mut u32) };
        Cache {
            locup_lock: ((value >> 0) & 0b1) > 0,
            sram_parity_lock: ((value >> 1) & 0b1) > 0,
            pvd_lock: ((value >> 2) & 0b1) > 0,
            byp_add_par: ((value >> 4) & 0b1) > 0,
            sram_pef: ((value >> 8) & 0b1) > 0,
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
                | ((self.locup_lock as u32) << 0)
                | ((self.sram_parity_lock as u32) << 1)
                | ((self.pvd_lock as u32) << 2)
                | ((self.byp_add_par as u32) << 4)
                | ((self.sram_pef as u32) << 8)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010000u32 + 0x18u32) as *mut u32, value) };
        }
    }
}
/// CCM SRAM protection register
pub mod rcr {
    pub struct ReadonlyCache {
        /// CCM SRAM page write protection bit
        pub page0_wp: bool,
        /// CCM SRAM page write protection bit
        pub page1_wp: bool,
        /// CCM SRAM page write protection bit
        pub page2_wp: bool,
        /// CCM SRAM page write protection bit
        pub page3_wp: bool,
        /// CCM SRAM page write protection bit
        pub page4_wp: bool,
        /// CCM SRAM page write protection bit
        pub page5_wp: bool,
        /// CCM SRAM page write protection bit
        pub page6_wp: bool,
        /// CCM SRAM page write protection bit
        pub page7_wp: bool,
    }
    pub struct Cache {
        /// CCM SRAM page write protection bit
        pub page0_wp: bool,
        /// CCM SRAM page write protection bit
        pub page1_wp: bool,
        /// CCM SRAM page write protection bit
        pub page2_wp: bool,
        /// CCM SRAM page write protection bit
        pub page3_wp: bool,
        /// CCM SRAM page write protection bit
        pub page4_wp: bool,
        /// CCM SRAM page write protection bit
        pub page5_wp: bool,
        /// CCM SRAM page write protection bit
        pub page6_wp: bool,
        /// CCM SRAM page write protection bit
        pub page7_wp: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010000u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            page0_wp: ((value >> 0) & 0b1) > 0,
            page1_wp: ((value >> 1) & 0b1) > 0,
            page2_wp: ((value >> 2) & 0b1) > 0,
            page3_wp: ((value >> 3) & 0b1) > 0,
            page4_wp: ((value >> 4) & 0b1) > 0,
            page5_wp: ((value >> 5) & 0b1) > 0,
            page6_wp: ((value >> 6) & 0b1) > 0,
            page7_wp: ((value >> 7) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40010000u32 + 0x4u32) as *mut u32) };
        Cache {
            page0_wp: ((value >> 0) & 0b1) > 0,
            page1_wp: ((value >> 1) & 0b1) > 0,
            page2_wp: ((value >> 2) & 0b1) > 0,
            page3_wp: ((value >> 3) & 0b1) > 0,
            page4_wp: ((value >> 4) & 0b1) > 0,
            page5_wp: ((value >> 5) & 0b1) > 0,
            page6_wp: ((value >> 6) & 0b1) > 0,
            page7_wp: ((value >> 7) & 0b1) > 0,
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
                | ((self.page0_wp as u32) << 0)
                | ((self.page1_wp as u32) << 1)
                | ((self.page2_wp as u32) << 2)
                | ((self.page3_wp as u32) << 3)
                | ((self.page4_wp as u32) << 4)
                | ((self.page5_wp as u32) << 5)
                | ((self.page6_wp as u32) << 6)
                | ((self.page7_wp as u32) << 7)
            ;
            unsafe { ::core::ptr::write_volatile((0x40010000u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x19</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40010000</baseAddress>
  <description>System configuration controller</description>
  <groupName>SYSCFG</groupName>
  <name>SYSCFG</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>configuration register 1</description>
      <displayName>CFGR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Memory mapping selection
                                bits
                            </description>
          <name>MEM_MODE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USB interrupt remap</description>
          <name>USB_IT_RMP</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Timer 1 ITR3 selection</description>
          <name>TIM1_ITR_RMP</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DAC trigger remap (when TSEL =
                                001)
                            </description>
          <name>DAC_TRIG_RMP</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADC24 DMA remapping bit</description>
          <name>ADC24_DMA_RMP</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                TIM16 DMA request remapping
                                bit
                            </description>
          <name>TIM16_DMA_RMP</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                TIM17 DMA request remapping
                                bit
                            </description>
          <name>TIM17_DMA_RMP</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                TIM6 and DAC1 DMA request remapping
                                bit
                            </description>
          <name>TIM6_DAC1_DMA_RMP</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                TIM7 and DAC2 DMA request remapping
                                bit
                            </description>
          <name>TIM7_DAC2_DMA_RMP</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Fast Mode Plus (FM+) driving capability
                                activation bits.
                            </description>
          <name>I2C_PB6_FM</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Fast Mode Plus (FM+) driving capability
                                activation bits.
                            </description>
          <name>I2C_PB7_FM</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Fast Mode Plus (FM+) driving capability
                                activation bits.
                            </description>
          <name>I2C_PB8_FM</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Fast Mode Plus (FM+) driving capability
                                activation bits.
                            </description>
          <name>I2C_PB9_FM</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I2C1 Fast Mode Plus</description>
          <name>I2C1_FM</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I2C2 Fast Mode Plus</description>
          <name>I2C2_FM</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Encoder mode</description>
          <name>ENCODER_MODE</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>6</bitWidth>
          <description>
                                Interrupt enable bits from
                                FPU
                            </description>
          <name>FPU_IT</name>
        </field>
      </fields>
      <name>CFGR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>
                        external interrupt configuration register
                        1
                    </description>
      <displayName>EXTICR1</displayName>
      <fields>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 3 configuration bits</description>
          <name>EXTI3</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 2 configuration bits</description>
          <name>EXTI2</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 1 configuration bits</description>
          <name>EXTI1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 0 configuration bits</description>
          <name>EXTI0</name>
        </field>
      </fields>
      <name>EXTICR1</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>
                        external interrupt configuration register
                        2
                    </description>
      <displayName>EXTICR2</displayName>
      <fields>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 7 configuration bits</description>
          <name>EXTI7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 6 configuration bits</description>
          <name>EXTI6</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 5 configuration bits</description>
          <name>EXTI5</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 4 configuration bits</description>
          <name>EXTI4</name>
        </field>
      </fields>
      <name>EXTICR2</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>
                        external interrupt configuration register
                        3
                    </description>
      <displayName>EXTICR3</displayName>
      <fields>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 11 configuration bits</description>
          <name>EXTI11</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 10 configuration bits</description>
          <name>EXTI10</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 9 configuration bits</description>
          <name>EXTI9</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 8 configuration bits</description>
          <name>EXTI8</name>
        </field>
      </fields>
      <name>EXTICR3</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>
                        external interrupt configuration register
                        4
                    </description>
      <displayName>EXTICR4</displayName>
      <fields>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 15 configuration bits</description>
          <name>EXTI15</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 14 configuration bits</description>
          <name>EXTI14</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 13 configuration bits</description>
          <name>EXTI13</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI 12 configuration bits</description>
          <name>EXTI12</name>
        </field>
      </fields>
      <name>EXTICR4</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>configuration register 2</description>
      <displayName>CFGR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Cortex-M0 LOCKUP bit enable
                                bit
                            </description>
          <name>LOCUP_LOCK</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SRAM parity lock bit</description>
          <name>SRAM_PARITY_LOCK</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PVD lock enable bit</description>
          <name>PVD_LOCK</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Bypass address bit 29 in parity
                                calculation
                            </description>
          <name>BYP_ADD_PAR</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SRAM parity flag</description>
          <name>SRAM_PEF</name>
        </field>
      </fields>
      <name>CFGR2</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>CCM SRAM protection register</description>
      <displayName>RCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                CCM SRAM page write protection
                                bit
                            </description>
          <name>PAGE0_WP</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                CCM SRAM page write protection
                                bit
                            </description>
          <name>PAGE1_WP</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                CCM SRAM page write protection
                                bit
                            </description>
          <name>PAGE2_WP</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                CCM SRAM page write protection
                                bit
                            </description>
          <name>PAGE3_WP</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                CCM SRAM page write protection
                                bit
                            </description>
          <name>PAGE4_WP</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                CCM SRAM page write protection
                                bit
                            </description>
          <name>PAGE5_WP</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                CCM SRAM page write protection
                                bit
                            </description>
          <name>PAGE6_WP</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                CCM SRAM page write protection
                                bit
                            </description>
          <name>PAGE7_WP</name>
        </field>
      </fields>
      <name>RCR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
