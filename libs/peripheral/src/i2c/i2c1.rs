/// MOD I2C1
/// Inter-integrated circuit
const BASE_ADDRESS: u32 = 0x40005400;
/// Control register 1
/// Size: 0x20 bits
pub mod cr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PE_BIT_OFFSET: u8 = 0;
	const PE_BIT_WIDTH: u8 = 1;
	/// Peripheral enable (Width: 1, Offset: 0)
	pub fn get_pe() -> u8 { ::read(REGISTER_ADDRESS, PE_BIT_OFFSET, PE_BIT_WIDTH) as u8 }
	/// Peripheral enable (Width: 1, Offset: 0)
	pub fn set_pe(value: u8) { ::write(REGISTER_ADDRESS, PE_BIT_OFFSET, PE_BIT_WIDTH, value as u32); }

	const TXIE_BIT_OFFSET: u8 = 1;
	const TXIE_BIT_WIDTH: u8 = 1;
	/// TX Interrupt enable (Width: 1, Offset: 1)
	pub fn get_txie() -> u8 { ::read(REGISTER_ADDRESS, TXIE_BIT_OFFSET, TXIE_BIT_WIDTH) as u8 }
	/// TX Interrupt enable (Width: 1, Offset: 1)
	pub fn set_txie(value: u8) { ::write(REGISTER_ADDRESS, TXIE_BIT_OFFSET, TXIE_BIT_WIDTH, value as u32); }

	const RXIE_BIT_OFFSET: u8 = 2;
	const RXIE_BIT_WIDTH: u8 = 1;
	/// RX Interrupt enable (Width: 1, Offset: 2)
	pub fn get_rxie() -> u8 { ::read(REGISTER_ADDRESS, RXIE_BIT_OFFSET, RXIE_BIT_WIDTH) as u8 }
	/// RX Interrupt enable (Width: 1, Offset: 2)
	pub fn set_rxie(value: u8) { ::write(REGISTER_ADDRESS, RXIE_BIT_OFFSET, RXIE_BIT_WIDTH, value as u32); }

	const ADDRIE_BIT_OFFSET: u8 = 3;
	const ADDRIE_BIT_WIDTH: u8 = 1;
	/// Address match interrupt enable (slave only) (Width: 1, Offset: 3)
	pub fn get_addrie() -> u8 { ::read(REGISTER_ADDRESS, ADDRIE_BIT_OFFSET, ADDRIE_BIT_WIDTH) as u8 }
	/// Address match interrupt enable (slave only) (Width: 1, Offset: 3)
	pub fn set_addrie(value: u8) { ::write(REGISTER_ADDRESS, ADDRIE_BIT_OFFSET, ADDRIE_BIT_WIDTH, value as u32); }

	const NACKIE_BIT_OFFSET: u8 = 4;
	const NACKIE_BIT_WIDTH: u8 = 1;
	/// Not acknowledge received interrupt enable (Width: 1, Offset: 4)
	pub fn get_nackie() -> u8 { ::read(REGISTER_ADDRESS, NACKIE_BIT_OFFSET, NACKIE_BIT_WIDTH) as u8 }
	/// Not acknowledge received interrupt enable (Width: 1, Offset: 4)
	pub fn set_nackie(value: u8) { ::write(REGISTER_ADDRESS, NACKIE_BIT_OFFSET, NACKIE_BIT_WIDTH, value as u32); }

	const STOPIE_BIT_OFFSET: u8 = 5;
	const STOPIE_BIT_WIDTH: u8 = 1;
	/// STOP detection Interrupt enable (Width: 1, Offset: 5)
	pub fn get_stopie() -> u8 { ::read(REGISTER_ADDRESS, STOPIE_BIT_OFFSET, STOPIE_BIT_WIDTH) as u8 }
	/// STOP detection Interrupt enable (Width: 1, Offset: 5)
	pub fn set_stopie(value: u8) { ::write(REGISTER_ADDRESS, STOPIE_BIT_OFFSET, STOPIE_BIT_WIDTH, value as u32); }

	const TCIE_BIT_OFFSET: u8 = 6;
	const TCIE_BIT_WIDTH: u8 = 1;
	/// Transfer Complete interrupt enable (Width: 1, Offset: 6)
	pub fn get_tcie() -> u8 { ::read(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH) as u8 }
	/// Transfer Complete interrupt enable (Width: 1, Offset: 6)
	pub fn set_tcie(value: u8) { ::write(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH, value as u32); }

	const ERRIE_BIT_OFFSET: u8 = 7;
	const ERRIE_BIT_WIDTH: u8 = 1;
	/// Error interrupts enable (Width: 1, Offset: 7)
	pub fn get_errie() -> u8 { ::read(REGISTER_ADDRESS, ERRIE_BIT_OFFSET, ERRIE_BIT_WIDTH) as u8 }
	/// Error interrupts enable (Width: 1, Offset: 7)
	pub fn set_errie(value: u8) { ::write(REGISTER_ADDRESS, ERRIE_BIT_OFFSET, ERRIE_BIT_WIDTH, value as u32); }

	const DNF_BIT_OFFSET: u8 = 8;
	const DNF_BIT_WIDTH: u8 = 4;
	/// Digital noise filter (Width: 4, Offset: 8)
	pub fn get_dnf() -> u8 { ::read(REGISTER_ADDRESS, DNF_BIT_OFFSET, DNF_BIT_WIDTH) as u8 }
	/// Digital noise filter (Width: 4, Offset: 8)
	pub fn set_dnf(value: u8) { ::write(REGISTER_ADDRESS, DNF_BIT_OFFSET, DNF_BIT_WIDTH, value as u32); }

	const ANFOFF_BIT_OFFSET: u8 = 12;
	const ANFOFF_BIT_WIDTH: u8 = 1;
	/// Analog noise filter OFF (Width: 1, Offset: 12)
	pub fn get_anfoff() -> u8 { ::read(REGISTER_ADDRESS, ANFOFF_BIT_OFFSET, ANFOFF_BIT_WIDTH) as u8 }
	/// Analog noise filter OFF (Width: 1, Offset: 12)
	pub fn set_anfoff(value: u8) { ::write(REGISTER_ADDRESS, ANFOFF_BIT_OFFSET, ANFOFF_BIT_WIDTH, value as u32); }

	const SWRST_BIT_OFFSET: u8 = 13;
	const SWRST_BIT_WIDTH: u8 = 1;
	/// Software reset (Width: 1, Offset: 13)
	pub fn set_swrst(value: u8) { ::write(REGISTER_ADDRESS, SWRST_BIT_OFFSET, SWRST_BIT_WIDTH, value as u32); }

	const TXDMAEN_BIT_OFFSET: u8 = 14;
	const TXDMAEN_BIT_WIDTH: u8 = 1;
	/// DMA transmission requests enable (Width: 1, Offset: 14)
	pub fn get_txdmaen() -> u8 { ::read(REGISTER_ADDRESS, TXDMAEN_BIT_OFFSET, TXDMAEN_BIT_WIDTH) as u8 }
	/// DMA transmission requests enable (Width: 1, Offset: 14)
	pub fn set_txdmaen(value: u8) { ::write(REGISTER_ADDRESS, TXDMAEN_BIT_OFFSET, TXDMAEN_BIT_WIDTH, value as u32); }

	const RXDMAEN_BIT_OFFSET: u8 = 15;
	const RXDMAEN_BIT_WIDTH: u8 = 1;
	/// DMA reception requests enable (Width: 1, Offset: 15)
	pub fn get_rxdmaen() -> u8 { ::read(REGISTER_ADDRESS, RXDMAEN_BIT_OFFSET, RXDMAEN_BIT_WIDTH) as u8 }
	/// DMA reception requests enable (Width: 1, Offset: 15)
	pub fn set_rxdmaen(value: u8) { ::write(REGISTER_ADDRESS, RXDMAEN_BIT_OFFSET, RXDMAEN_BIT_WIDTH, value as u32); }

	const SBC_BIT_OFFSET: u8 = 16;
	const SBC_BIT_WIDTH: u8 = 1;
	/// Slave byte control (Width: 1, Offset: 16)
	pub fn get_sbc() -> u8 { ::read(REGISTER_ADDRESS, SBC_BIT_OFFSET, SBC_BIT_WIDTH) as u8 }
	/// Slave byte control (Width: 1, Offset: 16)
	pub fn set_sbc(value: u8) { ::write(REGISTER_ADDRESS, SBC_BIT_OFFSET, SBC_BIT_WIDTH, value as u32); }

	const NOSTRETCH_BIT_OFFSET: u8 = 17;
	const NOSTRETCH_BIT_WIDTH: u8 = 1;
	/// Clock stretching disable (Width: 1, Offset: 17)
	pub fn get_nostretch() -> u8 { ::read(REGISTER_ADDRESS, NOSTRETCH_BIT_OFFSET, NOSTRETCH_BIT_WIDTH) as u8 }
	/// Clock stretching disable (Width: 1, Offset: 17)
	pub fn set_nostretch(value: u8) { ::write(REGISTER_ADDRESS, NOSTRETCH_BIT_OFFSET, NOSTRETCH_BIT_WIDTH, value as u32); }

	const WUPEN_BIT_OFFSET: u8 = 18;
	const WUPEN_BIT_WIDTH: u8 = 1;
	/// Wakeup from STOP enable (Width: 1, Offset: 18)
	pub fn get_wupen() -> u8 { ::read(REGISTER_ADDRESS, WUPEN_BIT_OFFSET, WUPEN_BIT_WIDTH) as u8 }
	/// Wakeup from STOP enable (Width: 1, Offset: 18)
	pub fn set_wupen(value: u8) { ::write(REGISTER_ADDRESS, WUPEN_BIT_OFFSET, WUPEN_BIT_WIDTH, value as u32); }

	const GCEN_BIT_OFFSET: u8 = 19;
	const GCEN_BIT_WIDTH: u8 = 1;
	/// General call enable (Width: 1, Offset: 19)
	pub fn get_gcen() -> u8 { ::read(REGISTER_ADDRESS, GCEN_BIT_OFFSET, GCEN_BIT_WIDTH) as u8 }
	/// General call enable (Width: 1, Offset: 19)
	pub fn set_gcen(value: u8) { ::write(REGISTER_ADDRESS, GCEN_BIT_OFFSET, GCEN_BIT_WIDTH, value as u32); }

	const SMBHEN_BIT_OFFSET: u8 = 20;
	const SMBHEN_BIT_WIDTH: u8 = 1;
	/// SMBus Host address enable (Width: 1, Offset: 20)
	pub fn get_smbhen() -> u8 { ::read(REGISTER_ADDRESS, SMBHEN_BIT_OFFSET, SMBHEN_BIT_WIDTH) as u8 }
	/// SMBus Host address enable (Width: 1, Offset: 20)
	pub fn set_smbhen(value: u8) { ::write(REGISTER_ADDRESS, SMBHEN_BIT_OFFSET, SMBHEN_BIT_WIDTH, value as u32); }

	const SMBDEN_BIT_OFFSET: u8 = 21;
	const SMBDEN_BIT_WIDTH: u8 = 1;
	/// SMBus Device Default address enable (Width: 1, Offset: 21)
	pub fn get_smbden() -> u8 { ::read(REGISTER_ADDRESS, SMBDEN_BIT_OFFSET, SMBDEN_BIT_WIDTH) as u8 }
	/// SMBus Device Default address enable (Width: 1, Offset: 21)
	pub fn set_smbden(value: u8) { ::write(REGISTER_ADDRESS, SMBDEN_BIT_OFFSET, SMBDEN_BIT_WIDTH, value as u32); }

	const ALERTEN_BIT_OFFSET: u8 = 22;
	const ALERTEN_BIT_WIDTH: u8 = 1;
	/// SMBUS alert enable (Width: 1, Offset: 22)
	pub fn get_alerten() -> u8 { ::read(REGISTER_ADDRESS, ALERTEN_BIT_OFFSET, ALERTEN_BIT_WIDTH) as u8 }
	/// SMBUS alert enable (Width: 1, Offset: 22)
	pub fn set_alerten(value: u8) { ::write(REGISTER_ADDRESS, ALERTEN_BIT_OFFSET, ALERTEN_BIT_WIDTH, value as u32); }

	const PECEN_BIT_OFFSET: u8 = 23;
	const PECEN_BIT_WIDTH: u8 = 1;
	/// PEC enable (Width: 1, Offset: 23)
	pub fn get_pecen() -> u8 { ::read(REGISTER_ADDRESS, PECEN_BIT_OFFSET, PECEN_BIT_WIDTH) as u8 }
	/// PEC enable (Width: 1, Offset: 23)
	pub fn set_pecen(value: u8) { ::write(REGISTER_ADDRESS, PECEN_BIT_OFFSET, PECEN_BIT_WIDTH, value as u32); }
}
/// Control register 2
/// Size: 0x20 bits
pub mod cr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PECBYTE_BIT_OFFSET: u8 = 26;
	const PECBYTE_BIT_WIDTH: u8 = 1;
	/// Packet error checking byte (Width: 1, Offset: 26)
	pub fn get_pecbyte() -> u8 { ::read(REGISTER_ADDRESS, PECBYTE_BIT_OFFSET, PECBYTE_BIT_WIDTH) as u8 }
	/// Packet error checking byte (Width: 1, Offset: 26)
	pub fn set_pecbyte(value: u8) { ::write(REGISTER_ADDRESS, PECBYTE_BIT_OFFSET, PECBYTE_BIT_WIDTH, value as u32); }

	const AUTOEND_BIT_OFFSET: u8 = 25;
	const AUTOEND_BIT_WIDTH: u8 = 1;
	/// Automatic end mode (master mode) (Width: 1, Offset: 25)
	pub fn get_autoend() -> u8 { ::read(REGISTER_ADDRESS, AUTOEND_BIT_OFFSET, AUTOEND_BIT_WIDTH) as u8 }
	/// Automatic end mode (master mode) (Width: 1, Offset: 25)
	pub fn set_autoend(value: u8) { ::write(REGISTER_ADDRESS, AUTOEND_BIT_OFFSET, AUTOEND_BIT_WIDTH, value as u32); }

	const RELOAD_BIT_OFFSET: u8 = 24;
	const RELOAD_BIT_WIDTH: u8 = 1;
	/// NBYTES reload mode (Width: 1, Offset: 24)
	pub fn get_reload() -> u8 { ::read(REGISTER_ADDRESS, RELOAD_BIT_OFFSET, RELOAD_BIT_WIDTH) as u8 }
	/// NBYTES reload mode (Width: 1, Offset: 24)
	pub fn set_reload(value: u8) { ::write(REGISTER_ADDRESS, RELOAD_BIT_OFFSET, RELOAD_BIT_WIDTH, value as u32); }

	const NBYTES_BIT_OFFSET: u8 = 16;
	const NBYTES_BIT_WIDTH: u8 = 8;
	/// Number of bytes (Width: 8, Offset: 16)
	pub fn get_nbytes() -> u8 { ::read(REGISTER_ADDRESS, NBYTES_BIT_OFFSET, NBYTES_BIT_WIDTH) as u8 }
	/// Number of bytes (Width: 8, Offset: 16)
	pub fn set_nbytes(value: u8) { ::write(REGISTER_ADDRESS, NBYTES_BIT_OFFSET, NBYTES_BIT_WIDTH, value as u32); }

	const NACK_BIT_OFFSET: u8 = 15;
	const NACK_BIT_WIDTH: u8 = 1;
	/// NACK generation (slave mode) (Width: 1, Offset: 15)
	pub fn get_nack() -> u8 { ::read(REGISTER_ADDRESS, NACK_BIT_OFFSET, NACK_BIT_WIDTH) as u8 }
	/// NACK generation (slave mode) (Width: 1, Offset: 15)
	pub fn set_nack(value: u8) { ::write(REGISTER_ADDRESS, NACK_BIT_OFFSET, NACK_BIT_WIDTH, value as u32); }

	const STOP_BIT_OFFSET: u8 = 14;
	const STOP_BIT_WIDTH: u8 = 1;
	/// Stop generation (master mode) (Width: 1, Offset: 14)
	pub fn get_stop() -> u8 { ::read(REGISTER_ADDRESS, STOP_BIT_OFFSET, STOP_BIT_WIDTH) as u8 }
	/// Stop generation (master mode) (Width: 1, Offset: 14)
	pub fn set_stop(value: u8) { ::write(REGISTER_ADDRESS, STOP_BIT_OFFSET, STOP_BIT_WIDTH, value as u32); }

	const START_BIT_OFFSET: u8 = 13;
	const START_BIT_WIDTH: u8 = 1;
	/// Start generation (Width: 1, Offset: 13)
	pub fn get_start() -> u8 { ::read(REGISTER_ADDRESS, START_BIT_OFFSET, START_BIT_WIDTH) as u8 }
	/// Start generation (Width: 1, Offset: 13)
	pub fn set_start(value: u8) { ::write(REGISTER_ADDRESS, START_BIT_OFFSET, START_BIT_WIDTH, value as u32); }

	const HEAD10R_BIT_OFFSET: u8 = 12;
	const HEAD10R_BIT_WIDTH: u8 = 1;
	/// 10-bit address header only read direction (master receiver mode) (Width: 1, Offset: 12)
	pub fn get_head10r() -> u8 { ::read(REGISTER_ADDRESS, HEAD10R_BIT_OFFSET, HEAD10R_BIT_WIDTH) as u8 }
	/// 10-bit address header only read direction (master receiver mode) (Width: 1, Offset: 12)
	pub fn set_head10r(value: u8) { ::write(REGISTER_ADDRESS, HEAD10R_BIT_OFFSET, HEAD10R_BIT_WIDTH, value as u32); }

	const ADD10_BIT_OFFSET: u8 = 11;
	const ADD10_BIT_WIDTH: u8 = 1;
	/// 10-bit addressing mode (master mode) (Width: 1, Offset: 11)
	pub fn get_add10() -> u8 { ::read(REGISTER_ADDRESS, ADD10_BIT_OFFSET, ADD10_BIT_WIDTH) as u8 }
	/// 10-bit addressing mode (master mode) (Width: 1, Offset: 11)
	pub fn set_add10(value: u8) { ::write(REGISTER_ADDRESS, ADD10_BIT_OFFSET, ADD10_BIT_WIDTH, value as u32); }

	const RD_WRN_BIT_OFFSET: u8 = 10;
	const RD_WRN_BIT_WIDTH: u8 = 1;
	/// Transfer direction (master mode) (Width: 1, Offset: 10)
	pub fn get_rd_wrn() -> u8 { ::read(REGISTER_ADDRESS, RD_WRN_BIT_OFFSET, RD_WRN_BIT_WIDTH) as u8 }
	/// Transfer direction (master mode) (Width: 1, Offset: 10)
	pub fn set_rd_wrn(value: u8) { ::write(REGISTER_ADDRESS, RD_WRN_BIT_OFFSET, RD_WRN_BIT_WIDTH, value as u32); }

	const SADD8_BIT_OFFSET: u8 = 8;
	const SADD8_BIT_WIDTH: u8 = 2;
	/// Slave address bit 9:8 (master mode) (Width: 2, Offset: 8)
	pub fn get_sadd8() -> u8 { ::read(REGISTER_ADDRESS, SADD8_BIT_OFFSET, SADD8_BIT_WIDTH) as u8 }
	/// Slave address bit 9:8 (master mode) (Width: 2, Offset: 8)
	pub fn set_sadd8(value: u8) { ::write(REGISTER_ADDRESS, SADD8_BIT_OFFSET, SADD8_BIT_WIDTH, value as u32); }

	const SADD1_BIT_OFFSET: u8 = 1;
	const SADD1_BIT_WIDTH: u8 = 7;
	/// Slave address bit 7:1 (master mode) (Width: 7, Offset: 1)
	pub fn get_sadd1() -> u8 { ::read(REGISTER_ADDRESS, SADD1_BIT_OFFSET, SADD1_BIT_WIDTH) as u8 }
	/// Slave address bit 7:1 (master mode) (Width: 7, Offset: 1)
	pub fn set_sadd1(value: u8) { ::write(REGISTER_ADDRESS, SADD1_BIT_OFFSET, SADD1_BIT_WIDTH, value as u32); }

	const SADD0_BIT_OFFSET: u8 = 0;
	const SADD0_BIT_WIDTH: u8 = 1;
	/// Slave address bit 0 (master mode) (Width: 1, Offset: 0)
	pub fn get_sadd0() -> u8 { ::read(REGISTER_ADDRESS, SADD0_BIT_OFFSET, SADD0_BIT_WIDTH) as u8 }
	/// Slave address bit 0 (master mode) (Width: 1, Offset: 0)
	pub fn set_sadd0(value: u8) { ::write(REGISTER_ADDRESS, SADD0_BIT_OFFSET, SADD0_BIT_WIDTH, value as u32); }
}
/// Own address register 1
/// Size: 0x20 bits
pub mod oar1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OA1_0_BIT_OFFSET: u8 = 0;
	const OA1_0_BIT_WIDTH: u8 = 1;
	/// Interface address (Width: 1, Offset: 0)
	pub fn get_oa1_0() -> u8 { ::read(REGISTER_ADDRESS, OA1_0_BIT_OFFSET, OA1_0_BIT_WIDTH) as u8 }
	/// Interface address (Width: 1, Offset: 0)
	pub fn set_oa1_0(value: u8) { ::write(REGISTER_ADDRESS, OA1_0_BIT_OFFSET, OA1_0_BIT_WIDTH, value as u32); }

	const OA1_1_BIT_OFFSET: u8 = 1;
	const OA1_1_BIT_WIDTH: u8 = 7;
	/// Interface address (Width: 7, Offset: 1)
	pub fn get_oa1_1() -> u8 { ::read(REGISTER_ADDRESS, OA1_1_BIT_OFFSET, OA1_1_BIT_WIDTH) as u8 }
	/// Interface address (Width: 7, Offset: 1)
	pub fn set_oa1_1(value: u8) { ::write(REGISTER_ADDRESS, OA1_1_BIT_OFFSET, OA1_1_BIT_WIDTH, value as u32); }

	const OA1_8_BIT_OFFSET: u8 = 8;
	const OA1_8_BIT_WIDTH: u8 = 2;
	/// Interface address (Width: 2, Offset: 8)
	pub fn get_oa1_8() -> u8 { ::read(REGISTER_ADDRESS, OA1_8_BIT_OFFSET, OA1_8_BIT_WIDTH) as u8 }
	/// Interface address (Width: 2, Offset: 8)
	pub fn set_oa1_8(value: u8) { ::write(REGISTER_ADDRESS, OA1_8_BIT_OFFSET, OA1_8_BIT_WIDTH, value as u32); }

	const OA1MODE_BIT_OFFSET: u8 = 10;
	const OA1MODE_BIT_WIDTH: u8 = 1;
	/// Own Address 1 10-bit mode (Width: 1, Offset: 10)
	pub fn get_oa1mode() -> u8 { ::read(REGISTER_ADDRESS, OA1MODE_BIT_OFFSET, OA1MODE_BIT_WIDTH) as u8 }
	/// Own Address 1 10-bit mode (Width: 1, Offset: 10)
	pub fn set_oa1mode(value: u8) { ::write(REGISTER_ADDRESS, OA1MODE_BIT_OFFSET, OA1MODE_BIT_WIDTH, value as u32); }

	const OA1EN_BIT_OFFSET: u8 = 15;
	const OA1EN_BIT_WIDTH: u8 = 1;
	/// Own Address 1 enable (Width: 1, Offset: 15)
	pub fn get_oa1en() -> u8 { ::read(REGISTER_ADDRESS, OA1EN_BIT_OFFSET, OA1EN_BIT_WIDTH) as u8 }
	/// Own Address 1 enable (Width: 1, Offset: 15)
	pub fn set_oa1en(value: u8) { ::write(REGISTER_ADDRESS, OA1EN_BIT_OFFSET, OA1EN_BIT_WIDTH, value as u32); }
}
/// Own address register 2
/// Size: 0x20 bits
pub mod oar2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OA2_BIT_OFFSET: u8 = 1;
	const OA2_BIT_WIDTH: u8 = 7;
	/// Interface address (Width: 7, Offset: 1)
	pub fn get_oa2() -> u8 { ::read(REGISTER_ADDRESS, OA2_BIT_OFFSET, OA2_BIT_WIDTH) as u8 }
	/// Interface address (Width: 7, Offset: 1)
	pub fn set_oa2(value: u8) { ::write(REGISTER_ADDRESS, OA2_BIT_OFFSET, OA2_BIT_WIDTH, value as u32); }

	const OA2MSK_BIT_OFFSET: u8 = 8;
	const OA2MSK_BIT_WIDTH: u8 = 3;
	/// Own Address 2 masks (Width: 3, Offset: 8)
	pub fn get_oa2msk() -> u8 { ::read(REGISTER_ADDRESS, OA2MSK_BIT_OFFSET, OA2MSK_BIT_WIDTH) as u8 }
	/// Own Address 2 masks (Width: 3, Offset: 8)
	pub fn set_oa2msk(value: u8) { ::write(REGISTER_ADDRESS, OA2MSK_BIT_OFFSET, OA2MSK_BIT_WIDTH, value as u32); }

	const OA2EN_BIT_OFFSET: u8 = 15;
	const OA2EN_BIT_WIDTH: u8 = 1;
	/// Own Address 2 enable (Width: 1, Offset: 15)
	pub fn get_oa2en() -> u8 { ::read(REGISTER_ADDRESS, OA2EN_BIT_OFFSET, OA2EN_BIT_WIDTH) as u8 }
	/// Own Address 2 enable (Width: 1, Offset: 15)
	pub fn set_oa2en(value: u8) { ::write(REGISTER_ADDRESS, OA2EN_BIT_OFFSET, OA2EN_BIT_WIDTH, value as u32); }
}
/// Timing register
/// Size: 0x20 bits
pub mod timingr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SCLL_BIT_OFFSET: u8 = 0;
	const SCLL_BIT_WIDTH: u8 = 8;
	/// SCL low period (master mode) (Width: 8, Offset: 0)
	pub fn get_scll() -> u8 { ::read(REGISTER_ADDRESS, SCLL_BIT_OFFSET, SCLL_BIT_WIDTH) as u8 }
	/// SCL low period (master mode) (Width: 8, Offset: 0)
	pub fn set_scll(value: u8) { ::write(REGISTER_ADDRESS, SCLL_BIT_OFFSET, SCLL_BIT_WIDTH, value as u32); }

	const SCLH_BIT_OFFSET: u8 = 8;
	const SCLH_BIT_WIDTH: u8 = 8;
	/// SCL high period (master mode) (Width: 8, Offset: 8)
	pub fn get_sclh() -> u8 { ::read(REGISTER_ADDRESS, SCLH_BIT_OFFSET, SCLH_BIT_WIDTH) as u8 }
	/// SCL high period (master mode) (Width: 8, Offset: 8)
	pub fn set_sclh(value: u8) { ::write(REGISTER_ADDRESS, SCLH_BIT_OFFSET, SCLH_BIT_WIDTH, value as u32); }

	const SDADEL_BIT_OFFSET: u8 = 16;
	const SDADEL_BIT_WIDTH: u8 = 4;
	/// Data hold time (Width: 4, Offset: 16)
	pub fn get_sdadel() -> u8 { ::read(REGISTER_ADDRESS, SDADEL_BIT_OFFSET, SDADEL_BIT_WIDTH) as u8 }
	/// Data hold time (Width: 4, Offset: 16)
	pub fn set_sdadel(value: u8) { ::write(REGISTER_ADDRESS, SDADEL_BIT_OFFSET, SDADEL_BIT_WIDTH, value as u32); }

	const SCLDEL_BIT_OFFSET: u8 = 20;
	const SCLDEL_BIT_WIDTH: u8 = 4;
	/// Data setup time (Width: 4, Offset: 20)
	pub fn get_scldel() -> u8 { ::read(REGISTER_ADDRESS, SCLDEL_BIT_OFFSET, SCLDEL_BIT_WIDTH) as u8 }
	/// Data setup time (Width: 4, Offset: 20)
	pub fn set_scldel(value: u8) { ::write(REGISTER_ADDRESS, SCLDEL_BIT_OFFSET, SCLDEL_BIT_WIDTH, value as u32); }

	const PRESC_BIT_OFFSET: u8 = 28;
	const PRESC_BIT_WIDTH: u8 = 4;
	/// Timing prescaler (Width: 4, Offset: 28)
	pub fn get_presc() -> u8 { ::read(REGISTER_ADDRESS, PRESC_BIT_OFFSET, PRESC_BIT_WIDTH) as u8 }
	/// Timing prescaler (Width: 4, Offset: 28)
	pub fn set_presc(value: u8) { ::write(REGISTER_ADDRESS, PRESC_BIT_OFFSET, PRESC_BIT_WIDTH, value as u32); }
}
/// Status register 1
/// Size: 0x20 bits
pub mod timeoutr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TIMEOUTA_BIT_OFFSET: u8 = 0;
	const TIMEOUTA_BIT_WIDTH: u8 = 12;
	/// Bus timeout A (Width: 12, Offset: 0)
	pub fn get_timeouta() -> u16 { ::read(REGISTER_ADDRESS, TIMEOUTA_BIT_OFFSET, TIMEOUTA_BIT_WIDTH) as u16 }
	/// Bus timeout A (Width: 12, Offset: 0)
	pub fn set_timeouta(value: u16) { ::write(REGISTER_ADDRESS, TIMEOUTA_BIT_OFFSET, TIMEOUTA_BIT_WIDTH, value as u32); }

	const TIDLE_BIT_OFFSET: u8 = 12;
	const TIDLE_BIT_WIDTH: u8 = 1;
	/// Idle clock timeout detection (Width: 1, Offset: 12)
	pub fn get_tidle() -> u8 { ::read(REGISTER_ADDRESS, TIDLE_BIT_OFFSET, TIDLE_BIT_WIDTH) as u8 }
	/// Idle clock timeout detection (Width: 1, Offset: 12)
	pub fn set_tidle(value: u8) { ::write(REGISTER_ADDRESS, TIDLE_BIT_OFFSET, TIDLE_BIT_WIDTH, value as u32); }

	const TIMOUTEN_BIT_OFFSET: u8 = 15;
	const TIMOUTEN_BIT_WIDTH: u8 = 1;
	/// Clock timeout enable (Width: 1, Offset: 15)
	pub fn get_timouten() -> u8 { ::read(REGISTER_ADDRESS, TIMOUTEN_BIT_OFFSET, TIMOUTEN_BIT_WIDTH) as u8 }
	/// Clock timeout enable (Width: 1, Offset: 15)
	pub fn set_timouten(value: u8) { ::write(REGISTER_ADDRESS, TIMOUTEN_BIT_OFFSET, TIMOUTEN_BIT_WIDTH, value as u32); }

	const TIMEOUTB_BIT_OFFSET: u8 = 16;
	const TIMEOUTB_BIT_WIDTH: u8 = 12;
	/// Bus timeout B (Width: 12, Offset: 16)
	pub fn get_timeoutb() -> u16 { ::read(REGISTER_ADDRESS, TIMEOUTB_BIT_OFFSET, TIMEOUTB_BIT_WIDTH) as u16 }
	/// Bus timeout B (Width: 12, Offset: 16)
	pub fn set_timeoutb(value: u16) { ::write(REGISTER_ADDRESS, TIMEOUTB_BIT_OFFSET, TIMEOUTB_BIT_WIDTH, value as u32); }

	const TEXTEN_BIT_OFFSET: u8 = 31;
	const TEXTEN_BIT_WIDTH: u8 = 1;
	/// Extended clock timeout enable (Width: 1, Offset: 31)
	pub fn get_texten() -> u8 { ::read(REGISTER_ADDRESS, TEXTEN_BIT_OFFSET, TEXTEN_BIT_WIDTH) as u8 }
	/// Extended clock timeout enable (Width: 1, Offset: 31)
	pub fn set_texten(value: u8) { ::write(REGISTER_ADDRESS, TEXTEN_BIT_OFFSET, TEXTEN_BIT_WIDTH, value as u32); }
}
/// Interrupt and Status register
/// Size: 0x20 bits
pub mod isr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ADDCODE_BIT_OFFSET: u8 = 17;
	const ADDCODE_BIT_WIDTH: u8 = 7;
	/// Address match code (Slave mode) (Width: 7, Offset: 17)
	pub fn get_addcode() -> u8 { ::read(REGISTER_ADDRESS, ADDCODE_BIT_OFFSET, ADDCODE_BIT_WIDTH) as u8 }

	const DIR_BIT_OFFSET: u8 = 16;
	const DIR_BIT_WIDTH: u8 = 1;
	/// Transfer direction (Slave mode) (Width: 1, Offset: 16)
	pub fn get_dir() -> u8 { ::read(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH) as u8 }

	const BUSY_BIT_OFFSET: u8 = 15;
	const BUSY_BIT_WIDTH: u8 = 1;
	/// Bus busy (Width: 1, Offset: 15)
	pub fn get_busy() -> u8 { ::read(REGISTER_ADDRESS, BUSY_BIT_OFFSET, BUSY_BIT_WIDTH) as u8 }

	const ALERT_BIT_OFFSET: u8 = 13;
	const ALERT_BIT_WIDTH: u8 = 1;
	/// SMBus alert (Width: 1, Offset: 13)
	pub fn get_alert() -> u8 { ::read(REGISTER_ADDRESS, ALERT_BIT_OFFSET, ALERT_BIT_WIDTH) as u8 }

	const TIMEOUT_BIT_OFFSET: u8 = 12;
	const TIMEOUT_BIT_WIDTH: u8 = 1;
	/// Timeout or t_low detection flag (Width: 1, Offset: 12)
	pub fn get_timeout() -> u8 { ::read(REGISTER_ADDRESS, TIMEOUT_BIT_OFFSET, TIMEOUT_BIT_WIDTH) as u8 }

	const PECERR_BIT_OFFSET: u8 = 11;
	const PECERR_BIT_WIDTH: u8 = 1;
	/// PEC Error in reception (Width: 1, Offset: 11)
	pub fn get_pecerr() -> u8 { ::read(REGISTER_ADDRESS, PECERR_BIT_OFFSET, PECERR_BIT_WIDTH) as u8 }

	const OVR_BIT_OFFSET: u8 = 10;
	const OVR_BIT_WIDTH: u8 = 1;
	/// Overrun/Underrun (slave mode) (Width: 1, Offset: 10)
	pub fn get_ovr() -> u8 { ::read(REGISTER_ADDRESS, OVR_BIT_OFFSET, OVR_BIT_WIDTH) as u8 }

	const ARLO_BIT_OFFSET: u8 = 9;
	const ARLO_BIT_WIDTH: u8 = 1;
	/// Arbitration lost (Width: 1, Offset: 9)
	pub fn get_arlo() -> u8 { ::read(REGISTER_ADDRESS, ARLO_BIT_OFFSET, ARLO_BIT_WIDTH) as u8 }

	const BERR_BIT_OFFSET: u8 = 8;
	const BERR_BIT_WIDTH: u8 = 1;
	/// Bus error (Width: 1, Offset: 8)
	pub fn get_berr() -> u8 { ::read(REGISTER_ADDRESS, BERR_BIT_OFFSET, BERR_BIT_WIDTH) as u8 }

	const TCR_BIT_OFFSET: u8 = 7;
	const TCR_BIT_WIDTH: u8 = 1;
	/// Transfer Complete Reload (Width: 1, Offset: 7)
	pub fn get_tcr() -> u8 { ::read(REGISTER_ADDRESS, TCR_BIT_OFFSET, TCR_BIT_WIDTH) as u8 }

	const TC_BIT_OFFSET: u8 = 6;
	const TC_BIT_WIDTH: u8 = 1;
	/// Transfer Complete (master mode) (Width: 1, Offset: 6)
	pub fn get_tc() -> u8 { ::read(REGISTER_ADDRESS, TC_BIT_OFFSET, TC_BIT_WIDTH) as u8 }

	const STOPF_BIT_OFFSET: u8 = 5;
	const STOPF_BIT_WIDTH: u8 = 1;
	/// Stop detection flag (Width: 1, Offset: 5)
	pub fn get_stopf() -> u8 { ::read(REGISTER_ADDRESS, STOPF_BIT_OFFSET, STOPF_BIT_WIDTH) as u8 }

	const NACKF_BIT_OFFSET: u8 = 4;
	const NACKF_BIT_WIDTH: u8 = 1;
	/// Not acknowledge received flag (Width: 1, Offset: 4)
	pub fn get_nackf() -> u8 { ::read(REGISTER_ADDRESS, NACKF_BIT_OFFSET, NACKF_BIT_WIDTH) as u8 }

	const ADDR_BIT_OFFSET: u8 = 3;
	const ADDR_BIT_WIDTH: u8 = 1;
	/// Address matched (slave mode) (Width: 1, Offset: 3)
	pub fn get_addr() -> u8 { ::read(REGISTER_ADDRESS, ADDR_BIT_OFFSET, ADDR_BIT_WIDTH) as u8 }

	const RXNE_BIT_OFFSET: u8 = 2;
	const RXNE_BIT_WIDTH: u8 = 1;
	/// Receive data register not empty (receivers) (Width: 1, Offset: 2)
	pub fn get_rxne() -> u8 { ::read(REGISTER_ADDRESS, RXNE_BIT_OFFSET, RXNE_BIT_WIDTH) as u8 }

	const TXIS_BIT_OFFSET: u8 = 1;
	const TXIS_BIT_WIDTH: u8 = 1;
	/// Transmit interrupt status (transmitters) (Width: 1, Offset: 1)
	pub fn get_txis() -> u8 { ::read(REGISTER_ADDRESS, TXIS_BIT_OFFSET, TXIS_BIT_WIDTH) as u8 }
	/// Transmit interrupt status (transmitters) (Width: 1, Offset: 1)
	pub fn set_txis(value: u8) { ::write(REGISTER_ADDRESS, TXIS_BIT_OFFSET, TXIS_BIT_WIDTH, value as u32); }

	const TXE_BIT_OFFSET: u8 = 0;
	const TXE_BIT_WIDTH: u8 = 1;
	/// Transmit data register empty (transmitters) (Width: 1, Offset: 0)
	pub fn get_txe() -> u8 { ::read(REGISTER_ADDRESS, TXE_BIT_OFFSET, TXE_BIT_WIDTH) as u8 }
	/// Transmit data register empty (transmitters) (Width: 1, Offset: 0)
	pub fn set_txe(value: u8) { ::write(REGISTER_ADDRESS, TXE_BIT_OFFSET, TXE_BIT_WIDTH, value as u32); }
}
/// Interrupt clear register
/// Size: 0x20 bits
pub mod icr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ALERTCF_BIT_OFFSET: u8 = 13;
	const ALERTCF_BIT_WIDTH: u8 = 1;
	/// Alert flag clear (Width: 1, Offset: 13)
	pub fn set_alertcf(value: u8) { ::write(REGISTER_ADDRESS, ALERTCF_BIT_OFFSET, ALERTCF_BIT_WIDTH, value as u32); }

	const TIMOUTCF_BIT_OFFSET: u8 = 12;
	const TIMOUTCF_BIT_WIDTH: u8 = 1;
	/// Timeout detection flag clear (Width: 1, Offset: 12)
	pub fn set_timoutcf(value: u8) { ::write(REGISTER_ADDRESS, TIMOUTCF_BIT_OFFSET, TIMOUTCF_BIT_WIDTH, value as u32); }

	const PECCF_BIT_OFFSET: u8 = 11;
	const PECCF_BIT_WIDTH: u8 = 1;
	/// PEC Error flag clear (Width: 1, Offset: 11)
	pub fn set_peccf(value: u8) { ::write(REGISTER_ADDRESS, PECCF_BIT_OFFSET, PECCF_BIT_WIDTH, value as u32); }

	const OVRCF_BIT_OFFSET: u8 = 10;
	const OVRCF_BIT_WIDTH: u8 = 1;
	/// Overrun/Underrun flag clear (Width: 1, Offset: 10)
	pub fn set_ovrcf(value: u8) { ::write(REGISTER_ADDRESS, OVRCF_BIT_OFFSET, OVRCF_BIT_WIDTH, value as u32); }

	const ARLOCF_BIT_OFFSET: u8 = 9;
	const ARLOCF_BIT_WIDTH: u8 = 1;
	/// Arbitration lost flag clear (Width: 1, Offset: 9)
	pub fn set_arlocf(value: u8) { ::write(REGISTER_ADDRESS, ARLOCF_BIT_OFFSET, ARLOCF_BIT_WIDTH, value as u32); }

	const BERRCF_BIT_OFFSET: u8 = 8;
	const BERRCF_BIT_WIDTH: u8 = 1;
	/// Bus error flag clear (Width: 1, Offset: 8)
	pub fn set_berrcf(value: u8) { ::write(REGISTER_ADDRESS, BERRCF_BIT_OFFSET, BERRCF_BIT_WIDTH, value as u32); }

	const STOPCF_BIT_OFFSET: u8 = 5;
	const STOPCF_BIT_WIDTH: u8 = 1;
	/// Stop detection flag clear (Width: 1, Offset: 5)
	pub fn set_stopcf(value: u8) { ::write(REGISTER_ADDRESS, STOPCF_BIT_OFFSET, STOPCF_BIT_WIDTH, value as u32); }

	const NACKCF_BIT_OFFSET: u8 = 4;
	const NACKCF_BIT_WIDTH: u8 = 1;
	/// Not Acknowledge flag clear (Width: 1, Offset: 4)
	pub fn set_nackcf(value: u8) { ::write(REGISTER_ADDRESS, NACKCF_BIT_OFFSET, NACKCF_BIT_WIDTH, value as u32); }

	const ADDRCF_BIT_OFFSET: u8 = 3;
	const ADDRCF_BIT_WIDTH: u8 = 1;
	/// Address Matched flag clear (Width: 1, Offset: 3)
	pub fn set_addrcf(value: u8) { ::write(REGISTER_ADDRESS, ADDRCF_BIT_OFFSET, ADDRCF_BIT_WIDTH, value as u32); }
}
/// PEC register
/// Size: 0x20 bits
pub mod pecr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x20;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PEC_BIT_OFFSET: u8 = 0;
	const PEC_BIT_WIDTH: u8 = 8;
	/// Packet error checking register (Width: 8, Offset: 0)
	pub fn get_pec() -> u8 { ::read(REGISTER_ADDRESS, PEC_BIT_OFFSET, PEC_BIT_WIDTH) as u8 }
}
/// Receive data register
/// Size: 0x20 bits
pub mod rxdr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x24;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const RXDATA_BIT_OFFSET: u8 = 0;
	const RXDATA_BIT_WIDTH: u8 = 8;
	/// 8-bit receive data (Width: 8, Offset: 0)
	pub fn get_rxdata() -> u8 { ::read(REGISTER_ADDRESS, RXDATA_BIT_OFFSET, RXDATA_BIT_WIDTH) as u8 }
}
/// Transmit data register
/// Size: 0x20 bits
pub mod txdr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x28;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TXDATA_BIT_OFFSET: u8 = 0;
	const TXDATA_BIT_WIDTH: u8 = 8;
	/// 8-bit transmit data (Width: 8, Offset: 0)
	pub fn get_txdata() -> u8 { ::read(REGISTER_ADDRESS, TXDATA_BIT_OFFSET, TXDATA_BIT_WIDTH) as u8 }
	/// 8-bit transmit data (Width: 8, Offset: 0)
	pub fn set_txdata(value: u8) { ::write(REGISTER_ADDRESS, TXDATA_BIT_OFFSET, TXDATA_BIT_WIDTH, value as u32); }
}
/// I2C1 event interrupt and EXTI Line23 interrupt
pub const INTERRUPT_I2C1_EV_EXTI23: u32 = 31;

/// I2C1 error interrupt
pub const INTERRUPT_I2C1_ER: u32 = 32;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>I2C1</name>
  <description>Inter-integrated circuit</description>
  <groupName>I2C</groupName>
  <baseAddress>0x40005400</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>CR1</name>
      <displayName>CR1</displayName>
      <description>Control register 1</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PE</name>
          <description>Peripheral enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TXIE</name>
          <description>TX Interrupt enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>RXIE</name>
          <description>RX Interrupt enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>ADDRIE</name>
          <description>Address match interrupt enable (slave
              only)</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>NACKIE</name>
          <description>Not acknowledge received interrupt
              enable</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>STOPIE</name>
          <description>STOP detection Interrupt
              enable</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TCIE</name>
          <description>Transfer Complete interrupt
              enable</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>ERRIE</name>
          <description>Error interrupts enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DNF</name>
          <description>Digital noise filter</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>ANFOFF</name>
          <description>Analog noise filter OFF</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SWRST</name>
          <description>Software reset</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <access>write-only</access>
        </field>
        <field>
          <name>TXDMAEN</name>
          <description>DMA transmission requests
              enable</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>RXDMAEN</name>
          <description>DMA reception requests
              enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SBC</name>
          <description>Slave byte control</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>NOSTRETCH</name>
          <description>Clock stretching disable</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>WUPEN</name>
          <description>Wakeup from STOP enable</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>GCEN</name>
          <description>General call enable</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SMBHEN</name>
          <description>SMBus Host address enable</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SMBDEN</name>
          <description>SMBus Device Default address
              enable</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>ALERTEN</name>
          <description>SMBUS alert enable</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PECEN</name>
          <description>PEC enable</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>CR2</name>
      <displayName>CR2</displayName>
      <description>Control register 2</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PECBYTE</name>
          <description>Packet error checking byte</description>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AUTOEND</name>
          <description>Automatic end mode (master
              mode)</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RELOAD</name>
          <description>NBYTES reload mode</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>NBYTES</name>
          <description>Number of bytes</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>NACK</name>
          <description>NACK generation (slave
              mode)</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>STOP</name>
          <description>Stop generation (master
              mode)</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>START</name>
          <description>Start generation</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HEAD10R</name>
          <description>10-bit address header only read
              direction (master receiver mode)</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADD10</name>
          <description>10-bit addressing mode (master
              mode)</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RD_WRN</name>
          <description>Transfer direction (master
              mode)</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SADD8</name>
          <description>Slave address bit 9:8 (master
              mode)</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>SADD1</name>
          <description>Slave address bit 7:1 (master
              mode)</description>
          <bitOffset>1</bitOffset>
          <bitWidth>7</bitWidth>
        </field>
        <field>
          <name>SADD0</name>
          <description>Slave address bit 0 (master
              mode)</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>OAR1</name>
      <displayName>OAR1</displayName>
      <description>Own address register 1</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OA1_0</name>
          <description>Interface address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OA1_1</name>
          <description>Interface address</description>
          <bitOffset>1</bitOffset>
          <bitWidth>7</bitWidth>
        </field>
        <field>
          <name>OA1_8</name>
          <description>Interface address</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OA1MODE</name>
          <description>Own Address 1 10-bit mode</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OA1EN</name>
          <description>Own Address 1 enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>OAR2</name>
      <displayName>OAR2</displayName>
      <description>Own address register 2</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OA2</name>
          <description>Interface address</description>
          <bitOffset>1</bitOffset>
          <bitWidth>7</bitWidth>
        </field>
        <field>
          <name>OA2MSK</name>
          <description>Own Address 2 masks</description>
          <bitOffset>8</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>OA2EN</name>
          <description>Own Address 2 enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>TIMINGR</name>
      <displayName>TIMINGR</displayName>
      <description>Timing register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SCLL</name>
          <description>SCL low period (master
              mode)</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>SCLH</name>
          <description>SCL high period (master
              mode)</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>SDADEL</name>
          <description>Data hold time</description>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>SCLDEL</name>
          <description>Data setup time</description>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>PRESC</name>
          <description>Timing prescaler</description>
          <bitOffset>28</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>TIMEOUTR</name>
      <displayName>TIMEOUTR</displayName>
      <description>Status register 1</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>TIMEOUTA</name>
          <description>Bus timeout A</description>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
        <field>
          <name>TIDLE</name>
          <description>Idle clock timeout
              detection</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIMOUTEN</name>
          <description>Clock timeout enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIMEOUTB</name>
          <description>Bus timeout B</description>
          <bitOffset>16</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
        <field>
          <name>TEXTEN</name>
          <description>Extended clock timeout
              enable</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ISR</name>
      <displayName>ISR</displayName>
      <description>Interrupt and Status register</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000001</resetValue>
      <fields>
        <field>
          <name>ADDCODE</name>
          <description>Address match code (Slave
              mode)</description>
          <bitOffset>17</bitOffset>
          <bitWidth>7</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>DIR</name>
          <description>Transfer direction (Slave
              mode)</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>BUSY</name>
          <description>Bus busy</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>ALERT</name>
          <description>SMBus alert</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>TIMEOUT</name>
          <description>Timeout or t_low detection
              flag</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>PECERR</name>
          <description>PEC Error in reception</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>OVR</name>
          <description>Overrun/Underrun (slave
              mode)</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>ARLO</name>
          <description>Arbitration lost</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>BERR</name>
          <description>Bus error</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>TCR</name>
          <description>Transfer Complete Reload</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>TC</name>
          <description>Transfer Complete (master
              mode)</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>STOPF</name>
          <description>Stop detection flag</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>NACKF</name>
          <description>Not acknowledge received
              flag</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>ADDR</name>
          <description>Address matched (slave
              mode)</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>RXNE</name>
          <description>Receive data register not empty
              (receivers)</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>TXIS</name>
          <description>Transmit interrupt status
              (transmitters)</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TXE</name>
          <description>Transmit data register empty
              (transmitters)</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>ICR</name>
      <displayName>ICR</displayName>
      <description>Interrupt clear register</description>
      <addressOffset>0x1C</addressOffset>
      <size>0x20</size>
      <access>write-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>ALERTCF</name>
          <description>Alert flag clear</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIMOUTCF</name>
          <description>Timeout detection flag
              clear</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PECCF</name>
          <description>PEC Error flag clear</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OVRCF</name>
          <description>Overrun/Underrun flag
              clear</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ARLOCF</name>
          <description>Arbitration lost flag
              clear</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BERRCF</name>
          <description>Bus error flag clear</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>STOPCF</name>
          <description>Stop detection flag clear</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>NACKCF</name>
          <description>Not Acknowledge flag clear</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADDRCF</name>
          <description>Address Matched flag clear</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>PECR</name>
      <displayName>PECR</displayName>
      <description>PEC register</description>
      <addressOffset>0x20</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PEC</name>
          <description>Packet error checking
              register</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>RXDR</name>
      <displayName>RXDR</displayName>
      <description>Receive data register</description>
      <addressOffset>0x24</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>RXDATA</name>
          <description>8-bit receive data</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>TXDR</name>
      <displayName>TXDR</displayName>
      <description>Transmit data register</description>
      <addressOffset>0x28</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>TXDATA</name>
          <description>8-bit transmit data</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>I2C1_EV_EXTI23</name>
    <description>I2C1 event interrupt and EXTI Line23
        interrupt</description>
    <value>31</value>
  </interrupt>
  <interrupt>
    <name>I2C1_ER</name>
    <description>I2C1 error interrupt</description>
    <value>32</value>
  </interrupt>
</peripheral>*/
