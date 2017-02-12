/// MOD TIM16
/// General-purpose-timers
const BASE_ADDRESS: u32 = 0x40014400;
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

	const OIS1N_BIT_OFFSET: u8 = 9;
	const OIS1N_BIT_WIDTH: u8 = 1;
	/// Output Idle state 1 (Width: 1, Offset: 9)
	pub fn get_ois1n() -> u8 { ::read(REGISTER_ADDRESS, OIS1N_BIT_OFFSET, OIS1N_BIT_WIDTH) as u8 }
	/// Output Idle state 1 (Width: 1, Offset: 9)
	pub fn set_ois1n(value: u8) { ::write(REGISTER_ADDRESS, OIS1N_BIT_OFFSET, OIS1N_BIT_WIDTH, value as u32); }

	const OIS1_BIT_OFFSET: u8 = 8;
	const OIS1_BIT_WIDTH: u8 = 1;
	/// Output Idle state 1 (Width: 1, Offset: 8)
	pub fn get_ois1() -> u8 { ::read(REGISTER_ADDRESS, OIS1_BIT_OFFSET, OIS1_BIT_WIDTH) as u8 }
	/// Output Idle state 1 (Width: 1, Offset: 8)
	pub fn set_ois1(value: u8) { ::write(REGISTER_ADDRESS, OIS1_BIT_OFFSET, OIS1_BIT_WIDTH, value as u32); }

	const CCDS_BIT_OFFSET: u8 = 3;
	const CCDS_BIT_WIDTH: u8 = 1;
	/// Capture/compare DMA selection (Width: 1, Offset: 3)
	pub fn get_ccds() -> u8 { ::read(REGISTER_ADDRESS, CCDS_BIT_OFFSET, CCDS_BIT_WIDTH) as u8 }
	/// Capture/compare DMA selection (Width: 1, Offset: 3)
	pub fn set_ccds(value: u8) { ::write(REGISTER_ADDRESS, CCDS_BIT_OFFSET, CCDS_BIT_WIDTH, value as u32); }

	const CCUS_BIT_OFFSET: u8 = 2;
	const CCUS_BIT_WIDTH: u8 = 1;
	/// Capture/compare control update selection (Width: 1, Offset: 2)
	pub fn get_ccus() -> u8 { ::read(REGISTER_ADDRESS, CCUS_BIT_OFFSET, CCUS_BIT_WIDTH) as u8 }
	/// Capture/compare control update selection (Width: 1, Offset: 2)
	pub fn set_ccus(value: u8) { ::write(REGISTER_ADDRESS, CCUS_BIT_OFFSET, CCUS_BIT_WIDTH, value as u32); }

	const CCPC_BIT_OFFSET: u8 = 0;
	const CCPC_BIT_WIDTH: u8 = 1;
	/// Capture/compare preloaded control (Width: 1, Offset: 0)
	pub fn get_ccpc() -> u8 { ::read(REGISTER_ADDRESS, CCPC_BIT_OFFSET, CCPC_BIT_WIDTH) as u8 }
	/// Capture/compare preloaded control (Width: 1, Offset: 0)
	pub fn set_ccpc(value: u8) { ::write(REGISTER_ADDRESS, CCPC_BIT_OFFSET, CCPC_BIT_WIDTH, value as u32); }
}
/// DMA/Interrupt enable register
/// Size: 0x20 bits
pub mod dier {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const UIE_BIT_OFFSET: u8 = 0;
	const UIE_BIT_WIDTH: u8 = 1;
	/// Update interrupt enable (Width: 1, Offset: 0)
	pub fn get_uie() -> u8 { ::read(REGISTER_ADDRESS, UIE_BIT_OFFSET, UIE_BIT_WIDTH) as u8 }
	/// Update interrupt enable (Width: 1, Offset: 0)
	pub fn set_uie(value: u8) { ::write(REGISTER_ADDRESS, UIE_BIT_OFFSET, UIE_BIT_WIDTH, value as u32); }

	const CC1IE_BIT_OFFSET: u8 = 1;
	const CC1IE_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 interrupt enable (Width: 1, Offset: 1)
	pub fn get_cc1ie() -> u8 { ::read(REGISTER_ADDRESS, CC1IE_BIT_OFFSET, CC1IE_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 interrupt enable (Width: 1, Offset: 1)
	pub fn set_cc1ie(value: u8) { ::write(REGISTER_ADDRESS, CC1IE_BIT_OFFSET, CC1IE_BIT_WIDTH, value as u32); }

	const COMIE_BIT_OFFSET: u8 = 5;
	const COMIE_BIT_WIDTH: u8 = 1;
	/// COM interrupt enable (Width: 1, Offset: 5)
	pub fn get_comie() -> u8 { ::read(REGISTER_ADDRESS, COMIE_BIT_OFFSET, COMIE_BIT_WIDTH) as u8 }
	/// COM interrupt enable (Width: 1, Offset: 5)
	pub fn set_comie(value: u8) { ::write(REGISTER_ADDRESS, COMIE_BIT_OFFSET, COMIE_BIT_WIDTH, value as u32); }

	const TIE_BIT_OFFSET: u8 = 6;
	const TIE_BIT_WIDTH: u8 = 1;
	/// Trigger interrupt enable (Width: 1, Offset: 6)
	pub fn get_tie() -> u8 { ::read(REGISTER_ADDRESS, TIE_BIT_OFFSET, TIE_BIT_WIDTH) as u8 }
	/// Trigger interrupt enable (Width: 1, Offset: 6)
	pub fn set_tie(value: u8) { ::write(REGISTER_ADDRESS, TIE_BIT_OFFSET, TIE_BIT_WIDTH, value as u32); }

	const BIE_BIT_OFFSET: u8 = 7;
	const BIE_BIT_WIDTH: u8 = 1;
	/// Break interrupt enable (Width: 1, Offset: 7)
	pub fn get_bie() -> u8 { ::read(REGISTER_ADDRESS, BIE_BIT_OFFSET, BIE_BIT_WIDTH) as u8 }
	/// Break interrupt enable (Width: 1, Offset: 7)
	pub fn set_bie(value: u8) { ::write(REGISTER_ADDRESS, BIE_BIT_OFFSET, BIE_BIT_WIDTH, value as u32); }

	const UDE_BIT_OFFSET: u8 = 8;
	const UDE_BIT_WIDTH: u8 = 1;
	/// Update DMA request enable (Width: 1, Offset: 8)
	pub fn get_ude() -> u8 { ::read(REGISTER_ADDRESS, UDE_BIT_OFFSET, UDE_BIT_WIDTH) as u8 }
	/// Update DMA request enable (Width: 1, Offset: 8)
	pub fn set_ude(value: u8) { ::write(REGISTER_ADDRESS, UDE_BIT_OFFSET, UDE_BIT_WIDTH, value as u32); }

	const CC1DE_BIT_OFFSET: u8 = 9;
	const CC1DE_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 DMA request enable (Width: 1, Offset: 9)
	pub fn get_cc1de() -> u8 { ::read(REGISTER_ADDRESS, CC1DE_BIT_OFFSET, CC1DE_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 DMA request enable (Width: 1, Offset: 9)
	pub fn set_cc1de(value: u8) { ::write(REGISTER_ADDRESS, CC1DE_BIT_OFFSET, CC1DE_BIT_WIDTH, value as u32); }

	const COMDE_BIT_OFFSET: u8 = 13;
	const COMDE_BIT_WIDTH: u8 = 1;
	/// COM DMA request enable (Width: 1, Offset: 13)
	pub fn get_comde() -> u8 { ::read(REGISTER_ADDRESS, COMDE_BIT_OFFSET, COMDE_BIT_WIDTH) as u8 }
	/// COM DMA request enable (Width: 1, Offset: 13)
	pub fn set_comde(value: u8) { ::write(REGISTER_ADDRESS, COMDE_BIT_OFFSET, COMDE_BIT_WIDTH, value as u32); }

	const TDE_BIT_OFFSET: u8 = 14;
	const TDE_BIT_WIDTH: u8 = 1;
	/// Trigger DMA request enable (Width: 1, Offset: 14)
	pub fn get_tde() -> u8 { ::read(REGISTER_ADDRESS, TDE_BIT_OFFSET, TDE_BIT_WIDTH) as u8 }
	/// Trigger DMA request enable (Width: 1, Offset: 14)
	pub fn set_tde(value: u8) { ::write(REGISTER_ADDRESS, TDE_BIT_OFFSET, TDE_BIT_WIDTH, value as u32); }
}
/// status register
/// Size: 0x20 bits
pub mod sr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CC1OF_BIT_OFFSET: u8 = 9;
	const CC1OF_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 overcapture flag (Width: 1, Offset: 9)
	pub fn get_cc1of() -> u8 { ::read(REGISTER_ADDRESS, CC1OF_BIT_OFFSET, CC1OF_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 overcapture flag (Width: 1, Offset: 9)
	pub fn set_cc1of(value: u8) { ::write(REGISTER_ADDRESS, CC1OF_BIT_OFFSET, CC1OF_BIT_WIDTH, value as u32); }

	const BIF_BIT_OFFSET: u8 = 7;
	const BIF_BIT_WIDTH: u8 = 1;
	/// Break interrupt flag (Width: 1, Offset: 7)
	pub fn get_bif() -> u8 { ::read(REGISTER_ADDRESS, BIF_BIT_OFFSET, BIF_BIT_WIDTH) as u8 }
	/// Break interrupt flag (Width: 1, Offset: 7)
	pub fn set_bif(value: u8) { ::write(REGISTER_ADDRESS, BIF_BIT_OFFSET, BIF_BIT_WIDTH, value as u32); }

	const TIF_BIT_OFFSET: u8 = 6;
	const TIF_BIT_WIDTH: u8 = 1;
	/// Trigger interrupt flag (Width: 1, Offset: 6)
	pub fn get_tif() -> u8 { ::read(REGISTER_ADDRESS, TIF_BIT_OFFSET, TIF_BIT_WIDTH) as u8 }
	/// Trigger interrupt flag (Width: 1, Offset: 6)
	pub fn set_tif(value: u8) { ::write(REGISTER_ADDRESS, TIF_BIT_OFFSET, TIF_BIT_WIDTH, value as u32); }

	const COMIF_BIT_OFFSET: u8 = 5;
	const COMIF_BIT_WIDTH: u8 = 1;
	/// COM interrupt flag (Width: 1, Offset: 5)
	pub fn get_comif() -> u8 { ::read(REGISTER_ADDRESS, COMIF_BIT_OFFSET, COMIF_BIT_WIDTH) as u8 }
	/// COM interrupt flag (Width: 1, Offset: 5)
	pub fn set_comif(value: u8) { ::write(REGISTER_ADDRESS, COMIF_BIT_OFFSET, COMIF_BIT_WIDTH, value as u32); }

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

	const BG_BIT_OFFSET: u8 = 7;
	const BG_BIT_WIDTH: u8 = 1;
	/// Break generation (Width: 1, Offset: 7)
	pub fn set_bg(value: u8) { ::write(REGISTER_ADDRESS, BG_BIT_OFFSET, BG_BIT_WIDTH, value as u32); }

	const TG_BIT_OFFSET: u8 = 6;
	const TG_BIT_WIDTH: u8 = 1;
	/// Trigger generation (Width: 1, Offset: 6)
	pub fn set_tg(value: u8) { ::write(REGISTER_ADDRESS, TG_BIT_OFFSET, TG_BIT_WIDTH, value as u32); }

	const COMG_BIT_OFFSET: u8 = 5;
	const COMG_BIT_WIDTH: u8 = 1;
	/// Capture/Compare control update generation (Width: 1, Offset: 5)
	pub fn set_comg(value: u8) { ::write(REGISTER_ADDRESS, COMG_BIT_OFFSET, COMG_BIT_WIDTH, value as u32); }

	const CC1G_BIT_OFFSET: u8 = 1;
	const CC1G_BIT_WIDTH: u8 = 1;
	/// Capture/compare 1 generation (Width: 1, Offset: 1)
	pub fn set_cc1g(value: u8) { ::write(REGISTER_ADDRESS, CC1G_BIT_OFFSET, CC1G_BIT_WIDTH, value as u32); }

	const UG_BIT_OFFSET: u8 = 0;
	const UG_BIT_WIDTH: u8 = 1;
	/// Update generation (Width: 1, Offset: 0)
	pub fn set_ug(value: u8) { ::write(REGISTER_ADDRESS, UG_BIT_OFFSET, UG_BIT_WIDTH, value as u32); }
}
/// capture/compare mode register (output mode)
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
	/// Output Compare 1 fast enable (Width: 1, Offset: 2)
	pub fn get_oc1fe() -> u8 { ::read(REGISTER_ADDRESS, OC1FE_BIT_OFFSET, OC1FE_BIT_WIDTH) as u8 }
	/// Output Compare 1 fast enable (Width: 1, Offset: 2)
	pub fn set_oc1fe(value: u8) { ::write(REGISTER_ADDRESS, OC1FE_BIT_OFFSET, OC1FE_BIT_WIDTH, value as u32); }

	const OC1PE_BIT_OFFSET: u8 = 3;
	const OC1PE_BIT_WIDTH: u8 = 1;
	/// Output Compare 1 preload enable (Width: 1, Offset: 3)
	pub fn get_oc1pe() -> u8 { ::read(REGISTER_ADDRESS, OC1PE_BIT_OFFSET, OC1PE_BIT_WIDTH) as u8 }
	/// Output Compare 1 preload enable (Width: 1, Offset: 3)
	pub fn set_oc1pe(value: u8) { ::write(REGISTER_ADDRESS, OC1PE_BIT_OFFSET, OC1PE_BIT_WIDTH, value as u32); }

	const OC1M_BIT_OFFSET: u8 = 4;
	const OC1M_BIT_WIDTH: u8 = 3;
	/// Output Compare 1 mode (Width: 3, Offset: 4)
	pub fn get_oc1m() -> u8 { ::read(REGISTER_ADDRESS, OC1M_BIT_OFFSET, OC1M_BIT_WIDTH) as u8 }
	/// Output Compare 1 mode (Width: 3, Offset: 4)
	pub fn set_oc1m(value: u8) { ::write(REGISTER_ADDRESS, OC1M_BIT_OFFSET, OC1M_BIT_WIDTH, value as u32); }

	const OC1M_3_BIT_OFFSET: u8 = 16;
	const OC1M_3_BIT_WIDTH: u8 = 1;
	/// Output Compare 1 mode (Width: 1, Offset: 16)
	pub fn get_oc1m_3() -> u8 { ::read(REGISTER_ADDRESS, OC1M_3_BIT_OFFSET, OC1M_3_BIT_WIDTH) as u8 }
	/// Output Compare 1 mode (Width: 1, Offset: 16)
	pub fn set_oc1m_3(value: u8) { ::write(REGISTER_ADDRESS, OC1M_3_BIT_OFFSET, OC1M_3_BIT_WIDTH, value as u32); }
}
/// capture/compare mode register 1 (input mode)
/// Size: 0x20 bits
pub mod ccmr1_input {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

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
/// capture/compare enable register
/// Size: 0x20 bits
pub mod ccer {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x20;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CC1NP_BIT_OFFSET: u8 = 3;
	const CC1NP_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 output Polarity (Width: 1, Offset: 3)
	pub fn get_cc1np() -> u8 { ::read(REGISTER_ADDRESS, CC1NP_BIT_OFFSET, CC1NP_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 output Polarity (Width: 1, Offset: 3)
	pub fn set_cc1np(value: u8) { ::write(REGISTER_ADDRESS, CC1NP_BIT_OFFSET, CC1NP_BIT_WIDTH, value as u32); }

	const CC1NE_BIT_OFFSET: u8 = 2;
	const CC1NE_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 complementary output enable (Width: 1, Offset: 2)
	pub fn get_cc1ne() -> u8 { ::read(REGISTER_ADDRESS, CC1NE_BIT_OFFSET, CC1NE_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 complementary output enable (Width: 1, Offset: 2)
	pub fn set_cc1ne(value: u8) { ::write(REGISTER_ADDRESS, CC1NE_BIT_OFFSET, CC1NE_BIT_WIDTH, value as u32); }

	const CC1P_BIT_OFFSET: u8 = 1;
	const CC1P_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 output Polarity (Width: 1, Offset: 1)
	pub fn get_cc1p() -> u8 { ::read(REGISTER_ADDRESS, CC1P_BIT_OFFSET, CC1P_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 output Polarity (Width: 1, Offset: 1)
	pub fn set_cc1p(value: u8) { ::write(REGISTER_ADDRESS, CC1P_BIT_OFFSET, CC1P_BIT_WIDTH, value as u32); }

	const CC1E_BIT_OFFSET: u8 = 0;
	const CC1E_BIT_WIDTH: u8 = 1;
	/// Capture/Compare 1 output enable (Width: 1, Offset: 0)
	pub fn get_cc1e() -> u8 { ::read(REGISTER_ADDRESS, CC1E_BIT_OFFSET, CC1E_BIT_WIDTH) as u8 }
	/// Capture/Compare 1 output enable (Width: 1, Offset: 0)
	pub fn set_cc1e(value: u8) { ::write(REGISTER_ADDRESS, CC1E_BIT_OFFSET, CC1E_BIT_WIDTH, value as u32); }
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
	/// UIF Copy (Width: 1, Offset: 31)
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
	const REP_BIT_WIDTH: u8 = 8;
	/// Repetition counter value (Width: 8, Offset: 0)
	pub fn get_rep() -> u8 { ::read(REGISTER_ADDRESS, REP_BIT_OFFSET, REP_BIT_WIDTH) as u8 }
	/// Repetition counter value (Width: 8, Offset: 0)
	pub fn set_rep(value: u8) { ::write(REGISTER_ADDRESS, REP_BIT_OFFSET, REP_BIT_WIDTH, value as u32); }
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
/// option register
/// Size: 0x20 bits
pub mod or {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x50;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;
}
/// TIM1 Update/TIM16 global interrupts
pub const INTERRUPT_TIM1_UP_TIM16: u32 = 25;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>TIM16</name>
  <description>General-purpose-timers</description>
  <groupName>TIMs</groupName>
  <baseAddress>0x40014400</baseAddress>
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
          <name>OIS1N</name>
          <description>Output Idle state 1</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OIS1</name>
          <description>Output Idle state 1</description>
          <bitOffset>8</bitOffset>
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
          <name>CCUS</name>
          <description>Capture/compare control update
              selection</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CCPC</name>
          <description>Capture/compare preloaded
              control</description>
          <bitOffset>0</bitOffset>
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
          <name>UIE</name>
          <description>Update interrupt enable</description>
          <bitOffset>0</bitOffset>
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
          <name>COMIE</name>
          <description>COM interrupt enable</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIE</name>
          <description>Trigger interrupt enable</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BIE</name>
          <description>Break interrupt enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>UDE</name>
          <description>Update DMA request enable</description>
          <bitOffset>8</bitOffset>
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
          <name>COMDE</name>
          <description>COM DMA request enable</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TDE</name>
          <description>Trigger DMA request enable</description>
          <bitOffset>14</bitOffset>
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
          <name>CC1OF</name>
          <description>Capture/Compare 1 overcapture
              flag</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BIF</name>
          <description>Break interrupt flag</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIF</name>
          <description>Trigger interrupt flag</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>COMIF</name>
          <description>COM interrupt flag</description>
          <bitOffset>5</bitOffset>
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
          <name>BG</name>
          <description>Break generation</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TG</name>
          <description>Trigger generation</description>
          <bitOffset>6</bitOffset>
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
      <description>capture/compare mode register (output
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
          <description>Output Compare 1 fast
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC1PE</name>
          <description>Output Compare 1 preload
              enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OC1M</name>
          <description>Output Compare 1 mode</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>OC1M_3</name>
          <description>Output Compare 1 mode</description>
          <bitOffset>16</bitOffset>
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
          <name>CC1NP</name>
          <description>Capture/Compare 1 output
              Polarity</description>
          <bitOffset>3</bitOffset>
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
          <name>CC1P</name>
          <description>Capture/Compare 1 output
              Polarity</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CC1E</name>
          <description>Capture/Compare 1 output
              enable</description>
          <bitOffset>0</bitOffset>
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
          <description>UIF Copy</description>
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
          <bitWidth>8</bitWidth>
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
      <name>BDTR</name>
      <displayName>BDTR</displayName>
      <description>break and dead-time register</description>
      <addressOffset>0x44</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
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
    <register>
      <name>OR</name>
      <displayName>OR</displayName>
      <description>option register</description>
      <addressOffset>0x50</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
    </register>
  </registers>
  <interrupt>
    <name>TIM1_UP_TIM16</name>
    <description>TIM1 Update/TIM16 global
        interrupts</description>
    <value>25</value>
  </interrupt>
</peripheral>*/
