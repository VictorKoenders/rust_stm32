/// MOD TSC
/// Touch sensing controller
const BASE_ADDRESS: u32 = 0x40024000;
/// control register
/// Size: 0x20 bits
pub mod cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CTPH_BIT_OFFSET: u8 = 28;
	const CTPH_BIT_WIDTH: u8 = 4;
	/// Charge transfer pulse high (Width: 4, Offset: 28)
	pub fn get_ctph() -> u8 { ::read(REGISTER_ADDRESS, CTPH_BIT_OFFSET, CTPH_BIT_WIDTH) as u8 }
	/// Charge transfer pulse high (Width: 4, Offset: 28)
	pub fn set_ctph(value: u8) { ::write(REGISTER_ADDRESS, CTPH_BIT_OFFSET, CTPH_BIT_WIDTH, value as u32); }

	const CTPL_BIT_OFFSET: u8 = 24;
	const CTPL_BIT_WIDTH: u8 = 4;
	/// Charge transfer pulse low (Width: 4, Offset: 24)
	pub fn get_ctpl() -> u8 { ::read(REGISTER_ADDRESS, CTPL_BIT_OFFSET, CTPL_BIT_WIDTH) as u8 }
	/// Charge transfer pulse low (Width: 4, Offset: 24)
	pub fn set_ctpl(value: u8) { ::write(REGISTER_ADDRESS, CTPL_BIT_OFFSET, CTPL_BIT_WIDTH, value as u32); }

	const SSD_BIT_OFFSET: u8 = 17;
	const SSD_BIT_WIDTH: u8 = 7;
	/// Spread spectrum deviation (Width: 7, Offset: 17)
	pub fn get_ssd() -> u8 { ::read(REGISTER_ADDRESS, SSD_BIT_OFFSET, SSD_BIT_WIDTH) as u8 }
	/// Spread spectrum deviation (Width: 7, Offset: 17)
	pub fn set_ssd(value: u8) { ::write(REGISTER_ADDRESS, SSD_BIT_OFFSET, SSD_BIT_WIDTH, value as u32); }

	const SSE_BIT_OFFSET: u8 = 16;
	const SSE_BIT_WIDTH: u8 = 1;
	/// Spread spectrum enable (Width: 1, Offset: 16)
	pub fn get_sse() -> u8 { ::read(REGISTER_ADDRESS, SSE_BIT_OFFSET, SSE_BIT_WIDTH) as u8 }
	/// Spread spectrum enable (Width: 1, Offset: 16)
	pub fn set_sse(value: u8) { ::write(REGISTER_ADDRESS, SSE_BIT_OFFSET, SSE_BIT_WIDTH, value as u32); }

	const SSPSC_BIT_OFFSET: u8 = 15;
	const SSPSC_BIT_WIDTH: u8 = 1;
	/// Spread spectrum prescaler (Width: 1, Offset: 15)
	pub fn get_sspsc() -> u8 { ::read(REGISTER_ADDRESS, SSPSC_BIT_OFFSET, SSPSC_BIT_WIDTH) as u8 }
	/// Spread spectrum prescaler (Width: 1, Offset: 15)
	pub fn set_sspsc(value: u8) { ::write(REGISTER_ADDRESS, SSPSC_BIT_OFFSET, SSPSC_BIT_WIDTH, value as u32); }

	const PGPSC_BIT_OFFSET: u8 = 12;
	const PGPSC_BIT_WIDTH: u8 = 3;
	/// pulse generator prescaler (Width: 3, Offset: 12)
	pub fn get_pgpsc() -> u8 { ::read(REGISTER_ADDRESS, PGPSC_BIT_OFFSET, PGPSC_BIT_WIDTH) as u8 }
	/// pulse generator prescaler (Width: 3, Offset: 12)
	pub fn set_pgpsc(value: u8) { ::write(REGISTER_ADDRESS, PGPSC_BIT_OFFSET, PGPSC_BIT_WIDTH, value as u32); }

	const MCV_BIT_OFFSET: u8 = 5;
	const MCV_BIT_WIDTH: u8 = 3;
	/// Max count value (Width: 3, Offset: 5)
	pub fn get_mcv() -> u8 { ::read(REGISTER_ADDRESS, MCV_BIT_OFFSET, MCV_BIT_WIDTH) as u8 }
	/// Max count value (Width: 3, Offset: 5)
	pub fn set_mcv(value: u8) { ::write(REGISTER_ADDRESS, MCV_BIT_OFFSET, MCV_BIT_WIDTH, value as u32); }

	const IODEF_BIT_OFFSET: u8 = 4;
	const IODEF_BIT_WIDTH: u8 = 1;
	/// I/O Default mode (Width: 1, Offset: 4)
	pub fn get_iodef() -> u8 { ::read(REGISTER_ADDRESS, IODEF_BIT_OFFSET, IODEF_BIT_WIDTH) as u8 }
	/// I/O Default mode (Width: 1, Offset: 4)
	pub fn set_iodef(value: u8) { ::write(REGISTER_ADDRESS, IODEF_BIT_OFFSET, IODEF_BIT_WIDTH, value as u32); }

	const SYNCPOL_BIT_OFFSET: u8 = 3;
	const SYNCPOL_BIT_WIDTH: u8 = 1;
	/// Synchronization pin polarity (Width: 1, Offset: 3)
	pub fn get_syncpol() -> u8 { ::read(REGISTER_ADDRESS, SYNCPOL_BIT_OFFSET, SYNCPOL_BIT_WIDTH) as u8 }
	/// Synchronization pin polarity (Width: 1, Offset: 3)
	pub fn set_syncpol(value: u8) { ::write(REGISTER_ADDRESS, SYNCPOL_BIT_OFFSET, SYNCPOL_BIT_WIDTH, value as u32); }

	const AM_BIT_OFFSET: u8 = 2;
	const AM_BIT_WIDTH: u8 = 1;
	/// Acquisition mode (Width: 1, Offset: 2)
	pub fn get_am() -> u8 { ::read(REGISTER_ADDRESS, AM_BIT_OFFSET, AM_BIT_WIDTH) as u8 }
	/// Acquisition mode (Width: 1, Offset: 2)
	pub fn set_am(value: u8) { ::write(REGISTER_ADDRESS, AM_BIT_OFFSET, AM_BIT_WIDTH, value as u32); }

	const START_BIT_OFFSET: u8 = 1;
	const START_BIT_WIDTH: u8 = 1;
	/// Start a new acquisition (Width: 1, Offset: 1)
	pub fn get_start() -> u8 { ::read(REGISTER_ADDRESS, START_BIT_OFFSET, START_BIT_WIDTH) as u8 }
	/// Start a new acquisition (Width: 1, Offset: 1)
	pub fn set_start(value: u8) { ::write(REGISTER_ADDRESS, START_BIT_OFFSET, START_BIT_WIDTH, value as u32); }

	const TSCE_BIT_OFFSET: u8 = 0;
	const TSCE_BIT_WIDTH: u8 = 1;
	/// Touch sensing controller enable (Width: 1, Offset: 0)
	pub fn get_tsce() -> u8 { ::read(REGISTER_ADDRESS, TSCE_BIT_OFFSET, TSCE_BIT_WIDTH) as u8 }
	/// Touch sensing controller enable (Width: 1, Offset: 0)
	pub fn set_tsce(value: u8) { ::write(REGISTER_ADDRESS, TSCE_BIT_OFFSET, TSCE_BIT_WIDTH, value as u32); }
}
/// interrupt enable register
/// Size: 0x20 bits
pub mod ier {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MCEIE_BIT_OFFSET: u8 = 1;
	const MCEIE_BIT_WIDTH: u8 = 1;
	/// Max count error interrupt enable (Width: 1, Offset: 1)
	pub fn get_mceie() -> u8 { ::read(REGISTER_ADDRESS, MCEIE_BIT_OFFSET, MCEIE_BIT_WIDTH) as u8 }
	/// Max count error interrupt enable (Width: 1, Offset: 1)
	pub fn set_mceie(value: u8) { ::write(REGISTER_ADDRESS, MCEIE_BIT_OFFSET, MCEIE_BIT_WIDTH, value as u32); }

	const EOAIE_BIT_OFFSET: u8 = 0;
	const EOAIE_BIT_WIDTH: u8 = 1;
	/// End of acquisition interrupt enable (Width: 1, Offset: 0)
	pub fn get_eoaie() -> u8 { ::read(REGISTER_ADDRESS, EOAIE_BIT_OFFSET, EOAIE_BIT_WIDTH) as u8 }
	/// End of acquisition interrupt enable (Width: 1, Offset: 0)
	pub fn set_eoaie(value: u8) { ::write(REGISTER_ADDRESS, EOAIE_BIT_OFFSET, EOAIE_BIT_WIDTH, value as u32); }
}
/// interrupt clear register
/// Size: 0x20 bits
pub mod icr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MCEIC_BIT_OFFSET: u8 = 1;
	const MCEIC_BIT_WIDTH: u8 = 1;
	/// Max count error interrupt clear (Width: 1, Offset: 1)
	pub fn get_mceic() -> u8 { ::read(REGISTER_ADDRESS, MCEIC_BIT_OFFSET, MCEIC_BIT_WIDTH) as u8 }
	/// Max count error interrupt clear (Width: 1, Offset: 1)
	pub fn set_mceic(value: u8) { ::write(REGISTER_ADDRESS, MCEIC_BIT_OFFSET, MCEIC_BIT_WIDTH, value as u32); }

	const EOAIC_BIT_OFFSET: u8 = 0;
	const EOAIC_BIT_WIDTH: u8 = 1;
	/// End of acquisition interrupt clear (Width: 1, Offset: 0)
	pub fn get_eoaic() -> u8 { ::read(REGISTER_ADDRESS, EOAIC_BIT_OFFSET, EOAIC_BIT_WIDTH) as u8 }
	/// End of acquisition interrupt clear (Width: 1, Offset: 0)
	pub fn set_eoaic(value: u8) { ::write(REGISTER_ADDRESS, EOAIC_BIT_OFFSET, EOAIC_BIT_WIDTH, value as u32); }
}
/// interrupt status register
/// Size: 0x20 bits
pub mod isr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MCEF_BIT_OFFSET: u8 = 1;
	const MCEF_BIT_WIDTH: u8 = 1;
	/// Max count error flag (Width: 1, Offset: 1)
	pub fn get_mcef() -> u8 { ::read(REGISTER_ADDRESS, MCEF_BIT_OFFSET, MCEF_BIT_WIDTH) as u8 }
	/// Max count error flag (Width: 1, Offset: 1)
	pub fn set_mcef(value: u8) { ::write(REGISTER_ADDRESS, MCEF_BIT_OFFSET, MCEF_BIT_WIDTH, value as u32); }

	const EOAF_BIT_OFFSET: u8 = 0;
	const EOAF_BIT_WIDTH: u8 = 1;
	/// End of acquisition flag (Width: 1, Offset: 0)
	pub fn get_eoaf() -> u8 { ::read(REGISTER_ADDRESS, EOAF_BIT_OFFSET, EOAF_BIT_WIDTH) as u8 }
	/// End of acquisition flag (Width: 1, Offset: 0)
	pub fn set_eoaf(value: u8) { ::write(REGISTER_ADDRESS, EOAF_BIT_OFFSET, EOAF_BIT_WIDTH, value as u32); }
}
/// I/O hysteresis control register
/// Size: 0x20 bits
pub mod iohcr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const G1_IO1_BIT_OFFSET: u8 = 0;
	const G1_IO1_BIT_WIDTH: u8 = 1;
	/// G1_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 0)
	pub fn get_g1_io1() -> u8 { ::read(REGISTER_ADDRESS, G1_IO1_BIT_OFFSET, G1_IO1_BIT_WIDTH) as u8 }
	/// G1_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 0)
	pub fn set_g1_io1(value: u8) { ::write(REGISTER_ADDRESS, G1_IO1_BIT_OFFSET, G1_IO1_BIT_WIDTH, value as u32); }

	const G1_IO2_BIT_OFFSET: u8 = 1;
	const G1_IO2_BIT_WIDTH: u8 = 1;
	/// G1_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 1)
	pub fn get_g1_io2() -> u8 { ::read(REGISTER_ADDRESS, G1_IO2_BIT_OFFSET, G1_IO2_BIT_WIDTH) as u8 }
	/// G1_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 1)
	pub fn set_g1_io2(value: u8) { ::write(REGISTER_ADDRESS, G1_IO2_BIT_OFFSET, G1_IO2_BIT_WIDTH, value as u32); }

	const G1_IO3_BIT_OFFSET: u8 = 2;
	const G1_IO3_BIT_WIDTH: u8 = 1;
	/// G1_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 2)
	pub fn get_g1_io3() -> u8 { ::read(REGISTER_ADDRESS, G1_IO3_BIT_OFFSET, G1_IO3_BIT_WIDTH) as u8 }
	/// G1_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 2)
	pub fn set_g1_io3(value: u8) { ::write(REGISTER_ADDRESS, G1_IO3_BIT_OFFSET, G1_IO3_BIT_WIDTH, value as u32); }

	const G1_IO4_BIT_OFFSET: u8 = 3;
	const G1_IO4_BIT_WIDTH: u8 = 1;
	/// G1_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 3)
	pub fn get_g1_io4() -> u8 { ::read(REGISTER_ADDRESS, G1_IO4_BIT_OFFSET, G1_IO4_BIT_WIDTH) as u8 }
	/// G1_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 3)
	pub fn set_g1_io4(value: u8) { ::write(REGISTER_ADDRESS, G1_IO4_BIT_OFFSET, G1_IO4_BIT_WIDTH, value as u32); }

	const G2_IO1_BIT_OFFSET: u8 = 4;
	const G2_IO1_BIT_WIDTH: u8 = 1;
	/// G2_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 4)
	pub fn get_g2_io1() -> u8 { ::read(REGISTER_ADDRESS, G2_IO1_BIT_OFFSET, G2_IO1_BIT_WIDTH) as u8 }
	/// G2_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 4)
	pub fn set_g2_io1(value: u8) { ::write(REGISTER_ADDRESS, G2_IO1_BIT_OFFSET, G2_IO1_BIT_WIDTH, value as u32); }

	const G2_IO2_BIT_OFFSET: u8 = 5;
	const G2_IO2_BIT_WIDTH: u8 = 1;
	/// G2_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 5)
	pub fn get_g2_io2() -> u8 { ::read(REGISTER_ADDRESS, G2_IO2_BIT_OFFSET, G2_IO2_BIT_WIDTH) as u8 }
	/// G2_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 5)
	pub fn set_g2_io2(value: u8) { ::write(REGISTER_ADDRESS, G2_IO2_BIT_OFFSET, G2_IO2_BIT_WIDTH, value as u32); }

	const G2_IO3_BIT_OFFSET: u8 = 6;
	const G2_IO3_BIT_WIDTH: u8 = 1;
	/// G2_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 6)
	pub fn get_g2_io3() -> u8 { ::read(REGISTER_ADDRESS, G2_IO3_BIT_OFFSET, G2_IO3_BIT_WIDTH) as u8 }
	/// G2_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 6)
	pub fn set_g2_io3(value: u8) { ::write(REGISTER_ADDRESS, G2_IO3_BIT_OFFSET, G2_IO3_BIT_WIDTH, value as u32); }

	const G2_IO4_BIT_OFFSET: u8 = 7;
	const G2_IO4_BIT_WIDTH: u8 = 1;
	/// G2_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 7)
	pub fn get_g2_io4() -> u8 { ::read(REGISTER_ADDRESS, G2_IO4_BIT_OFFSET, G2_IO4_BIT_WIDTH) as u8 }
	/// G2_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 7)
	pub fn set_g2_io4(value: u8) { ::write(REGISTER_ADDRESS, G2_IO4_BIT_OFFSET, G2_IO4_BIT_WIDTH, value as u32); }

	const G3_IO1_BIT_OFFSET: u8 = 8;
	const G3_IO1_BIT_WIDTH: u8 = 1;
	/// G3_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 8)
	pub fn get_g3_io1() -> u8 { ::read(REGISTER_ADDRESS, G3_IO1_BIT_OFFSET, G3_IO1_BIT_WIDTH) as u8 }
	/// G3_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 8)
	pub fn set_g3_io1(value: u8) { ::write(REGISTER_ADDRESS, G3_IO1_BIT_OFFSET, G3_IO1_BIT_WIDTH, value as u32); }

	const G3_IO2_BIT_OFFSET: u8 = 9;
	const G3_IO2_BIT_WIDTH: u8 = 1;
	/// G3_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 9)
	pub fn get_g3_io2() -> u8 { ::read(REGISTER_ADDRESS, G3_IO2_BIT_OFFSET, G3_IO2_BIT_WIDTH) as u8 }
	/// G3_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 9)
	pub fn set_g3_io2(value: u8) { ::write(REGISTER_ADDRESS, G3_IO2_BIT_OFFSET, G3_IO2_BIT_WIDTH, value as u32); }

	const G3_IO3_BIT_OFFSET: u8 = 10;
	const G3_IO3_BIT_WIDTH: u8 = 1;
	/// G3_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 10)
	pub fn get_g3_io3() -> u8 { ::read(REGISTER_ADDRESS, G3_IO3_BIT_OFFSET, G3_IO3_BIT_WIDTH) as u8 }
	/// G3_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 10)
	pub fn set_g3_io3(value: u8) { ::write(REGISTER_ADDRESS, G3_IO3_BIT_OFFSET, G3_IO3_BIT_WIDTH, value as u32); }

	const G3_IO4_BIT_OFFSET: u8 = 11;
	const G3_IO4_BIT_WIDTH: u8 = 1;
	/// G3_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 11)
	pub fn get_g3_io4() -> u8 { ::read(REGISTER_ADDRESS, G3_IO4_BIT_OFFSET, G3_IO4_BIT_WIDTH) as u8 }
	/// G3_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 11)
	pub fn set_g3_io4(value: u8) { ::write(REGISTER_ADDRESS, G3_IO4_BIT_OFFSET, G3_IO4_BIT_WIDTH, value as u32); }

	const G4_IO1_BIT_OFFSET: u8 = 12;
	const G4_IO1_BIT_WIDTH: u8 = 1;
	/// G4_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 12)
	pub fn get_g4_io1() -> u8 { ::read(REGISTER_ADDRESS, G4_IO1_BIT_OFFSET, G4_IO1_BIT_WIDTH) as u8 }
	/// G4_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 12)
	pub fn set_g4_io1(value: u8) { ::write(REGISTER_ADDRESS, G4_IO1_BIT_OFFSET, G4_IO1_BIT_WIDTH, value as u32); }

	const G4_IO2_BIT_OFFSET: u8 = 13;
	const G4_IO2_BIT_WIDTH: u8 = 1;
	/// G4_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 13)
	pub fn get_g4_io2() -> u8 { ::read(REGISTER_ADDRESS, G4_IO2_BIT_OFFSET, G4_IO2_BIT_WIDTH) as u8 }
	/// G4_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 13)
	pub fn set_g4_io2(value: u8) { ::write(REGISTER_ADDRESS, G4_IO2_BIT_OFFSET, G4_IO2_BIT_WIDTH, value as u32); }

	const G4_IO3_BIT_OFFSET: u8 = 14;
	const G4_IO3_BIT_WIDTH: u8 = 1;
	/// G4_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 14)
	pub fn get_g4_io3() -> u8 { ::read(REGISTER_ADDRESS, G4_IO3_BIT_OFFSET, G4_IO3_BIT_WIDTH) as u8 }
	/// G4_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 14)
	pub fn set_g4_io3(value: u8) { ::write(REGISTER_ADDRESS, G4_IO3_BIT_OFFSET, G4_IO3_BIT_WIDTH, value as u32); }

	const G4_IO4_BIT_OFFSET: u8 = 15;
	const G4_IO4_BIT_WIDTH: u8 = 1;
	/// G4_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 15)
	pub fn get_g4_io4() -> u8 { ::read(REGISTER_ADDRESS, G4_IO4_BIT_OFFSET, G4_IO4_BIT_WIDTH) as u8 }
	/// G4_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 15)
	pub fn set_g4_io4(value: u8) { ::write(REGISTER_ADDRESS, G4_IO4_BIT_OFFSET, G4_IO4_BIT_WIDTH, value as u32); }

	const G5_IO1_BIT_OFFSET: u8 = 16;
	const G5_IO1_BIT_WIDTH: u8 = 1;
	/// G5_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 16)
	pub fn get_g5_io1() -> u8 { ::read(REGISTER_ADDRESS, G5_IO1_BIT_OFFSET, G5_IO1_BIT_WIDTH) as u8 }
	/// G5_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 16)
	pub fn set_g5_io1(value: u8) { ::write(REGISTER_ADDRESS, G5_IO1_BIT_OFFSET, G5_IO1_BIT_WIDTH, value as u32); }

	const G5_IO2_BIT_OFFSET: u8 = 17;
	const G5_IO2_BIT_WIDTH: u8 = 1;
	/// G5_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 17)
	pub fn get_g5_io2() -> u8 { ::read(REGISTER_ADDRESS, G5_IO2_BIT_OFFSET, G5_IO2_BIT_WIDTH) as u8 }
	/// G5_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 17)
	pub fn set_g5_io2(value: u8) { ::write(REGISTER_ADDRESS, G5_IO2_BIT_OFFSET, G5_IO2_BIT_WIDTH, value as u32); }

	const G5_IO3_BIT_OFFSET: u8 = 18;
	const G5_IO3_BIT_WIDTH: u8 = 1;
	/// G5_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 18)
	pub fn get_g5_io3() -> u8 { ::read(REGISTER_ADDRESS, G5_IO3_BIT_OFFSET, G5_IO3_BIT_WIDTH) as u8 }
	/// G5_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 18)
	pub fn set_g5_io3(value: u8) { ::write(REGISTER_ADDRESS, G5_IO3_BIT_OFFSET, G5_IO3_BIT_WIDTH, value as u32); }

	const G5_IO4_BIT_OFFSET: u8 = 19;
	const G5_IO4_BIT_WIDTH: u8 = 1;
	/// G5_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 19)
	pub fn get_g5_io4() -> u8 { ::read(REGISTER_ADDRESS, G5_IO4_BIT_OFFSET, G5_IO4_BIT_WIDTH) as u8 }
	/// G5_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 19)
	pub fn set_g5_io4(value: u8) { ::write(REGISTER_ADDRESS, G5_IO4_BIT_OFFSET, G5_IO4_BIT_WIDTH, value as u32); }

	const G6_IO1_BIT_OFFSET: u8 = 20;
	const G6_IO1_BIT_WIDTH: u8 = 1;
	/// G6_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 20)
	pub fn get_g6_io1() -> u8 { ::read(REGISTER_ADDRESS, G6_IO1_BIT_OFFSET, G6_IO1_BIT_WIDTH) as u8 }
	/// G6_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 20)
	pub fn set_g6_io1(value: u8) { ::write(REGISTER_ADDRESS, G6_IO1_BIT_OFFSET, G6_IO1_BIT_WIDTH, value as u32); }

	const G6_IO2_BIT_OFFSET: u8 = 21;
	const G6_IO2_BIT_WIDTH: u8 = 1;
	/// G6_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 21)
	pub fn get_g6_io2() -> u8 { ::read(REGISTER_ADDRESS, G6_IO2_BIT_OFFSET, G6_IO2_BIT_WIDTH) as u8 }
	/// G6_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 21)
	pub fn set_g6_io2(value: u8) { ::write(REGISTER_ADDRESS, G6_IO2_BIT_OFFSET, G6_IO2_BIT_WIDTH, value as u32); }

	const G6_IO3_BIT_OFFSET: u8 = 22;
	const G6_IO3_BIT_WIDTH: u8 = 1;
	/// G6_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 22)
	pub fn get_g6_io3() -> u8 { ::read(REGISTER_ADDRESS, G6_IO3_BIT_OFFSET, G6_IO3_BIT_WIDTH) as u8 }
	/// G6_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 22)
	pub fn set_g6_io3(value: u8) { ::write(REGISTER_ADDRESS, G6_IO3_BIT_OFFSET, G6_IO3_BIT_WIDTH, value as u32); }

	const G6_IO4_BIT_OFFSET: u8 = 23;
	const G6_IO4_BIT_WIDTH: u8 = 1;
	/// G6_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 23)
	pub fn get_g6_io4() -> u8 { ::read(REGISTER_ADDRESS, G6_IO4_BIT_OFFSET, G6_IO4_BIT_WIDTH) as u8 }
	/// G6_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 23)
	pub fn set_g6_io4(value: u8) { ::write(REGISTER_ADDRESS, G6_IO4_BIT_OFFSET, G6_IO4_BIT_WIDTH, value as u32); }

	const G7_IO1_BIT_OFFSET: u8 = 24;
	const G7_IO1_BIT_WIDTH: u8 = 1;
	/// G7_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 24)
	pub fn get_g7_io1() -> u8 { ::read(REGISTER_ADDRESS, G7_IO1_BIT_OFFSET, G7_IO1_BIT_WIDTH) as u8 }
	/// G7_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 24)
	pub fn set_g7_io1(value: u8) { ::write(REGISTER_ADDRESS, G7_IO1_BIT_OFFSET, G7_IO1_BIT_WIDTH, value as u32); }

	const G7_IO2_BIT_OFFSET: u8 = 25;
	const G7_IO2_BIT_WIDTH: u8 = 1;
	/// G7_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 25)
	pub fn get_g7_io2() -> u8 { ::read(REGISTER_ADDRESS, G7_IO2_BIT_OFFSET, G7_IO2_BIT_WIDTH) as u8 }
	/// G7_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 25)
	pub fn set_g7_io2(value: u8) { ::write(REGISTER_ADDRESS, G7_IO2_BIT_OFFSET, G7_IO2_BIT_WIDTH, value as u32); }

	const G7_IO3_BIT_OFFSET: u8 = 26;
	const G7_IO3_BIT_WIDTH: u8 = 1;
	/// G7_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 26)
	pub fn get_g7_io3() -> u8 { ::read(REGISTER_ADDRESS, G7_IO3_BIT_OFFSET, G7_IO3_BIT_WIDTH) as u8 }
	/// G7_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 26)
	pub fn set_g7_io3(value: u8) { ::write(REGISTER_ADDRESS, G7_IO3_BIT_OFFSET, G7_IO3_BIT_WIDTH, value as u32); }

	const G7_IO4_BIT_OFFSET: u8 = 27;
	const G7_IO4_BIT_WIDTH: u8 = 1;
	/// G7_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 27)
	pub fn get_g7_io4() -> u8 { ::read(REGISTER_ADDRESS, G7_IO4_BIT_OFFSET, G7_IO4_BIT_WIDTH) as u8 }
	/// G7_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 27)
	pub fn set_g7_io4(value: u8) { ::write(REGISTER_ADDRESS, G7_IO4_BIT_OFFSET, G7_IO4_BIT_WIDTH, value as u32); }

	const G8_IO1_BIT_OFFSET: u8 = 28;
	const G8_IO1_BIT_WIDTH: u8 = 1;
	/// G8_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 28)
	pub fn get_g8_io1() -> u8 { ::read(REGISTER_ADDRESS, G8_IO1_BIT_OFFSET, G8_IO1_BIT_WIDTH) as u8 }
	/// G8_IO1 Schmitt trigger hysteresis mode (Width: 1, Offset: 28)
	pub fn set_g8_io1(value: u8) { ::write(REGISTER_ADDRESS, G8_IO1_BIT_OFFSET, G8_IO1_BIT_WIDTH, value as u32); }

	const G8_IO2_BIT_OFFSET: u8 = 29;
	const G8_IO2_BIT_WIDTH: u8 = 1;
	/// G8_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 29)
	pub fn get_g8_io2() -> u8 { ::read(REGISTER_ADDRESS, G8_IO2_BIT_OFFSET, G8_IO2_BIT_WIDTH) as u8 }
	/// G8_IO2 Schmitt trigger hysteresis mode (Width: 1, Offset: 29)
	pub fn set_g8_io2(value: u8) { ::write(REGISTER_ADDRESS, G8_IO2_BIT_OFFSET, G8_IO2_BIT_WIDTH, value as u32); }

	const G8_IO3_BIT_OFFSET: u8 = 30;
	const G8_IO3_BIT_WIDTH: u8 = 1;
	/// G8_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 30)
	pub fn get_g8_io3() -> u8 { ::read(REGISTER_ADDRESS, G8_IO3_BIT_OFFSET, G8_IO3_BIT_WIDTH) as u8 }
	/// G8_IO3 Schmitt trigger hysteresis mode (Width: 1, Offset: 30)
	pub fn set_g8_io3(value: u8) { ::write(REGISTER_ADDRESS, G8_IO3_BIT_OFFSET, G8_IO3_BIT_WIDTH, value as u32); }

	const G8_IO4_BIT_OFFSET: u8 = 31;
	const G8_IO4_BIT_WIDTH: u8 = 1;
	/// G8_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 31)
	pub fn get_g8_io4() -> u8 { ::read(REGISTER_ADDRESS, G8_IO4_BIT_OFFSET, G8_IO4_BIT_WIDTH) as u8 }
	/// G8_IO4 Schmitt trigger hysteresis mode (Width: 1, Offset: 31)
	pub fn set_g8_io4(value: u8) { ::write(REGISTER_ADDRESS, G8_IO4_BIT_OFFSET, G8_IO4_BIT_WIDTH, value as u32); }
}
/// I/O analog switch control register
/// Size: 0x20 bits
pub mod ioascr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const G1_IO1_BIT_OFFSET: u8 = 0;
	const G1_IO1_BIT_WIDTH: u8 = 1;
	/// G1_IO1 analog switch enable (Width: 1, Offset: 0)
	pub fn get_g1_io1() -> u8 { ::read(REGISTER_ADDRESS, G1_IO1_BIT_OFFSET, G1_IO1_BIT_WIDTH) as u8 }
	/// G1_IO1 analog switch enable (Width: 1, Offset: 0)
	pub fn set_g1_io1(value: u8) { ::write(REGISTER_ADDRESS, G1_IO1_BIT_OFFSET, G1_IO1_BIT_WIDTH, value as u32); }

	const G1_IO2_BIT_OFFSET: u8 = 1;
	const G1_IO2_BIT_WIDTH: u8 = 1;
	/// G1_IO2 analog switch enable (Width: 1, Offset: 1)
	pub fn get_g1_io2() -> u8 { ::read(REGISTER_ADDRESS, G1_IO2_BIT_OFFSET, G1_IO2_BIT_WIDTH) as u8 }
	/// G1_IO2 analog switch enable (Width: 1, Offset: 1)
	pub fn set_g1_io2(value: u8) { ::write(REGISTER_ADDRESS, G1_IO2_BIT_OFFSET, G1_IO2_BIT_WIDTH, value as u32); }

	const G1_IO3_BIT_OFFSET: u8 = 2;
	const G1_IO3_BIT_WIDTH: u8 = 1;
	/// G1_IO3 analog switch enable (Width: 1, Offset: 2)
	pub fn get_g1_io3() -> u8 { ::read(REGISTER_ADDRESS, G1_IO3_BIT_OFFSET, G1_IO3_BIT_WIDTH) as u8 }
	/// G1_IO3 analog switch enable (Width: 1, Offset: 2)
	pub fn set_g1_io3(value: u8) { ::write(REGISTER_ADDRESS, G1_IO3_BIT_OFFSET, G1_IO3_BIT_WIDTH, value as u32); }

	const G1_IO4_BIT_OFFSET: u8 = 3;
	const G1_IO4_BIT_WIDTH: u8 = 1;
	/// G1_IO4 analog switch enable (Width: 1, Offset: 3)
	pub fn get_g1_io4() -> u8 { ::read(REGISTER_ADDRESS, G1_IO4_BIT_OFFSET, G1_IO4_BIT_WIDTH) as u8 }
	/// G1_IO4 analog switch enable (Width: 1, Offset: 3)
	pub fn set_g1_io4(value: u8) { ::write(REGISTER_ADDRESS, G1_IO4_BIT_OFFSET, G1_IO4_BIT_WIDTH, value as u32); }

	const G2_IO1_BIT_OFFSET: u8 = 4;
	const G2_IO1_BIT_WIDTH: u8 = 1;
	/// G2_IO1 analog switch enable (Width: 1, Offset: 4)
	pub fn get_g2_io1() -> u8 { ::read(REGISTER_ADDRESS, G2_IO1_BIT_OFFSET, G2_IO1_BIT_WIDTH) as u8 }
	/// G2_IO1 analog switch enable (Width: 1, Offset: 4)
	pub fn set_g2_io1(value: u8) { ::write(REGISTER_ADDRESS, G2_IO1_BIT_OFFSET, G2_IO1_BIT_WIDTH, value as u32); }

	const G2_IO2_BIT_OFFSET: u8 = 5;
	const G2_IO2_BIT_WIDTH: u8 = 1;
	/// G2_IO2 analog switch enable (Width: 1, Offset: 5)
	pub fn get_g2_io2() -> u8 { ::read(REGISTER_ADDRESS, G2_IO2_BIT_OFFSET, G2_IO2_BIT_WIDTH) as u8 }
	/// G2_IO2 analog switch enable (Width: 1, Offset: 5)
	pub fn set_g2_io2(value: u8) { ::write(REGISTER_ADDRESS, G2_IO2_BIT_OFFSET, G2_IO2_BIT_WIDTH, value as u32); }

	const G2_IO3_BIT_OFFSET: u8 = 6;
	const G2_IO3_BIT_WIDTH: u8 = 1;
	/// G2_IO3 analog switch enable (Width: 1, Offset: 6)
	pub fn get_g2_io3() -> u8 { ::read(REGISTER_ADDRESS, G2_IO3_BIT_OFFSET, G2_IO3_BIT_WIDTH) as u8 }
	/// G2_IO3 analog switch enable (Width: 1, Offset: 6)
	pub fn set_g2_io3(value: u8) { ::write(REGISTER_ADDRESS, G2_IO3_BIT_OFFSET, G2_IO3_BIT_WIDTH, value as u32); }

	const G2_IO4_BIT_OFFSET: u8 = 7;
	const G2_IO4_BIT_WIDTH: u8 = 1;
	/// G2_IO4 analog switch enable (Width: 1, Offset: 7)
	pub fn get_g2_io4() -> u8 { ::read(REGISTER_ADDRESS, G2_IO4_BIT_OFFSET, G2_IO4_BIT_WIDTH) as u8 }
	/// G2_IO4 analog switch enable (Width: 1, Offset: 7)
	pub fn set_g2_io4(value: u8) { ::write(REGISTER_ADDRESS, G2_IO4_BIT_OFFSET, G2_IO4_BIT_WIDTH, value as u32); }

	const G3_IO1_BIT_OFFSET: u8 = 8;
	const G3_IO1_BIT_WIDTH: u8 = 1;
	/// G3_IO1 analog switch enable (Width: 1, Offset: 8)
	pub fn get_g3_io1() -> u8 { ::read(REGISTER_ADDRESS, G3_IO1_BIT_OFFSET, G3_IO1_BIT_WIDTH) as u8 }
	/// G3_IO1 analog switch enable (Width: 1, Offset: 8)
	pub fn set_g3_io1(value: u8) { ::write(REGISTER_ADDRESS, G3_IO1_BIT_OFFSET, G3_IO1_BIT_WIDTH, value as u32); }

	const G3_IO2_BIT_OFFSET: u8 = 9;
	const G3_IO2_BIT_WIDTH: u8 = 1;
	/// G3_IO2 analog switch enable (Width: 1, Offset: 9)
	pub fn get_g3_io2() -> u8 { ::read(REGISTER_ADDRESS, G3_IO2_BIT_OFFSET, G3_IO2_BIT_WIDTH) as u8 }
	/// G3_IO2 analog switch enable (Width: 1, Offset: 9)
	pub fn set_g3_io2(value: u8) { ::write(REGISTER_ADDRESS, G3_IO2_BIT_OFFSET, G3_IO2_BIT_WIDTH, value as u32); }

	const G3_IO3_BIT_OFFSET: u8 = 10;
	const G3_IO3_BIT_WIDTH: u8 = 1;
	/// G3_IO3 analog switch enable (Width: 1, Offset: 10)
	pub fn get_g3_io3() -> u8 { ::read(REGISTER_ADDRESS, G3_IO3_BIT_OFFSET, G3_IO3_BIT_WIDTH) as u8 }
	/// G3_IO3 analog switch enable (Width: 1, Offset: 10)
	pub fn set_g3_io3(value: u8) { ::write(REGISTER_ADDRESS, G3_IO3_BIT_OFFSET, G3_IO3_BIT_WIDTH, value as u32); }

	const G3_IO4_BIT_OFFSET: u8 = 11;
	const G3_IO4_BIT_WIDTH: u8 = 1;
	/// G3_IO4 analog switch enable (Width: 1, Offset: 11)
	pub fn get_g3_io4() -> u8 { ::read(REGISTER_ADDRESS, G3_IO4_BIT_OFFSET, G3_IO4_BIT_WIDTH) as u8 }
	/// G3_IO4 analog switch enable (Width: 1, Offset: 11)
	pub fn set_g3_io4(value: u8) { ::write(REGISTER_ADDRESS, G3_IO4_BIT_OFFSET, G3_IO4_BIT_WIDTH, value as u32); }

	const G4_IO1_BIT_OFFSET: u8 = 12;
	const G4_IO1_BIT_WIDTH: u8 = 1;
	/// G4_IO1 analog switch enable (Width: 1, Offset: 12)
	pub fn get_g4_io1() -> u8 { ::read(REGISTER_ADDRESS, G4_IO1_BIT_OFFSET, G4_IO1_BIT_WIDTH) as u8 }
	/// G4_IO1 analog switch enable (Width: 1, Offset: 12)
	pub fn set_g4_io1(value: u8) { ::write(REGISTER_ADDRESS, G4_IO1_BIT_OFFSET, G4_IO1_BIT_WIDTH, value as u32); }

	const G4_IO2_BIT_OFFSET: u8 = 13;
	const G4_IO2_BIT_WIDTH: u8 = 1;
	/// G4_IO2 analog switch enable (Width: 1, Offset: 13)
	pub fn get_g4_io2() -> u8 { ::read(REGISTER_ADDRESS, G4_IO2_BIT_OFFSET, G4_IO2_BIT_WIDTH) as u8 }
	/// G4_IO2 analog switch enable (Width: 1, Offset: 13)
	pub fn set_g4_io2(value: u8) { ::write(REGISTER_ADDRESS, G4_IO2_BIT_OFFSET, G4_IO2_BIT_WIDTH, value as u32); }

	const G4_IO3_BIT_OFFSET: u8 = 14;
	const G4_IO3_BIT_WIDTH: u8 = 1;
	/// G4_IO3 analog switch enable (Width: 1, Offset: 14)
	pub fn get_g4_io3() -> u8 { ::read(REGISTER_ADDRESS, G4_IO3_BIT_OFFSET, G4_IO3_BIT_WIDTH) as u8 }
	/// G4_IO3 analog switch enable (Width: 1, Offset: 14)
	pub fn set_g4_io3(value: u8) { ::write(REGISTER_ADDRESS, G4_IO3_BIT_OFFSET, G4_IO3_BIT_WIDTH, value as u32); }

	const G4_IO4_BIT_OFFSET: u8 = 15;
	const G4_IO4_BIT_WIDTH: u8 = 1;
	/// G4_IO4 analog switch enable (Width: 1, Offset: 15)
	pub fn get_g4_io4() -> u8 { ::read(REGISTER_ADDRESS, G4_IO4_BIT_OFFSET, G4_IO4_BIT_WIDTH) as u8 }
	/// G4_IO4 analog switch enable (Width: 1, Offset: 15)
	pub fn set_g4_io4(value: u8) { ::write(REGISTER_ADDRESS, G4_IO4_BIT_OFFSET, G4_IO4_BIT_WIDTH, value as u32); }

	const G5_IO1_BIT_OFFSET: u8 = 16;
	const G5_IO1_BIT_WIDTH: u8 = 1;
	/// G5_IO1 analog switch enable (Width: 1, Offset: 16)
	pub fn get_g5_io1() -> u8 { ::read(REGISTER_ADDRESS, G5_IO1_BIT_OFFSET, G5_IO1_BIT_WIDTH) as u8 }
	/// G5_IO1 analog switch enable (Width: 1, Offset: 16)
	pub fn set_g5_io1(value: u8) { ::write(REGISTER_ADDRESS, G5_IO1_BIT_OFFSET, G5_IO1_BIT_WIDTH, value as u32); }

	const G5_IO2_BIT_OFFSET: u8 = 17;
	const G5_IO2_BIT_WIDTH: u8 = 1;
	/// G5_IO2 analog switch enable (Width: 1, Offset: 17)
	pub fn get_g5_io2() -> u8 { ::read(REGISTER_ADDRESS, G5_IO2_BIT_OFFSET, G5_IO2_BIT_WIDTH) as u8 }
	/// G5_IO2 analog switch enable (Width: 1, Offset: 17)
	pub fn set_g5_io2(value: u8) { ::write(REGISTER_ADDRESS, G5_IO2_BIT_OFFSET, G5_IO2_BIT_WIDTH, value as u32); }

	const G5_IO3_BIT_OFFSET: u8 = 18;
	const G5_IO3_BIT_WIDTH: u8 = 1;
	/// G5_IO3 analog switch enable (Width: 1, Offset: 18)
	pub fn get_g5_io3() -> u8 { ::read(REGISTER_ADDRESS, G5_IO3_BIT_OFFSET, G5_IO3_BIT_WIDTH) as u8 }
	/// G5_IO3 analog switch enable (Width: 1, Offset: 18)
	pub fn set_g5_io3(value: u8) { ::write(REGISTER_ADDRESS, G5_IO3_BIT_OFFSET, G5_IO3_BIT_WIDTH, value as u32); }

	const G5_IO4_BIT_OFFSET: u8 = 19;
	const G5_IO4_BIT_WIDTH: u8 = 1;
	/// G5_IO4 analog switch enable (Width: 1, Offset: 19)
	pub fn get_g5_io4() -> u8 { ::read(REGISTER_ADDRESS, G5_IO4_BIT_OFFSET, G5_IO4_BIT_WIDTH) as u8 }
	/// G5_IO4 analog switch enable (Width: 1, Offset: 19)
	pub fn set_g5_io4(value: u8) { ::write(REGISTER_ADDRESS, G5_IO4_BIT_OFFSET, G5_IO4_BIT_WIDTH, value as u32); }

	const G6_IO1_BIT_OFFSET: u8 = 20;
	const G6_IO1_BIT_WIDTH: u8 = 1;
	/// G6_IO1 analog switch enable (Width: 1, Offset: 20)
	pub fn get_g6_io1() -> u8 { ::read(REGISTER_ADDRESS, G6_IO1_BIT_OFFSET, G6_IO1_BIT_WIDTH) as u8 }
	/// G6_IO1 analog switch enable (Width: 1, Offset: 20)
	pub fn set_g6_io1(value: u8) { ::write(REGISTER_ADDRESS, G6_IO1_BIT_OFFSET, G6_IO1_BIT_WIDTH, value as u32); }

	const G6_IO2_BIT_OFFSET: u8 = 21;
	const G6_IO2_BIT_WIDTH: u8 = 1;
	/// G6_IO2 analog switch enable (Width: 1, Offset: 21)
	pub fn get_g6_io2() -> u8 { ::read(REGISTER_ADDRESS, G6_IO2_BIT_OFFSET, G6_IO2_BIT_WIDTH) as u8 }
	/// G6_IO2 analog switch enable (Width: 1, Offset: 21)
	pub fn set_g6_io2(value: u8) { ::write(REGISTER_ADDRESS, G6_IO2_BIT_OFFSET, G6_IO2_BIT_WIDTH, value as u32); }

	const G6_IO3_BIT_OFFSET: u8 = 22;
	const G6_IO3_BIT_WIDTH: u8 = 1;
	/// G6_IO3 analog switch enable (Width: 1, Offset: 22)
	pub fn get_g6_io3() -> u8 { ::read(REGISTER_ADDRESS, G6_IO3_BIT_OFFSET, G6_IO3_BIT_WIDTH) as u8 }
	/// G6_IO3 analog switch enable (Width: 1, Offset: 22)
	pub fn set_g6_io3(value: u8) { ::write(REGISTER_ADDRESS, G6_IO3_BIT_OFFSET, G6_IO3_BIT_WIDTH, value as u32); }

	const G6_IO4_BIT_OFFSET: u8 = 23;
	const G6_IO4_BIT_WIDTH: u8 = 1;
	/// G6_IO4 analog switch enable (Width: 1, Offset: 23)
	pub fn get_g6_io4() -> u8 { ::read(REGISTER_ADDRESS, G6_IO4_BIT_OFFSET, G6_IO4_BIT_WIDTH) as u8 }
	/// G6_IO4 analog switch enable (Width: 1, Offset: 23)
	pub fn set_g6_io4(value: u8) { ::write(REGISTER_ADDRESS, G6_IO4_BIT_OFFSET, G6_IO4_BIT_WIDTH, value as u32); }

	const G7_IO1_BIT_OFFSET: u8 = 24;
	const G7_IO1_BIT_WIDTH: u8 = 1;
	/// G7_IO1 analog switch enable (Width: 1, Offset: 24)
	pub fn get_g7_io1() -> u8 { ::read(REGISTER_ADDRESS, G7_IO1_BIT_OFFSET, G7_IO1_BIT_WIDTH) as u8 }
	/// G7_IO1 analog switch enable (Width: 1, Offset: 24)
	pub fn set_g7_io1(value: u8) { ::write(REGISTER_ADDRESS, G7_IO1_BIT_OFFSET, G7_IO1_BIT_WIDTH, value as u32); }

	const G7_IO2_BIT_OFFSET: u8 = 25;
	const G7_IO2_BIT_WIDTH: u8 = 1;
	/// G7_IO2 analog switch enable (Width: 1, Offset: 25)
	pub fn get_g7_io2() -> u8 { ::read(REGISTER_ADDRESS, G7_IO2_BIT_OFFSET, G7_IO2_BIT_WIDTH) as u8 }
	/// G7_IO2 analog switch enable (Width: 1, Offset: 25)
	pub fn set_g7_io2(value: u8) { ::write(REGISTER_ADDRESS, G7_IO2_BIT_OFFSET, G7_IO2_BIT_WIDTH, value as u32); }

	const G7_IO3_BIT_OFFSET: u8 = 26;
	const G7_IO3_BIT_WIDTH: u8 = 1;
	/// G7_IO3 analog switch enable (Width: 1, Offset: 26)
	pub fn get_g7_io3() -> u8 { ::read(REGISTER_ADDRESS, G7_IO3_BIT_OFFSET, G7_IO3_BIT_WIDTH) as u8 }
	/// G7_IO3 analog switch enable (Width: 1, Offset: 26)
	pub fn set_g7_io3(value: u8) { ::write(REGISTER_ADDRESS, G7_IO3_BIT_OFFSET, G7_IO3_BIT_WIDTH, value as u32); }

	const G7_IO4_BIT_OFFSET: u8 = 27;
	const G7_IO4_BIT_WIDTH: u8 = 1;
	/// G7_IO4 analog switch enable (Width: 1, Offset: 27)
	pub fn get_g7_io4() -> u8 { ::read(REGISTER_ADDRESS, G7_IO4_BIT_OFFSET, G7_IO4_BIT_WIDTH) as u8 }
	/// G7_IO4 analog switch enable (Width: 1, Offset: 27)
	pub fn set_g7_io4(value: u8) { ::write(REGISTER_ADDRESS, G7_IO4_BIT_OFFSET, G7_IO4_BIT_WIDTH, value as u32); }

	const G8_IO1_BIT_OFFSET: u8 = 28;
	const G8_IO1_BIT_WIDTH: u8 = 1;
	/// G8_IO1 analog switch enable (Width: 1, Offset: 28)
	pub fn get_g8_io1() -> u8 { ::read(REGISTER_ADDRESS, G8_IO1_BIT_OFFSET, G8_IO1_BIT_WIDTH) as u8 }
	/// G8_IO1 analog switch enable (Width: 1, Offset: 28)
	pub fn set_g8_io1(value: u8) { ::write(REGISTER_ADDRESS, G8_IO1_BIT_OFFSET, G8_IO1_BIT_WIDTH, value as u32); }

	const G8_IO2_BIT_OFFSET: u8 = 29;
	const G8_IO2_BIT_WIDTH: u8 = 1;
	/// G8_IO2 analog switch enable (Width: 1, Offset: 29)
	pub fn get_g8_io2() -> u8 { ::read(REGISTER_ADDRESS, G8_IO2_BIT_OFFSET, G8_IO2_BIT_WIDTH) as u8 }
	/// G8_IO2 analog switch enable (Width: 1, Offset: 29)
	pub fn set_g8_io2(value: u8) { ::write(REGISTER_ADDRESS, G8_IO2_BIT_OFFSET, G8_IO2_BIT_WIDTH, value as u32); }

	const G8_IO3_BIT_OFFSET: u8 = 30;
	const G8_IO3_BIT_WIDTH: u8 = 1;
	/// G8_IO3 analog switch enable (Width: 1, Offset: 30)
	pub fn get_g8_io3() -> u8 { ::read(REGISTER_ADDRESS, G8_IO3_BIT_OFFSET, G8_IO3_BIT_WIDTH) as u8 }
	/// G8_IO3 analog switch enable (Width: 1, Offset: 30)
	pub fn set_g8_io3(value: u8) { ::write(REGISTER_ADDRESS, G8_IO3_BIT_OFFSET, G8_IO3_BIT_WIDTH, value as u32); }

	const G8_IO4_BIT_OFFSET: u8 = 31;
	const G8_IO4_BIT_WIDTH: u8 = 1;
	/// G8_IO4 analog switch enable (Width: 1, Offset: 31)
	pub fn get_g8_io4() -> u8 { ::read(REGISTER_ADDRESS, G8_IO4_BIT_OFFSET, G8_IO4_BIT_WIDTH) as u8 }
	/// G8_IO4 analog switch enable (Width: 1, Offset: 31)
	pub fn set_g8_io4(value: u8) { ::write(REGISTER_ADDRESS, G8_IO4_BIT_OFFSET, G8_IO4_BIT_WIDTH, value as u32); }
}
/// I/O sampling control register
/// Size: 0x20 bits
pub mod ioscr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x20;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const G1_IO1_BIT_OFFSET: u8 = 0;
	const G1_IO1_BIT_WIDTH: u8 = 1;
	/// G1_IO1 sampling mode (Width: 1, Offset: 0)
	pub fn get_g1_io1() -> u8 { ::read(REGISTER_ADDRESS, G1_IO1_BIT_OFFSET, G1_IO1_BIT_WIDTH) as u8 }
	/// G1_IO1 sampling mode (Width: 1, Offset: 0)
	pub fn set_g1_io1(value: u8) { ::write(REGISTER_ADDRESS, G1_IO1_BIT_OFFSET, G1_IO1_BIT_WIDTH, value as u32); }

	const G1_IO2_BIT_OFFSET: u8 = 1;
	const G1_IO2_BIT_WIDTH: u8 = 1;
	/// G1_IO2 sampling mode (Width: 1, Offset: 1)
	pub fn get_g1_io2() -> u8 { ::read(REGISTER_ADDRESS, G1_IO2_BIT_OFFSET, G1_IO2_BIT_WIDTH) as u8 }
	/// G1_IO2 sampling mode (Width: 1, Offset: 1)
	pub fn set_g1_io2(value: u8) { ::write(REGISTER_ADDRESS, G1_IO2_BIT_OFFSET, G1_IO2_BIT_WIDTH, value as u32); }

	const G1_IO3_BIT_OFFSET: u8 = 2;
	const G1_IO3_BIT_WIDTH: u8 = 1;
	/// G1_IO3 sampling mode (Width: 1, Offset: 2)
	pub fn get_g1_io3() -> u8 { ::read(REGISTER_ADDRESS, G1_IO3_BIT_OFFSET, G1_IO3_BIT_WIDTH) as u8 }
	/// G1_IO3 sampling mode (Width: 1, Offset: 2)
	pub fn set_g1_io3(value: u8) { ::write(REGISTER_ADDRESS, G1_IO3_BIT_OFFSET, G1_IO3_BIT_WIDTH, value as u32); }

	const G1_IO4_BIT_OFFSET: u8 = 3;
	const G1_IO4_BIT_WIDTH: u8 = 1;
	/// G1_IO4 sampling mode (Width: 1, Offset: 3)
	pub fn get_g1_io4() -> u8 { ::read(REGISTER_ADDRESS, G1_IO4_BIT_OFFSET, G1_IO4_BIT_WIDTH) as u8 }
	/// G1_IO4 sampling mode (Width: 1, Offset: 3)
	pub fn set_g1_io4(value: u8) { ::write(REGISTER_ADDRESS, G1_IO4_BIT_OFFSET, G1_IO4_BIT_WIDTH, value as u32); }

	const G2_IO1_BIT_OFFSET: u8 = 4;
	const G2_IO1_BIT_WIDTH: u8 = 1;
	/// G2_IO1 sampling mode (Width: 1, Offset: 4)
	pub fn get_g2_io1() -> u8 { ::read(REGISTER_ADDRESS, G2_IO1_BIT_OFFSET, G2_IO1_BIT_WIDTH) as u8 }
	/// G2_IO1 sampling mode (Width: 1, Offset: 4)
	pub fn set_g2_io1(value: u8) { ::write(REGISTER_ADDRESS, G2_IO1_BIT_OFFSET, G2_IO1_BIT_WIDTH, value as u32); }

	const G2_IO2_BIT_OFFSET: u8 = 5;
	const G2_IO2_BIT_WIDTH: u8 = 1;
	/// G2_IO2 sampling mode (Width: 1, Offset: 5)
	pub fn get_g2_io2() -> u8 { ::read(REGISTER_ADDRESS, G2_IO2_BIT_OFFSET, G2_IO2_BIT_WIDTH) as u8 }
	/// G2_IO2 sampling mode (Width: 1, Offset: 5)
	pub fn set_g2_io2(value: u8) { ::write(REGISTER_ADDRESS, G2_IO2_BIT_OFFSET, G2_IO2_BIT_WIDTH, value as u32); }

	const G2_IO3_BIT_OFFSET: u8 = 6;
	const G2_IO3_BIT_WIDTH: u8 = 1;
	/// G2_IO3 sampling mode (Width: 1, Offset: 6)
	pub fn get_g2_io3() -> u8 { ::read(REGISTER_ADDRESS, G2_IO3_BIT_OFFSET, G2_IO3_BIT_WIDTH) as u8 }
	/// G2_IO3 sampling mode (Width: 1, Offset: 6)
	pub fn set_g2_io3(value: u8) { ::write(REGISTER_ADDRESS, G2_IO3_BIT_OFFSET, G2_IO3_BIT_WIDTH, value as u32); }

	const G2_IO4_BIT_OFFSET: u8 = 7;
	const G2_IO4_BIT_WIDTH: u8 = 1;
	/// G2_IO4 sampling mode (Width: 1, Offset: 7)
	pub fn get_g2_io4() -> u8 { ::read(REGISTER_ADDRESS, G2_IO4_BIT_OFFSET, G2_IO4_BIT_WIDTH) as u8 }
	/// G2_IO4 sampling mode (Width: 1, Offset: 7)
	pub fn set_g2_io4(value: u8) { ::write(REGISTER_ADDRESS, G2_IO4_BIT_OFFSET, G2_IO4_BIT_WIDTH, value as u32); }

	const G3_IO1_BIT_OFFSET: u8 = 8;
	const G3_IO1_BIT_WIDTH: u8 = 1;
	/// G3_IO1 sampling mode (Width: 1, Offset: 8)
	pub fn get_g3_io1() -> u8 { ::read(REGISTER_ADDRESS, G3_IO1_BIT_OFFSET, G3_IO1_BIT_WIDTH) as u8 }
	/// G3_IO1 sampling mode (Width: 1, Offset: 8)
	pub fn set_g3_io1(value: u8) { ::write(REGISTER_ADDRESS, G3_IO1_BIT_OFFSET, G3_IO1_BIT_WIDTH, value as u32); }

	const G3_IO2_BIT_OFFSET: u8 = 9;
	const G3_IO2_BIT_WIDTH: u8 = 1;
	/// G3_IO2 sampling mode (Width: 1, Offset: 9)
	pub fn get_g3_io2() -> u8 { ::read(REGISTER_ADDRESS, G3_IO2_BIT_OFFSET, G3_IO2_BIT_WIDTH) as u8 }
	/// G3_IO2 sampling mode (Width: 1, Offset: 9)
	pub fn set_g3_io2(value: u8) { ::write(REGISTER_ADDRESS, G3_IO2_BIT_OFFSET, G3_IO2_BIT_WIDTH, value as u32); }

	const G3_IO3_BIT_OFFSET: u8 = 10;
	const G3_IO3_BIT_WIDTH: u8 = 1;
	/// G3_IO3 sampling mode (Width: 1, Offset: 10)
	pub fn get_g3_io3() -> u8 { ::read(REGISTER_ADDRESS, G3_IO3_BIT_OFFSET, G3_IO3_BIT_WIDTH) as u8 }
	/// G3_IO3 sampling mode (Width: 1, Offset: 10)
	pub fn set_g3_io3(value: u8) { ::write(REGISTER_ADDRESS, G3_IO3_BIT_OFFSET, G3_IO3_BIT_WIDTH, value as u32); }

	const G3_IO4_BIT_OFFSET: u8 = 11;
	const G3_IO4_BIT_WIDTH: u8 = 1;
	/// G3_IO4 sampling mode (Width: 1, Offset: 11)
	pub fn get_g3_io4() -> u8 { ::read(REGISTER_ADDRESS, G3_IO4_BIT_OFFSET, G3_IO4_BIT_WIDTH) as u8 }
	/// G3_IO4 sampling mode (Width: 1, Offset: 11)
	pub fn set_g3_io4(value: u8) { ::write(REGISTER_ADDRESS, G3_IO4_BIT_OFFSET, G3_IO4_BIT_WIDTH, value as u32); }

	const G4_IO1_BIT_OFFSET: u8 = 12;
	const G4_IO1_BIT_WIDTH: u8 = 1;
	/// G4_IO1 sampling mode (Width: 1, Offset: 12)
	pub fn get_g4_io1() -> u8 { ::read(REGISTER_ADDRESS, G4_IO1_BIT_OFFSET, G4_IO1_BIT_WIDTH) as u8 }
	/// G4_IO1 sampling mode (Width: 1, Offset: 12)
	pub fn set_g4_io1(value: u8) { ::write(REGISTER_ADDRESS, G4_IO1_BIT_OFFSET, G4_IO1_BIT_WIDTH, value as u32); }

	const G4_IO2_BIT_OFFSET: u8 = 13;
	const G4_IO2_BIT_WIDTH: u8 = 1;
	/// G4_IO2 sampling mode (Width: 1, Offset: 13)
	pub fn get_g4_io2() -> u8 { ::read(REGISTER_ADDRESS, G4_IO2_BIT_OFFSET, G4_IO2_BIT_WIDTH) as u8 }
	/// G4_IO2 sampling mode (Width: 1, Offset: 13)
	pub fn set_g4_io2(value: u8) { ::write(REGISTER_ADDRESS, G4_IO2_BIT_OFFSET, G4_IO2_BIT_WIDTH, value as u32); }

	const G4_IO3_BIT_OFFSET: u8 = 14;
	const G4_IO3_BIT_WIDTH: u8 = 1;
	/// G4_IO3 sampling mode (Width: 1, Offset: 14)
	pub fn get_g4_io3() -> u8 { ::read(REGISTER_ADDRESS, G4_IO3_BIT_OFFSET, G4_IO3_BIT_WIDTH) as u8 }
	/// G4_IO3 sampling mode (Width: 1, Offset: 14)
	pub fn set_g4_io3(value: u8) { ::write(REGISTER_ADDRESS, G4_IO3_BIT_OFFSET, G4_IO3_BIT_WIDTH, value as u32); }

	const G4_IO4_BIT_OFFSET: u8 = 15;
	const G4_IO4_BIT_WIDTH: u8 = 1;
	/// G4_IO4 sampling mode (Width: 1, Offset: 15)
	pub fn get_g4_io4() -> u8 { ::read(REGISTER_ADDRESS, G4_IO4_BIT_OFFSET, G4_IO4_BIT_WIDTH) as u8 }
	/// G4_IO4 sampling mode (Width: 1, Offset: 15)
	pub fn set_g4_io4(value: u8) { ::write(REGISTER_ADDRESS, G4_IO4_BIT_OFFSET, G4_IO4_BIT_WIDTH, value as u32); }

	const G5_IO1_BIT_OFFSET: u8 = 16;
	const G5_IO1_BIT_WIDTH: u8 = 1;
	/// G5_IO1 sampling mode (Width: 1, Offset: 16)
	pub fn get_g5_io1() -> u8 { ::read(REGISTER_ADDRESS, G5_IO1_BIT_OFFSET, G5_IO1_BIT_WIDTH) as u8 }
	/// G5_IO1 sampling mode (Width: 1, Offset: 16)
	pub fn set_g5_io1(value: u8) { ::write(REGISTER_ADDRESS, G5_IO1_BIT_OFFSET, G5_IO1_BIT_WIDTH, value as u32); }

	const G5_IO2_BIT_OFFSET: u8 = 17;
	const G5_IO2_BIT_WIDTH: u8 = 1;
	/// G5_IO2 sampling mode (Width: 1, Offset: 17)
	pub fn get_g5_io2() -> u8 { ::read(REGISTER_ADDRESS, G5_IO2_BIT_OFFSET, G5_IO2_BIT_WIDTH) as u8 }
	/// G5_IO2 sampling mode (Width: 1, Offset: 17)
	pub fn set_g5_io2(value: u8) { ::write(REGISTER_ADDRESS, G5_IO2_BIT_OFFSET, G5_IO2_BIT_WIDTH, value as u32); }

	const G5_IO3_BIT_OFFSET: u8 = 18;
	const G5_IO3_BIT_WIDTH: u8 = 1;
	/// G5_IO3 sampling mode (Width: 1, Offset: 18)
	pub fn get_g5_io3() -> u8 { ::read(REGISTER_ADDRESS, G5_IO3_BIT_OFFSET, G5_IO3_BIT_WIDTH) as u8 }
	/// G5_IO3 sampling mode (Width: 1, Offset: 18)
	pub fn set_g5_io3(value: u8) { ::write(REGISTER_ADDRESS, G5_IO3_BIT_OFFSET, G5_IO3_BIT_WIDTH, value as u32); }

	const G5_IO4_BIT_OFFSET: u8 = 19;
	const G5_IO4_BIT_WIDTH: u8 = 1;
	/// G5_IO4 sampling mode (Width: 1, Offset: 19)
	pub fn get_g5_io4() -> u8 { ::read(REGISTER_ADDRESS, G5_IO4_BIT_OFFSET, G5_IO4_BIT_WIDTH) as u8 }
	/// G5_IO4 sampling mode (Width: 1, Offset: 19)
	pub fn set_g5_io4(value: u8) { ::write(REGISTER_ADDRESS, G5_IO4_BIT_OFFSET, G5_IO4_BIT_WIDTH, value as u32); }

	const G6_IO1_BIT_OFFSET: u8 = 20;
	const G6_IO1_BIT_WIDTH: u8 = 1;
	/// G6_IO1 sampling mode (Width: 1, Offset: 20)
	pub fn get_g6_io1() -> u8 { ::read(REGISTER_ADDRESS, G6_IO1_BIT_OFFSET, G6_IO1_BIT_WIDTH) as u8 }
	/// G6_IO1 sampling mode (Width: 1, Offset: 20)
	pub fn set_g6_io1(value: u8) { ::write(REGISTER_ADDRESS, G6_IO1_BIT_OFFSET, G6_IO1_BIT_WIDTH, value as u32); }

	const G6_IO2_BIT_OFFSET: u8 = 21;
	const G6_IO2_BIT_WIDTH: u8 = 1;
	/// G6_IO2 sampling mode (Width: 1, Offset: 21)
	pub fn get_g6_io2() -> u8 { ::read(REGISTER_ADDRESS, G6_IO2_BIT_OFFSET, G6_IO2_BIT_WIDTH) as u8 }
	/// G6_IO2 sampling mode (Width: 1, Offset: 21)
	pub fn set_g6_io2(value: u8) { ::write(REGISTER_ADDRESS, G6_IO2_BIT_OFFSET, G6_IO2_BIT_WIDTH, value as u32); }

	const G6_IO3_BIT_OFFSET: u8 = 22;
	const G6_IO3_BIT_WIDTH: u8 = 1;
	/// G6_IO3 sampling mode (Width: 1, Offset: 22)
	pub fn get_g6_io3() -> u8 { ::read(REGISTER_ADDRESS, G6_IO3_BIT_OFFSET, G6_IO3_BIT_WIDTH) as u8 }
	/// G6_IO3 sampling mode (Width: 1, Offset: 22)
	pub fn set_g6_io3(value: u8) { ::write(REGISTER_ADDRESS, G6_IO3_BIT_OFFSET, G6_IO3_BIT_WIDTH, value as u32); }

	const G6_IO4_BIT_OFFSET: u8 = 23;
	const G6_IO4_BIT_WIDTH: u8 = 1;
	/// G6_IO4 sampling mode (Width: 1, Offset: 23)
	pub fn get_g6_io4() -> u8 { ::read(REGISTER_ADDRESS, G6_IO4_BIT_OFFSET, G6_IO4_BIT_WIDTH) as u8 }
	/// G6_IO4 sampling mode (Width: 1, Offset: 23)
	pub fn set_g6_io4(value: u8) { ::write(REGISTER_ADDRESS, G6_IO4_BIT_OFFSET, G6_IO4_BIT_WIDTH, value as u32); }

	const G7_IO1_BIT_OFFSET: u8 = 24;
	const G7_IO1_BIT_WIDTH: u8 = 1;
	/// G7_IO1 sampling mode (Width: 1, Offset: 24)
	pub fn get_g7_io1() -> u8 { ::read(REGISTER_ADDRESS, G7_IO1_BIT_OFFSET, G7_IO1_BIT_WIDTH) as u8 }
	/// G7_IO1 sampling mode (Width: 1, Offset: 24)
	pub fn set_g7_io1(value: u8) { ::write(REGISTER_ADDRESS, G7_IO1_BIT_OFFSET, G7_IO1_BIT_WIDTH, value as u32); }

	const G7_IO2_BIT_OFFSET: u8 = 25;
	const G7_IO2_BIT_WIDTH: u8 = 1;
	/// G7_IO2 sampling mode (Width: 1, Offset: 25)
	pub fn get_g7_io2() -> u8 { ::read(REGISTER_ADDRESS, G7_IO2_BIT_OFFSET, G7_IO2_BIT_WIDTH) as u8 }
	/// G7_IO2 sampling mode (Width: 1, Offset: 25)
	pub fn set_g7_io2(value: u8) { ::write(REGISTER_ADDRESS, G7_IO2_BIT_OFFSET, G7_IO2_BIT_WIDTH, value as u32); }

	const G7_IO3_BIT_OFFSET: u8 = 26;
	const G7_IO3_BIT_WIDTH: u8 = 1;
	/// G7_IO3 sampling mode (Width: 1, Offset: 26)
	pub fn get_g7_io3() -> u8 { ::read(REGISTER_ADDRESS, G7_IO3_BIT_OFFSET, G7_IO3_BIT_WIDTH) as u8 }
	/// G7_IO3 sampling mode (Width: 1, Offset: 26)
	pub fn set_g7_io3(value: u8) { ::write(REGISTER_ADDRESS, G7_IO3_BIT_OFFSET, G7_IO3_BIT_WIDTH, value as u32); }

	const G7_IO4_BIT_OFFSET: u8 = 27;
	const G7_IO4_BIT_WIDTH: u8 = 1;
	/// G7_IO4 sampling mode (Width: 1, Offset: 27)
	pub fn get_g7_io4() -> u8 { ::read(REGISTER_ADDRESS, G7_IO4_BIT_OFFSET, G7_IO4_BIT_WIDTH) as u8 }
	/// G7_IO4 sampling mode (Width: 1, Offset: 27)
	pub fn set_g7_io4(value: u8) { ::write(REGISTER_ADDRESS, G7_IO4_BIT_OFFSET, G7_IO4_BIT_WIDTH, value as u32); }

	const G8_IO1_BIT_OFFSET: u8 = 28;
	const G8_IO1_BIT_WIDTH: u8 = 1;
	/// G8_IO1 sampling mode (Width: 1, Offset: 28)
	pub fn get_g8_io1() -> u8 { ::read(REGISTER_ADDRESS, G8_IO1_BIT_OFFSET, G8_IO1_BIT_WIDTH) as u8 }
	/// G8_IO1 sampling mode (Width: 1, Offset: 28)
	pub fn set_g8_io1(value: u8) { ::write(REGISTER_ADDRESS, G8_IO1_BIT_OFFSET, G8_IO1_BIT_WIDTH, value as u32); }

	const G8_IO2_BIT_OFFSET: u8 = 29;
	const G8_IO2_BIT_WIDTH: u8 = 1;
	/// G8_IO2 sampling mode (Width: 1, Offset: 29)
	pub fn get_g8_io2() -> u8 { ::read(REGISTER_ADDRESS, G8_IO2_BIT_OFFSET, G8_IO2_BIT_WIDTH) as u8 }
	/// G8_IO2 sampling mode (Width: 1, Offset: 29)
	pub fn set_g8_io2(value: u8) { ::write(REGISTER_ADDRESS, G8_IO2_BIT_OFFSET, G8_IO2_BIT_WIDTH, value as u32); }

	const G8_IO3_BIT_OFFSET: u8 = 30;
	const G8_IO3_BIT_WIDTH: u8 = 1;
	/// G8_IO3 sampling mode (Width: 1, Offset: 30)
	pub fn get_g8_io3() -> u8 { ::read(REGISTER_ADDRESS, G8_IO3_BIT_OFFSET, G8_IO3_BIT_WIDTH) as u8 }
	/// G8_IO3 sampling mode (Width: 1, Offset: 30)
	pub fn set_g8_io3(value: u8) { ::write(REGISTER_ADDRESS, G8_IO3_BIT_OFFSET, G8_IO3_BIT_WIDTH, value as u32); }

	const G8_IO4_BIT_OFFSET: u8 = 31;
	const G8_IO4_BIT_WIDTH: u8 = 1;
	/// G8_IO4 sampling mode (Width: 1, Offset: 31)
	pub fn get_g8_io4() -> u8 { ::read(REGISTER_ADDRESS, G8_IO4_BIT_OFFSET, G8_IO4_BIT_WIDTH) as u8 }
	/// G8_IO4 sampling mode (Width: 1, Offset: 31)
	pub fn set_g8_io4(value: u8) { ::write(REGISTER_ADDRESS, G8_IO4_BIT_OFFSET, G8_IO4_BIT_WIDTH, value as u32); }
}
/// I/O channel control register
/// Size: 0x20 bits
pub mod ioccr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x28;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const G1_IO1_BIT_OFFSET: u8 = 0;
	const G1_IO1_BIT_WIDTH: u8 = 1;
	/// G1_IO1 channel mode (Width: 1, Offset: 0)
	pub fn get_g1_io1() -> u8 { ::read(REGISTER_ADDRESS, G1_IO1_BIT_OFFSET, G1_IO1_BIT_WIDTH) as u8 }
	/// G1_IO1 channel mode (Width: 1, Offset: 0)
	pub fn set_g1_io1(value: u8) { ::write(REGISTER_ADDRESS, G1_IO1_BIT_OFFSET, G1_IO1_BIT_WIDTH, value as u32); }

	const G1_IO2_BIT_OFFSET: u8 = 1;
	const G1_IO2_BIT_WIDTH: u8 = 1;
	/// G1_IO2 channel mode (Width: 1, Offset: 1)
	pub fn get_g1_io2() -> u8 { ::read(REGISTER_ADDRESS, G1_IO2_BIT_OFFSET, G1_IO2_BIT_WIDTH) as u8 }
	/// G1_IO2 channel mode (Width: 1, Offset: 1)
	pub fn set_g1_io2(value: u8) { ::write(REGISTER_ADDRESS, G1_IO2_BIT_OFFSET, G1_IO2_BIT_WIDTH, value as u32); }

	const G1_IO3_BIT_OFFSET: u8 = 2;
	const G1_IO3_BIT_WIDTH: u8 = 1;
	/// G1_IO3 channel mode (Width: 1, Offset: 2)
	pub fn get_g1_io3() -> u8 { ::read(REGISTER_ADDRESS, G1_IO3_BIT_OFFSET, G1_IO3_BIT_WIDTH) as u8 }
	/// G1_IO3 channel mode (Width: 1, Offset: 2)
	pub fn set_g1_io3(value: u8) { ::write(REGISTER_ADDRESS, G1_IO3_BIT_OFFSET, G1_IO3_BIT_WIDTH, value as u32); }

	const G1_IO4_BIT_OFFSET: u8 = 3;
	const G1_IO4_BIT_WIDTH: u8 = 1;
	/// G1_IO4 channel mode (Width: 1, Offset: 3)
	pub fn get_g1_io4() -> u8 { ::read(REGISTER_ADDRESS, G1_IO4_BIT_OFFSET, G1_IO4_BIT_WIDTH) as u8 }
	/// G1_IO4 channel mode (Width: 1, Offset: 3)
	pub fn set_g1_io4(value: u8) { ::write(REGISTER_ADDRESS, G1_IO4_BIT_OFFSET, G1_IO4_BIT_WIDTH, value as u32); }

	const G2_IO1_BIT_OFFSET: u8 = 4;
	const G2_IO1_BIT_WIDTH: u8 = 1;
	/// G2_IO1 channel mode (Width: 1, Offset: 4)
	pub fn get_g2_io1() -> u8 { ::read(REGISTER_ADDRESS, G2_IO1_BIT_OFFSET, G2_IO1_BIT_WIDTH) as u8 }
	/// G2_IO1 channel mode (Width: 1, Offset: 4)
	pub fn set_g2_io1(value: u8) { ::write(REGISTER_ADDRESS, G2_IO1_BIT_OFFSET, G2_IO1_BIT_WIDTH, value as u32); }

	const G2_IO2_BIT_OFFSET: u8 = 5;
	const G2_IO2_BIT_WIDTH: u8 = 1;
	/// G2_IO2 channel mode (Width: 1, Offset: 5)
	pub fn get_g2_io2() -> u8 { ::read(REGISTER_ADDRESS, G2_IO2_BIT_OFFSET, G2_IO2_BIT_WIDTH) as u8 }
	/// G2_IO2 channel mode (Width: 1, Offset: 5)
	pub fn set_g2_io2(value: u8) { ::write(REGISTER_ADDRESS, G2_IO2_BIT_OFFSET, G2_IO2_BIT_WIDTH, value as u32); }

	const G2_IO3_BIT_OFFSET: u8 = 6;
	const G2_IO3_BIT_WIDTH: u8 = 1;
	/// G2_IO3 channel mode (Width: 1, Offset: 6)
	pub fn get_g2_io3() -> u8 { ::read(REGISTER_ADDRESS, G2_IO3_BIT_OFFSET, G2_IO3_BIT_WIDTH) as u8 }
	/// G2_IO3 channel mode (Width: 1, Offset: 6)
	pub fn set_g2_io3(value: u8) { ::write(REGISTER_ADDRESS, G2_IO3_BIT_OFFSET, G2_IO3_BIT_WIDTH, value as u32); }

	const G2_IO4_BIT_OFFSET: u8 = 7;
	const G2_IO4_BIT_WIDTH: u8 = 1;
	/// G2_IO4 channel mode (Width: 1, Offset: 7)
	pub fn get_g2_io4() -> u8 { ::read(REGISTER_ADDRESS, G2_IO4_BIT_OFFSET, G2_IO4_BIT_WIDTH) as u8 }
	/// G2_IO4 channel mode (Width: 1, Offset: 7)
	pub fn set_g2_io4(value: u8) { ::write(REGISTER_ADDRESS, G2_IO4_BIT_OFFSET, G2_IO4_BIT_WIDTH, value as u32); }

	const G3_IO1_BIT_OFFSET: u8 = 8;
	const G3_IO1_BIT_WIDTH: u8 = 1;
	/// G3_IO1 channel mode (Width: 1, Offset: 8)
	pub fn get_g3_io1() -> u8 { ::read(REGISTER_ADDRESS, G3_IO1_BIT_OFFSET, G3_IO1_BIT_WIDTH) as u8 }
	/// G3_IO1 channel mode (Width: 1, Offset: 8)
	pub fn set_g3_io1(value: u8) { ::write(REGISTER_ADDRESS, G3_IO1_BIT_OFFSET, G3_IO1_BIT_WIDTH, value as u32); }

	const G3_IO2_BIT_OFFSET: u8 = 9;
	const G3_IO2_BIT_WIDTH: u8 = 1;
	/// G3_IO2 channel mode (Width: 1, Offset: 9)
	pub fn get_g3_io2() -> u8 { ::read(REGISTER_ADDRESS, G3_IO2_BIT_OFFSET, G3_IO2_BIT_WIDTH) as u8 }
	/// G3_IO2 channel mode (Width: 1, Offset: 9)
	pub fn set_g3_io2(value: u8) { ::write(REGISTER_ADDRESS, G3_IO2_BIT_OFFSET, G3_IO2_BIT_WIDTH, value as u32); }

	const G3_IO3_BIT_OFFSET: u8 = 10;
	const G3_IO3_BIT_WIDTH: u8 = 1;
	/// G3_IO3 channel mode (Width: 1, Offset: 10)
	pub fn get_g3_io3() -> u8 { ::read(REGISTER_ADDRESS, G3_IO3_BIT_OFFSET, G3_IO3_BIT_WIDTH) as u8 }
	/// G3_IO3 channel mode (Width: 1, Offset: 10)
	pub fn set_g3_io3(value: u8) { ::write(REGISTER_ADDRESS, G3_IO3_BIT_OFFSET, G3_IO3_BIT_WIDTH, value as u32); }

	const G3_IO4_BIT_OFFSET: u8 = 11;
	const G3_IO4_BIT_WIDTH: u8 = 1;
	/// G3_IO4 channel mode (Width: 1, Offset: 11)
	pub fn get_g3_io4() -> u8 { ::read(REGISTER_ADDRESS, G3_IO4_BIT_OFFSET, G3_IO4_BIT_WIDTH) as u8 }
	/// G3_IO4 channel mode (Width: 1, Offset: 11)
	pub fn set_g3_io4(value: u8) { ::write(REGISTER_ADDRESS, G3_IO4_BIT_OFFSET, G3_IO4_BIT_WIDTH, value as u32); }

	const G4_IO1_BIT_OFFSET: u8 = 12;
	const G4_IO1_BIT_WIDTH: u8 = 1;
	/// G4_IO1 channel mode (Width: 1, Offset: 12)
	pub fn get_g4_io1() -> u8 { ::read(REGISTER_ADDRESS, G4_IO1_BIT_OFFSET, G4_IO1_BIT_WIDTH) as u8 }
	/// G4_IO1 channel mode (Width: 1, Offset: 12)
	pub fn set_g4_io1(value: u8) { ::write(REGISTER_ADDRESS, G4_IO1_BIT_OFFSET, G4_IO1_BIT_WIDTH, value as u32); }

	const G4_IO2_BIT_OFFSET: u8 = 13;
	const G4_IO2_BIT_WIDTH: u8 = 1;
	/// G4_IO2 channel mode (Width: 1, Offset: 13)
	pub fn get_g4_io2() -> u8 { ::read(REGISTER_ADDRESS, G4_IO2_BIT_OFFSET, G4_IO2_BIT_WIDTH) as u8 }
	/// G4_IO2 channel mode (Width: 1, Offset: 13)
	pub fn set_g4_io2(value: u8) { ::write(REGISTER_ADDRESS, G4_IO2_BIT_OFFSET, G4_IO2_BIT_WIDTH, value as u32); }

	const G4_IO3_BIT_OFFSET: u8 = 14;
	const G4_IO3_BIT_WIDTH: u8 = 1;
	/// G4_IO3 channel mode (Width: 1, Offset: 14)
	pub fn get_g4_io3() -> u8 { ::read(REGISTER_ADDRESS, G4_IO3_BIT_OFFSET, G4_IO3_BIT_WIDTH) as u8 }
	/// G4_IO3 channel mode (Width: 1, Offset: 14)
	pub fn set_g4_io3(value: u8) { ::write(REGISTER_ADDRESS, G4_IO3_BIT_OFFSET, G4_IO3_BIT_WIDTH, value as u32); }

	const G4_IO4_BIT_OFFSET: u8 = 15;
	const G4_IO4_BIT_WIDTH: u8 = 1;
	/// G4_IO4 channel mode (Width: 1, Offset: 15)
	pub fn get_g4_io4() -> u8 { ::read(REGISTER_ADDRESS, G4_IO4_BIT_OFFSET, G4_IO4_BIT_WIDTH) as u8 }
	/// G4_IO4 channel mode (Width: 1, Offset: 15)
	pub fn set_g4_io4(value: u8) { ::write(REGISTER_ADDRESS, G4_IO4_BIT_OFFSET, G4_IO4_BIT_WIDTH, value as u32); }

	const G5_IO1_BIT_OFFSET: u8 = 16;
	const G5_IO1_BIT_WIDTH: u8 = 1;
	/// G5_IO1 channel mode (Width: 1, Offset: 16)
	pub fn get_g5_io1() -> u8 { ::read(REGISTER_ADDRESS, G5_IO1_BIT_OFFSET, G5_IO1_BIT_WIDTH) as u8 }
	/// G5_IO1 channel mode (Width: 1, Offset: 16)
	pub fn set_g5_io1(value: u8) { ::write(REGISTER_ADDRESS, G5_IO1_BIT_OFFSET, G5_IO1_BIT_WIDTH, value as u32); }

	const G5_IO2_BIT_OFFSET: u8 = 17;
	const G5_IO2_BIT_WIDTH: u8 = 1;
	/// G5_IO2 channel mode (Width: 1, Offset: 17)
	pub fn get_g5_io2() -> u8 { ::read(REGISTER_ADDRESS, G5_IO2_BIT_OFFSET, G5_IO2_BIT_WIDTH) as u8 }
	/// G5_IO2 channel mode (Width: 1, Offset: 17)
	pub fn set_g5_io2(value: u8) { ::write(REGISTER_ADDRESS, G5_IO2_BIT_OFFSET, G5_IO2_BIT_WIDTH, value as u32); }

	const G5_IO3_BIT_OFFSET: u8 = 18;
	const G5_IO3_BIT_WIDTH: u8 = 1;
	/// G5_IO3 channel mode (Width: 1, Offset: 18)
	pub fn get_g5_io3() -> u8 { ::read(REGISTER_ADDRESS, G5_IO3_BIT_OFFSET, G5_IO3_BIT_WIDTH) as u8 }
	/// G5_IO3 channel mode (Width: 1, Offset: 18)
	pub fn set_g5_io3(value: u8) { ::write(REGISTER_ADDRESS, G5_IO3_BIT_OFFSET, G5_IO3_BIT_WIDTH, value as u32); }

	const G5_IO4_BIT_OFFSET: u8 = 19;
	const G5_IO4_BIT_WIDTH: u8 = 1;
	/// G5_IO4 channel mode (Width: 1, Offset: 19)
	pub fn get_g5_io4() -> u8 { ::read(REGISTER_ADDRESS, G5_IO4_BIT_OFFSET, G5_IO4_BIT_WIDTH) as u8 }
	/// G5_IO4 channel mode (Width: 1, Offset: 19)
	pub fn set_g5_io4(value: u8) { ::write(REGISTER_ADDRESS, G5_IO4_BIT_OFFSET, G5_IO4_BIT_WIDTH, value as u32); }

	const G6_IO1_BIT_OFFSET: u8 = 20;
	const G6_IO1_BIT_WIDTH: u8 = 1;
	/// G6_IO1 channel mode (Width: 1, Offset: 20)
	pub fn get_g6_io1() -> u8 { ::read(REGISTER_ADDRESS, G6_IO1_BIT_OFFSET, G6_IO1_BIT_WIDTH) as u8 }
	/// G6_IO1 channel mode (Width: 1, Offset: 20)
	pub fn set_g6_io1(value: u8) { ::write(REGISTER_ADDRESS, G6_IO1_BIT_OFFSET, G6_IO1_BIT_WIDTH, value as u32); }

	const G6_IO2_BIT_OFFSET: u8 = 21;
	const G6_IO2_BIT_WIDTH: u8 = 1;
	/// G6_IO2 channel mode (Width: 1, Offset: 21)
	pub fn get_g6_io2() -> u8 { ::read(REGISTER_ADDRESS, G6_IO2_BIT_OFFSET, G6_IO2_BIT_WIDTH) as u8 }
	/// G6_IO2 channel mode (Width: 1, Offset: 21)
	pub fn set_g6_io2(value: u8) { ::write(REGISTER_ADDRESS, G6_IO2_BIT_OFFSET, G6_IO2_BIT_WIDTH, value as u32); }

	const G6_IO3_BIT_OFFSET: u8 = 22;
	const G6_IO3_BIT_WIDTH: u8 = 1;
	/// G6_IO3 channel mode (Width: 1, Offset: 22)
	pub fn get_g6_io3() -> u8 { ::read(REGISTER_ADDRESS, G6_IO3_BIT_OFFSET, G6_IO3_BIT_WIDTH) as u8 }
	/// G6_IO3 channel mode (Width: 1, Offset: 22)
	pub fn set_g6_io3(value: u8) { ::write(REGISTER_ADDRESS, G6_IO3_BIT_OFFSET, G6_IO3_BIT_WIDTH, value as u32); }

	const G6_IO4_BIT_OFFSET: u8 = 23;
	const G6_IO4_BIT_WIDTH: u8 = 1;
	/// G6_IO4 channel mode (Width: 1, Offset: 23)
	pub fn get_g6_io4() -> u8 { ::read(REGISTER_ADDRESS, G6_IO4_BIT_OFFSET, G6_IO4_BIT_WIDTH) as u8 }
	/// G6_IO4 channel mode (Width: 1, Offset: 23)
	pub fn set_g6_io4(value: u8) { ::write(REGISTER_ADDRESS, G6_IO4_BIT_OFFSET, G6_IO4_BIT_WIDTH, value as u32); }

	const G7_IO1_BIT_OFFSET: u8 = 24;
	const G7_IO1_BIT_WIDTH: u8 = 1;
	/// G7_IO1 channel mode (Width: 1, Offset: 24)
	pub fn get_g7_io1() -> u8 { ::read(REGISTER_ADDRESS, G7_IO1_BIT_OFFSET, G7_IO1_BIT_WIDTH) as u8 }
	/// G7_IO1 channel mode (Width: 1, Offset: 24)
	pub fn set_g7_io1(value: u8) { ::write(REGISTER_ADDRESS, G7_IO1_BIT_OFFSET, G7_IO1_BIT_WIDTH, value as u32); }

	const G7_IO2_BIT_OFFSET: u8 = 25;
	const G7_IO2_BIT_WIDTH: u8 = 1;
	/// G7_IO2 channel mode (Width: 1, Offset: 25)
	pub fn get_g7_io2() -> u8 { ::read(REGISTER_ADDRESS, G7_IO2_BIT_OFFSET, G7_IO2_BIT_WIDTH) as u8 }
	/// G7_IO2 channel mode (Width: 1, Offset: 25)
	pub fn set_g7_io2(value: u8) { ::write(REGISTER_ADDRESS, G7_IO2_BIT_OFFSET, G7_IO2_BIT_WIDTH, value as u32); }

	const G7_IO3_BIT_OFFSET: u8 = 26;
	const G7_IO3_BIT_WIDTH: u8 = 1;
	/// G7_IO3 channel mode (Width: 1, Offset: 26)
	pub fn get_g7_io3() -> u8 { ::read(REGISTER_ADDRESS, G7_IO3_BIT_OFFSET, G7_IO3_BIT_WIDTH) as u8 }
	/// G7_IO3 channel mode (Width: 1, Offset: 26)
	pub fn set_g7_io3(value: u8) { ::write(REGISTER_ADDRESS, G7_IO3_BIT_OFFSET, G7_IO3_BIT_WIDTH, value as u32); }

	const G7_IO4_BIT_OFFSET: u8 = 27;
	const G7_IO4_BIT_WIDTH: u8 = 1;
	/// G7_IO4 channel mode (Width: 1, Offset: 27)
	pub fn get_g7_io4() -> u8 { ::read(REGISTER_ADDRESS, G7_IO4_BIT_OFFSET, G7_IO4_BIT_WIDTH) as u8 }
	/// G7_IO4 channel mode (Width: 1, Offset: 27)
	pub fn set_g7_io4(value: u8) { ::write(REGISTER_ADDRESS, G7_IO4_BIT_OFFSET, G7_IO4_BIT_WIDTH, value as u32); }

	const G8_IO1_BIT_OFFSET: u8 = 28;
	const G8_IO1_BIT_WIDTH: u8 = 1;
	/// G8_IO1 channel mode (Width: 1, Offset: 28)
	pub fn get_g8_io1() -> u8 { ::read(REGISTER_ADDRESS, G8_IO1_BIT_OFFSET, G8_IO1_BIT_WIDTH) as u8 }
	/// G8_IO1 channel mode (Width: 1, Offset: 28)
	pub fn set_g8_io1(value: u8) { ::write(REGISTER_ADDRESS, G8_IO1_BIT_OFFSET, G8_IO1_BIT_WIDTH, value as u32); }

	const G8_IO2_BIT_OFFSET: u8 = 29;
	const G8_IO2_BIT_WIDTH: u8 = 1;
	/// G8_IO2 channel mode (Width: 1, Offset: 29)
	pub fn get_g8_io2() -> u8 { ::read(REGISTER_ADDRESS, G8_IO2_BIT_OFFSET, G8_IO2_BIT_WIDTH) as u8 }
	/// G8_IO2 channel mode (Width: 1, Offset: 29)
	pub fn set_g8_io2(value: u8) { ::write(REGISTER_ADDRESS, G8_IO2_BIT_OFFSET, G8_IO2_BIT_WIDTH, value as u32); }

	const G8_IO3_BIT_OFFSET: u8 = 30;
	const G8_IO3_BIT_WIDTH: u8 = 1;
	/// G8_IO3 channel mode (Width: 1, Offset: 30)
	pub fn get_g8_io3() -> u8 { ::read(REGISTER_ADDRESS, G8_IO3_BIT_OFFSET, G8_IO3_BIT_WIDTH) as u8 }
	/// G8_IO3 channel mode (Width: 1, Offset: 30)
	pub fn set_g8_io3(value: u8) { ::write(REGISTER_ADDRESS, G8_IO3_BIT_OFFSET, G8_IO3_BIT_WIDTH, value as u32); }

	const G8_IO4_BIT_OFFSET: u8 = 31;
	const G8_IO4_BIT_WIDTH: u8 = 1;
	/// G8_IO4 channel mode (Width: 1, Offset: 31)
	pub fn get_g8_io4() -> u8 { ::read(REGISTER_ADDRESS, G8_IO4_BIT_OFFSET, G8_IO4_BIT_WIDTH) as u8 }
	/// G8_IO4 channel mode (Width: 1, Offset: 31)
	pub fn set_g8_io4(value: u8) { ::write(REGISTER_ADDRESS, G8_IO4_BIT_OFFSET, G8_IO4_BIT_WIDTH, value as u32); }
}
/// I/O group control status register
/// Size: 0x20 bits
pub mod iogcsr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x30;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const G8S_BIT_OFFSET: u8 = 23;
	const G8S_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x status (Width: 1, Offset: 23)
	pub fn get_g8s() -> u8 { ::read(REGISTER_ADDRESS, G8S_BIT_OFFSET, G8S_BIT_WIDTH) as u8 }
	/// Analog I/O group x status (Width: 1, Offset: 23)
	pub fn set_g8s(value: u8) { ::write(REGISTER_ADDRESS, G8S_BIT_OFFSET, G8S_BIT_WIDTH, value as u32); }

	const G7S_BIT_OFFSET: u8 = 22;
	const G7S_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x status (Width: 1, Offset: 22)
	pub fn get_g7s() -> u8 { ::read(REGISTER_ADDRESS, G7S_BIT_OFFSET, G7S_BIT_WIDTH) as u8 }
	/// Analog I/O group x status (Width: 1, Offset: 22)
	pub fn set_g7s(value: u8) { ::write(REGISTER_ADDRESS, G7S_BIT_OFFSET, G7S_BIT_WIDTH, value as u32); }

	const G6S_BIT_OFFSET: u8 = 21;
	const G6S_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x status (Width: 1, Offset: 21)
	pub fn get_g6s() -> u8 { ::read(REGISTER_ADDRESS, G6S_BIT_OFFSET, G6S_BIT_WIDTH) as u8 }

	const G5S_BIT_OFFSET: u8 = 20;
	const G5S_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x status (Width: 1, Offset: 20)
	pub fn get_g5s() -> u8 { ::read(REGISTER_ADDRESS, G5S_BIT_OFFSET, G5S_BIT_WIDTH) as u8 }

	const G4S_BIT_OFFSET: u8 = 19;
	const G4S_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x status (Width: 1, Offset: 19)
	pub fn get_g4s() -> u8 { ::read(REGISTER_ADDRESS, G4S_BIT_OFFSET, G4S_BIT_WIDTH) as u8 }

	const G3S_BIT_OFFSET: u8 = 18;
	const G3S_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x status (Width: 1, Offset: 18)
	pub fn get_g3s() -> u8 { ::read(REGISTER_ADDRESS, G3S_BIT_OFFSET, G3S_BIT_WIDTH) as u8 }

	const G2S_BIT_OFFSET: u8 = 17;
	const G2S_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x status (Width: 1, Offset: 17)
	pub fn get_g2s() -> u8 { ::read(REGISTER_ADDRESS, G2S_BIT_OFFSET, G2S_BIT_WIDTH) as u8 }

	const G1S_BIT_OFFSET: u8 = 16;
	const G1S_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x status (Width: 1, Offset: 16)
	pub fn get_g1s() -> u8 { ::read(REGISTER_ADDRESS, G1S_BIT_OFFSET, G1S_BIT_WIDTH) as u8 }

	const G8E_BIT_OFFSET: u8 = 7;
	const G8E_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x enable (Width: 1, Offset: 7)
	pub fn get_g8e() -> u8 { ::read(REGISTER_ADDRESS, G8E_BIT_OFFSET, G8E_BIT_WIDTH) as u8 }
	/// Analog I/O group x enable (Width: 1, Offset: 7)
	pub fn set_g8e(value: u8) { ::write(REGISTER_ADDRESS, G8E_BIT_OFFSET, G8E_BIT_WIDTH, value as u32); }

	const G7E_BIT_OFFSET: u8 = 6;
	const G7E_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x enable (Width: 1, Offset: 6)
	pub fn get_g7e() -> u8 { ::read(REGISTER_ADDRESS, G7E_BIT_OFFSET, G7E_BIT_WIDTH) as u8 }
	/// Analog I/O group x enable (Width: 1, Offset: 6)
	pub fn set_g7e(value: u8) { ::write(REGISTER_ADDRESS, G7E_BIT_OFFSET, G7E_BIT_WIDTH, value as u32); }

	const G6E_BIT_OFFSET: u8 = 5;
	const G6E_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x enable (Width: 1, Offset: 5)
	pub fn get_g6e() -> u8 { ::read(REGISTER_ADDRESS, G6E_BIT_OFFSET, G6E_BIT_WIDTH) as u8 }
	/// Analog I/O group x enable (Width: 1, Offset: 5)
	pub fn set_g6e(value: u8) { ::write(REGISTER_ADDRESS, G6E_BIT_OFFSET, G6E_BIT_WIDTH, value as u32); }

	const G5E_BIT_OFFSET: u8 = 4;
	const G5E_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x enable (Width: 1, Offset: 4)
	pub fn get_g5e() -> u8 { ::read(REGISTER_ADDRESS, G5E_BIT_OFFSET, G5E_BIT_WIDTH) as u8 }
	/// Analog I/O group x enable (Width: 1, Offset: 4)
	pub fn set_g5e(value: u8) { ::write(REGISTER_ADDRESS, G5E_BIT_OFFSET, G5E_BIT_WIDTH, value as u32); }

	const G4E_BIT_OFFSET: u8 = 3;
	const G4E_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x enable (Width: 1, Offset: 3)
	pub fn get_g4e() -> u8 { ::read(REGISTER_ADDRESS, G4E_BIT_OFFSET, G4E_BIT_WIDTH) as u8 }
	/// Analog I/O group x enable (Width: 1, Offset: 3)
	pub fn set_g4e(value: u8) { ::write(REGISTER_ADDRESS, G4E_BIT_OFFSET, G4E_BIT_WIDTH, value as u32); }

	const G3E_BIT_OFFSET: u8 = 2;
	const G3E_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x enable (Width: 1, Offset: 2)
	pub fn get_g3e() -> u8 { ::read(REGISTER_ADDRESS, G3E_BIT_OFFSET, G3E_BIT_WIDTH) as u8 }
	/// Analog I/O group x enable (Width: 1, Offset: 2)
	pub fn set_g3e(value: u8) { ::write(REGISTER_ADDRESS, G3E_BIT_OFFSET, G3E_BIT_WIDTH, value as u32); }

	const G2E_BIT_OFFSET: u8 = 1;
	const G2E_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x enable (Width: 1, Offset: 1)
	pub fn get_g2e() -> u8 { ::read(REGISTER_ADDRESS, G2E_BIT_OFFSET, G2E_BIT_WIDTH) as u8 }
	/// Analog I/O group x enable (Width: 1, Offset: 1)
	pub fn set_g2e(value: u8) { ::write(REGISTER_ADDRESS, G2E_BIT_OFFSET, G2E_BIT_WIDTH, value as u32); }

	const G1E_BIT_OFFSET: u8 = 0;
	const G1E_BIT_WIDTH: u8 = 1;
	/// Analog I/O group x enable (Width: 1, Offset: 0)
	pub fn get_g1e() -> u8 { ::read(REGISTER_ADDRESS, G1E_BIT_OFFSET, G1E_BIT_WIDTH) as u8 }
	/// Analog I/O group x enable (Width: 1, Offset: 0)
	pub fn set_g1e(value: u8) { ::write(REGISTER_ADDRESS, G1E_BIT_OFFSET, G1E_BIT_WIDTH, value as u32); }
}
/// I/O group x counter register
/// Size: 0x20 bits
pub mod iog1cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x34;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CNT_BIT_OFFSET: u8 = 0;
	const CNT_BIT_WIDTH: u8 = 14;
	/// Counter value (Width: 14, Offset: 0)
	pub fn get_cnt() -> u16 { ::read(REGISTER_ADDRESS, CNT_BIT_OFFSET, CNT_BIT_WIDTH) as u16 }
}
/// I/O group x counter register
/// Size: 0x20 bits
pub mod iog2cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x38;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CNT_BIT_OFFSET: u8 = 0;
	const CNT_BIT_WIDTH: u8 = 14;
	/// Counter value (Width: 14, Offset: 0)
	pub fn get_cnt() -> u16 { ::read(REGISTER_ADDRESS, CNT_BIT_OFFSET, CNT_BIT_WIDTH) as u16 }
}
/// I/O group x counter register
/// Size: 0x20 bits
pub mod iog3cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x3C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CNT_BIT_OFFSET: u8 = 0;
	const CNT_BIT_WIDTH: u8 = 14;
	/// Counter value (Width: 14, Offset: 0)
	pub fn get_cnt() -> u16 { ::read(REGISTER_ADDRESS, CNT_BIT_OFFSET, CNT_BIT_WIDTH) as u16 }
}
/// I/O group x counter register
/// Size: 0x20 bits
pub mod iog4cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x40;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CNT_BIT_OFFSET: u8 = 0;
	const CNT_BIT_WIDTH: u8 = 14;
	/// Counter value (Width: 14, Offset: 0)
	pub fn get_cnt() -> u16 { ::read(REGISTER_ADDRESS, CNT_BIT_OFFSET, CNT_BIT_WIDTH) as u16 }
}
/// I/O group x counter register
/// Size: 0x20 bits
pub mod iog5cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x44;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CNT_BIT_OFFSET: u8 = 0;
	const CNT_BIT_WIDTH: u8 = 14;
	/// Counter value (Width: 14, Offset: 0)
	pub fn get_cnt() -> u16 { ::read(REGISTER_ADDRESS, CNT_BIT_OFFSET, CNT_BIT_WIDTH) as u16 }
}
/// I/O group x counter register
/// Size: 0x20 bits
pub mod iog6cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x48;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CNT_BIT_OFFSET: u8 = 0;
	const CNT_BIT_WIDTH: u8 = 14;
	/// Counter value (Width: 14, Offset: 0)
	pub fn get_cnt() -> u16 { ::read(REGISTER_ADDRESS, CNT_BIT_OFFSET, CNT_BIT_WIDTH) as u16 }
}
/// I/O group x counter register
/// Size: 0x20 bits
pub mod iog7cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CNT_BIT_OFFSET: u8 = 0;
	const CNT_BIT_WIDTH: u8 = 14;
	/// Counter value (Width: 14, Offset: 0)
	pub fn get_cnt() -> u16 { ::read(REGISTER_ADDRESS, CNT_BIT_OFFSET, CNT_BIT_WIDTH) as u16 }
}
/// I/O group x counter register
/// Size: 0x20 bits
pub mod iog8cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x50;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CNT_BIT_OFFSET: u8 = 0;
	const CNT_BIT_WIDTH: u8 = 14;
	/// Counter value (Width: 14, Offset: 0)
	pub fn get_cnt() -> u16 { ::read(REGISTER_ADDRESS, CNT_BIT_OFFSET, CNT_BIT_WIDTH) as u16 }
}
/// EXTI Line2 and Touch sensing interrupts
pub const INTERRUPT_EXTI2_TSC: u32 = 8;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>TSC</name>
  <description>Touch sensing controller</description>
  <groupName>TSC</groupName>
  <baseAddress>0x40024000</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>CR</name>
      <displayName>CR</displayName>
      <description>control register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CTPH</name>
          <description>Charge transfer pulse high</description>
          <bitOffset>28</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>CTPL</name>
          <description>Charge transfer pulse low</description>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>SSD</name>
          <description>Spread spectrum deviation</description>
          <bitOffset>17</bitOffset>
          <bitWidth>7</bitWidth>
        </field>
        <field>
          <name>SSE</name>
          <description>Spread spectrum enable</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SSPSC</name>
          <description>Spread spectrum prescaler</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PGPSC</name>
          <description>pulse generator prescaler</description>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>MCV</name>
          <description>Max count value</description>
          <bitOffset>5</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>IODEF</name>
          <description>I/O Default mode</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SYNCPOL</name>
          <description>Synchronization pin
              polarity</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AM</name>
          <description>Acquisition mode</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>START</name>
          <description>Start a new acquisition</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TSCE</name>
          <description>Touch sensing controller
              enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IER</name>
      <displayName>IER</displayName>
      <description>interrupt enable register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MCEIE</name>
          <description>Max count error interrupt
              enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOAIE</name>
          <description>End of acquisition interrupt
              enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ICR</name>
      <displayName>ICR</displayName>
      <description>interrupt clear register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MCEIC</name>
          <description>Max count error interrupt
              clear</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOAIC</name>
          <description>End of acquisition interrupt
              clear</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ISR</name>
      <displayName>ISR</displayName>
      <description>interrupt status register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MCEF</name>
          <description>Max count error flag</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOAF</name>
          <description>End of acquisition flag</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IOHCR</name>
      <displayName>IOHCR</displayName>
      <description>I/O hysteresis control
          register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0xFFFFFFFF</resetValue>
      <fields>
        <field>
          <name>G1_IO1</name>
          <description>G1_IO1 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G1_IO2</name>
          <description>G1_IO2 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G1_IO3</name>
          <description>G1_IO3 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G1_IO4</name>
          <description>G1_IO4 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO1</name>
          <description>G2_IO1 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO2</name>
          <description>G2_IO2 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO3</name>
          <description>G2_IO3 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO4</name>
          <description>G2_IO4 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO1</name>
          <description>G3_IO1 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO2</name>
          <description>G3_IO2 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO3</name>
          <description>G3_IO3 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO4</name>
          <description>G3_IO4 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO1</name>
          <description>G4_IO1 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO2</name>
          <description>G4_IO2 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO3</name>
          <description>G4_IO3 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO4</name>
          <description>G4_IO4 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO1</name>
          <description>G5_IO1 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO2</name>
          <description>G5_IO2 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO3</name>
          <description>G5_IO3 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO4</name>
          <description>G5_IO4 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO1</name>
          <description>G6_IO1 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO2</name>
          <description>G6_IO2 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO3</name>
          <description>G6_IO3 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO4</name>
          <description>G6_IO4 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO1</name>
          <description>G7_IO1 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO2</name>
          <description>G7_IO2 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO3</name>
          <description>G7_IO3 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO4</name>
          <description>G7_IO4 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO1</name>
          <description>G8_IO1 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO2</name>
          <description>G8_IO2 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO3</name>
          <description>G8_IO3 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO4</name>
          <description>G8_IO4 Schmitt trigger hysteresis
              mode</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IOASCR</name>
      <displayName>IOASCR</displayName>
      <description>I/O analog switch control
          register</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>G1_IO1</name>
          <description>G1_IO1 analog switch
              enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G1_IO2</name>
          <description>G1_IO2 analog switch
              enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G1_IO3</name>
          <description>G1_IO3 analog switch
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G1_IO4</name>
          <description>G1_IO4 analog switch
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO1</name>
          <description>G2_IO1 analog switch
              enable</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO2</name>
          <description>G2_IO2 analog switch
              enable</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO3</name>
          <description>G2_IO3 analog switch
              enable</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO4</name>
          <description>G2_IO4 analog switch
              enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO1</name>
          <description>G3_IO1 analog switch
              enable</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO2</name>
          <description>G3_IO2 analog switch
              enable</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO3</name>
          <description>G3_IO3 analog switch
              enable</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO4</name>
          <description>G3_IO4 analog switch
              enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO1</name>
          <description>G4_IO1 analog switch
              enable</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO2</name>
          <description>G4_IO2 analog switch
              enable</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO3</name>
          <description>G4_IO3 analog switch
              enable</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO4</name>
          <description>G4_IO4 analog switch
              enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO1</name>
          <description>G5_IO1 analog switch
              enable</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO2</name>
          <description>G5_IO2 analog switch
              enable</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO3</name>
          <description>G5_IO3 analog switch
              enable</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO4</name>
          <description>G5_IO4 analog switch
              enable</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO1</name>
          <description>G6_IO1 analog switch
              enable</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO2</name>
          <description>G6_IO2 analog switch
              enable</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO3</name>
          <description>G6_IO3 analog switch
              enable</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO4</name>
          <description>G6_IO4 analog switch
              enable</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO1</name>
          <description>G7_IO1 analog switch
              enable</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO2</name>
          <description>G7_IO2 analog switch
              enable</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO3</name>
          <description>G7_IO3 analog switch
              enable</description>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO4</name>
          <description>G7_IO4 analog switch
              enable</description>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO1</name>
          <description>G8_IO1 analog switch
              enable</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO2</name>
          <description>G8_IO2 analog switch
              enable</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO3</name>
          <description>G8_IO3 analog switch
              enable</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO4</name>
          <description>G8_IO4 analog switch
              enable</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IOSCR</name>
      <displayName>IOSCR</displayName>
      <description>I/O sampling control register</description>
      <addressOffset>0x20</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>G1_IO1</name>
          <description>G1_IO1 sampling mode</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G1_IO2</name>
          <description>G1_IO2 sampling mode</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G1_IO3</name>
          <description>G1_IO3 sampling mode</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G1_IO4</name>
          <description>G1_IO4 sampling mode</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO1</name>
          <description>G2_IO1 sampling mode</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO2</name>
          <description>G2_IO2 sampling mode</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO3</name>
          <description>G2_IO3 sampling mode</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO4</name>
          <description>G2_IO4 sampling mode</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO1</name>
          <description>G3_IO1 sampling mode</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO2</name>
          <description>G3_IO2 sampling mode</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO3</name>
          <description>G3_IO3 sampling mode</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO4</name>
          <description>G3_IO4 sampling mode</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO1</name>
          <description>G4_IO1 sampling mode</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO2</name>
          <description>G4_IO2 sampling mode</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO3</name>
          <description>G4_IO3 sampling mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO4</name>
          <description>G4_IO4 sampling mode</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO1</name>
          <description>G5_IO1 sampling mode</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO2</name>
          <description>G5_IO2 sampling mode</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO3</name>
          <description>G5_IO3 sampling mode</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO4</name>
          <description>G5_IO4 sampling mode</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO1</name>
          <description>G6_IO1 sampling mode</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO2</name>
          <description>G6_IO2 sampling mode</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO3</name>
          <description>G6_IO3 sampling mode</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO4</name>
          <description>G6_IO4 sampling mode</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO1</name>
          <description>G7_IO1 sampling mode</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO2</name>
          <description>G7_IO2 sampling mode</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO3</name>
          <description>G7_IO3 sampling mode</description>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO4</name>
          <description>G7_IO4 sampling mode</description>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO1</name>
          <description>G8_IO1 sampling mode</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO2</name>
          <description>G8_IO2 sampling mode</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO3</name>
          <description>G8_IO3 sampling mode</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO4</name>
          <description>G8_IO4 sampling mode</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IOCCR</name>
      <displayName>IOCCR</displayName>
      <description>I/O channel control register</description>
      <addressOffset>0x28</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>G1_IO1</name>
          <description>G1_IO1 channel mode</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G1_IO2</name>
          <description>G1_IO2 channel mode</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G1_IO3</name>
          <description>G1_IO3 channel mode</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G1_IO4</name>
          <description>G1_IO4 channel mode</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO1</name>
          <description>G2_IO1 channel mode</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO2</name>
          <description>G2_IO2 channel mode</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO3</name>
          <description>G2_IO3 channel mode</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G2_IO4</name>
          <description>G2_IO4 channel mode</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO1</name>
          <description>G3_IO1 channel mode</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO2</name>
          <description>G3_IO2 channel mode</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO3</name>
          <description>G3_IO3 channel mode</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G3_IO4</name>
          <description>G3_IO4 channel mode</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO1</name>
          <description>G4_IO1 channel mode</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO2</name>
          <description>G4_IO2 channel mode</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO3</name>
          <description>G4_IO3 channel mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G4_IO4</name>
          <description>G4_IO4 channel mode</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO1</name>
          <description>G5_IO1 channel mode</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO2</name>
          <description>G5_IO2 channel mode</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO3</name>
          <description>G5_IO3 channel mode</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G5_IO4</name>
          <description>G5_IO4 channel mode</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO1</name>
          <description>G6_IO1 channel mode</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO2</name>
          <description>G6_IO2 channel mode</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO3</name>
          <description>G6_IO3 channel mode</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G6_IO4</name>
          <description>G6_IO4 channel mode</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO1</name>
          <description>G7_IO1 channel mode</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO2</name>
          <description>G7_IO2 channel mode</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO3</name>
          <description>G7_IO3 channel mode</description>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G7_IO4</name>
          <description>G7_IO4 channel mode</description>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO1</name>
          <description>G8_IO1 channel mode</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO2</name>
          <description>G8_IO2 channel mode</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO3</name>
          <description>G8_IO3 channel mode</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>G8_IO4</name>
          <description>G8_IO4 channel mode</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IOGCSR</name>
      <displayName>IOGCSR</displayName>
      <description>I/O group control status
          register</description>
      <addressOffset>0x30</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>G8S</name>
          <description>Analog I/O group x status</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>G7S</name>
          <description>Analog I/O group x status</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>G6S</name>
          <description>Analog I/O group x status</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>G5S</name>
          <description>Analog I/O group x status</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>G4S</name>
          <description>Analog I/O group x status</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>G3S</name>
          <description>Analog I/O group x status</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>G2S</name>
          <description>Analog I/O group x status</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>G1S</name>
          <description>Analog I/O group x status</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>G8E</name>
          <description>Analog I/O group x enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>G7E</name>
          <description>Analog I/O group x enable</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>G6E</name>
          <description>Analog I/O group x enable</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>G5E</name>
          <description>Analog I/O group x enable</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>G4E</name>
          <description>Analog I/O group x enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>G3E</name>
          <description>Analog I/O group x enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>G2E</name>
          <description>Analog I/O group x enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>G1E</name>
          <description>Analog I/O group x enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>IOG1CR</name>
      <displayName>IOG1CR</displayName>
      <description>I/O group x counter register</description>
      <addressOffset>0x34</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CNT</name>
          <description>Counter value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IOG2CR</name>
      <displayName>IOG2CR</displayName>
      <description>I/O group x counter register</description>
      <addressOffset>0x38</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CNT</name>
          <description>Counter value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IOG3CR</name>
      <displayName>IOG3CR</displayName>
      <description>I/O group x counter register</description>
      <addressOffset>0x3C</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CNT</name>
          <description>Counter value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IOG4CR</name>
      <displayName>IOG4CR</displayName>
      <description>I/O group x counter register</description>
      <addressOffset>0x40</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CNT</name>
          <description>Counter value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IOG5CR</name>
      <displayName>IOG5CR</displayName>
      <description>I/O group x counter register</description>
      <addressOffset>0x44</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CNT</name>
          <description>Counter value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IOG6CR</name>
      <displayName>IOG6CR</displayName>
      <description>I/O group x counter register</description>
      <addressOffset>0x48</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CNT</name>
          <description>Counter value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IOG7CR</name>
      <displayName>IOG7CR</displayName>
      <description>I/O group x counter register</description>
      <addressOffset>0x4C</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CNT</name>
          <description>Counter value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>IOG8CR</name>
      <displayName>IOG8CR</displayName>
      <description>I/O group x counter register</description>
      <addressOffset>0x50</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CNT</name>
          <description>Counter value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>14</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>EXTI2_TSC</name>
    <description>EXTI Line2 and Touch sensing
        interrupts</description>
    <value>8</value>
  </interrupt>
</peripheral>*/
