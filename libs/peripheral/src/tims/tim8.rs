/// MOD TIM8
/// Advanced-timers
const BASE_ADDRESS: u32 = 0x40013400;
/// control register 1
/// Size: 0x20 bits
pub mod cr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CEN_BIT_OFFSET: u8 = 0;
	const CEN_BIT_WIDTH: u8 = 1;
	/// Counter enable (Width: 1, Offset: 0)
	pub fn get_cen() -> u8 { ::read(REGISTER_ADDRESS, CEN_BIT_OFFSET, CEN_BIT_WIDTH) as u8 }
	/// Counter enable (Width: 1, Offset: 0)
	pub fn set_cen(value: u8) { ::write(REGISTER_ADDRESS, CEN_BIT_OFFSET, CEN_BIT_WIDTH, value as u32); }

	const UDIS_BIT_OFFSET: u8 = 1;
	const UDIS_BIT_WIDTH: u8 = 1;
	/// Update disable (Width: 1, Offset: 1)
	pub fn get_udis() -> u8 { ::read(REGISTER_ADDRESS, UDIS_BIT_OFFSET, UDIS_BIT_WIDTH) as u8 }
	/// Update disable (Width: 1, Offset: 1)
	pub fn set_udis(value: u8) { ::write(REGISTER_ADDRESS, UDIS_BIT_OFFSET, UDIS_BIT_WIDTH, value as u32); }

	const URS_BIT_OFFSET: u8 = 2;
	const URS_BIT_WIDTH: u8 = 1;
	/// Update request source (Width: 1, Offset: 2)
	pub fn get_urs() -> u8 { ::read(REGISTER_ADDRESS, URS_BIT_OFFSET, URS_BIT_WIDTH) as u8 }
	/// Update request source (Width: 1, Offset: 2)
	pub fn set_urs(value: u8) { ::write(REGISTER_ADDRESS, URS_BIT_OFFSET, URS_BIT_WIDTH, value as u32); }

	const OPM_BIT_OFFSET: u8 = 3;
	const OPM_BIT_WIDTH: u8 = 1;
	/// One-pulse mode (Width: 1, Offset: 3)
	pub fn get_opm() -> u8 { ::read(REGISTER_ADDRESS, OPM_BIT_OFFSET, OPM_BIT_WIDTH) as u8 }
	/// One-pulse mode (Width: 1, Offset: 3)
	pub fn set_opm(value: u8) { ::write(REGISTER_ADDRESS, OPM_BIT_OFFSET, OPM_BIT_WIDTH, value as u32); }

	const DIR_BIT_OFFSET: u8 = 4;
	const DIR_BIT_WIDTH: u8 = 1;
	/// Direction (Width: 1, Offset: 4)
	pub fn get_dir() -> u8 { ::read(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH) as u8 }
	/// Direction (Width: 1, Offset: 4)
	pub fn set_dir(value: u8) { ::write(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH, value as u32); }

	const CMS_BIT_OFFSET: u8 = 5;
	const CMS_BIT_WIDTH: u8 = 2;
	/// Center-aligned mode selection (Width: 2, Offset: 5)
	pub fn get_cms() -> u8 { ::read(REGISTER_ADDRESS, CMS_BIT_OFFSET, CMS_BIT_WIDTH) as u8 }
	/// Center-aligned mode selection (Width: 2, Offset: 5)
	pub fn set_cms(value: u8) { ::write(REGISTER_ADDRESS, CMS_BIT_OFFSET, CMS_BIT_WIDTH, value as u32); }

	const ARPE_BIT_OFFSET: u8 = 7;
	const ARPE_BIT_WIDTH: u8 = 1;
	/// Auto-reload preload enable (Width: 1, Offset: 7)
	pub fn get_arpe() -> u8 { ::read(REGISTER_ADDRESS, ARPE_BIT_OFFSET, ARPE_BIT_WIDTH) as u8 }
	/// Auto-reload preload enable (Width: 1, Offset: 7)
	pub fn set_arpe(value: u8) { ::write(REGISTER_ADDRESS, ARPE_BIT_OFFSET, ARPE_BIT_WIDTH, value as u32); }

	const CKD_BIT_OFFSET: u8 = 8;
	const CKD_BIT_WIDTH: u8 = 2;
	/// Clock division (Width: 2, Offset: 8)
	pub fn get_ckd() -> u8 { ::read(REGISTER_ADDRESS, CKD_BIT_OFFSET, CKD_BIT_WIDTH) as u8 }
	/// Clock division (Width: 2, Offset: 8)
	pub fn set_ckd(value: u8) { ::write(REGISTER_ADDRESS, CKD_BIT_OFFSET, CKD_BIT_WIDTH, value as u32); }

	const UIFREMAP_BIT_OFFSET: u8 = 11;
	const UIFREMAP_BIT_WIDTH: u8 = 1;
	/// UIF status bit remapping (Width: 1, Offset: 11)
	pub fn get_uifremap() -> u8 { ::read(REGISTER_ADDRESS, UIFREMAP_BIT_OFFSET, UIFREMAP_BIT_WIDTH) as u8 }
	/// UIF status bit remapping (Width: 1, Offset: 11)
	pub fn set_uifremap(value: u8) { ::write(REGISTER_ADDRESS, UIFREMAP_BIT_OFFSET, UIFREMAP_BIT_WIDTH, value as u32); }
}
/// control register 2
/// Size: 0x20 bits
pub mod cr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CCPC_BIT_OFFSET: u8 = 0;
	const CCPC_BIT_WIDTH: u8 = 1;
	/// Capture/compare preloaded control (Width: 1, Offset: 0)
	pub fn get_ccpc() -> u8 { ::read(REGISTER_ADDRESS, CCPC_BIT_OFFSET, CCPC_BIT_WIDTH) as u8 }
	/// Capture/compare preloaded control (Width: 1, Offset: 0)
	pub fn set_ccpc(value: u8) { ::write(REGISTER_ADDRESS, CCPC_BIT_OFFSET, CCPC_BIT_WIDTH, value as u32); }

	const CCUS_BIT_OFFSET: u8 = 2;
	const CCUS_BIT_WIDTH: u8 = 1;
	/// Capture/compare control update selection (Width: 1, Offset: 2)
	pub fn get_ccus() -> u8 { ::read(REGISTER_ADDRESS, CCUS_BIT_OFFSET, CCUS_BIT_WIDTH) as u8 }
	/// Capture/compare control update selection (Width: 1, Offset: 2)
	pub fn set_ccus(value: u8) { ::write(REGISTER_ADDRESS, CCUS_BIT_OFFSET, CCUS_BIT_WIDTH, value as u32); }

	const CCDS_BIT_OFFSET: u8 = 3;
	const CCDS_BIT_WIDTH: u8 = 1;
	/// Capture/compare DMA selection (Width: 1, Offset: 3)
	pub fn get_ccds() -> u8 { ::read(REGISTER_ADDRESS, CCDS_BIT_OFFSET, CCDS_BIT_WIDTH) as u8 }
	/// Capture/compare DMA selection (Width: 1, Offset: 3)
	pub fn set_ccds(value: u8) { ::write(REGISTER_ADDRESS, CCDS_BIT_OFFSET, CCDS_BIT_WIDTH, value as u32); }

	const MMS_BIT_OFFSET: u8 = 4;
	const MMS_BIT_WIDTH: u8 = 3;
	/// Master mode selection (Width: 3, Offset: 4)
	pub fn get_mms() -> u8 { ::read(REGISTER_ADDRESS, MMS_BIT_OFFSET, MMS_BIT_WIDTH) as u8 }
	/// Master mode selection (Width: 3, Offset: 4)
	pub fn set_mms(value: u8) { ::write(REGISTER_ADDRESS, MMS_BIT_OFFSET, MMS_BIT_WIDTH, value as u32); }

	const TI1S_BIT_OFFSET: u8 = 7;
	const TI1S_BIT_WIDTH: u8 = 1;
	/// TI1 selection (Width: 1, Offset: 7)
	pub fn get_ti1s() -> u8 { ::read(REGISTER_ADDRESS, TI1S_BIT_OFFSET, TI1S_BIT_WIDTH) as u8 }
	/// TI1 selection (Width: 1, Offset: 7)
	pub fn set_ti1s(value: u8) { ::write(REGISTER_ADDRESS, TI1S_BIT_OFFSET, TI1S_BIT_WIDTH, value as u32); }

	const OIS1_BIT_OFFSET: u8 = 8;
	const OIS1_BIT_WIDTH: u8 = 1;
	/// Output Idle state 1 (Width: 1, Offset: 8)
	pub fn get_ois1() -> u8 { ::read(REGISTER_ADDRESS, OIS1_BIT_OFFSET, OIS1_BIT_WIDTH) as u8 }
	/// Output Idle state 1 (Width: 1, Offset: 8)
	pub fn set_ois1(value: u8) { ::write(REGISTER_ADDRESS, OIS1_BIT_OFFSET, OIS1_BIT_WIDTH, value as u32); }

	const OIS1N_BIT_OFFSET: u8 = 9;
	const OIS1N_BIT_WIDTH: u8 = 1;
	/// Output Idle state 1 (Width: 1, Offset: 9)
	pub fn get_ois1n() -> u8 { ::read(REGISTER_ADDRESS, OIS1N_BIT_OFFSET, OIS1N_BIT_WIDTH) as u8 }
	/// Output Idle state 1 (Width: 1, Offset: 9)
	pub fn set_ois1n(value: u8) { ::write(REGISTER_ADDRESS, OIS1N_BIT_OFFSET, OIS1N_BIT_WIDTH, value as u32); }

	const OIS2_BIT_OFFSET: u8 = 10;
	const OIS2_BIT_WIDTH: u8 = 1;
	/// Output Idle state 2 (Width: 1, Offset: 10)
	pub fn get_ois2() -> u8 { ::read(REGISTER_ADDRESS, OIS2_BIT_OFFSET, OIS2_BIT_WIDTH) as u8 }
	/// Output Idle state 2 (Width: 1, Offset: 10)
	pub fn set_ois2(value: u8) { ::write(REGISTER_ADDRESS, OIS2_BIT_OFFSET, OIS2_BIT_WIDTH, value as u32); }

	const OIS2N_BIT_OFFSET: u8 = 11;
	const OIS2N_BIT_WIDTH: u8 = 1;
	/// Output Idle state 2 (Width: 1, Offset: 11)
	pub fn get_ois2n() -> u8 { ::read(REGISTER_ADDRESS, OIS2N_BIT_OFFSET, OIS2N_BIT_WIDTH) as u8 }
	/// Output Idle state 2 (Width: 1, Offset: 11)
	pub fn set_ois2n(value: u8) { ::write(REGISTER_ADDRESS, OIS2N_BIT_OFFSET, OIS2N_BIT_WIDTH, value as u32); }

	const OIS3_BIT_OFFSET: u8 = 12;
	const OIS3_BIT_WIDTH: u8 = 1;
	/// Output Idle state 3 (Width: 1, Offset: 12)
	pub fn get_ois3() -> u8 { ::read(REGISTER_ADDRESS, OIS3_BIT_OFFSET, OIS3_BIT_WIDTH) as u8 }
	/// Output Idle state 3 (Width: 1, Offset: 12)
	pub fn set_ois3(value: u8) { ::write(REGISTER_ADDRESS, OIS3_BIT_OFFSET, OIS3_BIT_WIDTH, value as u32); }

	const OIS3N_BIT_OFFSET: u8 = 13;
	const OIS3N_BIT_WIDTH: u8 = 1;
	/// Output Idle state 3 (Width: 1, Offset: 13)
	pub fn get_ois3n() -> u8 { ::read(REGISTER_ADDRESS, OIS3N_BIT_OFFSET, OIS3N_BIT_WIDTH) as u8 }
	/// Output Idle state 3 (Width: 1, Offset: 13)
	pub fn set_ois3n(value: u8) { ::write(REGISTER_ADDRESS, OIS3N_BIT_OFFSET, OIS3N_BIT_WIDTH, value as u32); }

	const OIS4_BIT_OFFSET: u8 = 14;
	const OIS4_BIT_WIDTH: u8 = 1;
	/// Output Idle state 4 (Width: 1, Offset: 14)
	pub fn get_ois4() -> u8 { ::read(REGISTER_ADDRESS, OIS4_BIT_OFFSET, OIS4_BIT_WIDTH) as u8 }
	/// Output Idle state 4 (Width: 1, Offset: 14)
	pub fn set_ois4(value: u8) { ::write(REGISTER_ADDRESS, OIS4_BIT_OFFSET, OIS4_BIT_WIDTH, value as u32); }

	const OIS5_BIT_OFFSET: u8 = 16;
	const OIS5_BIT_WIDTH: u8 = 1;
	/// Output Idle state 5 (Width: 1, Offset: 16)
	pub fn get_ois5() -> u8 { ::read(REGISTER_ADDRESS, OIS5_BIT_OFFSET, OIS5_BIT_WIDTH) as u8 }
	/// Output Idle state 5 (Width: 1, Offset: 16)
	pub fn set_ois5(value: u8) { ::write(REGISTER_ADDRESS, OIS5_BIT_OFFSET, OIS5_BIT_WIDTH, value as u32); }

	const OIS6_BIT_OFFSET: u8 = 18;
	const OIS6_BIT_WIDTH: u8 = 1;
	/// Output Idle state 6 (Width: 1, Offset: 18)
	pub fn get_ois6() -> u8 { ::read(REGISTER_ADDRESS, OIS6_BIT_OFFSET, OIS6_BIT_WIDTH) as u8 }
	/// Output Idle state 6 (Width: 1, Offset: 18)
	pub fn set_ois6(value: u8) { ::write(REGISTER_ADDRESS, OIS6_BIT_OFFSET, OIS6_BIT_WIDTH, value as u32); }

	const MMS2_BIT_OFFSET: u8 = 20;
	const MMS2_BIT_WIDTH: u8 = 4;
	/// Master mode selection 2 (Width: 4, Offset: 20)
	pub fn get_mms2() -> u8 { ::read(REGISTER_ADDRESS, MMS2_BIT_OFFSET, MMS2_BIT_WIDTH) as u8 }
	/// Master mode selection 2 (Width: 4, Offset: 20)
	pub fn set_mms2(value: u8) { ::write(REGISTER_ADDRESS, MMS2_BIT_OFFSET, MMS2_BIT_WIDTH, value as u32); }
}
/// slave mode control register
/// Size: 0x20 bits
pub mod smcr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SMS_BIT_OFFSET: u8 = 0;
	const SMS_BIT_WIDTH: u8 = 3;
	/// Slave mode selection (Width: 3, Offset: 0)
	pub fn get_sms() -> u8 { ::read(REGISTER_ADDRESS, SMS_BIT_OFFSET, SMS_BIT_WIDTH) as u8 }
	/// Slave mode selection (Width: 3, Offset: 0)
	pub fn set_sms(value: u8) { ::write(REGISTER_ADDRESS, SMS_BIT_OFFSET, SMS_BIT_WIDTH, value as u32); }

	const OCCS_BIT_OFFSET: u8 = 3;
	const OCCS_BIT_WIDTH: u8 = 1;
	/// OCREF clear selection (Width: 1, Offset: 3)
	pub fn get_occs() -> u8 { ::read(REGISTER_ADDRESS, OCCS_BIT_OFFSET, OCCS_BIT_WIDTH) as u8 }
	/// OCREF clear selection (Width: 1, Offset: 3)
	pub fn set_occs(value: u8) { ::write(REGISTER_ADDRESS, OCCS_BIT_OFFSET, OCCS_BIT_WIDTH, value as u32); }

	const TS_BIT_OFFSET: u8 = 4;
	const TS_BIT_WIDTH: u8 = 3;
	/// Trigger selection (Width: 3, Offset: 4)
	pub fn get_ts() -> u8 { ::read(REGISTER_ADDRESS, TS_BIT_OFFSET, TS_BIT_WIDTH) as u8 }
	/// Trigger selection (Width: 3, Offset: 4)
	pub fn set_ts(value: u8) { ::write(REGISTER_ADDRESS, TS_BIT_OFFSET, TS_BIT_WIDTH, value as u32); }

	const MSM_BIT_OFFSET: u8 = 7;
	const MSM_BIT_WIDTH: u8 = 1;
	/// Master/Slave mode (Width: 1, Offset: 7)
	pub fn get_msm() -> u8 { ::read(REGISTER_ADDRESS, MSM_BIT_OFFSET, MSM_BIT_WIDTH) as u8 }
	/// Master/Slave mode (Width: 1, Offset: 7)
	pub fn set_msm(value: u8) { ::write(REGISTER_ADDRESS, MSM_BIT_OFFSET, MSM_BIT_WIDTH, value as u32); }

	const ETF_BIT_OFFSET: u8 = 8;
	const ETF_BIT_WIDTH: u8 = 4;
	/// External trigger filter (Width: 4, Offset: 8)
	pub fn get_etf() -> u8 { ::read(REGISTER_ADDRESS, ETF_BIT_OFFSET, ETF_BIT_WIDTH) as u8 }
	/// External trigger filter (Width: 4, Offset: 8)
	pub fn set_etf(value: u8) { ::write(REGISTER_ADDRESS, ETF_BIT_OFFSET, ETF_BIT_WIDTH, value as u32); }

	const ETPS_BIT_OFFSET: u8 = 12;
	const ETPS_BIT_WIDTH: u8 = 2;
	/// External trigger prescaler (Width: 2, Offset: 12)
	pub fn get_etps() -> u8 { ::read(REGISTER_ADDRESS, ETPS_BIT_OFFSET, ETPS_BIT_WIDTH) as u8 }
	/// External trigger prescaler (Width: 2, Offset: 12)
	pub fn set_etps(value: u8) { ::write(REGISTER_ADDRESS, ETPS_BIT_OFFSET, ETPS_BIT_WIDTH, value as u32); }

	const ECE_BIT_OFFSET: u8 = 14;
	const ECE_BIT_WIDTH: u8 = 1;
	/// External clock enable (Width: 1, Offset: 14)
	pub fn get_ece() -> u8 { ::read(REGISTER_ADDRESS, ECE_BIT_OFFSET, ECE_BIT_WIDTH) as u8 }
	/// External clock enable (Width: 1, Offset: 14)
	pub fn set_ece(value: u8) { ::write(REGISTER_ADDRESS, ECE_BIT_OFFSET, ECE_BIT_WIDTH, value as u32); }

	const ETP_BIT_OFFSET: u8 = 15;
	const ETP_BIT_WIDTH: u8 = 1;
	/// External trigger polarity (Width: 1, Offset: 15)
	pub fn get_etp() -> u8 { ::read(REGISTER_ADDRESS, ETP_BIT_OFFSET, ETP_BIT_WIDTH) as u8 }
	/// External trigger polarity (Width: 1, Offset: 15)
	pub fn set_etp(value: u8) { ::write(REGISTER_ADDRESS, ETP_BIT_OFFSET, ETP_BIT_WIDTH, value as u32); }

	const SMS3_BIT_OFFSET: u8 = 16;
	const SMS3_BIT_WIDTH: u8 = 1;
	/// Slave mode selection bit 3 (Width: 1, Offset: 16)
	pub fn get_sms3() -> u8 { ::read(REGISTER_ADDRESS, SMS3_BIT_OFFSET, SMS3_BIT_WIDTH) as u8 }
	/// Slave mode selection bit 3 (Width: 1, Offset: 16)
	pub fn set_sms3(value: u8) { ::write(REGISTER_ADDRESS, SMS3_BIT_OFFSET, SMS3_BIT_WIDTH, value as u32); }
}
/// DMA/Interrupt enable register
/// Size: 0x20 bits
pub mod dier {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TDE_BIT_OFFSET: u8 = 14;
	const TDE_BIT_WIDTH: u8 = 1;
	/// Trigger DMA request enable (Width: 1, Offset: 14)
	pub fn get_tde() -> u8 { ::read(REGISTER_ADDRESS, TDE_BIT_OFFSET, TDE_BIT_WIDTH) as u8 }
	/// Trigger DMA request enable (Width: 1, Offset: 14)
	pub fn set_tde(value: u8) { ::write(REGISTER_ADDRESS, TDE_BIT_OFFSET, TDE_BIT_WIDTH, value as u32); }

	const COMDE_BIT_OFFSET: u8 = 13;
	const COMDE_BIT_WIDTH: u8 = 1;
	/// Reserved (Width: 1, Offset: 13)
	pub fn get_comde() -> u8 { ::read(REGISTER_ADDRESS, COMDE_BIT_OFFSET, COMDE_BIT_WIDTH) as u8 }
	/// Reserved (Width: 1, Offset: 13)
	pub fn set_comde(value: u8) { ::write(REGISTER_ADDRESS, COMDE_BIT_OFFSET, COMDE_BIT_WIDTH, value as u32); }

	const CC4DE_BIT_OFFSET: u8 = 12;
	const CC4DE_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 4 DMA request enable (Width: 1, Offset: 12)
	pub fn get_cc4de() -> u8 { ::read(REGISTER_ADDRESS, CC4DE_BIT_OFFSET, CC4DE_BIT_WIDTH) as u8 }
	/// Capture/Compare 4 DMA request enable (Width: 1, Offset: 12)
	pub fn set_cc4de(value: u8) { ::write(REGISTER_ADDRESS, CC4DE_BIT_OFFSET, CC4DE_BIT_WIDTH, value as u32); }

	const CC3DE_BIT_OFFSET: u8 = 11;
	const CC3DE_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 3 DMA request enable (Width: 1, Offset: 11)
	pub fn get_cc3de() -> u8 { ::read(REGISTER_ADDRESS, CC3DE_BIT_OFFSET, CC3DE_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 DMA request enable (Width: 1, Offset: 11)
	pub fn set_cc3de(value: u8) { ::write(REGISTER_ADDRESS, CC3DE_BIT_OFFSET, CC3DE_BIT_WIDTH, value as u32); }

	const CC2DE_BIT_OFFSET: u8 = 10;
	const CC2DE_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 2 DMA request enable (Width: 1, Offset: 10)
	pub fn get_cc2de() -> u8 { ::read(REGISTER_ADDRESS, CC2DE_BIT_OFFSET, CC2DE_BIT_WIDTH) as u8 }
	/// Capture/Compare 2 DMA request enable (Width: 1, Offset: 10)
	pub fn set_cc2de(value: u8) { ::write(REGISTER_ADDRESS, CC2DE_BIT_OFFSET, CC2DE_BIT_WIDTH, value as u32); }

	const CC1DE_BIT_OFFSET: u8 = 9;
	const CC1DE_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 DMA request enable (Width: 1, Offset: 9)
	pub fn get_cc1de() -> u8 { ::read(REGISTER_ADDRESS, CC1DE_BIT_OFFSET, CC1DE_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 DMA request enable (Width: 1, Offset: 9)
	pub fn set_cc1de(value: u8) { ::write(REGISTER_ADDRESS, CC1DE_BIT_OFFSET, CC1DE_BIT_WIDTH, value as u32); }

	const UDE_BIT_OFFSET: u8 = 8;
	const UDE_BIT_WIDTH: u8 = 1;
	/// Update DMA request enable (Width: 1, Offset: 8)
	pub fn get_ude() -> u8 { ::read(REGISTER_ADDRESS, UDE_BIT_OFFSET, UDE_BIT_WIDTH) as u8 }
	/// Update DMA request enable (Width: 1, Offset: 8)
	pub fn set_ude(value: u8) { ::write(REGISTER_ADDRESS, UDE_BIT_OFFSET, UDE_BIT_WIDTH, value as u32); }

	const BIE_BIT_OFFSET: u8 = 7;
	const BIE_BIT_WIDTH: u8 = 1;
	/// Break interrupt enable (Width: 1, Offset: 7)
	pub fn get_bie() -> u8 { ::read(REGISTER_ADDRESS, BIE_BIT_OFFSET, BIE_BIT_WIDTH) as u8 }
	/// Break interrupt enable (Width: 1, Offset: 7)
	pub fn set_bie(value: u8) { ::write(REGISTER_ADDRESS, BIE_BIT_OFFSET, BIE_BIT_WIDTH, value as u32); }

	const TIE_BIT_OFFSET: u8 = 6;
	const TIE_BIT_WIDTH: u8 = 1;
	/// Trigger interrupt enable (Width: 1, Offset: 6)
	pub fn get_tie() -> u8 { ::read(REGISTER_ADDRESS, TIE_BIT_OFFSET, TIE_BIT_WIDTH) as u8 }
	/// Trigger interrupt enable (Width: 1, Offset: 6)
	pub fn set_tie(value: u8) { ::write(REGISTER_ADDRESS, TIE_BIT_OFFSET, TIE_BIT_WIDTH, value as u32); }

	const COMIE_BIT_OFFSET: u8 = 5;
	const COMIE_BIT_WIDTH: u8 = 1;
	/// COM interrupt enable (Width: 1, Offset: 5)
	pub fn get_comie() -> u8 { ::read(REGISTER_ADDRESS, COMIE_BIT_OFFSET, COMIE_BIT_WIDTH) as u8 }
	/// COM interrupt enable (Width: 1, Offset: 5)
	pub fn set_comie(value: u8) { ::write(REGISTER_ADDRESS, COMIE_BIT_OFFSET, COMIE_BIT_WIDTH, value as u32); }

	const CC4IE_BIT_OFFSET: u8 = 4;
	const CC4IE_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 4 interrupt enable (Width: 1, Offset: 4)
	pub fn get_cc4ie() -> u8 { ::read(REGISTER_ADDRESS, CC4IE_BIT_OFFSET, CC4IE_BIT_WIDTH) as u8 }
	/// Capture/Compare 4 interrupt enable (Width: 1, Offset: 4)
	pub fn set_cc4ie(value: u8) { ::write(REGISTER_ADDRESS, CC4IE_BIT_OFFSET, CC4IE_BIT_WIDTH, value as u32); }

	const CC3IE_BIT_OFFSET: u8 = 3;
	const CC3IE_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 3 interrupt enable (Width: 1, Offset: 3)
	pub fn get_cc3ie() -> u8 { ::read(REGISTER_ADDRESS, CC3IE_BIT_OFFSET, CC3IE_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 interrupt enable (Width: 1, Offset: 3)
	pub fn set_cc3ie(value: u8) { ::write(REGISTER_ADDRESS, CC3IE_BIT_OFFSET, CC3IE_BIT_WIDTH, value as u32); }

	const CC2IE_BIT_OFFSET: u8 = 2;
	const CC2IE_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 2 interrupt enable (Width: 1, Offset: 2)
	pub fn get_cc2ie() -> u8 { ::read(REGISTER_ADDRESS, CC2IE_BIT_OFFSET, CC2IE_BIT_WIDTH) as u8 }
	/// Capture/Compare 2 interrupt enable (Width: 1, Offset: 2)
	pub fn set_cc2ie(value: u8) { ::write(REGISTER_ADDRESS, CC2IE_BIT_OFFSET, CC2IE_BIT_WIDTH, value as u32); }

	const CC1IE_BIT_OFFSET: u8 = 1;
	const CC1IE_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 interrupt enable (Width: 1, Offset: 1)
	pub fn get_cc1ie() -> u8 { ::read(REGISTER_ADDRESS, CC1IE_BIT_OFFSET, CC1IE_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 interrupt enable (Width: 1, Offset: 1)
	pub fn set_cc1ie(value: u8) { ::write(REGISTER_ADDRESS, CC1IE_BIT_OFFSET, CC1IE_BIT_WIDTH, value as u32); }

	const UIE_BIT_OFFSET: u8 = 0;
	const UIE_BIT_WIDTH: u8 = 1;
	/// Update interrupt enable (Width: 1, Offset: 0)
	pub fn get_uie() -> u8 { ::read(REGISTER_ADDRESS, UIE_BIT_OFFSET, UIE_BIT_WIDTH) as u8 }
	/// Update interrupt enable (Width: 1, Offset: 0)
	pub fn set_uie(value: u8) { ::write(REGISTER_ADDRESS, UIE_BIT_OFFSET, UIE_BIT_WIDTH, value as u32); }
}
/// status register
/// Size: 0x20 bits
pub mod sr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const UIF_BIT_OFFSET: u8 = 0;
	const UIF_BIT_WIDTH: u8 = 1;
	/// Update interrupt flag (Width: 1, Offset: 0)
	pub fn get_uif() -> u8 { ::read(REGISTER_ADDRESS, UIF_BIT_OFFSET, UIF_BIT_WIDTH) as u8 }
	/// Update interrupt flag (Width: 1, Offset: 0)
	pub fn set_uif(value: u8) { ::write(REGISTER_ADDRESS, UIF_BIT_OFFSET, UIF_BIT_WIDTH, value as u32); }

	const CC1IF_BIT_OFFSET: u8 = 1;
	const CC1IF_BIT_WIDTH: u8 = 1;
	/// Capture/compare 1 interrupt flag (Width: 1, Offset: 1)
	pub fn get_cc1if() -> u8 { ::read(REGISTER_ADDRESS, CC1IF_BIT_OFFSET, CC1IF_BIT_WIDTH) as u8 }
	/// Capture/compare 1 interrupt flag (Width: 1, Offset: 1)
	pub fn set_cc1if(value: u8) { ::write(REGISTER_ADDRESS, CC1IF_BIT_OFFSET, CC1IF_BIT_WIDTH, value as u32); }

	const CC2IF_BIT_OFFSET: u8 = 2;
	const CC2IF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 2 interrupt flag (Width: 1, Offset: 2)
	pub fn get_cc2if() -> u8 { ::read(REGISTER_ADDRESS, CC2IF_BIT_OFFSET, CC2IF_BIT_WIDTH) as u8 }
	/// Capture/Compare 2 interrupt flag (Width: 1, Offset: 2)
	pub fn set_cc2if(value: u8) { ::write(REGISTER_ADDRESS, CC2IF_BIT_OFFSET, CC2IF_BIT_WIDTH, value as u32); }

	const CC3IF_BIT_OFFSET: u8 = 3;
	const CC3IF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 3 interrupt flag (Width: 1, Offset: 3)
	pub fn get_cc3if() -> u8 { ::read(REGISTER_ADDRESS, CC3IF_BIT_OFFSET, CC3IF_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 interrupt flag (Width: 1, Offset: 3)
	pub fn set_cc3if(value: u8) { ::write(REGISTER_ADDRESS, CC3IF_BIT_OFFSET, CC3IF_BIT_WIDTH, value as u32); }

	const CC4IF_BIT_OFFSET: u8 = 4;
	const CC4IF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 4 interrupt flag (Width: 1, Offset: 4)
	pub fn get_cc4if() -> u8 { ::read(REGISTER_ADDRESS, CC4IF_BIT_OFFSET, CC4IF_BIT_WIDTH) as u8 }
	/// Capture/Compare 4 interrupt flag (Width: 1, Offset: 4)
	pub fn set_cc4if(value: u8) { ::write(REGISTER_ADDRESS, CC4IF_BIT_OFFSET, CC4IF_BIT_WIDTH, value as u32); }

	const COMIF_BIT_OFFSET: u8 = 5;
	const COMIF_BIT_WIDTH: u8 = 1;
	/// COM interrupt flag (Width: 1, Offset: 5)
	pub fn get_comif() -> u8 { ::read(REGISTER_ADDRESS, COMIF_BIT_OFFSET, COMIF_BIT_WIDTH) as u8 }
	/// COM interrupt flag (Width: 1, Offset: 5)
	pub fn set_comif(value: u8) { ::write(REGISTER_ADDRESS, COMIF_BIT_OFFSET, COMIF_BIT_WIDTH, value as u32); }

	const TIF_BIT_OFFSET: u8 = 6;
	const TIF_BIT_WIDTH: u8 = 1;
	/// Trigger interrupt flag (Width: 1, Offset: 6)
	pub fn get_tif() -> u8 { ::read(REGISTER_ADDRESS, TIF_BIT_OFFSET, TIF_BIT_WIDTH) as u8 }
	/// Trigger interrupt flag (Width: 1, Offset: 6)
	pub fn set_tif(value: u8) { ::write(REGISTER_ADDRESS, TIF_BIT_OFFSET, TIF_BIT_WIDTH, value as u32); }

	const BIF_BIT_OFFSET: u8 = 7;
	const BIF_BIT_WIDTH: u8 = 1;
	/// Break interrupt flag (Width: 1, Offset: 7)
	pub fn get_bif() -> u8 { ::read(REGISTER_ADDRESS, BIF_BIT_OFFSET, BIF_BIT_WIDTH) as u8 }
	/// Break interrupt flag (Width: 1, Offset: 7)
	pub fn set_bif(value: u8) { ::write(REGISTER_ADDRESS, BIF_BIT_OFFSET, BIF_BIT_WIDTH, value as u32); }

	const B2IF_BIT_OFFSET: u8 = 8;
	const B2IF_BIT_WIDTH: u8 = 1;
	/// Break 2 interrupt flag (Width: 1, Offset: 8)
	pub fn get_b2if() -> u8 { ::read(REGISTER_ADDRESS, B2IF_BIT_OFFSET, B2IF_BIT_WIDTH) as u8 }
	/// Break 2 interrupt flag (Width: 1, Offset: 8)
	pub fn set_b2if(value: u8) { ::write(REGISTER_ADDRESS, B2IF_BIT_OFFSET, B2IF_BIT_WIDTH, value as u32); }

	const CC1OF_BIT_OFFSET: u8 = 9;
	const CC1OF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 overcapture flag (Width: 1, Offset: 9)
	pub fn get_cc1of() -> u8 { ::read(REGISTER_ADDRESS, CC1OF_BIT_OFFSET, CC1OF_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 overcapture flag (Width: 1, Offset: 9)
	pub fn set_cc1of(value: u8) { ::write(REGISTER_ADDRESS, CC1OF_BIT_OFFSET, CC1OF_BIT_WIDTH, value as u32); }

	const CC2OF_BIT_OFFSET: u8 = 10;
	const CC2OF_BIT_WIDTH: u8 = 1;
	/// Capture/compare 2 overcapture flag (Width: 1, Offset: 10)
	pub fn get_cc2of() -> u8 { ::read(REGISTER_ADDRESS, CC2OF_BIT_OFFSET, CC2OF_BIT_WIDTH) as u8 }
	/// Capture/compare 2 overcapture flag (Width: 1, Offset: 10)
	pub fn set_cc2of(value: u8) { ::write(REGISTER_ADDRESS, CC2OF_BIT_OFFSET, CC2OF_BIT_WIDTH, value as u32); }

	const CC3OF_BIT_OFFSET: u8 = 11;
	const CC3OF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 3 overcapture flag (Width: 1, Offset: 11)
	pub fn get_cc3of() -> u8 { ::read(REGISTER_ADDRESS, CC3OF_BIT_OFFSET, CC3OF_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 overcapture flag (Width: 1, Offset: 11)
	pub fn set_cc3of(value: u8) { ::write(REGISTER_ADDRESS, CC3OF_BIT_OFFSET, CC3OF_BIT_WIDTH, value as u32); }

	const CC4OF_BIT_OFFSET: u8 = 12;
	const CC4OF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 4 overcapture flag (Width: 1, Offset: 12)
	pub fn get_cc4of() -> u8 { ::read(REGISTER_ADDRESS, CC4OF_BIT_OFFSET, CC4OF_BIT_WIDTH) as u8 }
	/// Capture/Compare 4 overcapture flag (Width: 1, Offset: 12)
	pub fn set_cc4of(value: u8) { ::write(REGISTER_ADDRESS, CC4OF_BIT_OFFSET, CC4OF_BIT_WIDTH, value as u32); }

	const C5IF_BIT_OFFSET: u8 = 16;
	const C5IF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 5 interrupt flag (Width: 1, Offset: 16)
	pub fn get_c5if() -> u8 { ::read(REGISTER_ADDRESS, C5IF_BIT_OFFSET, C5IF_BIT_WIDTH) as u8 }
	/// Capture/Compare 5 interrupt flag (Width: 1, Offset: 16)
	pub fn set_c5if(value: u8) { ::write(REGISTER_ADDRESS, C5IF_BIT_OFFSET, C5IF_BIT_WIDTH, value as u32); }

	const C6IF_BIT_OFFSET: u8 = 17;
	const C6IF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 6 interrupt flag (Width: 1, Offset: 17)
	pub fn get_c6if() -> u8 { ::read(REGISTER_ADDRESS, C6IF_BIT_OFFSET, C6IF_BIT_WIDTH) as u8 }
	/// Capture/Compare 6 interrupt flag (Width: 1, Offset: 17)
	pub fn set_c6if(value: u8) { ::write(REGISTER_ADDRESS, C6IF_BIT_OFFSET, C6IF_BIT_WIDTH, value as u32); }
}
/// event generation register
/// Size: 0x20 bits
pub mod egr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const UG_BIT_OFFSET: u8 = 0;
	const UG_BIT_WIDTH: u8 = 1;
	/// Update generation (Width: 1, Offset: 0)
	pub fn set_ug(value: u8) { ::write(REGISTER_ADDRESS, UG_BIT_OFFSET, UG_BIT_WIDTH, value as u32); }

	const CC1G_BIT_OFFSET: u8 = 1;
	const CC1G_BIT_WIDTH: u8 = 1;
	/// Capture/compare 1 generation (Width: 1, Offset: 1)
	pub fn set_cc1g(value: u8) { ::write(REGISTER_ADDRESS, CC1G_BIT_OFFSET, CC1G_BIT_WIDTH, value as u32); }

	const CC2G_BIT_OFFSET: u8 = 2;
	const CC2G_BIT_WIDTH: u8 = 1;
	/// Capture/compare 2 generation (Width: 1, Offset: 2)
	pub fn set_cc2g(value: u8) { ::write(REGISTER_ADDRESS, CC2G_BIT_OFFSET, CC2G_BIT_WIDTH, value as u32); }

	const CC3G_BIT_OFFSET: u8 = 3;
	const CC3G_BIT_WIDTH: u8 = 1;
	/// Capture/compare 3 generation (Width: 1, Offset: 3)
	pub fn set_cc3g(value: u8) { ::write(REGISTER_ADDRESS, CC3G_BIT_OFFSET, CC3G_BIT_WIDTH, value as u32); }

	const CC4G_BIT_OFFSET: u8 = 4;
	const CC4G_BIT_WIDTH: u8 = 1;
	/// Capture/compare 4 generation (Width: 1, Offset: 4)
	pub fn set_cc4g(value: u8) { ::write(REGISTER_ADDRESS, CC4G_BIT_OFFSET, CC4G_BIT_WIDTH, value as u32); }

	const COMG_BIT_OFFSET: u8 = 5;
	const COMG_BIT_WIDTH: u8 = 1;
	/// Capture/Compare control update generation (Width: 1, Offset: 5)
	pub fn set_comg(value: u8) { ::write(REGISTER_ADDRESS, COMG_BIT_OFFSET, COMG_BIT_WIDTH, value as u32); }

	const TG_BIT_OFFSET: u8 = 6;
	const TG_BIT_WIDTH: u8 = 1;
	/// Trigger generation (Width: 1, Offset: 6)
	pub fn set_tg(value: u8) { ::write(REGISTER_ADDRESS, TG_BIT_OFFSET, TG_BIT_WIDTH, value as u32); }

	const BG_BIT_OFFSET: u8 = 7;
	const BG_BIT_WIDTH: u8 = 1;
	/// Break generation (Width: 1, Offset: 7)
	pub fn set_bg(value: u8) { ::write(REGISTER_ADDRESS, BG_BIT_OFFSET, BG_BIT_WIDTH, value as u32); }

	const B2G_BIT_OFFSET: u8 = 8;
	const B2G_BIT_WIDTH: u8 = 1;
	/// Break 2 generation (Width: 1, Offset: 8)
	pub fn set_b2g(value: u8) { ::write(REGISTER_ADDRESS, B2G_BIT_OFFSET, B2G_BIT_WIDTH, value as u32); }
}
/// capture/compare mode register (output mode)
/// Size: 0x20 bits
pub mod ccmr1_output {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OC2CE_BIT_OFFSET: u8 = 15;
	const OC2CE_BIT_WIDTH: u8 = 1;
	/// Output Compare 2 clear enable (Width: 1, Offset: 15)
	pub fn get_oc2ce() -> u8 { ::read(REGISTER_ADDRESS, OC2CE_BIT_OFFSET, OC2CE_BIT_WIDTH) as u8 }
	/// Output Compare 2 clear enable (Width: 1, Offset: 15)
	pub fn set_oc2ce(value: u8) { ::write(REGISTER_ADDRESS, OC2CE_BIT_OFFSET, OC2CE_BIT_WIDTH, value as u32); }

	const OC2M_BIT_OFFSET: u8 = 12;
	const OC2M_BIT_WIDTH: u8 = 3;
	/// Output Compare 2 mode (Width: 3, Offset: 12)
	pub fn get_oc2m() -> u8 { ::read(REGISTER_ADDRESS, OC2M_BIT_OFFSET, OC2M_BIT_WIDTH) as u8 }
	/// Output Compare 2 mode (Width: 3, Offset: 12)
	pub fn set_oc2m(value: u8) { ::write(REGISTER_ADDRESS, OC2M_BIT_OFFSET, OC2M_BIT_WIDTH, value as u32); }

	const OC2PE_BIT_OFFSET: u8 = 11;
	const OC2PE_BIT_WIDTH: u8 = 1;
	/// Output Compare 2 preload enable (Width: 1, Offset: 11)
	pub fn get_oc2pe() -> u8 { ::read(REGISTER_ADDRESS, OC2PE_BIT_OFFSET, OC2PE_BIT_WIDTH) as u8 }
	/// Output Compare 2 preload enable (Width: 1, Offset: 11)
	pub fn set_oc2pe(value: u8) { ::write(REGISTER_ADDRESS, OC2PE_BIT_OFFSET, OC2PE_BIT_WIDTH, value as u32); }

	const OC2FE_BIT_OFFSET: u8 = 10;
	const OC2FE_BIT_WIDTH: u8 = 1;
	/// Output Compare 2 fast enable (Width: 1, Offset: 10)
	pub fn get_oc2fe() -> u8 { ::read(REGISTER_ADDRESS, OC2FE_BIT_OFFSET, OC2FE_BIT_WIDTH) as u8 }
	/// Output Compare 2 fast enable (Width: 1, Offset: 10)
	pub fn set_oc2fe(value: u8) { ::write(REGISTER_ADDRESS, OC2FE_BIT_OFFSET, OC2FE_BIT_WIDTH, value as u32); }

	const CC2S_BIT_OFFSET: u8 = 8;
	const CC2S_BIT_WIDTH: u8 = 2;
	/// Capture/Compare 2 selection (Width: 2, Offset: 8)
	pub fn get_cc2s() -> u8 { ::read(REGISTER_ADDRESS, CC2S_BIT_OFFSET, CC2S_BIT_WIDTH) as u8 }
	/// Capture/Compare 2 selection (Width: 2, Offset: 8)
	pub fn set_cc2s(value: u8) { ::write(REGISTER_ADDRESS, CC2S_BIT_OFFSET, CC2S_BIT_WIDTH, value as u32); }

	const OC1CE_BIT_OFFSET: u8 = 7;
	const OC1CE_BIT_WIDTH: u8 = 1;
	/// Output Compare 1 clear enable (Width: 1, Offset: 7)
	pub fn get_oc1ce() -> u8 { ::read(REGISTER_ADDRESS, OC1CE_BIT_OFFSET, OC1CE_BIT_WIDTH) as u8 }
	/// Output Compare 1 clear enable (Width: 1, Offset: 7)
	pub fn set_oc1ce(value: u8) { ::write(REGISTER_ADDRESS, OC1CE_BIT_OFFSET, OC1CE_BIT_WIDTH, value as u32); }

	const OC1M_BIT_OFFSET: u8 = 4;
	const OC1M_BIT_WIDTH: u8 = 3;
	/// Output Compare 1 mode (Width: 3, Offset: 4)
	pub fn get_oc1m() -> u8 { ::read(REGISTER_ADDRESS, OC1M_BIT_OFFSET, OC1M_BIT_WIDTH) as u8 }
	/// Output Compare 1 mode (Width: 3, Offset: 4)
	pub fn set_oc1m(value: u8) { ::write(REGISTER_ADDRESS, OC1M_BIT_OFFSET, OC1M_BIT_WIDTH, value as u32); }

	const OC1PE_BIT_OFFSET: u8 = 3;
	const OC1PE_BIT_WIDTH: u8 = 1;
	/// Output Compare 1 preload enable (Width: 1, Offset: 3)
	pub fn get_oc1pe() -> u8 { ::read(REGISTER_ADDRESS, OC1PE_BIT_OFFSET, OC1PE_BIT_WIDTH) as u8 }
	/// Output Compare 1 preload enable (Width: 1, Offset: 3)
	pub fn set_oc1pe(value: u8) { ::write(REGISTER_ADDRESS, OC1PE_BIT_OFFSET, OC1PE_BIT_WIDTH, value as u32); }

	const OC1FE_BIT_OFFSET: u8 = 2;
	const OC1FE_BIT_WIDTH: u8 = 1;
	/// Output Compare 1 fast enable (Width: 1, Offset: 2)
	pub fn get_oc1fe() -> u8 { ::read(REGISTER_ADDRESS, OC1FE_BIT_OFFSET, OC1FE_BIT_WIDTH) as u8 }
	/// Output Compare 1 fast enable (Width: 1, Offset: 2)
	pub fn set_oc1fe(value: u8) { ::write(REGISTER_ADDRESS, OC1FE_BIT_OFFSET, OC1FE_BIT_WIDTH, value as u32); }

	const CC1S_BIT_OFFSET: u8 = 0;
	const CC1S_BIT_WIDTH: u8 = 2;
	/// Capture/Compare 1 selection (Width: 2, Offset: 0)
	pub fn get_cc1s() -> u8 { ::read(REGISTER_ADDRESS, CC1S_BIT_OFFSET, CC1S_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 selection (Width: 2, Offset: 0)
	pub fn set_cc1s(value: u8) { ::write(REGISTER_ADDRESS, CC1S_BIT_OFFSET, CC1S_BIT_WIDTH, value as u32); }

	const OC1M_3_BIT_OFFSET: u8 = 16;
	const OC1M_3_BIT_WIDTH: u8 = 1;
	/// Output Compare 1 mode bit 3 (Width: 1, Offset: 16)
	pub fn get_oc1m_3() -> u8 { ::read(REGISTER_ADDRESS, OC1M_3_BIT_OFFSET, OC1M_3_BIT_WIDTH) as u8 }
	/// Output Compare 1 mode bit 3 (Width: 1, Offset: 16)
	pub fn set_oc1m_3(value: u8) { ::write(REGISTER_ADDRESS, OC1M_3_BIT_OFFSET, OC1M_3_BIT_WIDTH, value as u32); }

	const OC2M_3_BIT_OFFSET: u8 = 24;
	const OC2M_3_BIT_WIDTH: u8 = 1;
	/// Output Compare 2 mode bit 3 (Width: 1, Offset: 24)
	pub fn get_oc2m_3() -> u8 { ::read(REGISTER_ADDRESS, OC2M_3_BIT_OFFSET, OC2M_3_BIT_WIDTH) as u8 }
	/// Output Compare 2 mode bit 3 (Width: 1, Offset: 24)
	pub fn set_oc2m_3(value: u8) { ::write(REGISTER_ADDRESS, OC2M_3_BIT_OFFSET, OC2M_3_BIT_WIDTH, value as u32); }
}
/// capture/compare mode register 1 (input mode)
/// Size: 0x20 bits
pub mod ccmr1_input {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IC2F_BIT_OFFSET: u8 = 12;
	const IC2F_BIT_WIDTH: u8 = 4;
	/// Input capture 2 filter (Width: 4, Offset: 12)
	pub fn get_ic2f() -> u8 { ::read(REGISTER_ADDRESS, IC2F_BIT_OFFSET, IC2F_BIT_WIDTH) as u8 }
	/// Input capture 2 filter (Width: 4, Offset: 12)
	pub fn set_ic2f(value: u8) { ::write(REGISTER_ADDRESS, IC2F_BIT_OFFSET, IC2F_BIT_WIDTH, value as u32); }

	const IC2PCS_BIT_OFFSET: u8 = 10;
	const IC2PCS_BIT_WIDTH: u8 = 2;
	/// Input capture 2 prescaler (Width: 2, Offset: 10)
	pub fn get_ic2pcs() -> u8 { ::read(REGISTER_ADDRESS, IC2PCS_BIT_OFFSET, IC2PCS_BIT_WIDTH) as u8 }
	/// Input capture 2 prescaler (Width: 2, Offset: 10)
	pub fn set_ic2pcs(value: u8) { ::write(REGISTER_ADDRESS, IC2PCS_BIT_OFFSET, IC2PCS_BIT_WIDTH, value as u32); }

	const CC2S_BIT_OFFSET: u8 = 8;
	const CC2S_BIT_WIDTH: u8 = 2;
	/// Capture/Compare 2 selection (Width: 2, Offset: 8)
	pub fn get_cc2s() -> u8 { ::read(REGISTER_ADDRESS, CC2S_BIT_OFFSET, CC2S_BIT_WIDTH) as u8 }
	/// Capture/Compare 2 selection (Width: 2, Offset: 8)
	pub fn set_cc2s(value: u8) { ::write(REGISTER_ADDRESS, CC2S_BIT_OFFSET, CC2S_BIT_WIDTH, value as u32); }

	const IC1F_BIT_OFFSET: u8 = 4;
	const IC1F_BIT_WIDTH: u8 = 4;
	/// Input capture 1 filter (Width: 4, Offset: 4)
	pub fn get_ic1f() -> u8 { ::read(REGISTER_ADDRESS, IC1F_BIT_OFFSET, IC1F_BIT_WIDTH) as u8 }
	/// Input capture 1 filter (Width: 4, Offset: 4)
	pub fn set_ic1f(value: u8) { ::write(REGISTER_ADDRESS, IC1F_BIT_OFFSET, IC1F_BIT_WIDTH, value as u32); }

	const IC1PCS_BIT_OFFSET: u8 = 2;
	const IC1PCS_BIT_WIDTH: u8 = 2;
	/// Input capture 1 prescaler (Width: 2, Offset: 2)
	pub fn get_ic1pcs() -> u8 { ::read(REGISTER_ADDRESS, IC1PCS_BIT_OFFSET, IC1PCS_BIT_WIDTH) as u8 }
	/// Input capture 1 prescaler (Width: 2, Offset: 2)
	pub fn set_ic1pcs(value: u8) { ::write(REGISTER_ADDRESS, IC1PCS_BIT_OFFSET, IC1PCS_BIT_WIDTH, value as u32); }

	const CC1S_BIT_OFFSET: u8 = 0;
	const CC1S_BIT_WIDTH: u8 = 2;
	/// Capture/Compare 1 selection (Width: 2, Offset: 0)
	pub fn get_cc1s() -> u8 { ::read(REGISTER_ADDRESS, CC1S_BIT_OFFSET, CC1S_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 selection (Width: 2, Offset: 0)
	pub fn set_cc1s(value: u8) { ::write(REGISTER_ADDRESS, CC1S_BIT_OFFSET, CC1S_BIT_WIDTH, value as u32); }
}
/// capture/compare mode register (output mode)
/// Size: 0x20 bits
pub mod ccmr2_output {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OC4CE_BIT_OFFSET: u8 = 15;
	const OC4CE_BIT_WIDTH: u8 = 1;
	/// Output compare 4 clear enable (Width: 1, Offset: 15)
	pub fn get_oc4ce() -> u8 { ::read(REGISTER_ADDRESS, OC4CE_BIT_OFFSET, OC4CE_BIT_WIDTH) as u8 }
	/// Output compare 4 clear enable (Width: 1, Offset: 15)
	pub fn set_oc4ce(value: u8) { ::write(REGISTER_ADDRESS, OC4CE_BIT_OFFSET, OC4CE_BIT_WIDTH, value as u32); }

	const OC4M_BIT_OFFSET: u8 = 12;
	const OC4M_BIT_WIDTH: u8 = 3;
	/// Output compare 4 mode (Width: 3, Offset: 12)
	pub fn get_oc4m() -> u8 { ::read(REGISTER_ADDRESS, OC4M_BIT_OFFSET, OC4M_BIT_WIDTH) as u8 }
	/// Output compare 4 mode (Width: 3, Offset: 12)
	pub fn set_oc4m(value: u8) { ::write(REGISTER_ADDRESS, OC4M_BIT_OFFSET, OC4M_BIT_WIDTH, value as u32); }

	const OC4PE_BIT_OFFSET: u8 = 11;
	const OC4PE_BIT_WIDTH: u8 = 1;
	/// Output compare 4 preload enable (Width: 1, Offset: 11)
	pub fn get_oc4pe() -> u8 { ::read(REGISTER_ADDRESS, OC4PE_BIT_OFFSET, OC4PE_BIT_WIDTH) as u8 }
	/// Output compare 4 preload enable (Width: 1, Offset: 11)
	pub fn set_oc4pe(value: u8) { ::write(REGISTER_ADDRESS, OC4PE_BIT_OFFSET, OC4PE_BIT_WIDTH, value as u32); }

	const OC4FE_BIT_OFFSET: u8 = 10;
	const OC4FE_BIT_WIDTH: u8 = 1;
	/// Output compare 4 fast enable (Width: 1, Offset: 10)
	pub fn get_oc4fe() -> u8 { ::read(REGISTER_ADDRESS, OC4FE_BIT_OFFSET, OC4FE_BIT_WIDTH) as u8 }
	/// Output compare 4 fast enable (Width: 1, Offset: 10)
	pub fn set_oc4fe(value: u8) { ::write(REGISTER_ADDRESS, OC4FE_BIT_OFFSET, OC4FE_BIT_WIDTH, value as u32); }

	const CC4S_BIT_OFFSET: u8 = 8;
	const CC4S_BIT_WIDTH: u8 = 2;
	/// Capture/Compare 4 selection (Width: 2, Offset: 8)
	pub fn get_cc4s() -> u8 { ::read(REGISTER_ADDRESS, CC4S_BIT_OFFSET, CC4S_BIT_WIDTH) as u8 }
	/// Capture/Compare 4 selection (Width: 2, Offset: 8)
	pub fn set_cc4s(value: u8) { ::write(REGISTER_ADDRESS, CC4S_BIT_OFFSET, CC4S_BIT_WIDTH, value as u32); }

	const OC3CE_BIT_OFFSET: u8 = 7;
	const OC3CE_BIT_WIDTH: u8 = 1;
	/// Output compare 3 clear enable (Width: 1, Offset: 7)
	pub fn get_oc3ce() -> u8 { ::read(REGISTER_ADDRESS, OC3CE_BIT_OFFSET, OC3CE_BIT_WIDTH) as u8 }
	/// Output compare 3 clear enable (Width: 1, Offset: 7)
	pub fn set_oc3ce(value: u8) { ::write(REGISTER_ADDRESS, OC3CE_BIT_OFFSET, OC3CE_BIT_WIDTH, value as u32); }

	const OC3M_BIT_OFFSET: u8 = 4;
	const OC3M_BIT_WIDTH: u8 = 3;
	/// Output compare 3 mode (Width: 3, Offset: 4)
	pub fn get_oc3m() -> u8 { ::read(REGISTER_ADDRESS, OC3M_BIT_OFFSET, OC3M_BIT_WIDTH) as u8 }
	/// Output compare 3 mode (Width: 3, Offset: 4)
	pub fn set_oc3m(value: u8) { ::write(REGISTER_ADDRESS, OC3M_BIT_OFFSET, OC3M_BIT_WIDTH, value as u32); }

	const OC3PE_BIT_OFFSET: u8 = 3;
	const OC3PE_BIT_WIDTH: u8 = 1;
	/// Output compare 3 preload enable (Width: 1, Offset: 3)
	pub fn get_oc3pe() -> u8 { ::read(REGISTER_ADDRESS, OC3PE_BIT_OFFSET, OC3PE_BIT_WIDTH) as u8 }
	/// Output compare 3 preload enable (Width: 1, Offset: 3)
	pub fn set_oc3pe(value: u8) { ::write(REGISTER_ADDRESS, OC3PE_BIT_OFFSET, OC3PE_BIT_WIDTH, value as u32); }

	const OC3FE_BIT_OFFSET: u8 = 2;
	const OC3FE_BIT_WIDTH: u8 = 1;
	/// Output compare 3 fast enable (Width: 1, Offset: 2)
	pub fn get_oc3fe() -> u8 { ::read(REGISTER_ADDRESS, OC3FE_BIT_OFFSET, OC3FE_BIT_WIDTH) as u8 }
	/// Output compare 3 fast enable (Width: 1, Offset: 2)
	pub fn set_oc3fe(value: u8) { ::write(REGISTER_ADDRESS, OC3FE_BIT_OFFSET, OC3FE_BIT_WIDTH, value as u32); }

	const CC3S_BIT_OFFSET: u8 = 0;
	const CC3S_BIT_WIDTH: u8 = 2;
	/// Capture/Compare 3 selection (Width: 2, Offset: 0)
	pub fn get_cc3s() -> u8 { ::read(REGISTER_ADDRESS, CC3S_BIT_OFFSET, CC3S_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 selection (Width: 2, Offset: 0)
	pub fn set_cc3s(value: u8) { ::write(REGISTER_ADDRESS, CC3S_BIT_OFFSET, CC3S_BIT_WIDTH, value as u32); }

	const OC3M_3_BIT_OFFSET: u8 = 16;
	const OC3M_3_BIT_WIDTH: u8 = 1;
	/// Output Compare 3 mode bit 3 (Width: 1, Offset: 16)
	pub fn get_oc3m_3() -> u8 { ::read(REGISTER_ADDRESS, OC3M_3_BIT_OFFSET, OC3M_3_BIT_WIDTH) as u8 }
	/// Output Compare 3 mode bit 3 (Width: 1, Offset: 16)
	pub fn set_oc3m_3(value: u8) { ::write(REGISTER_ADDRESS, OC3M_3_BIT_OFFSET, OC3M_3_BIT_WIDTH, value as u32); }

	const OC4M_3_BIT_OFFSET: u8 = 24;
	const OC4M_3_BIT_WIDTH: u8 = 1;
	/// Output Compare 4 mode bit 3 (Width: 1, Offset: 24)
	pub fn get_oc4m_3() -> u8 { ::read(REGISTER_ADDRESS, OC4M_3_BIT_OFFSET, OC4M_3_BIT_WIDTH) as u8 }
	/// Output Compare 4 mode bit 3 (Width: 1, Offset: 24)
	pub fn set_oc4m_3(value: u8) { ::write(REGISTER_ADDRESS, OC4M_3_BIT_OFFSET, OC4M_3_BIT_WIDTH, value as u32); }
}
/// capture/compare mode register 2 (input mode)
/// Size: 0x20 bits
pub mod ccmr2_input {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IC4F_BIT_OFFSET: u8 = 12;
	const IC4F_BIT_WIDTH: u8 = 4;
	/// Input capture 4 filter (Width: 4, Offset: 12)
	pub fn get_ic4f() -> u8 { ::read(REGISTER_ADDRESS, IC4F_BIT_OFFSET, IC4F_BIT_WIDTH) as u8 }
	/// Input capture 4 filter (Width: 4, Offset: 12)
	pub fn set_ic4f(value: u8) { ::write(REGISTER_ADDRESS, IC4F_BIT_OFFSET, IC4F_BIT_WIDTH, value as u32); }

	const IC4PSC_BIT_OFFSET: u8 = 10;
	const IC4PSC_BIT_WIDTH: u8 = 2;
	/// Input capture 4 prescaler (Width: 2, Offset: 10)
	pub fn get_ic4psc() -> u8 { ::read(REGISTER_ADDRESS, IC4PSC_BIT_OFFSET, IC4PSC_BIT_WIDTH) as u8 }
	/// Input capture 4 prescaler (Width: 2, Offset: 10)
	pub fn set_ic4psc(value: u8) { ::write(REGISTER_ADDRESS, IC4PSC_BIT_OFFSET, IC4PSC_BIT_WIDTH, value as u32); }

	const CC4S_BIT_OFFSET: u8 = 8;
	const CC4S_BIT_WIDTH: u8 = 2;
	/// Capture/Compare 4 selection (Width: 2, Offset: 8)
	pub fn get_cc4s() -> u8 { ::read(REGISTER_ADDRESS, CC4S_BIT_OFFSET, CC4S_BIT_WIDTH) as u8 }
	/// Capture/Compare 4 selection (Width: 2, Offset: 8)
	pub fn set_cc4s(value: u8) { ::write(REGISTER_ADDRESS, CC4S_BIT_OFFSET, CC4S_BIT_WIDTH, value as u32); }

	const IC3F_BIT_OFFSET: u8 = 4;
	const IC3F_BIT_WIDTH: u8 = 4;
	/// Input capture 3 filter (Width: 4, Offset: 4)
	pub fn get_ic3f() -> u8 { ::read(REGISTER_ADDRESS, IC3F_BIT_OFFSET, IC3F_BIT_WIDTH) as u8 }
	/// Input capture 3 filter (Width: 4, Offset: 4)
	pub fn set_ic3f(value: u8) { ::write(REGISTER_ADDRESS, IC3F_BIT_OFFSET, IC3F_BIT_WIDTH, value as u32); }

	const IC3PSC_BIT_OFFSET: u8 = 2;
	const IC3PSC_BIT_WIDTH: u8 = 2;
	/// Input capture 3 prescaler (Width: 2, Offset: 2)
	pub fn get_ic3psc() -> u8 { ::read(REGISTER_ADDRESS, IC3PSC_BIT_OFFSET, IC3PSC_BIT_WIDTH) as u8 }
	/// Input capture 3 prescaler (Width: 2, Offset: 2)
	pub fn set_ic3psc(value: u8) { ::write(REGISTER_ADDRESS, IC3PSC_BIT_OFFSET, IC3PSC_BIT_WIDTH, value as u32); }

	const CC3S_BIT_OFFSET: u8 = 0;
	const CC3S_BIT_WIDTH: u8 = 2;
	/// Capture/compare 3 selection (Width: 2, Offset: 0)
	pub fn get_cc3s() -> u8 { ::read(REGISTER_ADDRESS, CC3S_BIT_OFFSET, CC3S_BIT_WIDTH) as u8 }
	/// Capture/compare 3 selection (Width: 2, Offset: 0)
	pub fn set_cc3s(value: u8) { ::write(REGISTER_ADDRESS, CC3S_BIT_OFFSET, CC3S_BIT_WIDTH, value as u32); }
}
/// capture/compare enable register
/// Size: 0x20 bits
pub mod ccer {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x20;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CC1E_BIT_OFFSET: u8 = 0;
	const CC1E_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 output enable (Width: 1, Offset: 0)
	pub fn get_cc1e() -> u8 { ::read(REGISTER_ADDRESS, CC1E_BIT_OFFSET, CC1E_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 output enable (Width: 1, Offset: 0)
	pub fn set_cc1e(value: u8) { ::write(REGISTER_ADDRESS, CC1E_BIT_OFFSET, CC1E_BIT_WIDTH, value as u32); }

	const CC1P_BIT_OFFSET: u8 = 1;
	const CC1P_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 output Polarity (Width: 1, Offset: 1)
	pub fn get_cc1p() -> u8 { ::read(REGISTER_ADDRESS, CC1P_BIT_OFFSET, CC1P_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 output Polarity (Width: 1, Offset: 1)
	pub fn set_cc1p(value: u8) { ::write(REGISTER_ADDRESS, CC1P_BIT_OFFSET, CC1P_BIT_WIDTH, value as u32); }

	const CC1NE_BIT_OFFSET: u8 = 2;
	const CC1NE_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 complementary output enable (Width: 1, Offset: 2)
	pub fn get_cc1ne() -> u8 { ::read(REGISTER_ADDRESS, CC1NE_BIT_OFFSET, CC1NE_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 complementary output enable (Width: 1, Offset: 2)
	pub fn set_cc1ne(value: u8) { ::write(REGISTER_ADDRESS, CC1NE_BIT_OFFSET, CC1NE_BIT_WIDTH, value as u32); }

	const CC1NP_BIT_OFFSET: u8 = 3;
	const CC1NP_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 output Polarity (Width: 1, Offset: 3)
	pub fn get_cc1np() -> u8 { ::read(REGISTER_ADDRESS, CC1NP_BIT_OFFSET, CC1NP_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 output Polarity (Width: 1, Offset: 3)
	pub fn set_cc1np(value: u8) { ::write(REGISTER_ADDRESS, CC1NP_BIT_OFFSET, CC1NP_BIT_WIDTH, value as u32); }

	const CC2E_BIT_OFFSET: u8 = 4;
	const CC2E_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 2 output enable (Width: 1, Offset: 4)
	pub fn get_cc2e() -> u8 { ::read(REGISTER_ADDRESS, CC2E_BIT_OFFSET, CC2E_BIT_WIDTH) as u8 }
	/// Capture/Compare 2 output enable (Width: 1, Offset: 4)
	pub fn set_cc2e(value: u8) { ::write(REGISTER_ADDRESS, CC2E_BIT_OFFSET, CC2E_BIT_WIDTH, value as u32); }

	const CC2P_BIT_OFFSET: u8 = 5;
	const CC2P_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 2 output Polarity (Width: 1, Offset: 5)
	pub fn get_cc2p() -> u8 { ::read(REGISTER_ADDRESS, CC2P_BIT_OFFSET, CC2P_BIT_WIDTH) as u8 }
	/// Capture/Compare 2 output Polarity (Width: 1, Offset: 5)
	pub fn set_cc2p(value: u8) { ::write(REGISTER_ADDRESS, CC2P_BIT_OFFSET, CC2P_BIT_WIDTH, value as u32); }

	const CC2NE_BIT_OFFSET: u8 = 6;
	const CC2NE_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 2 complementary output enable (Width: 1, Offset: 6)
	pub fn get_cc2ne() -> u8 { ::read(REGISTER_ADDRESS, CC2NE_BIT_OFFSET, CC2NE_BIT_WIDTH) as u8 }
	/// Capture/Compare 2 complementary output enable (Width: 1, Offset: 6)
	pub fn set_cc2ne(value: u8) { ::write(REGISTER_ADDRESS, CC2NE_BIT_OFFSET, CC2NE_BIT_WIDTH, value as u32); }

	const CC2NP_BIT_OFFSET: u8 = 7;
	const CC2NP_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 2 output Polarity (Width: 1, Offset: 7)
	pub fn get_cc2np() -> u8 { ::read(REGISTER_ADDRESS, CC2NP_BIT_OFFSET, CC2NP_BIT_WIDTH) as u8 }
	/// Capture/Compare 2 output Polarity (Width: 1, Offset: 7)
	pub fn set_cc2np(value: u8) { ::write(REGISTER_ADDRESS, CC2NP_BIT_OFFSET, CC2NP_BIT_WIDTH, value as u32); }

	const CC3E_BIT_OFFSET: u8 = 8;
	const CC3E_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 3 output enable (Width: 1, Offset: 8)
	pub fn get_cc3e() -> u8 { ::read(REGISTER_ADDRESS, CC3E_BIT_OFFSET, CC3E_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 output enable (Width: 1, Offset: 8)
	pub fn set_cc3e(value: u8) { ::write(REGISTER_ADDRESS, CC3E_BIT_OFFSET, CC3E_BIT_WIDTH, value as u32); }

	const CC3P_BIT_OFFSET: u8 = 9;
	const CC3P_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 3 output Polarity (Width: 1, Offset: 9)
	pub fn get_cc3p() -> u8 { ::read(REGISTER_ADDRESS, CC3P_BIT_OFFSET, CC3P_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 output Polarity (Width: 1, Offset: 9)
	pub fn set_cc3p(value: u8) { ::write(REGISTER_ADDRESS, CC3P_BIT_OFFSET, CC3P_BIT_WIDTH, value as u32); }

	const CC3NE_BIT_OFFSET: u8 = 10;
	const CC3NE_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 3 complementary output enable (Width: 1, Offset: 10)
	pub fn get_cc3ne() -> u8 { ::read(REGISTER_ADDRESS, CC3NE_BIT_OFFSET, CC3NE_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 complementary output enable (Width: 1, Offset: 10)
	pub fn set_cc3ne(value: u8) { ::write(REGISTER_ADDRESS, CC3NE_BIT_OFFSET, CC3NE_BIT_WIDTH, value as u32); }

	const CC3NP_BIT_OFFSET: u8 = 11;
	const CC3NP_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 3 output Polarity (Width: 1, Offset: 11)
	pub fn get_cc3np() -> u8 { ::read(REGISTER_ADDRESS, CC3NP_BIT_OFFSET, CC3NP_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 output Polarity (Width: 1, Offset: 11)
	pub fn set_cc3np(value: u8) { ::write(REGISTER_ADDRESS, CC3NP_BIT_OFFSET, CC3NP_BIT_WIDTH, value as u32); }

	const CC4E_BIT_OFFSET: u8 = 12;
	const CC4E_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 4 output enable (Width: 1, Offset: 12)
	pub fn get_cc4e() -> u8 { ::read(REGISTER_ADDRESS, CC4E_BIT_OFFSET, CC4E_BIT_WIDTH) as u8 }
	/// Capture/Compare 4 output enable (Width: 1, Offset: 12)
	pub fn set_cc4e(value: u8) { ::write(REGISTER_ADDRESS, CC4E_BIT_OFFSET, CC4E_BIT_WIDTH, value as u32); }

	const CC4P_BIT_OFFSET: u8 = 13;
	const CC4P_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 3 output Polarity (Width: 1, Offset: 13)
	pub fn get_cc4p() -> u8 { ::read(REGISTER_ADDRESS, CC4P_BIT_OFFSET, CC4P_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 output Polarity (Width: 1, Offset: 13)
	pub fn set_cc4p(value: u8) { ::write(REGISTER_ADDRESS, CC4P_BIT_OFFSET, CC4P_BIT_WIDTH, value as u32); }

	const CC4NP_BIT_OFFSET: u8 = 15;
	const CC4NP_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 4 output Polarity (Width: 1, Offset: 15)
	pub fn get_cc4np() -> u8 { ::read(REGISTER_ADDRESS, CC4NP_BIT_OFFSET, CC4NP_BIT_WIDTH) as u8 }
	/// Capture/Compare 4 output Polarity (Width: 1, Offset: 15)
	pub fn set_cc4np(value: u8) { ::write(REGISTER_ADDRESS, CC4NP_BIT_OFFSET, CC4NP_BIT_WIDTH, value as u32); }

	const CC5E_BIT_OFFSET: u8 = 16;
	const CC5E_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 5 output enable (Width: 1, Offset: 16)
	pub fn get_cc5e() -> u8 { ::read(REGISTER_ADDRESS, CC5E_BIT_OFFSET, CC5E_BIT_WIDTH) as u8 }
	/// Capture/Compare 5 output enable (Width: 1, Offset: 16)
	pub fn set_cc5e(value: u8) { ::write(REGISTER_ADDRESS, CC5E_BIT_OFFSET, CC5E_BIT_WIDTH, value as u32); }

	const CC5P_BIT_OFFSET: u8 = 17;
	const CC5P_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 5 output Polarity (Width: 1, Offset: 17)
	pub fn get_cc5p() -> u8 { ::read(REGISTER_ADDRESS, CC5P_BIT_OFFSET, CC5P_BIT_WIDTH) as u8 }
	/// Capture/Compare 5 output Polarity (Width: 1, Offset: 17)
	pub fn set_cc5p(value: u8) { ::write(REGISTER_ADDRESS, CC5P_BIT_OFFSET, CC5P_BIT_WIDTH, value as u32); }

	const CC6E_BIT_OFFSET: u8 = 20;
	const CC6E_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 6 output enable (Width: 1, Offset: 20)
	pub fn get_cc6e() -> u8 { ::read(REGISTER_ADDRESS, CC6E_BIT_OFFSET, CC6E_BIT_WIDTH) as u8 }
	/// Capture/Compare 6 output enable (Width: 1, Offset: 20)
	pub fn set_cc6e(value: u8) { ::write(REGISTER_ADDRESS, CC6E_BIT_OFFSET, CC6E_BIT_WIDTH, value as u32); }

	const CC6P_BIT_OFFSET: u8 = 21;
	const CC6P_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 6 output Polarity (Width: 1, Offset: 21)
	pub fn get_cc6p() -> u8 { ::read(REGISTER_ADDRESS, CC6P_BIT_OFFSET, CC6P_BIT_WIDTH) as u8 }
	/// Capture/Compare 6 output Polarity (Width: 1, Offset: 21)
	pub fn set_cc6p(value: u8) { ::write(REGISTER_ADDRESS, CC6P_BIT_OFFSET, CC6P_BIT_WIDTH, value as u32); }
}
/// counter
/// Size: 0x20 bits
pub mod cnt {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x24;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CNT_BIT_OFFSET: u8 = 0;
	const CNT_BIT_WIDTH: u8 = 16;
	/// counter value (Width: 16, Offset: 0)
	pub fn get_cnt() -> u16 { ::read(REGISTER_ADDRESS, CNT_BIT_OFFSET, CNT_BIT_WIDTH) as u16 }
	/// counter value (Width: 16, Offset: 0)
	pub fn set_cnt(value: u16) { ::write(REGISTER_ADDRESS, CNT_BIT_OFFSET, CNT_BIT_WIDTH, value as u32); }

	const UIFCPY_BIT_OFFSET: u8 = 31;
	const UIFCPY_BIT_WIDTH: u8 = 1;
	/// UIF copy (Width: 1, Offset: 31)
	pub fn get_uifcpy() -> u8 { ::read(REGISTER_ADDRESS, UIFCPY_BIT_OFFSET, UIFCPY_BIT_WIDTH) as u8 }
}
/// prescaler
/// Size: 0x20 bits
pub mod psc {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x28;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PSC_BIT_OFFSET: u8 = 0;
	const PSC_BIT_WIDTH: u8 = 16;
	/// Prescaler value (Width: 16, Offset: 0)
	pub fn get_psc() -> u16 { ::read(REGISTER_ADDRESS, PSC_BIT_OFFSET, PSC_BIT_WIDTH) as u16 }
	/// Prescaler value (Width: 16, Offset: 0)
	pub fn set_psc(value: u16) { ::write(REGISTER_ADDRESS, PSC_BIT_OFFSET, PSC_BIT_WIDTH, value as u32); }
}
/// auto-reload register
/// Size: 0x20 bits
pub mod arr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x2C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ARR_BIT_OFFSET: u8 = 0;
	const ARR_BIT_WIDTH: u8 = 16;
	/// Auto-reload value (Width: 16, Offset: 0)
	pub fn get_arr() -> u16 { ::read(REGISTER_ADDRESS, ARR_BIT_OFFSET, ARR_BIT_WIDTH) as u16 }
	/// Auto-reload value (Width: 16, Offset: 0)
	pub fn set_arr(value: u16) { ::write(REGISTER_ADDRESS, ARR_BIT_OFFSET, ARR_BIT_WIDTH, value as u32); }
}
/// repetition counter register
/// Size: 0x20 bits
pub mod rcr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x30;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const REP_BIT_OFFSET: u8 = 0;
	const REP_BIT_WIDTH: u8 = 16;
	/// Repetition counter value (Width: 16, Offset: 0)
	pub fn get_rep() -> u16 { ::read(REGISTER_ADDRESS, REP_BIT_OFFSET, REP_BIT_WIDTH) as u16 }
	/// Repetition counter value (Width: 16, Offset: 0)
	pub fn set_rep(value: u16) { ::write(REGISTER_ADDRESS, REP_BIT_OFFSET, REP_BIT_WIDTH, value as u32); }
}
/// capture/compare register 1
/// Size: 0x20 bits
pub mod ccr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x34;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CCR1_BIT_OFFSET: u8 = 0;
	const CCR1_BIT_WIDTH: u8 = 16;
	/// Capture/Compare 1 value (Width: 16, Offset: 0)
	pub fn get_ccr1() -> u16 { ::read(REGISTER_ADDRESS, CCR1_BIT_OFFSET, CCR1_BIT_WIDTH) as u16 }
	/// Capture/Compare 1 value (Width: 16, Offset: 0)
	pub fn set_ccr1(value: u16) { ::write(REGISTER_ADDRESS, CCR1_BIT_OFFSET, CCR1_BIT_WIDTH, value as u32); }
}
/// capture/compare register 2
/// Size: 0x20 bits
pub mod ccr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x38;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CCR2_BIT_OFFSET: u8 = 0;
	const CCR2_BIT_WIDTH: u8 = 16;
	/// Capture/Compare 2 value (Width: 16, Offset: 0)
	pub fn get_ccr2() -> u16 { ::read(REGISTER_ADDRESS, CCR2_BIT_OFFSET, CCR2_BIT_WIDTH) as u16 }
	/// Capture/Compare 2 value (Width: 16, Offset: 0)
	pub fn set_ccr2(value: u16) { ::write(REGISTER_ADDRESS, CCR2_BIT_OFFSET, CCR2_BIT_WIDTH, value as u32); }
}
/// capture/compare register 3
/// Size: 0x20 bits
pub mod ccr3 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x3C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CCR3_BIT_OFFSET: u8 = 0;
	const CCR3_BIT_WIDTH: u8 = 16;
	/// Capture/Compare 3 value (Width: 16, Offset: 0)
	pub fn get_ccr3() -> u16 { ::read(REGISTER_ADDRESS, CCR3_BIT_OFFSET, CCR3_BIT_WIDTH) as u16 }
	/// Capture/Compare 3 value (Width: 16, Offset: 0)
	pub fn set_ccr3(value: u16) { ::write(REGISTER_ADDRESS, CCR3_BIT_OFFSET, CCR3_BIT_WIDTH, value as u32); }
}
/// capture/compare register 4
/// Size: 0x20 bits
pub mod ccr4 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x40;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CCR4_BIT_OFFSET: u8 = 0;
	const CCR4_BIT_WIDTH: u8 = 16;
	/// Capture/Compare 3 value (Width: 16, Offset: 0)
	pub fn get_ccr4() -> u16 { ::read(REGISTER_ADDRESS, CCR4_BIT_OFFSET, CCR4_BIT_WIDTH) as u16 }
	/// Capture/Compare 3 value (Width: 16, Offset: 0)
	pub fn set_ccr4(value: u16) { ::write(REGISTER_ADDRESS, CCR4_BIT_OFFSET, CCR4_BIT_WIDTH, value as u32); }
}
/// break and dead-time register
/// Size: 0x20 bits
pub mod bdtr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x44;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DTG_BIT_OFFSET: u8 = 0;
	const DTG_BIT_WIDTH: u8 = 8;
	/// Dead-time generator setup (Width: 8, Offset: 0)
	pub fn get_dtg() -> u8 { ::read(REGISTER_ADDRESS, DTG_BIT_OFFSET, DTG_BIT_WIDTH) as u8 }
	/// Dead-time generator setup (Width: 8, Offset: 0)
	pub fn set_dtg(value: u8) { ::write(REGISTER_ADDRESS, DTG_BIT_OFFSET, DTG_BIT_WIDTH, value as u32); }

	const LOCK_BIT_OFFSET: u8 = 8;
	const LOCK_BIT_WIDTH: u8 = 2;
	/// Lock configuration (Width: 2, Offset: 8)
	pub fn get_lock() -> u8 { ::read(REGISTER_ADDRESS, LOCK_BIT_OFFSET, LOCK_BIT_WIDTH) as u8 }
	/// Lock configuration (Width: 2, Offset: 8)
	pub fn set_lock(value: u8) { ::write(REGISTER_ADDRESS, LOCK_BIT_OFFSET, LOCK_BIT_WIDTH, value as u32); }

	const OSSI_BIT_OFFSET: u8 = 10;
	const OSSI_BIT_WIDTH: u8 = 1;
	/// Off-state selection for Idle mode (Width: 1, Offset: 10)
	pub fn get_ossi() -> u8 { ::read(REGISTER_ADDRESS, OSSI_BIT_OFFSET, OSSI_BIT_WIDTH) as u8 }
	/// Off-state selection for Idle mode (Width: 1, Offset: 10)
	pub fn set_ossi(value: u8) { ::write(REGISTER_ADDRESS, OSSI_BIT_OFFSET, OSSI_BIT_WIDTH, value as u32); }

	const OSSR_BIT_OFFSET: u8 = 11;
	const OSSR_BIT_WIDTH: u8 = 1;
	/// Off-state selection for Run mode (Width: 1, Offset: 11)
	pub fn get_ossr() -> u8 { ::read(REGISTER_ADDRESS, OSSR_BIT_OFFSET, OSSR_BIT_WIDTH) as u8 }
	/// Off-state selection for Run mode (Width: 1, Offset: 11)
	pub fn set_ossr(value: u8) { ::write(REGISTER_ADDRESS, OSSR_BIT_OFFSET, OSSR_BIT_WIDTH, value as u32); }

	const BKE_BIT_OFFSET: u8 = 12;
	const BKE_BIT_WIDTH: u8 = 1;
	/// Break enable (Width: 1, Offset: 12)
	pub fn get_bke() -> u8 { ::read(REGISTER_ADDRESS, BKE_BIT_OFFSET, BKE_BIT_WIDTH) as u8 }
	/// Break enable (Width: 1, Offset: 12)
	pub fn set_bke(value: u8) { ::write(REGISTER_ADDRESS, BKE_BIT_OFFSET, BKE_BIT_WIDTH, value as u32); }

	const BKP_BIT_OFFSET: u8 = 13;
	const BKP_BIT_WIDTH: u8 = 1;
	/// Break polarity (Width: 1, Offset: 13)
	pub fn get_bkp() -> u8 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u8 }
	/// Break polarity (Width: 1, Offset: 13)
	pub fn set_bkp(value: u8) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }

	const AOE_BIT_OFFSET: u8 = 14;
	const AOE_BIT_WIDTH: u8 = 1;
	/// Automatic output enable (Width: 1, Offset: 14)
	pub fn get_aoe() -> u8 { ::read(REGISTER_ADDRESS, AOE_BIT_OFFSET, AOE_BIT_WIDTH) as u8 }
	/// Automatic output enable (Width: 1, Offset: 14)
	pub fn set_aoe(value: u8) { ::write(REGISTER_ADDRESS, AOE_BIT_OFFSET, AOE_BIT_WIDTH, value as u32); }

	const MOE_BIT_OFFSET: u8 = 15;
	const MOE_BIT_WIDTH: u8 = 1;
	/// Main output enable (Width: 1, Offset: 15)
	pub fn get_moe() -> u8 { ::read(REGISTER_ADDRESS, MOE_BIT_OFFSET, MOE_BIT_WIDTH) as u8 }
	/// Main output enable (Width: 1, Offset: 15)
	pub fn set_moe(value: u8) { ::write(REGISTER_ADDRESS, MOE_BIT_OFFSET, MOE_BIT_WIDTH, value as u32); }

	const BKF_BIT_OFFSET: u8 = 16;
	const BKF_BIT_WIDTH: u8 = 4;
	/// Break filter (Width: 4, Offset: 16)
	pub fn get_bkf() -> u8 { ::read(REGISTER_ADDRESS, BKF_BIT_OFFSET, BKF_BIT_WIDTH) as u8 }
	/// Break filter (Width: 4, Offset: 16)
	pub fn set_bkf(value: u8) { ::write(REGISTER_ADDRESS, BKF_BIT_OFFSET, BKF_BIT_WIDTH, value as u32); }

	const BK2F_BIT_OFFSET: u8 = 20;
	const BK2F_BIT_WIDTH: u8 = 4;
	/// Break 2 filter (Width: 4, Offset: 20)
	pub fn get_bk2f() -> u8 { ::read(REGISTER_ADDRESS, BK2F_BIT_OFFSET, BK2F_BIT_WIDTH) as u8 }
	/// Break 2 filter (Width: 4, Offset: 20)
	pub fn set_bk2f(value: u8) { ::write(REGISTER_ADDRESS, BK2F_BIT_OFFSET, BK2F_BIT_WIDTH, value as u32); }

	const BK2E_BIT_OFFSET: u8 = 24;
	const BK2E_BIT_WIDTH: u8 = 1;
	/// Break 2 enable (Width: 1, Offset: 24)
	pub fn get_bk2e() -> u8 { ::read(REGISTER_ADDRESS, BK2E_BIT_OFFSET, BK2E_BIT_WIDTH) as u8 }
	/// Break 2 enable (Width: 1, Offset: 24)
	pub fn set_bk2e(value: u8) { ::write(REGISTER_ADDRESS, BK2E_BIT_OFFSET, BK2E_BIT_WIDTH, value as u32); }

	const BK2P_BIT_OFFSET: u8 = 25;
	const BK2P_BIT_WIDTH: u8 = 1;
	/// Break 2 polarity (Width: 1, Offset: 25)
	pub fn get_bk2p() -> u8 { ::read(REGISTER_ADDRESS, BK2P_BIT_OFFSET, BK2P_BIT_WIDTH) as u8 }
	/// Break 2 polarity (Width: 1, Offset: 25)
	pub fn set_bk2p(value: u8) { ::write(REGISTER_ADDRESS, BK2P_BIT_OFFSET, BK2P_BIT_WIDTH, value as u32); }
}
/// DMA control register
/// Size: 0x20 bits
pub mod dcr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x48;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DBL_BIT_OFFSET: u8 = 8;
	const DBL_BIT_WIDTH: u8 = 5;
	/// DMA burst length (Width: 5, Offset: 8)
	pub fn get_dbl() -> u8 { ::read(REGISTER_ADDRESS, DBL_BIT_OFFSET, DBL_BIT_WIDTH) as u8 }
	/// DMA burst length (Width: 5, Offset: 8)
	pub fn set_dbl(value: u8) { ::write(REGISTER_ADDRESS, DBL_BIT_OFFSET, DBL_BIT_WIDTH, value as u32); }

	const DBA_BIT_OFFSET: u8 = 0;
	const DBA_BIT_WIDTH: u8 = 5;
	/// DMA base address (Width: 5, Offset: 0)
	pub fn get_dba() -> u8 { ::read(REGISTER_ADDRESS, DBA_BIT_OFFSET, DBA_BIT_WIDTH) as u8 }
	/// DMA base address (Width: 5, Offset: 0)
	pub fn set_dba(value: u8) { ::write(REGISTER_ADDRESS, DBA_BIT_OFFSET, DBA_BIT_WIDTH, value as u32); }
}
/// DMA address for full transfer
/// Size: 0x20 bits
pub mod dmar {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DMAB_BIT_OFFSET: u8 = 0;
	const DMAB_BIT_WIDTH: u8 = 16;
	/// DMA register for burst accesses (Width: 16, Offset: 0)
	pub fn get_dmab() -> u16 { ::read(REGISTER_ADDRESS, DMAB_BIT_OFFSET, DMAB_BIT_WIDTH) as u16 }
	/// DMA register for burst accesses (Width: 16, Offset: 0)
	pub fn set_dmab(value: u16) { ::write(REGISTER_ADDRESS, DMAB_BIT_OFFSET, DMAB_BIT_WIDTH, value as u32); }
}
/// capture/compare mode register 3 (output mode)
/// Size: 0x20 bits
pub mod ccmr3_output {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x54;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OC5FE_BIT_OFFSET: u8 = 2;
	const OC5FE_BIT_WIDTH: u8 = 1;
	/// Output compare 5 fast enable (Width: 1, Offset: 2)
	pub fn get_oc5fe() -> u8 { ::read(REGISTER_ADDRESS, OC5FE_BIT_OFFSET, OC5FE_BIT_WIDTH) as u8 }
	/// Output compare 5 fast enable (Width: 1, Offset: 2)
	pub fn set_oc5fe(value: u8) { ::write(REGISTER_ADDRESS, OC5FE_BIT_OFFSET, OC5FE_BIT_WIDTH, value as u32); }

	const OC5PE_BIT_OFFSET: u8 = 3;
	const OC5PE_BIT_WIDTH: u8 = 1;
	/// Output compare 5 preload enable (Width: 1, Offset: 3)
	pub fn get_oc5pe() -> u8 { ::read(REGISTER_ADDRESS, OC5PE_BIT_OFFSET, OC5PE_BIT_WIDTH) as u8 }
	/// Output compare 5 preload enable (Width: 1, Offset: 3)
	pub fn set_oc5pe(value: u8) { ::write(REGISTER_ADDRESS, OC5PE_BIT_OFFSET, OC5PE_BIT_WIDTH, value as u32); }

	const OC5M_BIT_OFFSET: u8 = 4;
	const OC5M_BIT_WIDTH: u8 = 3;
	/// Output compare 5 mode (Width: 3, Offset: 4)
	pub fn get_oc5m() -> u8 { ::read(REGISTER_ADDRESS, OC5M_BIT_OFFSET, OC5M_BIT_WIDTH) as u8 }
	/// Output compare 5 mode (Width: 3, Offset: 4)
	pub fn set_oc5m(value: u8) { ::write(REGISTER_ADDRESS, OC5M_BIT_OFFSET, OC5M_BIT_WIDTH, value as u32); }

	const OC5CE_BIT_OFFSET: u8 = 7;
	const OC5CE_BIT_WIDTH: u8 = 1;
	/// Output compare 5 clear enable (Width: 1, Offset: 7)
	pub fn get_oc5ce() -> u8 { ::read(REGISTER_ADDRESS, OC5CE_BIT_OFFSET, OC5CE_BIT_WIDTH) as u8 }
	/// Output compare 5 clear enable (Width: 1, Offset: 7)
	pub fn set_oc5ce(value: u8) { ::write(REGISTER_ADDRESS, OC5CE_BIT_OFFSET, OC5CE_BIT_WIDTH, value as u32); }

	const OC6FE_BIT_OFFSET: u8 = 10;
	const OC6FE_BIT_WIDTH: u8 = 1;
	/// Output compare 6 fast enable (Width: 1, Offset: 10)
	pub fn get_oc6fe() -> u8 { ::read(REGISTER_ADDRESS, OC6FE_BIT_OFFSET, OC6FE_BIT_WIDTH) as u8 }
	/// Output compare 6 fast enable (Width: 1, Offset: 10)
	pub fn set_oc6fe(value: u8) { ::write(REGISTER_ADDRESS, OC6FE_BIT_OFFSET, OC6FE_BIT_WIDTH, value as u32); }

	const OC6PE_BIT_OFFSET: u8 = 11;
	const OC6PE_BIT_WIDTH: u8 = 1;
	/// Output compare 6 preload enable (Width: 1, Offset: 11)
	pub fn get_oc6pe() -> u8 { ::read(REGISTER_ADDRESS, OC6PE_BIT_OFFSET, OC6PE_BIT_WIDTH) as u8 }
	/// Output compare 6 preload enable (Width: 1, Offset: 11)
	pub fn set_oc6pe(value: u8) { ::write(REGISTER_ADDRESS, OC6PE_BIT_OFFSET, OC6PE_BIT_WIDTH, value as u32); }

	const OC6M_BIT_OFFSET: u8 = 12;
	const OC6M_BIT_WIDTH: u8 = 3;
	/// Output compare 6 mode (Width: 3, Offset: 12)
	pub fn get_oc6m() -> u8 { ::read(REGISTER_ADDRESS, OC6M_BIT_OFFSET, OC6M_BIT_WIDTH) as u8 }
	/// Output compare 6 mode (Width: 3, Offset: 12)
	pub fn set_oc6m(value: u8) { ::write(REGISTER_ADDRESS, OC6M_BIT_OFFSET, OC6M_BIT_WIDTH, value as u32); }

	const OC6CE_BIT_OFFSET: u8 = 15;
	const OC6CE_BIT_WIDTH: u8 = 1;
	/// Output compare 6 clear enable (Width: 1, Offset: 15)
	pub fn get_oc6ce() -> u8 { ::read(REGISTER_ADDRESS, OC6CE_BIT_OFFSET, OC6CE_BIT_WIDTH) as u8 }
	/// Output compare 6 clear enable (Width: 1, Offset: 15)
	pub fn set_oc6ce(value: u8) { ::write(REGISTER_ADDRESS, OC6CE_BIT_OFFSET, OC6CE_BIT_WIDTH, value as u32); }

	const OC5M_3_BIT_OFFSET: u8 = 16;
	const OC5M_3_BIT_WIDTH: u8 = 1;
	/// Outout Compare 5 mode bit 3 (Width: 1, Offset: 16)
	pub fn get_oc5m_3() -> u8 { ::read(REGISTER_ADDRESS, OC5M_3_BIT_OFFSET, OC5M_3_BIT_WIDTH) as u8 }
	/// Outout Compare 5 mode bit 3 (Width: 1, Offset: 16)
	pub fn set_oc5m_3(value: u8) { ::write(REGISTER_ADDRESS, OC5M_3_BIT_OFFSET, OC5M_3_BIT_WIDTH, value as u32); }

	const OC6M_3_BIT_OFFSET: u8 = 24;
	const OC6M_3_BIT_WIDTH: u8 = 1;
	/// Outout Compare 6 mode bit 3 (Width: 1, Offset: 24)
	pub fn get_oc6m_3() -> u8 { ::read(REGISTER_ADDRESS, OC6M_3_BIT_OFFSET, OC6M_3_BIT_WIDTH) as u8 }
	/// Outout Compare 6 mode bit 3 (Width: 1, Offset: 24)
	pub fn set_oc6m_3(value: u8) { ::write(REGISTER_ADDRESS, OC6M_3_BIT_OFFSET, OC6M_3_BIT_WIDTH, value as u32); }
}
/// capture/compare register 5
/// Size: 0x20 bits
pub mod ccr5 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x58;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CCR5_BIT_OFFSET: u8 = 0;
	const CCR5_BIT_WIDTH: u8 = 16;
	/// Capture/Compare 5 value (Width: 16, Offset: 0)
	pub fn get_ccr5() -> u16 { ::read(REGISTER_ADDRESS, CCR5_BIT_OFFSET, CCR5_BIT_WIDTH) as u16 }
	/// Capture/Compare 5 value (Width: 16, Offset: 0)
	pub fn set_ccr5(value: u16) { ::write(REGISTER_ADDRESS, CCR5_BIT_OFFSET, CCR5_BIT_WIDTH, value as u32); }

	const GC5C1_BIT_OFFSET: u8 = 29;
	const GC5C1_BIT_WIDTH: u8 = 1;
	/// Group Channel 5 and Channel 1 (Width: 1, Offset: 29)
	pub fn get_gc5c1() -> u8 { ::read(REGISTER_ADDRESS, GC5C1_BIT_OFFSET, GC5C1_BIT_WIDTH) as u8 }
	/// Group Channel 5 and Channel 1 (Width: 1, Offset: 29)
	pub fn set_gc5c1(value: u8) { ::write(REGISTER_ADDRESS, GC5C1_BIT_OFFSET, GC5C1_BIT_WIDTH, value as u32); }

	const GC5C2_BIT_OFFSET: u8 = 30;
	const GC5C2_BIT_WIDTH: u8 = 1;
	/// Group Channel 5 and Channel 2 (Width: 1, Offset: 30)
	pub fn get_gc5c2() -> u8 { ::read(REGISTER_ADDRESS, GC5C2_BIT_OFFSET, GC5C2_BIT_WIDTH) as u8 }
	/// Group Channel 5 and Channel 2 (Width: 1, Offset: 30)
	pub fn set_gc5c2(value: u8) { ::write(REGISTER_ADDRESS, GC5C2_BIT_OFFSET, GC5C2_BIT_WIDTH, value as u32); }

	const GC5C3_BIT_OFFSET: u8 = 31;
	const GC5C3_BIT_WIDTH: u8 = 1;
	/// Group Channel 5 and Channel 3 (Width: 1, Offset: 31)
	pub fn get_gc5c3() -> u8 { ::read(REGISTER_ADDRESS, GC5C3_BIT_OFFSET, GC5C3_BIT_WIDTH) as u8 }
	/// Group Channel 5 and Channel 3 (Width: 1, Offset: 31)
	pub fn set_gc5c3(value: u8) { ::write(REGISTER_ADDRESS, GC5C3_BIT_OFFSET, GC5C3_BIT_WIDTH, value as u32); }
}
/// capture/compare register 6
/// Size: 0x20 bits
pub mod ccr6 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x5C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CCR6_BIT_OFFSET: u8 = 0;
	const CCR6_BIT_WIDTH: u8 = 16;
	/// Capture/Compare 6 value (Width: 16, Offset: 0)
	pub fn get_ccr6() -> u16 { ::read(REGISTER_ADDRESS, CCR6_BIT_OFFSET, CCR6_BIT_WIDTH) as u16 }
	/// Capture/Compare 6 value (Width: 16, Offset: 0)
	pub fn set_ccr6(value: u16) { ::write(REGISTER_ADDRESS, CCR6_BIT_OFFSET, CCR6_BIT_WIDTH, value as u32); }
}
/// option registers
/// Size: 0x20 bits
pub mod or {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x60;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TIM8_ETR_ADC2_RMP_BIT_OFFSET: u8 = 0;
	const TIM8_ETR_ADC2_RMP_BIT_WIDTH: u8 = 2;
	/// TIM8_ETR_ADC2 remapping capability (Width: 2, Offset: 0)
	pub fn get_tim8_etr_adc2_rmp() -> u8 { ::read(REGISTER_ADDRESS, TIM8_ETR_ADC2_RMP_BIT_OFFSET, TIM8_ETR_ADC2_RMP_BIT_WIDTH) as u8 }
	/// TIM8_ETR_ADC2 remapping capability (Width: 2, Offset: 0)
	pub fn set_tim8_etr_adc2_rmp(value: u8) { ::write(REGISTER_ADDRESS, TIM8_ETR_ADC2_RMP_BIT_OFFSET, TIM8_ETR_ADC2_RMP_BIT_WIDTH, value as u32); }

	const TIM8_ETR_ADC3_RMP_BIT_OFFSET: u8 = 2;
	const TIM8_ETR_ADC3_RMP_BIT_WIDTH: u8 = 2;
	/// TIM8_ETR_ADC3 remapping capability (Width: 2, Offset: 2)
	pub fn get_tim8_etr_adc3_rmp() -> u8 { ::read(REGISTER_ADDRESS, TIM8_ETR_ADC3_RMP_BIT_OFFSET, TIM8_ETR_ADC3_RMP_BIT_WIDTH) as u8 }
	/// TIM8_ETR_ADC3 remapping capability (Width: 2, Offset: 2)
	pub fn set_tim8_etr_adc3_rmp(value: u8) { ::write(REGISTER_ADDRESS, TIM8_ETR_ADC3_RMP_BIT_OFFSET, TIM8_ETR_ADC3_RMP_BIT_WIDTH, value as u32); }
}
/// TIM8 break interrupt
pub const INTERRUPT_TIM8_BRK: u32 = 43;

/// TIM8 update interrupt
pub const INTERRUPT_TIM8_UP: u32 = 44;

/// TIM8 Trigger and commutation interrupts
pub const INTERRUPT_TIM8_TRG_COM: u32 = 45;

/// TIM8 capture compare interrupt
pub const INTERRUPT_TIM8_CC: u32 = 46;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>TIM8</name>
  <description>Advanced-timers</description>
  <groupName>TIMs</groupName>
  <baseAddress>0x40013400</baseAddress>
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
          <name>CEN</name>
          <description>Counter enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>UDIS</name>
          <description>Update disable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>URS</name>
          <description>Update request source</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OPM</name>
          <description>One-pulse mode</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DIR</name>
          <description>Direction</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CMS</name>
          <description>Center-aligned mode
              selection</description>
          <bitOffset>5</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>ARPE</name>
          <description>Auto-reload preload enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CKD</name>
          <description>Clock division</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>UIFREMAP</name>
          <description>UIF status bit remapping</description>
          <bitOffset>11</bitOffset>
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
          <name>CCPC</name>
          <description>Capture/compare preloaded
              control</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CCUS</name>
          <description>Capture/compare control update
              selection</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CCDS</name>
          <description>Capture/compare DMA
              selection</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MMS</name>
          <description>Master mode selection</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>TI1S</name>
          <description>TI1 selection</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OIS1</name>
          <description>Output Idle state 1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OIS1N</name>
          <description>Output Idle state 1</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OIS2</name>
          <description>Output Idle state 2</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OIS2N</name>
          <description>Output Idle state 2</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OIS3</name>
          <description>Output Idle state 3</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OIS3N</name>
          <description>Output Idle state 3</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OIS4</name>
          <description>Output Idle state 4</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OIS5</name>
          <description>Output Idle state 5</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OIS6</name>
          <description>Output Idle state 6</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MMS2</name>
          <description>Master mode selection 2</description>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SMCR</name>
      <displayName>SMCR</displayName>
      <description>slave mode control register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>SMS</name>
          <description>Slave mode selection</description>
          <bitOffset>0</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>OCCS</name>
          <description>OCREF clear selection</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TS</name>
          <description>Trigger selection</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>MSM</name>
          <description>Master/Slave mode</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ETF</name>
          <description>External trigger filter</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>ETPS</name>
          <description>External trigger prescaler</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>ECE</name>
          <description>External clock enable</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ETP</name>
          <description>External trigger polarity</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SMS3</name>
          <description>Slave mode selection bit 3</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DIER</name>
      <displayName>DIER</displayName>
      <description>DMA/Interrupt enable register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>TDE</name>
          <description>Trigger DMA request enable</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>COMDE</name>
          <description>Reserved</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC4DE</name>
          <description>Capture/Compare 4 DMA request
              enable</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC3DE</name>
          <description>Capture/Compare 3 DMA request
              enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC2DE</name>
          <description>Capture/Compare 2 DMA request
              enable</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC1DE</name>
          <description>Capture/Compare 1 DMA request
              enable</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>UDE</name>
          <description>Update DMA request enable</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BIE</name>
          <description>Break interrupt enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIE</name>
          <description>Trigger interrupt enable</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>COMIE</name>
          <description>COM interrupt enable</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC4IE</name>
          <description>Capture/Compare 4 interrupt
              enable</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC3IE</name>
          <description>Capture/Compare 3 interrupt
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC2IE</name>
          <description>Capture/Compare 2 interrupt
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC1IE</name>
          <description>Capture/Compare 1 interrupt
              enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>UIE</name>
          <description>Update interrupt enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SR</name>
      <displayName>SR</displayName>
      <description>status register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>UIF</name>
          <description>Update interrupt flag</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC1IF</name>
          <description>Capture/compare 1 interrupt
              flag</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC2IF</name>
          <description>Capture/Compare 2 interrupt
              flag</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC3IF</name>
          <description>Capture/Compare 3 interrupt
              flag</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC4IF</name>
          <description>Capture/Compare 4 interrupt
              flag</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>COMIF</name>
          <description>COM interrupt flag</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIF</name>
          <description>Trigger interrupt flag</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BIF</name>
          <description>Break interrupt flag</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>B2IF</name>
          <description>Break 2 interrupt flag</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC1OF</name>
          <description>Capture/Compare 1 overcapture
              flag</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC2OF</name>
          <description>Capture/compare 2 overcapture
              flag</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC3OF</name>
          <description>Capture/Compare 3 overcapture
              flag</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC4OF</name>
          <description>Capture/Compare 4 overcapture
              flag</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>C5IF</name>
          <description>Capture/Compare 5 interrupt
              flag</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>C6IF</name>
          <description>Capture/Compare 6 interrupt
              flag</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>EGR</name>
      <displayName>EGR</displayName>
      <description>event generation register</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>write-only</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>UG</name>
          <description>Update generation</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC1G</name>
          <description>Capture/compare 1
              generation</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC2G</name>
          <description>Capture/compare 2
              generation</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC3G</name>
          <description>Capture/compare 3
              generation</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC4G</name>
          <description>Capture/compare 4
              generation</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>COMG</name>
          <description>Capture/Compare control update
              generation</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TG</name>
          <description>Trigger generation</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BG</name>
          <description>Break generation</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>B2G</name>
          <description>Break 2 generation</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCMR1_Output</name>
      <displayName>CCMR1_Output</displayName>
      <description>capture/compare mode register (output
          mode)</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OC2CE</name>
          <description>Output Compare 2 clear
              enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC2M</name>
          <description>Output Compare 2 mode</description>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>OC2PE</name>
          <description>Output Compare 2 preload
              enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC2FE</name>
          <description>Output Compare 2 fast
              enable</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC2S</name>
          <description>Capture/Compare 2
              selection</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OC1CE</name>
          <description>Output Compare 1 clear
              enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC1M</name>
          <description>Output Compare 1 mode</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>OC1PE</name>
          <description>Output Compare 1 preload
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC1FE</name>
          <description>Output Compare 1 fast
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC1S</name>
          <description>Capture/Compare 1
              selection</description>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OC1M_3</name>
          <description>Output Compare 1 mode bit
              3</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC2M_3</name>
          <description>Output Compare 2 mode bit
              3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCMR1_Input</name>
      <displayName>CCMR1_Input</displayName>
      <description>capture/compare mode register 1 (input
          mode)</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IC2F</name>
          <description>Input capture 2 filter</description>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>IC2PCS</name>
          <description>Input capture 2 prescaler</description>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>CC2S</name>
          <description>Capture/Compare 2
              selection</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>IC1F</name>
          <description>Input capture 1 filter</description>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>IC1PCS</name>
          <description>Input capture 1 prescaler</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>CC1S</name>
          <description>Capture/Compare 1
              selection</description>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
      </fields>
      <alternateRegister>CCMR1_Output</alternateRegister>
    </register>
    <register>
      <name>CCMR2_Output</name>
      <displayName>CCMR2_Output</displayName>
      <description>capture/compare mode register (output
          mode)</description>
      <addressOffset>0x1C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OC4CE</name>
          <description>Output compare 4 clear
              enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC4M</name>
          <description>Output compare 4 mode</description>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>OC4PE</name>
          <description>Output compare 4 preload
              enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC4FE</name>
          <description>Output compare 4 fast
              enable</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC4S</name>
          <description>Capture/Compare 4
              selection</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OC3CE</name>
          <description>Output compare 3 clear
              enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC3M</name>
          <description>Output compare 3 mode</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>OC3PE</name>
          <description>Output compare 3 preload
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC3FE</name>
          <description>Output compare 3 fast
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC3S</name>
          <description>Capture/Compare 3
              selection</description>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OC3M_3</name>
          <description>Output Compare 3 mode bit
              3</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC4M_3</name>
          <description>Output Compare 4 mode bit
              3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCMR2_Input</name>
      <displayName>CCMR2_Input</displayName>
      <description>capture/compare mode register 2 (input
          mode)</description>
      <addressOffset>0x1C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IC4F</name>
          <description>Input capture 4 filter</description>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>IC4PSC</name>
          <description>Input capture 4 prescaler</description>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>CC4S</name>
          <description>Capture/Compare 4
              selection</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>IC3F</name>
          <description>Input capture 3 filter</description>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>IC3PSC</name>
          <description>Input capture 3 prescaler</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>CC3S</name>
          <description>Capture/compare 3
              selection</description>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
      </fields>
      <alternateRegister>CCMR2_Output</alternateRegister>
    </register>
    <register>
      <name>CCER</name>
      <displayName>CCER</displayName>
      <description>capture/compare enable
          register</description>
      <addressOffset>0x20</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>CC1E</name>
          <description>Capture/Compare 1 output
              enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC1P</name>
          <description>Capture/Compare 1 output
              Polarity</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC1NE</name>
          <description>Capture/Compare 1 complementary output
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC1NP</name>
          <description>Capture/Compare 1 output
              Polarity</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC2E</name>
          <description>Capture/Compare 2 output
              enable</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC2P</name>
          <description>Capture/Compare 2 output
              Polarity</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC2NE</name>
          <description>Capture/Compare 2 complementary output
              enable</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC2NP</name>
          <description>Capture/Compare 2 output
              Polarity</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC3E</name>
          <description>Capture/Compare 3 output
              enable</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC3P</name>
          <description>Capture/Compare 3 output
              Polarity</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC3NE</name>
          <description>Capture/Compare 3 complementary output
              enable</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC3NP</name>
          <description>Capture/Compare 3 output
              Polarity</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC4E</name>
          <description>Capture/Compare 4 output
              enable</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC4P</name>
          <description>Capture/Compare 3 output
              Polarity</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC4NP</name>
          <description>Capture/Compare 4 output
              Polarity</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC5E</name>
          <description>Capture/Compare 5 output
              enable</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC5P</name>
          <description>Capture/Compare 5 output
              Polarity</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC6E</name>
          <description>Capture/Compare 6 output
              enable</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC6P</name>
          <description>Capture/Compare 6 output
              Polarity</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CNT</name>
      <displayName>CNT</displayName>
      <description>counter</description>
      <addressOffset>0x24</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CNT</name>
          <description>counter value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>UIFCPY</name>
          <description>UIF copy</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
      </fields>
    </register>
    <register>
      <name>PSC</name>
      <displayName>PSC</displayName>
      <description>prescaler</description>
      <addressOffset>0x28</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>PSC</name>
          <description>Prescaler value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ARR</name>
      <displayName>ARR</displayName>
      <description>auto-reload register</description>
      <addressOffset>0x2C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>ARR</name>
          <description>Auto-reload value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>RCR</name>
      <displayName>RCR</displayName>
      <description>repetition counter register</description>
      <addressOffset>0x30</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>REP</name>
          <description>Repetition counter value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCR1</name>
      <displayName>CCR1</displayName>
      <description>capture/compare register 1</description>
      <addressOffset>0x34</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CCR1</name>
          <description>Capture/Compare 1 value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCR2</name>
      <displayName>CCR2</displayName>
      <description>capture/compare register 2</description>
      <addressOffset>0x38</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CCR2</name>
          <description>Capture/Compare 2 value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCR3</name>
      <displayName>CCR3</displayName>
      <description>capture/compare register 3</description>
      <addressOffset>0x3C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CCR3</name>
          <description>Capture/Compare 3 value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCR4</name>
      <displayName>CCR4</displayName>
      <description>capture/compare register 4</description>
      <addressOffset>0x40</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CCR4</name>
          <description>Capture/Compare 3 value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BDTR</name>
      <displayName>BDTR</displayName>
      <description>break and dead-time register</description>
      <addressOffset>0x44</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DTG</name>
          <description>Dead-time generator setup</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>LOCK</name>
          <description>Lock configuration</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OSSI</name>
          <description>Off-state selection for Idle
              mode</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OSSR</name>
          <description>Off-state selection for Run
              mode</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BKE</name>
          <description>Break enable</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BKP</name>
          <description>Break polarity</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AOE</name>
          <description>Automatic output enable</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MOE</name>
          <description>Main output enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BKF</name>
          <description>Break filter</description>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>BK2F</name>
          <description>Break 2 filter</description>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>BK2E</name>
          <description>Break 2 enable</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BK2P</name>
          <description>Break 2 polarity</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DCR</name>
      <displayName>DCR</displayName>
      <description>DMA control register</description>
      <addressOffset>0x48</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DBL</name>
          <description>DMA burst length</description>
          <bitOffset>8</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>DBA</name>
          <description>DMA base address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DMAR</name>
      <displayName>DMAR</displayName>
      <description>DMA address for full transfer</description>
      <addressOffset>0x4C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DMAB</name>
          <description>DMA register for burst
              accesses</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCMR3_Output</name>
      <displayName>CCMR3_Output</displayName>
      <description>capture/compare mode register 3 (output
          mode)</description>
      <addressOffset>0x54</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OC5FE</name>
          <description>Output compare 5 fast
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC5PE</name>
          <description>Output compare 5 preload
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC5M</name>
          <description>Output compare 5 mode</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>OC5CE</name>
          <description>Output compare 5 clear
              enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC6FE</name>
          <description>Output compare 6 fast
              enable</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC6PE</name>
          <description>Output compare 6 preload
              enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC6M</name>
          <description>Output compare 6 mode</description>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>OC6CE</name>
          <description>Output compare 6 clear
              enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC5M_3</name>
          <description>Outout Compare 5 mode bit
              3</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC6M_3</name>
          <description>Outout Compare 6 mode bit
              3</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCR5</name>
      <displayName>CCR5</displayName>
      <description>capture/compare register 5</description>
      <addressOffset>0x58</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CCR5</name>
          <description>Capture/Compare 5 value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
        <field>
          <name>GC5C1</name>
          <description>Group Channel 5 and Channel
              1</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>GC5C2</name>
          <description>Group Channel 5 and Channel
              2</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>GC5C3</name>
          <description>Group Channel 5 and Channel
              3</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCR6</name>
      <displayName>CCR6</displayName>
      <description>capture/compare register 6</description>
      <addressOffset>0x5C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CCR6</name>
          <description>Capture/Compare 6 value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>OR</name>
      <displayName>OR</displayName>
      <description>option registers</description>
      <addressOffset>0x60</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>TIM8_ETR_ADC2_RMP</name>
          <description>TIM8_ETR_ADC2 remapping
              capability</description>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>TIM8_ETR_ADC3_RMP</name>
          <description>TIM8_ETR_ADC3 remapping
              capability</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>TIM8_BRK</name>
    <description>TIM8 break interrupt</description>
    <value>43</value>
  </interrupt>
  <interrupt>
    <name>TIM8_UP</name>
    <description>TIM8 update interrupt</description>
    <value>44</value>
  </interrupt>
  <interrupt>
    <name>TIM8_TRG_COM</name>
    <description>TIM8 Trigger and commutation
        interrupts</description>
    <value>45</value>
  </interrupt>
  <interrupt>
    <name>TIM8_CC</name>
    <description>TIM8 capture compare interrupt</description>
    <value>46</value>
  </interrupt>
</peripheral>*/
