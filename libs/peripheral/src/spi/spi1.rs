/// MOD SPI1
/// Serial peripheral interface/Inter-IC sound
const BASE_ADDRESS: u32 = 0x40013000;
/// control register 1
/// Size: 0x20 bits
pub mod cr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BIDIMODE_BIT_OFFSET: u8 = 15;
	const BIDIMODE_BIT_WIDTH: u8 = 1;
	/// Bidirectional data mode enable (Width: 1, Offset: 15)
	pub fn get_bidimode() -> u8 { ::read(REGISTER_ADDRESS, BIDIMODE_BIT_OFFSET, BIDIMODE_BIT_WIDTH) as u8 }
	/// Bidirectional data mode enable (Width: 1, Offset: 15)
	pub fn set_bidimode(value: u8) { ::write(REGISTER_ADDRESS, BIDIMODE_BIT_OFFSET, BIDIMODE_BIT_WIDTH, value as u32); }

	const BIDIOE_BIT_OFFSET: u8 = 14;
	const BIDIOE_BIT_WIDTH: u8 = 1;
	/// Output enable in bidirectional mode (Width: 1, Offset: 14)
	pub fn get_bidioe() -> u8 { ::read(REGISTER_ADDRESS, BIDIOE_BIT_OFFSET, BIDIOE_BIT_WIDTH) as u8 }
	/// Output enable in bidirectional mode (Width: 1, Offset: 14)
	pub fn set_bidioe(value: u8) { ::write(REGISTER_ADDRESS, BIDIOE_BIT_OFFSET, BIDIOE_BIT_WIDTH, value as u32); }

	const CRCEN_BIT_OFFSET: u8 = 13;
	const CRCEN_BIT_WIDTH: u8 = 1;
	/// Hardware CRC calculation enable (Width: 1, Offset: 13)
	pub fn get_crcen() -> u8 { ::read(REGISTER_ADDRESS, CRCEN_BIT_OFFSET, CRCEN_BIT_WIDTH) as u8 }
	/// Hardware CRC calculation enable (Width: 1, Offset: 13)
	pub fn set_crcen(value: u8) { ::write(REGISTER_ADDRESS, CRCEN_BIT_OFFSET, CRCEN_BIT_WIDTH, value as u32); }

	const CRCNEXT_BIT_OFFSET: u8 = 12;
	const CRCNEXT_BIT_WIDTH: u8 = 1;
	/// CRC transfer next (Width: 1, Offset: 12)
	pub fn get_crcnext() -> u8 { ::read(REGISTER_ADDRESS, CRCNEXT_BIT_OFFSET, CRCNEXT_BIT_WIDTH) as u8 }
	/// CRC transfer next (Width: 1, Offset: 12)
	pub fn set_crcnext(value: u8) { ::write(REGISTER_ADDRESS, CRCNEXT_BIT_OFFSET, CRCNEXT_BIT_WIDTH, value as u32); }

	const DFF_BIT_OFFSET: u8 = 11;
	const DFF_BIT_WIDTH: u8 = 1;
	/// Data frame format (Width: 1, Offset: 11)
	pub fn get_dff() -> u8 { ::read(REGISTER_ADDRESS, DFF_BIT_OFFSET, DFF_BIT_WIDTH) as u8 }
	/// Data frame format (Width: 1, Offset: 11)
	pub fn set_dff(value: u8) { ::write(REGISTER_ADDRESS, DFF_BIT_OFFSET, DFF_BIT_WIDTH, value as u32); }

	const RXONLY_BIT_OFFSET: u8 = 10;
	const RXONLY_BIT_WIDTH: u8 = 1;
	/// Receive only (Width: 1, Offset: 10)
	pub fn get_rxonly() -> u8 { ::read(REGISTER_ADDRESS, RXONLY_BIT_OFFSET, RXONLY_BIT_WIDTH) as u8 }
	/// Receive only (Width: 1, Offset: 10)
	pub fn set_rxonly(value: u8) { ::write(REGISTER_ADDRESS, RXONLY_BIT_OFFSET, RXONLY_BIT_WIDTH, value as u32); }

	const SSM_BIT_OFFSET: u8 = 9;
	const SSM_BIT_WIDTH: u8 = 1;
	/// Software slave management (Width: 1, Offset: 9)
	pub fn get_ssm() -> u8 { ::read(REGISTER_ADDRESS, SSM_BIT_OFFSET, SSM_BIT_WIDTH) as u8 }
	/// Software slave management (Width: 1, Offset: 9)
	pub fn set_ssm(value: u8) { ::write(REGISTER_ADDRESS, SSM_BIT_OFFSET, SSM_BIT_WIDTH, value as u32); }

	const SSI_BIT_OFFSET: u8 = 8;
	const SSI_BIT_WIDTH: u8 = 1;
	/// Internal slave select (Width: 1, Offset: 8)
	pub fn get_ssi() -> u8 { ::read(REGISTER_ADDRESS, SSI_BIT_OFFSET, SSI_BIT_WIDTH) as u8 }
	/// Internal slave select (Width: 1, Offset: 8)
	pub fn set_ssi(value: u8) { ::write(REGISTER_ADDRESS, SSI_BIT_OFFSET, SSI_BIT_WIDTH, value as u32); }

	const LSBFIRST_BIT_OFFSET: u8 = 7;
	const LSBFIRST_BIT_WIDTH: u8 = 1;
	/// Frame format (Width: 1, Offset: 7)
	pub fn get_lsbfirst() -> u8 { ::read(REGISTER_ADDRESS, LSBFIRST_BIT_OFFSET, LSBFIRST_BIT_WIDTH) as u8 }
	/// Frame format (Width: 1, Offset: 7)
	pub fn set_lsbfirst(value: u8) { ::write(REGISTER_ADDRESS, LSBFIRST_BIT_OFFSET, LSBFIRST_BIT_WIDTH, value as u32); }

	const SPE_BIT_OFFSET: u8 = 6;
	const SPE_BIT_WIDTH: u8 = 1;
	/// SPI enable (Width: 1, Offset: 6)
	pub fn get_spe() -> u8 { ::read(REGISTER_ADDRESS, SPE_BIT_OFFSET, SPE_BIT_WIDTH) as u8 }
	/// SPI enable (Width: 1, Offset: 6)
	pub fn set_spe(value: u8) { ::write(REGISTER_ADDRESS, SPE_BIT_OFFSET, SPE_BIT_WIDTH, value as u32); }

	const BR_BIT_OFFSET: u8 = 3;
	const BR_BIT_WIDTH: u8 = 3;
	/// Baud rate control (Width: 3, Offset: 3)
	pub fn get_br() -> u8 { ::read(REGISTER_ADDRESS, BR_BIT_OFFSET, BR_BIT_WIDTH) as u8 }
	/// Baud rate control (Width: 3, Offset: 3)
	pub fn set_br(value: u8) { ::write(REGISTER_ADDRESS, BR_BIT_OFFSET, BR_BIT_WIDTH, value as u32); }

	const MSTR_BIT_OFFSET: u8 = 2;
	const MSTR_BIT_WIDTH: u8 = 1;
	/// Master selection (Width: 1, Offset: 2)
	pub fn get_mstr() -> u8 { ::read(REGISTER_ADDRESS, MSTR_BIT_OFFSET, MSTR_BIT_WIDTH) as u8 }
	/// Master selection (Width: 1, Offset: 2)
	pub fn set_mstr(value: u8) { ::write(REGISTER_ADDRESS, MSTR_BIT_OFFSET, MSTR_BIT_WIDTH, value as u32); }

	const CPOL_BIT_OFFSET: u8 = 1;
	const CPOL_BIT_WIDTH: u8 = 1;
	/// Clock polarity (Width: 1, Offset: 1)
	pub fn get_cpol() -> u8 { ::read(REGISTER_ADDRESS, CPOL_BIT_OFFSET, CPOL_BIT_WIDTH) as u8 }
	/// Clock polarity (Width: 1, Offset: 1)
	pub fn set_cpol(value: u8) { ::write(REGISTER_ADDRESS, CPOL_BIT_OFFSET, CPOL_BIT_WIDTH, value as u32); }

	const CPHA_BIT_OFFSET: u8 = 0;
	const CPHA_BIT_WIDTH: u8 = 1;
	/// Clock phase (Width: 1, Offset: 0)
	pub fn get_cpha() -> u8 { ::read(REGISTER_ADDRESS, CPHA_BIT_OFFSET, CPHA_BIT_WIDTH) as u8 }
	/// Clock phase (Width: 1, Offset: 0)
	pub fn set_cpha(value: u8) { ::write(REGISTER_ADDRESS, CPHA_BIT_OFFSET, CPHA_BIT_WIDTH, value as u32); }
}
/// control register 2
/// Size: 0x20 bits
pub mod cr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const RXDMAEN_BIT_OFFSET: u8 = 0;
	const RXDMAEN_BIT_WIDTH: u8 = 1;
	/// Rx buffer DMA enable (Width: 1, Offset: 0)
	pub fn get_rxdmaen() -> u8 { ::read(REGISTER_ADDRESS, RXDMAEN_BIT_OFFSET, RXDMAEN_BIT_WIDTH) as u8 }
	/// Rx buffer DMA enable (Width: 1, Offset: 0)
	pub fn set_rxdmaen(value: u8) { ::write(REGISTER_ADDRESS, RXDMAEN_BIT_OFFSET, RXDMAEN_BIT_WIDTH, value as u32); }

	const TXDMAEN_BIT_OFFSET: u8 = 1;
	const TXDMAEN_BIT_WIDTH: u8 = 1;
	/// Tx buffer DMA enable (Width: 1, Offset: 1)
	pub fn get_txdmaen() -> u8 { ::read(REGISTER_ADDRESS, TXDMAEN_BIT_OFFSET, TXDMAEN_BIT_WIDTH) as u8 }
	/// Tx buffer DMA enable (Width: 1, Offset: 1)
	pub fn set_txdmaen(value: u8) { ::write(REGISTER_ADDRESS, TXDMAEN_BIT_OFFSET, TXDMAEN_BIT_WIDTH, value as u32); }

	const SSOE_BIT_OFFSET: u8 = 2;
	const SSOE_BIT_WIDTH: u8 = 1;
	/// SS output enable (Width: 1, Offset: 2)
	pub fn get_ssoe() -> u8 { ::read(REGISTER_ADDRESS, SSOE_BIT_OFFSET, SSOE_BIT_WIDTH) as u8 }
	/// SS output enable (Width: 1, Offset: 2)
	pub fn set_ssoe(value: u8) { ::write(REGISTER_ADDRESS, SSOE_BIT_OFFSET, SSOE_BIT_WIDTH, value as u32); }

	const NSSP_BIT_OFFSET: u8 = 3;
	const NSSP_BIT_WIDTH: u8 = 1;
	/// NSS pulse management (Width: 1, Offset: 3)
	pub fn get_nssp() -> u8 { ::read(REGISTER_ADDRESS, NSSP_BIT_OFFSET, NSSP_BIT_WIDTH) as u8 }
	/// NSS pulse management (Width: 1, Offset: 3)
	pub fn set_nssp(value: u8) { ::write(REGISTER_ADDRESS, NSSP_BIT_OFFSET, NSSP_BIT_WIDTH, value as u32); }

	const FRF_BIT_OFFSET: u8 = 4;
	const FRF_BIT_WIDTH: u8 = 1;
	/// Frame format (Width: 1, Offset: 4)
	pub fn get_frf() -> u8 { ::read(REGISTER_ADDRESS, FRF_BIT_OFFSET, FRF_BIT_WIDTH) as u8 }
	/// Frame format (Width: 1, Offset: 4)
	pub fn set_frf(value: u8) { ::write(REGISTER_ADDRESS, FRF_BIT_OFFSET, FRF_BIT_WIDTH, value as u32); }

	const ERRIE_BIT_OFFSET: u8 = 5;
	const ERRIE_BIT_WIDTH: u8 = 1;
	/// Error interrupt enable (Width: 1, Offset: 5)
	pub fn get_errie() -> u8 { ::read(REGISTER_ADDRESS, ERRIE_BIT_OFFSET, ERRIE_BIT_WIDTH) as u8 }
	/// Error interrupt enable (Width: 1, Offset: 5)
	pub fn set_errie(value: u8) { ::write(REGISTER_ADDRESS, ERRIE_BIT_OFFSET, ERRIE_BIT_WIDTH, value as u32); }

	const RXNEIE_BIT_OFFSET: u8 = 6;
	const RXNEIE_BIT_WIDTH: u8 = 1;
	/// RX buffer not empty interrupt enable (Width: 1, Offset: 6)
	pub fn get_rxneie() -> u8 { ::read(REGISTER_ADDRESS, RXNEIE_BIT_OFFSET, RXNEIE_BIT_WIDTH) as u8 }
	/// RX buffer not empty interrupt enable (Width: 1, Offset: 6)
	pub fn set_rxneie(value: u8) { ::write(REGISTER_ADDRESS, RXNEIE_BIT_OFFSET, RXNEIE_BIT_WIDTH, value as u32); }

	const TXEIE_BIT_OFFSET: u8 = 7;
	const TXEIE_BIT_WIDTH: u8 = 1;
	/// Tx buffer empty interrupt enable (Width: 1, Offset: 7)
	pub fn get_txeie() -> u8 { ::read(REGISTER_ADDRESS, TXEIE_BIT_OFFSET, TXEIE_BIT_WIDTH) as u8 }
	/// Tx buffer empty interrupt enable (Width: 1, Offset: 7)
	pub fn set_txeie(value: u8) { ::write(REGISTER_ADDRESS, TXEIE_BIT_OFFSET, TXEIE_BIT_WIDTH, value as u32); }

	const DS_BIT_OFFSET: u8 = 8;
	const DS_BIT_WIDTH: u8 = 4;
	/// Data size (Width: 4, Offset: 8)
	pub fn get_ds() -> u8 { ::read(REGISTER_ADDRESS, DS_BIT_OFFSET, DS_BIT_WIDTH) as u8 }
	/// Data size (Width: 4, Offset: 8)
	pub fn set_ds(value: u8) { ::write(REGISTER_ADDRESS, DS_BIT_OFFSET, DS_BIT_WIDTH, value as u32); }

	const FRXTH_BIT_OFFSET: u8 = 12;
	const FRXTH_BIT_WIDTH: u8 = 1;
	/// FIFO reception threshold (Width: 1, Offset: 12)
	pub fn get_frxth() -> u8 { ::read(REGISTER_ADDRESS, FRXTH_BIT_OFFSET, FRXTH_BIT_WIDTH) as u8 }
	/// FIFO reception threshold (Width: 1, Offset: 12)
	pub fn set_frxth(value: u8) { ::write(REGISTER_ADDRESS, FRXTH_BIT_OFFSET, FRXTH_BIT_WIDTH, value as u32); }

	const LDMA_RX_BIT_OFFSET: u8 = 13;
	const LDMA_RX_BIT_WIDTH: u8 = 1;
	/// Last DMA transfer for reception (Width: 1, Offset: 13)
	pub fn get_ldma_rx() -> u8 { ::read(REGISTER_ADDRESS, LDMA_RX_BIT_OFFSET, LDMA_RX_BIT_WIDTH) as u8 }
	/// Last DMA transfer for reception (Width: 1, Offset: 13)
	pub fn set_ldma_rx(value: u8) { ::write(REGISTER_ADDRESS, LDMA_RX_BIT_OFFSET, LDMA_RX_BIT_WIDTH, value as u32); }

	const LDMA_TX_BIT_OFFSET: u8 = 14;
	const LDMA_TX_BIT_WIDTH: u8 = 1;
	/// Last DMA transfer for transmission (Width: 1, Offset: 14)
	pub fn get_ldma_tx() -> u8 { ::read(REGISTER_ADDRESS, LDMA_TX_BIT_OFFSET, LDMA_TX_BIT_WIDTH) as u8 }
	/// Last DMA transfer for transmission (Width: 1, Offset: 14)
	pub fn set_ldma_tx(value: u8) { ::write(REGISTER_ADDRESS, LDMA_TX_BIT_OFFSET, LDMA_TX_BIT_WIDTH, value as u32); }
}
/// status register
/// Size: 0x20 bits
pub mod sr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const RXNE_BIT_OFFSET: u8 = 0;
	const RXNE_BIT_WIDTH: u8 = 1;
	/// Receive buffer not empty (Width: 1, Offset: 0)
	pub fn get_rxne() -> u8 { ::read(REGISTER_ADDRESS, RXNE_BIT_OFFSET, RXNE_BIT_WIDTH) as u8 }

	const TXE_BIT_OFFSET: u8 = 1;
	const TXE_BIT_WIDTH: u8 = 1;
	/// Transmit buffer empty (Width: 1, Offset: 1)
	pub fn get_txe() -> u8 { ::read(REGISTER_ADDRESS, TXE_BIT_OFFSET, TXE_BIT_WIDTH) as u8 }

	const CHSIDE_BIT_OFFSET: u8 = 2;
	const CHSIDE_BIT_WIDTH: u8 = 1;
	/// Channel side (Width: 1, Offset: 2)
	pub fn get_chside() -> u8 { ::read(REGISTER_ADDRESS, CHSIDE_BIT_OFFSET, CHSIDE_BIT_WIDTH) as u8 }

	const UDR_BIT_OFFSET: u8 = 3;
	const UDR_BIT_WIDTH: u8 = 1;
	/// Underrun flag (Width: 1, Offset: 3)
	pub fn get_udr() -> u8 { ::read(REGISTER_ADDRESS, UDR_BIT_OFFSET, UDR_BIT_WIDTH) as u8 }

	const CRCERR_BIT_OFFSET: u8 = 4;
	const CRCERR_BIT_WIDTH: u8 = 1;
	/// CRC error flag (Width: 1, Offset: 4)
	pub fn get_crcerr() -> u8 { ::read(REGISTER_ADDRESS, CRCERR_BIT_OFFSET, CRCERR_BIT_WIDTH) as u8 }
	/// CRC error flag (Width: 1, Offset: 4)
	pub fn set_crcerr(value: u8) { ::write(REGISTER_ADDRESS, CRCERR_BIT_OFFSET, CRCERR_BIT_WIDTH, value as u32); }

	const MODF_BIT_OFFSET: u8 = 5;
	const MODF_BIT_WIDTH: u8 = 1;
	/// Mode fault (Width: 1, Offset: 5)
	pub fn get_modf() -> u8 { ::read(REGISTER_ADDRESS, MODF_BIT_OFFSET, MODF_BIT_WIDTH) as u8 }

	const OVR_BIT_OFFSET: u8 = 6;
	const OVR_BIT_WIDTH: u8 = 1;
	/// Overrun flag (Width: 1, Offset: 6)
	pub fn get_ovr() -> u8 { ::read(REGISTER_ADDRESS, OVR_BIT_OFFSET, OVR_BIT_WIDTH) as u8 }

	const BSY_BIT_OFFSET: u8 = 7;
	const BSY_BIT_WIDTH: u8 = 1;
	/// Busy flag (Width: 1, Offset: 7)
	pub fn get_bsy() -> u8 { ::read(REGISTER_ADDRESS, BSY_BIT_OFFSET, BSY_BIT_WIDTH) as u8 }

	const TIFRFE_BIT_OFFSET: u8 = 8;
	const TIFRFE_BIT_WIDTH: u8 = 1;
	/// TI frame format error (Width: 1, Offset: 8)
	pub fn get_tifrfe() -> u8 { ::read(REGISTER_ADDRESS, TIFRFE_BIT_OFFSET, TIFRFE_BIT_WIDTH) as u8 }

	const FRLVL_BIT_OFFSET: u8 = 9;
	const FRLVL_BIT_WIDTH: u8 = 2;
	/// FIFO reception level (Width: 2, Offset: 9)
	pub fn get_frlvl() -> u8 { ::read(REGISTER_ADDRESS, FRLVL_BIT_OFFSET, FRLVL_BIT_WIDTH) as u8 }

	const FTLVL_BIT_OFFSET: u8 = 11;
	const FTLVL_BIT_WIDTH: u8 = 2;
	/// FIFO transmission level (Width: 2, Offset: 11)
	pub fn get_ftlvl() -> u8 { ::read(REGISTER_ADDRESS, FTLVL_BIT_OFFSET, FTLVL_BIT_WIDTH) as u8 }
}
/// data register
/// Size: 0x20 bits
pub mod dr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DR_BIT_OFFSET: u8 = 0;
	const DR_BIT_WIDTH: u8 = 16;
	/// Data register (Width: 16, Offset: 0)
	pub fn get_dr() -> u16 { ::read(REGISTER_ADDRESS, DR_BIT_OFFSET, DR_BIT_WIDTH) as u16 }
	/// Data register (Width: 16, Offset: 0)
	pub fn set_dr(value: u16) { ::write(REGISTER_ADDRESS, DR_BIT_OFFSET, DR_BIT_WIDTH, value as u32); }
}
/// CRC polynomial register
/// Size: 0x20 bits
pub mod crcpr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CRCPOLY_BIT_OFFSET: u8 = 0;
	const CRCPOLY_BIT_WIDTH: u8 = 16;
	/// CRC polynomial register (Width: 16, Offset: 0)
	pub fn get_crcpoly() -> u16 { ::read(REGISTER_ADDRESS, CRCPOLY_BIT_OFFSET, CRCPOLY_BIT_WIDTH) as u16 }
	/// CRC polynomial register (Width: 16, Offset: 0)
	pub fn set_crcpoly(value: u16) { ::write(REGISTER_ADDRESS, CRCPOLY_BIT_OFFSET, CRCPOLY_BIT_WIDTH, value as u32); }
}
/// RX CRC register
/// Size: 0x20 bits
pub mod rxcrcr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const RxCRC_BIT_OFFSET: u8 = 0;
	const RxCRC_BIT_WIDTH: u8 = 16;
	/// Rx CRC register (Width: 16, Offset: 0)
	pub fn get_rxcrc() -> u16 { ::read(REGISTER_ADDRESS, RxCRC_BIT_OFFSET, RxCRC_BIT_WIDTH) as u16 }
}
/// TX CRC register
/// Size: 0x20 bits
pub mod txcrcr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TxCRC_BIT_OFFSET: u8 = 0;
	const TxCRC_BIT_WIDTH: u8 = 16;
	/// Tx CRC register (Width: 16, Offset: 0)
	pub fn get_txcrc() -> u16 { ::read(REGISTER_ADDRESS, TxCRC_BIT_OFFSET, TxCRC_BIT_WIDTH) as u16 }
}
/// I2S configuration register
/// Size: 0x20 bits
pub mod i2scfgr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const I2SMOD_BIT_OFFSET: u8 = 11;
	const I2SMOD_BIT_WIDTH: u8 = 1;
	/// I2S mode selection (Width: 1, Offset: 11)
	pub fn get_i2smod() -> u8 { ::read(REGISTER_ADDRESS, I2SMOD_BIT_OFFSET, I2SMOD_BIT_WIDTH) as u8 }
	/// I2S mode selection (Width: 1, Offset: 11)
	pub fn set_i2smod(value: u8) { ::write(REGISTER_ADDRESS, I2SMOD_BIT_OFFSET, I2SMOD_BIT_WIDTH, value as u32); }

	const I2SE_BIT_OFFSET: u8 = 10;
	const I2SE_BIT_WIDTH: u8 = 1;
	/// I2S Enable (Width: 1, Offset: 10)
	pub fn get_i2se() -> u8 { ::read(REGISTER_ADDRESS, I2SE_BIT_OFFSET, I2SE_BIT_WIDTH) as u8 }
	/// I2S Enable (Width: 1, Offset: 10)
	pub fn set_i2se(value: u8) { ::write(REGISTER_ADDRESS, I2SE_BIT_OFFSET, I2SE_BIT_WIDTH, value as u32); }

	const I2SCFG_BIT_OFFSET: u8 = 8;
	const I2SCFG_BIT_WIDTH: u8 = 2;
	/// I2S configuration mode (Width: 2, Offset: 8)
	pub fn get_i2scfg() -> u8 { ::read(REGISTER_ADDRESS, I2SCFG_BIT_OFFSET, I2SCFG_BIT_WIDTH) as u8 }
	/// I2S configuration mode (Width: 2, Offset: 8)
	pub fn set_i2scfg(value: u8) { ::write(REGISTER_ADDRESS, I2SCFG_BIT_OFFSET, I2SCFG_BIT_WIDTH, value as u32); }

	const PCMSYNC_BIT_OFFSET: u8 = 7;
	const PCMSYNC_BIT_WIDTH: u8 = 1;
	/// PCM frame synchronization (Width: 1, Offset: 7)
	pub fn get_pcmsync() -> u8 { ::read(REGISTER_ADDRESS, PCMSYNC_BIT_OFFSET, PCMSYNC_BIT_WIDTH) as u8 }
	/// PCM frame synchronization (Width: 1, Offset: 7)
	pub fn set_pcmsync(value: u8) { ::write(REGISTER_ADDRESS, PCMSYNC_BIT_OFFSET, PCMSYNC_BIT_WIDTH, value as u32); }

	const I2SSTD_BIT_OFFSET: u8 = 4;
	const I2SSTD_BIT_WIDTH: u8 = 2;
	/// I2S standard selection (Width: 2, Offset: 4)
	pub fn get_i2sstd() -> u8 { ::read(REGISTER_ADDRESS, I2SSTD_BIT_OFFSET, I2SSTD_BIT_WIDTH) as u8 }
	/// I2S standard selection (Width: 2, Offset: 4)
	pub fn set_i2sstd(value: u8) { ::write(REGISTER_ADDRESS, I2SSTD_BIT_OFFSET, I2SSTD_BIT_WIDTH, value as u32); }

	const CKPOL_BIT_OFFSET: u8 = 3;
	const CKPOL_BIT_WIDTH: u8 = 1;
	/// Steady state clock polarity (Width: 1, Offset: 3)
	pub fn get_ckpol() -> u8 { ::read(REGISTER_ADDRESS, CKPOL_BIT_OFFSET, CKPOL_BIT_WIDTH) as u8 }
	/// Steady state clock polarity (Width: 1, Offset: 3)
	pub fn set_ckpol(value: u8) { ::write(REGISTER_ADDRESS, CKPOL_BIT_OFFSET, CKPOL_BIT_WIDTH, value as u32); }

	const DATLEN_BIT_OFFSET: u8 = 1;
	const DATLEN_BIT_WIDTH: u8 = 2;
	/// Data length to be transferred (Width: 2, Offset: 1)
	pub fn get_datlen() -> u8 { ::read(REGISTER_ADDRESS, DATLEN_BIT_OFFSET, DATLEN_BIT_WIDTH) as u8 }
	/// Data length to be transferred (Width: 2, Offset: 1)
	pub fn set_datlen(value: u8) { ::write(REGISTER_ADDRESS, DATLEN_BIT_OFFSET, DATLEN_BIT_WIDTH, value as u32); }

	const CHLEN_BIT_OFFSET: u8 = 0;
	const CHLEN_BIT_WIDTH: u8 = 1;
	/// Channel length (number of bits per audio channel) (Width: 1, Offset: 0)
	pub fn get_chlen() -> u8 { ::read(REGISTER_ADDRESS, CHLEN_BIT_OFFSET, CHLEN_BIT_WIDTH) as u8 }
	/// Channel length (number of bits per audio channel) (Width: 1, Offset: 0)
	pub fn set_chlen(value: u8) { ::write(REGISTER_ADDRESS, CHLEN_BIT_OFFSET, CHLEN_BIT_WIDTH, value as u32); }
}
/// I2S prescaler register
/// Size: 0x20 bits
pub mod i2spr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x20;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MCKOE_BIT_OFFSET: u8 = 9;
	const MCKOE_BIT_WIDTH: u8 = 1;
	/// Master clock output enable (Width: 1, Offset: 9)
	pub fn get_mckoe() -> u8 { ::read(REGISTER_ADDRESS, MCKOE_BIT_OFFSET, MCKOE_BIT_WIDTH) as u8 }
	/// Master clock output enable (Width: 1, Offset: 9)
	pub fn set_mckoe(value: u8) { ::write(REGISTER_ADDRESS, MCKOE_BIT_OFFSET, MCKOE_BIT_WIDTH, value as u32); }

	const ODD_BIT_OFFSET: u8 = 8;
	const ODD_BIT_WIDTH: u8 = 1;
	/// Odd factor for the prescaler (Width: 1, Offset: 8)
	pub fn get_odd() -> u8 { ::read(REGISTER_ADDRESS, ODD_BIT_OFFSET, ODD_BIT_WIDTH) as u8 }
	/// Odd factor for the prescaler (Width: 1, Offset: 8)
	pub fn set_odd(value: u8) { ::write(REGISTER_ADDRESS, ODD_BIT_OFFSET, ODD_BIT_WIDTH, value as u32); }

	const I2SDIV_BIT_OFFSET: u8 = 0;
	const I2SDIV_BIT_WIDTH: u8 = 8;
	/// I2S Linear prescaler (Width: 8, Offset: 0)
	pub fn get_i2sdiv() -> u8 { ::read(REGISTER_ADDRESS, I2SDIV_BIT_OFFSET, I2SDIV_BIT_WIDTH) as u8 }
	/// I2S Linear prescaler (Width: 8, Offset: 0)
	pub fn set_i2sdiv(value: u8) { ::write(REGISTER_ADDRESS, I2SDIV_BIT_OFFSET, I2SDIV_BIT_WIDTH, value as u32); }
}
/// SPI1 global interrupt
pub const INTERRUPT_SPI1: u32 = 35;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>SPI1</name>
  <description>Serial peripheral interface/Inter-IC
      sound</description>
  <groupName>SPI</groupName>
  <baseAddress>0x40013000</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>CR1</name>
      <displayName>CR1</displayName>
      <description>control register 1</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>BIDIMODE</name>
          <description>Bidirectional data mode
              enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BIDIOE</name>
          <description>Output enable in bidirectional
              mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CRCEN</name>
          <description>Hardware CRC calculation
              enable</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CRCNEXT</name>
          <description>CRC transfer next</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DFF</name>
          <description>Data frame format</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RXONLY</name>
          <description>Receive only</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SSM</name>
          <description>Software slave management</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SSI</name>
          <description>Internal slave select</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LSBFIRST</name>
          <description>Frame format</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SPE</name>
          <description>SPI enable</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BR</name>
          <description>Baud rate control</description>
          <bitOffset>3</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>MSTR</name>
          <description>Master selection</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CPOL</name>
          <description>Clock polarity</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CPHA</name>
          <description>Clock phase</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CR2</name>
      <displayName>CR2</displayName>
      <description>control register 2</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>RXDMAEN</name>
          <description>Rx buffer DMA enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TXDMAEN</name>
          <description>Tx buffer DMA enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SSOE</name>
          <description>SS output enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>NSSP</name>
          <description>NSS pulse management</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>FRF</name>
          <description>Frame format</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ERRIE</name>
          <description>Error interrupt enable</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RXNEIE</name>
          <description>RX buffer not empty interrupt
              enable</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TXEIE</name>
          <description>Tx buffer empty interrupt
              enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DS</name>
          <description>Data size</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>FRXTH</name>
          <description>FIFO reception threshold</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LDMA_RX</name>
          <description>Last DMA transfer for
              reception</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LDMA_TX</name>
          <description>Last DMA transfer for
              transmission</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SR</name>
      <displayName>SR</displayName>
      <description>status register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <resetValue>0x0002</resetValue>
      <fields>
        <field>
          <name>RXNE</name>
          <description>Receive buffer not empty</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>TXE</name>
          <description>Transmit buffer empty</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>CHSIDE</name>
          <description>Channel side</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>UDR</name>
          <description>Underrun flag</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>CRCERR</name>
          <description>CRC error flag</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>MODF</name>
          <description>Mode fault</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>OVR</name>
          <description>Overrun flag</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>BSY</name>
          <description>Busy flag</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>TIFRFE</name>
          <description>TI frame format error</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>FRLVL</name>
          <description>FIFO reception level</description>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>FTLVL</name>
          <description>FIFO transmission level</description>
          <bitOffset>11</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-only</access>
        </field>
      </fields>
    </register>
    <register>
      <name>DR</name>
      <displayName>DR</displayName>
      <description>data register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>DR</name>
          <description>Data register</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CRCPR</name>
      <displayName>CRCPR</displayName>
      <description>CRC polynomial register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0007</resetValue>
      <fields>
        <field>
          <name>CRCPOLY</name>
          <description>CRC polynomial register</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>RXCRCR</name>
      <displayName>RXCRCR</displayName>
      <description>RX CRC register</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>RxCRC</name>
          <description>Rx CRC register</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>TXCRCR</name>
      <displayName>TXCRCR</displayName>
      <description>TX CRC register</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>TxCRC</name>
          <description>Tx CRC register</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>I2SCFGR</name>
      <displayName>I2SCFGR</displayName>
      <description>I2S configuration register</description>
      <addressOffset>0x1C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>I2SMOD</name>
          <description>I2S mode selection</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2SE</name>
          <description>I2S Enable</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2SCFG</name>
          <description>I2S configuration mode</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PCMSYNC</name>
          <description>PCM frame synchronization</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2SSTD</name>
          <description>I2S standard selection</description>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>CKPOL</name>
          <description>Steady state clock
              polarity</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DATLEN</name>
          <description>Data length to be
              transferred</description>
          <bitOffset>1</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>CHLEN</name>
          <description>Channel length (number of bits per audio
              channel)</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>I2SPR</name>
      <displayName>I2SPR</displayName>
      <description>I2S prescaler register</description>
      <addressOffset>0x20</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000010</resetValue>
      <fields>
        <field>
          <name>MCKOE</name>
          <description>Master clock output enable</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ODD</name>
          <description>Odd factor for the
              prescaler</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2SDIV</name>
          <description>I2S Linear prescaler</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>SPI1</name>
    <description>SPI1 global interrupt</description>
    <value>35</value>
  </interrupt>
</peripheral>*/
