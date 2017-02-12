/// MOD EXTI
/// External interrupt/event controller
const BASE_ADDRESS: u32 = 0x40010400;
/// Interrupt mask register
/// Size: 0x20 bits
pub mod imr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MR0_BIT_OFFSET: u8 = 0;
	const MR0_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 0 (Width: 1, Offset: 0)
	pub fn get_mr0() -> u8 { ::read(REGISTER_ADDRESS, MR0_BIT_OFFSET, MR0_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 0 (Width: 1, Offset: 0)
	pub fn set_mr0(value: u8) { ::write(REGISTER_ADDRESS, MR0_BIT_OFFSET, MR0_BIT_WIDTH, value as u32); }

	const MR1_BIT_OFFSET: u8 = 1;
	const MR1_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 1 (Width: 1, Offset: 1)
	pub fn get_mr1() -> u8 { ::read(REGISTER_ADDRESS, MR1_BIT_OFFSET, MR1_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 1 (Width: 1, Offset: 1)
	pub fn set_mr1(value: u8) { ::write(REGISTER_ADDRESS, MR1_BIT_OFFSET, MR1_BIT_WIDTH, value as u32); }

	const MR2_BIT_OFFSET: u8 = 2;
	const MR2_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 2 (Width: 1, Offset: 2)
	pub fn get_mr2() -> u8 { ::read(REGISTER_ADDRESS, MR2_BIT_OFFSET, MR2_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 2 (Width: 1, Offset: 2)
	pub fn set_mr2(value: u8) { ::write(REGISTER_ADDRESS, MR2_BIT_OFFSET, MR2_BIT_WIDTH, value as u32); }

	const MR3_BIT_OFFSET: u8 = 3;
	const MR3_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 3 (Width: 1, Offset: 3)
	pub fn get_mr3() -> u8 { ::read(REGISTER_ADDRESS, MR3_BIT_OFFSET, MR3_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 3 (Width: 1, Offset: 3)
	pub fn set_mr3(value: u8) { ::write(REGISTER_ADDRESS, MR3_BIT_OFFSET, MR3_BIT_WIDTH, value as u32); }

	const MR4_BIT_OFFSET: u8 = 4;
	const MR4_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 4 (Width: 1, Offset: 4)
	pub fn get_mr4() -> u8 { ::read(REGISTER_ADDRESS, MR4_BIT_OFFSET, MR4_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 4 (Width: 1, Offset: 4)
	pub fn set_mr4(value: u8) { ::write(REGISTER_ADDRESS, MR4_BIT_OFFSET, MR4_BIT_WIDTH, value as u32); }

	const MR5_BIT_OFFSET: u8 = 5;
	const MR5_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 5 (Width: 1, Offset: 5)
	pub fn get_mr5() -> u8 { ::read(REGISTER_ADDRESS, MR5_BIT_OFFSET, MR5_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 5 (Width: 1, Offset: 5)
	pub fn set_mr5(value: u8) { ::write(REGISTER_ADDRESS, MR5_BIT_OFFSET, MR5_BIT_WIDTH, value as u32); }

	const MR6_BIT_OFFSET: u8 = 6;
	const MR6_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 6 (Width: 1, Offset: 6)
	pub fn get_mr6() -> u8 { ::read(REGISTER_ADDRESS, MR6_BIT_OFFSET, MR6_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 6 (Width: 1, Offset: 6)
	pub fn set_mr6(value: u8) { ::write(REGISTER_ADDRESS, MR6_BIT_OFFSET, MR6_BIT_WIDTH, value as u32); }

	const MR7_BIT_OFFSET: u8 = 7;
	const MR7_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 7 (Width: 1, Offset: 7)
	pub fn get_mr7() -> u8 { ::read(REGISTER_ADDRESS, MR7_BIT_OFFSET, MR7_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 7 (Width: 1, Offset: 7)
	pub fn set_mr7(value: u8) { ::write(REGISTER_ADDRESS, MR7_BIT_OFFSET, MR7_BIT_WIDTH, value as u32); }

	const MR8_BIT_OFFSET: u8 = 8;
	const MR8_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 8 (Width: 1, Offset: 8)
	pub fn get_mr8() -> u8 { ::read(REGISTER_ADDRESS, MR8_BIT_OFFSET, MR8_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 8 (Width: 1, Offset: 8)
	pub fn set_mr8(value: u8) { ::write(REGISTER_ADDRESS, MR8_BIT_OFFSET, MR8_BIT_WIDTH, value as u32); }

	const MR9_BIT_OFFSET: u8 = 9;
	const MR9_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 9 (Width: 1, Offset: 9)
	pub fn get_mr9() -> u8 { ::read(REGISTER_ADDRESS, MR9_BIT_OFFSET, MR9_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 9 (Width: 1, Offset: 9)
	pub fn set_mr9(value: u8) { ::write(REGISTER_ADDRESS, MR9_BIT_OFFSET, MR9_BIT_WIDTH, value as u32); }

	const MR10_BIT_OFFSET: u8 = 10;
	const MR10_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 10 (Width: 1, Offset: 10)
	pub fn get_mr10() -> u8 { ::read(REGISTER_ADDRESS, MR10_BIT_OFFSET, MR10_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 10 (Width: 1, Offset: 10)
	pub fn set_mr10(value: u8) { ::write(REGISTER_ADDRESS, MR10_BIT_OFFSET, MR10_BIT_WIDTH, value as u32); }

	const MR11_BIT_OFFSET: u8 = 11;
	const MR11_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 11 (Width: 1, Offset: 11)
	pub fn get_mr11() -> u8 { ::read(REGISTER_ADDRESS, MR11_BIT_OFFSET, MR11_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 11 (Width: 1, Offset: 11)
	pub fn set_mr11(value: u8) { ::write(REGISTER_ADDRESS, MR11_BIT_OFFSET, MR11_BIT_WIDTH, value as u32); }

	const MR12_BIT_OFFSET: u8 = 12;
	const MR12_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 12 (Width: 1, Offset: 12)
	pub fn get_mr12() -> u8 { ::read(REGISTER_ADDRESS, MR12_BIT_OFFSET, MR12_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 12 (Width: 1, Offset: 12)
	pub fn set_mr12(value: u8) { ::write(REGISTER_ADDRESS, MR12_BIT_OFFSET, MR12_BIT_WIDTH, value as u32); }

	const MR13_BIT_OFFSET: u8 = 13;
	const MR13_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 13 (Width: 1, Offset: 13)
	pub fn get_mr13() -> u8 { ::read(REGISTER_ADDRESS, MR13_BIT_OFFSET, MR13_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 13 (Width: 1, Offset: 13)
	pub fn set_mr13(value: u8) { ::write(REGISTER_ADDRESS, MR13_BIT_OFFSET, MR13_BIT_WIDTH, value as u32); }

	const MR14_BIT_OFFSET: u8 = 14;
	const MR14_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 14 (Width: 1, Offset: 14)
	pub fn get_mr14() -> u8 { ::read(REGISTER_ADDRESS, MR14_BIT_OFFSET, MR14_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 14 (Width: 1, Offset: 14)
	pub fn set_mr14(value: u8) { ::write(REGISTER_ADDRESS, MR14_BIT_OFFSET, MR14_BIT_WIDTH, value as u32); }

	const MR15_BIT_OFFSET: u8 = 15;
	const MR15_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 15 (Width: 1, Offset: 15)
	pub fn get_mr15() -> u8 { ::read(REGISTER_ADDRESS, MR15_BIT_OFFSET, MR15_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 15 (Width: 1, Offset: 15)
	pub fn set_mr15(value: u8) { ::write(REGISTER_ADDRESS, MR15_BIT_OFFSET, MR15_BIT_WIDTH, value as u32); }

	const MR16_BIT_OFFSET: u8 = 16;
	const MR16_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 16 (Width: 1, Offset: 16)
	pub fn get_mr16() -> u8 { ::read(REGISTER_ADDRESS, MR16_BIT_OFFSET, MR16_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 16 (Width: 1, Offset: 16)
	pub fn set_mr16(value: u8) { ::write(REGISTER_ADDRESS, MR16_BIT_OFFSET, MR16_BIT_WIDTH, value as u32); }

	const MR17_BIT_OFFSET: u8 = 17;
	const MR17_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 17 (Width: 1, Offset: 17)
	pub fn get_mr17() -> u8 { ::read(REGISTER_ADDRESS, MR17_BIT_OFFSET, MR17_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 17 (Width: 1, Offset: 17)
	pub fn set_mr17(value: u8) { ::write(REGISTER_ADDRESS, MR17_BIT_OFFSET, MR17_BIT_WIDTH, value as u32); }

	const MR18_BIT_OFFSET: u8 = 18;
	const MR18_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 18 (Width: 1, Offset: 18)
	pub fn get_mr18() -> u8 { ::read(REGISTER_ADDRESS, MR18_BIT_OFFSET, MR18_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 18 (Width: 1, Offset: 18)
	pub fn set_mr18(value: u8) { ::write(REGISTER_ADDRESS, MR18_BIT_OFFSET, MR18_BIT_WIDTH, value as u32); }

	const MR19_BIT_OFFSET: u8 = 19;
	const MR19_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 19 (Width: 1, Offset: 19)
	pub fn get_mr19() -> u8 { ::read(REGISTER_ADDRESS, MR19_BIT_OFFSET, MR19_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 19 (Width: 1, Offset: 19)
	pub fn set_mr19(value: u8) { ::write(REGISTER_ADDRESS, MR19_BIT_OFFSET, MR19_BIT_WIDTH, value as u32); }

	const MR20_BIT_OFFSET: u8 = 20;
	const MR20_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 20 (Width: 1, Offset: 20)
	pub fn get_mr20() -> u8 { ::read(REGISTER_ADDRESS, MR20_BIT_OFFSET, MR20_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 20 (Width: 1, Offset: 20)
	pub fn set_mr20(value: u8) { ::write(REGISTER_ADDRESS, MR20_BIT_OFFSET, MR20_BIT_WIDTH, value as u32); }

	const MR21_BIT_OFFSET: u8 = 21;
	const MR21_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 21 (Width: 1, Offset: 21)
	pub fn get_mr21() -> u8 { ::read(REGISTER_ADDRESS, MR21_BIT_OFFSET, MR21_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 21 (Width: 1, Offset: 21)
	pub fn set_mr21(value: u8) { ::write(REGISTER_ADDRESS, MR21_BIT_OFFSET, MR21_BIT_WIDTH, value as u32); }

	const MR22_BIT_OFFSET: u8 = 22;
	const MR22_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 22 (Width: 1, Offset: 22)
	pub fn get_mr22() -> u8 { ::read(REGISTER_ADDRESS, MR22_BIT_OFFSET, MR22_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 22 (Width: 1, Offset: 22)
	pub fn set_mr22(value: u8) { ::write(REGISTER_ADDRESS, MR22_BIT_OFFSET, MR22_BIT_WIDTH, value as u32); }

	const MR23_BIT_OFFSET: u8 = 23;
	const MR23_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 23 (Width: 1, Offset: 23)
	pub fn get_mr23() -> u8 { ::read(REGISTER_ADDRESS, MR23_BIT_OFFSET, MR23_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 23 (Width: 1, Offset: 23)
	pub fn set_mr23(value: u8) { ::write(REGISTER_ADDRESS, MR23_BIT_OFFSET, MR23_BIT_WIDTH, value as u32); }

	const MR24_BIT_OFFSET: u8 = 24;
	const MR24_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 24 (Width: 1, Offset: 24)
	pub fn get_mr24() -> u8 { ::read(REGISTER_ADDRESS, MR24_BIT_OFFSET, MR24_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 24 (Width: 1, Offset: 24)
	pub fn set_mr24(value: u8) { ::write(REGISTER_ADDRESS, MR24_BIT_OFFSET, MR24_BIT_WIDTH, value as u32); }

	const MR25_BIT_OFFSET: u8 = 25;
	const MR25_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 25 (Width: 1, Offset: 25)
	pub fn get_mr25() -> u8 { ::read(REGISTER_ADDRESS, MR25_BIT_OFFSET, MR25_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 25 (Width: 1, Offset: 25)
	pub fn set_mr25(value: u8) { ::write(REGISTER_ADDRESS, MR25_BIT_OFFSET, MR25_BIT_WIDTH, value as u32); }

	const MR26_BIT_OFFSET: u8 = 26;
	const MR26_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 26 (Width: 1, Offset: 26)
	pub fn get_mr26() -> u8 { ::read(REGISTER_ADDRESS, MR26_BIT_OFFSET, MR26_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 26 (Width: 1, Offset: 26)
	pub fn set_mr26(value: u8) { ::write(REGISTER_ADDRESS, MR26_BIT_OFFSET, MR26_BIT_WIDTH, value as u32); }

	const MR27_BIT_OFFSET: u8 = 27;
	const MR27_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 27 (Width: 1, Offset: 27)
	pub fn get_mr27() -> u8 { ::read(REGISTER_ADDRESS, MR27_BIT_OFFSET, MR27_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 27 (Width: 1, Offset: 27)
	pub fn set_mr27(value: u8) { ::write(REGISTER_ADDRESS, MR27_BIT_OFFSET, MR27_BIT_WIDTH, value as u32); }

	const MR28_BIT_OFFSET: u8 = 28;
	const MR28_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 28 (Width: 1, Offset: 28)
	pub fn get_mr28() -> u8 { ::read(REGISTER_ADDRESS, MR28_BIT_OFFSET, MR28_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 28 (Width: 1, Offset: 28)
	pub fn set_mr28(value: u8) { ::write(REGISTER_ADDRESS, MR28_BIT_OFFSET, MR28_BIT_WIDTH, value as u32); }

	const MR29_BIT_OFFSET: u8 = 29;
	const MR29_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 29 (Width: 1, Offset: 29)
	pub fn get_mr29() -> u8 { ::read(REGISTER_ADDRESS, MR29_BIT_OFFSET, MR29_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 29 (Width: 1, Offset: 29)
	pub fn set_mr29(value: u8) { ::write(REGISTER_ADDRESS, MR29_BIT_OFFSET, MR29_BIT_WIDTH, value as u32); }

	const MR30_BIT_OFFSET: u8 = 30;
	const MR30_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 30 (Width: 1, Offset: 30)
	pub fn get_mr30() -> u8 { ::read(REGISTER_ADDRESS, MR30_BIT_OFFSET, MR30_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 30 (Width: 1, Offset: 30)
	pub fn set_mr30(value: u8) { ::write(REGISTER_ADDRESS, MR30_BIT_OFFSET, MR30_BIT_WIDTH, value as u32); }

	const MR31_BIT_OFFSET: u8 = 31;
	const MR31_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on line 31 (Width: 1, Offset: 31)
	pub fn get_mr31() -> u8 { ::read(REGISTER_ADDRESS, MR31_BIT_OFFSET, MR31_BIT_WIDTH) as u8 }
	/// Interrupt Mask on line 31 (Width: 1, Offset: 31)
	pub fn set_mr31(value: u8) { ::write(REGISTER_ADDRESS, MR31_BIT_OFFSET, MR31_BIT_WIDTH, value as u32); }
}
/// Event mask register
/// Size: 0x20 bits
pub mod emr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MR0_BIT_OFFSET: u8 = 0;
	const MR0_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 0 (Width: 1, Offset: 0)
	pub fn get_mr0() -> u8 { ::read(REGISTER_ADDRESS, MR0_BIT_OFFSET, MR0_BIT_WIDTH) as u8 }
	/// Event Mask on line 0 (Width: 1, Offset: 0)
	pub fn set_mr0(value: u8) { ::write(REGISTER_ADDRESS, MR0_BIT_OFFSET, MR0_BIT_WIDTH, value as u32); }

	const MR1_BIT_OFFSET: u8 = 1;
	const MR1_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 1 (Width: 1, Offset: 1)
	pub fn get_mr1() -> u8 { ::read(REGISTER_ADDRESS, MR1_BIT_OFFSET, MR1_BIT_WIDTH) as u8 }
	/// Event Mask on line 1 (Width: 1, Offset: 1)
	pub fn set_mr1(value: u8) { ::write(REGISTER_ADDRESS, MR1_BIT_OFFSET, MR1_BIT_WIDTH, value as u32); }

	const MR2_BIT_OFFSET: u8 = 2;
	const MR2_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 2 (Width: 1, Offset: 2)
	pub fn get_mr2() -> u8 { ::read(REGISTER_ADDRESS, MR2_BIT_OFFSET, MR2_BIT_WIDTH) as u8 }
	/// Event Mask on line 2 (Width: 1, Offset: 2)
	pub fn set_mr2(value: u8) { ::write(REGISTER_ADDRESS, MR2_BIT_OFFSET, MR2_BIT_WIDTH, value as u32); }

	const MR3_BIT_OFFSET: u8 = 3;
	const MR3_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 3 (Width: 1, Offset: 3)
	pub fn get_mr3() -> u8 { ::read(REGISTER_ADDRESS, MR3_BIT_OFFSET, MR3_BIT_WIDTH) as u8 }
	/// Event Mask on line 3 (Width: 1, Offset: 3)
	pub fn set_mr3(value: u8) { ::write(REGISTER_ADDRESS, MR3_BIT_OFFSET, MR3_BIT_WIDTH, value as u32); }

	const MR4_BIT_OFFSET: u8 = 4;
	const MR4_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 4 (Width: 1, Offset: 4)
	pub fn get_mr4() -> u8 { ::read(REGISTER_ADDRESS, MR4_BIT_OFFSET, MR4_BIT_WIDTH) as u8 }
	/// Event Mask on line 4 (Width: 1, Offset: 4)
	pub fn set_mr4(value: u8) { ::write(REGISTER_ADDRESS, MR4_BIT_OFFSET, MR4_BIT_WIDTH, value as u32); }

	const MR5_BIT_OFFSET: u8 = 5;
	const MR5_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 5 (Width: 1, Offset: 5)
	pub fn get_mr5() -> u8 { ::read(REGISTER_ADDRESS, MR5_BIT_OFFSET, MR5_BIT_WIDTH) as u8 }
	/// Event Mask on line 5 (Width: 1, Offset: 5)
	pub fn set_mr5(value: u8) { ::write(REGISTER_ADDRESS, MR5_BIT_OFFSET, MR5_BIT_WIDTH, value as u32); }

	const MR6_BIT_OFFSET: u8 = 6;
	const MR6_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 6 (Width: 1, Offset: 6)
	pub fn get_mr6() -> u8 { ::read(REGISTER_ADDRESS, MR6_BIT_OFFSET, MR6_BIT_WIDTH) as u8 }
	/// Event Mask on line 6 (Width: 1, Offset: 6)
	pub fn set_mr6(value: u8) { ::write(REGISTER_ADDRESS, MR6_BIT_OFFSET, MR6_BIT_WIDTH, value as u32); }

	const MR7_BIT_OFFSET: u8 = 7;
	const MR7_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 7 (Width: 1, Offset: 7)
	pub fn get_mr7() -> u8 { ::read(REGISTER_ADDRESS, MR7_BIT_OFFSET, MR7_BIT_WIDTH) as u8 }
	/// Event Mask on line 7 (Width: 1, Offset: 7)
	pub fn set_mr7(value: u8) { ::write(REGISTER_ADDRESS, MR7_BIT_OFFSET, MR7_BIT_WIDTH, value as u32); }

	const MR8_BIT_OFFSET: u8 = 8;
	const MR8_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 8 (Width: 1, Offset: 8)
	pub fn get_mr8() -> u8 { ::read(REGISTER_ADDRESS, MR8_BIT_OFFSET, MR8_BIT_WIDTH) as u8 }
	/// Event Mask on line 8 (Width: 1, Offset: 8)
	pub fn set_mr8(value: u8) { ::write(REGISTER_ADDRESS, MR8_BIT_OFFSET, MR8_BIT_WIDTH, value as u32); }

	const MR9_BIT_OFFSET: u8 = 9;
	const MR9_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 9 (Width: 1, Offset: 9)
	pub fn get_mr9() -> u8 { ::read(REGISTER_ADDRESS, MR9_BIT_OFFSET, MR9_BIT_WIDTH) as u8 }
	/// Event Mask on line 9 (Width: 1, Offset: 9)
	pub fn set_mr9(value: u8) { ::write(REGISTER_ADDRESS, MR9_BIT_OFFSET, MR9_BIT_WIDTH, value as u32); }

	const MR10_BIT_OFFSET: u8 = 10;
	const MR10_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 10 (Width: 1, Offset: 10)
	pub fn get_mr10() -> u8 { ::read(REGISTER_ADDRESS, MR10_BIT_OFFSET, MR10_BIT_WIDTH) as u8 }
	/// Event Mask on line 10 (Width: 1, Offset: 10)
	pub fn set_mr10(value: u8) { ::write(REGISTER_ADDRESS, MR10_BIT_OFFSET, MR10_BIT_WIDTH, value as u32); }

	const MR11_BIT_OFFSET: u8 = 11;
	const MR11_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 11 (Width: 1, Offset: 11)
	pub fn get_mr11() -> u8 { ::read(REGISTER_ADDRESS, MR11_BIT_OFFSET, MR11_BIT_WIDTH) as u8 }
	/// Event Mask on line 11 (Width: 1, Offset: 11)
	pub fn set_mr11(value: u8) { ::write(REGISTER_ADDRESS, MR11_BIT_OFFSET, MR11_BIT_WIDTH, value as u32); }

	const MR12_BIT_OFFSET: u8 = 12;
	const MR12_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 12 (Width: 1, Offset: 12)
	pub fn get_mr12() -> u8 { ::read(REGISTER_ADDRESS, MR12_BIT_OFFSET, MR12_BIT_WIDTH) as u8 }
	/// Event Mask on line 12 (Width: 1, Offset: 12)
	pub fn set_mr12(value: u8) { ::write(REGISTER_ADDRESS, MR12_BIT_OFFSET, MR12_BIT_WIDTH, value as u32); }

	const MR13_BIT_OFFSET: u8 = 13;
	const MR13_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 13 (Width: 1, Offset: 13)
	pub fn get_mr13() -> u8 { ::read(REGISTER_ADDRESS, MR13_BIT_OFFSET, MR13_BIT_WIDTH) as u8 }
	/// Event Mask on line 13 (Width: 1, Offset: 13)
	pub fn set_mr13(value: u8) { ::write(REGISTER_ADDRESS, MR13_BIT_OFFSET, MR13_BIT_WIDTH, value as u32); }

	const MR14_BIT_OFFSET: u8 = 14;
	const MR14_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 14 (Width: 1, Offset: 14)
	pub fn get_mr14() -> u8 { ::read(REGISTER_ADDRESS, MR14_BIT_OFFSET, MR14_BIT_WIDTH) as u8 }
	/// Event Mask on line 14 (Width: 1, Offset: 14)
	pub fn set_mr14(value: u8) { ::write(REGISTER_ADDRESS, MR14_BIT_OFFSET, MR14_BIT_WIDTH, value as u32); }

	const MR15_BIT_OFFSET: u8 = 15;
	const MR15_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 15 (Width: 1, Offset: 15)
	pub fn get_mr15() -> u8 { ::read(REGISTER_ADDRESS, MR15_BIT_OFFSET, MR15_BIT_WIDTH) as u8 }
	/// Event Mask on line 15 (Width: 1, Offset: 15)
	pub fn set_mr15(value: u8) { ::write(REGISTER_ADDRESS, MR15_BIT_OFFSET, MR15_BIT_WIDTH, value as u32); }

	const MR16_BIT_OFFSET: u8 = 16;
	const MR16_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 16 (Width: 1, Offset: 16)
	pub fn get_mr16() -> u8 { ::read(REGISTER_ADDRESS, MR16_BIT_OFFSET, MR16_BIT_WIDTH) as u8 }
	/// Event Mask on line 16 (Width: 1, Offset: 16)
	pub fn set_mr16(value: u8) { ::write(REGISTER_ADDRESS, MR16_BIT_OFFSET, MR16_BIT_WIDTH, value as u32); }

	const MR17_BIT_OFFSET: u8 = 17;
	const MR17_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 17 (Width: 1, Offset: 17)
	pub fn get_mr17() -> u8 { ::read(REGISTER_ADDRESS, MR17_BIT_OFFSET, MR17_BIT_WIDTH) as u8 }
	/// Event Mask on line 17 (Width: 1, Offset: 17)
	pub fn set_mr17(value: u8) { ::write(REGISTER_ADDRESS, MR17_BIT_OFFSET, MR17_BIT_WIDTH, value as u32); }

	const MR18_BIT_OFFSET: u8 = 18;
	const MR18_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 18 (Width: 1, Offset: 18)
	pub fn get_mr18() -> u8 { ::read(REGISTER_ADDRESS, MR18_BIT_OFFSET, MR18_BIT_WIDTH) as u8 }
	/// Event Mask on line 18 (Width: 1, Offset: 18)
	pub fn set_mr18(value: u8) { ::write(REGISTER_ADDRESS, MR18_BIT_OFFSET, MR18_BIT_WIDTH, value as u32); }

	const MR19_BIT_OFFSET: u8 = 19;
	const MR19_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 19 (Width: 1, Offset: 19)
	pub fn get_mr19() -> u8 { ::read(REGISTER_ADDRESS, MR19_BIT_OFFSET, MR19_BIT_WIDTH) as u8 }
	/// Event Mask on line 19 (Width: 1, Offset: 19)
	pub fn set_mr19(value: u8) { ::write(REGISTER_ADDRESS, MR19_BIT_OFFSET, MR19_BIT_WIDTH, value as u32); }

	const MR20_BIT_OFFSET: u8 = 20;
	const MR20_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 20 (Width: 1, Offset: 20)
	pub fn get_mr20() -> u8 { ::read(REGISTER_ADDRESS, MR20_BIT_OFFSET, MR20_BIT_WIDTH) as u8 }
	/// Event Mask on line 20 (Width: 1, Offset: 20)
	pub fn set_mr20(value: u8) { ::write(REGISTER_ADDRESS, MR20_BIT_OFFSET, MR20_BIT_WIDTH, value as u32); }

	const MR21_BIT_OFFSET: u8 = 21;
	const MR21_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 21 (Width: 1, Offset: 21)
	pub fn get_mr21() -> u8 { ::read(REGISTER_ADDRESS, MR21_BIT_OFFSET, MR21_BIT_WIDTH) as u8 }
	/// Event Mask on line 21 (Width: 1, Offset: 21)
	pub fn set_mr21(value: u8) { ::write(REGISTER_ADDRESS, MR21_BIT_OFFSET, MR21_BIT_WIDTH, value as u32); }

	const MR22_BIT_OFFSET: u8 = 22;
	const MR22_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 22 (Width: 1, Offset: 22)
	pub fn get_mr22() -> u8 { ::read(REGISTER_ADDRESS, MR22_BIT_OFFSET, MR22_BIT_WIDTH) as u8 }
	/// Event Mask on line 22 (Width: 1, Offset: 22)
	pub fn set_mr22(value: u8) { ::write(REGISTER_ADDRESS, MR22_BIT_OFFSET, MR22_BIT_WIDTH, value as u32); }

	const MR23_BIT_OFFSET: u8 = 23;
	const MR23_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 23 (Width: 1, Offset: 23)
	pub fn get_mr23() -> u8 { ::read(REGISTER_ADDRESS, MR23_BIT_OFFSET, MR23_BIT_WIDTH) as u8 }
	/// Event Mask on line 23 (Width: 1, Offset: 23)
	pub fn set_mr23(value: u8) { ::write(REGISTER_ADDRESS, MR23_BIT_OFFSET, MR23_BIT_WIDTH, value as u32); }

	const MR24_BIT_OFFSET: u8 = 24;
	const MR24_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 24 (Width: 1, Offset: 24)
	pub fn get_mr24() -> u8 { ::read(REGISTER_ADDRESS, MR24_BIT_OFFSET, MR24_BIT_WIDTH) as u8 }
	/// Event Mask on line 24 (Width: 1, Offset: 24)
	pub fn set_mr24(value: u8) { ::write(REGISTER_ADDRESS, MR24_BIT_OFFSET, MR24_BIT_WIDTH, value as u32); }

	const MR25_BIT_OFFSET: u8 = 25;
	const MR25_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 25 (Width: 1, Offset: 25)
	pub fn get_mr25() -> u8 { ::read(REGISTER_ADDRESS, MR25_BIT_OFFSET, MR25_BIT_WIDTH) as u8 }
	/// Event Mask on line 25 (Width: 1, Offset: 25)
	pub fn set_mr25(value: u8) { ::write(REGISTER_ADDRESS, MR25_BIT_OFFSET, MR25_BIT_WIDTH, value as u32); }

	const MR26_BIT_OFFSET: u8 = 26;
	const MR26_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 26 (Width: 1, Offset: 26)
	pub fn get_mr26() -> u8 { ::read(REGISTER_ADDRESS, MR26_BIT_OFFSET, MR26_BIT_WIDTH) as u8 }
	/// Event Mask on line 26 (Width: 1, Offset: 26)
	pub fn set_mr26(value: u8) { ::write(REGISTER_ADDRESS, MR26_BIT_OFFSET, MR26_BIT_WIDTH, value as u32); }

	const MR27_BIT_OFFSET: u8 = 27;
	const MR27_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 27 (Width: 1, Offset: 27)
	pub fn get_mr27() -> u8 { ::read(REGISTER_ADDRESS, MR27_BIT_OFFSET, MR27_BIT_WIDTH) as u8 }
	/// Event Mask on line 27 (Width: 1, Offset: 27)
	pub fn set_mr27(value: u8) { ::write(REGISTER_ADDRESS, MR27_BIT_OFFSET, MR27_BIT_WIDTH, value as u32); }

	const MR28_BIT_OFFSET: u8 = 28;
	const MR28_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 28 (Width: 1, Offset: 28)
	pub fn get_mr28() -> u8 { ::read(REGISTER_ADDRESS, MR28_BIT_OFFSET, MR28_BIT_WIDTH) as u8 }
	/// Event Mask on line 28 (Width: 1, Offset: 28)
	pub fn set_mr28(value: u8) { ::write(REGISTER_ADDRESS, MR28_BIT_OFFSET, MR28_BIT_WIDTH, value as u32); }

	const MR29_BIT_OFFSET: u8 = 29;
	const MR29_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 29 (Width: 1, Offset: 29)
	pub fn get_mr29() -> u8 { ::read(REGISTER_ADDRESS, MR29_BIT_OFFSET, MR29_BIT_WIDTH) as u8 }
	/// Event Mask on line 29 (Width: 1, Offset: 29)
	pub fn set_mr29(value: u8) { ::write(REGISTER_ADDRESS, MR29_BIT_OFFSET, MR29_BIT_WIDTH, value as u32); }

	const MR30_BIT_OFFSET: u8 = 30;
	const MR30_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 30 (Width: 1, Offset: 30)
	pub fn get_mr30() -> u8 { ::read(REGISTER_ADDRESS, MR30_BIT_OFFSET, MR30_BIT_WIDTH) as u8 }
	/// Event Mask on line 30 (Width: 1, Offset: 30)
	pub fn set_mr30(value: u8) { ::write(REGISTER_ADDRESS, MR30_BIT_OFFSET, MR30_BIT_WIDTH, value as u32); }

	const MR31_BIT_OFFSET: u8 = 31;
	const MR31_BIT_WIDTH: u8 = 1;
	/// Event Mask on line 31 (Width: 1, Offset: 31)
	pub fn get_mr31() -> u8 { ::read(REGISTER_ADDRESS, MR31_BIT_OFFSET, MR31_BIT_WIDTH) as u8 }
	/// Event Mask on line 31 (Width: 1, Offset: 31)
	pub fn set_mr31(value: u8) { ::write(REGISTER_ADDRESS, MR31_BIT_OFFSET, MR31_BIT_WIDTH, value as u32); }
}
/// Rising Trigger selection register
/// Size: 0x20 bits
pub mod rtsr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TR0_BIT_OFFSET: u8 = 0;
	const TR0_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 0 (Width: 1, Offset: 0)
	pub fn get_tr0() -> u8 { ::read(REGISTER_ADDRESS, TR0_BIT_OFFSET, TR0_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 0 (Width: 1, Offset: 0)
	pub fn set_tr0(value: u8) { ::write(REGISTER_ADDRESS, TR0_BIT_OFFSET, TR0_BIT_WIDTH, value as u32); }

	const TR1_BIT_OFFSET: u8 = 1;
	const TR1_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 1 (Width: 1, Offset: 1)
	pub fn get_tr1() -> u8 { ::read(REGISTER_ADDRESS, TR1_BIT_OFFSET, TR1_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 1 (Width: 1, Offset: 1)
	pub fn set_tr1(value: u8) { ::write(REGISTER_ADDRESS, TR1_BIT_OFFSET, TR1_BIT_WIDTH, value as u32); }

	const TR2_BIT_OFFSET: u8 = 2;
	const TR2_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 2 (Width: 1, Offset: 2)
	pub fn get_tr2() -> u8 { ::read(REGISTER_ADDRESS, TR2_BIT_OFFSET, TR2_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 2 (Width: 1, Offset: 2)
	pub fn set_tr2(value: u8) { ::write(REGISTER_ADDRESS, TR2_BIT_OFFSET, TR2_BIT_WIDTH, value as u32); }

	const TR3_BIT_OFFSET: u8 = 3;
	const TR3_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 3 (Width: 1, Offset: 3)
	pub fn get_tr3() -> u8 { ::read(REGISTER_ADDRESS, TR3_BIT_OFFSET, TR3_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 3 (Width: 1, Offset: 3)
	pub fn set_tr3(value: u8) { ::write(REGISTER_ADDRESS, TR3_BIT_OFFSET, TR3_BIT_WIDTH, value as u32); }

	const TR4_BIT_OFFSET: u8 = 4;
	const TR4_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 4 (Width: 1, Offset: 4)
	pub fn get_tr4() -> u8 { ::read(REGISTER_ADDRESS, TR4_BIT_OFFSET, TR4_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 4 (Width: 1, Offset: 4)
	pub fn set_tr4(value: u8) { ::write(REGISTER_ADDRESS, TR4_BIT_OFFSET, TR4_BIT_WIDTH, value as u32); }

	const TR5_BIT_OFFSET: u8 = 5;
	const TR5_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 5 (Width: 1, Offset: 5)
	pub fn get_tr5() -> u8 { ::read(REGISTER_ADDRESS, TR5_BIT_OFFSET, TR5_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 5 (Width: 1, Offset: 5)
	pub fn set_tr5(value: u8) { ::write(REGISTER_ADDRESS, TR5_BIT_OFFSET, TR5_BIT_WIDTH, value as u32); }

	const TR6_BIT_OFFSET: u8 = 6;
	const TR6_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 6 (Width: 1, Offset: 6)
	pub fn get_tr6() -> u8 { ::read(REGISTER_ADDRESS, TR6_BIT_OFFSET, TR6_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 6 (Width: 1, Offset: 6)
	pub fn set_tr6(value: u8) { ::write(REGISTER_ADDRESS, TR6_BIT_OFFSET, TR6_BIT_WIDTH, value as u32); }

	const TR7_BIT_OFFSET: u8 = 7;
	const TR7_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 7 (Width: 1, Offset: 7)
	pub fn get_tr7() -> u8 { ::read(REGISTER_ADDRESS, TR7_BIT_OFFSET, TR7_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 7 (Width: 1, Offset: 7)
	pub fn set_tr7(value: u8) { ::write(REGISTER_ADDRESS, TR7_BIT_OFFSET, TR7_BIT_WIDTH, value as u32); }

	const TR8_BIT_OFFSET: u8 = 8;
	const TR8_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 8 (Width: 1, Offset: 8)
	pub fn get_tr8() -> u8 { ::read(REGISTER_ADDRESS, TR8_BIT_OFFSET, TR8_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 8 (Width: 1, Offset: 8)
	pub fn set_tr8(value: u8) { ::write(REGISTER_ADDRESS, TR8_BIT_OFFSET, TR8_BIT_WIDTH, value as u32); }

	const TR9_BIT_OFFSET: u8 = 9;
	const TR9_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 9 (Width: 1, Offset: 9)
	pub fn get_tr9() -> u8 { ::read(REGISTER_ADDRESS, TR9_BIT_OFFSET, TR9_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 9 (Width: 1, Offset: 9)
	pub fn set_tr9(value: u8) { ::write(REGISTER_ADDRESS, TR9_BIT_OFFSET, TR9_BIT_WIDTH, value as u32); }

	const TR10_BIT_OFFSET: u8 = 10;
	const TR10_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 10 (Width: 1, Offset: 10)
	pub fn get_tr10() -> u8 { ::read(REGISTER_ADDRESS, TR10_BIT_OFFSET, TR10_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 10 (Width: 1, Offset: 10)
	pub fn set_tr10(value: u8) { ::write(REGISTER_ADDRESS, TR10_BIT_OFFSET, TR10_BIT_WIDTH, value as u32); }

	const TR11_BIT_OFFSET: u8 = 11;
	const TR11_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 11 (Width: 1, Offset: 11)
	pub fn get_tr11() -> u8 { ::read(REGISTER_ADDRESS, TR11_BIT_OFFSET, TR11_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 11 (Width: 1, Offset: 11)
	pub fn set_tr11(value: u8) { ::write(REGISTER_ADDRESS, TR11_BIT_OFFSET, TR11_BIT_WIDTH, value as u32); }

	const TR12_BIT_OFFSET: u8 = 12;
	const TR12_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 12 (Width: 1, Offset: 12)
	pub fn get_tr12() -> u8 { ::read(REGISTER_ADDRESS, TR12_BIT_OFFSET, TR12_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 12 (Width: 1, Offset: 12)
	pub fn set_tr12(value: u8) { ::write(REGISTER_ADDRESS, TR12_BIT_OFFSET, TR12_BIT_WIDTH, value as u32); }

	const TR13_BIT_OFFSET: u8 = 13;
	const TR13_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 13 (Width: 1, Offset: 13)
	pub fn get_tr13() -> u8 { ::read(REGISTER_ADDRESS, TR13_BIT_OFFSET, TR13_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 13 (Width: 1, Offset: 13)
	pub fn set_tr13(value: u8) { ::write(REGISTER_ADDRESS, TR13_BIT_OFFSET, TR13_BIT_WIDTH, value as u32); }

	const TR14_BIT_OFFSET: u8 = 14;
	const TR14_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 14 (Width: 1, Offset: 14)
	pub fn get_tr14() -> u8 { ::read(REGISTER_ADDRESS, TR14_BIT_OFFSET, TR14_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 14 (Width: 1, Offset: 14)
	pub fn set_tr14(value: u8) { ::write(REGISTER_ADDRESS, TR14_BIT_OFFSET, TR14_BIT_WIDTH, value as u32); }

	const TR15_BIT_OFFSET: u8 = 15;
	const TR15_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 15 (Width: 1, Offset: 15)
	pub fn get_tr15() -> u8 { ::read(REGISTER_ADDRESS, TR15_BIT_OFFSET, TR15_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 15 (Width: 1, Offset: 15)
	pub fn set_tr15(value: u8) { ::write(REGISTER_ADDRESS, TR15_BIT_OFFSET, TR15_BIT_WIDTH, value as u32); }

	const TR16_BIT_OFFSET: u8 = 16;
	const TR16_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 16 (Width: 1, Offset: 16)
	pub fn get_tr16() -> u8 { ::read(REGISTER_ADDRESS, TR16_BIT_OFFSET, TR16_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 16 (Width: 1, Offset: 16)
	pub fn set_tr16(value: u8) { ::write(REGISTER_ADDRESS, TR16_BIT_OFFSET, TR16_BIT_WIDTH, value as u32); }

	const TR17_BIT_OFFSET: u8 = 17;
	const TR17_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 17 (Width: 1, Offset: 17)
	pub fn get_tr17() -> u8 { ::read(REGISTER_ADDRESS, TR17_BIT_OFFSET, TR17_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 17 (Width: 1, Offset: 17)
	pub fn set_tr17(value: u8) { ::write(REGISTER_ADDRESS, TR17_BIT_OFFSET, TR17_BIT_WIDTH, value as u32); }

	const TR18_BIT_OFFSET: u8 = 18;
	const TR18_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 18 (Width: 1, Offset: 18)
	pub fn get_tr18() -> u8 { ::read(REGISTER_ADDRESS, TR18_BIT_OFFSET, TR18_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 18 (Width: 1, Offset: 18)
	pub fn set_tr18(value: u8) { ::write(REGISTER_ADDRESS, TR18_BIT_OFFSET, TR18_BIT_WIDTH, value as u32); }

	const TR19_BIT_OFFSET: u8 = 19;
	const TR19_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 19 (Width: 1, Offset: 19)
	pub fn get_tr19() -> u8 { ::read(REGISTER_ADDRESS, TR19_BIT_OFFSET, TR19_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 19 (Width: 1, Offset: 19)
	pub fn set_tr19(value: u8) { ::write(REGISTER_ADDRESS, TR19_BIT_OFFSET, TR19_BIT_WIDTH, value as u32); }

	const TR20_BIT_OFFSET: u8 = 20;
	const TR20_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 20 (Width: 1, Offset: 20)
	pub fn get_tr20() -> u8 { ::read(REGISTER_ADDRESS, TR20_BIT_OFFSET, TR20_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 20 (Width: 1, Offset: 20)
	pub fn set_tr20(value: u8) { ::write(REGISTER_ADDRESS, TR20_BIT_OFFSET, TR20_BIT_WIDTH, value as u32); }

	const TR21_BIT_OFFSET: u8 = 21;
	const TR21_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 21 (Width: 1, Offset: 21)
	pub fn get_tr21() -> u8 { ::read(REGISTER_ADDRESS, TR21_BIT_OFFSET, TR21_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 21 (Width: 1, Offset: 21)
	pub fn set_tr21(value: u8) { ::write(REGISTER_ADDRESS, TR21_BIT_OFFSET, TR21_BIT_WIDTH, value as u32); }

	const TR22_BIT_OFFSET: u8 = 22;
	const TR22_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 22 (Width: 1, Offset: 22)
	pub fn get_tr22() -> u8 { ::read(REGISTER_ADDRESS, TR22_BIT_OFFSET, TR22_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 22 (Width: 1, Offset: 22)
	pub fn set_tr22(value: u8) { ::write(REGISTER_ADDRESS, TR22_BIT_OFFSET, TR22_BIT_WIDTH, value as u32); }

	const TR29_BIT_OFFSET: u8 = 29;
	const TR29_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 29 (Width: 1, Offset: 29)
	pub fn get_tr29() -> u8 { ::read(REGISTER_ADDRESS, TR29_BIT_OFFSET, TR29_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 29 (Width: 1, Offset: 29)
	pub fn set_tr29(value: u8) { ::write(REGISTER_ADDRESS, TR29_BIT_OFFSET, TR29_BIT_WIDTH, value as u32); }

	const TR30_BIT_OFFSET: u8 = 30;
	const TR30_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 30 (Width: 1, Offset: 30)
	pub fn get_tr30() -> u8 { ::read(REGISTER_ADDRESS, TR30_BIT_OFFSET, TR30_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 30 (Width: 1, Offset: 30)
	pub fn set_tr30(value: u8) { ::write(REGISTER_ADDRESS, TR30_BIT_OFFSET, TR30_BIT_WIDTH, value as u32); }

	const TR31_BIT_OFFSET: u8 = 31;
	const TR31_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration of line 31 (Width: 1, Offset: 31)
	pub fn get_tr31() -> u8 { ::read(REGISTER_ADDRESS, TR31_BIT_OFFSET, TR31_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration of line 31 (Width: 1, Offset: 31)
	pub fn set_tr31(value: u8) { ::write(REGISTER_ADDRESS, TR31_BIT_OFFSET, TR31_BIT_WIDTH, value as u32); }
}
/// Falling Trigger selection register
/// Size: 0x20 bits
pub mod ftsr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TR0_BIT_OFFSET: u8 = 0;
	const TR0_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 0 (Width: 1, Offset: 0)
	pub fn get_tr0() -> u8 { ::read(REGISTER_ADDRESS, TR0_BIT_OFFSET, TR0_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 0 (Width: 1, Offset: 0)
	pub fn set_tr0(value: u8) { ::write(REGISTER_ADDRESS, TR0_BIT_OFFSET, TR0_BIT_WIDTH, value as u32); }

	const TR1_BIT_OFFSET: u8 = 1;
	const TR1_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 1 (Width: 1, Offset: 1)
	pub fn get_tr1() -> u8 { ::read(REGISTER_ADDRESS, TR1_BIT_OFFSET, TR1_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 1 (Width: 1, Offset: 1)
	pub fn set_tr1(value: u8) { ::write(REGISTER_ADDRESS, TR1_BIT_OFFSET, TR1_BIT_WIDTH, value as u32); }

	const TR2_BIT_OFFSET: u8 = 2;
	const TR2_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 2 (Width: 1, Offset: 2)
	pub fn get_tr2() -> u8 { ::read(REGISTER_ADDRESS, TR2_BIT_OFFSET, TR2_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 2 (Width: 1, Offset: 2)
	pub fn set_tr2(value: u8) { ::write(REGISTER_ADDRESS, TR2_BIT_OFFSET, TR2_BIT_WIDTH, value as u32); }

	const TR3_BIT_OFFSET: u8 = 3;
	const TR3_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 3 (Width: 1, Offset: 3)
	pub fn get_tr3() -> u8 { ::read(REGISTER_ADDRESS, TR3_BIT_OFFSET, TR3_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 3 (Width: 1, Offset: 3)
	pub fn set_tr3(value: u8) { ::write(REGISTER_ADDRESS, TR3_BIT_OFFSET, TR3_BIT_WIDTH, value as u32); }

	const TR4_BIT_OFFSET: u8 = 4;
	const TR4_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 4 (Width: 1, Offset: 4)
	pub fn get_tr4() -> u8 { ::read(REGISTER_ADDRESS, TR4_BIT_OFFSET, TR4_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 4 (Width: 1, Offset: 4)
	pub fn set_tr4(value: u8) { ::write(REGISTER_ADDRESS, TR4_BIT_OFFSET, TR4_BIT_WIDTH, value as u32); }

	const TR5_BIT_OFFSET: u8 = 5;
	const TR5_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 5 (Width: 1, Offset: 5)
	pub fn get_tr5() -> u8 { ::read(REGISTER_ADDRESS, TR5_BIT_OFFSET, TR5_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 5 (Width: 1, Offset: 5)
	pub fn set_tr5(value: u8) { ::write(REGISTER_ADDRESS, TR5_BIT_OFFSET, TR5_BIT_WIDTH, value as u32); }

	const TR6_BIT_OFFSET: u8 = 6;
	const TR6_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 6 (Width: 1, Offset: 6)
	pub fn get_tr6() -> u8 { ::read(REGISTER_ADDRESS, TR6_BIT_OFFSET, TR6_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 6 (Width: 1, Offset: 6)
	pub fn set_tr6(value: u8) { ::write(REGISTER_ADDRESS, TR6_BIT_OFFSET, TR6_BIT_WIDTH, value as u32); }

	const TR7_BIT_OFFSET: u8 = 7;
	const TR7_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 7 (Width: 1, Offset: 7)
	pub fn get_tr7() -> u8 { ::read(REGISTER_ADDRESS, TR7_BIT_OFFSET, TR7_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 7 (Width: 1, Offset: 7)
	pub fn set_tr7(value: u8) { ::write(REGISTER_ADDRESS, TR7_BIT_OFFSET, TR7_BIT_WIDTH, value as u32); }

	const TR8_BIT_OFFSET: u8 = 8;
	const TR8_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 8 (Width: 1, Offset: 8)
	pub fn get_tr8() -> u8 { ::read(REGISTER_ADDRESS, TR8_BIT_OFFSET, TR8_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 8 (Width: 1, Offset: 8)
	pub fn set_tr8(value: u8) { ::write(REGISTER_ADDRESS, TR8_BIT_OFFSET, TR8_BIT_WIDTH, value as u32); }

	const TR9_BIT_OFFSET: u8 = 9;
	const TR9_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 9 (Width: 1, Offset: 9)
	pub fn get_tr9() -> u8 { ::read(REGISTER_ADDRESS, TR9_BIT_OFFSET, TR9_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 9 (Width: 1, Offset: 9)
	pub fn set_tr9(value: u8) { ::write(REGISTER_ADDRESS, TR9_BIT_OFFSET, TR9_BIT_WIDTH, value as u32); }

	const TR10_BIT_OFFSET: u8 = 10;
	const TR10_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 10 (Width: 1, Offset: 10)
	pub fn get_tr10() -> u8 { ::read(REGISTER_ADDRESS, TR10_BIT_OFFSET, TR10_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 10 (Width: 1, Offset: 10)
	pub fn set_tr10(value: u8) { ::write(REGISTER_ADDRESS, TR10_BIT_OFFSET, TR10_BIT_WIDTH, value as u32); }

	const TR11_BIT_OFFSET: u8 = 11;
	const TR11_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 11 (Width: 1, Offset: 11)
	pub fn get_tr11() -> u8 { ::read(REGISTER_ADDRESS, TR11_BIT_OFFSET, TR11_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 11 (Width: 1, Offset: 11)
	pub fn set_tr11(value: u8) { ::write(REGISTER_ADDRESS, TR11_BIT_OFFSET, TR11_BIT_WIDTH, value as u32); }

	const TR12_BIT_OFFSET: u8 = 12;
	const TR12_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 12 (Width: 1, Offset: 12)
	pub fn get_tr12() -> u8 { ::read(REGISTER_ADDRESS, TR12_BIT_OFFSET, TR12_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 12 (Width: 1, Offset: 12)
	pub fn set_tr12(value: u8) { ::write(REGISTER_ADDRESS, TR12_BIT_OFFSET, TR12_BIT_WIDTH, value as u32); }

	const TR13_BIT_OFFSET: u8 = 13;
	const TR13_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 13 (Width: 1, Offset: 13)
	pub fn get_tr13() -> u8 { ::read(REGISTER_ADDRESS, TR13_BIT_OFFSET, TR13_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 13 (Width: 1, Offset: 13)
	pub fn set_tr13(value: u8) { ::write(REGISTER_ADDRESS, TR13_BIT_OFFSET, TR13_BIT_WIDTH, value as u32); }

	const TR14_BIT_OFFSET: u8 = 14;
	const TR14_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 14 (Width: 1, Offset: 14)
	pub fn get_tr14() -> u8 { ::read(REGISTER_ADDRESS, TR14_BIT_OFFSET, TR14_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 14 (Width: 1, Offset: 14)
	pub fn set_tr14(value: u8) { ::write(REGISTER_ADDRESS, TR14_BIT_OFFSET, TR14_BIT_WIDTH, value as u32); }

	const TR15_BIT_OFFSET: u8 = 15;
	const TR15_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 15 (Width: 1, Offset: 15)
	pub fn get_tr15() -> u8 { ::read(REGISTER_ADDRESS, TR15_BIT_OFFSET, TR15_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 15 (Width: 1, Offset: 15)
	pub fn set_tr15(value: u8) { ::write(REGISTER_ADDRESS, TR15_BIT_OFFSET, TR15_BIT_WIDTH, value as u32); }

	const TR16_BIT_OFFSET: u8 = 16;
	const TR16_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 16 (Width: 1, Offset: 16)
	pub fn get_tr16() -> u8 { ::read(REGISTER_ADDRESS, TR16_BIT_OFFSET, TR16_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 16 (Width: 1, Offset: 16)
	pub fn set_tr16(value: u8) { ::write(REGISTER_ADDRESS, TR16_BIT_OFFSET, TR16_BIT_WIDTH, value as u32); }

	const TR17_BIT_OFFSET: u8 = 17;
	const TR17_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 17 (Width: 1, Offset: 17)
	pub fn get_tr17() -> u8 { ::read(REGISTER_ADDRESS, TR17_BIT_OFFSET, TR17_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 17 (Width: 1, Offset: 17)
	pub fn set_tr17(value: u8) { ::write(REGISTER_ADDRESS, TR17_BIT_OFFSET, TR17_BIT_WIDTH, value as u32); }

	const TR18_BIT_OFFSET: u8 = 18;
	const TR18_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 18 (Width: 1, Offset: 18)
	pub fn get_tr18() -> u8 { ::read(REGISTER_ADDRESS, TR18_BIT_OFFSET, TR18_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 18 (Width: 1, Offset: 18)
	pub fn set_tr18(value: u8) { ::write(REGISTER_ADDRESS, TR18_BIT_OFFSET, TR18_BIT_WIDTH, value as u32); }

	const TR19_BIT_OFFSET: u8 = 19;
	const TR19_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 19 (Width: 1, Offset: 19)
	pub fn get_tr19() -> u8 { ::read(REGISTER_ADDRESS, TR19_BIT_OFFSET, TR19_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 19 (Width: 1, Offset: 19)
	pub fn set_tr19(value: u8) { ::write(REGISTER_ADDRESS, TR19_BIT_OFFSET, TR19_BIT_WIDTH, value as u32); }

	const TR20_BIT_OFFSET: u8 = 20;
	const TR20_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 20 (Width: 1, Offset: 20)
	pub fn get_tr20() -> u8 { ::read(REGISTER_ADDRESS, TR20_BIT_OFFSET, TR20_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 20 (Width: 1, Offset: 20)
	pub fn set_tr20(value: u8) { ::write(REGISTER_ADDRESS, TR20_BIT_OFFSET, TR20_BIT_WIDTH, value as u32); }

	const TR21_BIT_OFFSET: u8 = 21;
	const TR21_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 21 (Width: 1, Offset: 21)
	pub fn get_tr21() -> u8 { ::read(REGISTER_ADDRESS, TR21_BIT_OFFSET, TR21_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 21 (Width: 1, Offset: 21)
	pub fn set_tr21(value: u8) { ::write(REGISTER_ADDRESS, TR21_BIT_OFFSET, TR21_BIT_WIDTH, value as u32); }

	const TR22_BIT_OFFSET: u8 = 22;
	const TR22_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 22 (Width: 1, Offset: 22)
	pub fn get_tr22() -> u8 { ::read(REGISTER_ADDRESS, TR22_BIT_OFFSET, TR22_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 22 (Width: 1, Offset: 22)
	pub fn set_tr22(value: u8) { ::write(REGISTER_ADDRESS, TR22_BIT_OFFSET, TR22_BIT_WIDTH, value as u32); }

	const TR29_BIT_OFFSET: u8 = 29;
	const TR29_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 29 (Width: 1, Offset: 29)
	pub fn get_tr29() -> u8 { ::read(REGISTER_ADDRESS, TR29_BIT_OFFSET, TR29_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 29 (Width: 1, Offset: 29)
	pub fn set_tr29(value: u8) { ::write(REGISTER_ADDRESS, TR29_BIT_OFFSET, TR29_BIT_WIDTH, value as u32); }

	const TR30_BIT_OFFSET: u8 = 30;
	const TR30_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 30. (Width: 1, Offset: 30)
	pub fn get_tr30() -> u8 { ::read(REGISTER_ADDRESS, TR30_BIT_OFFSET, TR30_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 30. (Width: 1, Offset: 30)
	pub fn set_tr30(value: u8) { ::write(REGISTER_ADDRESS, TR30_BIT_OFFSET, TR30_BIT_WIDTH, value as u32); }

	const TR31_BIT_OFFSET: u8 = 31;
	const TR31_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration of line 31 (Width: 1, Offset: 31)
	pub fn get_tr31() -> u8 { ::read(REGISTER_ADDRESS, TR31_BIT_OFFSET, TR31_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration of line 31 (Width: 1, Offset: 31)
	pub fn set_tr31(value: u8) { ::write(REGISTER_ADDRESS, TR31_BIT_OFFSET, TR31_BIT_WIDTH, value as u32); }
}
/// Software interrupt event register
/// Size: 0x20 bits
pub mod swier1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SWIER0_BIT_OFFSET: u8 = 0;
	const SWIER0_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 0 (Width: 1, Offset: 0)
	pub fn get_swier0() -> u8 { ::read(REGISTER_ADDRESS, SWIER0_BIT_OFFSET, SWIER0_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 0 (Width: 1, Offset: 0)
	pub fn set_swier0(value: u8) { ::write(REGISTER_ADDRESS, SWIER0_BIT_OFFSET, SWIER0_BIT_WIDTH, value as u32); }

	const SWIER1_BIT_OFFSET: u8 = 1;
	const SWIER1_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 1 (Width: 1, Offset: 1)
	pub fn get_swier1() -> u8 { ::read(REGISTER_ADDRESS, SWIER1_BIT_OFFSET, SWIER1_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 1 (Width: 1, Offset: 1)
	pub fn set_swier1(value: u8) { ::write(REGISTER_ADDRESS, SWIER1_BIT_OFFSET, SWIER1_BIT_WIDTH, value as u32); }

	const SWIER2_BIT_OFFSET: u8 = 2;
	const SWIER2_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 2 (Width: 1, Offset: 2)
	pub fn get_swier2() -> u8 { ::read(REGISTER_ADDRESS, SWIER2_BIT_OFFSET, SWIER2_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 2 (Width: 1, Offset: 2)
	pub fn set_swier2(value: u8) { ::write(REGISTER_ADDRESS, SWIER2_BIT_OFFSET, SWIER2_BIT_WIDTH, value as u32); }

	const SWIER3_BIT_OFFSET: u8 = 3;
	const SWIER3_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 3 (Width: 1, Offset: 3)
	pub fn get_swier3() -> u8 { ::read(REGISTER_ADDRESS, SWIER3_BIT_OFFSET, SWIER3_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 3 (Width: 1, Offset: 3)
	pub fn set_swier3(value: u8) { ::write(REGISTER_ADDRESS, SWIER3_BIT_OFFSET, SWIER3_BIT_WIDTH, value as u32); }

	const SWIER4_BIT_OFFSET: u8 = 4;
	const SWIER4_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 4 (Width: 1, Offset: 4)
	pub fn get_swier4() -> u8 { ::read(REGISTER_ADDRESS, SWIER4_BIT_OFFSET, SWIER4_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 4 (Width: 1, Offset: 4)
	pub fn set_swier4(value: u8) { ::write(REGISTER_ADDRESS, SWIER4_BIT_OFFSET, SWIER4_BIT_WIDTH, value as u32); }

	const SWIER5_BIT_OFFSET: u8 = 5;
	const SWIER5_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 5 (Width: 1, Offset: 5)
	pub fn get_swier5() -> u8 { ::read(REGISTER_ADDRESS, SWIER5_BIT_OFFSET, SWIER5_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 5 (Width: 1, Offset: 5)
	pub fn set_swier5(value: u8) { ::write(REGISTER_ADDRESS, SWIER5_BIT_OFFSET, SWIER5_BIT_WIDTH, value as u32); }

	const SWIER6_BIT_OFFSET: u8 = 6;
	const SWIER6_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 6 (Width: 1, Offset: 6)
	pub fn get_swier6() -> u8 { ::read(REGISTER_ADDRESS, SWIER6_BIT_OFFSET, SWIER6_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 6 (Width: 1, Offset: 6)
	pub fn set_swier6(value: u8) { ::write(REGISTER_ADDRESS, SWIER6_BIT_OFFSET, SWIER6_BIT_WIDTH, value as u32); }

	const SWIER7_BIT_OFFSET: u8 = 7;
	const SWIER7_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 7 (Width: 1, Offset: 7)
	pub fn get_swier7() -> u8 { ::read(REGISTER_ADDRESS, SWIER7_BIT_OFFSET, SWIER7_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 7 (Width: 1, Offset: 7)
	pub fn set_swier7(value: u8) { ::write(REGISTER_ADDRESS, SWIER7_BIT_OFFSET, SWIER7_BIT_WIDTH, value as u32); }

	const SWIER8_BIT_OFFSET: u8 = 8;
	const SWIER8_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 8 (Width: 1, Offset: 8)
	pub fn get_swier8() -> u8 { ::read(REGISTER_ADDRESS, SWIER8_BIT_OFFSET, SWIER8_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 8 (Width: 1, Offset: 8)
	pub fn set_swier8(value: u8) { ::write(REGISTER_ADDRESS, SWIER8_BIT_OFFSET, SWIER8_BIT_WIDTH, value as u32); }

	const SWIER9_BIT_OFFSET: u8 = 9;
	const SWIER9_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 9 (Width: 1, Offset: 9)
	pub fn get_swier9() -> u8 { ::read(REGISTER_ADDRESS, SWIER9_BIT_OFFSET, SWIER9_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 9 (Width: 1, Offset: 9)
	pub fn set_swier9(value: u8) { ::write(REGISTER_ADDRESS, SWIER9_BIT_OFFSET, SWIER9_BIT_WIDTH, value as u32); }

	const SWIER10_BIT_OFFSET: u8 = 10;
	const SWIER10_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 10 (Width: 1, Offset: 10)
	pub fn get_swier10() -> u8 { ::read(REGISTER_ADDRESS, SWIER10_BIT_OFFSET, SWIER10_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 10 (Width: 1, Offset: 10)
	pub fn set_swier10(value: u8) { ::write(REGISTER_ADDRESS, SWIER10_BIT_OFFSET, SWIER10_BIT_WIDTH, value as u32); }

	const SWIER11_BIT_OFFSET: u8 = 11;
	const SWIER11_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 11 (Width: 1, Offset: 11)
	pub fn get_swier11() -> u8 { ::read(REGISTER_ADDRESS, SWIER11_BIT_OFFSET, SWIER11_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 11 (Width: 1, Offset: 11)
	pub fn set_swier11(value: u8) { ::write(REGISTER_ADDRESS, SWIER11_BIT_OFFSET, SWIER11_BIT_WIDTH, value as u32); }

	const SWIER12_BIT_OFFSET: u8 = 12;
	const SWIER12_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 12 (Width: 1, Offset: 12)
	pub fn get_swier12() -> u8 { ::read(REGISTER_ADDRESS, SWIER12_BIT_OFFSET, SWIER12_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 12 (Width: 1, Offset: 12)
	pub fn set_swier12(value: u8) { ::write(REGISTER_ADDRESS, SWIER12_BIT_OFFSET, SWIER12_BIT_WIDTH, value as u32); }

	const SWIER13_BIT_OFFSET: u8 = 13;
	const SWIER13_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 13 (Width: 1, Offset: 13)
	pub fn get_swier13() -> u8 { ::read(REGISTER_ADDRESS, SWIER13_BIT_OFFSET, SWIER13_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 13 (Width: 1, Offset: 13)
	pub fn set_swier13(value: u8) { ::write(REGISTER_ADDRESS, SWIER13_BIT_OFFSET, SWIER13_BIT_WIDTH, value as u32); }

	const SWIER14_BIT_OFFSET: u8 = 14;
	const SWIER14_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 14 (Width: 1, Offset: 14)
	pub fn get_swier14() -> u8 { ::read(REGISTER_ADDRESS, SWIER14_BIT_OFFSET, SWIER14_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 14 (Width: 1, Offset: 14)
	pub fn set_swier14(value: u8) { ::write(REGISTER_ADDRESS, SWIER14_BIT_OFFSET, SWIER14_BIT_WIDTH, value as u32); }

	const SWIER15_BIT_OFFSET: u8 = 15;
	const SWIER15_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 15 (Width: 1, Offset: 15)
	pub fn get_swier15() -> u8 { ::read(REGISTER_ADDRESS, SWIER15_BIT_OFFSET, SWIER15_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 15 (Width: 1, Offset: 15)
	pub fn set_swier15(value: u8) { ::write(REGISTER_ADDRESS, SWIER15_BIT_OFFSET, SWIER15_BIT_WIDTH, value as u32); }

	const SWIER16_BIT_OFFSET: u8 = 16;
	const SWIER16_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 16 (Width: 1, Offset: 16)
	pub fn get_swier16() -> u8 { ::read(REGISTER_ADDRESS, SWIER16_BIT_OFFSET, SWIER16_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 16 (Width: 1, Offset: 16)
	pub fn set_swier16(value: u8) { ::write(REGISTER_ADDRESS, SWIER16_BIT_OFFSET, SWIER16_BIT_WIDTH, value as u32); }

	const SWIER17_BIT_OFFSET: u8 = 17;
	const SWIER17_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 17 (Width: 1, Offset: 17)
	pub fn get_swier17() -> u8 { ::read(REGISTER_ADDRESS, SWIER17_BIT_OFFSET, SWIER17_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 17 (Width: 1, Offset: 17)
	pub fn set_swier17(value: u8) { ::write(REGISTER_ADDRESS, SWIER17_BIT_OFFSET, SWIER17_BIT_WIDTH, value as u32); }

	const SWIER18_BIT_OFFSET: u8 = 18;
	const SWIER18_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 18 (Width: 1, Offset: 18)
	pub fn get_swier18() -> u8 { ::read(REGISTER_ADDRESS, SWIER18_BIT_OFFSET, SWIER18_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 18 (Width: 1, Offset: 18)
	pub fn set_swier18(value: u8) { ::write(REGISTER_ADDRESS, SWIER18_BIT_OFFSET, SWIER18_BIT_WIDTH, value as u32); }

	const SWIER19_BIT_OFFSET: u8 = 19;
	const SWIER19_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 19 (Width: 1, Offset: 19)
	pub fn get_swier19() -> u8 { ::read(REGISTER_ADDRESS, SWIER19_BIT_OFFSET, SWIER19_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 19 (Width: 1, Offset: 19)
	pub fn set_swier19(value: u8) { ::write(REGISTER_ADDRESS, SWIER19_BIT_OFFSET, SWIER19_BIT_WIDTH, value as u32); }

	const SWIER20_BIT_OFFSET: u8 = 20;
	const SWIER20_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 20 (Width: 1, Offset: 20)
	pub fn get_swier20() -> u8 { ::read(REGISTER_ADDRESS, SWIER20_BIT_OFFSET, SWIER20_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 20 (Width: 1, Offset: 20)
	pub fn set_swier20(value: u8) { ::write(REGISTER_ADDRESS, SWIER20_BIT_OFFSET, SWIER20_BIT_WIDTH, value as u32); }

	const SWIER21_BIT_OFFSET: u8 = 21;
	const SWIER21_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 21 (Width: 1, Offset: 21)
	pub fn get_swier21() -> u8 { ::read(REGISTER_ADDRESS, SWIER21_BIT_OFFSET, SWIER21_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 21 (Width: 1, Offset: 21)
	pub fn set_swier21(value: u8) { ::write(REGISTER_ADDRESS, SWIER21_BIT_OFFSET, SWIER21_BIT_WIDTH, value as u32); }

	const SWIER22_BIT_OFFSET: u8 = 22;
	const SWIER22_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 22 (Width: 1, Offset: 22)
	pub fn get_swier22() -> u8 { ::read(REGISTER_ADDRESS, SWIER22_BIT_OFFSET, SWIER22_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 22 (Width: 1, Offset: 22)
	pub fn set_swier22(value: u8) { ::write(REGISTER_ADDRESS, SWIER22_BIT_OFFSET, SWIER22_BIT_WIDTH, value as u32); }

	const SWIER29_BIT_OFFSET: u8 = 29;
	const SWIER29_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 29 (Width: 1, Offset: 29)
	pub fn get_swier29() -> u8 { ::read(REGISTER_ADDRESS, SWIER29_BIT_OFFSET, SWIER29_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 29 (Width: 1, Offset: 29)
	pub fn set_swier29(value: u8) { ::write(REGISTER_ADDRESS, SWIER29_BIT_OFFSET, SWIER29_BIT_WIDTH, value as u32); }

	const SWIER30_BIT_OFFSET: u8 = 30;
	const SWIER30_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 309 (Width: 1, Offset: 30)
	pub fn get_swier30() -> u8 { ::read(REGISTER_ADDRESS, SWIER30_BIT_OFFSET, SWIER30_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 309 (Width: 1, Offset: 30)
	pub fn set_swier30(value: u8) { ::write(REGISTER_ADDRESS, SWIER30_BIT_OFFSET, SWIER30_BIT_WIDTH, value as u32); }

	const SWIER31_BIT_OFFSET: u8 = 31;
	const SWIER31_BIT_WIDTH: u8 = 1;
	/// Software Interrupt on line 319 (Width: 1, Offset: 31)
	pub fn get_swier31() -> u8 { ::read(REGISTER_ADDRESS, SWIER31_BIT_OFFSET, SWIER31_BIT_WIDTH) as u8 }
	/// Software Interrupt on line 319 (Width: 1, Offset: 31)
	pub fn set_swier31(value: u8) { ::write(REGISTER_ADDRESS, SWIER31_BIT_OFFSET, SWIER31_BIT_WIDTH, value as u32); }
}
/// Pending register
/// Size: 0x20 bits
pub mod pr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PR0_BIT_OFFSET: u8 = 0;
	const PR0_BIT_WIDTH: u8 = 1;
	/// Pending bit 0 (Width: 1, Offset: 0)
	pub fn get_pr0() -> u8 { ::read(REGISTER_ADDRESS, PR0_BIT_OFFSET, PR0_BIT_WIDTH) as u8 }
	/// Pending bit 0 (Width: 1, Offset: 0)
	pub fn set_pr0(value: u8) { ::write(REGISTER_ADDRESS, PR0_BIT_OFFSET, PR0_BIT_WIDTH, value as u32); }

	const PR1_BIT_OFFSET: u8 = 1;
	const PR1_BIT_WIDTH: u8 = 1;
	/// Pending bit 1 (Width: 1, Offset: 1)
	pub fn get_pr1() -> u8 { ::read(REGISTER_ADDRESS, PR1_BIT_OFFSET, PR1_BIT_WIDTH) as u8 }
	/// Pending bit 1 (Width: 1, Offset: 1)
	pub fn set_pr1(value: u8) { ::write(REGISTER_ADDRESS, PR1_BIT_OFFSET, PR1_BIT_WIDTH, value as u32); }

	const PR2_BIT_OFFSET: u8 = 2;
	const PR2_BIT_WIDTH: u8 = 1;
	/// Pending bit 2 (Width: 1, Offset: 2)
	pub fn get_pr2() -> u8 { ::read(REGISTER_ADDRESS, PR2_BIT_OFFSET, PR2_BIT_WIDTH) as u8 }
	/// Pending bit 2 (Width: 1, Offset: 2)
	pub fn set_pr2(value: u8) { ::write(REGISTER_ADDRESS, PR2_BIT_OFFSET, PR2_BIT_WIDTH, value as u32); }

	const PR3_BIT_OFFSET: u8 = 3;
	const PR3_BIT_WIDTH: u8 = 1;
	/// Pending bit 3 (Width: 1, Offset: 3)
	pub fn get_pr3() -> u8 { ::read(REGISTER_ADDRESS, PR3_BIT_OFFSET, PR3_BIT_WIDTH) as u8 }
	/// Pending bit 3 (Width: 1, Offset: 3)
	pub fn set_pr3(value: u8) { ::write(REGISTER_ADDRESS, PR3_BIT_OFFSET, PR3_BIT_WIDTH, value as u32); }

	const PR4_BIT_OFFSET: u8 = 4;
	const PR4_BIT_WIDTH: u8 = 1;
	/// Pending bit 4 (Width: 1, Offset: 4)
	pub fn get_pr4() -> u8 { ::read(REGISTER_ADDRESS, PR4_BIT_OFFSET, PR4_BIT_WIDTH) as u8 }
	/// Pending bit 4 (Width: 1, Offset: 4)
	pub fn set_pr4(value: u8) { ::write(REGISTER_ADDRESS, PR4_BIT_OFFSET, PR4_BIT_WIDTH, value as u32); }

	const PR5_BIT_OFFSET: u8 = 5;
	const PR5_BIT_WIDTH: u8 = 1;
	/// Pending bit 5 (Width: 1, Offset: 5)
	pub fn get_pr5() -> u8 { ::read(REGISTER_ADDRESS, PR5_BIT_OFFSET, PR5_BIT_WIDTH) as u8 }
	/// Pending bit 5 (Width: 1, Offset: 5)
	pub fn set_pr5(value: u8) { ::write(REGISTER_ADDRESS, PR5_BIT_OFFSET, PR5_BIT_WIDTH, value as u32); }

	const PR6_BIT_OFFSET: u8 = 6;
	const PR6_BIT_WIDTH: u8 = 1;
	/// Pending bit 6 (Width: 1, Offset: 6)
	pub fn get_pr6() -> u8 { ::read(REGISTER_ADDRESS, PR6_BIT_OFFSET, PR6_BIT_WIDTH) as u8 }
	/// Pending bit 6 (Width: 1, Offset: 6)
	pub fn set_pr6(value: u8) { ::write(REGISTER_ADDRESS, PR6_BIT_OFFSET, PR6_BIT_WIDTH, value as u32); }

	const PR7_BIT_OFFSET: u8 = 7;
	const PR7_BIT_WIDTH: u8 = 1;
	/// Pending bit 7 (Width: 1, Offset: 7)
	pub fn get_pr7() -> u8 { ::read(REGISTER_ADDRESS, PR7_BIT_OFFSET, PR7_BIT_WIDTH) as u8 }
	/// Pending bit 7 (Width: 1, Offset: 7)
	pub fn set_pr7(value: u8) { ::write(REGISTER_ADDRESS, PR7_BIT_OFFSET, PR7_BIT_WIDTH, value as u32); }

	const PR8_BIT_OFFSET: u8 = 8;
	const PR8_BIT_WIDTH: u8 = 1;
	/// Pending bit 8 (Width: 1, Offset: 8)
	pub fn get_pr8() -> u8 { ::read(REGISTER_ADDRESS, PR8_BIT_OFFSET, PR8_BIT_WIDTH) as u8 }
	/// Pending bit 8 (Width: 1, Offset: 8)
	pub fn set_pr8(value: u8) { ::write(REGISTER_ADDRESS, PR8_BIT_OFFSET, PR8_BIT_WIDTH, value as u32); }

	const PR9_BIT_OFFSET: u8 = 9;
	const PR9_BIT_WIDTH: u8 = 1;
	/// Pending bit 9 (Width: 1, Offset: 9)
	pub fn get_pr9() -> u8 { ::read(REGISTER_ADDRESS, PR9_BIT_OFFSET, PR9_BIT_WIDTH) as u8 }
	/// Pending bit 9 (Width: 1, Offset: 9)
	pub fn set_pr9(value: u8) { ::write(REGISTER_ADDRESS, PR9_BIT_OFFSET, PR9_BIT_WIDTH, value as u32); }

	const PR10_BIT_OFFSET: u8 = 10;
	const PR10_BIT_WIDTH: u8 = 1;
	/// Pending bit 10 (Width: 1, Offset: 10)
	pub fn get_pr10() -> u8 { ::read(REGISTER_ADDRESS, PR10_BIT_OFFSET, PR10_BIT_WIDTH) as u8 }
	/// Pending bit 10 (Width: 1, Offset: 10)
	pub fn set_pr10(value: u8) { ::write(REGISTER_ADDRESS, PR10_BIT_OFFSET, PR10_BIT_WIDTH, value as u32); }

	const PR11_BIT_OFFSET: u8 = 11;
	const PR11_BIT_WIDTH: u8 = 1;
	/// Pending bit 11 (Width: 1, Offset: 11)
	pub fn get_pr11() -> u8 { ::read(REGISTER_ADDRESS, PR11_BIT_OFFSET, PR11_BIT_WIDTH) as u8 }
	/// Pending bit 11 (Width: 1, Offset: 11)
	pub fn set_pr11(value: u8) { ::write(REGISTER_ADDRESS, PR11_BIT_OFFSET, PR11_BIT_WIDTH, value as u32); }

	const PR12_BIT_OFFSET: u8 = 12;
	const PR12_BIT_WIDTH: u8 = 1;
	/// Pending bit 12 (Width: 1, Offset: 12)
	pub fn get_pr12() -> u8 { ::read(REGISTER_ADDRESS, PR12_BIT_OFFSET, PR12_BIT_WIDTH) as u8 }
	/// Pending bit 12 (Width: 1, Offset: 12)
	pub fn set_pr12(value: u8) { ::write(REGISTER_ADDRESS, PR12_BIT_OFFSET, PR12_BIT_WIDTH, value as u32); }

	const PR13_BIT_OFFSET: u8 = 13;
	const PR13_BIT_WIDTH: u8 = 1;
	/// Pending bit 13 (Width: 1, Offset: 13)
	pub fn get_pr13() -> u8 { ::read(REGISTER_ADDRESS, PR13_BIT_OFFSET, PR13_BIT_WIDTH) as u8 }
	/// Pending bit 13 (Width: 1, Offset: 13)
	pub fn set_pr13(value: u8) { ::write(REGISTER_ADDRESS, PR13_BIT_OFFSET, PR13_BIT_WIDTH, value as u32); }

	const PR14_BIT_OFFSET: u8 = 14;
	const PR14_BIT_WIDTH: u8 = 1;
	/// Pending bit 14 (Width: 1, Offset: 14)
	pub fn get_pr14() -> u8 { ::read(REGISTER_ADDRESS, PR14_BIT_OFFSET, PR14_BIT_WIDTH) as u8 }
	/// Pending bit 14 (Width: 1, Offset: 14)
	pub fn set_pr14(value: u8) { ::write(REGISTER_ADDRESS, PR14_BIT_OFFSET, PR14_BIT_WIDTH, value as u32); }

	const PR15_BIT_OFFSET: u8 = 15;
	const PR15_BIT_WIDTH: u8 = 1;
	/// Pending bit 15 (Width: 1, Offset: 15)
	pub fn get_pr15() -> u8 { ::read(REGISTER_ADDRESS, PR15_BIT_OFFSET, PR15_BIT_WIDTH) as u8 }
	/// Pending bit 15 (Width: 1, Offset: 15)
	pub fn set_pr15(value: u8) { ::write(REGISTER_ADDRESS, PR15_BIT_OFFSET, PR15_BIT_WIDTH, value as u32); }

	const PR16_BIT_OFFSET: u8 = 16;
	const PR16_BIT_WIDTH: u8 = 1;
	/// Pending bit 16 (Width: 1, Offset: 16)
	pub fn get_pr16() -> u8 { ::read(REGISTER_ADDRESS, PR16_BIT_OFFSET, PR16_BIT_WIDTH) as u8 }
	/// Pending bit 16 (Width: 1, Offset: 16)
	pub fn set_pr16(value: u8) { ::write(REGISTER_ADDRESS, PR16_BIT_OFFSET, PR16_BIT_WIDTH, value as u32); }

	const PR17_BIT_OFFSET: u8 = 17;
	const PR17_BIT_WIDTH: u8 = 1;
	/// Pending bit 17 (Width: 1, Offset: 17)
	pub fn get_pr17() -> u8 { ::read(REGISTER_ADDRESS, PR17_BIT_OFFSET, PR17_BIT_WIDTH) as u8 }
	/// Pending bit 17 (Width: 1, Offset: 17)
	pub fn set_pr17(value: u8) { ::write(REGISTER_ADDRESS, PR17_BIT_OFFSET, PR17_BIT_WIDTH, value as u32); }

	const PR18_BIT_OFFSET: u8 = 18;
	const PR18_BIT_WIDTH: u8 = 1;
	/// Pending bit 18 (Width: 1, Offset: 18)
	pub fn get_pr18() -> u8 { ::read(REGISTER_ADDRESS, PR18_BIT_OFFSET, PR18_BIT_WIDTH) as u8 }
	/// Pending bit 18 (Width: 1, Offset: 18)
	pub fn set_pr18(value: u8) { ::write(REGISTER_ADDRESS, PR18_BIT_OFFSET, PR18_BIT_WIDTH, value as u32); }

	const PR19_BIT_OFFSET: u8 = 19;
	const PR19_BIT_WIDTH: u8 = 1;
	/// Pending bit 19 (Width: 1, Offset: 19)
	pub fn get_pr19() -> u8 { ::read(REGISTER_ADDRESS, PR19_BIT_OFFSET, PR19_BIT_WIDTH) as u8 }
	/// Pending bit 19 (Width: 1, Offset: 19)
	pub fn set_pr19(value: u8) { ::write(REGISTER_ADDRESS, PR19_BIT_OFFSET, PR19_BIT_WIDTH, value as u32); }

	const PR20_BIT_OFFSET: u8 = 20;
	const PR20_BIT_WIDTH: u8 = 1;
	/// Pending bit 20 (Width: 1, Offset: 20)
	pub fn get_pr20() -> u8 { ::read(REGISTER_ADDRESS, PR20_BIT_OFFSET, PR20_BIT_WIDTH) as u8 }
	/// Pending bit 20 (Width: 1, Offset: 20)
	pub fn set_pr20(value: u8) { ::write(REGISTER_ADDRESS, PR20_BIT_OFFSET, PR20_BIT_WIDTH, value as u32); }

	const PR21_BIT_OFFSET: u8 = 21;
	const PR21_BIT_WIDTH: u8 = 1;
	/// Pending bit 21 (Width: 1, Offset: 21)
	pub fn get_pr21() -> u8 { ::read(REGISTER_ADDRESS, PR21_BIT_OFFSET, PR21_BIT_WIDTH) as u8 }
	/// Pending bit 21 (Width: 1, Offset: 21)
	pub fn set_pr21(value: u8) { ::write(REGISTER_ADDRESS, PR21_BIT_OFFSET, PR21_BIT_WIDTH, value as u32); }

	const PR22_BIT_OFFSET: u8 = 22;
	const PR22_BIT_WIDTH: u8 = 1;
	/// Pending bit 22 (Width: 1, Offset: 22)
	pub fn get_pr22() -> u8 { ::read(REGISTER_ADDRESS, PR22_BIT_OFFSET, PR22_BIT_WIDTH) as u8 }
	/// Pending bit 22 (Width: 1, Offset: 22)
	pub fn set_pr22(value: u8) { ::write(REGISTER_ADDRESS, PR22_BIT_OFFSET, PR22_BIT_WIDTH, value as u32); }

	const PR29_BIT_OFFSET: u8 = 29;
	const PR29_BIT_WIDTH: u8 = 1;
	/// Pending bit 29 (Width: 1, Offset: 29)
	pub fn get_pr29() -> u8 { ::read(REGISTER_ADDRESS, PR29_BIT_OFFSET, PR29_BIT_WIDTH) as u8 }
	/// Pending bit 29 (Width: 1, Offset: 29)
	pub fn set_pr29(value: u8) { ::write(REGISTER_ADDRESS, PR29_BIT_OFFSET, PR29_BIT_WIDTH, value as u32); }

	const PR30_BIT_OFFSET: u8 = 30;
	const PR30_BIT_WIDTH: u8 = 1;
	/// Pending bit 30 (Width: 1, Offset: 30)
	pub fn get_pr30() -> u8 { ::read(REGISTER_ADDRESS, PR30_BIT_OFFSET, PR30_BIT_WIDTH) as u8 }
	/// Pending bit 30 (Width: 1, Offset: 30)
	pub fn set_pr30(value: u8) { ::write(REGISTER_ADDRESS, PR30_BIT_OFFSET, PR30_BIT_WIDTH, value as u32); }

	const PR31_BIT_OFFSET: u8 = 31;
	const PR31_BIT_WIDTH: u8 = 1;
	/// Pending bit 31 (Width: 1, Offset: 31)
	pub fn get_pr31() -> u8 { ::read(REGISTER_ADDRESS, PR31_BIT_OFFSET, PR31_BIT_WIDTH) as u8 }
	/// Pending bit 31 (Width: 1, Offset: 31)
	pub fn set_pr31(value: u8) { ::write(REGISTER_ADDRESS, PR31_BIT_OFFSET, PR31_BIT_WIDTH, value as u32); }
}
/// Interrupt mask register
/// Size: 0x20 bits
pub mod imr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MR32_BIT_OFFSET: u8 = 0;
	const MR32_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on external/internal line 32 (Width: 1, Offset: 0)
	pub fn get_mr32() -> u8 { ::read(REGISTER_ADDRESS, MR32_BIT_OFFSET, MR32_BIT_WIDTH) as u8 }
	/// Interrupt Mask on external/internal line 32 (Width: 1, Offset: 0)
	pub fn set_mr32(value: u8) { ::write(REGISTER_ADDRESS, MR32_BIT_OFFSET, MR32_BIT_WIDTH, value as u32); }

	const MR33_BIT_OFFSET: u8 = 1;
	const MR33_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on external/internal line 33 (Width: 1, Offset: 1)
	pub fn get_mr33() -> u8 { ::read(REGISTER_ADDRESS, MR33_BIT_OFFSET, MR33_BIT_WIDTH) as u8 }
	/// Interrupt Mask on external/internal line 33 (Width: 1, Offset: 1)
	pub fn set_mr33(value: u8) { ::write(REGISTER_ADDRESS, MR33_BIT_OFFSET, MR33_BIT_WIDTH, value as u32); }

	const MR34_BIT_OFFSET: u8 = 2;
	const MR34_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on external/internal line 34 (Width: 1, Offset: 2)
	pub fn get_mr34() -> u8 { ::read(REGISTER_ADDRESS, MR34_BIT_OFFSET, MR34_BIT_WIDTH) as u8 }
	/// Interrupt Mask on external/internal line 34 (Width: 1, Offset: 2)
	pub fn set_mr34(value: u8) { ::write(REGISTER_ADDRESS, MR34_BIT_OFFSET, MR34_BIT_WIDTH, value as u32); }

	const MR35_BIT_OFFSET: u8 = 3;
	const MR35_BIT_WIDTH: u8 = 1;
	/// Interrupt Mask on external/internal line 35 (Width: 1, Offset: 3)
	pub fn get_mr35() -> u8 { ::read(REGISTER_ADDRESS, MR35_BIT_OFFSET, MR35_BIT_WIDTH) as u8 }
	/// Interrupt Mask on external/internal line 35 (Width: 1, Offset: 3)
	pub fn set_mr35(value: u8) { ::write(REGISTER_ADDRESS, MR35_BIT_OFFSET, MR35_BIT_WIDTH, value as u32); }
}
/// Event mask register
/// Size: 0x20 bits
pub mod emr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MR32_BIT_OFFSET: u8 = 0;
	const MR32_BIT_WIDTH: u8 = 1;
	/// Event mask on external/internal line 32 (Width: 1, Offset: 0)
	pub fn get_mr32() -> u8 { ::read(REGISTER_ADDRESS, MR32_BIT_OFFSET, MR32_BIT_WIDTH) as u8 }
	/// Event mask on external/internal line 32 (Width: 1, Offset: 0)
	pub fn set_mr32(value: u8) { ::write(REGISTER_ADDRESS, MR32_BIT_OFFSET, MR32_BIT_WIDTH, value as u32); }

	const MR33_BIT_OFFSET: u8 = 1;
	const MR33_BIT_WIDTH: u8 = 1;
	/// Event mask on external/internal line 33 (Width: 1, Offset: 1)
	pub fn get_mr33() -> u8 { ::read(REGISTER_ADDRESS, MR33_BIT_OFFSET, MR33_BIT_WIDTH) as u8 }
	/// Event mask on external/internal line 33 (Width: 1, Offset: 1)
	pub fn set_mr33(value: u8) { ::write(REGISTER_ADDRESS, MR33_BIT_OFFSET, MR33_BIT_WIDTH, value as u32); }

	const MR34_BIT_OFFSET: u8 = 2;
	const MR34_BIT_WIDTH: u8 = 1;
	/// Event mask on external/internal line 34 (Width: 1, Offset: 2)
	pub fn get_mr34() -> u8 { ::read(REGISTER_ADDRESS, MR34_BIT_OFFSET, MR34_BIT_WIDTH) as u8 }
	/// Event mask on external/internal line 34 (Width: 1, Offset: 2)
	pub fn set_mr34(value: u8) { ::write(REGISTER_ADDRESS, MR34_BIT_OFFSET, MR34_BIT_WIDTH, value as u32); }

	const MR35_BIT_OFFSET: u8 = 3;
	const MR35_BIT_WIDTH: u8 = 1;
	/// Event mask on external/internal line 35 (Width: 1, Offset: 3)
	pub fn get_mr35() -> u8 { ::read(REGISTER_ADDRESS, MR35_BIT_OFFSET, MR35_BIT_WIDTH) as u8 }
	/// Event mask on external/internal line 35 (Width: 1, Offset: 3)
	pub fn set_mr35(value: u8) { ::write(REGISTER_ADDRESS, MR35_BIT_OFFSET, MR35_BIT_WIDTH, value as u32); }
}
/// Rising Trigger selection register
/// Size: 0x20 bits
pub mod rtsr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x20;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TR32_BIT_OFFSET: u8 = 0;
	const TR32_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration bit of line 32 (Width: 1, Offset: 0)
	pub fn get_tr32() -> u8 { ::read(REGISTER_ADDRESS, TR32_BIT_OFFSET, TR32_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration bit of line 32 (Width: 1, Offset: 0)
	pub fn set_tr32(value: u8) { ::write(REGISTER_ADDRESS, TR32_BIT_OFFSET, TR32_BIT_WIDTH, value as u32); }

	const TR33_BIT_OFFSET: u8 = 1;
	const TR33_BIT_WIDTH: u8 = 1;
	/// Rising trigger event configuration bit of line 33 (Width: 1, Offset: 1)
	pub fn get_tr33() -> u8 { ::read(REGISTER_ADDRESS, TR33_BIT_OFFSET, TR33_BIT_WIDTH) as u8 }
	/// Rising trigger event configuration bit of line 33 (Width: 1, Offset: 1)
	pub fn set_tr33(value: u8) { ::write(REGISTER_ADDRESS, TR33_BIT_OFFSET, TR33_BIT_WIDTH, value as u32); }
}
/// Falling Trigger selection register
/// Size: 0x20 bits
pub mod ftsr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x24;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TR32_BIT_OFFSET: u8 = 0;
	const TR32_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration bit of line 32 (Width: 1, Offset: 0)
	pub fn get_tr32() -> u8 { ::read(REGISTER_ADDRESS, TR32_BIT_OFFSET, TR32_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration bit of line 32 (Width: 1, Offset: 0)
	pub fn set_tr32(value: u8) { ::write(REGISTER_ADDRESS, TR32_BIT_OFFSET, TR32_BIT_WIDTH, value as u32); }

	const TR33_BIT_OFFSET: u8 = 1;
	const TR33_BIT_WIDTH: u8 = 1;
	/// Falling trigger event configuration bit of line 33 (Width: 1, Offset: 1)
	pub fn get_tr33() -> u8 { ::read(REGISTER_ADDRESS, TR33_BIT_OFFSET, TR33_BIT_WIDTH) as u8 }
	/// Falling trigger event configuration bit of line 33 (Width: 1, Offset: 1)
	pub fn set_tr33(value: u8) { ::write(REGISTER_ADDRESS, TR33_BIT_OFFSET, TR33_BIT_WIDTH, value as u32); }
}
/// Software interrupt event register
/// Size: 0x20 bits
pub mod swier2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x28;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SWIER32_BIT_OFFSET: u8 = 0;
	const SWIER32_BIT_WIDTH: u8 = 1;
	/// Software interrupt on line 32 (Width: 1, Offset: 0)
	pub fn get_swier32() -> u8 { ::read(REGISTER_ADDRESS, SWIER32_BIT_OFFSET, SWIER32_BIT_WIDTH) as u8 }
	/// Software interrupt on line 32 (Width: 1, Offset: 0)
	pub fn set_swier32(value: u8) { ::write(REGISTER_ADDRESS, SWIER32_BIT_OFFSET, SWIER32_BIT_WIDTH, value as u32); }

	const SWIER33_BIT_OFFSET: u8 = 1;
	const SWIER33_BIT_WIDTH: u8 = 1;
	/// Software interrupt on line 33 (Width: 1, Offset: 1)
	pub fn get_swier33() -> u8 { ::read(REGISTER_ADDRESS, SWIER33_BIT_OFFSET, SWIER33_BIT_WIDTH) as u8 }
	/// Software interrupt on line 33 (Width: 1, Offset: 1)
	pub fn set_swier33(value: u8) { ::write(REGISTER_ADDRESS, SWIER33_BIT_OFFSET, SWIER33_BIT_WIDTH, value as u32); }
}
/// Pending register
/// Size: 0x20 bits
pub mod pr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x2C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PR32_BIT_OFFSET: u8 = 0;
	const PR32_BIT_WIDTH: u8 = 1;
	/// Pending bit on line 32 (Width: 1, Offset: 0)
	pub fn get_pr32() -> u8 { ::read(REGISTER_ADDRESS, PR32_BIT_OFFSET, PR32_BIT_WIDTH) as u8 }
	/// Pending bit on line 32 (Width: 1, Offset: 0)
	pub fn set_pr32(value: u8) { ::write(REGISTER_ADDRESS, PR32_BIT_OFFSET, PR32_BIT_WIDTH, value as u32); }

	const PR33_BIT_OFFSET: u8 = 1;
	const PR33_BIT_WIDTH: u8 = 1;
	/// Pending bit on line 33 (Width: 1, Offset: 1)
	pub fn get_pr33() -> u8 { ::read(REGISTER_ADDRESS, PR33_BIT_OFFSET, PR33_BIT_WIDTH) as u8 }
	/// Pending bit on line 33 (Width: 1, Offset: 1)
	pub fn set_pr33(value: u8) { ::write(REGISTER_ADDRESS, PR33_BIT_OFFSET, PR33_BIT_WIDTH, value as u32); }
}
/// Tamper and TimeStamp interrupts
pub const INTERRUPT_TAMP_STAMP: u32 = 2;

/// EXTI Line0 interrupt
pub const INTERRUPT_EXTI0: u32 = 6;

/// EXTI Line3 interrupt
pub const INTERRUPT_EXTI1: u32 = 7;

/// EXTI Line2 and Touch sensing interrupts
pub const INTERRUPT_EXTI2_TSC: u32 = 8;

/// EXTI Line3 interrupt
pub const INTERRUPT_EXTI3: u32 = 9;

/// EXTI Line4 interrupt
pub const INTERRUPT_EXTI4: u32 = 10;

/// EXTI Line5 to Line9 interrupts
pub const INTERRUPT_EXTI9_5: u32 = 23;

/// I2C1 event interrupt and EXTI Line23 interrupt
pub const INTERRUPT_I2C1_EV_EXTI23: u32 = 31;

/// USART1 global interrupt and EXTI Line 25 interrupt
pub const INTERRUPT_USART1_EXTI25: u32 = 37;

/// USART2 global interrupt and EXTI Line 26 interrupt
pub const INTERRUPT_USART2_EXTI26: u32 = 38;

/// USART3 global interrupt and EXTI Line 28 interrupt
pub const INTERRUPT_USART3_EXTI28: u32 = 39;

/// EXTI Line15 to Line10 interrupts
pub const INTERRUPT_EXTI15_10: u32 = 40;

/// UART4 global and EXTI Line 34 interrupts
pub const INTERRUPT_UART4_EXTI34: u32 = 52;

/// UART5 global and EXTI Line 35 interrupts
pub const INTERRUPT_UART5_EXTI35: u32 = 53;

/// USB wakeup from Suspend and EXTI Line 18
pub const INTERRUPT_USB_WKUP_EXTI: u32 = 76;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>EXTI</name>
  <description>External interrupt/event
      controller</description>
  <groupName>EXTI</groupName>
  <baseAddress>0x40010400</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>IMR1</name>
      <displayName>IMR1</displayName>
      <description>Interrupt mask register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x1F800000</resetValue>
      <fields>
        <field>
          <name>MR0</name>
          <description>Interrupt Mask on line 0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR1</name>
          <description>Interrupt Mask on line 1</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR2</name>
          <description>Interrupt Mask on line 2</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR3</name>
          <description>Interrupt Mask on line 3</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR4</name>
          <description>Interrupt Mask on line 4</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR5</name>
          <description>Interrupt Mask on line 5</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR6</name>
          <description>Interrupt Mask on line 6</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR7</name>
          <description>Interrupt Mask on line 7</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR8</name>
          <description>Interrupt Mask on line 8</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR9</name>
          <description>Interrupt Mask on line 9</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR10</name>
          <description>Interrupt Mask on line 10</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR11</name>
          <description>Interrupt Mask on line 11</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR12</name>
          <description>Interrupt Mask on line 12</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR13</name>
          <description>Interrupt Mask on line 13</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR14</name>
          <description>Interrupt Mask on line 14</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR15</name>
          <description>Interrupt Mask on line 15</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR16</name>
          <description>Interrupt Mask on line 16</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR17</name>
          <description>Interrupt Mask on line 17</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR18</name>
          <description>Interrupt Mask on line 18</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR19</name>
          <description>Interrupt Mask on line 19</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR20</name>
          <description>Interrupt Mask on line 20</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR21</name>
          <description>Interrupt Mask on line 21</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR22</name>
          <description>Interrupt Mask on line 22</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR23</name>
          <description>Interrupt Mask on line 23</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR24</name>
          <description>Interrupt Mask on line 24</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR25</name>
          <description>Interrupt Mask on line 25</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR26</name>
          <description>Interrupt Mask on line 26</description>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR27</name>
          <description>Interrupt Mask on line 27</description>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR28</name>
          <description>Interrupt Mask on line 28</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR29</name>
          <description>Interrupt Mask on line 29</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR30</name>
          <description>Interrupt Mask on line 30</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR31</name>
          <description>Interrupt Mask on line 31</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>EMR1</name>
      <displayName>EMR1</displayName>
      <description>Event mask register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MR0</name>
          <description>Event Mask on line 0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR1</name>
          <description>Event Mask on line 1</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR2</name>
          <description>Event Mask on line 2</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR3</name>
          <description>Event Mask on line 3</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR4</name>
          <description>Event Mask on line 4</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR5</name>
          <description>Event Mask on line 5</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR6</name>
          <description>Event Mask on line 6</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR7</name>
          <description>Event Mask on line 7</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR8</name>
          <description>Event Mask on line 8</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR9</name>
          <description>Event Mask on line 9</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR10</name>
          <description>Event Mask on line 10</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR11</name>
          <description>Event Mask on line 11</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR12</name>
          <description>Event Mask on line 12</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR13</name>
          <description>Event Mask on line 13</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR14</name>
          <description>Event Mask on line 14</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR15</name>
          <description>Event Mask on line 15</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR16</name>
          <description>Event Mask on line 16</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR17</name>
          <description>Event Mask on line 17</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR18</name>
          <description>Event Mask on line 18</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR19</name>
          <description>Event Mask on line 19</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR20</name>
          <description>Event Mask on line 20</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR21</name>
          <description>Event Mask on line 21</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR22</name>
          <description>Event Mask on line 22</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR23</name>
          <description>Event Mask on line 23</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR24</name>
          <description>Event Mask on line 24</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR25</name>
          <description>Event Mask on line 25</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR26</name>
          <description>Event Mask on line 26</description>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR27</name>
          <description>Event Mask on line 27</description>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR28</name>
          <description>Event Mask on line 28</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR29</name>
          <description>Event Mask on line 29</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR30</name>
          <description>Event Mask on line 30</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR31</name>
          <description>Event Mask on line 31</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>RTSR1</name>
      <displayName>RTSR1</displayName>
      <description>Rising Trigger selection
          register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>TR0</name>
          <description>Rising trigger event configuration of
              line 0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR1</name>
          <description>Rising trigger event configuration of
              line 1</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR2</name>
          <description>Rising trigger event configuration of
              line 2</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR3</name>
          <description>Rising trigger event configuration of
              line 3</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR4</name>
          <description>Rising trigger event configuration of
              line 4</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR5</name>
          <description>Rising trigger event configuration of
              line 5</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR6</name>
          <description>Rising trigger event configuration of
              line 6</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR7</name>
          <description>Rising trigger event configuration of
              line 7</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR8</name>
          <description>Rising trigger event configuration of
              line 8</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR9</name>
          <description>Rising trigger event configuration of
              line 9</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR10</name>
          <description>Rising trigger event configuration of
              line 10</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR11</name>
          <description>Rising trigger event configuration of
              line 11</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR12</name>
          <description>Rising trigger event configuration of
              line 12</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR13</name>
          <description>Rising trigger event configuration of
              line 13</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR14</name>
          <description>Rising trigger event configuration of
              line 14</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR15</name>
          <description>Rising trigger event configuration of
              line 15</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR16</name>
          <description>Rising trigger event configuration of
              line 16</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR17</name>
          <description>Rising trigger event configuration of
              line 17</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR18</name>
          <description>Rising trigger event configuration of
              line 18</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR19</name>
          <description>Rising trigger event configuration of
              line 19</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR20</name>
          <description>Rising trigger event configuration of
              line 20</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR21</name>
          <description>Rising trigger event configuration of
              line 21</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR22</name>
          <description>Rising trigger event configuration of
              line 22</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR29</name>
          <description>Rising trigger event configuration of
              line 29</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR30</name>
          <description>Rising trigger event configuration of
              line 30</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR31</name>
          <description>Rising trigger event configuration of
              line 31</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>FTSR1</name>
      <displayName>FTSR1</displayName>
      <description>Falling Trigger selection
          register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>TR0</name>
          <description>Falling trigger event configuration of
              line 0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR1</name>
          <description>Falling trigger event configuration of
              line 1</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR2</name>
          <description>Falling trigger event configuration of
              line 2</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR3</name>
          <description>Falling trigger event configuration of
              line 3</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR4</name>
          <description>Falling trigger event configuration of
              line 4</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR5</name>
          <description>Falling trigger event configuration of
              line 5</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR6</name>
          <description>Falling trigger event configuration of
              line 6</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR7</name>
          <description>Falling trigger event configuration of
              line 7</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR8</name>
          <description>Falling trigger event configuration of
              line 8</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR9</name>
          <description>Falling trigger event configuration of
              line 9</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR10</name>
          <description>Falling trigger event configuration of
              line 10</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR11</name>
          <description>Falling trigger event configuration of
              line 11</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR12</name>
          <description>Falling trigger event configuration of
              line 12</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR13</name>
          <description>Falling trigger event configuration of
              line 13</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR14</name>
          <description>Falling trigger event configuration of
              line 14</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR15</name>
          <description>Falling trigger event configuration of
              line 15</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR16</name>
          <description>Falling trigger event configuration of
              line 16</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR17</name>
          <description>Falling trigger event configuration of
              line 17</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR18</name>
          <description>Falling trigger event configuration of
              line 18</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR19</name>
          <description>Falling trigger event configuration of
              line 19</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR20</name>
          <description>Falling trigger event configuration of
              line 20</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR21</name>
          <description>Falling trigger event configuration of
              line 21</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR22</name>
          <description>Falling trigger event configuration of
              line 22</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR29</name>
          <description>Falling trigger event configuration of
              line 29</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR30</name>
          <description>Falling trigger event configuration of
              line 30.</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR31</name>
          <description>Falling trigger event configuration of
              line 31</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SWIER1</name>
      <displayName>SWIER1</displayName>
      <description>Software interrupt event
          register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SWIER0</name>
          <description>Software Interrupt on line
              0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER1</name>
          <description>Software Interrupt on line
              1</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER2</name>
          <description>Software Interrupt on line
              2</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER3</name>
          <description>Software Interrupt on line
              3</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER4</name>
          <description>Software Interrupt on line
              4</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER5</name>
          <description>Software Interrupt on line
              5</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER6</name>
          <description>Software Interrupt on line
              6</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER7</name>
          <description>Software Interrupt on line
              7</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER8</name>
          <description>Software Interrupt on line
              8</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER9</name>
          <description>Software Interrupt on line
              9</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER10</name>
          <description>Software Interrupt on line
              10</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER11</name>
          <description>Software Interrupt on line
              11</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER12</name>
          <description>Software Interrupt on line
              12</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER13</name>
          <description>Software Interrupt on line
              13</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER14</name>
          <description>Software Interrupt on line
              14</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER15</name>
          <description>Software Interrupt on line
              15</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER16</name>
          <description>Software Interrupt on line
              16</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER17</name>
          <description>Software Interrupt on line
              17</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER18</name>
          <description>Software Interrupt on line
              18</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER19</name>
          <description>Software Interrupt on line
              19</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER20</name>
          <description>Software Interrupt on line
              20</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER21</name>
          <description>Software Interrupt on line
              21</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER22</name>
          <description>Software Interrupt on line
              22</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER29</name>
          <description>Software Interrupt on line
              29</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER30</name>
          <description>Software Interrupt on line
              309</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER31</name>
          <description>Software Interrupt on line
              319</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>PR1</name>
      <displayName>PR1</displayName>
      <description>Pending register</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PR0</name>
          <description>Pending bit 0</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR1</name>
          <description>Pending bit 1</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR2</name>
          <description>Pending bit 2</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR3</name>
          <description>Pending bit 3</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR4</name>
          <description>Pending bit 4</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR5</name>
          <description>Pending bit 5</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR6</name>
          <description>Pending bit 6</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR7</name>
          <description>Pending bit 7</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR8</name>
          <description>Pending bit 8</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR9</name>
          <description>Pending bit 9</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR10</name>
          <description>Pending bit 10</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR11</name>
          <description>Pending bit 11</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR12</name>
          <description>Pending bit 12</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR13</name>
          <description>Pending bit 13</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR14</name>
          <description>Pending bit 14</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR15</name>
          <description>Pending bit 15</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR16</name>
          <description>Pending bit 16</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR17</name>
          <description>Pending bit 17</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR18</name>
          <description>Pending bit 18</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR19</name>
          <description>Pending bit 19</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR20</name>
          <description>Pending bit 20</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR21</name>
          <description>Pending bit 21</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR22</name>
          <description>Pending bit 22</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR29</name>
          <description>Pending bit 29</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR30</name>
          <description>Pending bit 30</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR31</name>
          <description>Pending bit 31</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IMR2</name>
      <displayName>IMR2</displayName>
      <description>Interrupt mask register</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0xFFFFFFFC</resetValue>
      <fields>
        <field>
          <name>MR32</name>
          <description>Interrupt Mask on external/internal line
              32</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR33</name>
          <description>Interrupt Mask on external/internal line
              33</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR34</name>
          <description>Interrupt Mask on external/internal line
              34</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR35</name>
          <description>Interrupt Mask on external/internal line
              35</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>EMR2</name>
      <displayName>EMR2</displayName>
      <description>Event mask register</description>
      <addressOffset>0x1C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MR32</name>
          <description>Event mask on external/internal line
              32</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR33</name>
          <description>Event mask on external/internal line
              33</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR34</name>
          <description>Event mask on external/internal line
              34</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MR35</name>
          <description>Event mask on external/internal line
              35</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>RTSR2</name>
      <displayName>RTSR2</displayName>
      <description>Rising Trigger selection
          register</description>
      <addressOffset>0x20</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>TR32</name>
          <description>Rising trigger event configuration bit
              of line 32</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR33</name>
          <description>Rising trigger event configuration bit
              of line 33</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>FTSR2</name>
      <displayName>FTSR2</displayName>
      <description>Falling Trigger selection
          register</description>
      <addressOffset>0x24</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>TR32</name>
          <description>Falling trigger event configuration bit
              of line 32</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TR33</name>
          <description>Falling trigger event configuration bit
              of line 33</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SWIER2</name>
      <displayName>SWIER2</displayName>
      <description>Software interrupt event
          register</description>
      <addressOffset>0x28</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SWIER32</name>
          <description>Software interrupt on line
              32</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWIER33</name>
          <description>Software interrupt on line
              33</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>PR2</name>
      <displayName>PR2</displayName>
      <description>Pending register</description>
      <addressOffset>0x2C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PR32</name>
          <description>Pending bit on line 32</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PR33</name>
          <description>Pending bit on line 33</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>TAMP_STAMP</name>
    <description>Tamper and TimeStamp interrupts</description>
    <value>2</value>
  </interrupt>
  <interrupt>
    <name>EXTI0</name>
    <description>EXTI Line0 interrupt</description>
    <value>6</value>
  </interrupt>
  <interrupt>
    <name>EXTI1</name>
    <description>EXTI Line3 interrupt</description>
    <value>7</value>
  </interrupt>
  <interrupt>
    <name>EXTI2_TSC</name>
    <description>EXTI Line2 and Touch sensing
        interrupts</description>
    <value>8</value>
  </interrupt>
  <interrupt>
    <name>EXTI3</name>
    <description>EXTI Line3 interrupt</description>
    <value>9</value>
  </interrupt>
  <interrupt>
    <name>EXTI4</name>
    <description>EXTI Line4 interrupt</description>
    <value>10</value>
  </interrupt>
  <interrupt>
    <name>EXTI9_5</name>
    <description>EXTI Line5 to Line9 interrupts</description>
    <value>23</value>
  </interrupt>
  <interrupt>
    <name>I2C1_EV_EXTI23</name>
    <description>I2C1 event interrupt and EXTI Line23
        interrupt</description>
    <value>31</value>
  </interrupt>
  <interrupt>
    <name>USART1_EXTI25</name>
    <description>USART1 global interrupt and EXTI Line 25
        interrupt</description>
    <value>37</value>
  </interrupt>
  <interrupt>
    <name>USART2_EXTI26</name>
    <description>USART2 global interrupt and EXTI Line 26
        interrupt</description>
    <value>38</value>
  </interrupt>
  <interrupt>
    <name>USART3_EXTI28</name>
    <description>USART3 global interrupt and EXTI Line 28
        interrupt</description>
    <value>39</value>
  </interrupt>
  <interrupt>
    <name>EXTI15_10</name>
    <description>EXTI Line15 to Line10 interrupts</description>
    <value>40</value>
  </interrupt>
  <interrupt>
    <name>UART4_EXTI34</name>
    <description>UART4 global and EXTI Line 34
        interrupts</description>
    <value>52</value>
  </interrupt>
  <interrupt>
    <name>UART5_EXTI35</name>
    <description>UART5 global and EXTI Line 35
        interrupts</description>
    <value>53</value>
  </interrupt>
  <interrupt>
    <name>USB_WKUP_EXTI</name>
    <description>USB wakeup from Suspend and EXTI Line
        18</description>
    <value>76</value>
  </interrupt>
</peripheral>*/
