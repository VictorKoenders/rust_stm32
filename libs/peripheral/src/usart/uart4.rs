/// MOD UART4
/// Universal synchronous asynchronous receiver transmitter
const BASE_ADDRESS: u32 = 0x40004C00;
/// Control register 1
/// Size: 0x20 bits
pub mod cr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EOBIE_BIT_OFFSET: u8 = 27;
	const EOBIE_BIT_WIDTH: u8 = 1;
	/// End of Block interrupt enable (Width: 1, Offset: 27)
	pub fn get_eobie() -> u8 { ::read(REGISTER_ADDRESS, EOBIE_BIT_OFFSET, EOBIE_BIT_WIDTH) as u8 }
	/// End of Block interrupt enable (Width: 1, Offset: 27)
	pub fn set_eobie(value: u8) { ::write(REGISTER_ADDRESS, EOBIE_BIT_OFFSET, EOBIE_BIT_WIDTH, value as u32); }

	const RTOIE_BIT_OFFSET: u8 = 26;
	const RTOIE_BIT_WIDTH: u8 = 1;
	/// Receiver timeout interrupt enable (Width: 1, Offset: 26)
	pub fn get_rtoie() -> u8 { ::read(REGISTER_ADDRESS, RTOIE_BIT_OFFSET, RTOIE_BIT_WIDTH) as u8 }
	/// Receiver timeout interrupt enable (Width: 1, Offset: 26)
	pub fn set_rtoie(value: u8) { ::write(REGISTER_ADDRESS, RTOIE_BIT_OFFSET, RTOIE_BIT_WIDTH, value as u32); }

	const DEAT_BIT_OFFSET: u8 = 21;
	const DEAT_BIT_WIDTH: u8 = 5;
	/// Driver Enable assertion time (Width: 5, Offset: 21)
	pub fn get_deat() -> u8 { ::read(REGISTER_ADDRESS, DEAT_BIT_OFFSET, DEAT_BIT_WIDTH) as u8 }
	/// Driver Enable assertion time (Width: 5, Offset: 21)
	pub fn set_deat(value: u8) { ::write(REGISTER_ADDRESS, DEAT_BIT_OFFSET, DEAT_BIT_WIDTH, value as u32); }

	const DEDT_BIT_OFFSET: u8 = 16;
	const DEDT_BIT_WIDTH: u8 = 5;
	/// Driver Enable deassertion time (Width: 5, Offset: 16)
	pub fn get_dedt() -> u8 { ::read(REGISTER_ADDRESS, DEDT_BIT_OFFSET, DEDT_BIT_WIDTH) as u8 }
	/// Driver Enable deassertion time (Width: 5, Offset: 16)
	pub fn set_dedt(value: u8) { ::write(REGISTER_ADDRESS, DEDT_BIT_OFFSET, DEDT_BIT_WIDTH, value as u32); }

	const OVER8_BIT_OFFSET: u8 = 15;
	const OVER8_BIT_WIDTH: u8 = 1;
	/// Oversampling mode (Width: 1, Offset: 15)
	pub fn get_over8() -> u8 { ::read(REGISTER_ADDRESS, OVER8_BIT_OFFSET, OVER8_BIT_WIDTH) as u8 }
	/// Oversampling mode (Width: 1, Offset: 15)
	pub fn set_over8(value: u8) { ::write(REGISTER_ADDRESS, OVER8_BIT_OFFSET, OVER8_BIT_WIDTH, value as u32); }

	const CMIE_BIT_OFFSET: u8 = 14;
	const CMIE_BIT_WIDTH: u8 = 1;
	/// Character match interrupt enable (Width: 1, Offset: 14)
	pub fn get_cmie() -> u8 { ::read(REGISTER_ADDRESS, CMIE_BIT_OFFSET, CMIE_BIT_WIDTH) as u8 }
	/// Character match interrupt enable (Width: 1, Offset: 14)
	pub fn set_cmie(value: u8) { ::write(REGISTER_ADDRESS, CMIE_BIT_OFFSET, CMIE_BIT_WIDTH, value as u32); }

	const MME_BIT_OFFSET: u8 = 13;
	const MME_BIT_WIDTH: u8 = 1;
	/// Mute mode enable (Width: 1, Offset: 13)
	pub fn get_mme() -> u8 { ::read(REGISTER_ADDRESS, MME_BIT_OFFSET, MME_BIT_WIDTH) as u8 }
	/// Mute mode enable (Width: 1, Offset: 13)
	pub fn set_mme(value: u8) { ::write(REGISTER_ADDRESS, MME_BIT_OFFSET, MME_BIT_WIDTH, value as u32); }

	const M_BIT_OFFSET: u8 = 12;
	const M_BIT_WIDTH: u8 = 1;
	/// Word length (Width: 1, Offset: 12)
	pub fn get_m() -> u8 { ::read(REGISTER_ADDRESS, M_BIT_OFFSET, M_BIT_WIDTH) as u8 }
	/// Word length (Width: 1, Offset: 12)
	pub fn set_m(value: u8) { ::write(REGISTER_ADDRESS, M_BIT_OFFSET, M_BIT_WIDTH, value as u32); }

	const WAKE_BIT_OFFSET: u8 = 11;
	const WAKE_BIT_WIDTH: u8 = 1;
	/// Receiver wakeup method (Width: 1, Offset: 11)
	pub fn get_wake() -> u8 { ::read(REGISTER_ADDRESS, WAKE_BIT_OFFSET, WAKE_BIT_WIDTH) as u8 }
	/// Receiver wakeup method (Width: 1, Offset: 11)
	pub fn set_wake(value: u8) { ::write(REGISTER_ADDRESS, WAKE_BIT_OFFSET, WAKE_BIT_WIDTH, value as u32); }

	const PCE_BIT_OFFSET: u8 = 10;
	const PCE_BIT_WIDTH: u8 = 1;
	/// Parity control enable (Width: 1, Offset: 10)
	pub fn get_pce() -> u8 { ::read(REGISTER_ADDRESS, PCE_BIT_OFFSET, PCE_BIT_WIDTH) as u8 }
	/// Parity control enable (Width: 1, Offset: 10)
	pub fn set_pce(value: u8) { ::write(REGISTER_ADDRESS, PCE_BIT_OFFSET, PCE_BIT_WIDTH, value as u32); }

	const PS_BIT_OFFSET: u8 = 9;
	const PS_BIT_WIDTH: u8 = 1;
	/// Parity selection (Width: 1, Offset: 9)
	pub fn get_ps() -> u8 { ::read(REGISTER_ADDRESS, PS_BIT_OFFSET, PS_BIT_WIDTH) as u8 }
	/// Parity selection (Width: 1, Offset: 9)
	pub fn set_ps(value: u8) { ::write(REGISTER_ADDRESS, PS_BIT_OFFSET, PS_BIT_WIDTH, value as u32); }

	const PEIE_BIT_OFFSET: u8 = 8;
	const PEIE_BIT_WIDTH: u8 = 1;
	/// PE interrupt enable (Width: 1, Offset: 8)
	pub fn get_peie() -> u8 { ::read(REGISTER_ADDRESS, PEIE_BIT_OFFSET, PEIE_BIT_WIDTH) as u8 }
	/// PE interrupt enable (Width: 1, Offset: 8)
	pub fn set_peie(value: u8) { ::write(REGISTER_ADDRESS, PEIE_BIT_OFFSET, PEIE_BIT_WIDTH, value as u32); }

	const TXEIE_BIT_OFFSET: u8 = 7;
	const TXEIE_BIT_WIDTH: u8 = 1;
	/// interrupt enable (Width: 1, Offset: 7)
	pub fn get_txeie() -> u8 { ::read(REGISTER_ADDRESS, TXEIE_BIT_OFFSET, TXEIE_BIT_WIDTH) as u8 }
	/// interrupt enable (Width: 1, Offset: 7)
	pub fn set_txeie(value: u8) { ::write(REGISTER_ADDRESS, TXEIE_BIT_OFFSET, TXEIE_BIT_WIDTH, value as u32); }

	const TCIE_BIT_OFFSET: u8 = 6;
	const TCIE_BIT_WIDTH: u8 = 1;
	/// Transmission complete interrupt enable (Width: 1, Offset: 6)
	pub fn get_tcie() -> u8 { ::read(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH) as u8 }
	/// Transmission complete interrupt enable (Width: 1, Offset: 6)
	pub fn set_tcie(value: u8) { ::write(REGISTER_ADDRESS, TCIE_BIT_OFFSET, TCIE_BIT_WIDTH, value as u32); }

	const RXNEIE_BIT_OFFSET: u8 = 5;
	const RXNEIE_BIT_WIDTH: u8 = 1;
	/// RXNE interrupt enable (Width: 1, Offset: 5)
	pub fn get_rxneie() -> u8 { ::read(REGISTER_ADDRESS, RXNEIE_BIT_OFFSET, RXNEIE_BIT_WIDTH) as u8 }
	/// RXNE interrupt enable (Width: 1, Offset: 5)
	pub fn set_rxneie(value: u8) { ::write(REGISTER_ADDRESS, RXNEIE_BIT_OFFSET, RXNEIE_BIT_WIDTH, value as u32); }

	const IDLEIE_BIT_OFFSET: u8 = 4;
	const IDLEIE_BIT_WIDTH: u8 = 1;
	/// IDLE interrupt enable (Width: 1, Offset: 4)
	pub fn get_idleie() -> u8 { ::read(REGISTER_ADDRESS, IDLEIE_BIT_OFFSET, IDLEIE_BIT_WIDTH) as u8 }
	/// IDLE interrupt enable (Width: 1, Offset: 4)
	pub fn set_idleie(value: u8) { ::write(REGISTER_ADDRESS, IDLEIE_BIT_OFFSET, IDLEIE_BIT_WIDTH, value as u32); }

	const TE_BIT_OFFSET: u8 = 3;
	const TE_BIT_WIDTH: u8 = 1;
	/// Transmitter enable (Width: 1, Offset: 3)
	pub fn get_te() -> u8 { ::read(REGISTER_ADDRESS, TE_BIT_OFFSET, TE_BIT_WIDTH) as u8 }
	/// Transmitter enable (Width: 1, Offset: 3)
	pub fn set_te(value: u8) { ::write(REGISTER_ADDRESS, TE_BIT_OFFSET, TE_BIT_WIDTH, value as u32); }

	const RE_BIT_OFFSET: u8 = 2;
	const RE_BIT_WIDTH: u8 = 1;
	/// Receiver enable (Width: 1, Offset: 2)
	pub fn get_re() -> u8 { ::read(REGISTER_ADDRESS, RE_BIT_OFFSET, RE_BIT_WIDTH) as u8 }
	/// Receiver enable (Width: 1, Offset: 2)
	pub fn set_re(value: u8) { ::write(REGISTER_ADDRESS, RE_BIT_OFFSET, RE_BIT_WIDTH, value as u32); }

	const UESM_BIT_OFFSET: u8 = 1;
	const UESM_BIT_WIDTH: u8 = 1;
	/// USART enable in Stop mode (Width: 1, Offset: 1)
	pub fn get_uesm() -> u8 { ::read(REGISTER_ADDRESS, UESM_BIT_OFFSET, UESM_BIT_WIDTH) as u8 }
	/// USART enable in Stop mode (Width: 1, Offset: 1)
	pub fn set_uesm(value: u8) { ::write(REGISTER_ADDRESS, UESM_BIT_OFFSET, UESM_BIT_WIDTH, value as u32); }

	const UE_BIT_OFFSET: u8 = 0;
	const UE_BIT_WIDTH: u8 = 1;
	/// USART enable (Width: 1, Offset: 0)
	pub fn get_ue() -> u8 { ::read(REGISTER_ADDRESS, UE_BIT_OFFSET, UE_BIT_WIDTH) as u8 }
	/// USART enable (Width: 1, Offset: 0)
	pub fn set_ue(value: u8) { ::write(REGISTER_ADDRESS, UE_BIT_OFFSET, UE_BIT_WIDTH, value as u32); }
}
/// Control register 2
/// Size: 0x20 bits
pub mod cr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ADD4_BIT_OFFSET: u8 = 28;
	const ADD4_BIT_WIDTH: u8 = 4;
	/// Address of the USART node (Width: 4, Offset: 28)
	pub fn get_add4() -> u8 { ::read(REGISTER_ADDRESS, ADD4_BIT_OFFSET, ADD4_BIT_WIDTH) as u8 }
	/// Address of the USART node (Width: 4, Offset: 28)
	pub fn set_add4(value: u8) { ::write(REGISTER_ADDRESS, ADD4_BIT_OFFSET, ADD4_BIT_WIDTH, value as u32); }

	const ADD0_BIT_OFFSET: u8 = 24;
	const ADD0_BIT_WIDTH: u8 = 4;
	/// Address of the USART node (Width: 4, Offset: 24)
	pub fn get_add0() -> u8 { ::read(REGISTER_ADDRESS, ADD0_BIT_OFFSET, ADD0_BIT_WIDTH) as u8 }
	/// Address of the USART node (Width: 4, Offset: 24)
	pub fn set_add0(value: u8) { ::write(REGISTER_ADDRESS, ADD0_BIT_OFFSET, ADD0_BIT_WIDTH, value as u32); }

	const RTOEN_BIT_OFFSET: u8 = 23;
	const RTOEN_BIT_WIDTH: u8 = 1;
	/// Receiver timeout enable (Width: 1, Offset: 23)
	pub fn get_rtoen() -> u8 { ::read(REGISTER_ADDRESS, RTOEN_BIT_OFFSET, RTOEN_BIT_WIDTH) as u8 }
	/// Receiver timeout enable (Width: 1, Offset: 23)
	pub fn set_rtoen(value: u8) { ::write(REGISTER_ADDRESS, RTOEN_BIT_OFFSET, RTOEN_BIT_WIDTH, value as u32); }

	const ABRMOD_BIT_OFFSET: u8 = 21;
	const ABRMOD_BIT_WIDTH: u8 = 2;
	/// Auto baud rate mode (Width: 2, Offset: 21)
	pub fn get_abrmod() -> u8 { ::read(REGISTER_ADDRESS, ABRMOD_BIT_OFFSET, ABRMOD_BIT_WIDTH) as u8 }
	/// Auto baud rate mode (Width: 2, Offset: 21)
	pub fn set_abrmod(value: u8) { ::write(REGISTER_ADDRESS, ABRMOD_BIT_OFFSET, ABRMOD_BIT_WIDTH, value as u32); }

	const ABREN_BIT_OFFSET: u8 = 20;
	const ABREN_BIT_WIDTH: u8 = 1;
	/// Auto baud rate enable (Width: 1, Offset: 20)
	pub fn get_abren() -> u8 { ::read(REGISTER_ADDRESS, ABREN_BIT_OFFSET, ABREN_BIT_WIDTH) as u8 }
	/// Auto baud rate enable (Width: 1, Offset: 20)
	pub fn set_abren(value: u8) { ::write(REGISTER_ADDRESS, ABREN_BIT_OFFSET, ABREN_BIT_WIDTH, value as u32); }

	const MSBFIRST_BIT_OFFSET: u8 = 19;
	const MSBFIRST_BIT_WIDTH: u8 = 1;
	/// Most significant bit first (Width: 1, Offset: 19)
	pub fn get_msbfirst() -> u8 { ::read(REGISTER_ADDRESS, MSBFIRST_BIT_OFFSET, MSBFIRST_BIT_WIDTH) as u8 }
	/// Most significant bit first (Width: 1, Offset: 19)
	pub fn set_msbfirst(value: u8) { ::write(REGISTER_ADDRESS, MSBFIRST_BIT_OFFSET, MSBFIRST_BIT_WIDTH, value as u32); }

	const DATAINV_BIT_OFFSET: u8 = 18;
	const DATAINV_BIT_WIDTH: u8 = 1;
	/// Binary data inversion (Width: 1, Offset: 18)
	pub fn get_datainv() -> u8 { ::read(REGISTER_ADDRESS, DATAINV_BIT_OFFSET, DATAINV_BIT_WIDTH) as u8 }
	/// Binary data inversion (Width: 1, Offset: 18)
	pub fn set_datainv(value: u8) { ::write(REGISTER_ADDRESS, DATAINV_BIT_OFFSET, DATAINV_BIT_WIDTH, value as u32); }

	const TXINV_BIT_OFFSET: u8 = 17;
	const TXINV_BIT_WIDTH: u8 = 1;
	/// TX pin active level inversion (Width: 1, Offset: 17)
	pub fn get_txinv() -> u8 { ::read(REGISTER_ADDRESS, TXINV_BIT_OFFSET, TXINV_BIT_WIDTH) as u8 }
	/// TX pin active level inversion (Width: 1, Offset: 17)
	pub fn set_txinv(value: u8) { ::write(REGISTER_ADDRESS, TXINV_BIT_OFFSET, TXINV_BIT_WIDTH, value as u32); }

	const RXINV_BIT_OFFSET: u8 = 16;
	const RXINV_BIT_WIDTH: u8 = 1;
	/// RX pin active level inversion (Width: 1, Offset: 16)
	pub fn get_rxinv() -> u8 { ::read(REGISTER_ADDRESS, RXINV_BIT_OFFSET, RXINV_BIT_WIDTH) as u8 }
	/// RX pin active level inversion (Width: 1, Offset: 16)
	pub fn set_rxinv(value: u8) { ::write(REGISTER_ADDRESS, RXINV_BIT_OFFSET, RXINV_BIT_WIDTH, value as u32); }

	const SWAP_BIT_OFFSET: u8 = 15;
	const SWAP_BIT_WIDTH: u8 = 1;
	/// Swap TX/RX pins (Width: 1, Offset: 15)
	pub fn get_swap() -> u8 { ::read(REGISTER_ADDRESS, SWAP_BIT_OFFSET, SWAP_BIT_WIDTH) as u8 }
	/// Swap TX/RX pins (Width: 1, Offset: 15)
	pub fn set_swap(value: u8) { ::write(REGISTER_ADDRESS, SWAP_BIT_OFFSET, SWAP_BIT_WIDTH, value as u32); }

	const LINEN_BIT_OFFSET: u8 = 14;
	const LINEN_BIT_WIDTH: u8 = 1;
	/// LIN mode enable (Width: 1, Offset: 14)
	pub fn get_linen() -> u8 { ::read(REGISTER_ADDRESS, LINEN_BIT_OFFSET, LINEN_BIT_WIDTH) as u8 }
	/// LIN mode enable (Width: 1, Offset: 14)
	pub fn set_linen(value: u8) { ::write(REGISTER_ADDRESS, LINEN_BIT_OFFSET, LINEN_BIT_WIDTH, value as u32); }

	const STOP_BIT_OFFSET: u8 = 12;
	const STOP_BIT_WIDTH: u8 = 2;
	/// STOP bits (Width: 2, Offset: 12)
	pub fn get_stop() -> u8 { ::read(REGISTER_ADDRESS, STOP_BIT_OFFSET, STOP_BIT_WIDTH) as u8 }
	/// STOP bits (Width: 2, Offset: 12)
	pub fn set_stop(value: u8) { ::write(REGISTER_ADDRESS, STOP_BIT_OFFSET, STOP_BIT_WIDTH, value as u32); }

	const CLKEN_BIT_OFFSET: u8 = 11;
	const CLKEN_BIT_WIDTH: u8 = 1;
	/// Clock enable (Width: 1, Offset: 11)
	pub fn get_clken() -> u8 { ::read(REGISTER_ADDRESS, CLKEN_BIT_OFFSET, CLKEN_BIT_WIDTH) as u8 }
	/// Clock enable (Width: 1, Offset: 11)
	pub fn set_clken(value: u8) { ::write(REGISTER_ADDRESS, CLKEN_BIT_OFFSET, CLKEN_BIT_WIDTH, value as u32); }

	const CPOL_BIT_OFFSET: u8 = 10;
	const CPOL_BIT_WIDTH: u8 = 1;
	/// Clock polarity (Width: 1, Offset: 10)
	pub fn get_cpol() -> u8 { ::read(REGISTER_ADDRESS, CPOL_BIT_OFFSET, CPOL_BIT_WIDTH) as u8 }
	/// Clock polarity (Width: 1, Offset: 10)
	pub fn set_cpol(value: u8) { ::write(REGISTER_ADDRESS, CPOL_BIT_OFFSET, CPOL_BIT_WIDTH, value as u32); }

	const CPHA_BIT_OFFSET: u8 = 9;
	const CPHA_BIT_WIDTH: u8 = 1;
	/// Clock phase (Width: 1, Offset: 9)
	pub fn get_cpha() -> u8 { ::read(REGISTER_ADDRESS, CPHA_BIT_OFFSET, CPHA_BIT_WIDTH) as u8 }
	/// Clock phase (Width: 1, Offset: 9)
	pub fn set_cpha(value: u8) { ::write(REGISTER_ADDRESS, CPHA_BIT_OFFSET, CPHA_BIT_WIDTH, value as u32); }

	const LBCL_BIT_OFFSET: u8 = 8;
	const LBCL_BIT_WIDTH: u8 = 1;
	/// Last bit clock pulse (Width: 1, Offset: 8)
	pub fn get_lbcl() -> u8 { ::read(REGISTER_ADDRESS, LBCL_BIT_OFFSET, LBCL_BIT_WIDTH) as u8 }
	/// Last bit clock pulse (Width: 1, Offset: 8)
	pub fn set_lbcl(value: u8) { ::write(REGISTER_ADDRESS, LBCL_BIT_OFFSET, LBCL_BIT_WIDTH, value as u32); }

	const LBDIE_BIT_OFFSET: u8 = 6;
	const LBDIE_BIT_WIDTH: u8 = 1;
	/// LIN break detection interrupt enable (Width: 1, Offset: 6)
	pub fn get_lbdie() -> u8 { ::read(REGISTER_ADDRESS, LBDIE_BIT_OFFSET, LBDIE_BIT_WIDTH) as u8 }
	/// LIN break detection interrupt enable (Width: 1, Offset: 6)
	pub fn set_lbdie(value: u8) { ::write(REGISTER_ADDRESS, LBDIE_BIT_OFFSET, LBDIE_BIT_WIDTH, value as u32); }

	const LBDL_BIT_OFFSET: u8 = 5;
	const LBDL_BIT_WIDTH: u8 = 1;
	/// LIN break detection length (Width: 1, Offset: 5)
	pub fn get_lbdl() -> u8 { ::read(REGISTER_ADDRESS, LBDL_BIT_OFFSET, LBDL_BIT_WIDTH) as u8 }
	/// LIN break detection length (Width: 1, Offset: 5)
	pub fn set_lbdl(value: u8) { ::write(REGISTER_ADDRESS, LBDL_BIT_OFFSET, LBDL_BIT_WIDTH, value as u32); }

	const ADDM7_BIT_OFFSET: u8 = 4;
	const ADDM7_BIT_WIDTH: u8 = 1;
	/// 7-bit Address Detection/4-bit Address Detection (Width: 1, Offset: 4)
	pub fn get_addm7() -> u8 { ::read(REGISTER_ADDRESS, ADDM7_BIT_OFFSET, ADDM7_BIT_WIDTH) as u8 }
	/// 7-bit Address Detection/4-bit Address Detection (Width: 1, Offset: 4)
	pub fn set_addm7(value: u8) { ::write(REGISTER_ADDRESS, ADDM7_BIT_OFFSET, ADDM7_BIT_WIDTH, value as u32); }
}
/// Control register 3
/// Size: 0x20 bits
pub mod cr3 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const WUFIE_BIT_OFFSET: u8 = 22;
	const WUFIE_BIT_WIDTH: u8 = 1;
	/// Wakeup from Stop mode interrupt enable (Width: 1, Offset: 22)
	pub fn get_wufie() -> u8 { ::read(REGISTER_ADDRESS, WUFIE_BIT_OFFSET, WUFIE_BIT_WIDTH) as u8 }
	/// Wakeup from Stop mode interrupt enable (Width: 1, Offset: 22)
	pub fn set_wufie(value: u8) { ::write(REGISTER_ADDRESS, WUFIE_BIT_OFFSET, WUFIE_BIT_WIDTH, value as u32); }

	const WUS_BIT_OFFSET: u8 = 20;
	const WUS_BIT_WIDTH: u8 = 2;
	/// Wakeup from Stop mode interrupt flag selection (Width: 2, Offset: 20)
	pub fn get_wus() -> u8 { ::read(REGISTER_ADDRESS, WUS_BIT_OFFSET, WUS_BIT_WIDTH) as u8 }
	/// Wakeup from Stop mode interrupt flag selection (Width: 2, Offset: 20)
	pub fn set_wus(value: u8) { ::write(REGISTER_ADDRESS, WUS_BIT_OFFSET, WUS_BIT_WIDTH, value as u32); }

	const SCARCNT_BIT_OFFSET: u8 = 17;
	const SCARCNT_BIT_WIDTH: u8 = 3;
	/// Smartcard auto-retry count (Width: 3, Offset: 17)
	pub fn get_scarcnt() -> u8 { ::read(REGISTER_ADDRESS, SCARCNT_BIT_OFFSET, SCARCNT_BIT_WIDTH) as u8 }
	/// Smartcard auto-retry count (Width: 3, Offset: 17)
	pub fn set_scarcnt(value: u8) { ::write(REGISTER_ADDRESS, SCARCNT_BIT_OFFSET, SCARCNT_BIT_WIDTH, value as u32); }

	const DEP_BIT_OFFSET: u8 = 15;
	const DEP_BIT_WIDTH: u8 = 1;
	/// Driver enable polarity selection (Width: 1, Offset: 15)
	pub fn get_dep() -> u8 { ::read(REGISTER_ADDRESS, DEP_BIT_OFFSET, DEP_BIT_WIDTH) as u8 }
	/// Driver enable polarity selection (Width: 1, Offset: 15)
	pub fn set_dep(value: u8) { ::write(REGISTER_ADDRESS, DEP_BIT_OFFSET, DEP_BIT_WIDTH, value as u32); }

	const DEM_BIT_OFFSET: u8 = 14;
	const DEM_BIT_WIDTH: u8 = 1;
	/// Driver enable mode (Width: 1, Offset: 14)
	pub fn get_dem() -> u8 { ::read(REGISTER_ADDRESS, DEM_BIT_OFFSET, DEM_BIT_WIDTH) as u8 }
	/// Driver enable mode (Width: 1, Offset: 14)
	pub fn set_dem(value: u8) { ::write(REGISTER_ADDRESS, DEM_BIT_OFFSET, DEM_BIT_WIDTH, value as u32); }

	const DDRE_BIT_OFFSET: u8 = 13;
	const DDRE_BIT_WIDTH: u8 = 1;
	/// DMA Disable on Reception Error (Width: 1, Offset: 13)
	pub fn get_ddre() -> u8 { ::read(REGISTER_ADDRESS, DDRE_BIT_OFFSET, DDRE_BIT_WIDTH) as u8 }
	/// DMA Disable on Reception Error (Width: 1, Offset: 13)
	pub fn set_ddre(value: u8) { ::write(REGISTER_ADDRESS, DDRE_BIT_OFFSET, DDRE_BIT_WIDTH, value as u32); }

	const OVRDIS_BIT_OFFSET: u8 = 12;
	const OVRDIS_BIT_WIDTH: u8 = 1;
	/// Overrun Disable (Width: 1, Offset: 12)
	pub fn get_ovrdis() -> u8 { ::read(REGISTER_ADDRESS, OVRDIS_BIT_OFFSET, OVRDIS_BIT_WIDTH) as u8 }
	/// Overrun Disable (Width: 1, Offset: 12)
	pub fn set_ovrdis(value: u8) { ::write(REGISTER_ADDRESS, OVRDIS_BIT_OFFSET, OVRDIS_BIT_WIDTH, value as u32); }

	const ONEBIT_BIT_OFFSET: u8 = 11;
	const ONEBIT_BIT_WIDTH: u8 = 1;
	/// One sample bit method enable (Width: 1, Offset: 11)
	pub fn get_onebit() -> u8 { ::read(REGISTER_ADDRESS, ONEBIT_BIT_OFFSET, ONEBIT_BIT_WIDTH) as u8 }
	/// One sample bit method enable (Width: 1, Offset: 11)
	pub fn set_onebit(value: u8) { ::write(REGISTER_ADDRESS, ONEBIT_BIT_OFFSET, ONEBIT_BIT_WIDTH, value as u32); }

	const CTSIE_BIT_OFFSET: u8 = 10;
	const CTSIE_BIT_WIDTH: u8 = 1;
	/// CTS interrupt enable (Width: 1, Offset: 10)
	pub fn get_ctsie() -> u8 { ::read(REGISTER_ADDRESS, CTSIE_BIT_OFFSET, CTSIE_BIT_WIDTH) as u8 }
	/// CTS interrupt enable (Width: 1, Offset: 10)
	pub fn set_ctsie(value: u8) { ::write(REGISTER_ADDRESS, CTSIE_BIT_OFFSET, CTSIE_BIT_WIDTH, value as u32); }

	const CTSE_BIT_OFFSET: u8 = 9;
	const CTSE_BIT_WIDTH: u8 = 1;
	/// CTS enable (Width: 1, Offset: 9)
	pub fn get_ctse() -> u8 { ::read(REGISTER_ADDRESS, CTSE_BIT_OFFSET, CTSE_BIT_WIDTH) as u8 }
	/// CTS enable (Width: 1, Offset: 9)
	pub fn set_ctse(value: u8) { ::write(REGISTER_ADDRESS, CTSE_BIT_OFFSET, CTSE_BIT_WIDTH, value as u32); }

	const RTSE_BIT_OFFSET: u8 = 8;
	const RTSE_BIT_WIDTH: u8 = 1;
	/// RTS enable (Width: 1, Offset: 8)
	pub fn get_rtse() -> u8 { ::read(REGISTER_ADDRESS, RTSE_BIT_OFFSET, RTSE_BIT_WIDTH) as u8 }
	/// RTS enable (Width: 1, Offset: 8)
	pub fn set_rtse(value: u8) { ::write(REGISTER_ADDRESS, RTSE_BIT_OFFSET, RTSE_BIT_WIDTH, value as u32); }

	const DMAT_BIT_OFFSET: u8 = 7;
	const DMAT_BIT_WIDTH: u8 = 1;
	/// DMA enable transmitter (Width: 1, Offset: 7)
	pub fn get_dmat() -> u8 { ::read(REGISTER_ADDRESS, DMAT_BIT_OFFSET, DMAT_BIT_WIDTH) as u8 }
	/// DMA enable transmitter (Width: 1, Offset: 7)
	pub fn set_dmat(value: u8) { ::write(REGISTER_ADDRESS, DMAT_BIT_OFFSET, DMAT_BIT_WIDTH, value as u32); }

	const DMAR_BIT_OFFSET: u8 = 6;
	const DMAR_BIT_WIDTH: u8 = 1;
	/// DMA enable receiver (Width: 1, Offset: 6)
	pub fn get_dmar() -> u8 { ::read(REGISTER_ADDRESS, DMAR_BIT_OFFSET, DMAR_BIT_WIDTH) as u8 }
	/// DMA enable receiver (Width: 1, Offset: 6)
	pub fn set_dmar(value: u8) { ::write(REGISTER_ADDRESS, DMAR_BIT_OFFSET, DMAR_BIT_WIDTH, value as u32); }

	const SCEN_BIT_OFFSET: u8 = 5;
	const SCEN_BIT_WIDTH: u8 = 1;
	/// Smartcard mode enable (Width: 1, Offset: 5)
	pub fn get_scen() -> u8 { ::read(REGISTER_ADDRESS, SCEN_BIT_OFFSET, SCEN_BIT_WIDTH) as u8 }
	/// Smartcard mode enable (Width: 1, Offset: 5)
	pub fn set_scen(value: u8) { ::write(REGISTER_ADDRESS, SCEN_BIT_OFFSET, SCEN_BIT_WIDTH, value as u32); }

	const NACK_BIT_OFFSET: u8 = 4;
	const NACK_BIT_WIDTH: u8 = 1;
	/// Smartcard NACK enable (Width: 1, Offset: 4)
	pub fn get_nack() -> u8 { ::read(REGISTER_ADDRESS, NACK_BIT_OFFSET, NACK_BIT_WIDTH) as u8 }
	/// Smartcard NACK enable (Width: 1, Offset: 4)
	pub fn set_nack(value: u8) { ::write(REGISTER_ADDRESS, NACK_BIT_OFFSET, NACK_BIT_WIDTH, value as u32); }

	const HDSEL_BIT_OFFSET: u8 = 3;
	const HDSEL_BIT_WIDTH: u8 = 1;
	/// Half-duplex selection (Width: 1, Offset: 3)
	pub fn get_hdsel() -> u8 { ::read(REGISTER_ADDRESS, HDSEL_BIT_OFFSET, HDSEL_BIT_WIDTH) as u8 }
	/// Half-duplex selection (Width: 1, Offset: 3)
	pub fn set_hdsel(value: u8) { ::write(REGISTER_ADDRESS, HDSEL_BIT_OFFSET, HDSEL_BIT_WIDTH, value as u32); }

	const IRLP_BIT_OFFSET: u8 = 2;
	const IRLP_BIT_WIDTH: u8 = 1;
	/// IrDA low-power (Width: 1, Offset: 2)
	pub fn get_irlp() -> u8 { ::read(REGISTER_ADDRESS, IRLP_BIT_OFFSET, IRLP_BIT_WIDTH) as u8 }
	/// IrDA low-power (Width: 1, Offset: 2)
	pub fn set_irlp(value: u8) { ::write(REGISTER_ADDRESS, IRLP_BIT_OFFSET, IRLP_BIT_WIDTH, value as u32); }

	const IREN_BIT_OFFSET: u8 = 1;
	const IREN_BIT_WIDTH: u8 = 1;
	/// IrDA mode enable (Width: 1, Offset: 1)
	pub fn get_iren() -> u8 { ::read(REGISTER_ADDRESS, IREN_BIT_OFFSET, IREN_BIT_WIDTH) as u8 }
	/// IrDA mode enable (Width: 1, Offset: 1)
	pub fn set_iren(value: u8) { ::write(REGISTER_ADDRESS, IREN_BIT_OFFSET, IREN_BIT_WIDTH, value as u32); }

	const EIE_BIT_OFFSET: u8 = 0;
	const EIE_BIT_WIDTH: u8 = 1;
	/// Error interrupt enable (Width: 1, Offset: 0)
	pub fn get_eie() -> u8 { ::read(REGISTER_ADDRESS, EIE_BIT_OFFSET, EIE_BIT_WIDTH) as u8 }
	/// Error interrupt enable (Width: 1, Offset: 0)
	pub fn set_eie(value: u8) { ::write(REGISTER_ADDRESS, EIE_BIT_OFFSET, EIE_BIT_WIDTH, value as u32); }
}
/// Baud rate register
/// Size: 0x20 bits
pub mod brr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DIV_Mantissa_BIT_OFFSET: u8 = 4;
	const DIV_Mantissa_BIT_WIDTH: u8 = 12;
	/// mantissa of USARTDIV (Width: 12, Offset: 4)
	pub fn get_div_mantissa() -> u16 { ::read(REGISTER_ADDRESS, DIV_Mantissa_BIT_OFFSET, DIV_Mantissa_BIT_WIDTH) as u16 }
	/// mantissa of USARTDIV (Width: 12, Offset: 4)
	pub fn set_div_mantissa(value: u16) { ::write(REGISTER_ADDRESS, DIV_Mantissa_BIT_OFFSET, DIV_Mantissa_BIT_WIDTH, value as u32); }

	const DIV_Fraction_BIT_OFFSET: u8 = 0;
	const DIV_Fraction_BIT_WIDTH: u8 = 4;
	/// fraction of USARTDIV (Width: 4, Offset: 0)
	pub fn get_div_fraction() -> u8 { ::read(REGISTER_ADDRESS, DIV_Fraction_BIT_OFFSET, DIV_Fraction_BIT_WIDTH) as u8 }
	/// fraction of USARTDIV (Width: 4, Offset: 0)
	pub fn set_div_fraction(value: u8) { ::write(REGISTER_ADDRESS, DIV_Fraction_BIT_OFFSET, DIV_Fraction_BIT_WIDTH, value as u32); }
}
/// Guard time and prescaler register
/// Size: 0x20 bits
pub mod gtpr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const GT_BIT_OFFSET: u8 = 8;
	const GT_BIT_WIDTH: u8 = 8;
	/// Guard time value (Width: 8, Offset: 8)
	pub fn get_gt() -> u8 { ::read(REGISTER_ADDRESS, GT_BIT_OFFSET, GT_BIT_WIDTH) as u8 }
	/// Guard time value (Width: 8, Offset: 8)
	pub fn set_gt(value: u8) { ::write(REGISTER_ADDRESS, GT_BIT_OFFSET, GT_BIT_WIDTH, value as u32); }

	const PSC_BIT_OFFSET: u8 = 0;
	const PSC_BIT_WIDTH: u8 = 8;
	/// Prescaler value (Width: 8, Offset: 0)
	pub fn get_psc() -> u8 { ::read(REGISTER_ADDRESS, PSC_BIT_OFFSET, PSC_BIT_WIDTH) as u8 }
	/// Prescaler value (Width: 8, Offset: 0)
	pub fn set_psc(value: u8) { ::write(REGISTER_ADDRESS, PSC_BIT_OFFSET, PSC_BIT_WIDTH, value as u32); }
}
/// Receiver timeout register
/// Size: 0x20 bits
pub mod rtor {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BLEN_BIT_OFFSET: u8 = 24;
	const BLEN_BIT_WIDTH: u8 = 8;
	/// Block Length (Width: 8, Offset: 24)
	pub fn get_blen() -> u8 { ::read(REGISTER_ADDRESS, BLEN_BIT_OFFSET, BLEN_BIT_WIDTH) as u8 }
	/// Block Length (Width: 8, Offset: 24)
	pub fn set_blen(value: u8) { ::write(REGISTER_ADDRESS, BLEN_BIT_OFFSET, BLEN_BIT_WIDTH, value as u32); }

	const RTO_BIT_OFFSET: u8 = 0;
	const RTO_BIT_WIDTH: u8 = 24;
	/// Receiver timeout value (Width: 24, Offset: 0)
	pub fn get_rto() -> u32 { ::read(REGISTER_ADDRESS, RTO_BIT_OFFSET, RTO_BIT_WIDTH) as u32 }
	/// Receiver timeout value (Width: 24, Offset: 0)
	pub fn set_rto(value: u32) { ::write(REGISTER_ADDRESS, RTO_BIT_OFFSET, RTO_BIT_WIDTH, value as u32); }
}
/// Request register
/// Size: 0x20 bits
pub mod rqr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TXFRQ_BIT_OFFSET: u8 = 4;
	const TXFRQ_BIT_WIDTH: u8 = 1;
	/// Transmit data flush request (Width: 1, Offset: 4)
	pub fn get_txfrq() -> u8 { ::read(REGISTER_ADDRESS, TXFRQ_BIT_OFFSET, TXFRQ_BIT_WIDTH) as u8 }
	/// Transmit data flush request (Width: 1, Offset: 4)
	pub fn set_txfrq(value: u8) { ::write(REGISTER_ADDRESS, TXFRQ_BIT_OFFSET, TXFRQ_BIT_WIDTH, value as u32); }

	const RXFRQ_BIT_OFFSET: u8 = 3;
	const RXFRQ_BIT_WIDTH: u8 = 1;
	/// Receive data flush request (Width: 1, Offset: 3)
	pub fn get_rxfrq() -> u8 { ::read(REGISTER_ADDRESS, RXFRQ_BIT_OFFSET, RXFRQ_BIT_WIDTH) as u8 }
	/// Receive data flush request (Width: 1, Offset: 3)
	pub fn set_rxfrq(value: u8) { ::write(REGISTER_ADDRESS, RXFRQ_BIT_OFFSET, RXFRQ_BIT_WIDTH, value as u32); }

	const MMRQ_BIT_OFFSET: u8 = 2;
	const MMRQ_BIT_WIDTH: u8 = 1;
	/// Mute mode request (Width: 1, Offset: 2)
	pub fn get_mmrq() -> u8 { ::read(REGISTER_ADDRESS, MMRQ_BIT_OFFSET, MMRQ_BIT_WIDTH) as u8 }
	/// Mute mode request (Width: 1, Offset: 2)
	pub fn set_mmrq(value: u8) { ::write(REGISTER_ADDRESS, MMRQ_BIT_OFFSET, MMRQ_BIT_WIDTH, value as u32); }

	const SBKRQ_BIT_OFFSET: u8 = 1;
	const SBKRQ_BIT_WIDTH: u8 = 1;
	/// Send break request (Width: 1, Offset: 1)
	pub fn get_sbkrq() -> u8 { ::read(REGISTER_ADDRESS, SBKRQ_BIT_OFFSET, SBKRQ_BIT_WIDTH) as u8 }
	/// Send break request (Width: 1, Offset: 1)
	pub fn set_sbkrq(value: u8) { ::write(REGISTER_ADDRESS, SBKRQ_BIT_OFFSET, SBKRQ_BIT_WIDTH, value as u32); }

	const ABRRQ_BIT_OFFSET: u8 = 0;
	const ABRRQ_BIT_WIDTH: u8 = 1;
	/// Auto baud rate request (Width: 1, Offset: 0)
	pub fn get_abrrq() -> u8 { ::read(REGISTER_ADDRESS, ABRRQ_BIT_OFFSET, ABRRQ_BIT_WIDTH) as u8 }
	/// Auto baud rate request (Width: 1, Offset: 0)
	pub fn set_abrrq(value: u8) { ::write(REGISTER_ADDRESS, ABRRQ_BIT_OFFSET, ABRRQ_BIT_WIDTH, value as u32); }
}
/// Interrupt & status register
/// Size: 0x20 bits
pub mod isr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const REACK_BIT_OFFSET: u8 = 22;
	const REACK_BIT_WIDTH: u8 = 1;
	/// Receive enable acknowledge flag (Width: 1, Offset: 22)
	pub fn get_reack() -> u8 { ::read(REGISTER_ADDRESS, REACK_BIT_OFFSET, REACK_BIT_WIDTH) as u8 }

	const TEACK_BIT_OFFSET: u8 = 21;
	const TEACK_BIT_WIDTH: u8 = 1;
	/// Transmit enable acknowledge flag (Width: 1, Offset: 21)
	pub fn get_teack() -> u8 { ::read(REGISTER_ADDRESS, TEACK_BIT_OFFSET, TEACK_BIT_WIDTH) as u8 }

	const WUF_BIT_OFFSET: u8 = 20;
	const WUF_BIT_WIDTH: u8 = 1;
	/// Wakeup from Stop mode flag (Width: 1, Offset: 20)
	pub fn get_wuf() -> u8 { ::read(REGISTER_ADDRESS, WUF_BIT_OFFSET, WUF_BIT_WIDTH) as u8 }

	const RWU_BIT_OFFSET: u8 = 19;
	const RWU_BIT_WIDTH: u8 = 1;
	/// Receiver wakeup from Mute mode (Width: 1, Offset: 19)
	pub fn get_rwu() -> u8 { ::read(REGISTER_ADDRESS, RWU_BIT_OFFSET, RWU_BIT_WIDTH) as u8 }

	const SBKF_BIT_OFFSET: u8 = 18;
	const SBKF_BIT_WIDTH: u8 = 1;
	/// Send break flag (Width: 1, Offset: 18)
	pub fn get_sbkf() -> u8 { ::read(REGISTER_ADDRESS, SBKF_BIT_OFFSET, SBKF_BIT_WIDTH) as u8 }

	const CMF_BIT_OFFSET: u8 = 17;
	const CMF_BIT_WIDTH: u8 = 1;
	/// character match flag (Width: 1, Offset: 17)
	pub fn get_cmf() -> u8 { ::read(REGISTER_ADDRESS, CMF_BIT_OFFSET, CMF_BIT_WIDTH) as u8 }

	const BUSY_BIT_OFFSET: u8 = 16;
	const BUSY_BIT_WIDTH: u8 = 1;
	/// Busy flag (Width: 1, Offset: 16)
	pub fn get_busy() -> u8 { ::read(REGISTER_ADDRESS, BUSY_BIT_OFFSET, BUSY_BIT_WIDTH) as u8 }

	const ABRF_BIT_OFFSET: u8 = 15;
	const ABRF_BIT_WIDTH: u8 = 1;
	/// Auto baud rate flag (Width: 1, Offset: 15)
	pub fn get_abrf() -> u8 { ::read(REGISTER_ADDRESS, ABRF_BIT_OFFSET, ABRF_BIT_WIDTH) as u8 }

	const ABRE_BIT_OFFSET: u8 = 14;
	const ABRE_BIT_WIDTH: u8 = 1;
	/// Auto baud rate error (Width: 1, Offset: 14)
	pub fn get_abre() -> u8 { ::read(REGISTER_ADDRESS, ABRE_BIT_OFFSET, ABRE_BIT_WIDTH) as u8 }

	const EOBF_BIT_OFFSET: u8 = 12;
	const EOBF_BIT_WIDTH: u8 = 1;
	/// End of block flag (Width: 1, Offset: 12)
	pub fn get_eobf() -> u8 { ::read(REGISTER_ADDRESS, EOBF_BIT_OFFSET, EOBF_BIT_WIDTH) as u8 }

	const RTOF_BIT_OFFSET: u8 = 11;
	const RTOF_BIT_WIDTH: u8 = 1;
	/// Receiver timeout (Width: 1, Offset: 11)
	pub fn get_rtof() -> u8 { ::read(REGISTER_ADDRESS, RTOF_BIT_OFFSET, RTOF_BIT_WIDTH) as u8 }

	const CTS_BIT_OFFSET: u8 = 10;
	const CTS_BIT_WIDTH: u8 = 1;
	/// CTS flag (Width: 1, Offset: 10)
	pub fn get_cts() -> u8 { ::read(REGISTER_ADDRESS, CTS_BIT_OFFSET, CTS_BIT_WIDTH) as u8 }

	const CTSIF_BIT_OFFSET: u8 = 9;
	const CTSIF_BIT_WIDTH: u8 = 1;
	/// CTS interrupt flag (Width: 1, Offset: 9)
	pub fn get_ctsif() -> u8 { ::read(REGISTER_ADDRESS, CTSIF_BIT_OFFSET, CTSIF_BIT_WIDTH) as u8 }

	const LBDF_BIT_OFFSET: u8 = 8;
	const LBDF_BIT_WIDTH: u8 = 1;
	/// LIN break detection flag (Width: 1, Offset: 8)
	pub fn get_lbdf() -> u8 { ::read(REGISTER_ADDRESS, LBDF_BIT_OFFSET, LBDF_BIT_WIDTH) as u8 }

	const TXE_BIT_OFFSET: u8 = 7;
	const TXE_BIT_WIDTH: u8 = 1;
	/// Transmit data register empty (Width: 1, Offset: 7)
	pub fn get_txe() -> u8 { ::read(REGISTER_ADDRESS, TXE_BIT_OFFSET, TXE_BIT_WIDTH) as u8 }

	const TC_BIT_OFFSET: u8 = 6;
	const TC_BIT_WIDTH: u8 = 1;
	/// Transmission complete (Width: 1, Offset: 6)
	pub fn get_tc() -> u8 { ::read(REGISTER_ADDRESS, TC_BIT_OFFSET, TC_BIT_WIDTH) as u8 }

	const RXNE_BIT_OFFSET: u8 = 5;
	const RXNE_BIT_WIDTH: u8 = 1;
	/// Read data register not empty (Width: 1, Offset: 5)
	pub fn get_rxne() -> u8 { ::read(REGISTER_ADDRESS, RXNE_BIT_OFFSET, RXNE_BIT_WIDTH) as u8 }

	const IDLE_BIT_OFFSET: u8 = 4;
	const IDLE_BIT_WIDTH: u8 = 1;
	/// Idle line detected (Width: 1, Offset: 4)
	pub fn get_idle() -> u8 { ::read(REGISTER_ADDRESS, IDLE_BIT_OFFSET, IDLE_BIT_WIDTH) as u8 }

	const ORE_BIT_OFFSET: u8 = 3;
	const ORE_BIT_WIDTH: u8 = 1;
	/// Overrun error (Width: 1, Offset: 3)
	pub fn get_ore() -> u8 { ::read(REGISTER_ADDRESS, ORE_BIT_OFFSET, ORE_BIT_WIDTH) as u8 }

	const NF_BIT_OFFSET: u8 = 2;
	const NF_BIT_WIDTH: u8 = 1;
	/// Noise detected flag (Width: 1, Offset: 2)
	pub fn get_nf() -> u8 { ::read(REGISTER_ADDRESS, NF_BIT_OFFSET, NF_BIT_WIDTH) as u8 }

	const FE_BIT_OFFSET: u8 = 1;
	const FE_BIT_WIDTH: u8 = 1;
	/// Framing error (Width: 1, Offset: 1)
	pub fn get_fe() -> u8 { ::read(REGISTER_ADDRESS, FE_BIT_OFFSET, FE_BIT_WIDTH) as u8 }

	const PE_BIT_OFFSET: u8 = 0;
	const PE_BIT_WIDTH: u8 = 1;
	/// Parity error (Width: 1, Offset: 0)
	pub fn get_pe() -> u8 { ::read(REGISTER_ADDRESS, PE_BIT_OFFSET, PE_BIT_WIDTH) as u8 }
}
/// Interrupt flag clear register
/// Size: 0x20 bits
pub mod icr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x20;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const WUCF_BIT_OFFSET: u8 = 20;
	const WUCF_BIT_WIDTH: u8 = 1;
	/// Wakeup from Stop mode clear flag (Width: 1, Offset: 20)
	pub fn get_wucf() -> u8 { ::read(REGISTER_ADDRESS, WUCF_BIT_OFFSET, WUCF_BIT_WIDTH) as u8 }
	/// Wakeup from Stop mode clear flag (Width: 1, Offset: 20)
	pub fn set_wucf(value: u8) { ::write(REGISTER_ADDRESS, WUCF_BIT_OFFSET, WUCF_BIT_WIDTH, value as u32); }

	const CMCF_BIT_OFFSET: u8 = 17;
	const CMCF_BIT_WIDTH: u8 = 1;
	/// Character match clear flag (Width: 1, Offset: 17)
	pub fn get_cmcf() -> u8 { ::read(REGISTER_ADDRESS, CMCF_BIT_OFFSET, CMCF_BIT_WIDTH) as u8 }
	/// Character match clear flag (Width: 1, Offset: 17)
	pub fn set_cmcf(value: u8) { ::write(REGISTER_ADDRESS, CMCF_BIT_OFFSET, CMCF_BIT_WIDTH, value as u32); }

	const EOBCF_BIT_OFFSET: u8 = 12;
	const EOBCF_BIT_WIDTH: u8 = 1;
	/// End of timeout clear flag (Width: 1, Offset: 12)
	pub fn get_eobcf() -> u8 { ::read(REGISTER_ADDRESS, EOBCF_BIT_OFFSET, EOBCF_BIT_WIDTH) as u8 }
	/// End of timeout clear flag (Width: 1, Offset: 12)
	pub fn set_eobcf(value: u8) { ::write(REGISTER_ADDRESS, EOBCF_BIT_OFFSET, EOBCF_BIT_WIDTH, value as u32); }

	const RTOCF_BIT_OFFSET: u8 = 11;
	const RTOCF_BIT_WIDTH: u8 = 1;
	/// Receiver timeout clear flag (Width: 1, Offset: 11)
	pub fn get_rtocf() -> u8 { ::read(REGISTER_ADDRESS, RTOCF_BIT_OFFSET, RTOCF_BIT_WIDTH) as u8 }
	/// Receiver timeout clear flag (Width: 1, Offset: 11)
	pub fn set_rtocf(value: u8) { ::write(REGISTER_ADDRESS, RTOCF_BIT_OFFSET, RTOCF_BIT_WIDTH, value as u32); }

	const CTSCF_BIT_OFFSET: u8 = 9;
	const CTSCF_BIT_WIDTH: u8 = 1;
	/// CTS clear flag (Width: 1, Offset: 9)
	pub fn get_ctscf() -> u8 { ::read(REGISTER_ADDRESS, CTSCF_BIT_OFFSET, CTSCF_BIT_WIDTH) as u8 }
	/// CTS clear flag (Width: 1, Offset: 9)
	pub fn set_ctscf(value: u8) { ::write(REGISTER_ADDRESS, CTSCF_BIT_OFFSET, CTSCF_BIT_WIDTH, value as u32); }

	const LBDCF_BIT_OFFSET: u8 = 8;
	const LBDCF_BIT_WIDTH: u8 = 1;
	/// LIN break detection clear flag (Width: 1, Offset: 8)
	pub fn get_lbdcf() -> u8 { ::read(REGISTER_ADDRESS, LBDCF_BIT_OFFSET, LBDCF_BIT_WIDTH) as u8 }
	/// LIN break detection clear flag (Width: 1, Offset: 8)
	pub fn set_lbdcf(value: u8) { ::write(REGISTER_ADDRESS, LBDCF_BIT_OFFSET, LBDCF_BIT_WIDTH, value as u32); }

	const TCCF_BIT_OFFSET: u8 = 6;
	const TCCF_BIT_WIDTH: u8 = 1;
	/// Transmission complete clear flag (Width: 1, Offset: 6)
	pub fn get_tccf() -> u8 { ::read(REGISTER_ADDRESS, TCCF_BIT_OFFSET, TCCF_BIT_WIDTH) as u8 }
	/// Transmission complete clear flag (Width: 1, Offset: 6)
	pub fn set_tccf(value: u8) { ::write(REGISTER_ADDRESS, TCCF_BIT_OFFSET, TCCF_BIT_WIDTH, value as u32); }

	const IDLECF_BIT_OFFSET: u8 = 4;
	const IDLECF_BIT_WIDTH: u8 = 1;
	/// Idle line detected clear flag (Width: 1, Offset: 4)
	pub fn get_idlecf() -> u8 { ::read(REGISTER_ADDRESS, IDLECF_BIT_OFFSET, IDLECF_BIT_WIDTH) as u8 }
	/// Idle line detected clear flag (Width: 1, Offset: 4)
	pub fn set_idlecf(value: u8) { ::write(REGISTER_ADDRESS, IDLECF_BIT_OFFSET, IDLECF_BIT_WIDTH, value as u32); }

	const ORECF_BIT_OFFSET: u8 = 3;
	const ORECF_BIT_WIDTH: u8 = 1;
	/// Overrun error clear flag (Width: 1, Offset: 3)
	pub fn get_orecf() -> u8 { ::read(REGISTER_ADDRESS, ORECF_BIT_OFFSET, ORECF_BIT_WIDTH) as u8 }
	/// Overrun error clear flag (Width: 1, Offset: 3)
	pub fn set_orecf(value: u8) { ::write(REGISTER_ADDRESS, ORECF_BIT_OFFSET, ORECF_BIT_WIDTH, value as u32); }

	const NCF_BIT_OFFSET: u8 = 2;
	const NCF_BIT_WIDTH: u8 = 1;
	/// Noise detected clear flag (Width: 1, Offset: 2)
	pub fn get_ncf() -> u8 { ::read(REGISTER_ADDRESS, NCF_BIT_OFFSET, NCF_BIT_WIDTH) as u8 }
	/// Noise detected clear flag (Width: 1, Offset: 2)
	pub fn set_ncf(value: u8) { ::write(REGISTER_ADDRESS, NCF_BIT_OFFSET, NCF_BIT_WIDTH, value as u32); }

	const FECF_BIT_OFFSET: u8 = 1;
	const FECF_BIT_WIDTH: u8 = 1;
	/// Framing error clear flag (Width: 1, Offset: 1)
	pub fn get_fecf() -> u8 { ::read(REGISTER_ADDRESS, FECF_BIT_OFFSET, FECF_BIT_WIDTH) as u8 }
	/// Framing error clear flag (Width: 1, Offset: 1)
	pub fn set_fecf(value: u8) { ::write(REGISTER_ADDRESS, FECF_BIT_OFFSET, FECF_BIT_WIDTH, value as u32); }

	const PECF_BIT_OFFSET: u8 = 0;
	const PECF_BIT_WIDTH: u8 = 1;
	/// Parity error clear flag (Width: 1, Offset: 0)
	pub fn get_pecf() -> u8 { ::read(REGISTER_ADDRESS, PECF_BIT_OFFSET, PECF_BIT_WIDTH) as u8 }
	/// Parity error clear flag (Width: 1, Offset: 0)
	pub fn set_pecf(value: u8) { ::write(REGISTER_ADDRESS, PECF_BIT_OFFSET, PECF_BIT_WIDTH, value as u32); }
}
/// Receive data register
/// Size: 0x20 bits
pub mod rdr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x24;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const RDR_BIT_OFFSET: u8 = 0;
	const RDR_BIT_WIDTH: u8 = 9;
	/// Receive data value (Width: 9, Offset: 0)
	pub fn get_rdr() -> u16 { ::read(REGISTER_ADDRESS, RDR_BIT_OFFSET, RDR_BIT_WIDTH) as u16 }
}
/// Transmit data register
/// Size: 0x20 bits
pub mod tdr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x28;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TDR_BIT_OFFSET: u8 = 0;
	const TDR_BIT_WIDTH: u8 = 9;
	/// Transmit data value (Width: 9, Offset: 0)
	pub fn get_tdr() -> u16 { ::read(REGISTER_ADDRESS, TDR_BIT_OFFSET, TDR_BIT_WIDTH) as u16 }
	/// Transmit data value (Width: 9, Offset: 0)
	pub fn set_tdr(value: u16) { ::write(REGISTER_ADDRESS, TDR_BIT_OFFSET, TDR_BIT_WIDTH, value as u32); }
}
/// USART1 global interrupt and EXTI Line 25 interrupt
pub const INTERRUPT_USART1_EXTI25: u32 = 37;

/// UART4 global and EXTI Line 34 interrupts
pub const INTERRUPT_UART4_EXTI34: u32 = 52;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" derivedFrom="USART1">
  <name>UART4</name>
  <description>Universal synchronous asynchronous receiver
      transmitter</description>
  <groupName>USART</groupName>
  <baseAddress>0x40004C00</baseAddress>
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
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>EOBIE</name>
          <description>End of Block interrupt
              enable</description>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RTOIE</name>
          <description>Receiver timeout interrupt
              enable</description>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DEAT</name>
          <description>Driver Enable assertion
              time</description>
          <bitOffset>21</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>DEDT</name>
          <description>Driver Enable deassertion
              time</description>
          <bitOffset>16</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>OVER8</name>
          <description>Oversampling mode</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CMIE</name>
          <description>Character match interrupt
              enable</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MME</name>
          <description>Mute mode enable</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>M</name>
          <description>Word length</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>WAKE</name>
          <description>Receiver wakeup method</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PCE</name>
          <description>Parity control enable</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PS</name>
          <description>Parity selection</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PEIE</name>
          <description>PE interrupt enable</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TXEIE</name>
          <description>interrupt enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCIE</name>
          <description>Transmission complete interrupt
              enable</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RXNEIE</name>
          <description>RXNE interrupt enable</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDLEIE</name>
          <description>IDLE interrupt enable</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TE</name>
          <description>Transmitter enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RE</name>
          <description>Receiver enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>UESM</name>
          <description>USART enable in Stop mode</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>UE</name>
          <description>USART enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
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
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>ADD4</name>
          <description>Address of the USART node</description>
          <bitOffset>28</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>ADD0</name>
          <description>Address of the USART node</description>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>RTOEN</name>
          <description>Receiver timeout enable</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ABRMOD</name>
          <description>Auto baud rate mode</description>
          <bitOffset>21</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>ABREN</name>
          <description>Auto baud rate enable</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MSBFIRST</name>
          <description>Most significant bit first</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DATAINV</name>
          <description>Binary data inversion</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TXINV</name>
          <description>TX pin active level
              inversion</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RXINV</name>
          <description>RX pin active level
              inversion</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SWAP</name>
          <description>Swap TX/RX pins</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LINEN</name>
          <description>LIN mode enable</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>STOP</name>
          <description>STOP bits</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>CLKEN</name>
          <description>Clock enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CPOL</name>
          <description>Clock polarity</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CPHA</name>
          <description>Clock phase</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LBCL</name>
          <description>Last bit clock pulse</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LBDIE</name>
          <description>LIN break detection interrupt
              enable</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LBDL</name>
          <description>LIN break detection length</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADDM7</name>
          <description>7-bit Address Detection/4-bit Address
              Detection</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CR3</name>
      <displayName>CR3</displayName>
      <description>Control register 3</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>WUFIE</name>
          <description>Wakeup from Stop mode interrupt
              enable</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>WUS</name>
          <description>Wakeup from Stop mode interrupt flag
              selection</description>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>SCARCNT</name>
          <description>Smartcard auto-retry count</description>
          <bitOffset>17</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>DEP</name>
          <description>Driver enable polarity
              selection</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DEM</name>
          <description>Driver enable mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DDRE</name>
          <description>DMA Disable on Reception
              Error</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OVRDIS</name>
          <description>Overrun Disable</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ONEBIT</name>
          <description>One sample bit method
              enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTSIE</name>
          <description>CTS interrupt enable</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTSE</name>
          <description>CTS enable</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RTSE</name>
          <description>RTS enable</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DMAT</name>
          <description>DMA enable transmitter</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DMAR</name>
          <description>DMA enable receiver</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SCEN</name>
          <description>Smartcard mode enable</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>NACK</name>
          <description>Smartcard NACK enable</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HDSEL</name>
          <description>Half-duplex selection</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IRLP</name>
          <description>IrDA low-power</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IREN</name>
          <description>IrDA mode enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EIE</name>
          <description>Error interrupt enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BRR</name>
      <displayName>BRR</displayName>
      <description>Baud rate register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>DIV_Mantissa</name>
          <description>mantissa of USARTDIV</description>
          <bitOffset>4</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
        <field>
          <name>DIV_Fraction</name>
          <description>fraction of USARTDIV</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>GTPR</name>
      <displayName>GTPR</displayName>
      <description>Guard time and prescaler
          register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>GT</name>
          <description>Guard time value</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>PSC</name>
          <description>Prescaler value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>RTOR</name>
      <displayName>RTOR</displayName>
      <description>Receiver timeout register</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>BLEN</name>
          <description>Block Length</description>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>RTO</name>
          <description>Receiver timeout value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>24</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>RQR</name>
      <displayName>RQR</displayName>
      <description>Request register</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>TXFRQ</name>
          <description>Transmit data flush
              request</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RXFRQ</name>
          <description>Receive data flush request</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MMRQ</name>
          <description>Mute mode request</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SBKRQ</name>
          <description>Send break request</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ABRRQ</name>
          <description>Auto baud rate request</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ISR</name>
      <displayName>ISR</displayName>
      <description>Interrupt &amp; status
          register</description>
      <addressOffset>0x1C</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00C0</resetValue>
      <fields>
        <field>
          <name>REACK</name>
          <description>Receive enable acknowledge
              flag</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TEACK</name>
          <description>Transmit enable acknowledge
              flag</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>WUF</name>
          <description>Wakeup from Stop mode flag</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RWU</name>
          <description>Receiver wakeup from Mute
              mode</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SBKF</name>
          <description>Send break flag</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CMF</name>
          <description>character match flag</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BUSY</name>
          <description>Busy flag</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ABRF</name>
          <description>Auto baud rate flag</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ABRE</name>
          <description>Auto baud rate error</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOBF</name>
          <description>End of block flag</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RTOF</name>
          <description>Receiver timeout</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTS</name>
          <description>CTS flag</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTSIF</name>
          <description>CTS interrupt flag</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LBDF</name>
          <description>LIN break detection flag</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TXE</name>
          <description>Transmit data register
              empty</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TC</name>
          <description>Transmission complete</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RXNE</name>
          <description>Read data register not
              empty</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDLE</name>
          <description>Idle line detected</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ORE</name>
          <description>Overrun error</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>NF</name>
          <description>Noise detected flag</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>FE</name>
          <description>Framing error</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PE</name>
          <description>Parity error</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ICR</name>
      <displayName>ICR</displayName>
      <description>Interrupt flag clear register</description>
      <addressOffset>0x20</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>WUCF</name>
          <description>Wakeup from Stop mode clear
              flag</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CMCF</name>
          <description>Character match clear flag</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOBCF</name>
          <description>End of timeout clear flag</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RTOCF</name>
          <description>Receiver timeout clear
              flag</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTSCF</name>
          <description>CTS clear flag</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LBDCF</name>
          <description>LIN break detection clear
              flag</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TCCF</name>
          <description>Transmission complete clear
              flag</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IDLECF</name>
          <description>Idle line detected clear
              flag</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ORECF</name>
          <description>Overrun error clear flag</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>NCF</name>
          <description>Noise detected clear flag</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>FECF</name>
          <description>Framing error clear flag</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PECF</name>
          <description>Parity error clear flag</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>RDR</name>
      <displayName>RDR</displayName>
      <description>Receive data register</description>
      <addressOffset>0x24</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>RDR</name>
          <description>Receive data value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>9</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>TDR</name>
      <displayName>TDR</displayName>
      <description>Transmit data register</description>
      <addressOffset>0x28</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>TDR</name>
          <description>Transmit data value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>9</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>USART1_EXTI25</name>
    <description>USART1 global interrupt and EXTI Line 25
        interrupt</description>
    <value>37</value>
  </interrupt>
  <interrupt>
    <name>UART4_EXTI34</name>
    <description>UART4 global and EXTI Line 34
        interrupts</description>
    <value>52</value>
  </interrupt>
</peripheral>*/
