/// MOD NVIC
/// Nested Vectored Interrupt Controller
const BASE_ADDRESS: u32 = 0xE000E000;
/// Interrupt Controller Type Register
/// Size: 0x20 bits
pub mod ictr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const INTLINESNUM_BIT_OFFSET: u8 = 0;
	const INTLINESNUM_BIT_WIDTH: u8 = 4;
	/// Total number of interrupt lines in groups (Width: 4, Offset: 0)
	pub fn get_intlinesnum() -> u8 { ::read(REGISTER_ADDRESS, INTLINESNUM_BIT_OFFSET, INTLINESNUM_BIT_WIDTH) as u8 }
}
/// Software Triggered Interrupt Register
/// Size: 0x20 bits
pub mod stir {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xF00;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const INTID_BIT_OFFSET: u8 = 0;
	const INTID_BIT_WIDTH: u8 = 9;
	/// interrupt to be triggered (Width: 9, Offset: 0)
	pub fn set_intid(value: u16) { ::write(REGISTER_ADDRESS, INTID_BIT_OFFSET, INTID_BIT_WIDTH, value as u32); }
}
/// Interrupt Set-Enable Register
/// Size: 0x20 bits
pub mod iser0 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x100;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SETENA_BIT_OFFSET: u8 = 0;
	const SETENA_BIT_WIDTH: u8 = 32;
	/// SETENA (Width: 32, Offset: 0)
	pub fn get_setena() -> u32 { ::read(REGISTER_ADDRESS, SETENA_BIT_OFFSET, SETENA_BIT_WIDTH) as u32 }
	/// SETENA (Width: 32, Offset: 0)
	pub fn set_setena(value: u32) { ::write(REGISTER_ADDRESS, SETENA_BIT_OFFSET, SETENA_BIT_WIDTH, value as u32); }
}
/// Interrupt Set-Enable Register
/// Size: 0x20 bits
pub mod iser1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x104;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SETENA_BIT_OFFSET: u8 = 0;
	const SETENA_BIT_WIDTH: u8 = 32;
	/// SETENA (Width: 32, Offset: 0)
	pub fn get_setena() -> u32 { ::read(REGISTER_ADDRESS, SETENA_BIT_OFFSET, SETENA_BIT_WIDTH) as u32 }
	/// SETENA (Width: 32, Offset: 0)
	pub fn set_setena(value: u32) { ::write(REGISTER_ADDRESS, SETENA_BIT_OFFSET, SETENA_BIT_WIDTH, value as u32); }
}
/// Interrupt Set-Enable Register
/// Size: 0x20 bits
pub mod iser2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x108;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SETENA_BIT_OFFSET: u8 = 0;
	const SETENA_BIT_WIDTH: u8 = 32;
	/// SETENA (Width: 32, Offset: 0)
	pub fn get_setena() -> u32 { ::read(REGISTER_ADDRESS, SETENA_BIT_OFFSET, SETENA_BIT_WIDTH) as u32 }
	/// SETENA (Width: 32, Offset: 0)
	pub fn set_setena(value: u32) { ::write(REGISTER_ADDRESS, SETENA_BIT_OFFSET, SETENA_BIT_WIDTH, value as u32); }
}
/// Interrupt Clear-Enable Register
/// Size: 0x20 bits
pub mod icer0 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x180;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CLRENA_BIT_OFFSET: u8 = 0;
	const CLRENA_BIT_WIDTH: u8 = 32;
	/// CLRENA (Width: 32, Offset: 0)
	pub fn get_clrena() -> u32 { ::read(REGISTER_ADDRESS, CLRENA_BIT_OFFSET, CLRENA_BIT_WIDTH) as u32 }
	/// CLRENA (Width: 32, Offset: 0)
	pub fn set_clrena(value: u32) { ::write(REGISTER_ADDRESS, CLRENA_BIT_OFFSET, CLRENA_BIT_WIDTH, value as u32); }
}
/// Interrupt Clear-Enable Register
/// Size: 0x20 bits
pub mod icer1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x184;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CLRENA_BIT_OFFSET: u8 = 0;
	const CLRENA_BIT_WIDTH: u8 = 32;
	/// CLRENA (Width: 32, Offset: 0)
	pub fn get_clrena() -> u32 { ::read(REGISTER_ADDRESS, CLRENA_BIT_OFFSET, CLRENA_BIT_WIDTH) as u32 }
	/// CLRENA (Width: 32, Offset: 0)
	pub fn set_clrena(value: u32) { ::write(REGISTER_ADDRESS, CLRENA_BIT_OFFSET, CLRENA_BIT_WIDTH, value as u32); }
}
/// Interrupt Clear-Enable Register
/// Size: 0x20 bits
pub mod icer2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x188;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CLRENA_BIT_OFFSET: u8 = 0;
	const CLRENA_BIT_WIDTH: u8 = 32;
	/// CLRENA (Width: 32, Offset: 0)
	pub fn get_clrena() -> u32 { ::read(REGISTER_ADDRESS, CLRENA_BIT_OFFSET, CLRENA_BIT_WIDTH) as u32 }
	/// CLRENA (Width: 32, Offset: 0)
	pub fn set_clrena(value: u32) { ::write(REGISTER_ADDRESS, CLRENA_BIT_OFFSET, CLRENA_BIT_WIDTH, value as u32); }
}
/// Interrupt Set-Pending Register
/// Size: 0x20 bits
pub mod ispr0 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x200;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SETPEND_BIT_OFFSET: u8 = 0;
	const SETPEND_BIT_WIDTH: u8 = 32;
	/// SETPEND (Width: 32, Offset: 0)
	pub fn get_setpend() -> u32 { ::read(REGISTER_ADDRESS, SETPEND_BIT_OFFSET, SETPEND_BIT_WIDTH) as u32 }
	/// SETPEND (Width: 32, Offset: 0)
	pub fn set_setpend(value: u32) { ::write(REGISTER_ADDRESS, SETPEND_BIT_OFFSET, SETPEND_BIT_WIDTH, value as u32); }
}
/// Interrupt Set-Pending Register
/// Size: 0x20 bits
pub mod ispr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x204;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SETPEND_BIT_OFFSET: u8 = 0;
	const SETPEND_BIT_WIDTH: u8 = 32;
	/// SETPEND (Width: 32, Offset: 0)
	pub fn get_setpend() -> u32 { ::read(REGISTER_ADDRESS, SETPEND_BIT_OFFSET, SETPEND_BIT_WIDTH) as u32 }
	/// SETPEND (Width: 32, Offset: 0)
	pub fn set_setpend(value: u32) { ::write(REGISTER_ADDRESS, SETPEND_BIT_OFFSET, SETPEND_BIT_WIDTH, value as u32); }
}
/// Interrupt Set-Pending Register
/// Size: 0x20 bits
pub mod ispr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x208;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SETPEND_BIT_OFFSET: u8 = 0;
	const SETPEND_BIT_WIDTH: u8 = 32;
	/// SETPEND (Width: 32, Offset: 0)
	pub fn get_setpend() -> u32 { ::read(REGISTER_ADDRESS, SETPEND_BIT_OFFSET, SETPEND_BIT_WIDTH) as u32 }
	/// SETPEND (Width: 32, Offset: 0)
	pub fn set_setpend(value: u32) { ::write(REGISTER_ADDRESS, SETPEND_BIT_OFFSET, SETPEND_BIT_WIDTH, value as u32); }
}
/// Interrupt Clear-Pending Register
/// Size: 0x20 bits
pub mod icpr0 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x280;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CLRPEND_BIT_OFFSET: u8 = 0;
	const CLRPEND_BIT_WIDTH: u8 = 32;
	/// CLRPEND (Width: 32, Offset: 0)
	pub fn get_clrpend() -> u32 { ::read(REGISTER_ADDRESS, CLRPEND_BIT_OFFSET, CLRPEND_BIT_WIDTH) as u32 }
	/// CLRPEND (Width: 32, Offset: 0)
	pub fn set_clrpend(value: u32) { ::write(REGISTER_ADDRESS, CLRPEND_BIT_OFFSET, CLRPEND_BIT_WIDTH, value as u32); }
}
/// Interrupt Clear-Pending Register
/// Size: 0x20 bits
pub mod icpr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x284;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CLRPEND_BIT_OFFSET: u8 = 0;
	const CLRPEND_BIT_WIDTH: u8 = 32;
	/// CLRPEND (Width: 32, Offset: 0)
	pub fn get_clrpend() -> u32 { ::read(REGISTER_ADDRESS, CLRPEND_BIT_OFFSET, CLRPEND_BIT_WIDTH) as u32 }
	/// CLRPEND (Width: 32, Offset: 0)
	pub fn set_clrpend(value: u32) { ::write(REGISTER_ADDRESS, CLRPEND_BIT_OFFSET, CLRPEND_BIT_WIDTH, value as u32); }
}
/// Interrupt Clear-Pending Register
/// Size: 0x20 bits
pub mod icpr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x288;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CLRPEND_BIT_OFFSET: u8 = 0;
	const CLRPEND_BIT_WIDTH: u8 = 32;
	/// CLRPEND (Width: 32, Offset: 0)
	pub fn get_clrpend() -> u32 { ::read(REGISTER_ADDRESS, CLRPEND_BIT_OFFSET, CLRPEND_BIT_WIDTH) as u32 }
	/// CLRPEND (Width: 32, Offset: 0)
	pub fn set_clrpend(value: u32) { ::write(REGISTER_ADDRESS, CLRPEND_BIT_OFFSET, CLRPEND_BIT_WIDTH, value as u32); }
}
/// Interrupt Active Bit Register
/// Size: 0x20 bits
pub mod iabr0 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x300;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ACTIVE_BIT_OFFSET: u8 = 0;
	const ACTIVE_BIT_WIDTH: u8 = 32;
	/// ACTIVE (Width: 32, Offset: 0)
	pub fn get_active() -> u32 { ::read(REGISTER_ADDRESS, ACTIVE_BIT_OFFSET, ACTIVE_BIT_WIDTH) as u32 }
}
/// Interrupt Active Bit Register
/// Size: 0x20 bits
pub mod iabr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x304;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ACTIVE_BIT_OFFSET: u8 = 0;
	const ACTIVE_BIT_WIDTH: u8 = 32;
	/// ACTIVE (Width: 32, Offset: 0)
	pub fn get_active() -> u32 { ::read(REGISTER_ADDRESS, ACTIVE_BIT_OFFSET, ACTIVE_BIT_WIDTH) as u32 }
}
/// Interrupt Active Bit Register
/// Size: 0x20 bits
pub mod iabr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x308;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ACTIVE_BIT_OFFSET: u8 = 0;
	const ACTIVE_BIT_WIDTH: u8 = 32;
	/// ACTIVE (Width: 32, Offset: 0)
	pub fn get_active() -> u32 { ::read(REGISTER_ADDRESS, ACTIVE_BIT_OFFSET, ACTIVE_BIT_WIDTH) as u32 }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr0 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x400;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x404;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x408;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr3 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x40C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr4 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x410;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr5 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x414;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr6 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x418;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr7 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x41C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr8 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x420;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr9 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x424;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr10 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x428;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr11 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x42C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr12 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x430;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr13 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x434;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr14 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x438;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr15 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x43C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr16 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x440;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr17 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x444;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr18 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x448;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr19 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x44C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/// Interrupt Priority Register
/// Size: 0x20 bits
pub mod ipr20 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x450;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IPR_N0_BIT_OFFSET: u8 = 0;
	const IPR_N0_BIT_WIDTH: u8 = 8;
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn get_ipr_n0() -> u8 { ::read(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH) as u8 }
	/// IPR_N0 (Width: 8, Offset: 0)
	pub fn set_ipr_n0(value: u8) { ::write(REGISTER_ADDRESS, IPR_N0_BIT_OFFSET, IPR_N0_BIT_WIDTH, value as u32); }

	const IPR_N1_BIT_OFFSET: u8 = 8;
	const IPR_N1_BIT_WIDTH: u8 = 8;
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn get_ipr_n1() -> u8 { ::read(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH) as u8 }
	/// IPR_N1 (Width: 8, Offset: 8)
	pub fn set_ipr_n1(value: u8) { ::write(REGISTER_ADDRESS, IPR_N1_BIT_OFFSET, IPR_N1_BIT_WIDTH, value as u32); }

	const IPR_N2_BIT_OFFSET: u8 = 16;
	const IPR_N2_BIT_WIDTH: u8 = 8;
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn get_ipr_n2() -> u8 { ::read(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH) as u8 }
	/// IPR_N2 (Width: 8, Offset: 16)
	pub fn set_ipr_n2(value: u8) { ::write(REGISTER_ADDRESS, IPR_N2_BIT_OFFSET, IPR_N2_BIT_WIDTH, value as u32); }

	const IPR_N3_BIT_OFFSET: u8 = 24;
	const IPR_N3_BIT_WIDTH: u8 = 8;
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn get_ipr_n3() -> u8 { ::read(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH) as u8 }
	/// IPR_N3 (Width: 8, Offset: 24)
	pub fn set_ipr_n3(value: u8) { ::write(REGISTER_ADDRESS, IPR_N3_BIT_OFFSET, IPR_N3_BIT_WIDTH, value as u32); }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>NVIC</name>
  <description>Nested Vectored Interrupt
      Controller</description>
  <groupName>NVIC</groupName>
  <baseAddress>0xE000E000</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x1001</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>ICTR</name>
      <displayName>ICTR</displayName>
      <description>Interrupt Controller Type
          Register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>INTLINESNUM</name>
          <description>Total number of interrupt lines in
              groups</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>STIR</name>
      <displayName>STIR</displayName>
      <description>Software Triggered Interrupt
          Register</description>
      <addressOffset>0xF00</addressOffset>
      <size>0x20</size>
      <access>write-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>INTID</name>
          <description>interrupt to be triggered</description>
          <bitOffset>0</bitOffset>
          <bitWidth>9</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ISER0</name>
      <displayName>ISER0</displayName>
      <description>Interrupt Set-Enable Register</description>
      <addressOffset>0x100</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SETENA</name>
          <description>SETENA</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ISER1</name>
      <displayName>ISER1</displayName>
      <description>Interrupt Set-Enable Register</description>
      <addressOffset>0x104</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SETENA</name>
          <description>SETENA</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ISER2</name>
      <displayName>ISER2</displayName>
      <description>Interrupt Set-Enable Register</description>
      <addressOffset>0x108</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SETENA</name>
          <description>SETENA</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ICER0</name>
      <displayName>ICER0</displayName>
      <description>Interrupt Clear-Enable
          Register</description>
      <addressOffset>0x180</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CLRENA</name>
          <description>CLRENA</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ICER1</name>
      <displayName>ICER1</displayName>
      <description>Interrupt Clear-Enable
          Register</description>
      <addressOffset>0x184</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CLRENA</name>
          <description>CLRENA</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ICER2</name>
      <displayName>ICER2</displayName>
      <description>Interrupt Clear-Enable
          Register</description>
      <addressOffset>0x188</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CLRENA</name>
          <description>CLRENA</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ISPR0</name>
      <displayName>ISPR0</displayName>
      <description>Interrupt Set-Pending Register</description>
      <addressOffset>0x200</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SETPEND</name>
          <description>SETPEND</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ISPR1</name>
      <displayName>ISPR1</displayName>
      <description>Interrupt Set-Pending Register</description>
      <addressOffset>0x204</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SETPEND</name>
          <description>SETPEND</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ISPR2</name>
      <displayName>ISPR2</displayName>
      <description>Interrupt Set-Pending Register</description>
      <addressOffset>0x208</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SETPEND</name>
          <description>SETPEND</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ICPR0</name>
      <displayName>ICPR0</displayName>
      <description>Interrupt Clear-Pending
          Register</description>
      <addressOffset>0x280</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CLRPEND</name>
          <description>CLRPEND</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ICPR1</name>
      <displayName>ICPR1</displayName>
      <description>Interrupt Clear-Pending
          Register</description>
      <addressOffset>0x284</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CLRPEND</name>
          <description>CLRPEND</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ICPR2</name>
      <displayName>ICPR2</displayName>
      <description>Interrupt Clear-Pending
          Register</description>
      <addressOffset>0x288</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CLRPEND</name>
          <description>CLRPEND</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IABR0</name>
      <displayName>IABR0</displayName>
      <description>Interrupt Active Bit Register</description>
      <addressOffset>0x300</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>ACTIVE</name>
          <description>ACTIVE</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IABR1</name>
      <displayName>IABR1</displayName>
      <description>Interrupt Active Bit Register</description>
      <addressOffset>0x304</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>ACTIVE</name>
          <description>ACTIVE</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IABR2</name>
      <displayName>IABR2</displayName>
      <description>Interrupt Active Bit Register</description>
      <addressOffset>0x308</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>ACTIVE</name>
          <description>ACTIVE</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR0</name>
      <displayName>IPR0</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x400</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR1</name>
      <displayName>IPR1</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x404</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR2</name>
      <displayName>IPR2</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x408</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR3</name>
      <displayName>IPR3</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x40C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR4</name>
      <displayName>IPR4</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x410</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR5</name>
      <displayName>IPR5</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x414</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR6</name>
      <displayName>IPR6</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x418</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR7</name>
      <displayName>IPR7</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x41C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR8</name>
      <displayName>IPR8</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x420</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR9</name>
      <displayName>IPR9</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x424</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR10</name>
      <displayName>IPR10</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x428</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR11</name>
      <displayName>IPR11</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x42C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR12</name>
      <displayName>IPR12</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x430</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR13</name>
      <displayName>IPR13</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x434</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR14</name>
      <displayName>IPR14</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x438</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR15</name>
      <displayName>IPR15</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x43C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR16</name>
      <displayName>IPR16</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x440</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR17</name>
      <displayName>IPR17</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x444</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR18</name>
      <displayName>IPR18</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x448</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR19</name>
      <displayName>IPR19</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x44C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IPR20</name>
      <displayName>IPR20</displayName>
      <description>Interrupt Priority Register</description>
      <addressOffset>0x450</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IPR_N0</name>
          <description>IPR_N0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N1</name>
          <description>IPR_N1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N2</name>
          <description>IPR_N2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>IPR_N3</name>
          <description>IPR_N3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
</peripheral>*/
