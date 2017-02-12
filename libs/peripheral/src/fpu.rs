/// MOD FPU
/// Floting point unit
const BASE_ADDRESS: u32 = 0xE000ED88;
/// Coprocessor Access Control Register
/// Size: 0x20 bits
pub mod cpacr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CP0_BIT_OFFSET: u8 = 0;
	const CP0_BIT_WIDTH: u8 = 1;
	/// Access privileges for coprocessor 0 (Width: 1, Offset: 0)
	pub fn get_cp0() -> u8 { ::read(REGISTER_ADDRESS, CP0_BIT_OFFSET, CP0_BIT_WIDTH) as u8 }
	/// Access privileges for coprocessor 0 (Width: 1, Offset: 0)
	pub fn set_cp0(value: u8) { ::write(REGISTER_ADDRESS, CP0_BIT_OFFSET, CP0_BIT_WIDTH, value as u32); }

	const CP1_BIT_OFFSET: u8 = 2;
	const CP1_BIT_WIDTH: u8 = 1;
	/// Access privileges for coprocessor 1 (Width: 1, Offset: 2)
	pub fn get_cp1() -> u8 { ::read(REGISTER_ADDRESS, CP1_BIT_OFFSET, CP1_BIT_WIDTH) as u8 }
	/// Access privileges for coprocessor 1 (Width: 1, Offset: 2)
	pub fn set_cp1(value: u8) { ::write(REGISTER_ADDRESS, CP1_BIT_OFFSET, CP1_BIT_WIDTH, value as u32); }

	const CP2_BIT_OFFSET: u8 = 4;
	const CP2_BIT_WIDTH: u8 = 1;
	/// Access privileges for coprocessor 2 (Width: 1, Offset: 4)
	pub fn get_cp2() -> u8 { ::read(REGISTER_ADDRESS, CP2_BIT_OFFSET, CP2_BIT_WIDTH) as u8 }
	/// Access privileges for coprocessor 2 (Width: 1, Offset: 4)
	pub fn set_cp2(value: u8) { ::write(REGISTER_ADDRESS, CP2_BIT_OFFSET, CP2_BIT_WIDTH, value as u32); }

	const CP3_BIT_OFFSET: u8 = 6;
	const CP3_BIT_WIDTH: u8 = 1;
	/// Access privileges for coprocessor 3 (Width: 1, Offset: 6)
	pub fn get_cp3() -> u8 { ::read(REGISTER_ADDRESS, CP3_BIT_OFFSET, CP3_BIT_WIDTH) as u8 }
	/// Access privileges for coprocessor 3 (Width: 1, Offset: 6)
	pub fn set_cp3(value: u8) { ::write(REGISTER_ADDRESS, CP3_BIT_OFFSET, CP3_BIT_WIDTH, value as u32); }

	const CP4_BIT_OFFSET: u8 = 8;
	const CP4_BIT_WIDTH: u8 = 1;
	/// Access privileges for coprocessor 4 (Width: 1, Offset: 8)
	pub fn get_cp4() -> u8 { ::read(REGISTER_ADDRESS, CP4_BIT_OFFSET, CP4_BIT_WIDTH) as u8 }
	/// Access privileges for coprocessor 4 (Width: 1, Offset: 8)
	pub fn set_cp4(value: u8) { ::write(REGISTER_ADDRESS, CP4_BIT_OFFSET, CP4_BIT_WIDTH, value as u32); }

	const CP5_BIT_OFFSET: u8 = 10;
	const CP5_BIT_WIDTH: u8 = 1;
	/// Access privileges for coprocessor 5 (Width: 1, Offset: 10)
	pub fn get_cp5() -> u8 { ::read(REGISTER_ADDRESS, CP5_BIT_OFFSET, CP5_BIT_WIDTH) as u8 }
	/// Access privileges for coprocessor 5 (Width: 1, Offset: 10)
	pub fn set_cp5(value: u8) { ::write(REGISTER_ADDRESS, CP5_BIT_OFFSET, CP5_BIT_WIDTH, value as u32); }

	const CP6_BIT_OFFSET: u8 = 12;
	const CP6_BIT_WIDTH: u8 = 2;
	/// Access privileges for coprocessor 6 (Width: 2, Offset: 12)
	pub fn get_cp6() -> u8 { ::read(REGISTER_ADDRESS, CP6_BIT_OFFSET, CP6_BIT_WIDTH) as u8 }
	/// Access privileges for coprocessor 6 (Width: 2, Offset: 12)
	pub fn set_cp6(value: u8) { ::write(REGISTER_ADDRESS, CP6_BIT_OFFSET, CP6_BIT_WIDTH, value as u32); }

	const CP7_BIT_OFFSET: u8 = 14;
	const CP7_BIT_WIDTH: u8 = 1;
	/// Access privileges for coprocessor 7 (Width: 1, Offset: 14)
	pub fn get_cp7() -> u8 { ::read(REGISTER_ADDRESS, CP7_BIT_OFFSET, CP7_BIT_WIDTH) as u8 }
	/// Access privileges for coprocessor 7 (Width: 1, Offset: 14)
	pub fn set_cp7(value: u8) { ::write(REGISTER_ADDRESS, CP7_BIT_OFFSET, CP7_BIT_WIDTH, value as u32); }

	const CP10_BIT_OFFSET: u8 = 20;
	const CP10_BIT_WIDTH: u8 = 1;
	/// Access privileges for coprocessor 10 (Width: 1, Offset: 20)
	pub fn get_cp10() -> u8 { ::read(REGISTER_ADDRESS, CP10_BIT_OFFSET, CP10_BIT_WIDTH) as u8 }
	/// Access privileges for coprocessor 10 (Width: 1, Offset: 20)
	pub fn set_cp10(value: u8) { ::write(REGISTER_ADDRESS, CP10_BIT_OFFSET, CP10_BIT_WIDTH, value as u32); }

	const CP11_BIT_OFFSET: u8 = 22;
	const CP11_BIT_WIDTH: u8 = 1;
	/// Access privileges for coprocessor 11 (Width: 1, Offset: 22)
	pub fn get_cp11() -> u8 { ::read(REGISTER_ADDRESS, CP11_BIT_OFFSET, CP11_BIT_WIDTH) as u8 }
	/// Access privileges for coprocessor 11 (Width: 1, Offset: 22)
	pub fn set_cp11(value: u8) { ::write(REGISTER_ADDRESS, CP11_BIT_OFFSET, CP11_BIT_WIDTH, value as u32); }
}
/// FP Context Control Register
/// Size: 0x20 bits
pub mod fpccr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1AC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const LSPACT_BIT_OFFSET: u8 = 0;
	const LSPACT_BIT_WIDTH: u8 = 1;
	/// LSPACT (Width: 1, Offset: 0)
	pub fn get_lspact() -> u8 { ::read(REGISTER_ADDRESS, LSPACT_BIT_OFFSET, LSPACT_BIT_WIDTH) as u8 }
	/// LSPACT (Width: 1, Offset: 0)
	pub fn set_lspact(value: u8) { ::write(REGISTER_ADDRESS, LSPACT_BIT_OFFSET, LSPACT_BIT_WIDTH, value as u32); }

	const USER_BIT_OFFSET: u8 = 1;
	const USER_BIT_WIDTH: u8 = 1;
	/// USER (Width: 1, Offset: 1)
	pub fn get_user() -> u8 { ::read(REGISTER_ADDRESS, USER_BIT_OFFSET, USER_BIT_WIDTH) as u8 }
	/// USER (Width: 1, Offset: 1)
	pub fn set_user(value: u8) { ::write(REGISTER_ADDRESS, USER_BIT_OFFSET, USER_BIT_WIDTH, value as u32); }

	const THREAD_BIT_OFFSET: u8 = 3;
	const THREAD_BIT_WIDTH: u8 = 1;
	/// THREAD (Width: 1, Offset: 3)
	pub fn get_thread() -> u8 { ::read(REGISTER_ADDRESS, THREAD_BIT_OFFSET, THREAD_BIT_WIDTH) as u8 }
	/// THREAD (Width: 1, Offset: 3)
	pub fn set_thread(value: u8) { ::write(REGISTER_ADDRESS, THREAD_BIT_OFFSET, THREAD_BIT_WIDTH, value as u32); }

	const HFRDY_BIT_OFFSET: u8 = 4;
	const HFRDY_BIT_WIDTH: u8 = 1;
	/// HFRDY (Width: 1, Offset: 4)
	pub fn get_hfrdy() -> u8 { ::read(REGISTER_ADDRESS, HFRDY_BIT_OFFSET, HFRDY_BIT_WIDTH) as u8 }
	/// HFRDY (Width: 1, Offset: 4)
	pub fn set_hfrdy(value: u8) { ::write(REGISTER_ADDRESS, HFRDY_BIT_OFFSET, HFRDY_BIT_WIDTH, value as u32); }

	const MMRDY_BIT_OFFSET: u8 = 5;
	const MMRDY_BIT_WIDTH: u8 = 1;
	/// MMRDY (Width: 1, Offset: 5)
	pub fn get_mmrdy() -> u8 { ::read(REGISTER_ADDRESS, MMRDY_BIT_OFFSET, MMRDY_BIT_WIDTH) as u8 }
	/// MMRDY (Width: 1, Offset: 5)
	pub fn set_mmrdy(value: u8) { ::write(REGISTER_ADDRESS, MMRDY_BIT_OFFSET, MMRDY_BIT_WIDTH, value as u32); }

	const BFRDY_BIT_OFFSET: u8 = 6;
	const BFRDY_BIT_WIDTH: u8 = 1;
	/// BFRDY (Width: 1, Offset: 6)
	pub fn get_bfrdy() -> u8 { ::read(REGISTER_ADDRESS, BFRDY_BIT_OFFSET, BFRDY_BIT_WIDTH) as u8 }
	/// BFRDY (Width: 1, Offset: 6)
	pub fn set_bfrdy(value: u8) { ::write(REGISTER_ADDRESS, BFRDY_BIT_OFFSET, BFRDY_BIT_WIDTH, value as u32); }

	const MONRDY_BIT_OFFSET: u8 = 8;
	const MONRDY_BIT_WIDTH: u8 = 1;
	/// MONRDY (Width: 1, Offset: 8)
	pub fn get_monrdy() -> u8 { ::read(REGISTER_ADDRESS, MONRDY_BIT_OFFSET, MONRDY_BIT_WIDTH) as u8 }
	/// MONRDY (Width: 1, Offset: 8)
	pub fn set_monrdy(value: u8) { ::write(REGISTER_ADDRESS, MONRDY_BIT_OFFSET, MONRDY_BIT_WIDTH, value as u32); }

	const LSPEN_BIT_OFFSET: u8 = 30;
	const LSPEN_BIT_WIDTH: u8 = 1;
	/// LSPEN (Width: 1, Offset: 30)
	pub fn get_lspen() -> u8 { ::read(REGISTER_ADDRESS, LSPEN_BIT_OFFSET, LSPEN_BIT_WIDTH) as u8 }
	/// LSPEN (Width: 1, Offset: 30)
	pub fn set_lspen(value: u8) { ::write(REGISTER_ADDRESS, LSPEN_BIT_OFFSET, LSPEN_BIT_WIDTH, value as u32); }

	const ASPEN_BIT_OFFSET: u8 = 31;
	const ASPEN_BIT_WIDTH: u8 = 1;
	/// ASPEN (Width: 1, Offset: 31)
	pub fn get_aspen() -> u8 { ::read(REGISTER_ADDRESS, ASPEN_BIT_OFFSET, ASPEN_BIT_WIDTH) as u8 }
	/// ASPEN (Width: 1, Offset: 31)
	pub fn set_aspen(value: u8) { ::write(REGISTER_ADDRESS, ASPEN_BIT_OFFSET, ASPEN_BIT_WIDTH, value as u32); }
}
/// FP Context Address Register
/// Size: 0x20 bits
pub mod fpcar {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1B0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ADDRESS_BIT_OFFSET: u8 = 3;
	const ADDRESS_BIT_WIDTH: u8 = 29;
	/// ADDRESS (Width: 29, Offset: 3)
	pub fn get_address() -> u32 { ::read(REGISTER_ADDRESS, ADDRESS_BIT_OFFSET, ADDRESS_BIT_WIDTH) as u32 }
	/// ADDRESS (Width: 29, Offset: 3)
	pub fn set_address(value: u32) { ::write(REGISTER_ADDRESS, ADDRESS_BIT_OFFSET, ADDRESS_BIT_WIDTH, value as u32); }
}
/// FP Default Status Control Register
/// Size: 0x20 bits
pub mod fpdscr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1B4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const RMode_BIT_OFFSET: u8 = 22;
	const RMode_BIT_WIDTH: u8 = 2;
	/// RMode (Width: 2, Offset: 22)
	pub fn get_rmode() -> u8 { ::read(REGISTER_ADDRESS, RMode_BIT_OFFSET, RMode_BIT_WIDTH) as u8 }
	/// RMode (Width: 2, Offset: 22)
	pub fn set_rmode(value: u8) { ::write(REGISTER_ADDRESS, RMode_BIT_OFFSET, RMode_BIT_WIDTH, value as u32); }

	const FZ_BIT_OFFSET: u8 = 24;
	const FZ_BIT_WIDTH: u8 = 1;
	/// FZ (Width: 1, Offset: 24)
	pub fn get_fz() -> u8 { ::read(REGISTER_ADDRESS, FZ_BIT_OFFSET, FZ_BIT_WIDTH) as u8 }
	/// FZ (Width: 1, Offset: 24)
	pub fn set_fz(value: u8) { ::write(REGISTER_ADDRESS, FZ_BIT_OFFSET, FZ_BIT_WIDTH, value as u32); }

	const DN_BIT_OFFSET: u8 = 25;
	const DN_BIT_WIDTH: u8 = 1;
	/// DN (Width: 1, Offset: 25)
	pub fn get_dn() -> u8 { ::read(REGISTER_ADDRESS, DN_BIT_OFFSET, DN_BIT_WIDTH) as u8 }
	/// DN (Width: 1, Offset: 25)
	pub fn set_dn(value: u8) { ::write(REGISTER_ADDRESS, DN_BIT_OFFSET, DN_BIT_WIDTH, value as u32); }

	const AHP_BIT_OFFSET: u8 = 26;
	const AHP_BIT_WIDTH: u8 = 1;
	/// AHP (Width: 1, Offset: 26)
	pub fn get_ahp() -> u8 { ::read(REGISTER_ADDRESS, AHP_BIT_OFFSET, AHP_BIT_WIDTH) as u8 }
	/// AHP (Width: 1, Offset: 26)
	pub fn set_ahp(value: u8) { ::write(REGISTER_ADDRESS, AHP_BIT_OFFSET, AHP_BIT_WIDTH, value as u32); }
}
/// Media and VFP Feature Register 0
/// Size: 0x20 bits
pub mod mvfr0 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1B8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const A_SIMD_BIT_OFFSET: u8 = 0;
	const A_SIMD_BIT_WIDTH: u8 = 4;
	/// A_SIMD registers (Width: 4, Offset: 0)
	pub fn get_a_simd() -> u8 { ::read(REGISTER_ADDRESS, A_SIMD_BIT_OFFSET, A_SIMD_BIT_WIDTH) as u8 }

	const Single_precision_BIT_OFFSET: u8 = 4;
	const Single_precision_BIT_WIDTH: u8 = 4;
	/// Single_precision (Width: 4, Offset: 4)
	pub fn get_single_precision() -> u8 { ::read(REGISTER_ADDRESS, Single_precision_BIT_OFFSET, Single_precision_BIT_WIDTH) as u8 }

	const Double_precision_BIT_OFFSET: u8 = 8;
	const Double_precision_BIT_WIDTH: u8 = 4;
	/// Double_precision (Width: 4, Offset: 8)
	pub fn get_double_precision() -> u8 { ::read(REGISTER_ADDRESS, Double_precision_BIT_OFFSET, Double_precision_BIT_WIDTH) as u8 }

	const FP_exception_trapping_BIT_OFFSET: u8 = 12;
	const FP_exception_trapping_BIT_WIDTH: u8 = 4;
	/// FP exception trapping (Width: 4, Offset: 12)
	pub fn get_fp_exception_trapping() -> u8 { ::read(REGISTER_ADDRESS, FP_exception_trapping_BIT_OFFSET, FP_exception_trapping_BIT_WIDTH) as u8 }

	const Divide_BIT_OFFSET: u8 = 16;
	const Divide_BIT_WIDTH: u8 = 4;
	/// Divide (Width: 4, Offset: 16)
	pub fn get_divide() -> u8 { ::read(REGISTER_ADDRESS, Divide_BIT_OFFSET, Divide_BIT_WIDTH) as u8 }

	const Square_root_BIT_OFFSET: u8 = 20;
	const Square_root_BIT_WIDTH: u8 = 4;
	/// Square root (Width: 4, Offset: 20)
	pub fn get_square_root() -> u8 { ::read(REGISTER_ADDRESS, Square_root_BIT_OFFSET, Square_root_BIT_WIDTH) as u8 }

	const Short_vectors_BIT_OFFSET: u8 = 24;
	const Short_vectors_BIT_WIDTH: u8 = 4;
	/// Short vectors (Width: 4, Offset: 24)
	pub fn get_short_vectors() -> u8 { ::read(REGISTER_ADDRESS, Short_vectors_BIT_OFFSET, Short_vectors_BIT_WIDTH) as u8 }

	const FP_rounding_modes_BIT_OFFSET: u8 = 28;
	const FP_rounding_modes_BIT_WIDTH: u8 = 4;
	/// FP rounding modes (Width: 4, Offset: 28)
	pub fn get_fp_rounding_modes() -> u8 { ::read(REGISTER_ADDRESS, FP_rounding_modes_BIT_OFFSET, FP_rounding_modes_BIT_WIDTH) as u8 }
}
/// Media and VFP Feature Register 1
/// Size: 0x20 bits
pub mod mvfr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1BC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const FtZ_mode_BIT_OFFSET: u8 = 0;
	const FtZ_mode_BIT_WIDTH: u8 = 4;
	/// FtZ mode (Width: 4, Offset: 0)
	pub fn get_ftz_mode() -> u8 { ::read(REGISTER_ADDRESS, FtZ_mode_BIT_OFFSET, FtZ_mode_BIT_WIDTH) as u8 }

	const D_NaN_mode_BIT_OFFSET: u8 = 4;
	const D_NaN_mode_BIT_WIDTH: u8 = 4;
	/// D_NaN mode (Width: 4, Offset: 4)
	pub fn get_d_nan_mode() -> u8 { ::read(REGISTER_ADDRESS, D_NaN_mode_BIT_OFFSET, D_NaN_mode_BIT_WIDTH) as u8 }

	const FP_HPFP_BIT_OFFSET: u8 = 24;
	const FP_HPFP_BIT_WIDTH: u8 = 4;
	/// FP HPFP (Width: 4, Offset: 24)
	pub fn get_fp_hpfp() -> u8 { ::read(REGISTER_ADDRESS, FP_HPFP_BIT_OFFSET, FP_HPFP_BIT_WIDTH) as u8 }

	const FP_fused_MAC_BIT_OFFSET: u8 = 28;
	const FP_fused_MAC_BIT_WIDTH: u8 = 4;
	/// FP fused MAC (Width: 4, Offset: 28)
	pub fn get_fp_fused_mac() -> u8 { ::read(REGISTER_ADDRESS, FP_fused_MAC_BIT_OFFSET, FP_fused_MAC_BIT_WIDTH) as u8 }
}
/// Floating point interrupt
pub const INTERRUPT_FPU: u32 = 81;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>FPU</name>
  <description>Floting point unit</description>
  <groupName>FPU</groupName>
  <baseAddress>0xE000ED88</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x200</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>CPACR</name>
      <displayName>CPACR</displayName>
      <description>Coprocessor Access Control
          Register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CP0</name>
          <description>Access privileges for coprocessor
              0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CP1</name>
          <description>Access privileges for coprocessor
              1</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CP2</name>
          <description>Access privileges for coprocessor
              2</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CP3</name>
          <description>Access privileges for coprocessor
              3</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CP4</name>
          <description>Access privileges for coprocessor
              4</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CP5</name>
          <description>Access privileges for coprocessor
              5</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CP6</name>
          <description>Access privileges for coprocessor
              6</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>CP7</name>
          <description>Access privileges for coprocessor
              7</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CP10</name>
          <description>Access privileges for coprocessor
              10</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CP11</name>
          <description>Access privileges for coprocessor
              11</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>FPCCR</name>
      <displayName>FPCCR</displayName>
      <description>FP Context Control Register</description>
      <addressOffset>0x1AC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0xC0000000</resetValue>
      <fields>
        <field>
          <name>LSPACT</name>
          <description>LSPACT</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>USER</name>
          <description>USER</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>THREAD</name>
          <description>THREAD</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HFRDY</name>
          <description>HFRDY</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MMRDY</name>
          <description>MMRDY</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BFRDY</name>
          <description>BFRDY</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MONRDY</name>
          <description>MONRDY</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LSPEN</name>
          <description>LSPEN</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ASPEN</name>
          <description>ASPEN</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>FPCAR</name>
      <displayName>FPCAR</displayName>
      <description>FP Context Address Register</description>
      <addressOffset>0x1B0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>ADDRESS</name>
          <description>ADDRESS</description>
          <bitOffset>3</bitOffset>
          <bitWidth>29</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>FPDSCR</name>
      <displayName>FPDSCR</displayName>
      <description>FP Default Status Control
          Register</description>
      <addressOffset>0x1B4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>RMode</name>
          <description>RMode</description>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>FZ</name>
          <description>FZ</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DN</name>
          <description>DN</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AHP</name>
          <description>AHP</description>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>MVFR0</name>
      <displayName>MVFR0</displayName>
      <description>Media and VFP Feature Register
          0</description>
      <addressOffset>0x1B8</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x10110021</resetValue>
      <fields>
        <field>
          <name>A_SIMD</name>
          <description>A_SIMD registers</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>Single_precision</name>
          <description>Single_precision</description>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>Double_precision</name>
          <description>Double_precision</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>FP_exception_trapping</name>
          <description>FP exception trapping</description>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>Divide</name>
          <description>Divide</description>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>Square_root</name>
          <description>Square root</description>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>Short_vectors</name>
          <description>Short vectors</description>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>FP_rounding_modes</name>
          <description>FP rounding modes</description>
          <bitOffset>28</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>MVFR1</name>
      <displayName>MVFR1</displayName>
      <description>Media and VFP Feature Register
          1</description>
      <addressOffset>0x1BC</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x11000011</resetValue>
      <fields>
        <field>
          <name>FtZ_mode</name>
          <description>FtZ mode</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>D_NaN_mode</name>
          <description>D_NaN mode</description>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>FP_HPFP</name>
          <description>FP HPFP</description>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>FP_fused_MAC</name>
          <description>FP fused MAC</description>
          <bitOffset>28</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>FPU</name>
    <description>Floating point interrupt</description>
    <value>81</value>
  </interrupt>
</peripheral>*/
