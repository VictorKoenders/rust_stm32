/// MOD GPIOE
/// General-purpose I/Os
const BASE_ADDRESS: u32 = 0x48001000;
/// GPIO port mode register
/// Size: 0x20 bits
pub mod moder {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MODER15_BIT_OFFSET: u8 = 30;
	const MODER15_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 30)
	pub fn get_moder15() -> u8 { ::read(REGISTER_ADDRESS, MODER15_BIT_OFFSET, MODER15_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 30)
	pub fn set_moder15(value: u8) { ::write(REGISTER_ADDRESS, MODER15_BIT_OFFSET, MODER15_BIT_WIDTH, value as u32); }

	const MODER14_BIT_OFFSET: u8 = 28;
	const MODER14_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 28)
	pub fn get_moder14() -> u8 { ::read(REGISTER_ADDRESS, MODER14_BIT_OFFSET, MODER14_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 28)
	pub fn set_moder14(value: u8) { ::write(REGISTER_ADDRESS, MODER14_BIT_OFFSET, MODER14_BIT_WIDTH, value as u32); }

	const MODER13_BIT_OFFSET: u8 = 26;
	const MODER13_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 26)
	pub fn get_moder13() -> u8 { ::read(REGISTER_ADDRESS, MODER13_BIT_OFFSET, MODER13_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 26)
	pub fn set_moder13(value: u8) { ::write(REGISTER_ADDRESS, MODER13_BIT_OFFSET, MODER13_BIT_WIDTH, value as u32); }

	const MODER12_BIT_OFFSET: u8 = 24;
	const MODER12_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 24)
	pub fn get_moder12() -> u8 { ::read(REGISTER_ADDRESS, MODER12_BIT_OFFSET, MODER12_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 24)
	pub fn set_moder12(value: u8) { ::write(REGISTER_ADDRESS, MODER12_BIT_OFFSET, MODER12_BIT_WIDTH, value as u32); }

	const MODER11_BIT_OFFSET: u8 = 22;
	const MODER11_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 22)
	pub fn get_moder11() -> u8 { ::read(REGISTER_ADDRESS, MODER11_BIT_OFFSET, MODER11_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 22)
	pub fn set_moder11(value: u8) { ::write(REGISTER_ADDRESS, MODER11_BIT_OFFSET, MODER11_BIT_WIDTH, value as u32); }

	const MODER10_BIT_OFFSET: u8 = 20;
	const MODER10_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 20)
	pub fn get_moder10() -> u8 { ::read(REGISTER_ADDRESS, MODER10_BIT_OFFSET, MODER10_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 20)
	pub fn set_moder10(value: u8) { ::write(REGISTER_ADDRESS, MODER10_BIT_OFFSET, MODER10_BIT_WIDTH, value as u32); }

	const MODER9_BIT_OFFSET: u8 = 18;
	const MODER9_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 18)
	pub fn get_moder9() -> u8 { ::read(REGISTER_ADDRESS, MODER9_BIT_OFFSET, MODER9_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 18)
	pub fn set_moder9(value: u8) { ::write(REGISTER_ADDRESS, MODER9_BIT_OFFSET, MODER9_BIT_WIDTH, value as u32); }

	const MODER8_BIT_OFFSET: u8 = 16;
	const MODER8_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 16)
	pub fn get_moder8() -> u8 { ::read(REGISTER_ADDRESS, MODER8_BIT_OFFSET, MODER8_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 16)
	pub fn set_moder8(value: u8) { ::write(REGISTER_ADDRESS, MODER8_BIT_OFFSET, MODER8_BIT_WIDTH, value as u32); }

	const MODER7_BIT_OFFSET: u8 = 14;
	const MODER7_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 14)
	pub fn get_moder7() -> u8 { ::read(REGISTER_ADDRESS, MODER7_BIT_OFFSET, MODER7_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 14)
	pub fn set_moder7(value: u8) { ::write(REGISTER_ADDRESS, MODER7_BIT_OFFSET, MODER7_BIT_WIDTH, value as u32); }

	const MODER6_BIT_OFFSET: u8 = 12;
	const MODER6_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 12)
	pub fn get_moder6() -> u8 { ::read(REGISTER_ADDRESS, MODER6_BIT_OFFSET, MODER6_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 12)
	pub fn set_moder6(value: u8) { ::write(REGISTER_ADDRESS, MODER6_BIT_OFFSET, MODER6_BIT_WIDTH, value as u32); }

	const MODER5_BIT_OFFSET: u8 = 10;
	const MODER5_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 10)
	pub fn get_moder5() -> u8 { ::read(REGISTER_ADDRESS, MODER5_BIT_OFFSET, MODER5_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 10)
	pub fn set_moder5(value: u8) { ::write(REGISTER_ADDRESS, MODER5_BIT_OFFSET, MODER5_BIT_WIDTH, value as u32); }

	const MODER4_BIT_OFFSET: u8 = 8;
	const MODER4_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 8)
	pub fn get_moder4() -> u8 { ::read(REGISTER_ADDRESS, MODER4_BIT_OFFSET, MODER4_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 8)
	pub fn set_moder4(value: u8) { ::write(REGISTER_ADDRESS, MODER4_BIT_OFFSET, MODER4_BIT_WIDTH, value as u32); }

	const MODER3_BIT_OFFSET: u8 = 6;
	const MODER3_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 6)
	pub fn get_moder3() -> u8 { ::read(REGISTER_ADDRESS, MODER3_BIT_OFFSET, MODER3_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 6)
	pub fn set_moder3(value: u8) { ::write(REGISTER_ADDRESS, MODER3_BIT_OFFSET, MODER3_BIT_WIDTH, value as u32); }

	const MODER2_BIT_OFFSET: u8 = 4;
	const MODER2_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 4)
	pub fn get_moder2() -> u8 { ::read(REGISTER_ADDRESS, MODER2_BIT_OFFSET, MODER2_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 4)
	pub fn set_moder2(value: u8) { ::write(REGISTER_ADDRESS, MODER2_BIT_OFFSET, MODER2_BIT_WIDTH, value as u32); }

	const MODER1_BIT_OFFSET: u8 = 2;
	const MODER1_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 2)
	pub fn get_moder1() -> u8 { ::read(REGISTER_ADDRESS, MODER1_BIT_OFFSET, MODER1_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 2)
	pub fn set_moder1(value: u8) { ::write(REGISTER_ADDRESS, MODER1_BIT_OFFSET, MODER1_BIT_WIDTH, value as u32); }

	const MODER0_BIT_OFFSET: u8 = 0;
	const MODER0_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 0)
	pub fn get_moder0() -> u8 { ::read(REGISTER_ADDRESS, MODER0_BIT_OFFSET, MODER0_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 0)
	pub fn set_moder0(value: u8) { ::write(REGISTER_ADDRESS, MODER0_BIT_OFFSET, MODER0_BIT_WIDTH, value as u32); }
}
/// GPIO port output type register
/// Size: 0x20 bits
pub mod otyper {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OT15_BIT_OFFSET: u8 = 15;
	const OT15_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 15 (Width: 1, Offset: 15)
	pub fn get_ot15() -> u8 { ::read(REGISTER_ADDRESS, OT15_BIT_OFFSET, OT15_BIT_WIDTH) as u8 }
	/// Port x configuration bit 15 (Width: 1, Offset: 15)
	pub fn set_ot15(value: u8) { ::write(REGISTER_ADDRESS, OT15_BIT_OFFSET, OT15_BIT_WIDTH, value as u32); }

	const OT14_BIT_OFFSET: u8 = 14;
	const OT14_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 14 (Width: 1, Offset: 14)
	pub fn get_ot14() -> u8 { ::read(REGISTER_ADDRESS, OT14_BIT_OFFSET, OT14_BIT_WIDTH) as u8 }
	/// Port x configuration bit 14 (Width: 1, Offset: 14)
	pub fn set_ot14(value: u8) { ::write(REGISTER_ADDRESS, OT14_BIT_OFFSET, OT14_BIT_WIDTH, value as u32); }

	const OT13_BIT_OFFSET: u8 = 13;
	const OT13_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 13 (Width: 1, Offset: 13)
	pub fn get_ot13() -> u8 { ::read(REGISTER_ADDRESS, OT13_BIT_OFFSET, OT13_BIT_WIDTH) as u8 }
	/// Port x configuration bit 13 (Width: 1, Offset: 13)
	pub fn set_ot13(value: u8) { ::write(REGISTER_ADDRESS, OT13_BIT_OFFSET, OT13_BIT_WIDTH, value as u32); }

	const OT12_BIT_OFFSET: u8 = 12;
	const OT12_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 12 (Width: 1, Offset: 12)
	pub fn get_ot12() -> u8 { ::read(REGISTER_ADDRESS, OT12_BIT_OFFSET, OT12_BIT_WIDTH) as u8 }
	/// Port x configuration bit 12 (Width: 1, Offset: 12)
	pub fn set_ot12(value: u8) { ::write(REGISTER_ADDRESS, OT12_BIT_OFFSET, OT12_BIT_WIDTH, value as u32); }

	const OT11_BIT_OFFSET: u8 = 11;
	const OT11_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 11 (Width: 1, Offset: 11)
	pub fn get_ot11() -> u8 { ::read(REGISTER_ADDRESS, OT11_BIT_OFFSET, OT11_BIT_WIDTH) as u8 }
	/// Port x configuration bit 11 (Width: 1, Offset: 11)
	pub fn set_ot11(value: u8) { ::write(REGISTER_ADDRESS, OT11_BIT_OFFSET, OT11_BIT_WIDTH, value as u32); }

	const OT10_BIT_OFFSET: u8 = 10;
	const OT10_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 10 (Width: 1, Offset: 10)
	pub fn get_ot10() -> u8 { ::read(REGISTER_ADDRESS, OT10_BIT_OFFSET, OT10_BIT_WIDTH) as u8 }
	/// Port x configuration bit 10 (Width: 1, Offset: 10)
	pub fn set_ot10(value: u8) { ::write(REGISTER_ADDRESS, OT10_BIT_OFFSET, OT10_BIT_WIDTH, value as u32); }

	const OT9_BIT_OFFSET: u8 = 9;
	const OT9_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 9 (Width: 1, Offset: 9)
	pub fn get_ot9() -> u8 { ::read(REGISTER_ADDRESS, OT9_BIT_OFFSET, OT9_BIT_WIDTH) as u8 }
	/// Port x configuration bit 9 (Width: 1, Offset: 9)
	pub fn set_ot9(value: u8) { ::write(REGISTER_ADDRESS, OT9_BIT_OFFSET, OT9_BIT_WIDTH, value as u32); }

	const OT8_BIT_OFFSET: u8 = 8;
	const OT8_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 8 (Width: 1, Offset: 8)
	pub fn get_ot8() -> u8 { ::read(REGISTER_ADDRESS, OT8_BIT_OFFSET, OT8_BIT_WIDTH) as u8 }
	/// Port x configuration bit 8 (Width: 1, Offset: 8)
	pub fn set_ot8(value: u8) { ::write(REGISTER_ADDRESS, OT8_BIT_OFFSET, OT8_BIT_WIDTH, value as u32); }

	const OT7_BIT_OFFSET: u8 = 7;
	const OT7_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 7 (Width: 1, Offset: 7)
	pub fn get_ot7() -> u8 { ::read(REGISTER_ADDRESS, OT7_BIT_OFFSET, OT7_BIT_WIDTH) as u8 }
	/// Port x configuration bit 7 (Width: 1, Offset: 7)
	pub fn set_ot7(value: u8) { ::write(REGISTER_ADDRESS, OT7_BIT_OFFSET, OT7_BIT_WIDTH, value as u32); }

	const OT6_BIT_OFFSET: u8 = 6;
	const OT6_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 6 (Width: 1, Offset: 6)
	pub fn get_ot6() -> u8 { ::read(REGISTER_ADDRESS, OT6_BIT_OFFSET, OT6_BIT_WIDTH) as u8 }
	/// Port x configuration bit 6 (Width: 1, Offset: 6)
	pub fn set_ot6(value: u8) { ::write(REGISTER_ADDRESS, OT6_BIT_OFFSET, OT6_BIT_WIDTH, value as u32); }

	const OT5_BIT_OFFSET: u8 = 5;
	const OT5_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 5 (Width: 1, Offset: 5)
	pub fn get_ot5() -> u8 { ::read(REGISTER_ADDRESS, OT5_BIT_OFFSET, OT5_BIT_WIDTH) as u8 }
	/// Port x configuration bit 5 (Width: 1, Offset: 5)
	pub fn set_ot5(value: u8) { ::write(REGISTER_ADDRESS, OT5_BIT_OFFSET, OT5_BIT_WIDTH, value as u32); }

	const OT4_BIT_OFFSET: u8 = 4;
	const OT4_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 4 (Width: 1, Offset: 4)
	pub fn get_ot4() -> u8 { ::read(REGISTER_ADDRESS, OT4_BIT_OFFSET, OT4_BIT_WIDTH) as u8 }
	/// Port x configuration bit 4 (Width: 1, Offset: 4)
	pub fn set_ot4(value: u8) { ::write(REGISTER_ADDRESS, OT4_BIT_OFFSET, OT4_BIT_WIDTH, value as u32); }

	const OT3_BIT_OFFSET: u8 = 3;
	const OT3_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 3 (Width: 1, Offset: 3)
	pub fn get_ot3() -> u8 { ::read(REGISTER_ADDRESS, OT3_BIT_OFFSET, OT3_BIT_WIDTH) as u8 }
	/// Port x configuration bit 3 (Width: 1, Offset: 3)
	pub fn set_ot3(value: u8) { ::write(REGISTER_ADDRESS, OT3_BIT_OFFSET, OT3_BIT_WIDTH, value as u32); }

	const OT2_BIT_OFFSET: u8 = 2;
	const OT2_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 2 (Width: 1, Offset: 2)
	pub fn get_ot2() -> u8 { ::read(REGISTER_ADDRESS, OT2_BIT_OFFSET, OT2_BIT_WIDTH) as u8 }
	/// Port x configuration bit 2 (Width: 1, Offset: 2)
	pub fn set_ot2(value: u8) { ::write(REGISTER_ADDRESS, OT2_BIT_OFFSET, OT2_BIT_WIDTH, value as u32); }

	const OT1_BIT_OFFSET: u8 = 1;
	const OT1_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 1 (Width: 1, Offset: 1)
	pub fn get_ot1() -> u8 { ::read(REGISTER_ADDRESS, OT1_BIT_OFFSET, OT1_BIT_WIDTH) as u8 }
	/// Port x configuration bit 1 (Width: 1, Offset: 1)
	pub fn set_ot1(value: u8) { ::write(REGISTER_ADDRESS, OT1_BIT_OFFSET, OT1_BIT_WIDTH, value as u32); }

	const OT0_BIT_OFFSET: u8 = 0;
	const OT0_BIT_WIDTH: u8 = 1;
	/// Port x configuration bit 0 (Width: 1, Offset: 0)
	pub fn get_ot0() -> u8 { ::read(REGISTER_ADDRESS, OT0_BIT_OFFSET, OT0_BIT_WIDTH) as u8 }
	/// Port x configuration bit 0 (Width: 1, Offset: 0)
	pub fn set_ot0(value: u8) { ::write(REGISTER_ADDRESS, OT0_BIT_OFFSET, OT0_BIT_WIDTH, value as u32); }
}
/// GPIO port output speed register
/// Size: 0x20 bits
pub mod ospeedr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OSPEEDR15_BIT_OFFSET: u8 = 30;
	const OSPEEDR15_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 30)
	pub fn get_ospeedr15() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR15_BIT_OFFSET, OSPEEDR15_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 30)
	pub fn set_ospeedr15(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR15_BIT_OFFSET, OSPEEDR15_BIT_WIDTH, value as u32); }

	const OSPEEDR14_BIT_OFFSET: u8 = 28;
	const OSPEEDR14_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 28)
	pub fn get_ospeedr14() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR14_BIT_OFFSET, OSPEEDR14_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 28)
	pub fn set_ospeedr14(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR14_BIT_OFFSET, OSPEEDR14_BIT_WIDTH, value as u32); }

	const OSPEEDR13_BIT_OFFSET: u8 = 26;
	const OSPEEDR13_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 26)
	pub fn get_ospeedr13() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR13_BIT_OFFSET, OSPEEDR13_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 26)
	pub fn set_ospeedr13(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR13_BIT_OFFSET, OSPEEDR13_BIT_WIDTH, value as u32); }

	const OSPEEDR12_BIT_OFFSET: u8 = 24;
	const OSPEEDR12_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 24)
	pub fn get_ospeedr12() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR12_BIT_OFFSET, OSPEEDR12_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 24)
	pub fn set_ospeedr12(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR12_BIT_OFFSET, OSPEEDR12_BIT_WIDTH, value as u32); }

	const OSPEEDR11_BIT_OFFSET: u8 = 22;
	const OSPEEDR11_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 22)
	pub fn get_ospeedr11() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR11_BIT_OFFSET, OSPEEDR11_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 22)
	pub fn set_ospeedr11(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR11_BIT_OFFSET, OSPEEDR11_BIT_WIDTH, value as u32); }

	const OSPEEDR10_BIT_OFFSET: u8 = 20;
	const OSPEEDR10_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 20)
	pub fn get_ospeedr10() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR10_BIT_OFFSET, OSPEEDR10_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 20)
	pub fn set_ospeedr10(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR10_BIT_OFFSET, OSPEEDR10_BIT_WIDTH, value as u32); }

	const OSPEEDR9_BIT_OFFSET: u8 = 18;
	const OSPEEDR9_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 18)
	pub fn get_ospeedr9() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR9_BIT_OFFSET, OSPEEDR9_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 18)
	pub fn set_ospeedr9(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR9_BIT_OFFSET, OSPEEDR9_BIT_WIDTH, value as u32); }

	const OSPEEDR8_BIT_OFFSET: u8 = 16;
	const OSPEEDR8_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 16)
	pub fn get_ospeedr8() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR8_BIT_OFFSET, OSPEEDR8_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 16)
	pub fn set_ospeedr8(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR8_BIT_OFFSET, OSPEEDR8_BIT_WIDTH, value as u32); }

	const OSPEEDR7_BIT_OFFSET: u8 = 14;
	const OSPEEDR7_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 14)
	pub fn get_ospeedr7() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR7_BIT_OFFSET, OSPEEDR7_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 14)
	pub fn set_ospeedr7(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR7_BIT_OFFSET, OSPEEDR7_BIT_WIDTH, value as u32); }

	const OSPEEDR6_BIT_OFFSET: u8 = 12;
	const OSPEEDR6_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 12)
	pub fn get_ospeedr6() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR6_BIT_OFFSET, OSPEEDR6_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 12)
	pub fn set_ospeedr6(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR6_BIT_OFFSET, OSPEEDR6_BIT_WIDTH, value as u32); }

	const OSPEEDR5_BIT_OFFSET: u8 = 10;
	const OSPEEDR5_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 10)
	pub fn get_ospeedr5() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR5_BIT_OFFSET, OSPEEDR5_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 10)
	pub fn set_ospeedr5(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR5_BIT_OFFSET, OSPEEDR5_BIT_WIDTH, value as u32); }

	const OSPEEDR4_BIT_OFFSET: u8 = 8;
	const OSPEEDR4_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 8)
	pub fn get_ospeedr4() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR4_BIT_OFFSET, OSPEEDR4_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 8)
	pub fn set_ospeedr4(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR4_BIT_OFFSET, OSPEEDR4_BIT_WIDTH, value as u32); }

	const OSPEEDR3_BIT_OFFSET: u8 = 6;
	const OSPEEDR3_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 6)
	pub fn get_ospeedr3() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR3_BIT_OFFSET, OSPEEDR3_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 6)
	pub fn set_ospeedr3(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR3_BIT_OFFSET, OSPEEDR3_BIT_WIDTH, value as u32); }

	const OSPEEDR2_BIT_OFFSET: u8 = 4;
	const OSPEEDR2_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 4)
	pub fn get_ospeedr2() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR2_BIT_OFFSET, OSPEEDR2_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 4)
	pub fn set_ospeedr2(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR2_BIT_OFFSET, OSPEEDR2_BIT_WIDTH, value as u32); }

	const OSPEEDR1_BIT_OFFSET: u8 = 2;
	const OSPEEDR1_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 2)
	pub fn get_ospeedr1() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR1_BIT_OFFSET, OSPEEDR1_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 2)
	pub fn set_ospeedr1(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR1_BIT_OFFSET, OSPEEDR1_BIT_WIDTH, value as u32); }

	const OSPEEDR0_BIT_OFFSET: u8 = 0;
	const OSPEEDR0_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 0)
	pub fn get_ospeedr0() -> u8 { ::read(REGISTER_ADDRESS, OSPEEDR0_BIT_OFFSET, OSPEEDR0_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 0)
	pub fn set_ospeedr0(value: u8) { ::write(REGISTER_ADDRESS, OSPEEDR0_BIT_OFFSET, OSPEEDR0_BIT_WIDTH, value as u32); }
}
/// GPIO port pull-up/pull-down register
/// Size: 0x20 bits
pub mod pupdr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PUPDR15_BIT_OFFSET: u8 = 30;
	const PUPDR15_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 30)
	pub fn get_pupdr15() -> u8 { ::read(REGISTER_ADDRESS, PUPDR15_BIT_OFFSET, PUPDR15_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 30)
	pub fn set_pupdr15(value: u8) { ::write(REGISTER_ADDRESS, PUPDR15_BIT_OFFSET, PUPDR15_BIT_WIDTH, value as u32); }

	const PUPDR14_BIT_OFFSET: u8 = 28;
	const PUPDR14_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 28)
	pub fn get_pupdr14() -> u8 { ::read(REGISTER_ADDRESS, PUPDR14_BIT_OFFSET, PUPDR14_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 28)
	pub fn set_pupdr14(value: u8) { ::write(REGISTER_ADDRESS, PUPDR14_BIT_OFFSET, PUPDR14_BIT_WIDTH, value as u32); }

	const PUPDR13_BIT_OFFSET: u8 = 26;
	const PUPDR13_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 26)
	pub fn get_pupdr13() -> u8 { ::read(REGISTER_ADDRESS, PUPDR13_BIT_OFFSET, PUPDR13_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 26)
	pub fn set_pupdr13(value: u8) { ::write(REGISTER_ADDRESS, PUPDR13_BIT_OFFSET, PUPDR13_BIT_WIDTH, value as u32); }

	const PUPDR12_BIT_OFFSET: u8 = 24;
	const PUPDR12_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 24)
	pub fn get_pupdr12() -> u8 { ::read(REGISTER_ADDRESS, PUPDR12_BIT_OFFSET, PUPDR12_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 24)
	pub fn set_pupdr12(value: u8) { ::write(REGISTER_ADDRESS, PUPDR12_BIT_OFFSET, PUPDR12_BIT_WIDTH, value as u32); }

	const PUPDR11_BIT_OFFSET: u8 = 22;
	const PUPDR11_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 22)
	pub fn get_pupdr11() -> u8 { ::read(REGISTER_ADDRESS, PUPDR11_BIT_OFFSET, PUPDR11_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 22)
	pub fn set_pupdr11(value: u8) { ::write(REGISTER_ADDRESS, PUPDR11_BIT_OFFSET, PUPDR11_BIT_WIDTH, value as u32); }

	const PUPDR10_BIT_OFFSET: u8 = 20;
	const PUPDR10_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 20)
	pub fn get_pupdr10() -> u8 { ::read(REGISTER_ADDRESS, PUPDR10_BIT_OFFSET, PUPDR10_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 20)
	pub fn set_pupdr10(value: u8) { ::write(REGISTER_ADDRESS, PUPDR10_BIT_OFFSET, PUPDR10_BIT_WIDTH, value as u32); }

	const PUPDR9_BIT_OFFSET: u8 = 18;
	const PUPDR9_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 18)
	pub fn get_pupdr9() -> u8 { ::read(REGISTER_ADDRESS, PUPDR9_BIT_OFFSET, PUPDR9_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 18)
	pub fn set_pupdr9(value: u8) { ::write(REGISTER_ADDRESS, PUPDR9_BIT_OFFSET, PUPDR9_BIT_WIDTH, value as u32); }

	const PUPDR8_BIT_OFFSET: u8 = 16;
	const PUPDR8_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 16)
	pub fn get_pupdr8() -> u8 { ::read(REGISTER_ADDRESS, PUPDR8_BIT_OFFSET, PUPDR8_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 16)
	pub fn set_pupdr8(value: u8) { ::write(REGISTER_ADDRESS, PUPDR8_BIT_OFFSET, PUPDR8_BIT_WIDTH, value as u32); }

	const PUPDR7_BIT_OFFSET: u8 = 14;
	const PUPDR7_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 14)
	pub fn get_pupdr7() -> u8 { ::read(REGISTER_ADDRESS, PUPDR7_BIT_OFFSET, PUPDR7_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 14)
	pub fn set_pupdr7(value: u8) { ::write(REGISTER_ADDRESS, PUPDR7_BIT_OFFSET, PUPDR7_BIT_WIDTH, value as u32); }

	const PUPDR6_BIT_OFFSET: u8 = 12;
	const PUPDR6_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 12)
	pub fn get_pupdr6() -> u8 { ::read(REGISTER_ADDRESS, PUPDR6_BIT_OFFSET, PUPDR6_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 12)
	pub fn set_pupdr6(value: u8) { ::write(REGISTER_ADDRESS, PUPDR6_BIT_OFFSET, PUPDR6_BIT_WIDTH, value as u32); }

	const PUPDR5_BIT_OFFSET: u8 = 10;
	const PUPDR5_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 10)
	pub fn get_pupdr5() -> u8 { ::read(REGISTER_ADDRESS, PUPDR5_BIT_OFFSET, PUPDR5_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 10)
	pub fn set_pupdr5(value: u8) { ::write(REGISTER_ADDRESS, PUPDR5_BIT_OFFSET, PUPDR5_BIT_WIDTH, value as u32); }

	const PUPDR4_BIT_OFFSET: u8 = 8;
	const PUPDR4_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 8)
	pub fn get_pupdr4() -> u8 { ::read(REGISTER_ADDRESS, PUPDR4_BIT_OFFSET, PUPDR4_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 8)
	pub fn set_pupdr4(value: u8) { ::write(REGISTER_ADDRESS, PUPDR4_BIT_OFFSET, PUPDR4_BIT_WIDTH, value as u32); }

	const PUPDR3_BIT_OFFSET: u8 = 6;
	const PUPDR3_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 6)
	pub fn get_pupdr3() -> u8 { ::read(REGISTER_ADDRESS, PUPDR3_BIT_OFFSET, PUPDR3_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 6)
	pub fn set_pupdr3(value: u8) { ::write(REGISTER_ADDRESS, PUPDR3_BIT_OFFSET, PUPDR3_BIT_WIDTH, value as u32); }

	const PUPDR2_BIT_OFFSET: u8 = 4;
	const PUPDR2_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 4)
	pub fn get_pupdr2() -> u8 { ::read(REGISTER_ADDRESS, PUPDR2_BIT_OFFSET, PUPDR2_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 4)
	pub fn set_pupdr2(value: u8) { ::write(REGISTER_ADDRESS, PUPDR2_BIT_OFFSET, PUPDR2_BIT_WIDTH, value as u32); }

	const PUPDR1_BIT_OFFSET: u8 = 2;
	const PUPDR1_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 2)
	pub fn get_pupdr1() -> u8 { ::read(REGISTER_ADDRESS, PUPDR1_BIT_OFFSET, PUPDR1_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 2)
	pub fn set_pupdr1(value: u8) { ::write(REGISTER_ADDRESS, PUPDR1_BIT_OFFSET, PUPDR1_BIT_WIDTH, value as u32); }

	const PUPDR0_BIT_OFFSET: u8 = 0;
	const PUPDR0_BIT_WIDTH: u8 = 2;
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 0)
	pub fn get_pupdr0() -> u8 { ::read(REGISTER_ADDRESS, PUPDR0_BIT_OFFSET, PUPDR0_BIT_WIDTH) as u8 }
	/// Port x configuration bits (y = 0..15) (Width: 2, Offset: 0)
	pub fn set_pupdr0(value: u8) { ::write(REGISTER_ADDRESS, PUPDR0_BIT_OFFSET, PUPDR0_BIT_WIDTH, value as u32); }
}
/// GPIO port input data register
/// Size: 0x20 bits
pub mod idr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IDR15_BIT_OFFSET: u8 = 15;
	const IDR15_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 15)
	pub fn get_idr15() -> u8 { ::read(REGISTER_ADDRESS, IDR15_BIT_OFFSET, IDR15_BIT_WIDTH) as u8 }

	const IDR14_BIT_OFFSET: u8 = 14;
	const IDR14_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 14)
	pub fn get_idr14() -> u8 { ::read(REGISTER_ADDRESS, IDR14_BIT_OFFSET, IDR14_BIT_WIDTH) as u8 }

	const IDR13_BIT_OFFSET: u8 = 13;
	const IDR13_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 13)
	pub fn get_idr13() -> u8 { ::read(REGISTER_ADDRESS, IDR13_BIT_OFFSET, IDR13_BIT_WIDTH) as u8 }

	const IDR12_BIT_OFFSET: u8 = 12;
	const IDR12_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 12)
	pub fn get_idr12() -> u8 { ::read(REGISTER_ADDRESS, IDR12_BIT_OFFSET, IDR12_BIT_WIDTH) as u8 }

	const IDR11_BIT_OFFSET: u8 = 11;
	const IDR11_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 11)
	pub fn get_idr11() -> u8 { ::read(REGISTER_ADDRESS, IDR11_BIT_OFFSET, IDR11_BIT_WIDTH) as u8 }

	const IDR10_BIT_OFFSET: u8 = 10;
	const IDR10_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 10)
	pub fn get_idr10() -> u8 { ::read(REGISTER_ADDRESS, IDR10_BIT_OFFSET, IDR10_BIT_WIDTH) as u8 }

	const IDR9_BIT_OFFSET: u8 = 9;
	const IDR9_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 9)
	pub fn get_idr9() -> u8 { ::read(REGISTER_ADDRESS, IDR9_BIT_OFFSET, IDR9_BIT_WIDTH) as u8 }

	const IDR8_BIT_OFFSET: u8 = 8;
	const IDR8_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 8)
	pub fn get_idr8() -> u8 { ::read(REGISTER_ADDRESS, IDR8_BIT_OFFSET, IDR8_BIT_WIDTH) as u8 }

	const IDR7_BIT_OFFSET: u8 = 7;
	const IDR7_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 7)
	pub fn get_idr7() -> u8 { ::read(REGISTER_ADDRESS, IDR7_BIT_OFFSET, IDR7_BIT_WIDTH) as u8 }

	const IDR6_BIT_OFFSET: u8 = 6;
	const IDR6_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 6)
	pub fn get_idr6() -> u8 { ::read(REGISTER_ADDRESS, IDR6_BIT_OFFSET, IDR6_BIT_WIDTH) as u8 }

	const IDR5_BIT_OFFSET: u8 = 5;
	const IDR5_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 5)
	pub fn get_idr5() -> u8 { ::read(REGISTER_ADDRESS, IDR5_BIT_OFFSET, IDR5_BIT_WIDTH) as u8 }

	const IDR4_BIT_OFFSET: u8 = 4;
	const IDR4_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 4)
	pub fn get_idr4() -> u8 { ::read(REGISTER_ADDRESS, IDR4_BIT_OFFSET, IDR4_BIT_WIDTH) as u8 }

	const IDR3_BIT_OFFSET: u8 = 3;
	const IDR3_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 3)
	pub fn get_idr3() -> u8 { ::read(REGISTER_ADDRESS, IDR3_BIT_OFFSET, IDR3_BIT_WIDTH) as u8 }

	const IDR2_BIT_OFFSET: u8 = 2;
	const IDR2_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 2)
	pub fn get_idr2() -> u8 { ::read(REGISTER_ADDRESS, IDR2_BIT_OFFSET, IDR2_BIT_WIDTH) as u8 }

	const IDR1_BIT_OFFSET: u8 = 1;
	const IDR1_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 1)
	pub fn get_idr1() -> u8 { ::read(REGISTER_ADDRESS, IDR1_BIT_OFFSET, IDR1_BIT_WIDTH) as u8 }

	const IDR0_BIT_OFFSET: u8 = 0;
	const IDR0_BIT_WIDTH: u8 = 1;
	/// Port input data (y = 0..15) (Width: 1, Offset: 0)
	pub fn get_idr0() -> u8 { ::read(REGISTER_ADDRESS, IDR0_BIT_OFFSET, IDR0_BIT_WIDTH) as u8 }
}
/// GPIO port output data register
/// Size: 0x20 bits
pub mod odr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ODR15_BIT_OFFSET: u8 = 15;
	const ODR15_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 15)
	pub fn get_odr15() -> u8 { ::read(REGISTER_ADDRESS, ODR15_BIT_OFFSET, ODR15_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 15)
	pub fn set_odr15(value: u8) { ::write(REGISTER_ADDRESS, ODR15_BIT_OFFSET, ODR15_BIT_WIDTH, value as u32); }

	const ODR14_BIT_OFFSET: u8 = 14;
	const ODR14_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 14)
	pub fn get_odr14() -> u8 { ::read(REGISTER_ADDRESS, ODR14_BIT_OFFSET, ODR14_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 14)
	pub fn set_odr14(value: u8) { ::write(REGISTER_ADDRESS, ODR14_BIT_OFFSET, ODR14_BIT_WIDTH, value as u32); }

	const ODR13_BIT_OFFSET: u8 = 13;
	const ODR13_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 13)
	pub fn get_odr13() -> u8 { ::read(REGISTER_ADDRESS, ODR13_BIT_OFFSET, ODR13_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 13)
	pub fn set_odr13(value: u8) { ::write(REGISTER_ADDRESS, ODR13_BIT_OFFSET, ODR13_BIT_WIDTH, value as u32); }

	const ODR12_BIT_OFFSET: u8 = 12;
	const ODR12_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 12)
	pub fn get_odr12() -> u8 { ::read(REGISTER_ADDRESS, ODR12_BIT_OFFSET, ODR12_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 12)
	pub fn set_odr12(value: u8) { ::write(REGISTER_ADDRESS, ODR12_BIT_OFFSET, ODR12_BIT_WIDTH, value as u32); }

	const ODR11_BIT_OFFSET: u8 = 11;
	const ODR11_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 11)
	pub fn get_odr11() -> u8 { ::read(REGISTER_ADDRESS, ODR11_BIT_OFFSET, ODR11_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 11)
	pub fn set_odr11(value: u8) { ::write(REGISTER_ADDRESS, ODR11_BIT_OFFSET, ODR11_BIT_WIDTH, value as u32); }

	const ODR10_BIT_OFFSET: u8 = 10;
	const ODR10_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 10)
	pub fn get_odr10() -> u8 { ::read(REGISTER_ADDRESS, ODR10_BIT_OFFSET, ODR10_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 10)
	pub fn set_odr10(value: u8) { ::write(REGISTER_ADDRESS, ODR10_BIT_OFFSET, ODR10_BIT_WIDTH, value as u32); }

	const ODR9_BIT_OFFSET: u8 = 9;
	const ODR9_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 9)
	pub fn get_odr9() -> u8 { ::read(REGISTER_ADDRESS, ODR9_BIT_OFFSET, ODR9_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 9)
	pub fn set_odr9(value: u8) { ::write(REGISTER_ADDRESS, ODR9_BIT_OFFSET, ODR9_BIT_WIDTH, value as u32); }

	const ODR8_BIT_OFFSET: u8 = 8;
	const ODR8_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 8)
	pub fn get_odr8() -> u8 { ::read(REGISTER_ADDRESS, ODR8_BIT_OFFSET, ODR8_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 8)
	pub fn set_odr8(value: u8) { ::write(REGISTER_ADDRESS, ODR8_BIT_OFFSET, ODR8_BIT_WIDTH, value as u32); }

	const ODR7_BIT_OFFSET: u8 = 7;
	const ODR7_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 7)
	pub fn get_odr7() -> u8 { ::read(REGISTER_ADDRESS, ODR7_BIT_OFFSET, ODR7_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 7)
	pub fn set_odr7(value: u8) { ::write(REGISTER_ADDRESS, ODR7_BIT_OFFSET, ODR7_BIT_WIDTH, value as u32); }

	const ODR6_BIT_OFFSET: u8 = 6;
	const ODR6_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 6)
	pub fn get_odr6() -> u8 { ::read(REGISTER_ADDRESS, ODR6_BIT_OFFSET, ODR6_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 6)
	pub fn set_odr6(value: u8) { ::write(REGISTER_ADDRESS, ODR6_BIT_OFFSET, ODR6_BIT_WIDTH, value as u32); }

	const ODR5_BIT_OFFSET: u8 = 5;
	const ODR5_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 5)
	pub fn get_odr5() -> u8 { ::read(REGISTER_ADDRESS, ODR5_BIT_OFFSET, ODR5_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 5)
	pub fn set_odr5(value: u8) { ::write(REGISTER_ADDRESS, ODR5_BIT_OFFSET, ODR5_BIT_WIDTH, value as u32); }

	const ODR4_BIT_OFFSET: u8 = 4;
	const ODR4_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 4)
	pub fn get_odr4() -> u8 { ::read(REGISTER_ADDRESS, ODR4_BIT_OFFSET, ODR4_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 4)
	pub fn set_odr4(value: u8) { ::write(REGISTER_ADDRESS, ODR4_BIT_OFFSET, ODR4_BIT_WIDTH, value as u32); }

	const ODR3_BIT_OFFSET: u8 = 3;
	const ODR3_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 3)
	pub fn get_odr3() -> u8 { ::read(REGISTER_ADDRESS, ODR3_BIT_OFFSET, ODR3_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 3)
	pub fn set_odr3(value: u8) { ::write(REGISTER_ADDRESS, ODR3_BIT_OFFSET, ODR3_BIT_WIDTH, value as u32); }

	const ODR2_BIT_OFFSET: u8 = 2;
	const ODR2_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 2)
	pub fn get_odr2() -> u8 { ::read(REGISTER_ADDRESS, ODR2_BIT_OFFSET, ODR2_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 2)
	pub fn set_odr2(value: u8) { ::write(REGISTER_ADDRESS, ODR2_BIT_OFFSET, ODR2_BIT_WIDTH, value as u32); }

	const ODR1_BIT_OFFSET: u8 = 1;
	const ODR1_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 1)
	pub fn get_odr1() -> u8 { ::read(REGISTER_ADDRESS, ODR1_BIT_OFFSET, ODR1_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 1)
	pub fn set_odr1(value: u8) { ::write(REGISTER_ADDRESS, ODR1_BIT_OFFSET, ODR1_BIT_WIDTH, value as u32); }

	const ODR0_BIT_OFFSET: u8 = 0;
	const ODR0_BIT_WIDTH: u8 = 1;
	/// Port output data (y = 0..15) (Width: 1, Offset: 0)
	pub fn get_odr0() -> u8 { ::read(REGISTER_ADDRESS, ODR0_BIT_OFFSET, ODR0_BIT_WIDTH) as u8 }
	/// Port output data (y = 0..15) (Width: 1, Offset: 0)
	pub fn set_odr0(value: u8) { ::write(REGISTER_ADDRESS, ODR0_BIT_OFFSET, ODR0_BIT_WIDTH, value as u32); }
}
/// GPIO port bit set/reset register
/// Size: 0x20 bits
pub mod bsrr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BR15_BIT_OFFSET: u8 = 31;
	const BR15_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 31)
	pub fn set_br15(value: u8) { ::write(REGISTER_ADDRESS, BR15_BIT_OFFSET, BR15_BIT_WIDTH, value as u32); }

	const BR14_BIT_OFFSET: u8 = 30;
	const BR14_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 30)
	pub fn set_br14(value: u8) { ::write(REGISTER_ADDRESS, BR14_BIT_OFFSET, BR14_BIT_WIDTH, value as u32); }

	const BR13_BIT_OFFSET: u8 = 29;
	const BR13_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 29)
	pub fn set_br13(value: u8) { ::write(REGISTER_ADDRESS, BR13_BIT_OFFSET, BR13_BIT_WIDTH, value as u32); }

	const BR12_BIT_OFFSET: u8 = 28;
	const BR12_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 28)
	pub fn set_br12(value: u8) { ::write(REGISTER_ADDRESS, BR12_BIT_OFFSET, BR12_BIT_WIDTH, value as u32); }

	const BR11_BIT_OFFSET: u8 = 27;
	const BR11_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 27)
	pub fn set_br11(value: u8) { ::write(REGISTER_ADDRESS, BR11_BIT_OFFSET, BR11_BIT_WIDTH, value as u32); }

	const BR10_BIT_OFFSET: u8 = 26;
	const BR10_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 26)
	pub fn set_br10(value: u8) { ::write(REGISTER_ADDRESS, BR10_BIT_OFFSET, BR10_BIT_WIDTH, value as u32); }

	const BR9_BIT_OFFSET: u8 = 25;
	const BR9_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 25)
	pub fn set_br9(value: u8) { ::write(REGISTER_ADDRESS, BR9_BIT_OFFSET, BR9_BIT_WIDTH, value as u32); }

	const BR8_BIT_OFFSET: u8 = 24;
	const BR8_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 24)
	pub fn set_br8(value: u8) { ::write(REGISTER_ADDRESS, BR8_BIT_OFFSET, BR8_BIT_WIDTH, value as u32); }

	const BR7_BIT_OFFSET: u8 = 23;
	const BR7_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 23)
	pub fn set_br7(value: u8) { ::write(REGISTER_ADDRESS, BR7_BIT_OFFSET, BR7_BIT_WIDTH, value as u32); }

	const BR6_BIT_OFFSET: u8 = 22;
	const BR6_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 22)
	pub fn set_br6(value: u8) { ::write(REGISTER_ADDRESS, BR6_BIT_OFFSET, BR6_BIT_WIDTH, value as u32); }

	const BR5_BIT_OFFSET: u8 = 21;
	const BR5_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 21)
	pub fn set_br5(value: u8) { ::write(REGISTER_ADDRESS, BR5_BIT_OFFSET, BR5_BIT_WIDTH, value as u32); }

	const BR4_BIT_OFFSET: u8 = 20;
	const BR4_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 20)
	pub fn set_br4(value: u8) { ::write(REGISTER_ADDRESS, BR4_BIT_OFFSET, BR4_BIT_WIDTH, value as u32); }

	const BR3_BIT_OFFSET: u8 = 19;
	const BR3_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 19)
	pub fn set_br3(value: u8) { ::write(REGISTER_ADDRESS, BR3_BIT_OFFSET, BR3_BIT_WIDTH, value as u32); }

	const BR2_BIT_OFFSET: u8 = 18;
	const BR2_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 18)
	pub fn set_br2(value: u8) { ::write(REGISTER_ADDRESS, BR2_BIT_OFFSET, BR2_BIT_WIDTH, value as u32); }

	const BR1_BIT_OFFSET: u8 = 17;
	const BR1_BIT_WIDTH: u8 = 1;
	/// Port x reset bit y (y = 0..15) (Width: 1, Offset: 17)
	pub fn set_br1(value: u8) { ::write(REGISTER_ADDRESS, BR1_BIT_OFFSET, BR1_BIT_WIDTH, value as u32); }

	const BR0_BIT_OFFSET: u8 = 16;
	const BR0_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 16)
	pub fn set_br0(value: u8) { ::write(REGISTER_ADDRESS, BR0_BIT_OFFSET, BR0_BIT_WIDTH, value as u32); }

	const BS15_BIT_OFFSET: u8 = 15;
	const BS15_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 15)
	pub fn set_bs15(value: u8) { ::write(REGISTER_ADDRESS, BS15_BIT_OFFSET, BS15_BIT_WIDTH, value as u32); }

	const BS14_BIT_OFFSET: u8 = 14;
	const BS14_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 14)
	pub fn set_bs14(value: u8) { ::write(REGISTER_ADDRESS, BS14_BIT_OFFSET, BS14_BIT_WIDTH, value as u32); }

	const BS13_BIT_OFFSET: u8 = 13;
	const BS13_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 13)
	pub fn set_bs13(value: u8) { ::write(REGISTER_ADDRESS, BS13_BIT_OFFSET, BS13_BIT_WIDTH, value as u32); }

	const BS12_BIT_OFFSET: u8 = 12;
	const BS12_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 12)
	pub fn set_bs12(value: u8) { ::write(REGISTER_ADDRESS, BS12_BIT_OFFSET, BS12_BIT_WIDTH, value as u32); }

	const BS11_BIT_OFFSET: u8 = 11;
	const BS11_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 11)
	pub fn set_bs11(value: u8) { ::write(REGISTER_ADDRESS, BS11_BIT_OFFSET, BS11_BIT_WIDTH, value as u32); }

	const BS10_BIT_OFFSET: u8 = 10;
	const BS10_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 10)
	pub fn set_bs10(value: u8) { ::write(REGISTER_ADDRESS, BS10_BIT_OFFSET, BS10_BIT_WIDTH, value as u32); }

	const BS9_BIT_OFFSET: u8 = 9;
	const BS9_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 9)
	pub fn set_bs9(value: u8) { ::write(REGISTER_ADDRESS, BS9_BIT_OFFSET, BS9_BIT_WIDTH, value as u32); }

	const BS8_BIT_OFFSET: u8 = 8;
	const BS8_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 8)
	pub fn set_bs8(value: u8) { ::write(REGISTER_ADDRESS, BS8_BIT_OFFSET, BS8_BIT_WIDTH, value as u32); }

	const BS7_BIT_OFFSET: u8 = 7;
	const BS7_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 7)
	pub fn set_bs7(value: u8) { ::write(REGISTER_ADDRESS, BS7_BIT_OFFSET, BS7_BIT_WIDTH, value as u32); }

	const BS6_BIT_OFFSET: u8 = 6;
	const BS6_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 6)
	pub fn set_bs6(value: u8) { ::write(REGISTER_ADDRESS, BS6_BIT_OFFSET, BS6_BIT_WIDTH, value as u32); }

	const BS5_BIT_OFFSET: u8 = 5;
	const BS5_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 5)
	pub fn set_bs5(value: u8) { ::write(REGISTER_ADDRESS, BS5_BIT_OFFSET, BS5_BIT_WIDTH, value as u32); }

	const BS4_BIT_OFFSET: u8 = 4;
	const BS4_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 4)
	pub fn set_bs4(value: u8) { ::write(REGISTER_ADDRESS, BS4_BIT_OFFSET, BS4_BIT_WIDTH, value as u32); }

	const BS3_BIT_OFFSET: u8 = 3;
	const BS3_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 3)
	pub fn set_bs3(value: u8) { ::write(REGISTER_ADDRESS, BS3_BIT_OFFSET, BS3_BIT_WIDTH, value as u32); }

	const BS2_BIT_OFFSET: u8 = 2;
	const BS2_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 2)
	pub fn set_bs2(value: u8) { ::write(REGISTER_ADDRESS, BS2_BIT_OFFSET, BS2_BIT_WIDTH, value as u32); }

	const BS1_BIT_OFFSET: u8 = 1;
	const BS1_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 1)
	pub fn set_bs1(value: u8) { ::write(REGISTER_ADDRESS, BS1_BIT_OFFSET, BS1_BIT_WIDTH, value as u32); }

	const BS0_BIT_OFFSET: u8 = 0;
	const BS0_BIT_WIDTH: u8 = 1;
	/// Port x set bit y (y= 0..15) (Width: 1, Offset: 0)
	pub fn set_bs0(value: u8) { ::write(REGISTER_ADDRESS, BS0_BIT_OFFSET, BS0_BIT_WIDTH, value as u32); }
}
/// GPIO port configuration lock register
/// Size: 0x20 bits
pub mod lckr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const LCKK_BIT_OFFSET: u8 = 16;
	const LCKK_BIT_WIDTH: u8 = 1;
	/// Lok Key (Width: 1, Offset: 16)
	pub fn get_lckk() -> u8 { ::read(REGISTER_ADDRESS, LCKK_BIT_OFFSET, LCKK_BIT_WIDTH) as u8 }
	/// Lok Key (Width: 1, Offset: 16)
	pub fn set_lckk(value: u8) { ::write(REGISTER_ADDRESS, LCKK_BIT_OFFSET, LCKK_BIT_WIDTH, value as u32); }

	const LCK15_BIT_OFFSET: u8 = 15;
	const LCK15_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 15)
	pub fn get_lck15() -> u8 { ::read(REGISTER_ADDRESS, LCK15_BIT_OFFSET, LCK15_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 15)
	pub fn set_lck15(value: u8) { ::write(REGISTER_ADDRESS, LCK15_BIT_OFFSET, LCK15_BIT_WIDTH, value as u32); }

	const LCK14_BIT_OFFSET: u8 = 14;
	const LCK14_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 14)
	pub fn get_lck14() -> u8 { ::read(REGISTER_ADDRESS, LCK14_BIT_OFFSET, LCK14_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 14)
	pub fn set_lck14(value: u8) { ::write(REGISTER_ADDRESS, LCK14_BIT_OFFSET, LCK14_BIT_WIDTH, value as u32); }

	const LCK13_BIT_OFFSET: u8 = 13;
	const LCK13_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 13)
	pub fn get_lck13() -> u8 { ::read(REGISTER_ADDRESS, LCK13_BIT_OFFSET, LCK13_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 13)
	pub fn set_lck13(value: u8) { ::write(REGISTER_ADDRESS, LCK13_BIT_OFFSET, LCK13_BIT_WIDTH, value as u32); }

	const LCK12_BIT_OFFSET: u8 = 12;
	const LCK12_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 12)
	pub fn get_lck12() -> u8 { ::read(REGISTER_ADDRESS, LCK12_BIT_OFFSET, LCK12_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 12)
	pub fn set_lck12(value: u8) { ::write(REGISTER_ADDRESS, LCK12_BIT_OFFSET, LCK12_BIT_WIDTH, value as u32); }

	const LCK11_BIT_OFFSET: u8 = 11;
	const LCK11_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 11)
	pub fn get_lck11() -> u8 { ::read(REGISTER_ADDRESS, LCK11_BIT_OFFSET, LCK11_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 11)
	pub fn set_lck11(value: u8) { ::write(REGISTER_ADDRESS, LCK11_BIT_OFFSET, LCK11_BIT_WIDTH, value as u32); }

	const LCK10_BIT_OFFSET: u8 = 10;
	const LCK10_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 10)
	pub fn get_lck10() -> u8 { ::read(REGISTER_ADDRESS, LCK10_BIT_OFFSET, LCK10_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 10)
	pub fn set_lck10(value: u8) { ::write(REGISTER_ADDRESS, LCK10_BIT_OFFSET, LCK10_BIT_WIDTH, value as u32); }

	const LCK9_BIT_OFFSET: u8 = 9;
	const LCK9_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 9)
	pub fn get_lck9() -> u8 { ::read(REGISTER_ADDRESS, LCK9_BIT_OFFSET, LCK9_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 9)
	pub fn set_lck9(value: u8) { ::write(REGISTER_ADDRESS, LCK9_BIT_OFFSET, LCK9_BIT_WIDTH, value as u32); }

	const LCK8_BIT_OFFSET: u8 = 8;
	const LCK8_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 8)
	pub fn get_lck8() -> u8 { ::read(REGISTER_ADDRESS, LCK8_BIT_OFFSET, LCK8_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 8)
	pub fn set_lck8(value: u8) { ::write(REGISTER_ADDRESS, LCK8_BIT_OFFSET, LCK8_BIT_WIDTH, value as u32); }

	const LCK7_BIT_OFFSET: u8 = 7;
	const LCK7_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 7)
	pub fn get_lck7() -> u8 { ::read(REGISTER_ADDRESS, LCK7_BIT_OFFSET, LCK7_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 7)
	pub fn set_lck7(value: u8) { ::write(REGISTER_ADDRESS, LCK7_BIT_OFFSET, LCK7_BIT_WIDTH, value as u32); }

	const LCK6_BIT_OFFSET: u8 = 6;
	const LCK6_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 6)
	pub fn get_lck6() -> u8 { ::read(REGISTER_ADDRESS, LCK6_BIT_OFFSET, LCK6_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 6)
	pub fn set_lck6(value: u8) { ::write(REGISTER_ADDRESS, LCK6_BIT_OFFSET, LCK6_BIT_WIDTH, value as u32); }

	const LCK5_BIT_OFFSET: u8 = 5;
	const LCK5_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 5)
	pub fn get_lck5() -> u8 { ::read(REGISTER_ADDRESS, LCK5_BIT_OFFSET, LCK5_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 5)
	pub fn set_lck5(value: u8) { ::write(REGISTER_ADDRESS, LCK5_BIT_OFFSET, LCK5_BIT_WIDTH, value as u32); }

	const LCK4_BIT_OFFSET: u8 = 4;
	const LCK4_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 4)
	pub fn get_lck4() -> u8 { ::read(REGISTER_ADDRESS, LCK4_BIT_OFFSET, LCK4_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 4)
	pub fn set_lck4(value: u8) { ::write(REGISTER_ADDRESS, LCK4_BIT_OFFSET, LCK4_BIT_WIDTH, value as u32); }

	const LCK3_BIT_OFFSET: u8 = 3;
	const LCK3_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 3)
	pub fn get_lck3() -> u8 { ::read(REGISTER_ADDRESS, LCK3_BIT_OFFSET, LCK3_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 3)
	pub fn set_lck3(value: u8) { ::write(REGISTER_ADDRESS, LCK3_BIT_OFFSET, LCK3_BIT_WIDTH, value as u32); }

	const LCK2_BIT_OFFSET: u8 = 2;
	const LCK2_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 2)
	pub fn get_lck2() -> u8 { ::read(REGISTER_ADDRESS, LCK2_BIT_OFFSET, LCK2_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 2)
	pub fn set_lck2(value: u8) { ::write(REGISTER_ADDRESS, LCK2_BIT_OFFSET, LCK2_BIT_WIDTH, value as u32); }

	const LCK1_BIT_OFFSET: u8 = 1;
	const LCK1_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 1)
	pub fn get_lck1() -> u8 { ::read(REGISTER_ADDRESS, LCK1_BIT_OFFSET, LCK1_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 1)
	pub fn set_lck1(value: u8) { ::write(REGISTER_ADDRESS, LCK1_BIT_OFFSET, LCK1_BIT_WIDTH, value as u32); }

	const LCK0_BIT_OFFSET: u8 = 0;
	const LCK0_BIT_WIDTH: u8 = 1;
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 0)
	pub fn get_lck0() -> u8 { ::read(REGISTER_ADDRESS, LCK0_BIT_OFFSET, LCK0_BIT_WIDTH) as u8 }
	/// Port x lock bit y (y= 0..15) (Width: 1, Offset: 0)
	pub fn set_lck0(value: u8) { ::write(REGISTER_ADDRESS, LCK0_BIT_OFFSET, LCK0_BIT_WIDTH, value as u32); }
}
/// GPIO alternate function low register
/// Size: 0x20 bits
pub mod afrl {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x20;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const AFRL7_BIT_OFFSET: u8 = 28;
	const AFRL7_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 28)
	pub fn get_afrl7() -> u8 { ::read(REGISTER_ADDRESS, AFRL7_BIT_OFFSET, AFRL7_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 28)
	pub fn set_afrl7(value: u8) { ::write(REGISTER_ADDRESS, AFRL7_BIT_OFFSET, AFRL7_BIT_WIDTH, value as u32); }

	const AFRL6_BIT_OFFSET: u8 = 24;
	const AFRL6_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 24)
	pub fn get_afrl6() -> u8 { ::read(REGISTER_ADDRESS, AFRL6_BIT_OFFSET, AFRL6_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 24)
	pub fn set_afrl6(value: u8) { ::write(REGISTER_ADDRESS, AFRL6_BIT_OFFSET, AFRL6_BIT_WIDTH, value as u32); }

	const AFRL5_BIT_OFFSET: u8 = 20;
	const AFRL5_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 20)
	pub fn get_afrl5() -> u8 { ::read(REGISTER_ADDRESS, AFRL5_BIT_OFFSET, AFRL5_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 20)
	pub fn set_afrl5(value: u8) { ::write(REGISTER_ADDRESS, AFRL5_BIT_OFFSET, AFRL5_BIT_WIDTH, value as u32); }

	const AFRL4_BIT_OFFSET: u8 = 16;
	const AFRL4_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 16)
	pub fn get_afrl4() -> u8 { ::read(REGISTER_ADDRESS, AFRL4_BIT_OFFSET, AFRL4_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 16)
	pub fn set_afrl4(value: u8) { ::write(REGISTER_ADDRESS, AFRL4_BIT_OFFSET, AFRL4_BIT_WIDTH, value as u32); }

	const AFRL3_BIT_OFFSET: u8 = 12;
	const AFRL3_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 12)
	pub fn get_afrl3() -> u8 { ::read(REGISTER_ADDRESS, AFRL3_BIT_OFFSET, AFRL3_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 12)
	pub fn set_afrl3(value: u8) { ::write(REGISTER_ADDRESS, AFRL3_BIT_OFFSET, AFRL3_BIT_WIDTH, value as u32); }

	const AFRL2_BIT_OFFSET: u8 = 8;
	const AFRL2_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 8)
	pub fn get_afrl2() -> u8 { ::read(REGISTER_ADDRESS, AFRL2_BIT_OFFSET, AFRL2_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 8)
	pub fn set_afrl2(value: u8) { ::write(REGISTER_ADDRESS, AFRL2_BIT_OFFSET, AFRL2_BIT_WIDTH, value as u32); }

	const AFRL1_BIT_OFFSET: u8 = 4;
	const AFRL1_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 4)
	pub fn get_afrl1() -> u8 { ::read(REGISTER_ADDRESS, AFRL1_BIT_OFFSET, AFRL1_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 4)
	pub fn set_afrl1(value: u8) { ::write(REGISTER_ADDRESS, AFRL1_BIT_OFFSET, AFRL1_BIT_WIDTH, value as u32); }

	const AFRL0_BIT_OFFSET: u8 = 0;
	const AFRL0_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 0)
	pub fn get_afrl0() -> u8 { ::read(REGISTER_ADDRESS, AFRL0_BIT_OFFSET, AFRL0_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 0..7) (Width: 4, Offset: 0)
	pub fn set_afrl0(value: u8) { ::write(REGISTER_ADDRESS, AFRL0_BIT_OFFSET, AFRL0_BIT_WIDTH, value as u32); }
}
/// GPIO alternate function high register
/// Size: 0x20 bits
pub mod afrh {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x24;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const AFRH15_BIT_OFFSET: u8 = 28;
	const AFRH15_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 28)
	pub fn get_afrh15() -> u8 { ::read(REGISTER_ADDRESS, AFRH15_BIT_OFFSET, AFRH15_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 28)
	pub fn set_afrh15(value: u8) { ::write(REGISTER_ADDRESS, AFRH15_BIT_OFFSET, AFRH15_BIT_WIDTH, value as u32); }

	const AFRH14_BIT_OFFSET: u8 = 24;
	const AFRH14_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 24)
	pub fn get_afrh14() -> u8 { ::read(REGISTER_ADDRESS, AFRH14_BIT_OFFSET, AFRH14_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 24)
	pub fn set_afrh14(value: u8) { ::write(REGISTER_ADDRESS, AFRH14_BIT_OFFSET, AFRH14_BIT_WIDTH, value as u32); }

	const AFRH13_BIT_OFFSET: u8 = 20;
	const AFRH13_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 20)
	pub fn get_afrh13() -> u8 { ::read(REGISTER_ADDRESS, AFRH13_BIT_OFFSET, AFRH13_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 20)
	pub fn set_afrh13(value: u8) { ::write(REGISTER_ADDRESS, AFRH13_BIT_OFFSET, AFRH13_BIT_WIDTH, value as u32); }

	const AFRH12_BIT_OFFSET: u8 = 16;
	const AFRH12_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 16)
	pub fn get_afrh12() -> u8 { ::read(REGISTER_ADDRESS, AFRH12_BIT_OFFSET, AFRH12_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 16)
	pub fn set_afrh12(value: u8) { ::write(REGISTER_ADDRESS, AFRH12_BIT_OFFSET, AFRH12_BIT_WIDTH, value as u32); }

	const AFRH11_BIT_OFFSET: u8 = 12;
	const AFRH11_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 12)
	pub fn get_afrh11() -> u8 { ::read(REGISTER_ADDRESS, AFRH11_BIT_OFFSET, AFRH11_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 12)
	pub fn set_afrh11(value: u8) { ::write(REGISTER_ADDRESS, AFRH11_BIT_OFFSET, AFRH11_BIT_WIDTH, value as u32); }

	const AFRH10_BIT_OFFSET: u8 = 8;
	const AFRH10_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 8)
	pub fn get_afrh10() -> u8 { ::read(REGISTER_ADDRESS, AFRH10_BIT_OFFSET, AFRH10_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 8)
	pub fn set_afrh10(value: u8) { ::write(REGISTER_ADDRESS, AFRH10_BIT_OFFSET, AFRH10_BIT_WIDTH, value as u32); }

	const AFRH9_BIT_OFFSET: u8 = 4;
	const AFRH9_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 4)
	pub fn get_afrh9() -> u8 { ::read(REGISTER_ADDRESS, AFRH9_BIT_OFFSET, AFRH9_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 4)
	pub fn set_afrh9(value: u8) { ::write(REGISTER_ADDRESS, AFRH9_BIT_OFFSET, AFRH9_BIT_WIDTH, value as u32); }

	const AFRH8_BIT_OFFSET: u8 = 0;
	const AFRH8_BIT_WIDTH: u8 = 4;
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 0)
	pub fn get_afrh8() -> u8 { ::read(REGISTER_ADDRESS, AFRH8_BIT_OFFSET, AFRH8_BIT_WIDTH) as u8 }
	/// Alternate function selection for port x bit y (y = 8..15) (Width: 4, Offset: 0)
	pub fn set_afrh8(value: u8) { ::write(REGISTER_ADDRESS, AFRH8_BIT_OFFSET, AFRH8_BIT_WIDTH, value as u32); }
}
/// Port bit reset register
/// Size: 0x20 bits
pub mod brr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x28;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BR0_BIT_OFFSET: u8 = 0;
	const BR0_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 0)
	pub fn set_br0(value: u8) { ::write(REGISTER_ADDRESS, BR0_BIT_OFFSET, BR0_BIT_WIDTH, value as u32); }

	const BR1_BIT_OFFSET: u8 = 1;
	const BR1_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 1)
	pub fn set_br1(value: u8) { ::write(REGISTER_ADDRESS, BR1_BIT_OFFSET, BR1_BIT_WIDTH, value as u32); }

	const BR2_BIT_OFFSET: u8 = 2;
	const BR2_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 2)
	pub fn set_br2(value: u8) { ::write(REGISTER_ADDRESS, BR2_BIT_OFFSET, BR2_BIT_WIDTH, value as u32); }

	const BR3_BIT_OFFSET: u8 = 3;
	const BR3_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 3)
	pub fn set_br3(value: u8) { ::write(REGISTER_ADDRESS, BR3_BIT_OFFSET, BR3_BIT_WIDTH, value as u32); }

	const BR4_BIT_OFFSET: u8 = 4;
	const BR4_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 4)
	pub fn set_br4(value: u8) { ::write(REGISTER_ADDRESS, BR4_BIT_OFFSET, BR4_BIT_WIDTH, value as u32); }

	const BR5_BIT_OFFSET: u8 = 5;
	const BR5_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 5)
	pub fn set_br5(value: u8) { ::write(REGISTER_ADDRESS, BR5_BIT_OFFSET, BR5_BIT_WIDTH, value as u32); }

	const BR6_BIT_OFFSET: u8 = 6;
	const BR6_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 6)
	pub fn set_br6(value: u8) { ::write(REGISTER_ADDRESS, BR6_BIT_OFFSET, BR6_BIT_WIDTH, value as u32); }

	const BR7_BIT_OFFSET: u8 = 7;
	const BR7_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 7)
	pub fn set_br7(value: u8) { ::write(REGISTER_ADDRESS, BR7_BIT_OFFSET, BR7_BIT_WIDTH, value as u32); }

	const BR8_BIT_OFFSET: u8 = 8;
	const BR8_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 8)
	pub fn set_br8(value: u8) { ::write(REGISTER_ADDRESS, BR8_BIT_OFFSET, BR8_BIT_WIDTH, value as u32); }

	const BR9_BIT_OFFSET: u8 = 9;
	const BR9_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 9)
	pub fn set_br9(value: u8) { ::write(REGISTER_ADDRESS, BR9_BIT_OFFSET, BR9_BIT_WIDTH, value as u32); }

	const BR10_BIT_OFFSET: u8 = 10;
	const BR10_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 10)
	pub fn set_br10(value: u8) { ::write(REGISTER_ADDRESS, BR10_BIT_OFFSET, BR10_BIT_WIDTH, value as u32); }

	const BR11_BIT_OFFSET: u8 = 11;
	const BR11_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 11)
	pub fn set_br11(value: u8) { ::write(REGISTER_ADDRESS, BR11_BIT_OFFSET, BR11_BIT_WIDTH, value as u32); }

	const BR12_BIT_OFFSET: u8 = 12;
	const BR12_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 12)
	pub fn set_br12(value: u8) { ::write(REGISTER_ADDRESS, BR12_BIT_OFFSET, BR12_BIT_WIDTH, value as u32); }

	const BR13_BIT_OFFSET: u8 = 13;
	const BR13_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 13)
	pub fn set_br13(value: u8) { ::write(REGISTER_ADDRESS, BR13_BIT_OFFSET, BR13_BIT_WIDTH, value as u32); }

	const BR14_BIT_OFFSET: u8 = 14;
	const BR14_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 14)
	pub fn set_br14(value: u8) { ::write(REGISTER_ADDRESS, BR14_BIT_OFFSET, BR14_BIT_WIDTH, value as u32); }

	const BR15_BIT_OFFSET: u8 = 15;
	const BR15_BIT_WIDTH: u8 = 1;
	/// Port x Reset bit y (Width: 1, Offset: 15)
	pub fn set_br15(value: u8) { ::write(REGISTER_ADDRESS, BR15_BIT_OFFSET, BR15_BIT_WIDTH, value as u32); }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" derivedFrom="GPIOB">
  <name>GPIOE</name>
  <description>General-purpose I/Os</description>
  <groupName>GPIO</groupName>
  <baseAddress>0x48001000</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>MODER</name>
      <displayName>MODER</displayName>
      <description>GPIO port mode register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MODER15</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>30</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER14</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER13</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>26</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER12</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER11</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER10</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER9</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>18</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER8</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER7</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>14</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER6</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER5</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER4</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER3</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER2</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER1</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MODER0</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>OTYPER</name>
      <displayName>OTYPER</displayName>
      <description>GPIO port output type register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OT15</name>
          <description>Port x configuration bit
              15</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT14</name>
          <description>Port x configuration bit
              14</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT13</name>
          <description>Port x configuration bit
              13</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT12</name>
          <description>Port x configuration bit
              12</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT11</name>
          <description>Port x configuration bit
              11</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT10</name>
          <description>Port x configuration bit
              10</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT9</name>
          <description>Port x configuration bit 9</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT8</name>
          <description>Port x configuration bit 8</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT7</name>
          <description>Port x configuration bit 7</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT6</name>
          <description>Port x configuration bit 6</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT5</name>
          <description>Port x configuration bit 5</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT4</name>
          <description>Port x configuration bit 4</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT3</name>
          <description>Port x configuration bit 3</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT2</name>
          <description>Port x configuration bit 2</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT1</name>
          <description>Port x configuration bit 1</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OT0</name>
          <description>Port x configuration bit 0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>OSPEEDR</name>
      <displayName>OSPEEDR</displayName>
      <description>GPIO port output speed
          register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OSPEEDR15</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>30</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR14</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR13</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>26</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR12</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR11</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR10</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR9</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>18</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR8</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR7</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>14</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR6</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR5</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR4</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR3</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR2</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR1</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSPEEDR0</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>PUPDR</name>
      <displayName>PUPDR</displayName>
      <description>GPIO port pull-up/pull-down
          register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PUPDR15</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>30</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR14</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR13</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>26</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR12</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR11</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR10</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR9</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>18</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR8</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR7</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>14</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR6</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR5</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR4</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR3</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR2</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR1</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PUPDR0</name>
          <description>Port x configuration bits (y =
              0..15)</description>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IDR</name>
      <displayName>IDR</displayName>
      <description>GPIO port input data register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IDR15</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR14</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR13</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR12</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR11</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR10</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR9</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR8</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR7</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR6</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR5</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR4</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR3</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR2</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR1</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDR0</name>
          <description>Port input data (y =
              0..15)</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ODR</name>
      <displayName>ODR</displayName>
      <description>GPIO port output data register</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>ODR15</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR14</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR13</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR12</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR11</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR10</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR9</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR8</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR7</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR6</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR5</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR4</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR3</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR2</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR1</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODR0</name>
          <description>Port output data (y =
              0..15)</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BSRR</name>
      <displayName>BSRR</displayName>
      <description>GPIO port bit set/reset
          register</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <access>write-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BR15</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR14</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR13</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR12</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR11</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR10</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR9</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR8</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR7</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR6</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR5</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR4</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR3</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR2</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR1</name>
          <description>Port x reset bit y (y =
              0..15)</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR0</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS15</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS14</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS13</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS12</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS11</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS10</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS9</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS8</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS7</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS6</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS5</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS4</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS3</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS2</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS1</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BS0</name>
          <description>Port x set bit y (y=
              0..15)</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>LCKR</name>
      <displayName>LCKR</displayName>
      <description>GPIO port configuration lock
          register</description>
      <addressOffset>0x1C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>LCKK</name>
          <description>Lok Key</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK15</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK14</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK13</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK12</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK11</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK10</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK9</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK8</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK7</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK6</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK5</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK4</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK3</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK2</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK1</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LCK0</name>
          <description>Port x lock bit y (y=
              0..15)</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>AFRL</name>
      <displayName>AFRL</displayName>
      <description>GPIO alternate function low
          register</description>
      <addressOffset>0x20</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>AFRL7</name>
          <description>Alternate function selection for port x
              bit y (y = 0..7)</description>
          <bitOffset>28</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>AFRL6</name>
          <description>Alternate function selection for port x
              bit y (y = 0..7)</description>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>AFRL5</name>
          <description>Alternate function selection for port x
              bit y (y = 0..7)</description>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>AFRL4</name>
          <description>Alternate function selection for port x
              bit y (y = 0..7)</description>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>AFRL3</name>
          <description>Alternate function selection for port x
              bit y (y = 0..7)</description>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>AFRL2</name>
          <description>Alternate function selection for port x
              bit y (y = 0..7)</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>AFRL1</name>
          <description>Alternate function selection for port x
              bit y (y = 0..7)</description>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>AFRL0</name>
          <description>Alternate function selection for port x
              bit y (y = 0..7)</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>AFRH</name>
      <displayName>AFRH</displayName>
      <description>GPIO alternate function high
          register</description>
      <addressOffset>0x24</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>AFRH15</name>
          <description>Alternate function selection for port x
              bit y (y = 8..15)</description>
          <bitOffset>28</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>AFRH14</name>
          <description>Alternate function selection for port x
              bit y (y = 8..15)</description>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>AFRH13</name>
          <description>Alternate function selection for port x
              bit y (y = 8..15)</description>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>AFRH12</name>
          <description>Alternate function selection for port x
              bit y (y = 8..15)</description>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>AFRH11</name>
          <description>Alternate function selection for port x
              bit y (y = 8..15)</description>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>AFRH10</name>
          <description>Alternate function selection for port x
              bit y (y = 8..15)</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>AFRH9</name>
          <description>Alternate function selection for port x
              bit y (y = 8..15)</description>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>AFRH8</name>
          <description>Alternate function selection for port x
              bit y (y = 8..15)</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BRR</name>
      <displayName>BRR</displayName>
      <description>Port bit reset register</description>
      <addressOffset>0x28</addressOffset>
      <size>0x20</size>
      <access>write-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BR0</name>
          <description>Port x Reset bit y</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR1</name>
          <description>Port x Reset bit y</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR2</name>
          <description>Port x Reset bit y</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR3</name>
          <description>Port x Reset bit y</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR4</name>
          <description>Port x Reset bit y</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR5</name>
          <description>Port x Reset bit y</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR6</name>
          <description>Port x Reset bit y</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR7</name>
          <description>Port x Reset bit y</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR8</name>
          <description>Port x Reset bit y</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR9</name>
          <description>Port x Reset bit y</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR10</name>
          <description>Port x Reset bit y</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR11</name>
          <description>Port x Reset bit y</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR12</name>
          <description>Port x Reset bit y</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR13</name>
          <description>Port x Reset bit y</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR14</name>
          <description>Port x Reset bit y</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR15</name>
          <description>Port x Reset bit y</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
</peripheral>*/
