/// MOD ADC1
/// Analog-to-Digital Converter
const BASE_ADDRESS: u32 = 0x50000000;
/// interrupt and status register
/// Size: 0x20 bits
pub mod isr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const JQOVF_BIT_OFFSET: u8 = 10;
	const JQOVF_BIT_WIDTH: u8 = 1;
	/// JQOVF (Width: 1, Offset: 10)
	pub fn get_jqovf() -> u8 { ::read(REGISTER_ADDRESS, JQOVF_BIT_OFFSET, JQOVF_BIT_WIDTH) as u8 }
	/// JQOVF (Width: 1, Offset: 10)
	pub fn set_jqovf(value: u8) { ::write(REGISTER_ADDRESS, JQOVF_BIT_OFFSET, JQOVF_BIT_WIDTH, value as u32); }

	const AWD3_BIT_OFFSET: u8 = 9;
	const AWD3_BIT_WIDTH: u8 = 1;
	/// AWD3 (Width: 1, Offset: 9)
	pub fn get_awd3() -> u8 { ::read(REGISTER_ADDRESS, AWD3_BIT_OFFSET, AWD3_BIT_WIDTH) as u8 }
	/// AWD3 (Width: 1, Offset: 9)
	pub fn set_awd3(value: u8) { ::write(REGISTER_ADDRESS, AWD3_BIT_OFFSET, AWD3_BIT_WIDTH, value as u32); }

	const AWD2_BIT_OFFSET: u8 = 8;
	const AWD2_BIT_WIDTH: u8 = 1;
	/// AWD2 (Width: 1, Offset: 8)
	pub fn get_awd2() -> u8 { ::read(REGISTER_ADDRESS, AWD2_BIT_OFFSET, AWD2_BIT_WIDTH) as u8 }
	/// AWD2 (Width: 1, Offset: 8)
	pub fn set_awd2(value: u8) { ::write(REGISTER_ADDRESS, AWD2_BIT_OFFSET, AWD2_BIT_WIDTH, value as u32); }

	const AWD1_BIT_OFFSET: u8 = 7;
	const AWD1_BIT_WIDTH: u8 = 1;
	/// AWD1 (Width: 1, Offset: 7)
	pub fn get_awd1() -> u8 { ::read(REGISTER_ADDRESS, AWD1_BIT_OFFSET, AWD1_BIT_WIDTH) as u8 }
	/// AWD1 (Width: 1, Offset: 7)
	pub fn set_awd1(value: u8) { ::write(REGISTER_ADDRESS, AWD1_BIT_OFFSET, AWD1_BIT_WIDTH, value as u32); }

	const JEOS_BIT_OFFSET: u8 = 6;
	const JEOS_BIT_WIDTH: u8 = 1;
	/// JEOS (Width: 1, Offset: 6)
	pub fn get_jeos() -> u8 { ::read(REGISTER_ADDRESS, JEOS_BIT_OFFSET, JEOS_BIT_WIDTH) as u8 }
	/// JEOS (Width: 1, Offset: 6)
	pub fn set_jeos(value: u8) { ::write(REGISTER_ADDRESS, JEOS_BIT_OFFSET, JEOS_BIT_WIDTH, value as u32); }

	const JEOC_BIT_OFFSET: u8 = 5;
	const JEOC_BIT_WIDTH: u8 = 1;
	/// JEOC (Width: 1, Offset: 5)
	pub fn get_jeoc() -> u8 { ::read(REGISTER_ADDRESS, JEOC_BIT_OFFSET, JEOC_BIT_WIDTH) as u8 }
	/// JEOC (Width: 1, Offset: 5)
	pub fn set_jeoc(value: u8) { ::write(REGISTER_ADDRESS, JEOC_BIT_OFFSET, JEOC_BIT_WIDTH, value as u32); }

	const OVR_BIT_OFFSET: u8 = 4;
	const OVR_BIT_WIDTH: u8 = 1;
	/// OVR (Width: 1, Offset: 4)
	pub fn get_ovr() -> u8 { ::read(REGISTER_ADDRESS, OVR_BIT_OFFSET, OVR_BIT_WIDTH) as u8 }
	/// OVR (Width: 1, Offset: 4)
	pub fn set_ovr(value: u8) { ::write(REGISTER_ADDRESS, OVR_BIT_OFFSET, OVR_BIT_WIDTH, value as u32); }

	const EOS_BIT_OFFSET: u8 = 3;
	const EOS_BIT_WIDTH: u8 = 1;
	/// EOS (Width: 1, Offset: 3)
	pub fn get_eos() -> u8 { ::read(REGISTER_ADDRESS, EOS_BIT_OFFSET, EOS_BIT_WIDTH) as u8 }
	/// EOS (Width: 1, Offset: 3)
	pub fn set_eos(value: u8) { ::write(REGISTER_ADDRESS, EOS_BIT_OFFSET, EOS_BIT_WIDTH, value as u32); }

	const EOC_BIT_OFFSET: u8 = 2;
	const EOC_BIT_WIDTH: u8 = 1;
	/// EOC (Width: 1, Offset: 2)
	pub fn get_eoc() -> u8 { ::read(REGISTER_ADDRESS, EOC_BIT_OFFSET, EOC_BIT_WIDTH) as u8 }
	/// EOC (Width: 1, Offset: 2)
	pub fn set_eoc(value: u8) { ::write(REGISTER_ADDRESS, EOC_BIT_OFFSET, EOC_BIT_WIDTH, value as u32); }

	const EOSMP_BIT_OFFSET: u8 = 1;
	const EOSMP_BIT_WIDTH: u8 = 1;
	/// EOSMP (Width: 1, Offset: 1)
	pub fn get_eosmp() -> u8 { ::read(REGISTER_ADDRESS, EOSMP_BIT_OFFSET, EOSMP_BIT_WIDTH) as u8 }
	/// EOSMP (Width: 1, Offset: 1)
	pub fn set_eosmp(value: u8) { ::write(REGISTER_ADDRESS, EOSMP_BIT_OFFSET, EOSMP_BIT_WIDTH, value as u32); }

	const ADRDY_BIT_OFFSET: u8 = 0;
	const ADRDY_BIT_WIDTH: u8 = 1;
	/// ADRDY (Width: 1, Offset: 0)
	pub fn get_adrdy() -> u8 { ::read(REGISTER_ADDRESS, ADRDY_BIT_OFFSET, ADRDY_BIT_WIDTH) as u8 }
	/// ADRDY (Width: 1, Offset: 0)
	pub fn set_adrdy(value: u8) { ::write(REGISTER_ADDRESS, ADRDY_BIT_OFFSET, ADRDY_BIT_WIDTH, value as u32); }
}
/// interrupt enable register
/// Size: 0x20 bits
pub mod ier {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const JQOVFIE_BIT_OFFSET: u8 = 10;
	const JQOVFIE_BIT_WIDTH: u8 = 1;
	/// JQOVFIE (Width: 1, Offset: 10)
	pub fn get_jqovfie() -> u8 { ::read(REGISTER_ADDRESS, JQOVFIE_BIT_OFFSET, JQOVFIE_BIT_WIDTH) as u8 }
	/// JQOVFIE (Width: 1, Offset: 10)
	pub fn set_jqovfie(value: u8) { ::write(REGISTER_ADDRESS, JQOVFIE_BIT_OFFSET, JQOVFIE_BIT_WIDTH, value as u32); }

	const AWD3IE_BIT_OFFSET: u8 = 9;
	const AWD3IE_BIT_WIDTH: u8 = 1;
	/// AWD3IE (Width: 1, Offset: 9)
	pub fn get_awd3ie() -> u8 { ::read(REGISTER_ADDRESS, AWD3IE_BIT_OFFSET, AWD3IE_BIT_WIDTH) as u8 }
	/// AWD3IE (Width: 1, Offset: 9)
	pub fn set_awd3ie(value: u8) { ::write(REGISTER_ADDRESS, AWD3IE_BIT_OFFSET, AWD3IE_BIT_WIDTH, value as u32); }

	const AWD2IE_BIT_OFFSET: u8 = 8;
	const AWD2IE_BIT_WIDTH: u8 = 1;
	/// AWD2IE (Width: 1, Offset: 8)
	pub fn get_awd2ie() -> u8 { ::read(REGISTER_ADDRESS, AWD2IE_BIT_OFFSET, AWD2IE_BIT_WIDTH) as u8 }
	/// AWD2IE (Width: 1, Offset: 8)
	pub fn set_awd2ie(value: u8) { ::write(REGISTER_ADDRESS, AWD2IE_BIT_OFFSET, AWD2IE_BIT_WIDTH, value as u32); }

	const AWD1IE_BIT_OFFSET: u8 = 7;
	const AWD1IE_BIT_WIDTH: u8 = 1;
	/// AWD1IE (Width: 1, Offset: 7)
	pub fn get_awd1ie() -> u8 { ::read(REGISTER_ADDRESS, AWD1IE_BIT_OFFSET, AWD1IE_BIT_WIDTH) as u8 }
	/// AWD1IE (Width: 1, Offset: 7)
	pub fn set_awd1ie(value: u8) { ::write(REGISTER_ADDRESS, AWD1IE_BIT_OFFSET, AWD1IE_BIT_WIDTH, value as u32); }

	const JEOSIE_BIT_OFFSET: u8 = 6;
	const JEOSIE_BIT_WIDTH: u8 = 1;
	/// JEOSIE (Width: 1, Offset: 6)
	pub fn get_jeosie() -> u8 { ::read(REGISTER_ADDRESS, JEOSIE_BIT_OFFSET, JEOSIE_BIT_WIDTH) as u8 }
	/// JEOSIE (Width: 1, Offset: 6)
	pub fn set_jeosie(value: u8) { ::write(REGISTER_ADDRESS, JEOSIE_BIT_OFFSET, JEOSIE_BIT_WIDTH, value as u32); }

	const JEOCIE_BIT_OFFSET: u8 = 5;
	const JEOCIE_BIT_WIDTH: u8 = 1;
	/// JEOCIE (Width: 1, Offset: 5)
	pub fn get_jeocie() -> u8 { ::read(REGISTER_ADDRESS, JEOCIE_BIT_OFFSET, JEOCIE_BIT_WIDTH) as u8 }
	/// JEOCIE (Width: 1, Offset: 5)
	pub fn set_jeocie(value: u8) { ::write(REGISTER_ADDRESS, JEOCIE_BIT_OFFSET, JEOCIE_BIT_WIDTH, value as u32); }

	const OVRIE_BIT_OFFSET: u8 = 4;
	const OVRIE_BIT_WIDTH: u8 = 1;
	/// OVRIE (Width: 1, Offset: 4)
	pub fn get_ovrie() -> u8 { ::read(REGISTER_ADDRESS, OVRIE_BIT_OFFSET, OVRIE_BIT_WIDTH) as u8 }
	/// OVRIE (Width: 1, Offset: 4)
	pub fn set_ovrie(value: u8) { ::write(REGISTER_ADDRESS, OVRIE_BIT_OFFSET, OVRIE_BIT_WIDTH, value as u32); }

	const EOSIE_BIT_OFFSET: u8 = 3;
	const EOSIE_BIT_WIDTH: u8 = 1;
	/// EOSIE (Width: 1, Offset: 3)
	pub fn get_eosie() -> u8 { ::read(REGISTER_ADDRESS, EOSIE_BIT_OFFSET, EOSIE_BIT_WIDTH) as u8 }
	/// EOSIE (Width: 1, Offset: 3)
	pub fn set_eosie(value: u8) { ::write(REGISTER_ADDRESS, EOSIE_BIT_OFFSET, EOSIE_BIT_WIDTH, value as u32); }

	const EOCIE_BIT_OFFSET: u8 = 2;
	const EOCIE_BIT_WIDTH: u8 = 1;
	/// EOCIE (Width: 1, Offset: 2)
	pub fn get_eocie() -> u8 { ::read(REGISTER_ADDRESS, EOCIE_BIT_OFFSET, EOCIE_BIT_WIDTH) as u8 }
	/// EOCIE (Width: 1, Offset: 2)
	pub fn set_eocie(value: u8) { ::write(REGISTER_ADDRESS, EOCIE_BIT_OFFSET, EOCIE_BIT_WIDTH, value as u32); }

	const EOSMPIE_BIT_OFFSET: u8 = 1;
	const EOSMPIE_BIT_WIDTH: u8 = 1;
	/// EOSMPIE (Width: 1, Offset: 1)
	pub fn get_eosmpie() -> u8 { ::read(REGISTER_ADDRESS, EOSMPIE_BIT_OFFSET, EOSMPIE_BIT_WIDTH) as u8 }
	/// EOSMPIE (Width: 1, Offset: 1)
	pub fn set_eosmpie(value: u8) { ::write(REGISTER_ADDRESS, EOSMPIE_BIT_OFFSET, EOSMPIE_BIT_WIDTH, value as u32); }

	const ADRDYIE_BIT_OFFSET: u8 = 0;
	const ADRDYIE_BIT_WIDTH: u8 = 1;
	/// ADRDYIE (Width: 1, Offset: 0)
	pub fn get_adrdyie() -> u8 { ::read(REGISTER_ADDRESS, ADRDYIE_BIT_OFFSET, ADRDYIE_BIT_WIDTH) as u8 }
	/// ADRDYIE (Width: 1, Offset: 0)
	pub fn set_adrdyie(value: u8) { ::write(REGISTER_ADDRESS, ADRDYIE_BIT_OFFSET, ADRDYIE_BIT_WIDTH, value as u32); }
}
/// control register
/// Size: 0x20 bits
pub mod cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ADCAL_BIT_OFFSET: u8 = 31;
	const ADCAL_BIT_WIDTH: u8 = 1;
	/// ADCAL (Width: 1, Offset: 31)
	pub fn get_adcal() -> u8 { ::read(REGISTER_ADDRESS, ADCAL_BIT_OFFSET, ADCAL_BIT_WIDTH) as u8 }
	/// ADCAL (Width: 1, Offset: 31)
	pub fn set_adcal(value: u8) { ::write(REGISTER_ADDRESS, ADCAL_BIT_OFFSET, ADCAL_BIT_WIDTH, value as u32); }

	const ADCALDIF_BIT_OFFSET: u8 = 30;
	const ADCALDIF_BIT_WIDTH: u8 = 1;
	/// ADCALDIF (Width: 1, Offset: 30)
	pub fn get_adcaldif() -> u8 { ::read(REGISTER_ADDRESS, ADCALDIF_BIT_OFFSET, ADCALDIF_BIT_WIDTH) as u8 }
	/// ADCALDIF (Width: 1, Offset: 30)
	pub fn set_adcaldif(value: u8) { ::write(REGISTER_ADDRESS, ADCALDIF_BIT_OFFSET, ADCALDIF_BIT_WIDTH, value as u32); }

	const DEEPPWD_BIT_OFFSET: u8 = 29;
	const DEEPPWD_BIT_WIDTH: u8 = 1;
	/// DEEPPWD (Width: 1, Offset: 29)
	pub fn get_deeppwd() -> u8 { ::read(REGISTER_ADDRESS, DEEPPWD_BIT_OFFSET, DEEPPWD_BIT_WIDTH) as u8 }
	/// DEEPPWD (Width: 1, Offset: 29)
	pub fn set_deeppwd(value: u8) { ::write(REGISTER_ADDRESS, DEEPPWD_BIT_OFFSET, DEEPPWD_BIT_WIDTH, value as u32); }

	const ADVREGEN_BIT_OFFSET: u8 = 28;
	const ADVREGEN_BIT_WIDTH: u8 = 1;
	/// ADVREGEN (Width: 1, Offset: 28)
	pub fn get_advregen() -> u8 { ::read(REGISTER_ADDRESS, ADVREGEN_BIT_OFFSET, ADVREGEN_BIT_WIDTH) as u8 }
	/// ADVREGEN (Width: 1, Offset: 28)
	pub fn set_advregen(value: u8) { ::write(REGISTER_ADDRESS, ADVREGEN_BIT_OFFSET, ADVREGEN_BIT_WIDTH, value as u32); }

	const JADSTP_BIT_OFFSET: u8 = 5;
	const JADSTP_BIT_WIDTH: u8 = 1;
	/// JADSTP (Width: 1, Offset: 5)
	pub fn get_jadstp() -> u8 { ::read(REGISTER_ADDRESS, JADSTP_BIT_OFFSET, JADSTP_BIT_WIDTH) as u8 }
	/// JADSTP (Width: 1, Offset: 5)
	pub fn set_jadstp(value: u8) { ::write(REGISTER_ADDRESS, JADSTP_BIT_OFFSET, JADSTP_BIT_WIDTH, value as u32); }

	const ADSTP_BIT_OFFSET: u8 = 4;
	const ADSTP_BIT_WIDTH: u8 = 1;
	/// ADSTP (Width: 1, Offset: 4)
	pub fn get_adstp() -> u8 { ::read(REGISTER_ADDRESS, ADSTP_BIT_OFFSET, ADSTP_BIT_WIDTH) as u8 }
	/// ADSTP (Width: 1, Offset: 4)
	pub fn set_adstp(value: u8) { ::write(REGISTER_ADDRESS, ADSTP_BIT_OFFSET, ADSTP_BIT_WIDTH, value as u32); }

	const JADSTART_BIT_OFFSET: u8 = 3;
	const JADSTART_BIT_WIDTH: u8 = 1;
	/// JADSTART (Width: 1, Offset: 3)
	pub fn get_jadstart() -> u8 { ::read(REGISTER_ADDRESS, JADSTART_BIT_OFFSET, JADSTART_BIT_WIDTH) as u8 }
	/// JADSTART (Width: 1, Offset: 3)
	pub fn set_jadstart(value: u8) { ::write(REGISTER_ADDRESS, JADSTART_BIT_OFFSET, JADSTART_BIT_WIDTH, value as u32); }

	const ADSTART_BIT_OFFSET: u8 = 2;
	const ADSTART_BIT_WIDTH: u8 = 1;
	/// ADSTART (Width: 1, Offset: 2)
	pub fn get_adstart() -> u8 { ::read(REGISTER_ADDRESS, ADSTART_BIT_OFFSET, ADSTART_BIT_WIDTH) as u8 }
	/// ADSTART (Width: 1, Offset: 2)
	pub fn set_adstart(value: u8) { ::write(REGISTER_ADDRESS, ADSTART_BIT_OFFSET, ADSTART_BIT_WIDTH, value as u32); }

	const ADDIS_BIT_OFFSET: u8 = 1;
	const ADDIS_BIT_WIDTH: u8 = 1;
	/// ADDIS (Width: 1, Offset: 1)
	pub fn get_addis() -> u8 { ::read(REGISTER_ADDRESS, ADDIS_BIT_OFFSET, ADDIS_BIT_WIDTH) as u8 }
	/// ADDIS (Width: 1, Offset: 1)
	pub fn set_addis(value: u8) { ::write(REGISTER_ADDRESS, ADDIS_BIT_OFFSET, ADDIS_BIT_WIDTH, value as u32); }

	const ADEN_BIT_OFFSET: u8 = 0;
	const ADEN_BIT_WIDTH: u8 = 1;
	/// ADEN (Width: 1, Offset: 0)
	pub fn get_aden() -> u8 { ::read(REGISTER_ADDRESS, ADEN_BIT_OFFSET, ADEN_BIT_WIDTH) as u8 }
	/// ADEN (Width: 1, Offset: 0)
	pub fn set_aden(value: u8) { ::write(REGISTER_ADDRESS, ADEN_BIT_OFFSET, ADEN_BIT_WIDTH, value as u32); }
}
/// configuration register
/// Size: 0x20 bits
pub mod cfgr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const AWDCH1CH_BIT_OFFSET: u8 = 26;
	const AWDCH1CH_BIT_WIDTH: u8 = 5;
	/// AWDCH1CH (Width: 5, Offset: 26)
	pub fn get_awdch1ch() -> u8 { ::read(REGISTER_ADDRESS, AWDCH1CH_BIT_OFFSET, AWDCH1CH_BIT_WIDTH) as u8 }
	/// AWDCH1CH (Width: 5, Offset: 26)
	pub fn set_awdch1ch(value: u8) { ::write(REGISTER_ADDRESS, AWDCH1CH_BIT_OFFSET, AWDCH1CH_BIT_WIDTH, value as u32); }

	const JAUTO_BIT_OFFSET: u8 = 25;
	const JAUTO_BIT_WIDTH: u8 = 1;
	/// JAUTO (Width: 1, Offset: 25)
	pub fn get_jauto() -> u8 { ::read(REGISTER_ADDRESS, JAUTO_BIT_OFFSET, JAUTO_BIT_WIDTH) as u8 }
	/// JAUTO (Width: 1, Offset: 25)
	pub fn set_jauto(value: u8) { ::write(REGISTER_ADDRESS, JAUTO_BIT_OFFSET, JAUTO_BIT_WIDTH, value as u32); }

	const JAWD1EN_BIT_OFFSET: u8 = 24;
	const JAWD1EN_BIT_WIDTH: u8 = 1;
	/// JAWD1EN (Width: 1, Offset: 24)
	pub fn get_jawd1en() -> u8 { ::read(REGISTER_ADDRESS, JAWD1EN_BIT_OFFSET, JAWD1EN_BIT_WIDTH) as u8 }
	/// JAWD1EN (Width: 1, Offset: 24)
	pub fn set_jawd1en(value: u8) { ::write(REGISTER_ADDRESS, JAWD1EN_BIT_OFFSET, JAWD1EN_BIT_WIDTH, value as u32); }

	const AWD1EN_BIT_OFFSET: u8 = 23;
	const AWD1EN_BIT_WIDTH: u8 = 1;
	/// AWD1EN (Width: 1, Offset: 23)
	pub fn get_awd1en() -> u8 { ::read(REGISTER_ADDRESS, AWD1EN_BIT_OFFSET, AWD1EN_BIT_WIDTH) as u8 }
	/// AWD1EN (Width: 1, Offset: 23)
	pub fn set_awd1en(value: u8) { ::write(REGISTER_ADDRESS, AWD1EN_BIT_OFFSET, AWD1EN_BIT_WIDTH, value as u32); }

	const AWD1SGL_BIT_OFFSET: u8 = 22;
	const AWD1SGL_BIT_WIDTH: u8 = 1;
	/// AWD1SGL (Width: 1, Offset: 22)
	pub fn get_awd1sgl() -> u8 { ::read(REGISTER_ADDRESS, AWD1SGL_BIT_OFFSET, AWD1SGL_BIT_WIDTH) as u8 }
	/// AWD1SGL (Width: 1, Offset: 22)
	pub fn set_awd1sgl(value: u8) { ::write(REGISTER_ADDRESS, AWD1SGL_BIT_OFFSET, AWD1SGL_BIT_WIDTH, value as u32); }

	const JQM_BIT_OFFSET: u8 = 21;
	const JQM_BIT_WIDTH: u8 = 1;
	/// JQM (Width: 1, Offset: 21)
	pub fn get_jqm() -> u8 { ::read(REGISTER_ADDRESS, JQM_BIT_OFFSET, JQM_BIT_WIDTH) as u8 }
	/// JQM (Width: 1, Offset: 21)
	pub fn set_jqm(value: u8) { ::write(REGISTER_ADDRESS, JQM_BIT_OFFSET, JQM_BIT_WIDTH, value as u32); }

	const JDISCEN_BIT_OFFSET: u8 = 20;
	const JDISCEN_BIT_WIDTH: u8 = 1;
	/// JDISCEN (Width: 1, Offset: 20)
	pub fn get_jdiscen() -> u8 { ::read(REGISTER_ADDRESS, JDISCEN_BIT_OFFSET, JDISCEN_BIT_WIDTH) as u8 }
	/// JDISCEN (Width: 1, Offset: 20)
	pub fn set_jdiscen(value: u8) { ::write(REGISTER_ADDRESS, JDISCEN_BIT_OFFSET, JDISCEN_BIT_WIDTH, value as u32); }

	const DISCNUM_BIT_OFFSET: u8 = 17;
	const DISCNUM_BIT_WIDTH: u8 = 3;
	/// DISCNUM (Width: 3, Offset: 17)
	pub fn get_discnum() -> u8 { ::read(REGISTER_ADDRESS, DISCNUM_BIT_OFFSET, DISCNUM_BIT_WIDTH) as u8 }
	/// DISCNUM (Width: 3, Offset: 17)
	pub fn set_discnum(value: u8) { ::write(REGISTER_ADDRESS, DISCNUM_BIT_OFFSET, DISCNUM_BIT_WIDTH, value as u32); }

	const DISCEN_BIT_OFFSET: u8 = 16;
	const DISCEN_BIT_WIDTH: u8 = 1;
	/// DISCEN (Width: 1, Offset: 16)
	pub fn get_discen() -> u8 { ::read(REGISTER_ADDRESS, DISCEN_BIT_OFFSET, DISCEN_BIT_WIDTH) as u8 }
	/// DISCEN (Width: 1, Offset: 16)
	pub fn set_discen(value: u8) { ::write(REGISTER_ADDRESS, DISCEN_BIT_OFFSET, DISCEN_BIT_WIDTH, value as u32); }

	const AUTOFF_BIT_OFFSET: u8 = 15;
	const AUTOFF_BIT_WIDTH: u8 = 1;
	/// AUTOFF (Width: 1, Offset: 15)
	pub fn get_autoff() -> u8 { ::read(REGISTER_ADDRESS, AUTOFF_BIT_OFFSET, AUTOFF_BIT_WIDTH) as u8 }
	/// AUTOFF (Width: 1, Offset: 15)
	pub fn set_autoff(value: u8) { ::write(REGISTER_ADDRESS, AUTOFF_BIT_OFFSET, AUTOFF_BIT_WIDTH, value as u32); }

	const AUTDLY_BIT_OFFSET: u8 = 14;
	const AUTDLY_BIT_WIDTH: u8 = 1;
	/// AUTDLY (Width: 1, Offset: 14)
	pub fn get_autdly() -> u8 { ::read(REGISTER_ADDRESS, AUTDLY_BIT_OFFSET, AUTDLY_BIT_WIDTH) as u8 }
	/// AUTDLY (Width: 1, Offset: 14)
	pub fn set_autdly(value: u8) { ::write(REGISTER_ADDRESS, AUTDLY_BIT_OFFSET, AUTDLY_BIT_WIDTH, value as u32); }

	const CONT_BIT_OFFSET: u8 = 13;
	const CONT_BIT_WIDTH: u8 = 1;
	/// CONT (Width: 1, Offset: 13)
	pub fn get_cont() -> u8 { ::read(REGISTER_ADDRESS, CONT_BIT_OFFSET, CONT_BIT_WIDTH) as u8 }
	/// CONT (Width: 1, Offset: 13)
	pub fn set_cont(value: u8) { ::write(REGISTER_ADDRESS, CONT_BIT_OFFSET, CONT_BIT_WIDTH, value as u32); }

	const OVRMOD_BIT_OFFSET: u8 = 12;
	const OVRMOD_BIT_WIDTH: u8 = 1;
	/// OVRMOD (Width: 1, Offset: 12)
	pub fn get_ovrmod() -> u8 { ::read(REGISTER_ADDRESS, OVRMOD_BIT_OFFSET, OVRMOD_BIT_WIDTH) as u8 }
	/// OVRMOD (Width: 1, Offset: 12)
	pub fn set_ovrmod(value: u8) { ::write(REGISTER_ADDRESS, OVRMOD_BIT_OFFSET, OVRMOD_BIT_WIDTH, value as u32); }

	const EXTEN_BIT_OFFSET: u8 = 10;
	const EXTEN_BIT_WIDTH: u8 = 2;
	/// EXTEN (Width: 2, Offset: 10)
	pub fn get_exten() -> u8 { ::read(REGISTER_ADDRESS, EXTEN_BIT_OFFSET, EXTEN_BIT_WIDTH) as u8 }
	/// EXTEN (Width: 2, Offset: 10)
	pub fn set_exten(value: u8) { ::write(REGISTER_ADDRESS, EXTEN_BIT_OFFSET, EXTEN_BIT_WIDTH, value as u32); }

	const EXTSEL_BIT_OFFSET: u8 = 6;
	const EXTSEL_BIT_WIDTH: u8 = 4;
	/// EXTSEL (Width: 4, Offset: 6)
	pub fn get_extsel() -> u8 { ::read(REGISTER_ADDRESS, EXTSEL_BIT_OFFSET, EXTSEL_BIT_WIDTH) as u8 }
	/// EXTSEL (Width: 4, Offset: 6)
	pub fn set_extsel(value: u8) { ::write(REGISTER_ADDRESS, EXTSEL_BIT_OFFSET, EXTSEL_BIT_WIDTH, value as u32); }

	const ALIGN_BIT_OFFSET: u8 = 5;
	const ALIGN_BIT_WIDTH: u8 = 1;
	/// ALIGN (Width: 1, Offset: 5)
	pub fn get_align() -> u8 { ::read(REGISTER_ADDRESS, ALIGN_BIT_OFFSET, ALIGN_BIT_WIDTH) as u8 }
	/// ALIGN (Width: 1, Offset: 5)
	pub fn set_align(value: u8) { ::write(REGISTER_ADDRESS, ALIGN_BIT_OFFSET, ALIGN_BIT_WIDTH, value as u32); }

	const RES_BIT_OFFSET: u8 = 3;
	const RES_BIT_WIDTH: u8 = 2;
	/// RES (Width: 2, Offset: 3)
	pub fn get_res() -> u8 { ::read(REGISTER_ADDRESS, RES_BIT_OFFSET, RES_BIT_WIDTH) as u8 }
	/// RES (Width: 2, Offset: 3)
	pub fn set_res(value: u8) { ::write(REGISTER_ADDRESS, RES_BIT_OFFSET, RES_BIT_WIDTH, value as u32); }

	const DMACFG_BIT_OFFSET: u8 = 1;
	const DMACFG_BIT_WIDTH: u8 = 1;
	/// DMACFG (Width: 1, Offset: 1)
	pub fn get_dmacfg() -> u8 { ::read(REGISTER_ADDRESS, DMACFG_BIT_OFFSET, DMACFG_BIT_WIDTH) as u8 }
	/// DMACFG (Width: 1, Offset: 1)
	pub fn set_dmacfg(value: u8) { ::write(REGISTER_ADDRESS, DMACFG_BIT_OFFSET, DMACFG_BIT_WIDTH, value as u32); }

	const DMAEN_BIT_OFFSET: u8 = 0;
	const DMAEN_BIT_WIDTH: u8 = 1;
	/// DMAEN (Width: 1, Offset: 0)
	pub fn get_dmaen() -> u8 { ::read(REGISTER_ADDRESS, DMAEN_BIT_OFFSET, DMAEN_BIT_WIDTH) as u8 }
	/// DMAEN (Width: 1, Offset: 0)
	pub fn set_dmaen(value: u8) { ::write(REGISTER_ADDRESS, DMAEN_BIT_OFFSET, DMAEN_BIT_WIDTH, value as u32); }
}
/// sample time register 1
/// Size: 0x20 bits
pub mod smpr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SMP9_BIT_OFFSET: u8 = 27;
	const SMP9_BIT_WIDTH: u8 = 3;
	/// SMP9 (Width: 3, Offset: 27)
	pub fn get_smp9() -> u8 { ::read(REGISTER_ADDRESS, SMP9_BIT_OFFSET, SMP9_BIT_WIDTH) as u8 }
	/// SMP9 (Width: 3, Offset: 27)
	pub fn set_smp9(value: u8) { ::write(REGISTER_ADDRESS, SMP9_BIT_OFFSET, SMP9_BIT_WIDTH, value as u32); }

	const SMP8_BIT_OFFSET: u8 = 24;
	const SMP8_BIT_WIDTH: u8 = 3;
	/// SMP8 (Width: 3, Offset: 24)
	pub fn get_smp8() -> u8 { ::read(REGISTER_ADDRESS, SMP8_BIT_OFFSET, SMP8_BIT_WIDTH) as u8 }
	/// SMP8 (Width: 3, Offset: 24)
	pub fn set_smp8(value: u8) { ::write(REGISTER_ADDRESS, SMP8_BIT_OFFSET, SMP8_BIT_WIDTH, value as u32); }

	const SMP7_BIT_OFFSET: u8 = 21;
	const SMP7_BIT_WIDTH: u8 = 3;
	/// SMP7 (Width: 3, Offset: 21)
	pub fn get_smp7() -> u8 { ::read(REGISTER_ADDRESS, SMP7_BIT_OFFSET, SMP7_BIT_WIDTH) as u8 }
	/// SMP7 (Width: 3, Offset: 21)
	pub fn set_smp7(value: u8) { ::write(REGISTER_ADDRESS, SMP7_BIT_OFFSET, SMP7_BIT_WIDTH, value as u32); }

	const SMP6_BIT_OFFSET: u8 = 18;
	const SMP6_BIT_WIDTH: u8 = 3;
	/// SMP6 (Width: 3, Offset: 18)
	pub fn get_smp6() -> u8 { ::read(REGISTER_ADDRESS, SMP6_BIT_OFFSET, SMP6_BIT_WIDTH) as u8 }
	/// SMP6 (Width: 3, Offset: 18)
	pub fn set_smp6(value: u8) { ::write(REGISTER_ADDRESS, SMP6_BIT_OFFSET, SMP6_BIT_WIDTH, value as u32); }

	const SMP5_BIT_OFFSET: u8 = 15;
	const SMP5_BIT_WIDTH: u8 = 3;
	/// SMP5 (Width: 3, Offset: 15)
	pub fn get_smp5() -> u8 { ::read(REGISTER_ADDRESS, SMP5_BIT_OFFSET, SMP5_BIT_WIDTH) as u8 }
	/// SMP5 (Width: 3, Offset: 15)
	pub fn set_smp5(value: u8) { ::write(REGISTER_ADDRESS, SMP5_BIT_OFFSET, SMP5_BIT_WIDTH, value as u32); }

	const SMP4_BIT_OFFSET: u8 = 12;
	const SMP4_BIT_WIDTH: u8 = 3;
	/// SMP4 (Width: 3, Offset: 12)
	pub fn get_smp4() -> u8 { ::read(REGISTER_ADDRESS, SMP4_BIT_OFFSET, SMP4_BIT_WIDTH) as u8 }
	/// SMP4 (Width: 3, Offset: 12)
	pub fn set_smp4(value: u8) { ::write(REGISTER_ADDRESS, SMP4_BIT_OFFSET, SMP4_BIT_WIDTH, value as u32); }

	const SMP3_BIT_OFFSET: u8 = 9;
	const SMP3_BIT_WIDTH: u8 = 3;
	/// SMP3 (Width: 3, Offset: 9)
	pub fn get_smp3() -> u8 { ::read(REGISTER_ADDRESS, SMP3_BIT_OFFSET, SMP3_BIT_WIDTH) as u8 }
	/// SMP3 (Width: 3, Offset: 9)
	pub fn set_smp3(value: u8) { ::write(REGISTER_ADDRESS, SMP3_BIT_OFFSET, SMP3_BIT_WIDTH, value as u32); }

	const SMP2_BIT_OFFSET: u8 = 6;
	const SMP2_BIT_WIDTH: u8 = 3;
	/// SMP2 (Width: 3, Offset: 6)
	pub fn get_smp2() -> u8 { ::read(REGISTER_ADDRESS, SMP2_BIT_OFFSET, SMP2_BIT_WIDTH) as u8 }
	/// SMP2 (Width: 3, Offset: 6)
	pub fn set_smp2(value: u8) { ::write(REGISTER_ADDRESS, SMP2_BIT_OFFSET, SMP2_BIT_WIDTH, value as u32); }

	const SMP1_BIT_OFFSET: u8 = 3;
	const SMP1_BIT_WIDTH: u8 = 3;
	/// SMP1 (Width: 3, Offset: 3)
	pub fn get_smp1() -> u8 { ::read(REGISTER_ADDRESS, SMP1_BIT_OFFSET, SMP1_BIT_WIDTH) as u8 }
	/// SMP1 (Width: 3, Offset: 3)
	pub fn set_smp1(value: u8) { ::write(REGISTER_ADDRESS, SMP1_BIT_OFFSET, SMP1_BIT_WIDTH, value as u32); }
}
/// sample time register 2
/// Size: 0x20 bits
pub mod smpr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SMP18_BIT_OFFSET: u8 = 24;
	const SMP18_BIT_WIDTH: u8 = 3;
	/// SMP18 (Width: 3, Offset: 24)
	pub fn get_smp18() -> u8 { ::read(REGISTER_ADDRESS, SMP18_BIT_OFFSET, SMP18_BIT_WIDTH) as u8 }
	/// SMP18 (Width: 3, Offset: 24)
	pub fn set_smp18(value: u8) { ::write(REGISTER_ADDRESS, SMP18_BIT_OFFSET, SMP18_BIT_WIDTH, value as u32); }

	const SMP17_BIT_OFFSET: u8 = 21;
	const SMP17_BIT_WIDTH: u8 = 3;
	/// SMP17 (Width: 3, Offset: 21)
	pub fn get_smp17() -> u8 { ::read(REGISTER_ADDRESS, SMP17_BIT_OFFSET, SMP17_BIT_WIDTH) as u8 }
	/// SMP17 (Width: 3, Offset: 21)
	pub fn set_smp17(value: u8) { ::write(REGISTER_ADDRESS, SMP17_BIT_OFFSET, SMP17_BIT_WIDTH, value as u32); }

	const SMP16_BIT_OFFSET: u8 = 18;
	const SMP16_BIT_WIDTH: u8 = 3;
	/// SMP16 (Width: 3, Offset: 18)
	pub fn get_smp16() -> u8 { ::read(REGISTER_ADDRESS, SMP16_BIT_OFFSET, SMP16_BIT_WIDTH) as u8 }
	/// SMP16 (Width: 3, Offset: 18)
	pub fn set_smp16(value: u8) { ::write(REGISTER_ADDRESS, SMP16_BIT_OFFSET, SMP16_BIT_WIDTH, value as u32); }

	const SMP15_BIT_OFFSET: u8 = 15;
	const SMP15_BIT_WIDTH: u8 = 3;
	/// SMP15 (Width: 3, Offset: 15)
	pub fn get_smp15() -> u8 { ::read(REGISTER_ADDRESS, SMP15_BIT_OFFSET, SMP15_BIT_WIDTH) as u8 }
	/// SMP15 (Width: 3, Offset: 15)
	pub fn set_smp15(value: u8) { ::write(REGISTER_ADDRESS, SMP15_BIT_OFFSET, SMP15_BIT_WIDTH, value as u32); }

	const SMP14_BIT_OFFSET: u8 = 12;
	const SMP14_BIT_WIDTH: u8 = 3;
	/// SMP14 (Width: 3, Offset: 12)
	pub fn get_smp14() -> u8 { ::read(REGISTER_ADDRESS, SMP14_BIT_OFFSET, SMP14_BIT_WIDTH) as u8 }
	/// SMP14 (Width: 3, Offset: 12)
	pub fn set_smp14(value: u8) { ::write(REGISTER_ADDRESS, SMP14_BIT_OFFSET, SMP14_BIT_WIDTH, value as u32); }

	const SMP13_BIT_OFFSET: u8 = 9;
	const SMP13_BIT_WIDTH: u8 = 3;
	/// SMP13 (Width: 3, Offset: 9)
	pub fn get_smp13() -> u8 { ::read(REGISTER_ADDRESS, SMP13_BIT_OFFSET, SMP13_BIT_WIDTH) as u8 }
	/// SMP13 (Width: 3, Offset: 9)
	pub fn set_smp13(value: u8) { ::write(REGISTER_ADDRESS, SMP13_BIT_OFFSET, SMP13_BIT_WIDTH, value as u32); }

	const SMP12_BIT_OFFSET: u8 = 6;
	const SMP12_BIT_WIDTH: u8 = 3;
	/// SMP12 (Width: 3, Offset: 6)
	pub fn get_smp12() -> u8 { ::read(REGISTER_ADDRESS, SMP12_BIT_OFFSET, SMP12_BIT_WIDTH) as u8 }
	/// SMP12 (Width: 3, Offset: 6)
	pub fn set_smp12(value: u8) { ::write(REGISTER_ADDRESS, SMP12_BIT_OFFSET, SMP12_BIT_WIDTH, value as u32); }

	const SMP11_BIT_OFFSET: u8 = 3;
	const SMP11_BIT_WIDTH: u8 = 3;
	/// SMP11 (Width: 3, Offset: 3)
	pub fn get_smp11() -> u8 { ::read(REGISTER_ADDRESS, SMP11_BIT_OFFSET, SMP11_BIT_WIDTH) as u8 }
	/// SMP11 (Width: 3, Offset: 3)
	pub fn set_smp11(value: u8) { ::write(REGISTER_ADDRESS, SMP11_BIT_OFFSET, SMP11_BIT_WIDTH, value as u32); }

	const SMP10_BIT_OFFSET: u8 = 0;
	const SMP10_BIT_WIDTH: u8 = 3;
	/// SMP10 (Width: 3, Offset: 0)
	pub fn get_smp10() -> u8 { ::read(REGISTER_ADDRESS, SMP10_BIT_OFFSET, SMP10_BIT_WIDTH) as u8 }
	/// SMP10 (Width: 3, Offset: 0)
	pub fn set_smp10(value: u8) { ::write(REGISTER_ADDRESS, SMP10_BIT_OFFSET, SMP10_BIT_WIDTH, value as u32); }
}
/// watchdog threshold register 1
/// Size: 0x20 bits
pub mod tr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x20;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const HT1_BIT_OFFSET: u8 = 16;
	const HT1_BIT_WIDTH: u8 = 12;
	/// HT1 (Width: 12, Offset: 16)
	pub fn get_ht1() -> u16 { ::read(REGISTER_ADDRESS, HT1_BIT_OFFSET, HT1_BIT_WIDTH) as u16 }
	/// HT1 (Width: 12, Offset: 16)
	pub fn set_ht1(value: u16) { ::write(REGISTER_ADDRESS, HT1_BIT_OFFSET, HT1_BIT_WIDTH, value as u32); }

	const LT1_BIT_OFFSET: u8 = 0;
	const LT1_BIT_WIDTH: u8 = 12;
	/// LT1 (Width: 12, Offset: 0)
	pub fn get_lt1() -> u16 { ::read(REGISTER_ADDRESS, LT1_BIT_OFFSET, LT1_BIT_WIDTH) as u16 }
	/// LT1 (Width: 12, Offset: 0)
	pub fn set_lt1(value: u16) { ::write(REGISTER_ADDRESS, LT1_BIT_OFFSET, LT1_BIT_WIDTH, value as u32); }
}
/// watchdog threshold register
/// Size: 0x20 bits
pub mod tr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x24;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const HT2_BIT_OFFSET: u8 = 16;
	const HT2_BIT_WIDTH: u8 = 8;
	/// HT2 (Width: 8, Offset: 16)
	pub fn get_ht2() -> u8 { ::read(REGISTER_ADDRESS, HT2_BIT_OFFSET, HT2_BIT_WIDTH) as u8 }
	/// HT2 (Width: 8, Offset: 16)
	pub fn set_ht2(value: u8) { ::write(REGISTER_ADDRESS, HT2_BIT_OFFSET, HT2_BIT_WIDTH, value as u32); }

	const LT2_BIT_OFFSET: u8 = 0;
	const LT2_BIT_WIDTH: u8 = 8;
	/// LT2 (Width: 8, Offset: 0)
	pub fn get_lt2() -> u8 { ::read(REGISTER_ADDRESS, LT2_BIT_OFFSET, LT2_BIT_WIDTH) as u8 }
	/// LT2 (Width: 8, Offset: 0)
	pub fn set_lt2(value: u8) { ::write(REGISTER_ADDRESS, LT2_BIT_OFFSET, LT2_BIT_WIDTH, value as u32); }
}
/// watchdog threshold register 3
/// Size: 0x20 bits
pub mod tr3 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x28;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const HT3_BIT_OFFSET: u8 = 16;
	const HT3_BIT_WIDTH: u8 = 8;
	/// HT3 (Width: 8, Offset: 16)
	pub fn get_ht3() -> u8 { ::read(REGISTER_ADDRESS, HT3_BIT_OFFSET, HT3_BIT_WIDTH) as u8 }
	/// HT3 (Width: 8, Offset: 16)
	pub fn set_ht3(value: u8) { ::write(REGISTER_ADDRESS, HT3_BIT_OFFSET, HT3_BIT_WIDTH, value as u32); }

	const LT3_BIT_OFFSET: u8 = 0;
	const LT3_BIT_WIDTH: u8 = 8;
	/// LT3 (Width: 8, Offset: 0)
	pub fn get_lt3() -> u8 { ::read(REGISTER_ADDRESS, LT3_BIT_OFFSET, LT3_BIT_WIDTH) as u8 }
	/// LT3 (Width: 8, Offset: 0)
	pub fn set_lt3(value: u8) { ::write(REGISTER_ADDRESS, LT3_BIT_OFFSET, LT3_BIT_WIDTH, value as u32); }
}
/// regular sequence register 1
/// Size: 0x20 bits
pub mod sqr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x30;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SQ4_BIT_OFFSET: u8 = 24;
	const SQ4_BIT_WIDTH: u8 = 5;
	/// SQ4 (Width: 5, Offset: 24)
	pub fn get_sq4() -> u8 { ::read(REGISTER_ADDRESS, SQ4_BIT_OFFSET, SQ4_BIT_WIDTH) as u8 }
	/// SQ4 (Width: 5, Offset: 24)
	pub fn set_sq4(value: u8) { ::write(REGISTER_ADDRESS, SQ4_BIT_OFFSET, SQ4_BIT_WIDTH, value as u32); }

	const SQ3_BIT_OFFSET: u8 = 18;
	const SQ3_BIT_WIDTH: u8 = 5;
	/// SQ3 (Width: 5, Offset: 18)
	pub fn get_sq3() -> u8 { ::read(REGISTER_ADDRESS, SQ3_BIT_OFFSET, SQ3_BIT_WIDTH) as u8 }
	/// SQ3 (Width: 5, Offset: 18)
	pub fn set_sq3(value: u8) { ::write(REGISTER_ADDRESS, SQ3_BIT_OFFSET, SQ3_BIT_WIDTH, value as u32); }

	const SQ2_BIT_OFFSET: u8 = 12;
	const SQ2_BIT_WIDTH: u8 = 5;
	/// SQ2 (Width: 5, Offset: 12)
	pub fn get_sq2() -> u8 { ::read(REGISTER_ADDRESS, SQ2_BIT_OFFSET, SQ2_BIT_WIDTH) as u8 }
	/// SQ2 (Width: 5, Offset: 12)
	pub fn set_sq2(value: u8) { ::write(REGISTER_ADDRESS, SQ2_BIT_OFFSET, SQ2_BIT_WIDTH, value as u32); }

	const SQ1_BIT_OFFSET: u8 = 6;
	const SQ1_BIT_WIDTH: u8 = 5;
	/// SQ1 (Width: 5, Offset: 6)
	pub fn get_sq1() -> u8 { ::read(REGISTER_ADDRESS, SQ1_BIT_OFFSET, SQ1_BIT_WIDTH) as u8 }
	/// SQ1 (Width: 5, Offset: 6)
	pub fn set_sq1(value: u8) { ::write(REGISTER_ADDRESS, SQ1_BIT_OFFSET, SQ1_BIT_WIDTH, value as u32); }

	const L3_BIT_OFFSET: u8 = 0;
	const L3_BIT_WIDTH: u8 = 4;
	/// L3 (Width: 4, Offset: 0)
	pub fn get_l3() -> u8 { ::read(REGISTER_ADDRESS, L3_BIT_OFFSET, L3_BIT_WIDTH) as u8 }
	/// L3 (Width: 4, Offset: 0)
	pub fn set_l3(value: u8) { ::write(REGISTER_ADDRESS, L3_BIT_OFFSET, L3_BIT_WIDTH, value as u32); }
}
/// regular sequence register 2
/// Size: 0x20 bits
pub mod sqr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x34;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SQ9_BIT_OFFSET: u8 = 24;
	const SQ9_BIT_WIDTH: u8 = 5;
	/// SQ9 (Width: 5, Offset: 24)
	pub fn get_sq9() -> u8 { ::read(REGISTER_ADDRESS, SQ9_BIT_OFFSET, SQ9_BIT_WIDTH) as u8 }
	/// SQ9 (Width: 5, Offset: 24)
	pub fn set_sq9(value: u8) { ::write(REGISTER_ADDRESS, SQ9_BIT_OFFSET, SQ9_BIT_WIDTH, value as u32); }

	const SQ8_BIT_OFFSET: u8 = 18;
	const SQ8_BIT_WIDTH: u8 = 5;
	/// SQ8 (Width: 5, Offset: 18)
	pub fn get_sq8() -> u8 { ::read(REGISTER_ADDRESS, SQ8_BIT_OFFSET, SQ8_BIT_WIDTH) as u8 }
	/// SQ8 (Width: 5, Offset: 18)
	pub fn set_sq8(value: u8) { ::write(REGISTER_ADDRESS, SQ8_BIT_OFFSET, SQ8_BIT_WIDTH, value as u32); }

	const SQ7_BIT_OFFSET: u8 = 12;
	const SQ7_BIT_WIDTH: u8 = 5;
	/// SQ7 (Width: 5, Offset: 12)
	pub fn get_sq7() -> u8 { ::read(REGISTER_ADDRESS, SQ7_BIT_OFFSET, SQ7_BIT_WIDTH) as u8 }
	/// SQ7 (Width: 5, Offset: 12)
	pub fn set_sq7(value: u8) { ::write(REGISTER_ADDRESS, SQ7_BIT_OFFSET, SQ7_BIT_WIDTH, value as u32); }

	const SQ6_BIT_OFFSET: u8 = 6;
	const SQ6_BIT_WIDTH: u8 = 5;
	/// SQ6 (Width: 5, Offset: 6)
	pub fn get_sq6() -> u8 { ::read(REGISTER_ADDRESS, SQ6_BIT_OFFSET, SQ6_BIT_WIDTH) as u8 }
	/// SQ6 (Width: 5, Offset: 6)
	pub fn set_sq6(value: u8) { ::write(REGISTER_ADDRESS, SQ6_BIT_OFFSET, SQ6_BIT_WIDTH, value as u32); }

	const SQ5_BIT_OFFSET: u8 = 0;
	const SQ5_BIT_WIDTH: u8 = 5;
	/// SQ5 (Width: 5, Offset: 0)
	pub fn get_sq5() -> u8 { ::read(REGISTER_ADDRESS, SQ5_BIT_OFFSET, SQ5_BIT_WIDTH) as u8 }
	/// SQ5 (Width: 5, Offset: 0)
	pub fn set_sq5(value: u8) { ::write(REGISTER_ADDRESS, SQ5_BIT_OFFSET, SQ5_BIT_WIDTH, value as u32); }
}
/// regular sequence register 3
/// Size: 0x20 bits
pub mod sqr3 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x38;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SQ14_BIT_OFFSET: u8 = 24;
	const SQ14_BIT_WIDTH: u8 = 5;
	/// SQ14 (Width: 5, Offset: 24)
	pub fn get_sq14() -> u8 { ::read(REGISTER_ADDRESS, SQ14_BIT_OFFSET, SQ14_BIT_WIDTH) as u8 }
	/// SQ14 (Width: 5, Offset: 24)
	pub fn set_sq14(value: u8) { ::write(REGISTER_ADDRESS, SQ14_BIT_OFFSET, SQ14_BIT_WIDTH, value as u32); }

	const SQ13_BIT_OFFSET: u8 = 18;
	const SQ13_BIT_WIDTH: u8 = 5;
	/// SQ13 (Width: 5, Offset: 18)
	pub fn get_sq13() -> u8 { ::read(REGISTER_ADDRESS, SQ13_BIT_OFFSET, SQ13_BIT_WIDTH) as u8 }
	/// SQ13 (Width: 5, Offset: 18)
	pub fn set_sq13(value: u8) { ::write(REGISTER_ADDRESS, SQ13_BIT_OFFSET, SQ13_BIT_WIDTH, value as u32); }

	const SQ12_BIT_OFFSET: u8 = 12;
	const SQ12_BIT_WIDTH: u8 = 5;
	/// SQ12 (Width: 5, Offset: 12)
	pub fn get_sq12() -> u8 { ::read(REGISTER_ADDRESS, SQ12_BIT_OFFSET, SQ12_BIT_WIDTH) as u8 }
	/// SQ12 (Width: 5, Offset: 12)
	pub fn set_sq12(value: u8) { ::write(REGISTER_ADDRESS, SQ12_BIT_OFFSET, SQ12_BIT_WIDTH, value as u32); }

	const SQ11_BIT_OFFSET: u8 = 6;
	const SQ11_BIT_WIDTH: u8 = 5;
	/// SQ11 (Width: 5, Offset: 6)
	pub fn get_sq11() -> u8 { ::read(REGISTER_ADDRESS, SQ11_BIT_OFFSET, SQ11_BIT_WIDTH) as u8 }
	/// SQ11 (Width: 5, Offset: 6)
	pub fn set_sq11(value: u8) { ::write(REGISTER_ADDRESS, SQ11_BIT_OFFSET, SQ11_BIT_WIDTH, value as u32); }

	const SQ10_BIT_OFFSET: u8 = 0;
	const SQ10_BIT_WIDTH: u8 = 5;
	/// SQ10 (Width: 5, Offset: 0)
	pub fn get_sq10() -> u8 { ::read(REGISTER_ADDRESS, SQ10_BIT_OFFSET, SQ10_BIT_WIDTH) as u8 }
	/// SQ10 (Width: 5, Offset: 0)
	pub fn set_sq10(value: u8) { ::write(REGISTER_ADDRESS, SQ10_BIT_OFFSET, SQ10_BIT_WIDTH, value as u32); }
}
/// regular sequence register 4
/// Size: 0x20 bits
pub mod sqr4 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x3C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SQ16_BIT_OFFSET: u8 = 6;
	const SQ16_BIT_WIDTH: u8 = 5;
	/// SQ16 (Width: 5, Offset: 6)
	pub fn get_sq16() -> u8 { ::read(REGISTER_ADDRESS, SQ16_BIT_OFFSET, SQ16_BIT_WIDTH) as u8 }
	/// SQ16 (Width: 5, Offset: 6)
	pub fn set_sq16(value: u8) { ::write(REGISTER_ADDRESS, SQ16_BIT_OFFSET, SQ16_BIT_WIDTH, value as u32); }

	const SQ15_BIT_OFFSET: u8 = 0;
	const SQ15_BIT_WIDTH: u8 = 5;
	/// SQ15 (Width: 5, Offset: 0)
	pub fn get_sq15() -> u8 { ::read(REGISTER_ADDRESS, SQ15_BIT_OFFSET, SQ15_BIT_WIDTH) as u8 }
	/// SQ15 (Width: 5, Offset: 0)
	pub fn set_sq15(value: u8) { ::write(REGISTER_ADDRESS, SQ15_BIT_OFFSET, SQ15_BIT_WIDTH, value as u32); }
}
/// regular Data Register
/// Size: 0x20 bits
pub mod dr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x40;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const regularDATA_BIT_OFFSET: u8 = 0;
	const regularDATA_BIT_WIDTH: u8 = 16;
	/// regularDATA (Width: 16, Offset: 0)
	pub fn get_regulardata() -> u16 { ::read(REGISTER_ADDRESS, regularDATA_BIT_OFFSET, regularDATA_BIT_WIDTH) as u16 }
}
/// injected sequence register
/// Size: 0x20 bits
pub mod jsqr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const JSQ4_BIT_OFFSET: u8 = 26;
	const JSQ4_BIT_WIDTH: u8 = 5;
	/// JSQ4 (Width: 5, Offset: 26)
	pub fn get_jsq4() -> u8 { ::read(REGISTER_ADDRESS, JSQ4_BIT_OFFSET, JSQ4_BIT_WIDTH) as u8 }
	/// JSQ4 (Width: 5, Offset: 26)
	pub fn set_jsq4(value: u8) { ::write(REGISTER_ADDRESS, JSQ4_BIT_OFFSET, JSQ4_BIT_WIDTH, value as u32); }

	const JSQ3_BIT_OFFSET: u8 = 20;
	const JSQ3_BIT_WIDTH: u8 = 5;
	/// JSQ3 (Width: 5, Offset: 20)
	pub fn get_jsq3() -> u8 { ::read(REGISTER_ADDRESS, JSQ3_BIT_OFFSET, JSQ3_BIT_WIDTH) as u8 }
	/// JSQ3 (Width: 5, Offset: 20)
	pub fn set_jsq3(value: u8) { ::write(REGISTER_ADDRESS, JSQ3_BIT_OFFSET, JSQ3_BIT_WIDTH, value as u32); }

	const JSQ2_BIT_OFFSET: u8 = 14;
	const JSQ2_BIT_WIDTH: u8 = 5;
	/// JSQ2 (Width: 5, Offset: 14)
	pub fn get_jsq2() -> u8 { ::read(REGISTER_ADDRESS, JSQ2_BIT_OFFSET, JSQ2_BIT_WIDTH) as u8 }
	/// JSQ2 (Width: 5, Offset: 14)
	pub fn set_jsq2(value: u8) { ::write(REGISTER_ADDRESS, JSQ2_BIT_OFFSET, JSQ2_BIT_WIDTH, value as u32); }

	const JSQ1_BIT_OFFSET: u8 = 8;
	const JSQ1_BIT_WIDTH: u8 = 5;
	/// JSQ1 (Width: 5, Offset: 8)
	pub fn get_jsq1() -> u8 { ::read(REGISTER_ADDRESS, JSQ1_BIT_OFFSET, JSQ1_BIT_WIDTH) as u8 }
	/// JSQ1 (Width: 5, Offset: 8)
	pub fn set_jsq1(value: u8) { ::write(REGISTER_ADDRESS, JSQ1_BIT_OFFSET, JSQ1_BIT_WIDTH, value as u32); }

	const JEXTEN_BIT_OFFSET: u8 = 6;
	const JEXTEN_BIT_WIDTH: u8 = 2;
	/// JEXTEN (Width: 2, Offset: 6)
	pub fn get_jexten() -> u8 { ::read(REGISTER_ADDRESS, JEXTEN_BIT_OFFSET, JEXTEN_BIT_WIDTH) as u8 }
	/// JEXTEN (Width: 2, Offset: 6)
	pub fn set_jexten(value: u8) { ::write(REGISTER_ADDRESS, JEXTEN_BIT_OFFSET, JEXTEN_BIT_WIDTH, value as u32); }

	const JEXTSEL_BIT_OFFSET: u8 = 2;
	const JEXTSEL_BIT_WIDTH: u8 = 4;
	/// JEXTSEL (Width: 4, Offset: 2)
	pub fn get_jextsel() -> u8 { ::read(REGISTER_ADDRESS, JEXTSEL_BIT_OFFSET, JEXTSEL_BIT_WIDTH) as u8 }
	/// JEXTSEL (Width: 4, Offset: 2)
	pub fn set_jextsel(value: u8) { ::write(REGISTER_ADDRESS, JEXTSEL_BIT_OFFSET, JEXTSEL_BIT_WIDTH, value as u32); }

	const JL_BIT_OFFSET: u8 = 0;
	const JL_BIT_WIDTH: u8 = 2;
	/// JL (Width: 2, Offset: 0)
	pub fn get_jl() -> u8 { ::read(REGISTER_ADDRESS, JL_BIT_OFFSET, JL_BIT_WIDTH) as u8 }
	/// JL (Width: 2, Offset: 0)
	pub fn set_jl(value: u8) { ::write(REGISTER_ADDRESS, JL_BIT_OFFSET, JL_BIT_WIDTH, value as u32); }
}
/// offset register 1
/// Size: 0x20 bits
pub mod ofr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x60;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OFFSET1_EN_BIT_OFFSET: u8 = 31;
	const OFFSET1_EN_BIT_WIDTH: u8 = 1;
	/// OFFSET1_EN (Width: 1, Offset: 31)
	pub fn get_offset1_en() -> u8 { ::read(REGISTER_ADDRESS, OFFSET1_EN_BIT_OFFSET, OFFSET1_EN_BIT_WIDTH) as u8 }
	/// OFFSET1_EN (Width: 1, Offset: 31)
	pub fn set_offset1_en(value: u8) { ::write(REGISTER_ADDRESS, OFFSET1_EN_BIT_OFFSET, OFFSET1_EN_BIT_WIDTH, value as u32); }

	const OFFSET1_CH_BIT_OFFSET: u8 = 26;
	const OFFSET1_CH_BIT_WIDTH: u8 = 5;
	/// OFFSET1_CH (Width: 5, Offset: 26)
	pub fn get_offset1_ch() -> u8 { ::read(REGISTER_ADDRESS, OFFSET1_CH_BIT_OFFSET, OFFSET1_CH_BIT_WIDTH) as u8 }
	/// OFFSET1_CH (Width: 5, Offset: 26)
	pub fn set_offset1_ch(value: u8) { ::write(REGISTER_ADDRESS, OFFSET1_CH_BIT_OFFSET, OFFSET1_CH_BIT_WIDTH, value as u32); }

	const OFFSET1_BIT_OFFSET: u8 = 0;
	const OFFSET1_BIT_WIDTH: u8 = 12;
	/// OFFSET1 (Width: 12, Offset: 0)
	pub fn get_offset1() -> u16 { ::read(REGISTER_ADDRESS, OFFSET1_BIT_OFFSET, OFFSET1_BIT_WIDTH) as u16 }
	/// OFFSET1 (Width: 12, Offset: 0)
	pub fn set_offset1(value: u16) { ::write(REGISTER_ADDRESS, OFFSET1_BIT_OFFSET, OFFSET1_BIT_WIDTH, value as u32); }
}
/// offset register 2
/// Size: 0x20 bits
pub mod ofr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x64;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OFFSET2_EN_BIT_OFFSET: u8 = 31;
	const OFFSET2_EN_BIT_WIDTH: u8 = 1;
	/// OFFSET2_EN (Width: 1, Offset: 31)
	pub fn get_offset2_en() -> u8 { ::read(REGISTER_ADDRESS, OFFSET2_EN_BIT_OFFSET, OFFSET2_EN_BIT_WIDTH) as u8 }
	/// OFFSET2_EN (Width: 1, Offset: 31)
	pub fn set_offset2_en(value: u8) { ::write(REGISTER_ADDRESS, OFFSET2_EN_BIT_OFFSET, OFFSET2_EN_BIT_WIDTH, value as u32); }

	const OFFSET2_CH_BIT_OFFSET: u8 = 26;
	const OFFSET2_CH_BIT_WIDTH: u8 = 5;
	/// OFFSET2_CH (Width: 5, Offset: 26)
	pub fn get_offset2_ch() -> u8 { ::read(REGISTER_ADDRESS, OFFSET2_CH_BIT_OFFSET, OFFSET2_CH_BIT_WIDTH) as u8 }
	/// OFFSET2_CH (Width: 5, Offset: 26)
	pub fn set_offset2_ch(value: u8) { ::write(REGISTER_ADDRESS, OFFSET2_CH_BIT_OFFSET, OFFSET2_CH_BIT_WIDTH, value as u32); }

	const OFFSET2_BIT_OFFSET: u8 = 0;
	const OFFSET2_BIT_WIDTH: u8 = 12;
	/// OFFSET2 (Width: 12, Offset: 0)
	pub fn get_offset2() -> u16 { ::read(REGISTER_ADDRESS, OFFSET2_BIT_OFFSET, OFFSET2_BIT_WIDTH) as u16 }
	/// OFFSET2 (Width: 12, Offset: 0)
	pub fn set_offset2(value: u16) { ::write(REGISTER_ADDRESS, OFFSET2_BIT_OFFSET, OFFSET2_BIT_WIDTH, value as u32); }
}
/// offset register 3
/// Size: 0x20 bits
pub mod ofr3 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x68;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OFFSET3_EN_BIT_OFFSET: u8 = 31;
	const OFFSET3_EN_BIT_WIDTH: u8 = 1;
	/// OFFSET3_EN (Width: 1, Offset: 31)
	pub fn get_offset3_en() -> u8 { ::read(REGISTER_ADDRESS, OFFSET3_EN_BIT_OFFSET, OFFSET3_EN_BIT_WIDTH) as u8 }
	/// OFFSET3_EN (Width: 1, Offset: 31)
	pub fn set_offset3_en(value: u8) { ::write(REGISTER_ADDRESS, OFFSET3_EN_BIT_OFFSET, OFFSET3_EN_BIT_WIDTH, value as u32); }

	const OFFSET3_CH_BIT_OFFSET: u8 = 26;
	const OFFSET3_CH_BIT_WIDTH: u8 = 5;
	/// OFFSET3_CH (Width: 5, Offset: 26)
	pub fn get_offset3_ch() -> u8 { ::read(REGISTER_ADDRESS, OFFSET3_CH_BIT_OFFSET, OFFSET3_CH_BIT_WIDTH) as u8 }
	/// OFFSET3_CH (Width: 5, Offset: 26)
	pub fn set_offset3_ch(value: u8) { ::write(REGISTER_ADDRESS, OFFSET3_CH_BIT_OFFSET, OFFSET3_CH_BIT_WIDTH, value as u32); }

	const OFFSET3_BIT_OFFSET: u8 = 0;
	const OFFSET3_BIT_WIDTH: u8 = 12;
	/// OFFSET3 (Width: 12, Offset: 0)
	pub fn get_offset3() -> u16 { ::read(REGISTER_ADDRESS, OFFSET3_BIT_OFFSET, OFFSET3_BIT_WIDTH) as u16 }
	/// OFFSET3 (Width: 12, Offset: 0)
	pub fn set_offset3(value: u16) { ::write(REGISTER_ADDRESS, OFFSET3_BIT_OFFSET, OFFSET3_BIT_WIDTH, value as u32); }
}
/// offset register 4
/// Size: 0x20 bits
pub mod ofr4 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x6C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OFFSET4_EN_BIT_OFFSET: u8 = 31;
	const OFFSET4_EN_BIT_WIDTH: u8 = 1;
	/// OFFSET4_EN (Width: 1, Offset: 31)
	pub fn get_offset4_en() -> u8 { ::read(REGISTER_ADDRESS, OFFSET4_EN_BIT_OFFSET, OFFSET4_EN_BIT_WIDTH) as u8 }
	/// OFFSET4_EN (Width: 1, Offset: 31)
	pub fn set_offset4_en(value: u8) { ::write(REGISTER_ADDRESS, OFFSET4_EN_BIT_OFFSET, OFFSET4_EN_BIT_WIDTH, value as u32); }

	const OFFSET4_CH_BIT_OFFSET: u8 = 26;
	const OFFSET4_CH_BIT_WIDTH: u8 = 5;
	/// OFFSET4_CH (Width: 5, Offset: 26)
	pub fn get_offset4_ch() -> u8 { ::read(REGISTER_ADDRESS, OFFSET4_CH_BIT_OFFSET, OFFSET4_CH_BIT_WIDTH) as u8 }
	/// OFFSET4_CH (Width: 5, Offset: 26)
	pub fn set_offset4_ch(value: u8) { ::write(REGISTER_ADDRESS, OFFSET4_CH_BIT_OFFSET, OFFSET4_CH_BIT_WIDTH, value as u32); }

	const OFFSET4_BIT_OFFSET: u8 = 0;
	const OFFSET4_BIT_WIDTH: u8 = 12;
	/// OFFSET4 (Width: 12, Offset: 0)
	pub fn get_offset4() -> u16 { ::read(REGISTER_ADDRESS, OFFSET4_BIT_OFFSET, OFFSET4_BIT_WIDTH) as u16 }
	/// OFFSET4 (Width: 12, Offset: 0)
	pub fn set_offset4(value: u16) { ::write(REGISTER_ADDRESS, OFFSET4_BIT_OFFSET, OFFSET4_BIT_WIDTH, value as u32); }
}
/// injected data register 1
/// Size: 0x20 bits
pub mod jdr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x80;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const JDATA1_BIT_OFFSET: u8 = 0;
	const JDATA1_BIT_WIDTH: u8 = 16;
	/// JDATA1 (Width: 16, Offset: 0)
	pub fn get_jdata1() -> u16 { ::read(REGISTER_ADDRESS, JDATA1_BIT_OFFSET, JDATA1_BIT_WIDTH) as u16 }
}
/// injected data register 2
/// Size: 0x20 bits
pub mod jdr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x84;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const JDATA2_BIT_OFFSET: u8 = 0;
	const JDATA2_BIT_WIDTH: u8 = 16;
	/// JDATA2 (Width: 16, Offset: 0)
	pub fn get_jdata2() -> u16 { ::read(REGISTER_ADDRESS, JDATA2_BIT_OFFSET, JDATA2_BIT_WIDTH) as u16 }
}
/// injected data register 3
/// Size: 0x20 bits
pub mod jdr3 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x88;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const JDATA3_BIT_OFFSET: u8 = 0;
	const JDATA3_BIT_WIDTH: u8 = 16;
	/// JDATA3 (Width: 16, Offset: 0)
	pub fn get_jdata3() -> u16 { ::read(REGISTER_ADDRESS, JDATA3_BIT_OFFSET, JDATA3_BIT_WIDTH) as u16 }
}
/// injected data register 4
/// Size: 0x20 bits
pub mod jdr4 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const JDATA4_BIT_OFFSET: u8 = 0;
	const JDATA4_BIT_WIDTH: u8 = 16;
	/// JDATA4 (Width: 16, Offset: 0)
	pub fn get_jdata4() -> u16 { ::read(REGISTER_ADDRESS, JDATA4_BIT_OFFSET, JDATA4_BIT_WIDTH) as u16 }
}
/// Analog Watchdog 2 Configuration Register
/// Size: 0x20 bits
pub mod awd2cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xA0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const AWD2CH_BIT_OFFSET: u8 = 1;
	const AWD2CH_BIT_WIDTH: u8 = 18;
	/// AWD2CH (Width: 18, Offset: 1)
	pub fn get_awd2ch() -> u32 { ::read(REGISTER_ADDRESS, AWD2CH_BIT_OFFSET, AWD2CH_BIT_WIDTH) as u32 }
	/// AWD2CH (Width: 18, Offset: 1)
	pub fn set_awd2ch(value: u32) { ::write(REGISTER_ADDRESS, AWD2CH_BIT_OFFSET, AWD2CH_BIT_WIDTH, value as u32); }
}
/// Analog Watchdog 3 Configuration Register
/// Size: 0x20 bits
pub mod awd3cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xA4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const AWD3CH_BIT_OFFSET: u8 = 1;
	const AWD3CH_BIT_WIDTH: u8 = 18;
	/// AWD3CH (Width: 18, Offset: 1)
	pub fn get_awd3ch() -> u32 { ::read(REGISTER_ADDRESS, AWD3CH_BIT_OFFSET, AWD3CH_BIT_WIDTH) as u32 }
	/// AWD3CH (Width: 18, Offset: 1)
	pub fn set_awd3ch(value: u32) { ::write(REGISTER_ADDRESS, AWD3CH_BIT_OFFSET, AWD3CH_BIT_WIDTH, value as u32); }
}
/// Differential Mode Selection Register 2
/// Size: 0x20 bits
pub mod difsel {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xB0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DIFSEL_1_15_BIT_OFFSET: u8 = 1;
	const DIFSEL_1_15_BIT_WIDTH: u8 = 15;
	/// Differential mode for channels 15 to 1 (Width: 15, Offset: 1)
	pub fn get_difsel_1_15() -> u16 { ::read(REGISTER_ADDRESS, DIFSEL_1_15_BIT_OFFSET, DIFSEL_1_15_BIT_WIDTH) as u16 }
	/// Differential mode for channels 15 to 1 (Width: 15, Offset: 1)
	pub fn set_difsel_1_15(value: u16) { ::write(REGISTER_ADDRESS, DIFSEL_1_15_BIT_OFFSET, DIFSEL_1_15_BIT_WIDTH, value as u32); }

	const DIFSEL_16_18_BIT_OFFSET: u8 = 16;
	const DIFSEL_16_18_BIT_WIDTH: u8 = 3;
	/// Differential mode for channels 18 to 16 (Width: 3, Offset: 16)
	pub fn get_difsel_16_18() -> u8 { ::read(REGISTER_ADDRESS, DIFSEL_16_18_BIT_OFFSET, DIFSEL_16_18_BIT_WIDTH) as u8 }
}
/// Calibration Factors
/// Size: 0x20 bits
pub mod calfact {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xB4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CALFACT_D_BIT_OFFSET: u8 = 16;
	const CALFACT_D_BIT_WIDTH: u8 = 7;
	/// CALFACT_D (Width: 7, Offset: 16)
	pub fn get_calfact_d() -> u8 { ::read(REGISTER_ADDRESS, CALFACT_D_BIT_OFFSET, CALFACT_D_BIT_WIDTH) as u8 }
	/// CALFACT_D (Width: 7, Offset: 16)
	pub fn set_calfact_d(value: u8) { ::write(REGISTER_ADDRESS, CALFACT_D_BIT_OFFSET, CALFACT_D_BIT_WIDTH, value as u32); }

	const CALFACT_S_BIT_OFFSET: u8 = 0;
	const CALFACT_S_BIT_WIDTH: u8 = 7;
	/// CALFACT_S (Width: 7, Offset: 0)
	pub fn get_calfact_s() -> u8 { ::read(REGISTER_ADDRESS, CALFACT_S_BIT_OFFSET, CALFACT_S_BIT_WIDTH) as u8 }
	/// CALFACT_S (Width: 7, Offset: 0)
	pub fn set_calfact_s(value: u8) { ::write(REGISTER_ADDRESS, CALFACT_S_BIT_OFFSET, CALFACT_S_BIT_WIDTH, value as u32); }
}
/// ADC1 and ADC2 global interrupt
pub const INTERRUPT_ADC1_2: u32 = 18;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>ADC1</name>
  <description>Analog-to-Digital Converter</description>
  <groupName>ADC</groupName>
  <baseAddress>0x50000000</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x100</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>ISR</name>
      <displayName>ISR</displayName>
      <description>interrupt and status register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>JQOVF</name>
          <description>JQOVF</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AWD3</name>
          <description>AWD3</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AWD2</name>
          <description>AWD2</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AWD1</name>
          <description>AWD1</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JEOS</name>
          <description>JEOS</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JEOC</name>
          <description>JEOC</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OVR</name>
          <description>OVR</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOS</name>
          <description>EOS</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOC</name>
          <description>EOC</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOSMP</name>
          <description>EOSMP</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADRDY</name>
          <description>ADRDY</description>
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
          <name>JQOVFIE</name>
          <description>JQOVFIE</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AWD3IE</name>
          <description>AWD3IE</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AWD2IE</name>
          <description>AWD2IE</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AWD1IE</name>
          <description>AWD1IE</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JEOSIE</name>
          <description>JEOSIE</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JEOCIE</name>
          <description>JEOCIE</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OVRIE</name>
          <description>OVRIE</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOSIE</name>
          <description>EOSIE</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOCIE</name>
          <description>EOCIE</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EOSMPIE</name>
          <description>EOSMPIE</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADRDYIE</name>
          <description>ADRDYIE</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CR</name>
      <displayName>CR</displayName>
      <description>control register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>ADCAL</name>
          <description>ADCAL</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADCALDIF</name>
          <description>ADCALDIF</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DEEPPWD</name>
          <description>DEEPPWD</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADVREGEN</name>
          <description>ADVREGEN</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JADSTP</name>
          <description>JADSTP</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADSTP</name>
          <description>ADSTP</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JADSTART</name>
          <description>JADSTART</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADSTART</name>
          <description>ADSTART</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADDIS</name>
          <description>ADDIS</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADEN</name>
          <description>ADEN</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CFGR</name>
      <displayName>CFGR</displayName>
      <description>configuration register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>AWDCH1CH</name>
          <description>AWDCH1CH</description>
          <bitOffset>26</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>JAUTO</name>
          <description>JAUTO</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JAWD1EN</name>
          <description>JAWD1EN</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AWD1EN</name>
          <description>AWD1EN</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AWD1SGL</name>
          <description>AWD1SGL</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JQM</name>
          <description>JQM</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>JDISCEN</name>
          <description>JDISCEN</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DISCNUM</name>
          <description>DISCNUM</description>
          <bitOffset>17</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>DISCEN</name>
          <description>DISCEN</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AUTOFF</name>
          <description>AUTOFF</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>AUTDLY</name>
          <description>AUTDLY</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CONT</name>
          <description>CONT</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OVRMOD</name>
          <description>OVRMOD</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EXTEN</name>
          <description>EXTEN</description>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>EXTSEL</name>
          <description>EXTSEL</description>
          <bitOffset>6</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>ALIGN</name>
          <description>ALIGN</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RES</name>
          <description>RES</description>
          <bitOffset>3</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>DMACFG</name>
          <description>DMACFG</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DMAEN</name>
          <description>DMAEN</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SMPR1</name>
      <displayName>SMPR1</displayName>
      <description>sample time register 1</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SMP9</name>
          <description>SMP9</description>
          <bitOffset>27</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP8</name>
          <description>SMP8</description>
          <bitOffset>24</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP7</name>
          <description>SMP7</description>
          <bitOffset>21</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP6</name>
          <description>SMP6</description>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP5</name>
          <description>SMP5</description>
          <bitOffset>15</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP4</name>
          <description>SMP4</description>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP3</name>
          <description>SMP3</description>
          <bitOffset>9</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP2</name>
          <description>SMP2</description>
          <bitOffset>6</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP1</name>
          <description>SMP1</description>
          <bitOffset>3</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SMPR2</name>
      <displayName>SMPR2</displayName>
      <description>sample time register 2</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SMP18</name>
          <description>SMP18</description>
          <bitOffset>24</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP17</name>
          <description>SMP17</description>
          <bitOffset>21</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP16</name>
          <description>SMP16</description>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP15</name>
          <description>SMP15</description>
          <bitOffset>15</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP14</name>
          <description>SMP14</description>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP13</name>
          <description>SMP13</description>
          <bitOffset>9</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP12</name>
          <description>SMP12</description>
          <bitOffset>6</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP11</name>
          <description>SMP11</description>
          <bitOffset>3</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SMP10</name>
          <description>SMP10</description>
          <bitOffset>0</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>TR1</name>
      <displayName>TR1</displayName>
      <description>watchdog threshold register 1</description>
      <addressOffset>0x20</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0FFF0000</resetValue>
      <fields>
        <field>
          <name>HT1</name>
          <description>HT1</description>
          <bitOffset>16</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
        <field>
          <name>LT1</name>
          <description>LT1</description>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>TR2</name>
      <displayName>TR2</displayName>
      <description>watchdog threshold register</description>
      <addressOffset>0x24</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0FFF0000</resetValue>
      <fields>
        <field>
          <name>HT2</name>
          <description>HT2</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>LT2</name>
          <description>LT2</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>TR3</name>
      <displayName>TR3</displayName>
      <description>watchdog threshold register 3</description>
      <addressOffset>0x28</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0FFF0000</resetValue>
      <fields>
        <field>
          <name>HT3</name>
          <description>HT3</description>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
        <field>
          <name>LT3</name>
          <description>LT3</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SQR1</name>
      <displayName>SQR1</displayName>
      <description>regular sequence register 1</description>
      <addressOffset>0x30</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SQ4</name>
          <description>SQ4</description>
          <bitOffset>24</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>SQ3</name>
          <description>SQ3</description>
          <bitOffset>18</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>SQ2</name>
          <description>SQ2</description>
          <bitOffset>12</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>SQ1</name>
          <description>SQ1</description>
          <bitOffset>6</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>L3</name>
          <description>L3</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SQR2</name>
      <displayName>SQR2</displayName>
      <description>regular sequence register 2</description>
      <addressOffset>0x34</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SQ9</name>
          <description>SQ9</description>
          <bitOffset>24</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>SQ8</name>
          <description>SQ8</description>
          <bitOffset>18</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>SQ7</name>
          <description>SQ7</description>
          <bitOffset>12</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>SQ6</name>
          <description>SQ6</description>
          <bitOffset>6</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>SQ5</name>
          <description>SQ5</description>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SQR3</name>
      <displayName>SQR3</displayName>
      <description>regular sequence register 3</description>
      <addressOffset>0x38</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SQ14</name>
          <description>SQ14</description>
          <bitOffset>24</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>SQ13</name>
          <description>SQ13</description>
          <bitOffset>18</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>SQ12</name>
          <description>SQ12</description>
          <bitOffset>12</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>SQ11</name>
          <description>SQ11</description>
          <bitOffset>6</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>SQ10</name>
          <description>SQ10</description>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SQR4</name>
      <displayName>SQR4</displayName>
      <description>regular sequence register 4</description>
      <addressOffset>0x3C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SQ16</name>
          <description>SQ16</description>
          <bitOffset>6</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>SQ15</name>
          <description>SQ15</description>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DR</name>
      <displayName>DR</displayName>
      <description>regular Data Register</description>
      <addressOffset>0x40</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>regularDATA</name>
          <description>regularDATA</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>JSQR</name>
      <displayName>JSQR</displayName>
      <description>injected sequence register</description>
      <addressOffset>0x4C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>JSQ4</name>
          <description>JSQ4</description>
          <bitOffset>26</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>JSQ3</name>
          <description>JSQ3</description>
          <bitOffset>20</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>JSQ2</name>
          <description>JSQ2</description>
          <bitOffset>14</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>JSQ1</name>
          <description>JSQ1</description>
          <bitOffset>8</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>JEXTEN</name>
          <description>JEXTEN</description>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>JEXTSEL</name>
          <description>JEXTSEL</description>
          <bitOffset>2</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>JL</name>
          <description>JL</description>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>OFR1</name>
      <displayName>OFR1</displayName>
      <description>offset register 1</description>
      <addressOffset>0x60</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OFFSET1_EN</name>
          <description>OFFSET1_EN</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OFFSET1_CH</name>
          <description>OFFSET1_CH</description>
          <bitOffset>26</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>OFFSET1</name>
          <description>OFFSET1</description>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>OFR2</name>
      <displayName>OFR2</displayName>
      <description>offset register 2</description>
      <addressOffset>0x64</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OFFSET2_EN</name>
          <description>OFFSET2_EN</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OFFSET2_CH</name>
          <description>OFFSET2_CH</description>
          <bitOffset>26</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>OFFSET2</name>
          <description>OFFSET2</description>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>OFR3</name>
      <displayName>OFR3</displayName>
      <description>offset register 3</description>
      <addressOffset>0x68</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OFFSET3_EN</name>
          <description>OFFSET3_EN</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OFFSET3_CH</name>
          <description>OFFSET3_CH</description>
          <bitOffset>26</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>OFFSET3</name>
          <description>OFFSET3</description>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>OFR4</name>
      <displayName>OFR4</displayName>
      <description>offset register 4</description>
      <addressOffset>0x6C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OFFSET4_EN</name>
          <description>OFFSET4_EN</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OFFSET4_CH</name>
          <description>OFFSET4_CH</description>
          <bitOffset>26</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>OFFSET4</name>
          <description>OFFSET4</description>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>JDR1</name>
      <displayName>JDR1</displayName>
      <description>injected data register 1</description>
      <addressOffset>0x80</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>JDATA1</name>
          <description>JDATA1</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>JDR2</name>
      <displayName>JDR2</displayName>
      <description>injected data register 2</description>
      <addressOffset>0x84</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>JDATA2</name>
          <description>JDATA2</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>JDR3</name>
      <displayName>JDR3</displayName>
      <description>injected data register 3</description>
      <addressOffset>0x88</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>JDATA3</name>
          <description>JDATA3</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>JDR4</name>
      <displayName>JDR4</displayName>
      <description>injected data register 4</description>
      <addressOffset>0x8C</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>JDATA4</name>
          <description>JDATA4</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>AWD2CR</name>
      <displayName>AWD2CR</displayName>
      <description>Analog Watchdog 2 Configuration
          Register</description>
      <addressOffset>0xA0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>AWD2CH</name>
          <description>AWD2CH</description>
          <bitOffset>1</bitOffset>
          <bitWidth>18</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>AWD3CR</name>
      <displayName>AWD3CR</displayName>
      <description>Analog Watchdog 3 Configuration
          Register</description>
      <addressOffset>0xA4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>AWD3CH</name>
          <description>AWD3CH</description>
          <bitOffset>1</bitOffset>
          <bitWidth>18</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DIFSEL</name>
      <displayName>DIFSEL</displayName>
      <description>Differential Mode Selection Register
          2</description>
      <addressOffset>0xB0</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>DIFSEL_1_15</name>
          <description>Differential mode for channels 15 to
              1</description>
          <bitOffset>1</bitOffset>
          <bitWidth>15</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DIFSEL_16_18</name>
          <description>Differential mode for channels 18 to
              16</description>
          <bitOffset>16</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-only</access>
        </field>
      </fields>
    </register>
    <register>
      <name>CALFACT</name>
      <displayName>CALFACT</displayName>
      <description>Calibration Factors</description>
      <addressOffset>0xB4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CALFACT_D</name>
          <description>CALFACT_D</description>
          <bitOffset>16</bitOffset>
          <bitWidth>7</bitWidth>
        </field>
        <field>
          <name>CALFACT_S</name>
          <description>CALFACT_S</description>
          <bitOffset>0</bitOffset>
          <bitWidth>7</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>ADC1_2</name>
    <description>ADC1 and ADC2 global interrupt</description>
    <value>18</value>
  </interrupt>
</peripheral>*/
