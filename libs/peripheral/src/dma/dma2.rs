/// MOD DMA2
/// DMA controller 1
const BASE_ADDRESS: u32 = 0x40020400;
/// DMA interrupt status register (DMA_ISR)
/// Size: 0x20 bits
pub mod isr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const GIF1_BIT_OFFSET: u8 = 0;
	const GIF1_BIT_WIDTH: u8 = 1;
	/// Channel 1 Global interrupt flag (Width: 1, Offset: 0)
	pub fn get_gif1() -> u8 { ::read(REGISTER_ADDRESS, GIF1_BIT_OFFSET, GIF1_BIT_WIDTH) as u8 }

	const TCIF1_BIT_OFFSET: u8 = 1;
	const TCIF1_BIT_WIDTH: u8 = 1;
	/// Channel 1 Transfer Complete flag (Width: 1, Offset: 1)
	pub fn get_tcif1() -> u8 { ::read(REGISTER_ADDRESS, TCIF1_BIT_OFFSET, TCIF1_BIT_WIDTH) as u8 }

	const HTIF1_BIT_OFFSET: u8 = 2;
	const HTIF1_BIT_WIDTH: u8 = 1;
	/// Channel 1 Half Transfer Complete flag (Width: 1, Offset: 2)
	pub fn get_htif1() -> u8 { ::read(REGISTER_ADDRESS, HTIF1_BIT_OFFSET, HTIF1_BIT_WIDTH) as u8 }

	const TEIF1_BIT_OFFSET: u8 = 3;
	const TEIF1_BIT_WIDTH: u8 = 1;
	/// Channel 1 Transfer Error flag (Width: 1, Offset: 3)
	pub fn get_teif1() -> u8 { ::read(REGISTER_ADDRESS, TEIF1_BIT_OFFSET, TEIF1_BIT_WIDTH) as u8 }

	const GIF2_BIT_OFFSET: u8 = 4;
	const GIF2_BIT_WIDTH: u8 = 1;
	/// Channel 2 Global interrupt flag (Width: 1, Offset: 4)
	pub fn get_gif2() -> u8 { ::read(REGISTER_ADDRESS, GIF2_BIT_OFFSET, GIF2_BIT_WIDTH) as u8 }

	const TCIF2_BIT_OFFSET: u8 = 5;
	const TCIF2_BIT_WIDTH: u8 = 1;
	/// Channel 2 Transfer Complete flag (Width: 1, Offset: 5)
	pub fn get_tcif2() -> u8 { ::read(REGISTER_ADDRESS, TCIF2_BIT_OFFSET, TCIF2_BIT_WIDTH) as u8 }

	const HTIF2_BIT_OFFSET: u8 = 6;
	const HTIF2_BIT_WIDTH: u8 = 1;
	/// Channel 2 Half Transfer Complete flag (Width: 1, Offset: 6)
	pub fn get_htif2() -> u8 { ::read(REGISTER_ADDRESS, HTIF2_BIT_OFFSET, HTIF2_BIT_WIDTH) as u8 }

	const TEIF2_BIT_OFFSET: u8 = 7;
	const TEIF2_BIT_WIDTH: u8 = 1;
	/// Channel 2 Transfer Error flag (Width: 1, Offset: 7)
	pub fn get_teif2() -> u8 { ::read(REGISTER_ADDRESS, TEIF2_BIT_OFFSET, TEIF2_BIT_WIDTH) as u8 }

	const GIF3_BIT_OFFSET: u8 = 8;
	const GIF3_BIT_WIDTH: u8 = 1;
	/// Channel 3 Global interrupt flag (Width: 1, Offset: 8)
	pub fn get_gif3() -> u8 { ::read(REGISTER_ADDRESS, GIF3_BIT_OFFSET, GIF3_BIT_WIDTH) as u8 }

	const TCIF3_BIT_OFFSET: u8 = 9;
	const TCIF3_BIT_WIDTH: u8 = 1;
	/// Channel 3 Transfer Complete flag (Width: 1, Offset: 9)
	pub fn get_tcif3() -> u8 { ::read(REGISTER_ADDRESS, TCIF3_BIT_OFFSET, TCIF3_BIT_WIDTH) as u8 }

	const HTIF3_BIT_OFFSET: u8 = 10;
	const HTIF3_BIT_WIDTH: u8 = 1;
	/// Channel 3 Half Transfer Complete flag (Width: 1, Offset: 10)
	pub fn get_htif3() -> u8 { ::read(REGISTER_ADDRESS, HTIF3_BIT_OFFSET, HTIF3_BIT_WIDTH) as u8 }

	const TEIF3_BIT_OFFSET: u8 = 11;
	const TEIF3_BIT_WIDTH: u8 = 1;
	/// Channel 3 Transfer Error flag (Width: 1, Offset: 11)
	pub fn get_teif3() -> u8 { ::read(REGISTER_ADDRESS, TEIF3_BIT_OFFSET, TEIF3_BIT_WIDTH) as u8 }

	const GIF4_BIT_OFFSET: u8 = 12;
	const GIF4_BIT_WIDTH: u8 = 1;
	/// Channel 4 Global interrupt flag (Width: 1, Offset: 12)
	pub fn get_gif4() -> u8 { ::read(REGISTER_ADDRESS, GIF4_BIT_OFFSET, GIF4_BIT_WIDTH) as u8 }

	const TCIF4_BIT_OFFSET: u8 = 13;
	const TCIF4_BIT_WIDTH: u8 = 1;
	/// Channel 4 Transfer Complete flag (Width: 1, Offset: 13)
	pub fn get_tcif4() -> u8 { ::read(REGISTER_ADDRESS, TCIF4_BIT_OFFSET, TCIF4_BIT_WIDTH) as u8 }

	const HTIF4_BIT_OFFSET: u8 = 14;
	const HTIF4_BIT_WIDTH: u8 = 1;
	/// Channel 4 Half Transfer Complete flag (Width: 1, Offset: 14)
	pub fn get_htif4() -> u8 { ::read(REGISTER_ADDRESS, HTIF4_BIT_OFFSET, HTIF4_BIT_WIDTH) as u8 }

	const TEIF4_BIT_OFFSET: u8 = 15;
	const TEIF4_BIT_WIDTH: u8 = 1;
	/// Channel 4 Transfer Error flag (Width: 1, Offset: 15)
	pub fn get_teif4() -> u8 { ::read(REGISTER_ADDRESS, TEIF4_BIT_OFFSET, TEIF4_BIT_WIDTH) as u8 }

	const GIF5_BIT_OFFSET: u8 = 16;
	const GIF5_BIT_WIDTH: u8 = 1;
	/// Channel 5 Global interrupt flag (Width: 1, Offset: 16)
	pub fn get_gif5() -> u8 { ::read(REGISTER_ADDRESS, GIF5_BIT_OFFSET, GIF5_BIT_WIDTH) as u8 }

	const TCIF5_BIT_OFFSET: u8 = 17;
	const TCIF5_BIT_WIDTH: u8 = 1;
	/// Channel 5 Transfer Complete flag (Width: 1, Offset: 17)
	pub fn get_tcif5() -> u8 { ::read(REGISTER_ADDRESS, TCIF5_BIT_OFFSET, TCIF5_BIT_WIDTH) as u8 }

	const HTIF5_BIT_OFFSET: u8 = 18;
	const HTIF5_BIT_WIDTH: u8 = 1;
	/// Channel 5 Half Transfer Complete flag (Width: 1, Offset: 18)
	pub fn get_htif5() -> u8 { ::read(REGISTER_ADDRESS, HTIF5_BIT_OFFSET, HTIF5_BIT_WIDTH) as u8 }

	const TEIF5_BIT_OFFSET: u8 = 19;
	const TEIF5_BIT_WIDTH: u8 = 1;
	/// Channel 5 Transfer Error flag (Width: 1, Offset: 19)
	pub fn get_teif5() -> u8 { ::read(REGISTER_ADDRESS, TEIF5_BIT_OFFSET, TEIF5_BIT_WIDTH) as u8 }

	const GIF6_BIT_OFFSET: u8 = 20;
	const GIF6_BIT_WIDTH: u8 = 1;
	/// Channel 6 Global interrupt flag (Width: 1, Offset: 20)
	pub fn get_gif6() -> u8 { ::read(REGISTER_ADDRESS, GIF6_BIT_OFFSET, GIF6_BIT_WIDTH) as u8 }

	const TCIF6_BIT_OFFSET: u8 = 21;
	const TCIF6_BIT_WIDTH: u8 = 1;
	/// Channel 6 Transfer Complete flag (Width: 1, Offset: 21)
	pub fn get_tcif6() -> u8 { ::read(REGISTER_ADDRESS, TCIF6_BIT_OFFSET, TCIF6_BIT_WIDTH) as u8 }

	const HTIF6_BIT_OFFSET: u8 = 22;
	const HTIF6_BIT_WIDTH: u8 = 1;
	/// Channel 6 Half Transfer Complete flag (Width: 1, Offset: 22)
	pub fn get_htif6() -> u8 { ::read(REGISTER_ADDRESS, HTIF6_BIT_OFFSET, HTIF6_BIT_WIDTH) as u8 }

	const TEIF6_BIT_OFFSET: u8 = 23;
	const TEIF6_BIT_WIDTH: u8 = 1;
	/// Channel 6 Transfer Error flag (Width: 1, Offset: 23)
	pub fn get_teif6() -> u8 { ::read(REGISTER_ADDRESS, TEIF6_BIT_OFFSET, TEIF6_BIT_WIDTH) as u8 }

	const GIF7_BIT_OFFSET: u8 = 24;
	const GIF7_BIT_WIDTH: u8 = 1;
	/// Channel 7 Global interrupt flag (Width: 1, Offset: 24)
	pub fn get_gif7() -> u8 { ::read(REGISTER_ADDRESS, GIF7_BIT_OFFSET, GIF7_BIT_WIDTH) as u8 }

	const TCIF7_BIT_OFFSET: u8 = 25;
	const TCIF7_BIT_WIDTH: u8 = 1;
	/// Channel 7 Transfer Complete flag (Width: 1, Offset: 25)
	pub fn get_tcif7() -> u8 { ::read(REGISTER_ADDRESS, TCIF7_BIT_OFFSET, TCIF7_BIT_WIDTH) as u8 }

	const HTIF7_BIT_OFFSET: u8 = 26;
	const HTIF7_BIT_WIDTH: u8 = 1;
	/// Channel 7 Half Transfer Complete flag (Width: 1, Offset: 26)
	pub fn get_htif7() -> u8 { ::read(REGISTER_ADDRESS, HTIF7_BIT_OFFSET, HTIF7_BIT_WIDTH) as u8 }

	const TEIF7_BIT_OFFSET: u8 = 27;
	const TEIF7_BIT_WIDTH: u8 = 1;
	/// Channel 7 Transfer Error flag (Width: 1, Offset: 27)
	pub fn get_teif7() -> u8 { ::read(REGISTER_ADDRESS, TEIF7_BIT_OFFSET, TEIF7_BIT_WIDTH) as u8 }
}
/// DMA interrupt flag clear register (DMA_IFCR)
/// Size: 0x20 bits
pub mod ifcr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CGIF1_BIT_OFFSET: u8 = 0;
	const CGIF1_BIT_WIDTH: u8 = 1;
	/// Channel 1 Global interrupt clear (Width: 1, Offset: 0)
	pub fn set_cgif1(value: u8) { ::write(REGISTER_ADDRESS, CGIF1_BIT_OFFSET, CGIF1_BIT_WIDTH, value as u32); }

	const CTCIF1_BIT_OFFSET: u8 = 1;
	const CTCIF1_BIT_WIDTH: u8 = 1;
	/// Channel 1 Transfer Complete clear (Width: 1, Offset: 1)
	pub fn set_ctcif1(value: u8) { ::write(REGISTER_ADDRESS, CTCIF1_BIT_OFFSET, CTCIF1_BIT_WIDTH, value as u32); }

	const CHTIF1_BIT_OFFSET: u8 = 2;
	const CHTIF1_BIT_WIDTH: u8 = 1;
	/// Channel 1 Half Transfer clear (Width: 1, Offset: 2)
	pub fn set_chtif1(value: u8) { ::write(REGISTER_ADDRESS, CHTIF1_BIT_OFFSET, CHTIF1_BIT_WIDTH, value as u32); }

	const CTEIF1_BIT_OFFSET: u8 = 3;
	const CTEIF1_BIT_WIDTH: u8 = 1;
	/// Channel 1 Transfer Error clear (Width: 1, Offset: 3)
	pub fn set_cteif1(value: u8) { ::write(REGISTER_ADDRESS, CTEIF1_BIT_OFFSET, CTEIF1_BIT_WIDTH, value as u32); }

	const CGIF2_BIT_OFFSET: u8 = 4;
	const CGIF2_BIT_WIDTH: u8 = 1;
	/// Channel 2 Global interrupt clear (Width: 1, Offset: 4)
	pub fn set_cgif2(value: u8) { ::write(REGISTER_ADDRESS, CGIF2_BIT_OFFSET, CGIF2_BIT_WIDTH, value as u32); }

	const CTCIF2_BIT_OFFSET: u8 = 5;
	const CTCIF2_BIT_WIDTH: u8 = 1;
	/// Channel 2 Transfer Complete clear (Width: 1, Offset: 5)
	pub fn set_ctcif2(value: u8) { ::write(REGISTER_ADDRESS, CTCIF2_BIT_OFFSET, CTCIF2_BIT_WIDTH, value as u32); }

	const CHTIF2_BIT_OFFSET: u8 = 6;
	const CHTIF2_BIT_WIDTH: u8 = 1;
	/// Channel 2 Half Transfer clear (Width: 1, Offset: 6)
	pub fn set_chtif2(value: u8) { ::write(REGISTER_ADDRESS, CHTIF2_BIT_OFFSET, CHTIF2_BIT_WIDTH, value as u32); }

	const CTEIF2_BIT_OFFSET: u8 = 7;
	const CTEIF2_BIT_WIDTH: u8 = 1;
	/// Channel 2 Transfer Error clear (Width: 1, Offset: 7)
	pub fn set_cteif2(value: u8) { ::write(REGISTER_ADDRESS, CTEIF2_BIT_OFFSET, CTEIF2_BIT_WIDTH, value as u32); }

	const CGIF3_BIT_OFFSET: u8 = 8;
	const CGIF3_BIT_WIDTH: u8 = 1;
	/// Channel 3 Global interrupt clear (Width: 1, Offset: 8)
	pub fn set_cgif3(value: u8) { ::write(REGISTER_ADDRESS, CGIF3_BIT_OFFSET, CGIF3_BIT_WIDTH, value as u32); }

	const CTCIF3_BIT_OFFSET: u8 = 9;
	const CTCIF3_BIT_WIDTH: u8 = 1;
	/// Channel 3 Transfer Complete clear (Width: 1, Offset: 9)
	pub fn set_ctcif3(value: u8) { ::write(REGISTER_ADDRESS, CTCIF3_BIT_OFFSET, CTCIF3_BIT_WIDTH, value as u32); }

	const CHTIF3_BIT_OFFSET: u8 = 10;
	const CHTIF3_BIT_WIDTH: u8 = 1;
	/// Channel 3 Half Transfer clear (Width: 1, Offset: 10)
	pub fn set_chtif3(value: u8) { ::write(REGISTER_ADDRESS, CHTIF3_BIT_OFFSET, CHTIF3_BIT_WIDTH, value as u32); }

	const CTEIF3_BIT_OFFSET: u8 = 11;
	const CTEIF3_BIT_WIDTH: u8 = 1;
	/// Channel 3 Transfer Error clear (Width: 1, Offset: 11)
	pub fn set_cteif3(value: u8) { ::write(REGISTER_ADDRESS, CTEIF3_BIT_OFFSET, CTEIF3_BIT_WIDTH, value as u32); }

	const CGIF4_BIT_OFFSET: u8 = 12;
	const CGIF4_BIT_WIDTH: u8 = 1;
	/// Channel 4 Global interrupt clear (Width: 1, Offset: 12)
	pub fn set_cgif4(value: u8) { ::write(REGISTER_ADDRESS, CGIF4_BIT_OFFSET, CGIF4_BIT_WIDTH, value as u32); }

	const CTCIF4_BIT_OFFSET: u8 = 13;
	const CTCIF4_BIT_WIDTH: u8 = 1;
	/// Channel 4 Transfer Complete clear (Width: 1, Offset: 13)
	pub fn set_ctcif4(value: u8) { ::write(REGISTER_ADDRESS, CTCIF4_BIT_OFFSET, CTCIF4_BIT_WIDTH, value as u32); }

	const CHTIF4_BIT_OFFSET: u8 = 14;
	const CHTIF4_BIT_WIDTH: u8 = 1;
	/// Channel 4 Half Transfer clear (Width: 1, Offset: 14)
	pub fn set_chtif4(value: u8) { ::write(REGISTER_ADDRESS, CHTIF4_BIT_OFFSET, CHTIF4_BIT_WIDTH, value as u32); }

	const CTEIF4_BIT_OFFSET: u8 = 15;
	const CTEIF4_BIT_WIDTH: u8 = 1;
	/// Channel 4 Transfer Error clear (Width: 1, Offset: 15)
	pub fn set_cteif4(value: u8) { ::write(REGISTER_ADDRESS, CTEIF4_BIT_OFFSET, CTEIF4_BIT_WIDTH, value as u32); }

	const CGIF5_BIT_OFFSET: u8 = 16;
	const CGIF5_BIT_WIDTH: u8 = 1;
	/// Channel 5 Global interrupt clear (Width: 1, Offset: 16)
	pub fn set_cgif5(value: u8) { ::write(REGISTER_ADDRESS, CGIF5_BIT_OFFSET, CGIF5_BIT_WIDTH, value as u32); }

	const CTCIF5_BIT_OFFSET: u8 = 17;
	const CTCIF5_BIT_WIDTH: u8 = 1;
	/// Channel 5 Transfer Complete clear (Width: 1, Offset: 17)
	pub fn set_ctcif5(value: u8) { ::write(REGISTER_ADDRESS, CTCIF5_BIT_OFFSET, CTCIF5_BIT_WIDTH, value as u32); }

	const CHTIF5_BIT_OFFSET: u8 = 18;
	const CHTIF5_BIT_WIDTH: u8 = 1;
	/// Channel 5 Half Transfer clear (Width: 1, Offset: 18)
	pub fn set_chtif5(value: u8) { ::write(REGISTER_ADDRESS, CHTIF5_BIT_OFFSET, CHTIF5_BIT_WIDTH, value as u32); }

	const CTEIF5_BIT_OFFSET: u8 = 19;
	const CTEIF5_BIT_WIDTH: u8 = 1;
	/// Channel 5 Transfer Error clear (Width: 1, Offset: 19)
	pub fn set_cteif5(value: u8) { ::write(REGISTER_ADDRESS, CTEIF5_BIT_OFFSET, CTEIF5_BIT_WIDTH, value as u32); }

	const CGIF6_BIT_OFFSET: u8 = 20;
	const CGIF6_BIT_WIDTH: u8 = 1;
	/// Channel 6 Global interrupt clear (Width: 1, Offset: 20)
	pub fn set_cgif6(value: u8) { ::write(REGISTER_ADDRESS, CGIF6_BIT_OFFSET, CGIF6_BIT_WIDTH, value as u32); }

	const CTCIF6_BIT_OFFSET: u8 = 21;
	const CTCIF6_BIT_WIDTH: u8 = 1;
	/// Channel 6 Transfer Complete clear (Width: 1, Offset: 21)
	pub fn set_ctcif6(value: u8) { ::write(REGISTER_ADDRESS, CTCIF6_BIT_OFFSET, CTCIF6_BIT_WIDTH, value as u32); }

	const CHTIF6_BIT_OFFSET: u8 = 22;
	const CHTIF6_BIT_WIDTH: u8 = 1;
	/// Channel 6 Half Transfer clear (Width: 1, Offset: 22)
	pub fn set_chtif6(value: u8) { ::write(REGISTER_ADDRESS, CHTIF6_BIT_OFFSET, CHTIF6_BIT_WIDTH, value as u32); }

	const CTEIF6_BIT_OFFSET: u8 = 23;
	const CTEIF6_BIT_WIDTH: u8 = 1;
	/// Channel 6 Transfer Error clear (Width: 1, Offset: 23)
	pub fn set_cteif6(value: u8) { ::write(REGISTER_ADDRESS, CTEIF6_BIT_OFFSET, CTEIF6_BIT_WIDTH, value as u32); }

	const CGIF7_BIT_OFFSET: u8 = 24;
	const CGIF7_BIT_WIDTH: u8 = 1;
	/// Channel 7 Global interrupt clear (Width: 1, Offset: 24)
	pub fn set_cgif7(value: u8) { ::write(REGISTER_ADDRESS, CGIF7_BIT_OFFSET, CGIF7_BIT_WIDTH, value as u32); }

	const CTCIF7_BIT_OFFSET: u8 = 25;
	const CTCIF7_BIT_WIDTH: u8 = 1;
	/// Channel 7 Transfer Complete clear (Width: 1, Offset: 25)
	pub fn set_ctcif7(value: u8) { ::write(REGISTER_ADDRESS, CTCIF7_BIT_OFFSET, CTCIF7_BIT_WIDTH, value as u32); }

	const CHTIF7_BIT_OFFSET: u8 = 26;
	const CHTIF7_BIT_WIDTH: u8 = 1;
	/// Channel 7 Half Transfer clear (Width: 1, Offset: 26)
	pub fn set_chtif7(value: u8) { ::write(REGISTER_ADDRESS, CHTIF7_BIT_OFFSET, CHTIF7_BIT_WIDTH, value as u32); }

	const CTEIF7_BIT_OFFSET: u8 = 27;
	const CTEIF7_BIT_WIDTH: u8 = 1;
	/// Channel 7 Transfer Error clear (Width: 1, Offset: 27)
	pub fn set_cteif7(value: u8) { ::write(REGISTER_ADDRESS, CTEIF7_BIT_OFFSET, CTEIF7_BIT_WIDTH, value as u32); }
}
/// DMA channel configuration register (DMA_CCR)
/// Size: 0x20 bits
pub mod ccr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EN_BIT_OFFSET: u8 = 0;
	const EN_BIT_WIDTH: u8 = 1;
	/// Channel enable (Width: 1, Offset: 0)
	pub fn get_en() -> u8 { ::read(REGISTER_ADDRESS, EN_BIT_OFFSET, EN_BIT_WIDTH) as u8 }
	/// Channel enable (Width: 1, Offset: 0)
	pub fn set_en(value: u8) { ::write(REGISTER_ADDRESS, EN_BIT_OFFSET, EN_BIT_WIDTH, value as u32); }

	const TCIE_BIT_OFFSET: u8 = 1;
	const TCIE_BIT_WIDTH: u8 = 1;
	/// Transfer complete interrupt enable (Width: 1, Offset: 1)
	pub fn get_tcie() -> u8 { ::read(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH) as u8 }
	/// Transfer complete interrupt enable (Width: 1, Offset: 1)
	pub fn set_tcie(value: u8) { ::write(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH, value as u32); }

	const HTIE_BIT_OFFSET: u8 = 2;
	const HTIE_BIT_WIDTH: u8 = 1;
	/// Half Transfer interrupt enable (Width: 1, Offset: 2)
	pub fn get_htie() -> u8 { ::read(REGISTER_ADDRESS, HTIE_BIT_OFFSET, HTIE_BIT_WIDTH) as u8 }
	/// Half Transfer interrupt enable (Width: 1, Offset: 2)
	pub fn set_htie(value: u8) { ::write(REGISTER_ADDRESS, HTIE_BIT_OFFSET, HTIE_BIT_WIDTH, value as u32); }

	const TEIE_BIT_OFFSET: u8 = 3;
	const TEIE_BIT_WIDTH: u8 = 1;
	/// Transfer error interrupt enable (Width: 1, Offset: 3)
	pub fn get_teie() -> u8 { ::read(REGISTER_ADDRESS, TEIE_BIT_OFFSET, TEIE_BIT_WIDTH) as u8 }
	/// Transfer error interrupt enable (Width: 1, Offset: 3)
	pub fn set_teie(value: u8) { ::write(REGISTER_ADDRESS, TEIE_BIT_OFFSET, TEIE_BIT_WIDTH, value as u32); }

	const DIR_BIT_OFFSET: u8 = 4;
	const DIR_BIT_WIDTH: u8 = 1;
	/// Data transfer direction (Width: 1, Offset: 4)
	pub fn get_dir() -> u8 { ::read(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH) as u8 }
	/// Data transfer direction (Width: 1, Offset: 4)
	pub fn set_dir(value: u8) { ::write(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH, value as u32); }

	const CIRC_BIT_OFFSET: u8 = 5;
	const CIRC_BIT_WIDTH: u8 = 1;
	/// Circular mode (Width: 1, Offset: 5)
	pub fn get_circ() -> u8 { ::read(REGISTER_ADDRESS, CIRC_BIT_OFFSET, CIRC_BIT_WIDTH) as u8 }
	/// Circular mode (Width: 1, Offset: 5)
	pub fn set_circ(value: u8) { ::write(REGISTER_ADDRESS, CIRC_BIT_OFFSET, CIRC_BIT_WIDTH, value as u32); }

	const PINC_BIT_OFFSET: u8 = 6;
	const PINC_BIT_WIDTH: u8 = 1;
	/// Peripheral increment mode (Width: 1, Offset: 6)
	pub fn get_pinc() -> u8 { ::read(REGISTER_ADDRESS, PINC_BIT_OFFSET, PINC_BIT_WIDTH) as u8 }
	/// Peripheral increment mode (Width: 1, Offset: 6)
	pub fn set_pinc(value: u8) { ::write(REGISTER_ADDRESS, PINC_BIT_OFFSET, PINC_BIT_WIDTH, value as u32); }

	const MINC_BIT_OFFSET: u8 = 7;
	const MINC_BIT_WIDTH: u8 = 1;
	/// Memory increment mode (Width: 1, Offset: 7)
	pub fn get_minc() -> u8 { ::read(REGISTER_ADDRESS, MINC_BIT_OFFSET, MINC_BIT_WIDTH) as u8 }
	/// Memory increment mode (Width: 1, Offset: 7)
	pub fn set_minc(value: u8) { ::write(REGISTER_ADDRESS, MINC_BIT_OFFSET, MINC_BIT_WIDTH, value as u32); }

	const PSIZE_BIT_OFFSET: u8 = 8;
	const PSIZE_BIT_WIDTH: u8 = 2;
	/// Peripheral size (Width: 2, Offset: 8)
	pub fn get_psize() -> u8 { ::read(REGISTER_ADDRESS, PSIZE_BIT_OFFSET, PSIZE_BIT_WIDTH) as u8 }
	/// Peripheral size (Width: 2, Offset: 8)
	pub fn set_psize(value: u8) { ::write(REGISTER_ADDRESS, PSIZE_BIT_OFFSET, PSIZE_BIT_WIDTH, value as u32); }

	const MSIZE_BIT_OFFSET: u8 = 10;
	const MSIZE_BIT_WIDTH: u8 = 2;
	/// Memory size (Width: 2, Offset: 10)
	pub fn get_msize() -> u8 { ::read(REGISTER_ADDRESS, MSIZE_BIT_OFFSET, MSIZE_BIT_WIDTH) as u8 }
	/// Memory size (Width: 2, Offset: 10)
	pub fn set_msize(value: u8) { ::write(REGISTER_ADDRESS, MSIZE_BIT_OFFSET, MSIZE_BIT_WIDTH, value as u32); }

	const PL_BIT_OFFSET: u8 = 12;
	const PL_BIT_WIDTH: u8 = 2;
	/// Channel Priority level (Width: 2, Offset: 12)
	pub fn get_pl() -> u8 { ::read(REGISTER_ADDRESS, PL_BIT_OFFSET, PL_BIT_WIDTH) as u8 }
	/// Channel Priority level (Width: 2, Offset: 12)
	pub fn set_pl(value: u8) { ::write(REGISTER_ADDRESS, PL_BIT_OFFSET, PL_BIT_WIDTH, value as u32); }

	const MEM2MEM_BIT_OFFSET: u8 = 14;
	const MEM2MEM_BIT_WIDTH: u8 = 1;
	/// Memory to memory mode (Width: 1, Offset: 14)
	pub fn get_mem2mem() -> u8 { ::read(REGISTER_ADDRESS, MEM2MEM_BIT_OFFSET, MEM2MEM_BIT_WIDTH) as u8 }
	/// Memory to memory mode (Width: 1, Offset: 14)
	pub fn set_mem2mem(value: u8) { ::write(REGISTER_ADDRESS, MEM2MEM_BIT_OFFSET, MEM2MEM_BIT_WIDTH, value as u32); }
}
/// DMA channel 1 number of data register
/// Size: 0x20 bits
pub mod cndtr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const NDT_BIT_OFFSET: u8 = 0;
	const NDT_BIT_WIDTH: u8 = 16;
	/// Number of data to transfer (Width: 16, Offset: 0)
	pub fn get_ndt() -> u16 { ::read(REGISTER_ADDRESS, NDT_BIT_OFFSET, NDT_BIT_WIDTH) as u16 }
	/// Number of data to transfer (Width: 16, Offset: 0)
	pub fn set_ndt(value: u16) { ::write(REGISTER_ADDRESS, NDT_BIT_OFFSET, NDT_BIT_WIDTH, value as u32); }
}
/// DMA channel 1 peripheral address register
/// Size: 0x20 bits
pub mod cpar1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PA_BIT_OFFSET: u8 = 0;
	const PA_BIT_WIDTH: u8 = 32;
	/// Peripheral address (Width: 32, Offset: 0)
	pub fn get_pa() -> u32 { ::read(REGISTER_ADDRESS, PA_BIT_OFFSET, PA_BIT_WIDTH) as u32 }
	/// Peripheral address (Width: 32, Offset: 0)
	pub fn set_pa(value: u32) { ::write(REGISTER_ADDRESS, PA_BIT_OFFSET, PA_BIT_WIDTH, value as u32); }
}
/// DMA channel 1 memory address register
/// Size: 0x20 bits
pub mod cmar1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MA_BIT_OFFSET: u8 = 0;
	const MA_BIT_WIDTH: u8 = 32;
	/// Memory address (Width: 32, Offset: 0)
	pub fn get_ma() -> u32 { ::read(REGISTER_ADDRESS, MA_BIT_OFFSET, MA_BIT_WIDTH) as u32 }
	/// Memory address (Width: 32, Offset: 0)
	pub fn set_ma(value: u32) { ::write(REGISTER_ADDRESS, MA_BIT_OFFSET, MA_BIT_WIDTH, value as u32); }
}
/// DMA channel configuration register (DMA_CCR)
/// Size: 0x20 bits
pub mod ccr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EN_BIT_OFFSET: u8 = 0;
	const EN_BIT_WIDTH: u8 = 1;
	/// Channel enable (Width: 1, Offset: 0)
	pub fn get_en() -> u8 { ::read(REGISTER_ADDRESS, EN_BIT_OFFSET, EN_BIT_WIDTH) as u8 }
	/// Channel enable (Width: 1, Offset: 0)
	pub fn set_en(value: u8) { ::write(REGISTER_ADDRESS, EN_BIT_OFFSET, EN_BIT_WIDTH, value as u32); }

	const TCIE_BIT_OFFSET: u8 = 1;
	const TCIE_BIT_WIDTH: u8 = 1;
	/// Transfer complete interrupt enable (Width: 1, Offset: 1)
	pub fn get_tcie() -> u8 { ::read(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH) as u8 }
	/// Transfer complete interrupt enable (Width: 1, Offset: 1)
	pub fn set_tcie(value: u8) { ::write(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH, value as u32); }

	const HTIE_BIT_OFFSET: u8 = 2;
	const HTIE_BIT_WIDTH: u8 = 1;
	/// Half Transfer interrupt enable (Width: 1, Offset: 2)
	pub fn get_htie() -> u8 { ::read(REGISTER_ADDRESS, HTIE_BIT_OFFSET, HTIE_BIT_WIDTH) as u8 }
	/// Half Transfer interrupt enable (Width: 1, Offset: 2)
	pub fn set_htie(value: u8) { ::write(REGISTER_ADDRESS, HTIE_BIT_OFFSET, HTIE_BIT_WIDTH, value as u32); }

	const TEIE_BIT_OFFSET: u8 = 3;
	const TEIE_BIT_WIDTH: u8 = 1;
	/// Transfer error interrupt enable (Width: 1, Offset: 3)
	pub fn get_teie() -> u8 { ::read(REGISTER_ADDRESS, TEIE_BIT_OFFSET, TEIE_BIT_WIDTH) as u8 }
	/// Transfer error interrupt enable (Width: 1, Offset: 3)
	pub fn set_teie(value: u8) { ::write(REGISTER_ADDRESS, TEIE_BIT_OFFSET, TEIE_BIT_WIDTH, value as u32); }

	const DIR_BIT_OFFSET: u8 = 4;
	const DIR_BIT_WIDTH: u8 = 1;
	/// Data transfer direction (Width: 1, Offset: 4)
	pub fn get_dir() -> u8 { ::read(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH) as u8 }
	/// Data transfer direction (Width: 1, Offset: 4)
	pub fn set_dir(value: u8) { ::write(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH, value as u32); }

	const CIRC_BIT_OFFSET: u8 = 5;
	const CIRC_BIT_WIDTH: u8 = 1;
	/// Circular mode (Width: 1, Offset: 5)
	pub fn get_circ() -> u8 { ::read(REGISTER_ADDRESS, CIRC_BIT_OFFSET, CIRC_BIT_WIDTH) as u8 }
	/// Circular mode (Width: 1, Offset: 5)
	pub fn set_circ(value: u8) { ::write(REGISTER_ADDRESS, CIRC_BIT_OFFSET, CIRC_BIT_WIDTH, value as u32); }

	const PINC_BIT_OFFSET: u8 = 6;
	const PINC_BIT_WIDTH: u8 = 1;
	/// Peripheral increment mode (Width: 1, Offset: 6)
	pub fn get_pinc() -> u8 { ::read(REGISTER_ADDRESS, PINC_BIT_OFFSET, PINC_BIT_WIDTH) as u8 }
	/// Peripheral increment mode (Width: 1, Offset: 6)
	pub fn set_pinc(value: u8) { ::write(REGISTER_ADDRESS, PINC_BIT_OFFSET, PINC_BIT_WIDTH, value as u32); }

	const MINC_BIT_OFFSET: u8 = 7;
	const MINC_BIT_WIDTH: u8 = 1;
	/// Memory increment mode (Width: 1, Offset: 7)
	pub fn get_minc() -> u8 { ::read(REGISTER_ADDRESS, MINC_BIT_OFFSET, MINC_BIT_WIDTH) as u8 }
	/// Memory increment mode (Width: 1, Offset: 7)
	pub fn set_minc(value: u8) { ::write(REGISTER_ADDRESS, MINC_BIT_OFFSET, MINC_BIT_WIDTH, value as u32); }

	const PSIZE_BIT_OFFSET: u8 = 8;
	const PSIZE_BIT_WIDTH: u8 = 2;
	/// Peripheral size (Width: 2, Offset: 8)
	pub fn get_psize() -> u8 { ::read(REGISTER_ADDRESS, PSIZE_BIT_OFFSET, PSIZE_BIT_WIDTH) as u8 }
	/// Peripheral size (Width: 2, Offset: 8)
	pub fn set_psize(value: u8) { ::write(REGISTER_ADDRESS, PSIZE_BIT_OFFSET, PSIZE_BIT_WIDTH, value as u32); }

	const MSIZE_BIT_OFFSET: u8 = 10;
	const MSIZE_BIT_WIDTH: u8 = 2;
	/// Memory size (Width: 2, Offset: 10)
	pub fn get_msize() -> u8 { ::read(REGISTER_ADDRESS, MSIZE_BIT_OFFSET, MSIZE_BIT_WIDTH) as u8 }
	/// Memory size (Width: 2, Offset: 10)
	pub fn set_msize(value: u8) { ::write(REGISTER_ADDRESS, MSIZE_BIT_OFFSET, MSIZE_BIT_WIDTH, value as u32); }

	const PL_BIT_OFFSET: u8 = 12;
	const PL_BIT_WIDTH: u8 = 2;
	/// Channel Priority level (Width: 2, Offset: 12)
	pub fn get_pl() -> u8 { ::read(REGISTER_ADDRESS, PL_BIT_OFFSET, PL_BIT_WIDTH) as u8 }
	/// Channel Priority level (Width: 2, Offset: 12)
	pub fn set_pl(value: u8) { ::write(REGISTER_ADDRESS, PL_BIT_OFFSET, PL_BIT_WIDTH, value as u32); }

	const MEM2MEM_BIT_OFFSET: u8 = 14;
	const MEM2MEM_BIT_WIDTH: u8 = 1;
	/// Memory to memory mode (Width: 1, Offset: 14)
	pub fn get_mem2mem() -> u8 { ::read(REGISTER_ADDRESS, MEM2MEM_BIT_OFFSET, MEM2MEM_BIT_WIDTH) as u8 }
	/// Memory to memory mode (Width: 1, Offset: 14)
	pub fn set_mem2mem(value: u8) { ::write(REGISTER_ADDRESS, MEM2MEM_BIT_OFFSET, MEM2MEM_BIT_WIDTH, value as u32); }
}
/// DMA channel 2 number of data register
/// Size: 0x20 bits
pub mod cndtr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x20;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const NDT_BIT_OFFSET: u8 = 0;
	const NDT_BIT_WIDTH: u8 = 16;
	/// Number of data to transfer (Width: 16, Offset: 0)
	pub fn get_ndt() -> u16 { ::read(REGISTER_ADDRESS, NDT_BIT_OFFSET, NDT_BIT_WIDTH) as u16 }
	/// Number of data to transfer (Width: 16, Offset: 0)
	pub fn set_ndt(value: u16) { ::write(REGISTER_ADDRESS, NDT_BIT_OFFSET, NDT_BIT_WIDTH, value as u32); }
}
/// DMA channel 2 peripheral address register
/// Size: 0x20 bits
pub mod cpar2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x24;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PA_BIT_OFFSET: u8 = 0;
	const PA_BIT_WIDTH: u8 = 32;
	/// Peripheral address (Width: 32, Offset: 0)
	pub fn get_pa() -> u32 { ::read(REGISTER_ADDRESS, PA_BIT_OFFSET, PA_BIT_WIDTH) as u32 }
	/// Peripheral address (Width: 32, Offset: 0)
	pub fn set_pa(value: u32) { ::write(REGISTER_ADDRESS, PA_BIT_OFFSET, PA_BIT_WIDTH, value as u32); }
}
/// DMA channel 2 memory address register
/// Size: 0x20 bits
pub mod cmar2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x28;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MA_BIT_OFFSET: u8 = 0;
	const MA_BIT_WIDTH: u8 = 32;
	/// Memory address (Width: 32, Offset: 0)
	pub fn get_ma() -> u32 { ::read(REGISTER_ADDRESS, MA_BIT_OFFSET, MA_BIT_WIDTH) as u32 }
	/// Memory address (Width: 32, Offset: 0)
	pub fn set_ma(value: u32) { ::write(REGISTER_ADDRESS, MA_BIT_OFFSET, MA_BIT_WIDTH, value as u32); }
}
/// DMA channel configuration register (DMA_CCR)
/// Size: 0x20 bits
pub mod ccr3 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x30;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EN_BIT_OFFSET: u8 = 0;
	const EN_BIT_WIDTH: u8 = 1;
	/// Channel enable (Width: 1, Offset: 0)
	pub fn get_en() -> u8 { ::read(REGISTER_ADDRESS, EN_BIT_OFFSET, EN_BIT_WIDTH) as u8 }
	/// Channel enable (Width: 1, Offset: 0)
	pub fn set_en(value: u8) { ::write(REGISTER_ADDRESS, EN_BIT_OFFSET, EN_BIT_WIDTH, value as u32); }

	const TCIE_BIT_OFFSET: u8 = 1;
	const TCIE_BIT_WIDTH: u8 = 1;
	/// Transfer complete interrupt enable (Width: 1, Offset: 1)
	pub fn get_tcie() -> u8 { ::read(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH) as u8 }
	/// Transfer complete interrupt enable (Width: 1, Offset: 1)
	pub fn set_tcie(value: u8) { ::write(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH, value as u32); }

	const HTIE_BIT_OFFSET: u8 = 2;
	const HTIE_BIT_WIDTH: u8 = 1;
	/// Half Transfer interrupt enable (Width: 1, Offset: 2)
	pub fn get_htie() -> u8 { ::read(REGISTER_ADDRESS, HTIE_BIT_OFFSET, HTIE_BIT_WIDTH) as u8 }
	/// Half Transfer interrupt enable (Width: 1, Offset: 2)
	pub fn set_htie(value: u8) { ::write(REGISTER_ADDRESS, HTIE_BIT_OFFSET, HTIE_BIT_WIDTH, value as u32); }

	const TEIE_BIT_OFFSET: u8 = 3;
	const TEIE_BIT_WIDTH: u8 = 1;
	/// Transfer error interrupt enable (Width: 1, Offset: 3)
	pub fn get_teie() -> u8 { ::read(REGISTER_ADDRESS, TEIE_BIT_OFFSET, TEIE_BIT_WIDTH) as u8 }
	/// Transfer error interrupt enable (Width: 1, Offset: 3)
	pub fn set_teie(value: u8) { ::write(REGISTER_ADDRESS, TEIE_BIT_OFFSET, TEIE_BIT_WIDTH, value as u32); }

	const DIR_BIT_OFFSET: u8 = 4;
	const DIR_BIT_WIDTH: u8 = 1;
	/// Data transfer direction (Width: 1, Offset: 4)
	pub fn get_dir() -> u8 { ::read(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH) as u8 }
	/// Data transfer direction (Width: 1, Offset: 4)
	pub fn set_dir(value: u8) { ::write(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH, value as u32); }

	const CIRC_BIT_OFFSET: u8 = 5;
	const CIRC_BIT_WIDTH: u8 = 1;
	/// Circular mode (Width: 1, Offset: 5)
	pub fn get_circ() -> u8 { ::read(REGISTER_ADDRESS, CIRC_BIT_OFFSET, CIRC_BIT_WIDTH) as u8 }
	/// Circular mode (Width: 1, Offset: 5)
	pub fn set_circ(value: u8) { ::write(REGISTER_ADDRESS, CIRC_BIT_OFFSET, CIRC_BIT_WIDTH, value as u32); }

	const PINC_BIT_OFFSET: u8 = 6;
	const PINC_BIT_WIDTH: u8 = 1;
	/// Peripheral increment mode (Width: 1, Offset: 6)
	pub fn get_pinc() -> u8 { ::read(REGISTER_ADDRESS, PINC_BIT_OFFSET, PINC_BIT_WIDTH) as u8 }
	/// Peripheral increment mode (Width: 1, Offset: 6)
	pub fn set_pinc(value: u8) { ::write(REGISTER_ADDRESS, PINC_BIT_OFFSET, PINC_BIT_WIDTH, value as u32); }

	const MINC_BIT_OFFSET: u8 = 7;
	const MINC_BIT_WIDTH: u8 = 1;
	/// Memory increment mode (Width: 1, Offset: 7)
	pub fn get_minc() -> u8 { ::read(REGISTER_ADDRESS, MINC_BIT_OFFSET, MINC_BIT_WIDTH) as u8 }
	/// Memory increment mode (Width: 1, Offset: 7)
	pub fn set_minc(value: u8) { ::write(REGISTER_ADDRESS, MINC_BIT_OFFSET, MINC_BIT_WIDTH, value as u32); }

	const PSIZE_BIT_OFFSET: u8 = 8;
	const PSIZE_BIT_WIDTH: u8 = 2;
	/// Peripheral size (Width: 2, Offset: 8)
	pub fn get_psize() -> u8 { ::read(REGISTER_ADDRESS, PSIZE_BIT_OFFSET, PSIZE_BIT_WIDTH) as u8 }
	/// Peripheral size (Width: 2, Offset: 8)
	pub fn set_psize(value: u8) { ::write(REGISTER_ADDRESS, PSIZE_BIT_OFFSET, PSIZE_BIT_WIDTH, value as u32); }

	const MSIZE_BIT_OFFSET: u8 = 10;
	const MSIZE_BIT_WIDTH: u8 = 2;
	/// Memory size (Width: 2, Offset: 10)
	pub fn get_msize() -> u8 { ::read(REGISTER_ADDRESS, MSIZE_BIT_OFFSET, MSIZE_BIT_WIDTH) as u8 }
	/// Memory size (Width: 2, Offset: 10)
	pub fn set_msize(value: u8) { ::write(REGISTER_ADDRESS, MSIZE_BIT_OFFSET, MSIZE_BIT_WIDTH, value as u32); }

	const PL_BIT_OFFSET: u8 = 12;
	const PL_BIT_WIDTH: u8 = 2;
	/// Channel Priority level (Width: 2, Offset: 12)
	pub fn get_pl() -> u8 { ::read(REGISTER_ADDRESS, PL_BIT_OFFSET, PL_BIT_WIDTH) as u8 }
	/// Channel Priority level (Width: 2, Offset: 12)
	pub fn set_pl(value: u8) { ::write(REGISTER_ADDRESS, PL_BIT_OFFSET, PL_BIT_WIDTH, value as u32); }

	const MEM2MEM_BIT_OFFSET: u8 = 14;
	const MEM2MEM_BIT_WIDTH: u8 = 1;
	/// Memory to memory mode (Width: 1, Offset: 14)
	pub fn get_mem2mem() -> u8 { ::read(REGISTER_ADDRESS, MEM2MEM_BIT_OFFSET, MEM2MEM_BIT_WIDTH) as u8 }
	/// Memory to memory mode (Width: 1, Offset: 14)
	pub fn set_mem2mem(value: u8) { ::write(REGISTER_ADDRESS, MEM2MEM_BIT_OFFSET, MEM2MEM_BIT_WIDTH, value as u32); }
}
/// DMA channel 3 number of data register
/// Size: 0x20 bits
pub mod cndtr3 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x34;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const NDT_BIT_OFFSET: u8 = 0;
	const NDT_BIT_WIDTH: u8 = 16;
	/// Number of data to transfer (Width: 16, Offset: 0)
	pub fn get_ndt() -> u16 { ::read(REGISTER_ADDRESS, NDT_BIT_OFFSET, NDT_BIT_WIDTH) as u16 }
	/// Number of data to transfer (Width: 16, Offset: 0)
	pub fn set_ndt(value: u16) { ::write(REGISTER_ADDRESS, NDT_BIT_OFFSET, NDT_BIT_WIDTH, value as u32); }
}
/// DMA channel 3 peripheral address register
/// Size: 0x20 bits
pub mod cpar3 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x38;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PA_BIT_OFFSET: u8 = 0;
	const PA_BIT_WIDTH: u8 = 32;
	/// Peripheral address (Width: 32, Offset: 0)
	pub fn get_pa() -> u32 { ::read(REGISTER_ADDRESS, PA_BIT_OFFSET, PA_BIT_WIDTH) as u32 }
	/// Peripheral address (Width: 32, Offset: 0)
	pub fn set_pa(value: u32) { ::write(REGISTER_ADDRESS, PA_BIT_OFFSET, PA_BIT_WIDTH, value as u32); }
}
/// DMA channel 3 memory address register
/// Size: 0x20 bits
pub mod cmar3 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x3C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MA_BIT_OFFSET: u8 = 0;
	const MA_BIT_WIDTH: u8 = 32;
	/// Memory address (Width: 32, Offset: 0)
	pub fn get_ma() -> u32 { ::read(REGISTER_ADDRESS, MA_BIT_OFFSET, MA_BIT_WIDTH) as u32 }
	/// Memory address (Width: 32, Offset: 0)
	pub fn set_ma(value: u32) { ::write(REGISTER_ADDRESS, MA_BIT_OFFSET, MA_BIT_WIDTH, value as u32); }
}
/// DMA channel configuration register (DMA_CCR)
/// Size: 0x20 bits
pub mod ccr4 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x44;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EN_BIT_OFFSET: u8 = 0;
	const EN_BIT_WIDTH: u8 = 1;
	/// Channel enable (Width: 1, Offset: 0)
	pub fn get_en() -> u8 { ::read(REGISTER_ADDRESS, EN_BIT_OFFSET, EN_BIT_WIDTH) as u8 }
	/// Channel enable (Width: 1, Offset: 0)
	pub fn set_en(value: u8) { ::write(REGISTER_ADDRESS, EN_BIT_OFFSET, EN_BIT_WIDTH, value as u32); }

	const TCIE_BIT_OFFSET: u8 = 1;
	const TCIE_BIT_WIDTH: u8 = 1;
	/// Transfer complete interrupt enable (Width: 1, Offset: 1)
	pub fn get_tcie() -> u8 { ::read(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH) as u8 }
	/// Transfer complete interrupt enable (Width: 1, Offset: 1)
	pub fn set_tcie(value: u8) { ::write(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH, value as u32); }

	const HTIE_BIT_OFFSET: u8 = 2;
	const HTIE_BIT_WIDTH: u8 = 1;
	/// Half Transfer interrupt enable (Width: 1, Offset: 2)
	pub fn get_htie() -> u8 { ::read(REGISTER_ADDRESS, HTIE_BIT_OFFSET, HTIE_BIT_WIDTH) as u8 }
	/// Half Transfer interrupt enable (Width: 1, Offset: 2)
	pub fn set_htie(value: u8) { ::write(REGISTER_ADDRESS, HTIE_BIT_OFFSET, HTIE_BIT_WIDTH, value as u32); }

	const TEIE_BIT_OFFSET: u8 = 3;
	const TEIE_BIT_WIDTH: u8 = 1;
	/// Transfer error interrupt enable (Width: 1, Offset: 3)
	pub fn get_teie() -> u8 { ::read(REGISTER_ADDRESS, TEIE_BIT_OFFSET, TEIE_BIT_WIDTH) as u8 }
	/// Transfer error interrupt enable (Width: 1, Offset: 3)
	pub fn set_teie(value: u8) { ::write(REGISTER_ADDRESS, TEIE_BIT_OFFSET, TEIE_BIT_WIDTH, value as u32); }

	const DIR_BIT_OFFSET: u8 = 4;
	const DIR_BIT_WIDTH: u8 = 1;
	/// Data transfer direction (Width: 1, Offset: 4)
	pub fn get_dir() -> u8 { ::read(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH) as u8 }
	/// Data transfer direction (Width: 1, Offset: 4)
	pub fn set_dir(value: u8) { ::write(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH, value as u32); }

	const CIRC_BIT_OFFSET: u8 = 5;
	const CIRC_BIT_WIDTH: u8 = 1;
	/// Circular mode (Width: 1, Offset: 5)
	pub fn get_circ() -> u8 { ::read(REGISTER_ADDRESS, CIRC_BIT_OFFSET, CIRC_BIT_WIDTH) as u8 }
	/// Circular mode (Width: 1, Offset: 5)
	pub fn set_circ(value: u8) { ::write(REGISTER_ADDRESS, CIRC_BIT_OFFSET, CIRC_BIT_WIDTH, value as u32); }

	const PINC_BIT_OFFSET: u8 = 6;
	const PINC_BIT_WIDTH: u8 = 1;
	/// Peripheral increment mode (Width: 1, Offset: 6)
	pub fn get_pinc() -> u8 { ::read(REGISTER_ADDRESS, PINC_BIT_OFFSET, PINC_BIT_WIDTH) as u8 }
	/// Peripheral increment mode (Width: 1, Offset: 6)
	pub fn set_pinc(value: u8) { ::write(REGISTER_ADDRESS, PINC_BIT_OFFSET, PINC_BIT_WIDTH, value as u32); }

	const MINC_BIT_OFFSET: u8 = 7;
	const MINC_BIT_WIDTH: u8 = 1;
	/// Memory increment mode (Width: 1, Offset: 7)
	pub fn get_minc() -> u8 { ::read(REGISTER_ADDRESS, MINC_BIT_OFFSET, MINC_BIT_WIDTH) as u8 }
	/// Memory increment mode (Width: 1, Offset: 7)
	pub fn set_minc(value: u8) { ::write(REGISTER_ADDRESS, MINC_BIT_OFFSET, MINC_BIT_WIDTH, value as u32); }

	const PSIZE_BIT_OFFSET: u8 = 8;
	const PSIZE_BIT_WIDTH: u8 = 2;
	/// Peripheral size (Width: 2, Offset: 8)
	pub fn get_psize() -> u8 { ::read(REGISTER_ADDRESS, PSIZE_BIT_OFFSET, PSIZE_BIT_WIDTH) as u8 }
	/// Peripheral size (Width: 2, Offset: 8)
	pub fn set_psize(value: u8) { ::write(REGISTER_ADDRESS, PSIZE_BIT_OFFSET, PSIZE_BIT_WIDTH, value as u32); }

	const MSIZE_BIT_OFFSET: u8 = 10;
	const MSIZE_BIT_WIDTH: u8 = 2;
	/// Memory size (Width: 2, Offset: 10)
	pub fn get_msize() -> u8 { ::read(REGISTER_ADDRESS, MSIZE_BIT_OFFSET, MSIZE_BIT_WIDTH) as u8 }
	/// Memory size (Width: 2, Offset: 10)
	pub fn set_msize(value: u8) { ::write(REGISTER_ADDRESS, MSIZE_BIT_OFFSET, MSIZE_BIT_WIDTH, value as u32); }

	const PL_BIT_OFFSET: u8 = 12;
	const PL_BIT_WIDTH: u8 = 2;
	/// Channel Priority level (Width: 2, Offset: 12)
	pub fn get_pl() -> u8 { ::read(REGISTER_ADDRESS, PL_BIT_OFFSET, PL_BIT_WIDTH) as u8 }
	/// Channel Priority level (Width: 2, Offset: 12)
	pub fn set_pl(value: u8) { ::write(REGISTER_ADDRESS, PL_BIT_OFFSET, PL_BIT_WIDTH, value as u32); }

	const MEM2MEM_BIT_OFFSET: u8 = 14;
	const MEM2MEM_BIT_WIDTH: u8 = 1;
	/// Memory to memory mode (Width: 1, Offset: 14)
	pub fn get_mem2mem() -> u8 { ::read(REGISTER_ADDRESS, MEM2MEM_BIT_OFFSET, MEM2MEM_BIT_WIDTH) as u8 }
	/// Memory to memory mode (Width: 1, Offset: 14)
	pub fn set_mem2mem(value: u8) { ::write(REGISTER_ADDRESS, MEM2MEM_BIT_OFFSET, MEM2MEM_BIT_WIDTH, value as u32); }
}
/// DMA channel 4 number of data register
/// Size: 0x20 bits
pub mod cndtr4 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x48;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const NDT_BIT_OFFSET: u8 = 0;
	const NDT_BIT_WIDTH: u8 = 16;
	/// Number of data to transfer (Width: 16, Offset: 0)
	pub fn get_ndt() -> u16 { ::read(REGISTER_ADDRESS, NDT_BIT_OFFSET, NDT_BIT_WIDTH) as u16 }
	/// Number of data to transfer (Width: 16, Offset: 0)
	pub fn set_ndt(value: u16) { ::write(REGISTER_ADDRESS, NDT_BIT_OFFSET, NDT_BIT_WIDTH, value as u32); }
}
/// DMA channel 4 peripheral address register
/// Size: 0x20 bits
pub mod cpar4 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PA_BIT_OFFSET: u8 = 0;
	const PA_BIT_WIDTH: u8 = 32;
	/// Peripheral address (Width: 32, Offset: 0)
	pub fn get_pa() -> u32 { ::read(REGISTER_ADDRESS, PA_BIT_OFFSET, PA_BIT_WIDTH) as u32 }
	/// Peripheral address (Width: 32, Offset: 0)
	pub fn set_pa(value: u32) { ::write(REGISTER_ADDRESS, PA_BIT_OFFSET, PA_BIT_WIDTH, value as u32); }
}
/// DMA channel 4 memory address register
/// Size: 0x20 bits
pub mod cmar4 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x50;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MA_BIT_OFFSET: u8 = 0;
	const MA_BIT_WIDTH: u8 = 32;
	/// Memory address (Width: 32, Offset: 0)
	pub fn get_ma() -> u32 { ::read(REGISTER_ADDRESS, MA_BIT_OFFSET, MA_BIT_WIDTH) as u32 }
	/// Memory address (Width: 32, Offset: 0)
	pub fn set_ma(value: u32) { ::write(REGISTER_ADDRESS, MA_BIT_OFFSET, MA_BIT_WIDTH, value as u32); }
}
/// DMA channel configuration register (DMA_CCR)
/// Size: 0x20 bits
pub mod ccr5 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x58;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EN_BIT_OFFSET: u8 = 0;
	const EN_BIT_WIDTH: u8 = 1;
	/// Channel enable (Width: 1, Offset: 0)
	pub fn get_en() -> u8 { ::read(REGISTER_ADDRESS, EN_BIT_OFFSET, EN_BIT_WIDTH) as u8 }
	/// Channel enable (Width: 1, Offset: 0)
	pub fn set_en(value: u8) { ::write(REGISTER_ADDRESS, EN_BIT_OFFSET, EN_BIT_WIDTH, value as u32); }

	const TCIE_BIT_OFFSET: u8 = 1;
	const TCIE_BIT_WIDTH: u8 = 1;
	/// Transfer complete interrupt enable (Width: 1, Offset: 1)
	pub fn get_tcie() -> u8 { ::read(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH) as u8 }
	/// Transfer complete interrupt enable (Width: 1, Offset: 1)
	pub fn set_tcie(value: u8) { ::write(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH, value as u32); }

	const HTIE_BIT_OFFSET: u8 = 2;
	const HTIE_BIT_WIDTH: u8 = 1;
	/// Half Transfer interrupt enable (Width: 1, Offset: 2)
	pub fn get_htie() -> u8 { ::read(REGISTER_ADDRESS, HTIE_BIT_OFFSET, HTIE_BIT_WIDTH) as u8 }
	/// Half Transfer interrupt enable (Width: 1, Offset: 2)
	pub fn set_htie(value: u8) { ::write(REGISTER_ADDRESS, HTIE_BIT_OFFSET, HTIE_BIT_WIDTH, value as u32); }

	const TEIE_BIT_OFFSET: u8 = 3;
	const TEIE_BIT_WIDTH: u8 = 1;
	/// Transfer error interrupt enable (Width: 1, Offset: 3)
	pub fn get_teie() -> u8 { ::read(REGISTER_ADDRESS, TEIE_BIT_OFFSET, TEIE_BIT_WIDTH) as u8 }
	/// Transfer error interrupt enable (Width: 1, Offset: 3)
	pub fn set_teie(value: u8) { ::write(REGISTER_ADDRESS, TEIE_BIT_OFFSET, TEIE_BIT_WIDTH, value as u32); }

	const DIR_BIT_OFFSET: u8 = 4;
	const DIR_BIT_WIDTH: u8 = 1;
	/// Data transfer direction (Width: 1, Offset: 4)
	pub fn get_dir() -> u8 { ::read(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH) as u8 }
	/// Data transfer direction (Width: 1, Offset: 4)
	pub fn set_dir(value: u8) { ::write(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH, value as u32); }

	const CIRC_BIT_OFFSET: u8 = 5;
	const CIRC_BIT_WIDTH: u8 = 1;
	/// Circular mode (Width: 1, Offset: 5)
	pub fn get_circ() -> u8 { ::read(REGISTER_ADDRESS, CIRC_BIT_OFFSET, CIRC_BIT_WIDTH) as u8 }
	/// Circular mode (Width: 1, Offset: 5)
	pub fn set_circ(value: u8) { ::write(REGISTER_ADDRESS, CIRC_BIT_OFFSET, CIRC_BIT_WIDTH, value as u32); }

	const PINC_BIT_OFFSET: u8 = 6;
	const PINC_BIT_WIDTH: u8 = 1;
	/// Peripheral increment mode (Width: 1, Offset: 6)
	pub fn get_pinc() -> u8 { ::read(REGISTER_ADDRESS, PINC_BIT_OFFSET, PINC_BIT_WIDTH) as u8 }
	/// Peripheral increment mode (Width: 1, Offset: 6)
	pub fn set_pinc(value: u8) { ::write(REGISTER_ADDRESS, PINC_BIT_OFFSET, PINC_BIT_WIDTH, value as u32); }

	const MINC_BIT_OFFSET: u8 = 7;
	const MINC_BIT_WIDTH: u8 = 1;
	/// Memory increment mode (Width: 1, Offset: 7)
	pub fn get_minc() -> u8 { ::read(REGISTER_ADDRESS, MINC_BIT_OFFSET, MINC_BIT_WIDTH) as u8 }
	/// Memory increment mode (Width: 1, Offset: 7)
	pub fn set_minc(value: u8) { ::write(REGISTER_ADDRESS, MINC_BIT_OFFSET, MINC_BIT_WIDTH, value as u32); }

	const PSIZE_BIT_OFFSET: u8 = 8;
	const PSIZE_BIT_WIDTH: u8 = 2;
	/// Peripheral size (Width: 2, Offset: 8)
	pub fn get_psize() -> u8 { ::read(REGISTER_ADDRESS, PSIZE_BIT_OFFSET, PSIZE_BIT_WIDTH) as u8 }
	/// Peripheral size (Width: 2, Offset: 8)
	pub fn set_psize(value: u8) { ::write(REGISTER_ADDRESS, PSIZE_BIT_OFFSET, PSIZE_BIT_WIDTH, value as u32); }

	const MSIZE_BIT_OFFSET: u8 = 10;
	const MSIZE_BIT_WIDTH: u8 = 2;
	/// Memory size (Width: 2, Offset: 10)
	pub fn get_msize() -> u8 { ::read(REGISTER_ADDRESS, MSIZE_BIT_OFFSET, MSIZE_BIT_WIDTH) as u8 }
	/// Memory size (Width: 2, Offset: 10)
	pub fn set_msize(value: u8) { ::write(REGISTER_ADDRESS, MSIZE_BIT_OFFSET, MSIZE_BIT_WIDTH, value as u32); }

	const PL_BIT_OFFSET: u8 = 12;
	const PL_BIT_WIDTH: u8 = 2;
	/// Channel Priority level (Width: 2, Offset: 12)
	pub fn get_pl() -> u8 { ::read(REGISTER_ADDRESS, PL_BIT_OFFSET, PL_BIT_WIDTH) as u8 }
	/// Channel Priority level (Width: 2, Offset: 12)
	pub fn set_pl(value: u8) { ::write(REGISTER_ADDRESS, PL_BIT_OFFSET, PL_BIT_WIDTH, value as u32); }

	const MEM2MEM_BIT_OFFSET: u8 = 14;
	const MEM2MEM_BIT_WIDTH: u8 = 1;
	/// Memory to memory mode (Width: 1, Offset: 14)
	pub fn get_mem2mem() -> u8 { ::read(REGISTER_ADDRESS, MEM2MEM_BIT_OFFSET, MEM2MEM_BIT_WIDTH) as u8 }
	/// Memory to memory mode (Width: 1, Offset: 14)
	pub fn set_mem2mem(value: u8) { ::write(REGISTER_ADDRESS, MEM2MEM_BIT_OFFSET, MEM2MEM_BIT_WIDTH, value as u32); }
}
/// DMA channel 5 number of data register
/// Size: 0x20 bits
pub mod cndtr5 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x5C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const NDT_BIT_OFFSET: u8 = 0;
	const NDT_BIT_WIDTH: u8 = 16;
	/// Number of data to transfer (Width: 16, Offset: 0)
	pub fn get_ndt() -> u16 { ::read(REGISTER_ADDRESS, NDT_BIT_OFFSET, NDT_BIT_WIDTH) as u16 }
	/// Number of data to transfer (Width: 16, Offset: 0)
	pub fn set_ndt(value: u16) { ::write(REGISTER_ADDRESS, NDT_BIT_OFFSET, NDT_BIT_WIDTH, value as u32); }
}
/// DMA channel 5 peripheral address register
/// Size: 0x20 bits
pub mod cpar5 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x60;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PA_BIT_OFFSET: u8 = 0;
	const PA_BIT_WIDTH: u8 = 32;
	/// Peripheral address (Width: 32, Offset: 0)
	pub fn get_pa() -> u32 { ::read(REGISTER_ADDRESS, PA_BIT_OFFSET, PA_BIT_WIDTH) as u32 }
	/// Peripheral address (Width: 32, Offset: 0)
	pub fn set_pa(value: u32) { ::write(REGISTER_ADDRESS, PA_BIT_OFFSET, PA_BIT_WIDTH, value as u32); }
}
/// DMA channel 5 memory address register
/// Size: 0x20 bits
pub mod cmar5 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x64;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MA_BIT_OFFSET: u8 = 0;
	const MA_BIT_WIDTH: u8 = 32;
	/// Memory address (Width: 32, Offset: 0)
	pub fn get_ma() -> u32 { ::read(REGISTER_ADDRESS, MA_BIT_OFFSET, MA_BIT_WIDTH) as u32 }
	/// Memory address (Width: 32, Offset: 0)
	pub fn set_ma(value: u32) { ::write(REGISTER_ADDRESS, MA_BIT_OFFSET, MA_BIT_WIDTH, value as u32); }
}
/// DMA channel configuration register (DMA_CCR)
/// Size: 0x20 bits
pub mod ccr6 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x6C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EN_BIT_OFFSET: u8 = 0;
	const EN_BIT_WIDTH: u8 = 1;
	/// Channel enable (Width: 1, Offset: 0)
	pub fn get_en() -> u8 { ::read(REGISTER_ADDRESS, EN_BIT_OFFSET, EN_BIT_WIDTH) as u8 }
	/// Channel enable (Width: 1, Offset: 0)
	pub fn set_en(value: u8) { ::write(REGISTER_ADDRESS, EN_BIT_OFFSET, EN_BIT_WIDTH, value as u32); }

	const TCIE_BIT_OFFSET: u8 = 1;
	const TCIE_BIT_WIDTH: u8 = 1;
	/// Transfer complete interrupt enable (Width: 1, Offset: 1)
	pub fn get_tcie() -> u8 { ::read(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH) as u8 }
	/// Transfer complete interrupt enable (Width: 1, Offset: 1)
	pub fn set_tcie(value: u8) { ::write(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH, value as u32); }

	const HTIE_BIT_OFFSET: u8 = 2;
	const HTIE_BIT_WIDTH: u8 = 1;
	/// Half Transfer interrupt enable (Width: 1, Offset: 2)
	pub fn get_htie() -> u8 { ::read(REGISTER_ADDRESS, HTIE_BIT_OFFSET, HTIE_BIT_WIDTH) as u8 }
	/// Half Transfer interrupt enable (Width: 1, Offset: 2)
	pub fn set_htie(value: u8) { ::write(REGISTER_ADDRESS, HTIE_BIT_OFFSET, HTIE_BIT_WIDTH, value as u32); }

	const TEIE_BIT_OFFSET: u8 = 3;
	const TEIE_BIT_WIDTH: u8 = 1;
	/// Transfer error interrupt enable (Width: 1, Offset: 3)
	pub fn get_teie() -> u8 { ::read(REGISTER_ADDRESS, TEIE_BIT_OFFSET, TEIE_BIT_WIDTH) as u8 }
	/// Transfer error interrupt enable (Width: 1, Offset: 3)
	pub fn set_teie(value: u8) { ::write(REGISTER_ADDRESS, TEIE_BIT_OFFSET, TEIE_BIT_WIDTH, value as u32); }

	const DIR_BIT_OFFSET: u8 = 4;
	const DIR_BIT_WIDTH: u8 = 1;
	/// Data transfer direction (Width: 1, Offset: 4)
	pub fn get_dir() -> u8 { ::read(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH) as u8 }
	/// Data transfer direction (Width: 1, Offset: 4)
	pub fn set_dir(value: u8) { ::write(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH, value as u32); }

	const CIRC_BIT_OFFSET: u8 = 5;
	const CIRC_BIT_WIDTH: u8 = 1;
	/// Circular mode (Width: 1, Offset: 5)
	pub fn get_circ() -> u8 { ::read(REGISTER_ADDRESS, CIRC_BIT_OFFSET, CIRC_BIT_WIDTH) as u8 }
	/// Circular mode (Width: 1, Offset: 5)
	pub fn set_circ(value: u8) { ::write(REGISTER_ADDRESS, CIRC_BIT_OFFSET, CIRC_BIT_WIDTH, value as u32); }

	const PINC_BIT_OFFSET: u8 = 6;
	const PINC_BIT_WIDTH: u8 = 1;
	/// Peripheral increment mode (Width: 1, Offset: 6)
	pub fn get_pinc() -> u8 { ::read(REGISTER_ADDRESS, PINC_BIT_OFFSET, PINC_BIT_WIDTH) as u8 }
	/// Peripheral increment mode (Width: 1, Offset: 6)
	pub fn set_pinc(value: u8) { ::write(REGISTER_ADDRESS, PINC_BIT_OFFSET, PINC_BIT_WIDTH, value as u32); }

	const MINC_BIT_OFFSET: u8 = 7;
	const MINC_BIT_WIDTH: u8 = 1;
	/// Memory increment mode (Width: 1, Offset: 7)
	pub fn get_minc() -> u8 { ::read(REGISTER_ADDRESS, MINC_BIT_OFFSET, MINC_BIT_WIDTH) as u8 }
	/// Memory increment mode (Width: 1, Offset: 7)
	pub fn set_minc(value: u8) { ::write(REGISTER_ADDRESS, MINC_BIT_OFFSET, MINC_BIT_WIDTH, value as u32); }

	const PSIZE_BIT_OFFSET: u8 = 8;
	const PSIZE_BIT_WIDTH: u8 = 2;
	/// Peripheral size (Width: 2, Offset: 8)
	pub fn get_psize() -> u8 { ::read(REGISTER_ADDRESS, PSIZE_BIT_OFFSET, PSIZE_BIT_WIDTH) as u8 }
	/// Peripheral size (Width: 2, Offset: 8)
	pub fn set_psize(value: u8) { ::write(REGISTER_ADDRESS, PSIZE_BIT_OFFSET, PSIZE_BIT_WIDTH, value as u32); }

	const MSIZE_BIT_OFFSET: u8 = 10;
	const MSIZE_BIT_WIDTH: u8 = 2;
	/// Memory size (Width: 2, Offset: 10)
	pub fn get_msize() -> u8 { ::read(REGISTER_ADDRESS, MSIZE_BIT_OFFSET, MSIZE_BIT_WIDTH) as u8 }
	/// Memory size (Width: 2, Offset: 10)
	pub fn set_msize(value: u8) { ::write(REGISTER_ADDRESS, MSIZE_BIT_OFFSET, MSIZE_BIT_WIDTH, value as u32); }

	const PL_BIT_OFFSET: u8 = 12;
	const PL_BIT_WIDTH: u8 = 2;
	/// Channel Priority level (Width: 2, Offset: 12)
	pub fn get_pl() -> u8 { ::read(REGISTER_ADDRESS, PL_BIT_OFFSET, PL_BIT_WIDTH) as u8 }
	/// Channel Priority level (Width: 2, Offset: 12)
	pub fn set_pl(value: u8) { ::write(REGISTER_ADDRESS, PL_BIT_OFFSET, PL_BIT_WIDTH, value as u32); }

	const MEM2MEM_BIT_OFFSET: u8 = 14;
	const MEM2MEM_BIT_WIDTH: u8 = 1;
	/// Memory to memory mode (Width: 1, Offset: 14)
	pub fn get_mem2mem() -> u8 { ::read(REGISTER_ADDRESS, MEM2MEM_BIT_OFFSET, MEM2MEM_BIT_WIDTH) as u8 }
	/// Memory to memory mode (Width: 1, Offset: 14)
	pub fn set_mem2mem(value: u8) { ::write(REGISTER_ADDRESS, MEM2MEM_BIT_OFFSET, MEM2MEM_BIT_WIDTH, value as u32); }
}
/// DMA channel 6 number of data register
/// Size: 0x20 bits
pub mod cndtr6 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x70;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const NDT_BIT_OFFSET: u8 = 0;
	const NDT_BIT_WIDTH: u8 = 16;
	/// Number of data to transfer (Width: 16, Offset: 0)
	pub fn get_ndt() -> u16 { ::read(REGISTER_ADDRESS, NDT_BIT_OFFSET, NDT_BIT_WIDTH) as u16 }
	/// Number of data to transfer (Width: 16, Offset: 0)
	pub fn set_ndt(value: u16) { ::write(REGISTER_ADDRESS, NDT_BIT_OFFSET, NDT_BIT_WIDTH, value as u32); }
}
/// DMA channel 6 peripheral address register
/// Size: 0x20 bits
pub mod cpar6 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x74;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PA_BIT_OFFSET: u8 = 0;
	const PA_BIT_WIDTH: u8 = 32;
	/// Peripheral address (Width: 32, Offset: 0)
	pub fn get_pa() -> u32 { ::read(REGISTER_ADDRESS, PA_BIT_OFFSET, PA_BIT_WIDTH) as u32 }
	/// Peripheral address (Width: 32, Offset: 0)
	pub fn set_pa(value: u32) { ::write(REGISTER_ADDRESS, PA_BIT_OFFSET, PA_BIT_WIDTH, value as u32); }
}
/// DMA channel 6 memory address register
/// Size: 0x20 bits
pub mod cmar6 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x78;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MA_BIT_OFFSET: u8 = 0;
	const MA_BIT_WIDTH: u8 = 32;
	/// Memory address (Width: 32, Offset: 0)
	pub fn get_ma() -> u32 { ::read(REGISTER_ADDRESS, MA_BIT_OFFSET, MA_BIT_WIDTH) as u32 }
	/// Memory address (Width: 32, Offset: 0)
	pub fn set_ma(value: u32) { ::write(REGISTER_ADDRESS, MA_BIT_OFFSET, MA_BIT_WIDTH, value as u32); }
}
/// DMA channel configuration register (DMA_CCR)
/// Size: 0x20 bits
pub mod ccr7 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x80;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EN_BIT_OFFSET: u8 = 0;
	const EN_BIT_WIDTH: u8 = 1;
	/// Channel enable (Width: 1, Offset: 0)
	pub fn get_en() -> u8 { ::read(REGISTER_ADDRESS, EN_BIT_OFFSET, EN_BIT_WIDTH) as u8 }
	/// Channel enable (Width: 1, Offset: 0)
	pub fn set_en(value: u8) { ::write(REGISTER_ADDRESS, EN_BIT_OFFSET, EN_BIT_WIDTH, value as u32); }

	const TCIE_BIT_OFFSET: u8 = 1;
	const TCIE_BIT_WIDTH: u8 = 1;
	/// Transfer complete interrupt enable (Width: 1, Offset: 1)
	pub fn get_tcie() -> u8 { ::read(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH) as u8 }
	/// Transfer complete interrupt enable (Width: 1, Offset: 1)
	pub fn set_tcie(value: u8) { ::write(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH, value as u32); }

	const HTIE_BIT_OFFSET: u8 = 2;
	const HTIE_BIT_WIDTH: u8 = 1;
	/// Half Transfer interrupt enable (Width: 1, Offset: 2)
	pub fn get_htie() -> u8 { ::read(REGISTER_ADDRESS, HTIE_BIT_OFFSET, HTIE_BIT_WIDTH) as u8 }
	/// Half Transfer interrupt enable (Width: 1, Offset: 2)
	pub fn set_htie(value: u8) { ::write(REGISTER_ADDRESS, HTIE_BIT_OFFSET, HTIE_BIT_WIDTH, value as u32); }

	const TEIE_BIT_OFFSET: u8 = 3;
	const TEIE_BIT_WIDTH: u8 = 1;
	/// Transfer error interrupt enable (Width: 1, Offset: 3)
	pub fn get_teie() -> u8 { ::read(REGISTER_ADDRESS, TEIE_BIT_OFFSET, TEIE_BIT_WIDTH) as u8 }
	/// Transfer error interrupt enable (Width: 1, Offset: 3)
	pub fn set_teie(value: u8) { ::write(REGISTER_ADDRESS, TEIE_BIT_OFFSET, TEIE_BIT_WIDTH, value as u32); }

	const DIR_BIT_OFFSET: u8 = 4;
	const DIR_BIT_WIDTH: u8 = 1;
	/// Data transfer direction (Width: 1, Offset: 4)
	pub fn get_dir() -> u8 { ::read(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH) as u8 }
	/// Data transfer direction (Width: 1, Offset: 4)
	pub fn set_dir(value: u8) { ::write(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH, value as u32); }

	const CIRC_BIT_OFFSET: u8 = 5;
	const CIRC_BIT_WIDTH: u8 = 1;
	/// Circular mode (Width: 1, Offset: 5)
	pub fn get_circ() -> u8 { ::read(REGISTER_ADDRESS, CIRC_BIT_OFFSET, CIRC_BIT_WIDTH) as u8 }
	/// Circular mode (Width: 1, Offset: 5)
	pub fn set_circ(value: u8) { ::write(REGISTER_ADDRESS, CIRC_BIT_OFFSET, CIRC_BIT_WIDTH, value as u32); }

	const PINC_BIT_OFFSET: u8 = 6;
	const PINC_BIT_WIDTH: u8 = 1;
	/// Peripheral increment mode (Width: 1, Offset: 6)
	pub fn get_pinc() -> u8 { ::read(REGISTER_ADDRESS, PINC_BIT_OFFSET, PINC_BIT_WIDTH) as u8 }
	/// Peripheral increment mode (Width: 1, Offset: 6)
	pub fn set_pinc(value: u8) { ::write(REGISTER_ADDRESS, PINC_BIT_OFFSET, PINC_BIT_WIDTH, value as u32); }

	const MINC_BIT_OFFSET: u8 = 7;
	const MINC_BIT_WIDTH: u8 = 1;
	/// Memory increment mode (Width: 1, Offset: 7)
	pub fn get_minc() -> u8 { ::read(REGISTER_ADDRESS, MINC_BIT_OFFSET, MINC_BIT_WIDTH) as u8 }
	/// Memory increment mode (Width: 1, Offset: 7)
	pub fn set_minc(value: u8) { ::write(REGISTER_ADDRESS, MINC_BIT_OFFSET, MINC_BIT_WIDTH, value as u32); }

	const PSIZE_BIT_OFFSET: u8 = 8;
	const PSIZE_BIT_WIDTH: u8 = 2;
	/// Peripheral size (Width: 2, Offset: 8)
	pub fn get_psize() -> u8 { ::read(REGISTER_ADDRESS, PSIZE_BIT_OFFSET, PSIZE_BIT_WIDTH) as u8 }
	/// Peripheral size (Width: 2, Offset: 8)
	pub fn set_psize(value: u8) { ::write(REGISTER_ADDRESS, PSIZE_BIT_OFFSET, PSIZE_BIT_WIDTH, value as u32); }

	const MSIZE_BIT_OFFSET: u8 = 10;
	const MSIZE_BIT_WIDTH: u8 = 2;
	/// Memory size (Width: 2, Offset: 10)
	pub fn get_msize() -> u8 { ::read(REGISTER_ADDRESS, MSIZE_BIT_OFFSET, MSIZE_BIT_WIDTH) as u8 }
	/// Memory size (Width: 2, Offset: 10)
	pub fn set_msize(value: u8) { ::write(REGISTER_ADDRESS, MSIZE_BIT_OFFSET, MSIZE_BIT_WIDTH, value as u32); }

	const PL_BIT_OFFSET: u8 = 12;
	const PL_BIT_WIDTH: u8 = 2;
	/// Channel Priority level (Width: 2, Offset: 12)
	pub fn get_pl() -> u8 { ::read(REGISTER_ADDRESS, PL_BIT_OFFSET, PL_BIT_WIDTH) as u8 }
	/// Channel Priority level (Width: 2, Offset: 12)
	pub fn set_pl(value: u8) { ::write(REGISTER_ADDRESS, PL_BIT_OFFSET, PL_BIT_WIDTH, value as u32); }

	const MEM2MEM_BIT_OFFSET: u8 = 14;
	const MEM2MEM_BIT_WIDTH: u8 = 1;
	/// Memory to memory mode (Width: 1, Offset: 14)
	pub fn get_mem2mem() -> u8 { ::read(REGISTER_ADDRESS, MEM2MEM_BIT_OFFSET, MEM2MEM_BIT_WIDTH) as u8 }
	/// Memory to memory mode (Width: 1, Offset: 14)
	pub fn set_mem2mem(value: u8) { ::write(REGISTER_ADDRESS, MEM2MEM_BIT_OFFSET, MEM2MEM_BIT_WIDTH, value as u32); }
}
/// DMA channel 7 number of data register
/// Size: 0x20 bits
pub mod cndtr7 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x84;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const NDT_BIT_OFFSET: u8 = 0;
	const NDT_BIT_WIDTH: u8 = 16;
	/// Number of data to transfer (Width: 16, Offset: 0)
	pub fn get_ndt() -> u16 { ::read(REGISTER_ADDRESS, NDT_BIT_OFFSET, NDT_BIT_WIDTH) as u16 }
	/// Number of data to transfer (Width: 16, Offset: 0)
	pub fn set_ndt(value: u16) { ::write(REGISTER_ADDRESS, NDT_BIT_OFFSET, NDT_BIT_WIDTH, value as u32); }
}
/// DMA channel 7 peripheral address register
/// Size: 0x20 bits
pub mod cpar7 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x88;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PA_BIT_OFFSET: u8 = 0;
	const PA_BIT_WIDTH: u8 = 32;
	/// Peripheral address (Width: 32, Offset: 0)
	pub fn get_pa() -> u32 { ::read(REGISTER_ADDRESS, PA_BIT_OFFSET, PA_BIT_WIDTH) as u32 }
	/// Peripheral address (Width: 32, Offset: 0)
	pub fn set_pa(value: u32) { ::write(REGISTER_ADDRESS, PA_BIT_OFFSET, PA_BIT_WIDTH, value as u32); }
}
/// DMA channel 7 memory address register
/// Size: 0x20 bits
pub mod cmar7 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MA_BIT_OFFSET: u8 = 0;
	const MA_BIT_WIDTH: u8 = 32;
	/// Memory address (Width: 32, Offset: 0)
	pub fn get_ma() -> u32 { ::read(REGISTER_ADDRESS, MA_BIT_OFFSET, MA_BIT_WIDTH) as u32 }
	/// Memory address (Width: 32, Offset: 0)
	pub fn set_ma(value: u32) { ::write(REGISTER_ADDRESS, MA_BIT_OFFSET, MA_BIT_WIDTH, value as u32); }
}
/// DMA1 channel 1 interrupt
pub const INTERRUPT_DMA1_CH1: u32 = 11;

/// DMA1 channel 2 interrupt
pub const INTERRUPT_DMA1_CH2: u32 = 12;

/// DMA1 channel 3 interrupt
pub const INTERRUPT_DMA1_CH3: u32 = 13;

/// DMA1 channel 4 interrupt
pub const INTERRUPT_DMA1_CH4: u32 = 14;

/// DMA1 channel 5 interrupt
pub const INTERRUPT_DMA1_CH5: u32 = 15;

/// DMA1 channel 6 interrupt
pub const INTERRUPT_DMA1_CH6: u32 = 16;

/// DMA1 channel 7interrupt
pub const INTERRUPT_DMA1_CH7: u32 = 17;

/// DMA2 channel1 global interrupt
pub const INTERRUPT_DMA2_CH1: u32 = 56;

/// DMA2 channel2 global interrupt
pub const INTERRUPT_DMA2_CH2: u32 = 57;

/// DMA2 channel3 global interrupt
pub const INTERRUPT_DMA2_CH3: u32 = 58;

/// DMA2 channel4 global interrupt
pub const INTERRUPT_DMA2_CH4: u32 = 59;

/// DMA2 channel5 global interrupt
pub const INTERRUPT_DMA2_CH5: u32 = 60;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" derivedFrom="DMA1">
  <name>DMA2</name>
  <description>DMA controller 1</description>
  <groupName>DMA</groupName>
  <baseAddress>0x40020400</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>ISR</name>
      <displayName>ISR</displayName>
      <description>DMA interrupt status register
          (DMA_ISR)</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>GIF1</name>
          <description>Channel 1 Global interrupt
              flag</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIF1</name>
          <description>Channel 1 Transfer Complete
              flag</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HTIF1</name>
          <description>Channel 1 Half Transfer Complete
              flag</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEIF1</name>
          <description>Channel 1 Transfer Error
              flag</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>GIF2</name>
          <description>Channel 2 Global interrupt
              flag</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIF2</name>
          <description>Channel 2 Transfer Complete
              flag</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HTIF2</name>
          <description>Channel 2 Half Transfer Complete
              flag</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEIF2</name>
          <description>Channel 2 Transfer Error
              flag</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>GIF3</name>
          <description>Channel 3 Global interrupt
              flag</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIF3</name>
          <description>Channel 3 Transfer Complete
              flag</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HTIF3</name>
          <description>Channel 3 Half Transfer Complete
              flag</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEIF3</name>
          <description>Channel 3 Transfer Error
              flag</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>GIF4</name>
          <description>Channel 4 Global interrupt
              flag</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIF4</name>
          <description>Channel 4 Transfer Complete
              flag</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HTIF4</name>
          <description>Channel 4 Half Transfer Complete
              flag</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEIF4</name>
          <description>Channel 4 Transfer Error
              flag</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>GIF5</name>
          <description>Channel 5 Global interrupt
              flag</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIF5</name>
          <description>Channel 5 Transfer Complete
              flag</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HTIF5</name>
          <description>Channel 5 Half Transfer Complete
              flag</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEIF5</name>
          <description>Channel 5 Transfer Error
              flag</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>GIF6</name>
          <description>Channel 6 Global interrupt
              flag</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIF6</name>
          <description>Channel 6 Transfer Complete
              flag</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HTIF6</name>
          <description>Channel 6 Half Transfer Complete
              flag</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEIF6</name>
          <description>Channel 6 Transfer Error
              flag</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>GIF7</name>
          <description>Channel 7 Global interrupt
              flag</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIF7</name>
          <description>Channel 7 Transfer Complete
              flag</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HTIF7</name>
          <description>Channel 7 Half Transfer Complete
              flag</description>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEIF7</name>
          <description>Channel 7 Transfer Error
              flag</description>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IFCR</name>
      <displayName>IFCR</displayName>
      <description>DMA interrupt flag clear register
          (DMA_IFCR)</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>write-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CGIF1</name>
          <description>Channel 1 Global interrupt
              clear</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTCIF1</name>
          <description>Channel 1 Transfer Complete
              clear</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CHTIF1</name>
          <description>Channel 1 Half Transfer
              clear</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTEIF1</name>
          <description>Channel 1 Transfer Error
              clear</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CGIF2</name>
          <description>Channel 2 Global interrupt
              clear</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTCIF2</name>
          <description>Channel 2 Transfer Complete
              clear</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CHTIF2</name>
          <description>Channel 2 Half Transfer
              clear</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTEIF2</name>
          <description>Channel 2 Transfer Error
              clear</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CGIF3</name>
          <description>Channel 3 Global interrupt
              clear</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTCIF3</name>
          <description>Channel 3 Transfer Complete
              clear</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CHTIF3</name>
          <description>Channel 3 Half Transfer
              clear</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTEIF3</name>
          <description>Channel 3 Transfer Error
              clear</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CGIF4</name>
          <description>Channel 4 Global interrupt
              clear</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTCIF4</name>
          <description>Channel 4 Transfer Complete
              clear</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CHTIF4</name>
          <description>Channel 4 Half Transfer
              clear</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTEIF4</name>
          <description>Channel 4 Transfer Error
              clear</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CGIF5</name>
          <description>Channel 5 Global interrupt
              clear</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTCIF5</name>
          <description>Channel 5 Transfer Complete
              clear</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CHTIF5</name>
          <description>Channel 5 Half Transfer
              clear</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTEIF5</name>
          <description>Channel 5 Transfer Error
              clear</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CGIF6</name>
          <description>Channel 6 Global interrupt
              clear</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTCIF6</name>
          <description>Channel 6 Transfer Complete
              clear</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CHTIF6</name>
          <description>Channel 6 Half Transfer
              clear</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTEIF6</name>
          <description>Channel 6 Transfer Error
              clear</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CGIF7</name>
          <description>Channel 7 Global interrupt
              clear</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTCIF7</name>
          <description>Channel 7 Transfer Complete
              clear</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CHTIF7</name>
          <description>Channel 7 Half Transfer
              clear</description>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTEIF7</name>
          <description>Channel 7 Transfer Error
              clear</description>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCR1</name>
      <displayName>CCR1</displayName>
      <description>DMA channel configuration register
          (DMA_CCR)</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EN</name>
          <description>Channel enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIE</name>
          <description>Transfer complete interrupt
              enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HTIE</name>
          <description>Half Transfer interrupt
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEIE</name>
          <description>Transfer error interrupt
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DIR</name>
          <description>Data transfer direction</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CIRC</name>
          <description>Circular mode</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PINC</name>
          <description>Peripheral increment mode</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MINC</name>
          <description>Memory increment mode</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PSIZE</name>
          <description>Peripheral size</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MSIZE</name>
          <description>Memory size</description>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PL</name>
          <description>Channel Priority level</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MEM2MEM</name>
          <description>Memory to memory mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CNDTR1</name>
      <displayName>CNDTR1</displayName>
      <description>DMA channel 1 number of data
          register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>NDT</name>
          <description>Number of data to transfer</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CPAR1</name>
      <displayName>CPAR1</displayName>
      <description>DMA channel 1 peripheral address
          register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PA</name>
          <description>Peripheral address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CMAR1</name>
      <displayName>CMAR1</displayName>
      <description>DMA channel 1 memory address
          register</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MA</name>
          <description>Memory address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCR2</name>
      <displayName>CCR2</displayName>
      <description>DMA channel configuration register
          (DMA_CCR)</description>
      <addressOffset>0x1C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EN</name>
          <description>Channel enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIE</name>
          <description>Transfer complete interrupt
              enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HTIE</name>
          <description>Half Transfer interrupt
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEIE</name>
          <description>Transfer error interrupt
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DIR</name>
          <description>Data transfer direction</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CIRC</name>
          <description>Circular mode</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PINC</name>
          <description>Peripheral increment mode</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MINC</name>
          <description>Memory increment mode</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PSIZE</name>
          <description>Peripheral size</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MSIZE</name>
          <description>Memory size</description>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PL</name>
          <description>Channel Priority level</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MEM2MEM</name>
          <description>Memory to memory mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CNDTR2</name>
      <displayName>CNDTR2</displayName>
      <description>DMA channel 2 number of data
          register</description>
      <addressOffset>0x20</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>NDT</name>
          <description>Number of data to transfer</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CPAR2</name>
      <displayName>CPAR2</displayName>
      <description>DMA channel 2 peripheral address
          register</description>
      <addressOffset>0x24</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PA</name>
          <description>Peripheral address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CMAR2</name>
      <displayName>CMAR2</displayName>
      <description>DMA channel 2 memory address
          register</description>
      <addressOffset>0x28</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MA</name>
          <description>Memory address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCR3</name>
      <displayName>CCR3</displayName>
      <description>DMA channel configuration register
          (DMA_CCR)</description>
      <addressOffset>0x30</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EN</name>
          <description>Channel enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIE</name>
          <description>Transfer complete interrupt
              enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HTIE</name>
          <description>Half Transfer interrupt
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEIE</name>
          <description>Transfer error interrupt
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DIR</name>
          <description>Data transfer direction</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CIRC</name>
          <description>Circular mode</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PINC</name>
          <description>Peripheral increment mode</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MINC</name>
          <description>Memory increment mode</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PSIZE</name>
          <description>Peripheral size</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MSIZE</name>
          <description>Memory size</description>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PL</name>
          <description>Channel Priority level</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MEM2MEM</name>
          <description>Memory to memory mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CNDTR3</name>
      <displayName>CNDTR3</displayName>
      <description>DMA channel 3 number of data
          register</description>
      <addressOffset>0x34</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>NDT</name>
          <description>Number of data to transfer</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CPAR3</name>
      <displayName>CPAR3</displayName>
      <description>DMA channel 3 peripheral address
          register</description>
      <addressOffset>0x38</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PA</name>
          <description>Peripheral address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CMAR3</name>
      <displayName>CMAR3</displayName>
      <description>DMA channel 3 memory address
          register</description>
      <addressOffset>0x3C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MA</name>
          <description>Memory address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCR4</name>
      <displayName>CCR4</displayName>
      <description>DMA channel configuration register
          (DMA_CCR)</description>
      <addressOffset>0x44</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EN</name>
          <description>Channel enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIE</name>
          <description>Transfer complete interrupt
              enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HTIE</name>
          <description>Half Transfer interrupt
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEIE</name>
          <description>Transfer error interrupt
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DIR</name>
          <description>Data transfer direction</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CIRC</name>
          <description>Circular mode</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PINC</name>
          <description>Peripheral increment mode</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MINC</name>
          <description>Memory increment mode</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PSIZE</name>
          <description>Peripheral size</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MSIZE</name>
          <description>Memory size</description>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PL</name>
          <description>Channel Priority level</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MEM2MEM</name>
          <description>Memory to memory mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CNDTR4</name>
      <displayName>CNDTR4</displayName>
      <description>DMA channel 4 number of data
          register</description>
      <addressOffset>0x48</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>NDT</name>
          <description>Number of data to transfer</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CPAR4</name>
      <displayName>CPAR4</displayName>
      <description>DMA channel 4 peripheral address
          register</description>
      <addressOffset>0x4C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PA</name>
          <description>Peripheral address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CMAR4</name>
      <displayName>CMAR4</displayName>
      <description>DMA channel 4 memory address
          register</description>
      <addressOffset>0x50</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MA</name>
          <description>Memory address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCR5</name>
      <displayName>CCR5</displayName>
      <description>DMA channel configuration register
          (DMA_CCR)</description>
      <addressOffset>0x58</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EN</name>
          <description>Channel enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIE</name>
          <description>Transfer complete interrupt
              enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HTIE</name>
          <description>Half Transfer interrupt
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEIE</name>
          <description>Transfer error interrupt
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DIR</name>
          <description>Data transfer direction</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CIRC</name>
          <description>Circular mode</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PINC</name>
          <description>Peripheral increment mode</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MINC</name>
          <description>Memory increment mode</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PSIZE</name>
          <description>Peripheral size</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MSIZE</name>
          <description>Memory size</description>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PL</name>
          <description>Channel Priority level</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MEM2MEM</name>
          <description>Memory to memory mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CNDTR5</name>
      <displayName>CNDTR5</displayName>
      <description>DMA channel 5 number of data
          register</description>
      <addressOffset>0x5C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>NDT</name>
          <description>Number of data to transfer</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CPAR5</name>
      <displayName>CPAR5</displayName>
      <description>DMA channel 5 peripheral address
          register</description>
      <addressOffset>0x60</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PA</name>
          <description>Peripheral address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CMAR5</name>
      <displayName>CMAR5</displayName>
      <description>DMA channel 5 memory address
          register</description>
      <addressOffset>0x64</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MA</name>
          <description>Memory address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCR6</name>
      <displayName>CCR6</displayName>
      <description>DMA channel configuration register
          (DMA_CCR)</description>
      <addressOffset>0x6C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EN</name>
          <description>Channel enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIE</name>
          <description>Transfer complete interrupt
              enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HTIE</name>
          <description>Half Transfer interrupt
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEIE</name>
          <description>Transfer error interrupt
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DIR</name>
          <description>Data transfer direction</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CIRC</name>
          <description>Circular mode</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PINC</name>
          <description>Peripheral increment mode</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MINC</name>
          <description>Memory increment mode</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PSIZE</name>
          <description>Peripheral size</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MSIZE</name>
          <description>Memory size</description>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PL</name>
          <description>Channel Priority level</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MEM2MEM</name>
          <description>Memory to memory mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CNDTR6</name>
      <displayName>CNDTR6</displayName>
      <description>DMA channel 6 number of data
          register</description>
      <addressOffset>0x70</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>NDT</name>
          <description>Number of data to transfer</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CPAR6</name>
      <displayName>CPAR6</displayName>
      <description>DMA channel 6 peripheral address
          register</description>
      <addressOffset>0x74</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PA</name>
          <description>Peripheral address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CMAR6</name>
      <displayName>CMAR6</displayName>
      <description>DMA channel 6 memory address
          register</description>
      <addressOffset>0x78</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MA</name>
          <description>Memory address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCR7</name>
      <displayName>CCR7</displayName>
      <description>DMA channel configuration register
          (DMA_CCR)</description>
      <addressOffset>0x80</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EN</name>
          <description>Channel enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIE</name>
          <description>Transfer complete interrupt
              enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HTIE</name>
          <description>Half Transfer interrupt
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEIE</name>
          <description>Transfer error interrupt
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DIR</name>
          <description>Data transfer direction</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CIRC</name>
          <description>Circular mode</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PINC</name>
          <description>Peripheral increment mode</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MINC</name>
          <description>Memory increment mode</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PSIZE</name>
          <description>Peripheral size</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MSIZE</name>
          <description>Memory size</description>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PL</name>
          <description>Channel Priority level</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>MEM2MEM</name>
          <description>Memory to memory mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CNDTR7</name>
      <displayName>CNDTR7</displayName>
      <description>DMA channel 7 number of data
          register</description>
      <addressOffset>0x84</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>NDT</name>
          <description>Number of data to transfer</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CPAR7</name>
      <displayName>CPAR7</displayName>
      <description>DMA channel 7 peripheral address
          register</description>
      <addressOffset>0x88</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PA</name>
          <description>Peripheral address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CMAR7</name>
      <displayName>CMAR7</displayName>
      <description>DMA channel 7 memory address
          register</description>
      <addressOffset>0x8C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MA</name>
          <description>Memory address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>DMA1_CH1</name>
    <description>DMA1 channel 1 interrupt</description>
    <value>11</value>
  </interrupt>
  <interrupt>
    <name>DMA1_CH2</name>
    <description>DMA1 channel 2 interrupt</description>
    <value>12</value>
  </interrupt>
  <interrupt>
    <name>DMA1_CH3</name>
    <description>DMA1 channel 3 interrupt</description>
    <value>13</value>
  </interrupt>
  <interrupt>
    <name>DMA1_CH4</name>
    <description>DMA1 channel 4 interrupt</description>
    <value>14</value>
  </interrupt>
  <interrupt>
    <name>DMA1_CH5</name>
    <description>DMA1 channel 5 interrupt</description>
    <value>15</value>
  </interrupt>
  <interrupt>
    <name>DMA1_CH6</name>
    <description>DMA1 channel 6 interrupt</description>
    <value>16</value>
  </interrupt>
  <interrupt>
    <name>DMA1_CH7</name>
    <description>DMA1 channel 7interrupt</description>
    <value>17</value>
  </interrupt>
  <interrupt>
    <name>DMA2_CH1</name>
    <description>DMA2 channel1 global interrupt</description>
    <value>56</value>
  </interrupt>
  <interrupt>
    <name>DMA2_CH2</name>
    <description>DMA2 channel2 global interrupt</description>
    <value>57</value>
  </interrupt>
  <interrupt>
    <name>DMA2_CH3</name>
    <description>DMA2 channel3 global interrupt</description>
    <value>58</value>
  </interrupt>
  <interrupt>
    <name>DMA2_CH4</name>
    <description>DMA2 channel4 global interrupt</description>
    <value>59</value>
  </interrupt>
  <interrupt>
    <name>DMA2_CH5</name>
    <description>DMA2 channel5 global interrupt</description>
    <value>60</value>
  </interrupt>
</peripheral>*/
