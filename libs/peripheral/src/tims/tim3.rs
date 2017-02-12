/// MOD TIM3
/// General purpose timer
const BASE_ADDRESS: u32 = 0x40000400;
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

	const TI1S_BIT_OFFSET: u8 = 7;
	const TI1S_BIT_WIDTH: u8 = 1;
	/// TI1 selection (Width: 1, Offset: 7)
	pub fn get_ti1s() -> u8 { ::read(REGISTER_ADDRESS, TI1S_BIT_OFFSET, TI1S_BIT_WIDTH) as u8 }
	/// TI1 selection (Width: 1, Offset: 7)
	pub fn set_ti1s(value: u8) { ::write(REGISTER_ADDRESS, TI1S_BIT_OFFSET, TI1S_BIT_WIDTH, value as u32); }

	const MMS_BIT_OFFSET: u8 = 4;
	const MMS_BIT_WIDTH: u8 = 3;
	/// Master mode selection (Width: 3, Offset: 4)
	pub fn get_mms() -> u8 { ::read(REGISTER_ADDRESS, MMS_BIT_OFFSET, MMS_BIT_WIDTH) as u8 }
	/// Master mode selection (Width: 3, Offset: 4)
	pub fn set_mms(value: u8) { ::write(REGISTER_ADDRESS, MMS_BIT_OFFSET, MMS_BIT_WIDTH, value as u32); }

	const CCDS_BIT_OFFSET: u8 = 3;
	const CCDS_BIT_WIDTH: u8 = 1;
	/// Capture/compare DMA selection (Width: 1, Offset: 3)
	pub fn get_ccds() -> u8 { ::read(REGISTER_ADDRESS, CCDS_BIT_OFFSET, CCDS_BIT_WIDTH) as u8 }
	/// Capture/compare DMA selection (Width: 1, Offset: 3)
	pub fn set_ccds(value: u8) { ::write(REGISTER_ADDRESS, CCDS_BIT_OFFSET, CCDS_BIT_WIDTH, value as u32); }
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

	const SMS_3_BIT_OFFSET: u8 = 16;
	const SMS_3_BIT_WIDTH: u8 = 1;
	/// Slave mode selection bit3 (Width: 1, Offset: 16)
	pub fn get_sms_3() -> u8 { ::read(REGISTER_ADDRESS, SMS_3_BIT_OFFSET, SMS_3_BIT_WIDTH) as u8 }
	/// Slave mode selection bit3 (Width: 1, Offset: 16)
	pub fn set_sms_3(value: u8) { ::write(REGISTER_ADDRESS, SMS_3_BIT_OFFSET, SMS_3_BIT_WIDTH, value as u32); }
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

	const TIE_BIT_OFFSET: u8 = 6;
	const TIE_BIT_WIDTH: u8 = 1;
	/// Trigger interrupt enable (Width: 1, Offset: 6)
	pub fn get_tie() -> u8 { ::read(REGISTER_ADDRESS, TIE_BIT_OFFSET, TIE_BIT_WIDTH) as u8 }
	/// Trigger interrupt enable (Width: 1, Offset: 6)
	pub fn set_tie(value: u8) { ::write(REGISTER_ADDRESS, TIE_BIT_OFFSET, TIE_BIT_WIDTH, value as u32); }

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

	const CC4OF_BIT_OFFSET: u8 = 12;
	const CC4OF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 4 overcapture flag (Width: 1, Offset: 12)
	pub fn get_cc4of() -> u8 { ::read(REGISTER_ADDRESS, CC4OF_BIT_OFFSET, CC4OF_BIT_WIDTH) as u8 }
	/// Capture/Compare 4 overcapture flag (Width: 1, Offset: 12)
	pub fn set_cc4of(value: u8) { ::write(REGISTER_ADDRESS, CC4OF_BIT_OFFSET, CC4OF_BIT_WIDTH, value as u32); }

	const CC3OF_BIT_OFFSET: u8 = 11;
	const CC3OF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 3 overcapture flag (Width: 1, Offset: 11)
	pub fn get_cc3of() -> u8 { ::read(REGISTER_ADDRESS, CC3OF_BIT_OFFSET, CC3OF_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 overcapture flag (Width: 1, Offset: 11)
	pub fn set_cc3of(value: u8) { ::write(REGISTER_ADDRESS, CC3OF_BIT_OFFSET, CC3OF_BIT_WIDTH, value as u32); }

	const CC2OF_BIT_OFFSET: u8 = 10;
	const CC2OF_BIT_WIDTH: u8 = 1;
	/// Capture/compare 2 overcapture flag (Width: 1, Offset: 10)
	pub fn get_cc2of() -> u8 { ::read(REGISTER_ADDRESS, CC2OF_BIT_OFFSET, CC2OF_BIT_WIDTH) as u8 }
	/// Capture/compare 2 overcapture flag (Width: 1, Offset: 10)
	pub fn set_cc2of(value: u8) { ::write(REGISTER_ADDRESS, CC2OF_BIT_OFFSET, CC2OF_BIT_WIDTH, value as u32); }

	const CC1OF_BIT_OFFSET: u8 = 9;
	const CC1OF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 overcapture flag (Width: 1, Offset: 9)
	pub fn get_cc1of() -> u8 { ::read(REGISTER_ADDRESS, CC1OF_BIT_OFFSET, CC1OF_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 overcapture flag (Width: 1, Offset: 9)
	pub fn set_cc1of(value: u8) { ::write(REGISTER_ADDRESS, CC1OF_BIT_OFFSET, CC1OF_BIT_WIDTH, value as u32); }

	const TIF_BIT_OFFSET: u8 = 6;
	const TIF_BIT_WIDTH: u8 = 1;
	/// Trigger interrupt flag (Width: 1, Offset: 6)
	pub fn get_tif() -> u8 { ::read(REGISTER_ADDRESS, TIF_BIT_OFFSET, TIF_BIT_WIDTH) as u8 }
	/// Trigger interrupt flag (Width: 1, Offset: 6)
	pub fn set_tif(value: u8) { ::write(REGISTER_ADDRESS, TIF_BIT_OFFSET, TIF_BIT_WIDTH, value as u32); }

	const CC4IF_BIT_OFFSET: u8 = 4;
	const CC4IF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 4 interrupt flag (Width: 1, Offset: 4)
	pub fn get_cc4if() -> u8 { ::read(REGISTER_ADDRESS, CC4IF_BIT_OFFSET, CC4IF_BIT_WIDTH) as u8 }
	/// Capture/Compare 4 interrupt flag (Width: 1, Offset: 4)
	pub fn set_cc4if(value: u8) { ::write(REGISTER_ADDRESS, CC4IF_BIT_OFFSET, CC4IF_BIT_WIDTH, value as u32); }

	const CC3IF_BIT_OFFSET: u8 = 3;
	const CC3IF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 3 interrupt flag (Width: 1, Offset: 3)
	pub fn get_cc3if() -> u8 { ::read(REGISTER_ADDRESS, CC3IF_BIT_OFFSET, CC3IF_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 interrupt flag (Width: 1, Offset: 3)
	pub fn set_cc3if(value: u8) { ::write(REGISTER_ADDRESS, CC3IF_BIT_OFFSET, CC3IF_BIT_WIDTH, value as u32); }

	const CC2IF_BIT_OFFSET: u8 = 2;
	const CC2IF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 2 interrupt flag (Width: 1, Offset: 2)
	pub fn get_cc2if() -> u8 { ::read(REGISTER_ADDRESS, CC2IF_BIT_OFFSET, CC2IF_BIT_WIDTH) as u8 }
	/// Capture/Compare 2 interrupt flag (Width: 1, Offset: 2)
	pub fn set_cc2if(value: u8) { ::write(REGISTER_ADDRESS, CC2IF_BIT_OFFSET, CC2IF_BIT_WIDTH, value as u32); }

	const CC1IF_BIT_OFFSET: u8 = 1;
	const CC1IF_BIT_WIDTH: u8 = 1;
	/// Capture/compare 1 interrupt flag (Width: 1, Offset: 1)
	pub fn get_cc1if() -> u8 { ::read(REGISTER_ADDRESS, CC1IF_BIT_OFFSET, CC1IF_BIT_WIDTH) as u8 }
	/// Capture/compare 1 interrupt flag (Width: 1, Offset: 1)
	pub fn set_cc1if(value: u8) { ::write(REGISTER_ADDRESS, CC1IF_BIT_OFFSET, CC1IF_BIT_WIDTH, value as u32); }

	const UIF_BIT_OFFSET: u8 = 0;
	const UIF_BIT_WIDTH: u8 = 1;
	/// Update interrupt flag (Width: 1, Offset: 0)
	pub fn get_uif() -> u8 { ::read(REGISTER_ADDRESS, UIF_BIT_OFFSET, UIF_BIT_WIDTH) as u8 }
	/// Update interrupt flag (Width: 1, Offset: 0)
	pub fn set_uif(value: u8) { ::write(REGISTER_ADDRESS, UIF_BIT_OFFSET, UIF_BIT_WIDTH, value as u32); }
}
/// event generation register
/// Size: 0x20 bits
pub mod egr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TG_BIT_OFFSET: u8 = 6;
	const TG_BIT_WIDTH: u8 = 1;
	/// Trigger generation (Width: 1, Offset: 6)
	pub fn set_tg(value: u8) { ::write(REGISTER_ADDRESS, TG_BIT_OFFSET, TG_BIT_WIDTH, value as u32); }

	const CC4G_BIT_OFFSET: u8 = 4;
	const CC4G_BIT_WIDTH: u8 = 1;
	/// Capture/compare 4 generation (Width: 1, Offset: 4)
	pub fn set_cc4g(value: u8) { ::write(REGISTER_ADDRESS, CC4G_BIT_OFFSET, CC4G_BIT_WIDTH, value as u32); }

	const CC3G_BIT_OFFSET: u8 = 3;
	const CC3G_BIT_WIDTH: u8 = 1;
	/// Capture/compare 3 generation (Width: 1, Offset: 3)
	pub fn set_cc3g(value: u8) { ::write(REGISTER_ADDRESS, CC3G_BIT_OFFSET, CC3G_BIT_WIDTH, value as u32); }

	const CC2G_BIT_OFFSET: u8 = 2;
	const CC2G_BIT_WIDTH: u8 = 1;
	/// Capture/compare 2 generation (Width: 1, Offset: 2)
	pub fn set_cc2g(value: u8) { ::write(REGISTER_ADDRESS, CC2G_BIT_OFFSET, CC2G_BIT_WIDTH, value as u32); }

	const CC1G_BIT_OFFSET: u8 = 1;
	const CC1G_BIT_WIDTH: u8 = 1;
	/// Capture/compare 1 generation (Width: 1, Offset: 1)
	pub fn set_cc1g(value: u8) { ::write(REGISTER_ADDRESS, CC1G_BIT_OFFSET, CC1G_BIT_WIDTH, value as u32); }

	const UG_BIT_OFFSET: u8 = 0;
	const UG_BIT_WIDTH: u8 = 1;
	/// Update generation (Width: 1, Offset: 0)
	pub fn set_ug(value: u8) { ::write(REGISTER_ADDRESS, UG_BIT_OFFSET, UG_BIT_WIDTH, value as u32); }
}
/// capture/compare mode register 1 (output mode)
/// Size: 0x20 bits
pub mod ccmr1_output {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CC1S_BIT_OFFSET: u8 = 0;
	const CC1S_BIT_WIDTH: u8 = 2;
	/// Capture/Compare 1 selection (Width: 2, Offset: 0)
	pub fn get_cc1s() -> u8 { ::read(REGISTER_ADDRESS, CC1S_BIT_OFFSET, CC1S_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 selection (Width: 2, Offset: 0)
	pub fn set_cc1s(value: u8) { ::write(REGISTER_ADDRESS, CC1S_BIT_OFFSET, CC1S_BIT_WIDTH, value as u32); }

	const OC1FE_BIT_OFFSET: u8 = 2;
	const OC1FE_BIT_WIDTH: u8 = 1;
	/// Output compare 1 fast enable (Width: 1, Offset: 2)
	pub fn get_oc1fe() -> u8 { ::read(REGISTER_ADDRESS, OC1FE_BIT_OFFSET, OC1FE_BIT_WIDTH) as u8 }
	/// Output compare 1 fast enable (Width: 1, Offset: 2)
	pub fn set_oc1fe(value: u8) { ::write(REGISTER_ADDRESS, OC1FE_BIT_OFFSET, OC1FE_BIT_WIDTH, value as u32); }

	const OC1PE_BIT_OFFSET: u8 = 3;
	const OC1PE_BIT_WIDTH: u8 = 1;
	/// Output compare 1 preload enable (Width: 1, Offset: 3)
	pub fn get_oc1pe() -> u8 { ::read(REGISTER_ADDRESS, OC1PE_BIT_OFFSET, OC1PE_BIT_WIDTH) as u8 }
	/// Output compare 1 preload enable (Width: 1, Offset: 3)
	pub fn set_oc1pe(value: u8) { ::write(REGISTER_ADDRESS, OC1PE_BIT_OFFSET, OC1PE_BIT_WIDTH, value as u32); }

	const OC1M_BIT_OFFSET: u8 = 4;
	const OC1M_BIT_WIDTH: u8 = 3;
	/// Output compare 1 mode (Width: 3, Offset: 4)
	pub fn get_oc1m() -> u8 { ::read(REGISTER_ADDRESS, OC1M_BIT_OFFSET, OC1M_BIT_WIDTH) as u8 }
	/// Output compare 1 mode (Width: 3, Offset: 4)
	pub fn set_oc1m(value: u8) { ::write(REGISTER_ADDRESS, OC1M_BIT_OFFSET, OC1M_BIT_WIDTH, value as u32); }

	const OC1CE_BIT_OFFSET: u8 = 7;
	const OC1CE_BIT_WIDTH: u8 = 1;
	/// Output compare 1 clear enable (Width: 1, Offset: 7)
	pub fn get_oc1ce() -> u8 { ::read(REGISTER_ADDRESS, OC1CE_BIT_OFFSET, OC1CE_BIT_WIDTH) as u8 }
	/// Output compare 1 clear enable (Width: 1, Offset: 7)
	pub fn set_oc1ce(value: u8) { ::write(REGISTER_ADDRESS, OC1CE_BIT_OFFSET, OC1CE_BIT_WIDTH, value as u32); }

	const CC2S_BIT_OFFSET: u8 = 8;
	const CC2S_BIT_WIDTH: u8 = 2;
	/// Capture/Compare 2 selection (Width: 2, Offset: 8)
	pub fn get_cc2s() -> u8 { ::read(REGISTER_ADDRESS, CC2S_BIT_OFFSET, CC2S_BIT_WIDTH) as u8 }
	/// Capture/Compare 2 selection (Width: 2, Offset: 8)
	pub fn set_cc2s(value: u8) { ::write(REGISTER_ADDRESS, CC2S_BIT_OFFSET, CC2S_BIT_WIDTH, value as u32); }

	const OC2FE_BIT_OFFSET: u8 = 10;
	const OC2FE_BIT_WIDTH: u8 = 1;
	/// Output compare 2 fast enable (Width: 1, Offset: 10)
	pub fn get_oc2fe() -> u8 { ::read(REGISTER_ADDRESS, OC2FE_BIT_OFFSET, OC2FE_BIT_WIDTH) as u8 }
	/// Output compare 2 fast enable (Width: 1, Offset: 10)
	pub fn set_oc2fe(value: u8) { ::write(REGISTER_ADDRESS, OC2FE_BIT_OFFSET, OC2FE_BIT_WIDTH, value as u32); }

	const OC2PE_BIT_OFFSET: u8 = 11;
	const OC2PE_BIT_WIDTH: u8 = 1;
	/// Output compare 2 preload enable (Width: 1, Offset: 11)
	pub fn get_oc2pe() -> u8 { ::read(REGISTER_ADDRESS, OC2PE_BIT_OFFSET, OC2PE_BIT_WIDTH) as u8 }
	/// Output compare 2 preload enable (Width: 1, Offset: 11)
	pub fn set_oc2pe(value: u8) { ::write(REGISTER_ADDRESS, OC2PE_BIT_OFFSET, OC2PE_BIT_WIDTH, value as u32); }

	const OC2M_BIT_OFFSET: u8 = 12;
	const OC2M_BIT_WIDTH: u8 = 3;
	/// Output compare 2 mode (Width: 3, Offset: 12)
	pub fn get_oc2m() -> u8 { ::read(REGISTER_ADDRESS, OC2M_BIT_OFFSET, OC2M_BIT_WIDTH) as u8 }
	/// Output compare 2 mode (Width: 3, Offset: 12)
	pub fn set_oc2m(value: u8) { ::write(REGISTER_ADDRESS, OC2M_BIT_OFFSET, OC2M_BIT_WIDTH, value as u32); }

	const OC2CE_BIT_OFFSET: u8 = 15;
	const OC2CE_BIT_WIDTH: u8 = 1;
	/// Output compare 2 clear enable (Width: 1, Offset: 15)
	pub fn get_oc2ce() -> u8 { ::read(REGISTER_ADDRESS, OC2CE_BIT_OFFSET, OC2CE_BIT_WIDTH) as u8 }
	/// Output compare 2 clear enable (Width: 1, Offset: 15)
	pub fn set_oc2ce(value: u8) { ::write(REGISTER_ADDRESS, OC2CE_BIT_OFFSET, OC2CE_BIT_WIDTH, value as u32); }

	const OC1M_3_BIT_OFFSET: u8 = 16;
	const OC1M_3_BIT_WIDTH: u8 = 1;
	/// Output compare 1 mode bit 3 (Width: 1, Offset: 16)
	pub fn get_oc1m_3() -> u8 { ::read(REGISTER_ADDRESS, OC1M_3_BIT_OFFSET, OC1M_3_BIT_WIDTH) as u8 }
	/// Output compare 1 mode bit 3 (Width: 1, Offset: 16)
	pub fn set_oc1m_3(value: u8) { ::write(REGISTER_ADDRESS, OC1M_3_BIT_OFFSET, OC1M_3_BIT_WIDTH, value as u32); }

	const OC2M_3_BIT_OFFSET: u8 = 24;
	const OC2M_3_BIT_WIDTH: u8 = 1;
	/// Output compare 2 mode bit 3 (Width: 1, Offset: 24)
	pub fn get_oc2m_3() -> u8 { ::read(REGISTER_ADDRESS, OC2M_3_BIT_OFFSET, OC2M_3_BIT_WIDTH) as u8 }
	/// Output compare 2 mode bit 3 (Width: 1, Offset: 24)
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

	const IC2PSC_BIT_OFFSET: u8 = 10;
	const IC2PSC_BIT_WIDTH: u8 = 2;
	/// Input capture 2 prescaler (Width: 2, Offset: 10)
	pub fn get_ic2psc() -> u8 { ::read(REGISTER_ADDRESS, IC2PSC_BIT_OFFSET, IC2PSC_BIT_WIDTH) as u8 }
	/// Input capture 2 prescaler (Width: 2, Offset: 10)
	pub fn set_ic2psc(value: u8) { ::write(REGISTER_ADDRESS, IC2PSC_BIT_OFFSET, IC2PSC_BIT_WIDTH, value as u32); }

	const CC2S_BIT_OFFSET: u8 = 8;
	const CC2S_BIT_WIDTH: u8 = 2;
	/// Capture/compare 2 selection (Width: 2, Offset: 8)
	pub fn get_cc2s() -> u8 { ::read(REGISTER_ADDRESS, CC2S_BIT_OFFSET, CC2S_BIT_WIDTH) as u8 }
	/// Capture/compare 2 selection (Width: 2, Offset: 8)
	pub fn set_cc2s(value: u8) { ::write(REGISTER_ADDRESS, CC2S_BIT_OFFSET, CC2S_BIT_WIDTH, value as u32); }

	const IC1F_BIT_OFFSET: u8 = 4;
	const IC1F_BIT_WIDTH: u8 = 4;
	/// Input capture 1 filter (Width: 4, Offset: 4)
	pub fn get_ic1f() -> u8 { ::read(REGISTER_ADDRESS, IC1F_BIT_OFFSET, IC1F_BIT_WIDTH) as u8 }
	/// Input capture 1 filter (Width: 4, Offset: 4)
	pub fn set_ic1f(value: u8) { ::write(REGISTER_ADDRESS, IC1F_BIT_OFFSET, IC1F_BIT_WIDTH, value as u32); }

	const IC1PSC_BIT_OFFSET: u8 = 2;
	const IC1PSC_BIT_WIDTH: u8 = 2;
	/// Input capture 1 prescaler (Width: 2, Offset: 2)
	pub fn get_ic1psc() -> u8 { ::read(REGISTER_ADDRESS, IC1PSC_BIT_OFFSET, IC1PSC_BIT_WIDTH) as u8 }
	/// Input capture 1 prescaler (Width: 2, Offset: 2)
	pub fn set_ic1psc(value: u8) { ::write(REGISTER_ADDRESS, IC1PSC_BIT_OFFSET, IC1PSC_BIT_WIDTH, value as u32); }

	const CC1S_BIT_OFFSET: u8 = 0;
	const CC1S_BIT_WIDTH: u8 = 2;
	/// Capture/Compare 1 selection (Width: 2, Offset: 0)
	pub fn get_cc1s() -> u8 { ::read(REGISTER_ADDRESS, CC1S_BIT_OFFSET, CC1S_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 selection (Width: 2, Offset: 0)
	pub fn set_cc1s(value: u8) { ::write(REGISTER_ADDRESS, CC1S_BIT_OFFSET, CC1S_BIT_WIDTH, value as u32); }
}
/// capture/compare mode register 2 (output mode)
/// Size: 0x20 bits
pub mod ccmr2_output {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CC3S_BIT_OFFSET: u8 = 0;
	const CC3S_BIT_WIDTH: u8 = 2;
	/// Capture/Compare 3 selection (Width: 2, Offset: 0)
	pub fn get_cc3s() -> u8 { ::read(REGISTER_ADDRESS, CC3S_BIT_OFFSET, CC3S_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 selection (Width: 2, Offset: 0)
	pub fn set_cc3s(value: u8) { ::write(REGISTER_ADDRESS, CC3S_BIT_OFFSET, CC3S_BIT_WIDTH, value as u32); }

	const OC3FE_BIT_OFFSET: u8 = 2;
	const OC3FE_BIT_WIDTH: u8 = 1;
	/// Output compare 3 fast enable (Width: 1, Offset: 2)
	pub fn get_oc3fe() -> u8 { ::read(REGISTER_ADDRESS, OC3FE_BIT_OFFSET, OC3FE_BIT_WIDTH) as u8 }
	/// Output compare 3 fast enable (Width: 1, Offset: 2)
	pub fn set_oc3fe(value: u8) { ::write(REGISTER_ADDRESS, OC3FE_BIT_OFFSET, OC3FE_BIT_WIDTH, value as u32); }

	const OC3PE_BIT_OFFSET: u8 = 3;
	const OC3PE_BIT_WIDTH: u8 = 1;
	/// Output compare 3 preload enable (Width: 1, Offset: 3)
	pub fn get_oc3pe() -> u8 { ::read(REGISTER_ADDRESS, OC3PE_BIT_OFFSET, OC3PE_BIT_WIDTH) as u8 }
	/// Output compare 3 preload enable (Width: 1, Offset: 3)
	pub fn set_oc3pe(value: u8) { ::write(REGISTER_ADDRESS, OC3PE_BIT_OFFSET, OC3PE_BIT_WIDTH, value as u32); }

	const OC3M_BIT_OFFSET: u8 = 4;
	const OC3M_BIT_WIDTH: u8 = 3;
	/// Output compare 3 mode (Width: 3, Offset: 4)
	pub fn get_oc3m() -> u8 { ::read(REGISTER_ADDRESS, OC3M_BIT_OFFSET, OC3M_BIT_WIDTH) as u8 }
	/// Output compare 3 mode (Width: 3, Offset: 4)
	pub fn set_oc3m(value: u8) { ::write(REGISTER_ADDRESS, OC3M_BIT_OFFSET, OC3M_BIT_WIDTH, value as u32); }

	const OC3CE_BIT_OFFSET: u8 = 7;
	const OC3CE_BIT_WIDTH: u8 = 1;
	/// Output compare 3 clear enable (Width: 1, Offset: 7)
	pub fn get_oc3ce() -> u8 { ::read(REGISTER_ADDRESS, OC3CE_BIT_OFFSET, OC3CE_BIT_WIDTH) as u8 }
	/// Output compare 3 clear enable (Width: 1, Offset: 7)
	pub fn set_oc3ce(value: u8) { ::write(REGISTER_ADDRESS, OC3CE_BIT_OFFSET, OC3CE_BIT_WIDTH, value as u32); }

	const CC4S_BIT_OFFSET: u8 = 8;
	const CC4S_BIT_WIDTH: u8 = 2;
	/// Capture/Compare 4 selection (Width: 2, Offset: 8)
	pub fn get_cc4s() -> u8 { ::read(REGISTER_ADDRESS, CC4S_BIT_OFFSET, CC4S_BIT_WIDTH) as u8 }
	/// Capture/Compare 4 selection (Width: 2, Offset: 8)
	pub fn set_cc4s(value: u8) { ::write(REGISTER_ADDRESS, CC4S_BIT_OFFSET, CC4S_BIT_WIDTH, value as u32); }

	const OC4FE_BIT_OFFSET: u8 = 10;
	const OC4FE_BIT_WIDTH: u8 = 1;
	/// Output compare 4 fast enable (Width: 1, Offset: 10)
	pub fn get_oc4fe() -> u8 { ::read(REGISTER_ADDRESS, OC4FE_BIT_OFFSET, OC4FE_BIT_WIDTH) as u8 }
	/// Output compare 4 fast enable (Width: 1, Offset: 10)
	pub fn set_oc4fe(value: u8) { ::write(REGISTER_ADDRESS, OC4FE_BIT_OFFSET, OC4FE_BIT_WIDTH, value as u32); }

	const OC4PE_BIT_OFFSET: u8 = 11;
	const OC4PE_BIT_WIDTH: u8 = 1;
	/// Output compare 4 preload enable (Width: 1, Offset: 11)
	pub fn get_oc4pe() -> u8 { ::read(REGISTER_ADDRESS, OC4PE_BIT_OFFSET, OC4PE_BIT_WIDTH) as u8 }
	/// Output compare 4 preload enable (Width: 1, Offset: 11)
	pub fn set_oc4pe(value: u8) { ::write(REGISTER_ADDRESS, OC4PE_BIT_OFFSET, OC4PE_BIT_WIDTH, value as u32); }

	const OC4M_BIT_OFFSET: u8 = 12;
	const OC4M_BIT_WIDTH: u8 = 3;
	/// Output compare 4 mode (Width: 3, Offset: 12)
	pub fn get_oc4m() -> u8 { ::read(REGISTER_ADDRESS, OC4M_BIT_OFFSET, OC4M_BIT_WIDTH) as u8 }
	/// Output compare 4 mode (Width: 3, Offset: 12)
	pub fn set_oc4m(value: u8) { ::write(REGISTER_ADDRESS, OC4M_BIT_OFFSET, OC4M_BIT_WIDTH, value as u32); }

	const O24CE_BIT_OFFSET: u8 = 15;
	const O24CE_BIT_WIDTH: u8 = 1;
	/// Output compare 4 clear enable (Width: 1, Offset: 15)
	pub fn get_o24ce() -> u8 { ::read(REGISTER_ADDRESS, O24CE_BIT_OFFSET, O24CE_BIT_WIDTH) as u8 }
	/// Output compare 4 clear enable (Width: 1, Offset: 15)
	pub fn set_o24ce(value: u8) { ::write(REGISTER_ADDRESS, O24CE_BIT_OFFSET, O24CE_BIT_WIDTH, value as u32); }

	const OC3M_3_BIT_OFFSET: u8 = 16;
	const OC3M_3_BIT_WIDTH: u8 = 1;
	/// Output compare 3 mode bit3 (Width: 1, Offset: 16)
	pub fn get_oc3m_3() -> u8 { ::read(REGISTER_ADDRESS, OC3M_3_BIT_OFFSET, OC3M_3_BIT_WIDTH) as u8 }
	/// Output compare 3 mode bit3 (Width: 1, Offset: 16)
	pub fn set_oc3m_3(value: u8) { ::write(REGISTER_ADDRESS, OC3M_3_BIT_OFFSET, OC3M_3_BIT_WIDTH, value as u32); }

	const OC4M_3_BIT_OFFSET: u8 = 24;
	const OC4M_3_BIT_WIDTH: u8 = 1;
	/// Output compare 4 mode bit3 (Width: 1, Offset: 24)
	pub fn get_oc4m_3() -> u8 { ::read(REGISTER_ADDRESS, OC4M_3_BIT_OFFSET, OC4M_3_BIT_WIDTH) as u8 }
	/// Output compare 4 mode bit3 (Width: 1, Offset: 24)
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
	/// Capture/Compare 3 selection (Width: 2, Offset: 0)
	pub fn get_cc3s() -> u8 { ::read(REGISTER_ADDRESS, CC3S_BIT_OFFSET, CC3S_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 selection (Width: 2, Offset: 0)
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
	/// Capture/Compare 3 output Polarity (Width: 1, Offset: 15)
	pub fn get_cc4np() -> u8 { ::read(REGISTER_ADDRESS, CC4NP_BIT_OFFSET, CC4NP_BIT_WIDTH) as u8 }
	/// Capture/Compare 3 output Polarity (Width: 1, Offset: 15)
	pub fn set_cc4np(value: u8) { ::write(REGISTER_ADDRESS, CC4NP_BIT_OFFSET, CC4NP_BIT_WIDTH, value as u32); }
}
/// counter
/// Size: 0x20 bits
pub mod cnt {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x24;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CNTL_BIT_OFFSET: u8 = 0;
	const CNTL_BIT_WIDTH: u8 = 16;
	/// Low counter value (Width: 16, Offset: 0)
	pub fn get_cntl() -> u16 { ::read(REGISTER_ADDRESS, CNTL_BIT_OFFSET, CNTL_BIT_WIDTH) as u16 }
	/// Low counter value (Width: 16, Offset: 0)
	pub fn set_cntl(value: u16) { ::write(REGISTER_ADDRESS, CNTL_BIT_OFFSET, CNTL_BIT_WIDTH, value as u32); }

	const CNTH_BIT_OFFSET: u8 = 16;
	const CNTH_BIT_WIDTH: u8 = 15;
	/// High counter value (Width: 15, Offset: 16)
	pub fn get_cnth() -> u16 { ::read(REGISTER_ADDRESS, CNTH_BIT_OFFSET, CNTH_BIT_WIDTH) as u16 }
	/// High counter value (Width: 15, Offset: 16)
	pub fn set_cnth(value: u16) { ::write(REGISTER_ADDRESS, CNTH_BIT_OFFSET, CNTH_BIT_WIDTH, value as u32); }

	const CNT_or_UIFCPY_BIT_OFFSET: u8 = 31;
	const CNT_or_UIFCPY_BIT_WIDTH: u8 = 1;
	/// if IUFREMAP=0 than CNT with read write access else UIFCPY with read only access (Width: 1, Offset: 31)
	pub fn get_cnt_or_uifcpy() -> u8 { ::read(REGISTER_ADDRESS, CNT_or_UIFCPY_BIT_OFFSET, CNT_or_UIFCPY_BIT_WIDTH) as u8 }
	/// if IUFREMAP=0 than CNT with read write access else UIFCPY with read only access (Width: 1, Offset: 31)
	pub fn set_cnt_or_uifcpy(value: u8) { ::write(REGISTER_ADDRESS, CNT_or_UIFCPY_BIT_OFFSET, CNT_or_UIFCPY_BIT_WIDTH, value as u32); }
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

	const ARRL_BIT_OFFSET: u8 = 0;
	const ARRL_BIT_WIDTH: u8 = 16;
	/// Low Auto-reload value (Width: 16, Offset: 0)
	pub fn get_arrl() -> u16 { ::read(REGISTER_ADDRESS, ARRL_BIT_OFFSET, ARRL_BIT_WIDTH) as u16 }
	/// Low Auto-reload value (Width: 16, Offset: 0)
	pub fn set_arrl(value: u16) { ::write(REGISTER_ADDRESS, ARRL_BIT_OFFSET, ARRL_BIT_WIDTH, value as u32); }

	const ARRH_BIT_OFFSET: u8 = 16;
	const ARRH_BIT_WIDTH: u8 = 16;
	/// High Auto-reload value (Width: 16, Offset: 16)
	pub fn get_arrh() -> u16 { ::read(REGISTER_ADDRESS, ARRH_BIT_OFFSET, ARRH_BIT_WIDTH) as u16 }
	/// High Auto-reload value (Width: 16, Offset: 16)
	pub fn set_arrh(value: u16) { ::write(REGISTER_ADDRESS, ARRH_BIT_OFFSET, ARRH_BIT_WIDTH, value as u32); }
}
/// capture/compare register 1
/// Size: 0x20 bits
pub mod ccr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x34;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CCR1L_BIT_OFFSET: u8 = 0;
	const CCR1L_BIT_WIDTH: u8 = 16;
	/// Low Capture/Compare 1 value (Width: 16, Offset: 0)
	pub fn get_ccr1l() -> u16 { ::read(REGISTER_ADDRESS, CCR1L_BIT_OFFSET, CCR1L_BIT_WIDTH) as u16 }
	/// Low Capture/Compare 1 value (Width: 16, Offset: 0)
	pub fn set_ccr1l(value: u16) { ::write(REGISTER_ADDRESS, CCR1L_BIT_OFFSET, CCR1L_BIT_WIDTH, value as u32); }

	const CCR1H_BIT_OFFSET: u8 = 16;
	const CCR1H_BIT_WIDTH: u8 = 16;
	/// High Capture/Compare 1 value (on TIM2) (Width: 16, Offset: 16)
	pub fn get_ccr1h() -> u16 { ::read(REGISTER_ADDRESS, CCR1H_BIT_OFFSET, CCR1H_BIT_WIDTH) as u16 }
	/// High Capture/Compare 1 value (on TIM2) (Width: 16, Offset: 16)
	pub fn set_ccr1h(value: u16) { ::write(REGISTER_ADDRESS, CCR1H_BIT_OFFSET, CCR1H_BIT_WIDTH, value as u32); }
}
/// capture/compare register 2
/// Size: 0x20 bits
pub mod ccr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x38;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CCR2L_BIT_OFFSET: u8 = 0;
	const CCR2L_BIT_WIDTH: u8 = 16;
	/// Low Capture/Compare 2 value (Width: 16, Offset: 0)
	pub fn get_ccr2l() -> u16 { ::read(REGISTER_ADDRESS, CCR2L_BIT_OFFSET, CCR2L_BIT_WIDTH) as u16 }
	/// Low Capture/Compare 2 value (Width: 16, Offset: 0)
	pub fn set_ccr2l(value: u16) { ::write(REGISTER_ADDRESS, CCR2L_BIT_OFFSET, CCR2L_BIT_WIDTH, value as u32); }

	const CCR2H_BIT_OFFSET: u8 = 16;
	const CCR2H_BIT_WIDTH: u8 = 16;
	/// High Capture/Compare 2 value (on TIM2) (Width: 16, Offset: 16)
	pub fn get_ccr2h() -> u16 { ::read(REGISTER_ADDRESS, CCR2H_BIT_OFFSET, CCR2H_BIT_WIDTH) as u16 }
	/// High Capture/Compare 2 value (on TIM2) (Width: 16, Offset: 16)
	pub fn set_ccr2h(value: u16) { ::write(REGISTER_ADDRESS, CCR2H_BIT_OFFSET, CCR2H_BIT_WIDTH, value as u32); }
}
/// capture/compare register 3
/// Size: 0x20 bits
pub mod ccr3 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x3C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CCR3L_BIT_OFFSET: u8 = 0;
	const CCR3L_BIT_WIDTH: u8 = 16;
	/// Low Capture/Compare value (Width: 16, Offset: 0)
	pub fn get_ccr3l() -> u16 { ::read(REGISTER_ADDRESS, CCR3L_BIT_OFFSET, CCR3L_BIT_WIDTH) as u16 }
	/// Low Capture/Compare value (Width: 16, Offset: 0)
	pub fn set_ccr3l(value: u16) { ::write(REGISTER_ADDRESS, CCR3L_BIT_OFFSET, CCR3L_BIT_WIDTH, value as u32); }

	const CCR3H_BIT_OFFSET: u8 = 16;
	const CCR3H_BIT_WIDTH: u8 = 16;
	/// High Capture/Compare value (on TIM2) (Width: 16, Offset: 16)
	pub fn get_ccr3h() -> u16 { ::read(REGISTER_ADDRESS, CCR3H_BIT_OFFSET, CCR3H_BIT_WIDTH) as u16 }
	/// High Capture/Compare value (on TIM2) (Width: 16, Offset: 16)
	pub fn set_ccr3h(value: u16) { ::write(REGISTER_ADDRESS, CCR3H_BIT_OFFSET, CCR3H_BIT_WIDTH, value as u32); }
}
/// capture/compare register 4
/// Size: 0x20 bits
pub mod ccr4 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x40;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CCR4L_BIT_OFFSET: u8 = 0;
	const CCR4L_BIT_WIDTH: u8 = 16;
	/// Low Capture/Compare value (Width: 16, Offset: 0)
	pub fn get_ccr4l() -> u16 { ::read(REGISTER_ADDRESS, CCR4L_BIT_OFFSET, CCR4L_BIT_WIDTH) as u16 }
	/// Low Capture/Compare value (Width: 16, Offset: 0)
	pub fn set_ccr4l(value: u16) { ::write(REGISTER_ADDRESS, CCR4L_BIT_OFFSET, CCR4L_BIT_WIDTH, value as u32); }

	const CCR4H_BIT_OFFSET: u8 = 16;
	const CCR4H_BIT_WIDTH: u8 = 16;
	/// High Capture/Compare value (on TIM2) (Width: 16, Offset: 16)
	pub fn get_ccr4h() -> u16 { ::read(REGISTER_ADDRESS, CCR4H_BIT_OFFSET, CCR4H_BIT_WIDTH) as u16 }
	/// High Capture/Compare value (on TIM2) (Width: 16, Offset: 16)
	pub fn set_ccr4h(value: u16) { ::write(REGISTER_ADDRESS, CCR4H_BIT_OFFSET, CCR4H_BIT_WIDTH, value as u32); }
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
/// TIM2 global interrupt
pub const INTERRUPT_TIM2: u32 = 28;

/// TIM3 global interrupt
pub const INTERRUPT_TIM3: u32 = 29;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" derivedFrom="TIM2">
  <name>TIM3</name>
  <description>General purpose timer</description>
  <groupName>TIMs</groupName>
  <baseAddress>0x40000400</baseAddress>
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
          <name>TI1S</name>
          <description>TI1 selection</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MMS</name>
          <description>Master mode selection</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>CCDS</name>
          <description>Capture/compare DMA
              selection</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
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
          <name>SMS_3</name>
          <description>Slave mode selection bit3</description>
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
          <name>TIE</name>
          <description>Trigger interrupt enable</description>
          <bitOffset>6</bitOffset>
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
          <name>CC4OF</name>
          <description>Capture/Compare 4 overcapture
              flag</description>
          <bitOffset>12</bitOffset>
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
          <name>CC2OF</name>
          <description>Capture/compare 2 overcapture
              flag</description>
          <bitOffset>10</bitOffset>
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
          <name>TIF</name>
          <description>Trigger interrupt flag</description>
          <bitOffset>6</bitOffset>
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
          <name>CC3IF</name>
          <description>Capture/Compare 3 interrupt
              flag</description>
          <bitOffset>3</bitOffset>
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
          <name>CC1IF</name>
          <description>Capture/compare 1 interrupt
              flag</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>UIF</name>
          <description>Update interrupt flag</description>
          <bitOffset>0</bitOffset>
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
          <name>TG</name>
          <description>Trigger generation</description>
          <bitOffset>6</bitOffset>
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
          <name>CC3G</name>
          <description>Capture/compare 3
              generation</description>
          <bitOffset>3</bitOffset>
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
          <name>CC1G</name>
          <description>Capture/compare 1
              generation</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>UG</name>
          <description>Update generation</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CCMR1_Output</name>
      <displayName>CCMR1_Output</displayName>
      <description>capture/compare mode register 1 (output
          mode)</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CC1S</name>
          <description>Capture/Compare 1
              selection</description>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OC1FE</name>
          <description>Output compare 1 fast
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC1PE</name>
          <description>Output compare 1 preload
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC1M</name>
          <description>Output compare 1 mode</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>OC1CE</name>
          <description>Output compare 1 clear
              enable</description>
          <bitOffset>7</bitOffset>
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
          <name>OC2FE</name>
          <description>Output compare 2 fast
              enable</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC2PE</name>
          <description>Output compare 2 preload
              enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC2M</name>
          <description>Output compare 2 mode</description>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>OC2CE</name>
          <description>Output compare 2 clear
              enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC1M_3</name>
          <description>Output compare 1 mode bit
              3</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC2M_3</name>
          <description>Output compare 2 mode bit
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
          <name>IC2PSC</name>
          <description>Input capture 2 prescaler</description>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>CC2S</name>
          <description>Capture/compare 2
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
          <name>IC1PSC</name>
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
      <description>capture/compare mode register 2 (output
          mode)</description>
      <addressOffset>0x1C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CC3S</name>
          <description>Capture/Compare 3
              selection</description>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>OC3FE</name>
          <description>Output compare 3 fast
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC3PE</name>
          <description>Output compare 3 preload
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC3M</name>
          <description>Output compare 3 mode</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>OC3CE</name>
          <description>Output compare 3 clear
              enable</description>
          <bitOffset>7</bitOffset>
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
          <name>OC4FE</name>
          <description>Output compare 4 fast
              enable</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC4PE</name>
          <description>Output compare 4 preload
              enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC4M</name>
          <description>Output compare 4 mode</description>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>O24CE</name>
          <description>Output compare 4 clear
              enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC3M_3</name>
          <description>Output compare 3 mode bit3</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC4M_3</name>
          <description>Output compare 4 mode bit3</description>
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
          <description>Capture/Compare 3
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
          <description>Capture/Compare 3 output
              Polarity</description>
          <bitOffset>15</bitOffset>
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
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CNTL</name>
          <description>Low counter value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
        <field>
          <name>CNTH</name>
          <description>High counter value</description>
          <bitOffset>16</bitOffset>
          <bitWidth>15</bitWidth>
        </field>
        <field>
          <name>CNT_or_UIFCPY</name>
          <description>if IUFREMAP=0 than CNT with read write
              access else UIFCPY with read only
              access</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
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
          <name>ARRL</name>
          <description>Low Auto-reload value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
        <field>
          <name>ARRH</name>
          <description>High Auto-reload value</description>
          <bitOffset>16</bitOffset>
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
          <name>CCR1L</name>
          <description>Low Capture/Compare 1
              value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
        <field>
          <name>CCR1H</name>
          <description>High Capture/Compare 1 value (on
              TIM2)</description>
          <bitOffset>16</bitOffset>
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
          <name>CCR2L</name>
          <description>Low Capture/Compare 2
              value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
        <field>
          <name>CCR2H</name>
          <description>High Capture/Compare 2 value (on
              TIM2)</description>
          <bitOffset>16</bitOffset>
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
          <name>CCR3L</name>
          <description>Low Capture/Compare value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
        <field>
          <name>CCR3H</name>
          <description>High Capture/Compare value (on
              TIM2)</description>
          <bitOffset>16</bitOffset>
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
          <name>CCR4L</name>
          <description>Low Capture/Compare value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
        <field>
          <name>CCR4H</name>
          <description>High Capture/Compare value (on
              TIM2)</description>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
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
      <resetValue>0x0000</resetValue>
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
      <resetValue>0x0000</resetValue>
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
  </registers>
  <interrupt>
    <name>TIM2</name>
    <description>TIM2 global interrupt</description>
    <value>28</value>
  </interrupt>
  <interrupt>
    <name>TIM3</name>
    <description>TIM3 global interrupt</description>
    <value>29</value>
  </interrupt>
</peripheral>*/
