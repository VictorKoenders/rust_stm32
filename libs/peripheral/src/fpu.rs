/// Coprocessor Access Control Register
pub mod cpacr {
    pub struct ReadonlyCache {
        /// Access privileges for coprocessor 0
        pub cp0: bool,
        /// Access privileges for coprocessor 1
        pub cp1: bool,
        /// Access privileges for coprocessor 2
        pub cp2: bool,
        /// Access privileges for coprocessor 3
        pub cp3: bool,
        /// Access privileges for coprocessor 4
        pub cp4: bool,
        /// Access privileges for coprocessor 5
        pub cp5: bool,
        /// Access privileges for coprocessor 6
        pub cp6: bool,
        /// Access privileges for coprocessor 7
        pub cp7: bool,
        /// Access privileges for coprocessor 10
        pub cp10: bool,
        /// Access privileges for coprocessor 11
        pub cp11: bool,
    }
    pub struct Cache {
        /// Access privileges for coprocessor 0
        pub cp0: bool,
        /// Access privileges for coprocessor 1
        pub cp1: bool,
        /// Access privileges for coprocessor 2
        pub cp2: bool,
        /// Access privileges for coprocessor 3
        pub cp3: bool,
        /// Access privileges for coprocessor 4
        pub cp4: bool,
        /// Access privileges for coprocessor 5
        pub cp5: bool,
        /// Access privileges for coprocessor 6
        pub cp6: bool,
        /// Access privileges for coprocessor 7
        pub cp7: bool,
        /// Access privileges for coprocessor 10
        pub cp10: bool,
        /// Access privileges for coprocessor 11
        pub cp11: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            cp0: ((value >> 0) & 0b1) > 0,
            cp1: ((value >> 2) & 0b1) > 0,
            cp2: ((value >> 4) & 0b1) > 0,
            cp3: ((value >> 6) & 0b1) > 0,
            cp4: ((value >> 8) & 0b1) > 0,
            cp5: ((value >> 10) & 0b1) > 0,
            cp6: ((value >> 12) & 0b1) > 0,
            cp7: ((value >> 14) & 0b1) > 0,
            cp10: ((value >> 20) & 0b1) > 0,
            cp11: ((value >> 22) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x0u32) as *mut u32) };
        Cache {
            cp0: ((value >> 0) & 0b1) > 0,
            cp1: ((value >> 2) & 0b1) > 0,
            cp2: ((value >> 4) & 0b1) > 0,
            cp3: ((value >> 6) & 0b1) > 0,
            cp4: ((value >> 8) & 0b1) > 0,
            cp5: ((value >> 10) & 0b1) > 0,
            cp6: ((value >> 12) & 0b1) > 0,
            cp7: ((value >> 14) & 0b1) > 0,
            cp10: ((value >> 20) & 0b1) > 0,
            cp11: ((value >> 22) & 0b1) > 0,
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
                | ((self.cp0 as u32) << 0)
                | ((self.cp1 as u32) << 2)
                | ((self.cp2 as u32) << 4)
                | ((self.cp3 as u32) << 6)
                | ((self.cp4 as u32) << 8)
                | ((self.cp5 as u32) << 10)
                | ((self.cp6 as u32) << 12)
                | ((self.cp7 as u32) << 14)
                | ((self.cp10 as u32) << 20)
                | ((self.cp11 as u32) << 22)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000ED88u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// FP Context Control Register
pub mod fpccr {
    pub struct ReadonlyCache {
        /// LSPACT
        pub lspact: bool,
        /// USER
        pub user: bool,
        /// THREAD
        pub thread: bool,
        /// HFRDY
        pub hfrdy: bool,
        /// MMRDY
        pub mmrdy: bool,
        /// BFRDY
        pub bfrdy: bool,
        /// MONRDY
        pub monrdy: bool,
        /// LSPEN
        pub lspen: bool,
        /// ASPEN
        pub aspen: bool,
    }
    pub struct Cache {
        /// LSPACT
        pub lspact: bool,
        /// USER
        pub user: bool,
        /// THREAD
        pub thread: bool,
        /// HFRDY
        pub hfrdy: bool,
        /// MMRDY
        pub mmrdy: bool,
        /// BFRDY
        pub bfrdy: bool,
        /// MONRDY
        pub monrdy: bool,
        /// LSPEN
        pub lspen: bool,
        /// ASPEN
        pub aspen: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1ACu32) as *mut u32) };
        ReadonlyCache {
            lspact: ((value >> 0) & 0b1) > 0,
            user: ((value >> 1) & 0b1) > 0,
            thread: ((value >> 3) & 0b1) > 0,
            hfrdy: ((value >> 4) & 0b1) > 0,
            mmrdy: ((value >> 5) & 0b1) > 0,
            bfrdy: ((value >> 6) & 0b1) > 0,
            monrdy: ((value >> 8) & 0b1) > 0,
            lspen: ((value >> 30) & 0b1) > 0,
            aspen: ((value >> 31) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1ACu32) as *mut u32) };
        Cache {
            lspact: ((value >> 0) & 0b1) > 0,
            user: ((value >> 1) & 0b1) > 0,
            thread: ((value >> 3) & 0b1) > 0,
            hfrdy: ((value >> 4) & 0b1) > 0,
            mmrdy: ((value >> 5) & 0b1) > 0,
            bfrdy: ((value >> 6) & 0b1) > 0,
            monrdy: ((value >> 8) & 0b1) > 0,
            lspen: ((value >> 30) & 0b1) > 0,
            aspen: ((value >> 31) & 0b1) > 0,
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
                | ((self.lspact as u32) << 0)
                | ((self.user as u32) << 1)
                | ((self.thread as u32) << 3)
                | ((self.hfrdy as u32) << 4)
                | ((self.mmrdy as u32) << 5)
                | ((self.bfrdy as u32) << 6)
                | ((self.monrdy as u32) << 8)
                | ((self.lspen as u32) << 30)
                | ((self.aspen as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000ED88u32 + 0x1ACu32) as *mut u32, value) };
        }
    }
}
/// FP Context Address Register
pub mod fpcar {
    pub struct ReadonlyCache {
        /// ADDRESS
        pub address: u32,
    }
    pub struct Cache {
        /// ADDRESS
        pub address: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1B0u32) as *mut u32) };
        ReadonlyCache {
            address: ((value >> 3) & 0b11111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1B0u32) as *mut u32) };
        Cache {
            address: ((value >> 3) & 0b11111111111111111111111111111) as u32,
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
                | ((self.address as u32) << 3)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000ED88u32 + 0x1B0u32) as *mut u32, value) };
        }
    }
}
/// FP Default Status Control Register
pub mod fpdscr {
    pub struct ReadonlyCache {
        /// RMode
        pub rmode: u8,
        /// FZ
        pub fz: u8,
        /// DN
        pub dn: u8,
        /// AHP
        pub ahp: u8,
    }
    pub struct Cache {
        /// RMode
        pub rmode: u8,
        /// FZ
        pub fz: u8,
        /// DN
        pub dn: u8,
        /// AHP
        pub ahp: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1B4u32) as *mut u32) };
        ReadonlyCache {
            rmode: ((value >> 22) & 0b11) as u8,
            fz: ((value >> 24) & 0b11) as u8,
            dn: ((value >> 25) & 0b11) as u8,
            ahp: ((value >> 26) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1B4u32) as *mut u32) };
        Cache {
            rmode: ((value >> 22) & 0b11) as u8,
            fz: ((value >> 24) & 0b11) as u8,
            dn: ((value >> 25) & 0b11) as u8,
            ahp: ((value >> 26) & 0b11) as u8,
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
                | ((self.rmode as u32) << 22)
                | ((self.fz as u32) << 24)
                | ((self.dn as u32) << 25)
                | ((self.ahp as u32) << 26)
            ;
            unsafe { ::core::ptr::write_volatile((0xE000ED88u32 + 0x1B4u32) as *mut u32, value) };
        }
    }
}
/// Media and VFP Feature Register 0
pub mod mvfr0 {
    /// A_SIMD registers
    /// Access: read-only, Width: 4, Offset: 0
    /// Get A_SIMD registers
    pub fn a_simd() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1B8u32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
    /// Single_precision
    /// Access: read-only, Width: 4, Offset: 4
    /// Get Single_precision
    pub fn single_precision() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1B8u32) as *mut u32) };
        let value = value & (0b1111 << 4);
        value as u8
    }
    /// Double_precision
    /// Access: read-only, Width: 4, Offset: 8
    /// Get Double_precision
    pub fn double_precision() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1B8u32) as *mut u32) };
        let value = value & (0b1111 << 8);
        value as u8
    }
    /// FP exception trapping
    /// Access: read-only, Width: 4, Offset: 12
    /// Get FP exception trapping
    pub fn fp_exception_trapping() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1B8u32) as *mut u32) };
        let value = value & (0b1111 << 12);
        value as u8
    }
    /// Divide
    /// Access: read-only, Width: 4, Offset: 16
    /// Get Divide
    pub fn divide() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1B8u32) as *mut u32) };
        let value = value & (0b1111 << 16);
        value as u8
    }
    /// Square root
    /// Access: read-only, Width: 4, Offset: 20
    /// Get Square root
    pub fn square_root() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1B8u32) as *mut u32) };
        let value = value & (0b1111 << 20);
        value as u8
    }
    /// Short vectors
    /// Access: read-only, Width: 4, Offset: 24
    /// Get Short vectors
    pub fn short_vectors() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1B8u32) as *mut u32) };
        let value = value & (0b1111 << 24);
        value as u8
    }
    /// FP rounding modes
    /// Access: read-only, Width: 4, Offset: 28
    /// Get FP rounding modes
    pub fn fp_rounding_modes() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1B8u32) as *mut u32) };
        let value = value & (0b1111 << 28);
        value as u8
    }
}
/// Media and VFP Feature Register 1
pub mod mvfr1 {
    /// FtZ mode
    /// Access: read-only, Width: 4, Offset: 0
    /// Get FtZ mode
    pub fn ftz_mode() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1BCu32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
    /// D_NaN mode
    /// Access: read-only, Width: 4, Offset: 4
    /// Get D_NaN mode
    pub fn d_nan_mode() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1BCu32) as *mut u32) };
        let value = value & (0b1111 << 4);
        value as u8
    }
    /// FP HPFP
    /// Access: read-only, Width: 4, Offset: 24
    /// Get FP HPFP
    pub fn fp_hpfp() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1BCu32) as *mut u32) };
        let value = value & (0b1111 << 24);
        value as u8
    }
    /// FP fused MAC
    /// Access: read-only, Width: 4, Offset: 28
    /// Get FP fused MAC
    pub fn fp_fused_mac() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0xE000ED88u32 + 0x1BCu32) as *mut u32) };
        let value = value & (0b1111 << 28);
        value as u8
    }
}
/// Floating point interrupt
pub const INTERRUPT_FPU: u32 = 81;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x200</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0xE000ED88</baseAddress>
  <description>Floting point unit</description>
  <groupName>FPU</groupName>
  <interrupt>
    <description>Floating point interrupt</description>
    <name>FPU</name>
    <value>81</value>
  </interrupt>
  <name>FPU</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>
                        Coprocessor Access Control
                        Register
                    </description>
      <displayName>CPACR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Access privileges for coprocessor
                                0
                            </description>
          <name>CP0</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Access privileges for coprocessor
                                1
                            </description>
          <name>CP1</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Access privileges for coprocessor
                                2
                            </description>
          <name>CP2</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Access privileges for coprocessor
                                3
                            </description>
          <name>CP3</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Access privileges for coprocessor
                                4
                            </description>
          <name>CP4</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Access privileges for coprocessor
                                5
                            </description>
          <name>CP5</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Access privileges for coprocessor
                                6
                            </description>
          <name>CP6</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Access privileges for coprocessor
                                7
                            </description>
          <name>CP7</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Access privileges for coprocessor
                                10
                            </description>
          <name>CP10</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Access privileges for coprocessor
                                11
                            </description>
          <name>CP11</name>
        </field>
      </fields>
      <name>CPACR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1AC</addressOffset>
      <description>FP Context Control Register</description>
      <displayName>FPCCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LSPACT</description>
          <name>LSPACT</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USER</description>
          <name>USER</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>THREAD</description>
          <name>THREAD</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>HFRDY</description>
          <name>HFRDY</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>MMRDY</description>
          <name>MMRDY</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>BFRDY</description>
          <name>BFRDY</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>MONRDY</description>
          <name>MONRDY</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LSPEN</description>
          <name>LSPEN</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ASPEN</description>
          <name>ASPEN</name>
        </field>
      </fields>
      <name>FPCCR</name>
      <resetValue>0xC0000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1B0</addressOffset>
      <description>FP Context Address Register</description>
      <displayName>FPCAR</displayName>
      <fields>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>29</bitWidth>
          <description>ADDRESS</description>
          <name>ADDRESS</name>
        </field>
      </fields>
      <name>FPCAR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1B4</addressOffset>
      <description>
                        FP Default Status Control
                        Register
                    </description>
      <displayName>FPDSCR</displayName>
      <fields>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
          <description>RMode</description>
          <name>RMode</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FZ</description>
          <name>FZ</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DN</description>
          <name>DN</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AHP</description>
          <name>AHP</name>
        </field>
      </fields>
      <name>FPDSCR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1B8</addressOffset>
      <description>
                        Media and VFP Feature Register
                        0
                    </description>
      <displayName>MVFR0</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>A_SIMD registers</description>
          <name>A_SIMD</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Single_precision</description>
          <name>Single_precision</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Double_precision</description>
          <name>Double_precision</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
          <description>FP exception trapping</description>
          <name>FP_exception_trapping</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Divide</description>
          <name>Divide</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Square root</description>
          <name>Square_root</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Short vectors</description>
          <name>Short_vectors</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>4</bitWidth>
          <description>FP rounding modes</description>
          <name>FP_rounding_modes</name>
        </field>
      </fields>
      <name>MVFR0</name>
      <resetValue>0x10110021</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1BC</addressOffset>
      <description>
                        Media and VFP Feature Register
                        1
                    </description>
      <displayName>MVFR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>FtZ mode</description>
          <name>FtZ_mode</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>D_NaN mode</description>
          <name>D_NaN_mode</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>FP HPFP</description>
          <name>FP_HPFP</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>4</bitWidth>
          <description>FP fused MAC</description>
          <name>FP_fused_MAC</name>
        </field>
      </fields>
      <name>MVFR1</name>
      <resetValue>0x11000011</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
