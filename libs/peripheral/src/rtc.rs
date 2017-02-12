/// MOD RTC
/// Real-time clock
const BASE_ADDRESS: u32 = 0x40002800;
/// time register
/// Size: 0x20 bits
pub mod tr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PM_BIT_OFFSET: u8 = 22;
	const PM_BIT_WIDTH: u8 = 1;
	/// AM/PM notation (Width: 1, Offset: 22)
	pub fn get_pm() -> u8 { ::read(REGISTER_ADDRESS, PM_BIT_OFFSET, PM_BIT_WIDTH) as u8 }
	/// AM/PM notation (Width: 1, Offset: 22)
	pub fn set_pm(value: u8) { ::write(REGISTER_ADDRESS, PM_BIT_OFFSET, PM_BIT_WIDTH, value as u32); }

	const HT_BIT_OFFSET: u8 = 20;
	const HT_BIT_WIDTH: u8 = 2;
	/// Hour tens in BCD format (Width: 2, Offset: 20)
	pub fn get_ht() -> u8 { ::read(REGISTER_ADDRESS, HT_BIT_OFFSET, HT_BIT_WIDTH) as u8 }
	/// Hour tens in BCD format (Width: 2, Offset: 20)
	pub fn set_ht(value: u8) { ::write(REGISTER_ADDRESS, HT_BIT_OFFSET, HT_BIT_WIDTH, value as u32); }

	const HU_BIT_OFFSET: u8 = 16;
	const HU_BIT_WIDTH: u8 = 4;
	/// Hour units in BCD format (Width: 4, Offset: 16)
	pub fn get_hu() -> u8 { ::read(REGISTER_ADDRESS, HU_BIT_OFFSET, HU_BIT_WIDTH) as u8 }
	/// Hour units in BCD format (Width: 4, Offset: 16)
	pub fn set_hu(value: u8) { ::write(REGISTER_ADDRESS, HU_BIT_OFFSET, HU_BIT_WIDTH, value as u32); }

	const MNT_BIT_OFFSET: u8 = 12;
	const MNT_BIT_WIDTH: u8 = 3;
	/// Minute tens in BCD format (Width: 3, Offset: 12)
	pub fn get_mnt() -> u8 { ::read(REGISTER_ADDRESS, MNT_BIT_OFFSET, MNT_BIT_WIDTH) as u8 }
	/// Minute tens in BCD format (Width: 3, Offset: 12)
	pub fn set_mnt(value: u8) { ::write(REGISTER_ADDRESS, MNT_BIT_OFFSET, MNT_BIT_WIDTH, value as u32); }

	const MNU_BIT_OFFSET: u8 = 8;
	const MNU_BIT_WIDTH: u8 = 4;
	/// Minute units in BCD format (Width: 4, Offset: 8)
	pub fn get_mnu() -> u8 { ::read(REGISTER_ADDRESS, MNU_BIT_OFFSET, MNU_BIT_WIDTH) as u8 }
	/// Minute units in BCD format (Width: 4, Offset: 8)
	pub fn set_mnu(value: u8) { ::write(REGISTER_ADDRESS, MNU_BIT_OFFSET, MNU_BIT_WIDTH, value as u32); }

	const ST_BIT_OFFSET: u8 = 4;
	const ST_BIT_WIDTH: u8 = 3;
	/// Second tens in BCD format (Width: 3, Offset: 4)
	pub fn get_st() -> u8 { ::read(REGISTER_ADDRESS, ST_BIT_OFFSET, ST_BIT_WIDTH) as u8 }
	/// Second tens in BCD format (Width: 3, Offset: 4)
	pub fn set_st(value: u8) { ::write(REGISTER_ADDRESS, ST_BIT_OFFSET, ST_BIT_WIDTH, value as u32); }

	const SU_BIT_OFFSET: u8 = 0;
	const SU_BIT_WIDTH: u8 = 4;
	/// Second units in BCD format (Width: 4, Offset: 0)
	pub fn get_su() -> u8 { ::read(REGISTER_ADDRESS, SU_BIT_OFFSET, SU_BIT_WIDTH) as u8 }
	/// Second units in BCD format (Width: 4, Offset: 0)
	pub fn set_su(value: u8) { ::write(REGISTER_ADDRESS, SU_BIT_OFFSET, SU_BIT_WIDTH, value as u32); }
}
/// date register
/// Size: 0x20 bits
pub mod dr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const YT_BIT_OFFSET: u8 = 20;
	const YT_BIT_WIDTH: u8 = 4;
	/// Year tens in BCD format (Width: 4, Offset: 20)
	pub fn get_yt() -> u8 { ::read(REGISTER_ADDRESS, YT_BIT_OFFSET, YT_BIT_WIDTH) as u8 }
	/// Year tens in BCD format (Width: 4, Offset: 20)
	pub fn set_yt(value: u8) { ::write(REGISTER_ADDRESS, YT_BIT_OFFSET, YT_BIT_WIDTH, value as u32); }

	const YU_BIT_OFFSET: u8 = 16;
	const YU_BIT_WIDTH: u8 = 4;
	/// Year units in BCD format (Width: 4, Offset: 16)
	pub fn get_yu() -> u8 { ::read(REGISTER_ADDRESS, YU_BIT_OFFSET, YU_BIT_WIDTH) as u8 }
	/// Year units in BCD format (Width: 4, Offset: 16)
	pub fn set_yu(value: u8) { ::write(REGISTER_ADDRESS, YU_BIT_OFFSET, YU_BIT_WIDTH, value as u32); }

	const WDU_BIT_OFFSET: u8 = 13;
	const WDU_BIT_WIDTH: u8 = 3;
	/// Week day units (Width: 3, Offset: 13)
	pub fn get_wdu() -> u8 { ::read(REGISTER_ADDRESS, WDU_BIT_OFFSET, WDU_BIT_WIDTH) as u8 }
	/// Week day units (Width: 3, Offset: 13)
	pub fn set_wdu(value: u8) { ::write(REGISTER_ADDRESS, WDU_BIT_OFFSET, WDU_BIT_WIDTH, value as u32); }

	const MT_BIT_OFFSET: u8 = 12;
	const MT_BIT_WIDTH: u8 = 1;
	/// Month tens in BCD format (Width: 1, Offset: 12)
	pub fn get_mt() -> u8 { ::read(REGISTER_ADDRESS, MT_BIT_OFFSET, MT_BIT_WIDTH) as u8 }
	/// Month tens in BCD format (Width: 1, Offset: 12)
	pub fn set_mt(value: u8) { ::write(REGISTER_ADDRESS, MT_BIT_OFFSET, MT_BIT_WIDTH, value as u32); }

	const MU_BIT_OFFSET: u8 = 8;
	const MU_BIT_WIDTH: u8 = 4;
	/// Month units in BCD format (Width: 4, Offset: 8)
	pub fn get_mu() -> u8 { ::read(REGISTER_ADDRESS, MU_BIT_OFFSET, MU_BIT_WIDTH) as u8 }
	/// Month units in BCD format (Width: 4, Offset: 8)
	pub fn set_mu(value: u8) { ::write(REGISTER_ADDRESS, MU_BIT_OFFSET, MU_BIT_WIDTH, value as u32); }

	const DT_BIT_OFFSET: u8 = 4;
	const DT_BIT_WIDTH: u8 = 2;
	/// Date tens in BCD format (Width: 2, Offset: 4)
	pub fn get_dt() -> u8 { ::read(REGISTER_ADDRESS, DT_BIT_OFFSET, DT_BIT_WIDTH) as u8 }
	/// Date tens in BCD format (Width: 2, Offset: 4)
	pub fn set_dt(value: u8) { ::write(REGISTER_ADDRESS, DT_BIT_OFFSET, DT_BIT_WIDTH, value as u32); }

	const DU_BIT_OFFSET: u8 = 0;
	const DU_BIT_WIDTH: u8 = 4;
	/// Date units in BCD format (Width: 4, Offset: 0)
	pub fn get_du() -> u8 { ::read(REGISTER_ADDRESS, DU_BIT_OFFSET, DU_BIT_WIDTH) as u8 }
	/// Date units in BCD format (Width: 4, Offset: 0)
	pub fn set_du(value: u8) { ::write(REGISTER_ADDRESS, DU_BIT_OFFSET, DU_BIT_WIDTH, value as u32); }
}
/// control register
/// Size: 0x20 bits
pub mod cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const WCKSEL_BIT_OFFSET: u8 = 0;
	const WCKSEL_BIT_WIDTH: u8 = 3;
	/// Wakeup clock selection (Width: 3, Offset: 0)
	pub fn get_wcksel() -> u8 { ::read(REGISTER_ADDRESS, WCKSEL_BIT_OFFSET, WCKSEL_BIT_WIDTH) as u8 }
	/// Wakeup clock selection (Width: 3, Offset: 0)
	pub fn set_wcksel(value: u8) { ::write(REGISTER_ADDRESS, WCKSEL_BIT_OFFSET, WCKSEL_BIT_WIDTH, value as u32); }

	const TSEDGE_BIT_OFFSET: u8 = 3;
	const TSEDGE_BIT_WIDTH: u8 = 1;
	/// Time-stamp event active edge (Width: 1, Offset: 3)
	pub fn get_tsedge() -> u8 { ::read(REGISTER_ADDRESS, TSEDGE_BIT_OFFSET, TSEDGE_BIT_WIDTH) as u8 }
	/// Time-stamp event active edge (Width: 1, Offset: 3)
	pub fn set_tsedge(value: u8) { ::write(REGISTER_ADDRESS, TSEDGE_BIT_OFFSET, TSEDGE_BIT_WIDTH, value as u32); }

	const REFCKON_BIT_OFFSET: u8 = 4;
	const REFCKON_BIT_WIDTH: u8 = 1;
	/// Reference clock detection enable (50 or 60 Hz) (Width: 1, Offset: 4)
	pub fn get_refckon() -> u8 { ::read(REGISTER_ADDRESS, REFCKON_BIT_OFFSET, REFCKON_BIT_WIDTH) as u8 }
	/// Reference clock detection enable (50 or 60 Hz) (Width: 1, Offset: 4)
	pub fn set_refckon(value: u8) { ::write(REGISTER_ADDRESS, REFCKON_BIT_OFFSET, REFCKON_BIT_WIDTH, value as u32); }

	const BYPSHAD_BIT_OFFSET: u8 = 5;
	const BYPSHAD_BIT_WIDTH: u8 = 1;
	/// Bypass the shadow registers (Width: 1, Offset: 5)
	pub fn get_bypshad() -> u8 { ::read(REGISTER_ADDRESS, BYPSHAD_BIT_OFFSET, BYPSHAD_BIT_WIDTH) as u8 }
	/// Bypass the shadow registers (Width: 1, Offset: 5)
	pub fn set_bypshad(value: u8) { ::write(REGISTER_ADDRESS, BYPSHAD_BIT_OFFSET, BYPSHAD_BIT_WIDTH, value as u32); }

	const FMT_BIT_OFFSET: u8 = 6;
	const FMT_BIT_WIDTH: u8 = 1;
	/// Hour format (Width: 1, Offset: 6)
	pub fn get_fmt() -> u8 { ::read(REGISTER_ADDRESS, FMT_BIT_OFFSET, FMT_BIT_WIDTH) as u8 }
	/// Hour format (Width: 1, Offset: 6)
	pub fn set_fmt(value: u8) { ::write(REGISTER_ADDRESS, FMT_BIT_OFFSET, FMT_BIT_WIDTH, value as u32); }

	const ALRAE_BIT_OFFSET: u8 = 8;
	const ALRAE_BIT_WIDTH: u8 = 1;
	/// Alarm A enable (Width: 1, Offset: 8)
	pub fn get_alrae() -> u8 { ::read(REGISTER_ADDRESS, ALRAE_BIT_OFFSET, ALRAE_BIT_WIDTH) as u8 }
	/// Alarm A enable (Width: 1, Offset: 8)
	pub fn set_alrae(value: u8) { ::write(REGISTER_ADDRESS, ALRAE_BIT_OFFSET, ALRAE_BIT_WIDTH, value as u32); }

	const ALRBE_BIT_OFFSET: u8 = 9;
	const ALRBE_BIT_WIDTH: u8 = 1;
	/// Alarm B enable (Width: 1, Offset: 9)
	pub fn get_alrbe() -> u8 { ::read(REGISTER_ADDRESS, ALRBE_BIT_OFFSET, ALRBE_BIT_WIDTH) as u8 }
	/// Alarm B enable (Width: 1, Offset: 9)
	pub fn set_alrbe(value: u8) { ::write(REGISTER_ADDRESS, ALRBE_BIT_OFFSET, ALRBE_BIT_WIDTH, value as u32); }

	const WUTE_BIT_OFFSET: u8 = 10;
	const WUTE_BIT_WIDTH: u8 = 1;
	/// Wakeup timer enable (Width: 1, Offset: 10)
	pub fn get_wute() -> u8 { ::read(REGISTER_ADDRESS, WUTE_BIT_OFFSET, WUTE_BIT_WIDTH) as u8 }
	/// Wakeup timer enable (Width: 1, Offset: 10)
	pub fn set_wute(value: u8) { ::write(REGISTER_ADDRESS, WUTE_BIT_OFFSET, WUTE_BIT_WIDTH, value as u32); }

	const TSE_BIT_OFFSET: u8 = 11;
	const TSE_BIT_WIDTH: u8 = 1;
	/// Time stamp enable (Width: 1, Offset: 11)
	pub fn get_tse() -> u8 { ::read(REGISTER_ADDRESS, TSE_BIT_OFFSET, TSE_BIT_WIDTH) as u8 }
	/// Time stamp enable (Width: 1, Offset: 11)
	pub fn set_tse(value: u8) { ::write(REGISTER_ADDRESS, TSE_BIT_OFFSET, TSE_BIT_WIDTH, value as u32); }

	const ALRAIE_BIT_OFFSET: u8 = 12;
	const ALRAIE_BIT_WIDTH: u8 = 1;
	/// Alarm A interrupt enable (Width: 1, Offset: 12)
	pub fn get_alraie() -> u8 { ::read(REGISTER_ADDRESS, ALRAIE_BIT_OFFSET, ALRAIE_BIT_WIDTH) as u8 }
	/// Alarm A interrupt enable (Width: 1, Offset: 12)
	pub fn set_alraie(value: u8) { ::write(REGISTER_ADDRESS, ALRAIE_BIT_OFFSET, ALRAIE_BIT_WIDTH, value as u32); }

	const ALRBIE_BIT_OFFSET: u8 = 13;
	const ALRBIE_BIT_WIDTH: u8 = 1;
	/// Alarm B interrupt enable (Width: 1, Offset: 13)
	pub fn get_alrbie() -> u8 { ::read(REGISTER_ADDRESS, ALRBIE_BIT_OFFSET, ALRBIE_BIT_WIDTH) as u8 }
	/// Alarm B interrupt enable (Width: 1, Offset: 13)
	pub fn set_alrbie(value: u8) { ::write(REGISTER_ADDRESS, ALRBIE_BIT_OFFSET, ALRBIE_BIT_WIDTH, value as u32); }

	const WUTIE_BIT_OFFSET: u8 = 14;
	const WUTIE_BIT_WIDTH: u8 = 1;
	/// Wakeup timer interrupt enable (Width: 1, Offset: 14)
	pub fn get_wutie() -> u8 { ::read(REGISTER_ADDRESS, WUTIE_BIT_OFFSET, WUTIE_BIT_WIDTH) as u8 }
	/// Wakeup timer interrupt enable (Width: 1, Offset: 14)
	pub fn set_wutie(value: u8) { ::write(REGISTER_ADDRESS, WUTIE_BIT_OFFSET, WUTIE_BIT_WIDTH, value as u32); }

	const TSIE_BIT_OFFSET: u8 = 15;
	const TSIE_BIT_WIDTH: u8 = 1;
	/// Time-stamp interrupt enable (Width: 1, Offset: 15)
	pub fn get_tsie() -> u8 { ::read(REGISTER_ADDRESS, TSIE_BIT_OFFSET, TSIE_BIT_WIDTH) as u8 }
	/// Time-stamp interrupt enable (Width: 1, Offset: 15)
	pub fn set_tsie(value: u8) { ::write(REGISTER_ADDRESS, TSIE_BIT_OFFSET, TSIE_BIT_WIDTH, value as u32); }

	const ADD1H_BIT_OFFSET: u8 = 16;
	const ADD1H_BIT_WIDTH: u8 = 1;
	/// Add 1 hour (summer time change) (Width: 1, Offset: 16)
	pub fn get_add1h() -> u8 { ::read(REGISTER_ADDRESS, ADD1H_BIT_OFFSET, ADD1H_BIT_WIDTH) as u8 }
	/// Add 1 hour (summer time change) (Width: 1, Offset: 16)
	pub fn set_add1h(value: u8) { ::write(REGISTER_ADDRESS, ADD1H_BIT_OFFSET, ADD1H_BIT_WIDTH, value as u32); }

	const SUB1H_BIT_OFFSET: u8 = 17;
	const SUB1H_BIT_WIDTH: u8 = 1;
	/// Subtract 1 hour (winter time change) (Width: 1, Offset: 17)
	pub fn get_sub1h() -> u8 { ::read(REGISTER_ADDRESS, SUB1H_BIT_OFFSET, SUB1H_BIT_WIDTH) as u8 }
	/// Subtract 1 hour (winter time change) (Width: 1, Offset: 17)
	pub fn set_sub1h(value: u8) { ::write(REGISTER_ADDRESS, SUB1H_BIT_OFFSET, SUB1H_BIT_WIDTH, value as u32); }

	const BKP_BIT_OFFSET: u8 = 18;
	const BKP_BIT_WIDTH: u8 = 1;
	/// Backup (Width: 1, Offset: 18)
	pub fn get_bkp() -> u8 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u8 }
	/// Backup (Width: 1, Offset: 18)
	pub fn set_bkp(value: u8) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }

	const COSEL_BIT_OFFSET: u8 = 19;
	const COSEL_BIT_WIDTH: u8 = 1;
	/// Calibration output selection (Width: 1, Offset: 19)
	pub fn get_cosel() -> u8 { ::read(REGISTER_ADDRESS, COSEL_BIT_OFFSET, COSEL_BIT_WIDTH) as u8 }
	/// Calibration output selection (Width: 1, Offset: 19)
	pub fn set_cosel(value: u8) { ::write(REGISTER_ADDRESS, COSEL_BIT_OFFSET, COSEL_BIT_WIDTH, value as u32); }

	const POL_BIT_OFFSET: u8 = 20;
	const POL_BIT_WIDTH: u8 = 1;
	/// Output polarity (Width: 1, Offset: 20)
	pub fn get_pol() -> u8 { ::read(REGISTER_ADDRESS, POL_BIT_OFFSET, POL_BIT_WIDTH) as u8 }
	/// Output polarity (Width: 1, Offset: 20)
	pub fn set_pol(value: u8) { ::write(REGISTER_ADDRESS, POL_BIT_OFFSET, POL_BIT_WIDTH, value as u32); }

	const OSEL_BIT_OFFSET: u8 = 21;
	const OSEL_BIT_WIDTH: u8 = 2;
	/// Output selection (Width: 2, Offset: 21)
	pub fn get_osel() -> u8 { ::read(REGISTER_ADDRESS, OSEL_BIT_OFFSET, OSEL_BIT_WIDTH) as u8 }
	/// Output selection (Width: 2, Offset: 21)
	pub fn set_osel(value: u8) { ::write(REGISTER_ADDRESS, OSEL_BIT_OFFSET, OSEL_BIT_WIDTH, value as u32); }

	const COE_BIT_OFFSET: u8 = 23;
	const COE_BIT_WIDTH: u8 = 1;
	/// Calibration output enable (Width: 1, Offset: 23)
	pub fn get_coe() -> u8 { ::read(REGISTER_ADDRESS, COE_BIT_OFFSET, COE_BIT_WIDTH) as u8 }
	/// Calibration output enable (Width: 1, Offset: 23)
	pub fn set_coe(value: u8) { ::write(REGISTER_ADDRESS, COE_BIT_OFFSET, COE_BIT_WIDTH, value as u32); }
}
/// initialization and status register
/// Size: 0x20 bits
pub mod isr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ALRAWF_BIT_OFFSET: u8 = 0;
	const ALRAWF_BIT_WIDTH: u8 = 1;
	/// Alarm A write flag (Width: 1, Offset: 0)
	pub fn get_alrawf() -> u8 { ::read(REGISTER_ADDRESS, ALRAWF_BIT_OFFSET, ALRAWF_BIT_WIDTH) as u8 }

	const ALRBWF_BIT_OFFSET: u8 = 1;
	const ALRBWF_BIT_WIDTH: u8 = 1;
	/// Alarm B write flag (Width: 1, Offset: 1)
	pub fn get_alrbwf() -> u8 { ::read(REGISTER_ADDRESS, ALRBWF_BIT_OFFSET, ALRBWF_BIT_WIDTH) as u8 }

	const WUTWF_BIT_OFFSET: u8 = 2;
	const WUTWF_BIT_WIDTH: u8 = 1;
	/// Wakeup timer write flag (Width: 1, Offset: 2)
	pub fn get_wutwf() -> u8 { ::read(REGISTER_ADDRESS, WUTWF_BIT_OFFSET, WUTWF_BIT_WIDTH) as u8 }

	const SHPF_BIT_OFFSET: u8 = 3;
	const SHPF_BIT_WIDTH: u8 = 1;
	/// Shift operation pending (Width: 1, Offset: 3)
	pub fn get_shpf() -> u8 { ::read(REGISTER_ADDRESS, SHPF_BIT_OFFSET, SHPF_BIT_WIDTH) as u8 }
	/// Shift operation pending (Width: 1, Offset: 3)
	pub fn set_shpf(value: u8) { ::write(REGISTER_ADDRESS, SHPF_BIT_OFFSET, SHPF_BIT_WIDTH, value as u32); }

	const INITS_BIT_OFFSET: u8 = 4;
	const INITS_BIT_WIDTH: u8 = 1;
	/// Initialization status flag (Width: 1, Offset: 4)
	pub fn get_inits() -> u8 { ::read(REGISTER_ADDRESS, INITS_BIT_OFFSET, INITS_BIT_WIDTH) as u8 }

	const RSF_BIT_OFFSET: u8 = 5;
	const RSF_BIT_WIDTH: u8 = 1;
	/// Registers synchronization flag (Width: 1, Offset: 5)
	pub fn get_rsf() -> u8 { ::read(REGISTER_ADDRESS, RSF_BIT_OFFSET, RSF_BIT_WIDTH) as u8 }
	/// Registers synchronization flag (Width: 1, Offset: 5)
	pub fn set_rsf(value: u8) { ::write(REGISTER_ADDRESS, RSF_BIT_OFFSET, RSF_BIT_WIDTH, value as u32); }

	const INITF_BIT_OFFSET: u8 = 6;
	const INITF_BIT_WIDTH: u8 = 1;
	/// Initialization flag (Width: 1, Offset: 6)
	pub fn get_initf() -> u8 { ::read(REGISTER_ADDRESS, INITF_BIT_OFFSET, INITF_BIT_WIDTH) as u8 }

	const INIT_BIT_OFFSET: u8 = 7;
	const INIT_BIT_WIDTH: u8 = 1;
	/// Initialization mode (Width: 1, Offset: 7)
	pub fn get_init() -> u8 { ::read(REGISTER_ADDRESS, INIT_BIT_OFFSET, INIT_BIT_WIDTH) as u8 }
	/// Initialization mode (Width: 1, Offset: 7)
	pub fn set_init(value: u8) { ::write(REGISTER_ADDRESS, INIT_BIT_OFFSET, INIT_BIT_WIDTH, value as u32); }

	const ALRAF_BIT_OFFSET: u8 = 8;
	const ALRAF_BIT_WIDTH: u8 = 1;
	/// Alarm A flag (Width: 1, Offset: 8)
	pub fn get_alraf() -> u8 { ::read(REGISTER_ADDRESS, ALRAF_BIT_OFFSET, ALRAF_BIT_WIDTH) as u8 }
	/// Alarm A flag (Width: 1, Offset: 8)
	pub fn set_alraf(value: u8) { ::write(REGISTER_ADDRESS, ALRAF_BIT_OFFSET, ALRAF_BIT_WIDTH, value as u32); }

	const ALRBF_BIT_OFFSET: u8 = 9;
	const ALRBF_BIT_WIDTH: u8 = 1;
	/// Alarm B flag (Width: 1, Offset: 9)
	pub fn get_alrbf() -> u8 { ::read(REGISTER_ADDRESS, ALRBF_BIT_OFFSET, ALRBF_BIT_WIDTH) as u8 }
	/// Alarm B flag (Width: 1, Offset: 9)
	pub fn set_alrbf(value: u8) { ::write(REGISTER_ADDRESS, ALRBF_BIT_OFFSET, ALRBF_BIT_WIDTH, value as u32); }

	const WUTF_BIT_OFFSET: u8 = 10;
	const WUTF_BIT_WIDTH: u8 = 1;
	/// Wakeup timer flag (Width: 1, Offset: 10)
	pub fn get_wutf() -> u8 { ::read(REGISTER_ADDRESS, WUTF_BIT_OFFSET, WUTF_BIT_WIDTH) as u8 }
	/// Wakeup timer flag (Width: 1, Offset: 10)
	pub fn set_wutf(value: u8) { ::write(REGISTER_ADDRESS, WUTF_BIT_OFFSET, WUTF_BIT_WIDTH, value as u32); }

	const TSF_BIT_OFFSET: u8 = 11;
	const TSF_BIT_WIDTH: u8 = 1;
	/// Time-stamp flag (Width: 1, Offset: 11)
	pub fn get_tsf() -> u8 { ::read(REGISTER_ADDRESS, TSF_BIT_OFFSET, TSF_BIT_WIDTH) as u8 }
	/// Time-stamp flag (Width: 1, Offset: 11)
	pub fn set_tsf(value: u8) { ::write(REGISTER_ADDRESS, TSF_BIT_OFFSET, TSF_BIT_WIDTH, value as u32); }

	const TSOVF_BIT_OFFSET: u8 = 12;
	const TSOVF_BIT_WIDTH: u8 = 1;
	/// Time-stamp overflow flag (Width: 1, Offset: 12)
	pub fn get_tsovf() -> u8 { ::read(REGISTER_ADDRESS, TSOVF_BIT_OFFSET, TSOVF_BIT_WIDTH) as u8 }
	/// Time-stamp overflow flag (Width: 1, Offset: 12)
	pub fn set_tsovf(value: u8) { ::write(REGISTER_ADDRESS, TSOVF_BIT_OFFSET, TSOVF_BIT_WIDTH, value as u32); }

	const TAMP1F_BIT_OFFSET: u8 = 13;
	const TAMP1F_BIT_WIDTH: u8 = 1;
	/// Tamper detection flag (Width: 1, Offset: 13)
	pub fn get_tamp1f() -> u8 { ::read(REGISTER_ADDRESS, TAMP1F_BIT_OFFSET, TAMP1F_BIT_WIDTH) as u8 }
	/// Tamper detection flag (Width: 1, Offset: 13)
	pub fn set_tamp1f(value: u8) { ::write(REGISTER_ADDRESS, TAMP1F_BIT_OFFSET, TAMP1F_BIT_WIDTH, value as u32); }

	const TAMP2F_BIT_OFFSET: u8 = 14;
	const TAMP2F_BIT_WIDTH: u8 = 1;
	/// RTC_TAMP2 detection flag (Width: 1, Offset: 14)
	pub fn get_tamp2f() -> u8 { ::read(REGISTER_ADDRESS, TAMP2F_BIT_OFFSET, TAMP2F_BIT_WIDTH) as u8 }
	/// RTC_TAMP2 detection flag (Width: 1, Offset: 14)
	pub fn set_tamp2f(value: u8) { ::write(REGISTER_ADDRESS, TAMP2F_BIT_OFFSET, TAMP2F_BIT_WIDTH, value as u32); }

	const TAMP3F_BIT_OFFSET: u8 = 15;
	const TAMP3F_BIT_WIDTH: u8 = 1;
	/// RTC_TAMP3 detection flag (Width: 1, Offset: 15)
	pub fn get_tamp3f() -> u8 { ::read(REGISTER_ADDRESS, TAMP3F_BIT_OFFSET, TAMP3F_BIT_WIDTH) as u8 }
	/// RTC_TAMP3 detection flag (Width: 1, Offset: 15)
	pub fn set_tamp3f(value: u8) { ::write(REGISTER_ADDRESS, TAMP3F_BIT_OFFSET, TAMP3F_BIT_WIDTH, value as u32); }

	const RECALPF_BIT_OFFSET: u8 = 16;
	const RECALPF_BIT_WIDTH: u8 = 1;
	/// Recalibration pending Flag (Width: 1, Offset: 16)
	pub fn get_recalpf() -> u8 { ::read(REGISTER_ADDRESS, RECALPF_BIT_OFFSET, RECALPF_BIT_WIDTH) as u8 }
}
/// prescaler register
/// Size: 0x20 bits
pub mod prer {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PREDIV_A_BIT_OFFSET: u8 = 16;
	const PREDIV_A_BIT_WIDTH: u8 = 7;
	/// Asynchronous prescaler factor (Width: 7, Offset: 16)
	pub fn get_prediv_a() -> u8 { ::read(REGISTER_ADDRESS, PREDIV_A_BIT_OFFSET, PREDIV_A_BIT_WIDTH) as u8 }
	/// Asynchronous prescaler factor (Width: 7, Offset: 16)
	pub fn set_prediv_a(value: u8) { ::write(REGISTER_ADDRESS, PREDIV_A_BIT_OFFSET, PREDIV_A_BIT_WIDTH, value as u32); }

	const PREDIV_S_BIT_OFFSET: u8 = 0;
	const PREDIV_S_BIT_WIDTH: u8 = 15;
	/// Synchronous prescaler factor (Width: 15, Offset: 0)
	pub fn get_prediv_s() -> u16 { ::read(REGISTER_ADDRESS, PREDIV_S_BIT_OFFSET, PREDIV_S_BIT_WIDTH) as u16 }
	/// Synchronous prescaler factor (Width: 15, Offset: 0)
	pub fn set_prediv_s(value: u16) { ::write(REGISTER_ADDRESS, PREDIV_S_BIT_OFFSET, PREDIV_S_BIT_WIDTH, value as u32); }
}
/// wakeup timer register
/// Size: 0x20 bits
pub mod wutr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const WUT_BIT_OFFSET: u8 = 0;
	const WUT_BIT_WIDTH: u8 = 16;
	/// Wakeup auto-reload value bits (Width: 16, Offset: 0)
	pub fn get_wut() -> u16 { ::read(REGISTER_ADDRESS, WUT_BIT_OFFSET, WUT_BIT_WIDTH) as u16 }
	/// Wakeup auto-reload value bits (Width: 16, Offset: 0)
	pub fn set_wut(value: u16) { ::write(REGISTER_ADDRESS, WUT_BIT_OFFSET, WUT_BIT_WIDTH, value as u32); }
}
/// alarm A register
/// Size: 0x20 bits
pub mod alrmar {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MSK4_BIT_OFFSET: u8 = 31;
	const MSK4_BIT_WIDTH: u8 = 1;
	/// Alarm A date mask (Width: 1, Offset: 31)
	pub fn get_msk4() -> u8 { ::read(REGISTER_ADDRESS, MSK4_BIT_OFFSET, MSK4_BIT_WIDTH) as u8 }
	/// Alarm A date mask (Width: 1, Offset: 31)
	pub fn set_msk4(value: u8) { ::write(REGISTER_ADDRESS, MSK4_BIT_OFFSET, MSK4_BIT_WIDTH, value as u32); }

	const WDSEL_BIT_OFFSET: u8 = 30;
	const WDSEL_BIT_WIDTH: u8 = 1;
	/// Week day selection (Width: 1, Offset: 30)
	pub fn get_wdsel() -> u8 { ::read(REGISTER_ADDRESS, WDSEL_BIT_OFFSET, WDSEL_BIT_WIDTH) as u8 }
	/// Week day selection (Width: 1, Offset: 30)
	pub fn set_wdsel(value: u8) { ::write(REGISTER_ADDRESS, WDSEL_BIT_OFFSET, WDSEL_BIT_WIDTH, value as u32); }

	const DT_BIT_OFFSET: u8 = 28;
	const DT_BIT_WIDTH: u8 = 2;
	/// Date tens in BCD format (Width: 2, Offset: 28)
	pub fn get_dt() -> u8 { ::read(REGISTER_ADDRESS, DT_BIT_OFFSET, DT_BIT_WIDTH) as u8 }
	/// Date tens in BCD format (Width: 2, Offset: 28)
	pub fn set_dt(value: u8) { ::write(REGISTER_ADDRESS, DT_BIT_OFFSET, DT_BIT_WIDTH, value as u32); }

	const DU_BIT_OFFSET: u8 = 24;
	const DU_BIT_WIDTH: u8 = 4;
	/// Date units or day in BCD format (Width: 4, Offset: 24)
	pub fn get_du() -> u8 { ::read(REGISTER_ADDRESS, DU_BIT_OFFSET, DU_BIT_WIDTH) as u8 }
	/// Date units or day in BCD format (Width: 4, Offset: 24)
	pub fn set_du(value: u8) { ::write(REGISTER_ADDRESS, DU_BIT_OFFSET, DU_BIT_WIDTH, value as u32); }

	const MSK3_BIT_OFFSET: u8 = 23;
	const MSK3_BIT_WIDTH: u8 = 1;
	/// Alarm A hours mask (Width: 1, Offset: 23)
	pub fn get_msk3() -> u8 { ::read(REGISTER_ADDRESS, MSK3_BIT_OFFSET, MSK3_BIT_WIDTH) as u8 }
	/// Alarm A hours mask (Width: 1, Offset: 23)
	pub fn set_msk3(value: u8) { ::write(REGISTER_ADDRESS, MSK3_BIT_OFFSET, MSK3_BIT_WIDTH, value as u32); }

	const PM_BIT_OFFSET: u8 = 22;
	const PM_BIT_WIDTH: u8 = 1;
	/// AM/PM notation (Width: 1, Offset: 22)
	pub fn get_pm() -> u8 { ::read(REGISTER_ADDRESS, PM_BIT_OFFSET, PM_BIT_WIDTH) as u8 }
	/// AM/PM notation (Width: 1, Offset: 22)
	pub fn set_pm(value: u8) { ::write(REGISTER_ADDRESS, PM_BIT_OFFSET, PM_BIT_WIDTH, value as u32); }

	const HT_BIT_OFFSET: u8 = 20;
	const HT_BIT_WIDTH: u8 = 2;
	/// Hour tens in BCD format (Width: 2, Offset: 20)
	pub fn get_ht() -> u8 { ::read(REGISTER_ADDRESS, HT_BIT_OFFSET, HT_BIT_WIDTH) as u8 }
	/// Hour tens in BCD format (Width: 2, Offset: 20)
	pub fn set_ht(value: u8) { ::write(REGISTER_ADDRESS, HT_BIT_OFFSET, HT_BIT_WIDTH, value as u32); }

	const HU_BIT_OFFSET: u8 = 16;
	const HU_BIT_WIDTH: u8 = 4;
	/// Hour units in BCD format (Width: 4, Offset: 16)
	pub fn get_hu() -> u8 { ::read(REGISTER_ADDRESS, HU_BIT_OFFSET, HU_BIT_WIDTH) as u8 }
	/// Hour units in BCD format (Width: 4, Offset: 16)
	pub fn set_hu(value: u8) { ::write(REGISTER_ADDRESS, HU_BIT_OFFSET, HU_BIT_WIDTH, value as u32); }

	const MSK2_BIT_OFFSET: u8 = 15;
	const MSK2_BIT_WIDTH: u8 = 1;
	/// Alarm A minutes mask (Width: 1, Offset: 15)
	pub fn get_msk2() -> u8 { ::read(REGISTER_ADDRESS, MSK2_BIT_OFFSET, MSK2_BIT_WIDTH) as u8 }
	/// Alarm A minutes mask (Width: 1, Offset: 15)
	pub fn set_msk2(value: u8) { ::write(REGISTER_ADDRESS, MSK2_BIT_OFFSET, MSK2_BIT_WIDTH, value as u32); }

	const MNT_BIT_OFFSET: u8 = 12;
	const MNT_BIT_WIDTH: u8 = 3;
	/// Minute tens in BCD format (Width: 3, Offset: 12)
	pub fn get_mnt() -> u8 { ::read(REGISTER_ADDRESS, MNT_BIT_OFFSET, MNT_BIT_WIDTH) as u8 }
	/// Minute tens in BCD format (Width: 3, Offset: 12)
	pub fn set_mnt(value: u8) { ::write(REGISTER_ADDRESS, MNT_BIT_OFFSET, MNT_BIT_WIDTH, value as u32); }

	const MNU_BIT_OFFSET: u8 = 8;
	const MNU_BIT_WIDTH: u8 = 4;
	/// Minute units in BCD format (Width: 4, Offset: 8)
	pub fn get_mnu() -> u8 { ::read(REGISTER_ADDRESS, MNU_BIT_OFFSET, MNU_BIT_WIDTH) as u8 }
	/// Minute units in BCD format (Width: 4, Offset: 8)
	pub fn set_mnu(value: u8) { ::write(REGISTER_ADDRESS, MNU_BIT_OFFSET, MNU_BIT_WIDTH, value as u32); }

	const MSK1_BIT_OFFSET: u8 = 7;
	const MSK1_BIT_WIDTH: u8 = 1;
	/// Alarm A seconds mask (Width: 1, Offset: 7)
	pub fn get_msk1() -> u8 { ::read(REGISTER_ADDRESS, MSK1_BIT_OFFSET, MSK1_BIT_WIDTH) as u8 }
	/// Alarm A seconds mask (Width: 1, Offset: 7)
	pub fn set_msk1(value: u8) { ::write(REGISTER_ADDRESS, MSK1_BIT_OFFSET, MSK1_BIT_WIDTH, value as u32); }

	const ST_BIT_OFFSET: u8 = 4;
	const ST_BIT_WIDTH: u8 = 3;
	/// Second tens in BCD format (Width: 3, Offset: 4)
	pub fn get_st() -> u8 { ::read(REGISTER_ADDRESS, ST_BIT_OFFSET, ST_BIT_WIDTH) as u8 }
	/// Second tens in BCD format (Width: 3, Offset: 4)
	pub fn set_st(value: u8) { ::write(REGISTER_ADDRESS, ST_BIT_OFFSET, ST_BIT_WIDTH, value as u32); }

	const SU_BIT_OFFSET: u8 = 0;
	const SU_BIT_WIDTH: u8 = 4;
	/// Second units in BCD format (Width: 4, Offset: 0)
	pub fn get_su() -> u8 { ::read(REGISTER_ADDRESS, SU_BIT_OFFSET, SU_BIT_WIDTH) as u8 }
	/// Second units in BCD format (Width: 4, Offset: 0)
	pub fn set_su(value: u8) { ::write(REGISTER_ADDRESS, SU_BIT_OFFSET, SU_BIT_WIDTH, value as u32); }
}
/// alarm B register
/// Size: 0x20 bits
pub mod alrmbr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x20;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MSK4_BIT_OFFSET: u8 = 31;
	const MSK4_BIT_WIDTH: u8 = 1;
	/// Alarm B date mask (Width: 1, Offset: 31)
	pub fn get_msk4() -> u8 { ::read(REGISTER_ADDRESS, MSK4_BIT_OFFSET, MSK4_BIT_WIDTH) as u8 }
	/// Alarm B date mask (Width: 1, Offset: 31)
	pub fn set_msk4(value: u8) { ::write(REGISTER_ADDRESS, MSK4_BIT_OFFSET, MSK4_BIT_WIDTH, value as u32); }

	const WDSEL_BIT_OFFSET: u8 = 30;
	const WDSEL_BIT_WIDTH: u8 = 1;
	/// Week day selection (Width: 1, Offset: 30)
	pub fn get_wdsel() -> u8 { ::read(REGISTER_ADDRESS, WDSEL_BIT_OFFSET, WDSEL_BIT_WIDTH) as u8 }
	/// Week day selection (Width: 1, Offset: 30)
	pub fn set_wdsel(value: u8) { ::write(REGISTER_ADDRESS, WDSEL_BIT_OFFSET, WDSEL_BIT_WIDTH, value as u32); }

	const DT_BIT_OFFSET: u8 = 28;
	const DT_BIT_WIDTH: u8 = 2;
	/// Date tens in BCD format (Width: 2, Offset: 28)
	pub fn get_dt() -> u8 { ::read(REGISTER_ADDRESS, DT_BIT_OFFSET, DT_BIT_WIDTH) as u8 }
	/// Date tens in BCD format (Width: 2, Offset: 28)
	pub fn set_dt(value: u8) { ::write(REGISTER_ADDRESS, DT_BIT_OFFSET, DT_BIT_WIDTH, value as u32); }

	const DU_BIT_OFFSET: u8 = 24;
	const DU_BIT_WIDTH: u8 = 4;
	/// Date units or day in BCD format (Width: 4, Offset: 24)
	pub fn get_du() -> u8 { ::read(REGISTER_ADDRESS, DU_BIT_OFFSET, DU_BIT_WIDTH) as u8 }
	/// Date units or day in BCD format (Width: 4, Offset: 24)
	pub fn set_du(value: u8) { ::write(REGISTER_ADDRESS, DU_BIT_OFFSET, DU_BIT_WIDTH, value as u32); }

	const MSK3_BIT_OFFSET: u8 = 23;
	const MSK3_BIT_WIDTH: u8 = 1;
	/// Alarm B hours mask (Width: 1, Offset: 23)
	pub fn get_msk3() -> u8 { ::read(REGISTER_ADDRESS, MSK3_BIT_OFFSET, MSK3_BIT_WIDTH) as u8 }
	/// Alarm B hours mask (Width: 1, Offset: 23)
	pub fn set_msk3(value: u8) { ::write(REGISTER_ADDRESS, MSK3_BIT_OFFSET, MSK3_BIT_WIDTH, value as u32); }

	const PM_BIT_OFFSET: u8 = 22;
	const PM_BIT_WIDTH: u8 = 1;
	/// AM/PM notation (Width: 1, Offset: 22)
	pub fn get_pm() -> u8 { ::read(REGISTER_ADDRESS, PM_BIT_OFFSET, PM_BIT_WIDTH) as u8 }
	/// AM/PM notation (Width: 1, Offset: 22)
	pub fn set_pm(value: u8) { ::write(REGISTER_ADDRESS, PM_BIT_OFFSET, PM_BIT_WIDTH, value as u32); }

	const HT_BIT_OFFSET: u8 = 20;
	const HT_BIT_WIDTH: u8 = 2;
	/// Hour tens in BCD format (Width: 2, Offset: 20)
	pub fn get_ht() -> u8 { ::read(REGISTER_ADDRESS, HT_BIT_OFFSET, HT_BIT_WIDTH) as u8 }
	/// Hour tens in BCD format (Width: 2, Offset: 20)
	pub fn set_ht(value: u8) { ::write(REGISTER_ADDRESS, HT_BIT_OFFSET, HT_BIT_WIDTH, value as u32); }

	const HU_BIT_OFFSET: u8 = 16;
	const HU_BIT_WIDTH: u8 = 4;
	/// Hour units in BCD format (Width: 4, Offset: 16)
	pub fn get_hu() -> u8 { ::read(REGISTER_ADDRESS, HU_BIT_OFFSET, HU_BIT_WIDTH) as u8 }
	/// Hour units in BCD format (Width: 4, Offset: 16)
	pub fn set_hu(value: u8) { ::write(REGISTER_ADDRESS, HU_BIT_OFFSET, HU_BIT_WIDTH, value as u32); }

	const MSK2_BIT_OFFSET: u8 = 15;
	const MSK2_BIT_WIDTH: u8 = 1;
	/// Alarm B minutes mask (Width: 1, Offset: 15)
	pub fn get_msk2() -> u8 { ::read(REGISTER_ADDRESS, MSK2_BIT_OFFSET, MSK2_BIT_WIDTH) as u8 }
	/// Alarm B minutes mask (Width: 1, Offset: 15)
	pub fn set_msk2(value: u8) { ::write(REGISTER_ADDRESS, MSK2_BIT_OFFSET, MSK2_BIT_WIDTH, value as u32); }

	const MNT_BIT_OFFSET: u8 = 12;
	const MNT_BIT_WIDTH: u8 = 3;
	/// Minute tens in BCD format (Width: 3, Offset: 12)
	pub fn get_mnt() -> u8 { ::read(REGISTER_ADDRESS, MNT_BIT_OFFSET, MNT_BIT_WIDTH) as u8 }
	/// Minute tens in BCD format (Width: 3, Offset: 12)
	pub fn set_mnt(value: u8) { ::write(REGISTER_ADDRESS, MNT_BIT_OFFSET, MNT_BIT_WIDTH, value as u32); }

	const MNU_BIT_OFFSET: u8 = 8;
	const MNU_BIT_WIDTH: u8 = 4;
	/// Minute units in BCD format (Width: 4, Offset: 8)
	pub fn get_mnu() -> u8 { ::read(REGISTER_ADDRESS, MNU_BIT_OFFSET, MNU_BIT_WIDTH) as u8 }
	/// Minute units in BCD format (Width: 4, Offset: 8)
	pub fn set_mnu(value: u8) { ::write(REGISTER_ADDRESS, MNU_BIT_OFFSET, MNU_BIT_WIDTH, value as u32); }

	const MSK1_BIT_OFFSET: u8 = 7;
	const MSK1_BIT_WIDTH: u8 = 1;
	/// Alarm B seconds mask (Width: 1, Offset: 7)
	pub fn get_msk1() -> u8 { ::read(REGISTER_ADDRESS, MSK1_BIT_OFFSET, MSK1_BIT_WIDTH) as u8 }
	/// Alarm B seconds mask (Width: 1, Offset: 7)
	pub fn set_msk1(value: u8) { ::write(REGISTER_ADDRESS, MSK1_BIT_OFFSET, MSK1_BIT_WIDTH, value as u32); }

	const ST_BIT_OFFSET: u8 = 4;
	const ST_BIT_WIDTH: u8 = 3;
	/// Second tens in BCD format (Width: 3, Offset: 4)
	pub fn get_st() -> u8 { ::read(REGISTER_ADDRESS, ST_BIT_OFFSET, ST_BIT_WIDTH) as u8 }
	/// Second tens in BCD format (Width: 3, Offset: 4)
	pub fn set_st(value: u8) { ::write(REGISTER_ADDRESS, ST_BIT_OFFSET, ST_BIT_WIDTH, value as u32); }

	const SU_BIT_OFFSET: u8 = 0;
	const SU_BIT_WIDTH: u8 = 4;
	/// Second units in BCD format (Width: 4, Offset: 0)
	pub fn get_su() -> u8 { ::read(REGISTER_ADDRESS, SU_BIT_OFFSET, SU_BIT_WIDTH) as u8 }
	/// Second units in BCD format (Width: 4, Offset: 0)
	pub fn set_su(value: u8) { ::write(REGISTER_ADDRESS, SU_BIT_OFFSET, SU_BIT_WIDTH, value as u32); }
}
/// write protection register
/// Size: 0x20 bits
pub mod wpr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x24;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const KEY_BIT_OFFSET: u8 = 0;
	const KEY_BIT_WIDTH: u8 = 8;
	/// Write protection key (Width: 8, Offset: 0)
	pub fn set_key(value: u8) { ::write(REGISTER_ADDRESS, KEY_BIT_OFFSET, KEY_BIT_WIDTH, value as u32); }
}
/// sub second register
/// Size: 0x20 bits
pub mod ssr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x28;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SS_BIT_OFFSET: u8 = 0;
	const SS_BIT_WIDTH: u8 = 16;
	/// Sub second value (Width: 16, Offset: 0)
	pub fn get_ss() -> u16 { ::read(REGISTER_ADDRESS, SS_BIT_OFFSET, SS_BIT_WIDTH) as u16 }
}
/// shift control register
/// Size: 0x20 bits
pub mod shiftr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x2C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ADD1S_BIT_OFFSET: u8 = 31;
	const ADD1S_BIT_WIDTH: u8 = 1;
	/// Add one second (Width: 1, Offset: 31)
	pub fn set_add1s(value: u8) { ::write(REGISTER_ADDRESS, ADD1S_BIT_OFFSET, ADD1S_BIT_WIDTH, value as u32); }

	const SUBFS_BIT_OFFSET: u8 = 0;
	const SUBFS_BIT_WIDTH: u8 = 15;
	/// Subtract a fraction of a second (Width: 15, Offset: 0)
	pub fn set_subfs(value: u16) { ::write(REGISTER_ADDRESS, SUBFS_BIT_OFFSET, SUBFS_BIT_WIDTH, value as u32); }
}
/// time stamp time register
/// Size: 0x20 bits
pub mod tstr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x30;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SU_BIT_OFFSET: u8 = 0;
	const SU_BIT_WIDTH: u8 = 4;
	/// Second units in BCD format (Width: 4, Offset: 0)
	pub fn get_su() -> u8 { ::read(REGISTER_ADDRESS, SU_BIT_OFFSET, SU_BIT_WIDTH) as u8 }

	const ST_BIT_OFFSET: u8 = 4;
	const ST_BIT_WIDTH: u8 = 3;
	/// Second tens in BCD format (Width: 3, Offset: 4)
	pub fn get_st() -> u8 { ::read(REGISTER_ADDRESS, ST_BIT_OFFSET, ST_BIT_WIDTH) as u8 }

	const MNU_BIT_OFFSET: u8 = 8;
	const MNU_BIT_WIDTH: u8 = 4;
	/// Minute units in BCD format (Width: 4, Offset: 8)
	pub fn get_mnu() -> u8 { ::read(REGISTER_ADDRESS, MNU_BIT_OFFSET, MNU_BIT_WIDTH) as u8 }

	const MNT_BIT_OFFSET: u8 = 12;
	const MNT_BIT_WIDTH: u8 = 3;
	/// Minute tens in BCD format (Width: 3, Offset: 12)
	pub fn get_mnt() -> u8 { ::read(REGISTER_ADDRESS, MNT_BIT_OFFSET, MNT_BIT_WIDTH) as u8 }

	const HU_BIT_OFFSET: u8 = 16;
	const HU_BIT_WIDTH: u8 = 4;
	/// Hour units in BCD format (Width: 4, Offset: 16)
	pub fn get_hu() -> u8 { ::read(REGISTER_ADDRESS, HU_BIT_OFFSET, HU_BIT_WIDTH) as u8 }

	const HT_BIT_OFFSET: u8 = 20;
	const HT_BIT_WIDTH: u8 = 2;
	/// Hour tens in BCD format (Width: 2, Offset: 20)
	pub fn get_ht() -> u8 { ::read(REGISTER_ADDRESS, HT_BIT_OFFSET, HT_BIT_WIDTH) as u8 }

	const PM_BIT_OFFSET: u8 = 22;
	const PM_BIT_WIDTH: u8 = 1;
	/// AM/PM notation (Width: 1, Offset: 22)
	pub fn get_pm() -> u8 { ::read(REGISTER_ADDRESS, PM_BIT_OFFSET, PM_BIT_WIDTH) as u8 }
}
/// time stamp date register
/// Size: 0x20 bits
pub mod tsdr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x34;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const WDU_BIT_OFFSET: u8 = 13;
	const WDU_BIT_WIDTH: u8 = 3;
	/// Week day units (Width: 3, Offset: 13)
	pub fn get_wdu() -> u8 { ::read(REGISTER_ADDRESS, WDU_BIT_OFFSET, WDU_BIT_WIDTH) as u8 }

	const MT_BIT_OFFSET: u8 = 12;
	const MT_BIT_WIDTH: u8 = 1;
	/// Month tens in BCD format (Width: 1, Offset: 12)
	pub fn get_mt() -> u8 { ::read(REGISTER_ADDRESS, MT_BIT_OFFSET, MT_BIT_WIDTH) as u8 }

	const MU_BIT_OFFSET: u8 = 8;
	const MU_BIT_WIDTH: u8 = 4;
	/// Month units in BCD format (Width: 4, Offset: 8)
	pub fn get_mu() -> u8 { ::read(REGISTER_ADDRESS, MU_BIT_OFFSET, MU_BIT_WIDTH) as u8 }

	const DT_BIT_OFFSET: u8 = 4;
	const DT_BIT_WIDTH: u8 = 2;
	/// Date tens in BCD format (Width: 2, Offset: 4)
	pub fn get_dt() -> u8 { ::read(REGISTER_ADDRESS, DT_BIT_OFFSET, DT_BIT_WIDTH) as u8 }

	const DU_BIT_OFFSET: u8 = 0;
	const DU_BIT_WIDTH: u8 = 4;
	/// Date units in BCD format (Width: 4, Offset: 0)
	pub fn get_du() -> u8 { ::read(REGISTER_ADDRESS, DU_BIT_OFFSET, DU_BIT_WIDTH) as u8 }
}
/// timestamp sub second register
/// Size: 0x20 bits
pub mod tsssr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x38;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SS_BIT_OFFSET: u8 = 0;
	const SS_BIT_WIDTH: u8 = 16;
	/// Sub second value (Width: 16, Offset: 0)
	pub fn get_ss() -> u16 { ::read(REGISTER_ADDRESS, SS_BIT_OFFSET, SS_BIT_WIDTH) as u16 }
}
/// calibration register
/// Size: 0x20 bits
pub mod calr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x3C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const CALP_BIT_OFFSET: u8 = 15;
	const CALP_BIT_WIDTH: u8 = 1;
	/// Increase frequency of RTC by 488.5 ppm (Width: 1, Offset: 15)
	pub fn get_calp() -> u8 { ::read(REGISTER_ADDRESS, CALP_BIT_OFFSET, CALP_BIT_WIDTH) as u8 }
	/// Increase frequency of RTC by 488.5 ppm (Width: 1, Offset: 15)
	pub fn set_calp(value: u8) { ::write(REGISTER_ADDRESS, CALP_BIT_OFFSET, CALP_BIT_WIDTH, value as u32); }

	const CALW8_BIT_OFFSET: u8 = 14;
	const CALW8_BIT_WIDTH: u8 = 1;
	/// Use an 8-second calibration cycle period (Width: 1, Offset: 14)
	pub fn get_calw8() -> u8 { ::read(REGISTER_ADDRESS, CALW8_BIT_OFFSET, CALW8_BIT_WIDTH) as u8 }
	/// Use an 8-second calibration cycle period (Width: 1, Offset: 14)
	pub fn set_calw8(value: u8) { ::write(REGISTER_ADDRESS, CALW8_BIT_OFFSET, CALW8_BIT_WIDTH, value as u32); }

	const CALW16_BIT_OFFSET: u8 = 13;
	const CALW16_BIT_WIDTH: u8 = 1;
	/// Use a 16-second calibration cycle period (Width: 1, Offset: 13)
	pub fn get_calw16() -> u8 { ::read(REGISTER_ADDRESS, CALW16_BIT_OFFSET, CALW16_BIT_WIDTH) as u8 }
	/// Use a 16-second calibration cycle period (Width: 1, Offset: 13)
	pub fn set_calw16(value: u8) { ::write(REGISTER_ADDRESS, CALW16_BIT_OFFSET, CALW16_BIT_WIDTH, value as u32); }

	const CALM_BIT_OFFSET: u8 = 0;
	const CALM_BIT_WIDTH: u8 = 9;
	/// Calibration minus (Width: 9, Offset: 0)
	pub fn get_calm() -> u16 { ::read(REGISTER_ADDRESS, CALM_BIT_OFFSET, CALM_BIT_WIDTH) as u16 }
	/// Calibration minus (Width: 9, Offset: 0)
	pub fn set_calm(value: u16) { ::write(REGISTER_ADDRESS, CALM_BIT_OFFSET, CALM_BIT_WIDTH, value as u32); }
}
/// tamper and alternate function configuration register
/// Size: 0x20 bits
pub mod tafcr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x40;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TAMP1E_BIT_OFFSET: u8 = 0;
	const TAMP1E_BIT_WIDTH: u8 = 1;
	/// Tamper 1 detection enable (Width: 1, Offset: 0)
	pub fn get_tamp1e() -> u8 { ::read(REGISTER_ADDRESS, TAMP1E_BIT_OFFSET, TAMP1E_BIT_WIDTH) as u8 }
	/// Tamper 1 detection enable (Width: 1, Offset: 0)
	pub fn set_tamp1e(value: u8) { ::write(REGISTER_ADDRESS, TAMP1E_BIT_OFFSET, TAMP1E_BIT_WIDTH, value as u32); }

	const TAMP1TRG_BIT_OFFSET: u8 = 1;
	const TAMP1TRG_BIT_WIDTH: u8 = 1;
	/// Active level for tamper 1 (Width: 1, Offset: 1)
	pub fn get_tamp1trg() -> u8 { ::read(REGISTER_ADDRESS, TAMP1TRG_BIT_OFFSET, TAMP1TRG_BIT_WIDTH) as u8 }
	/// Active level for tamper 1 (Width: 1, Offset: 1)
	pub fn set_tamp1trg(value: u8) { ::write(REGISTER_ADDRESS, TAMP1TRG_BIT_OFFSET, TAMP1TRG_BIT_WIDTH, value as u32); }

	const TAMPIE_BIT_OFFSET: u8 = 2;
	const TAMPIE_BIT_WIDTH: u8 = 1;
	/// Tamper interrupt enable (Width: 1, Offset: 2)
	pub fn get_tampie() -> u8 { ::read(REGISTER_ADDRESS, TAMPIE_BIT_OFFSET, TAMPIE_BIT_WIDTH) as u8 }
	/// Tamper interrupt enable (Width: 1, Offset: 2)
	pub fn set_tampie(value: u8) { ::write(REGISTER_ADDRESS, TAMPIE_BIT_OFFSET, TAMPIE_BIT_WIDTH, value as u32); }

	const TAMP2E_BIT_OFFSET: u8 = 3;
	const TAMP2E_BIT_WIDTH: u8 = 1;
	/// Tamper 2 detection enable (Width: 1, Offset: 3)
	pub fn get_tamp2e() -> u8 { ::read(REGISTER_ADDRESS, TAMP2E_BIT_OFFSET, TAMP2E_BIT_WIDTH) as u8 }
	/// Tamper 2 detection enable (Width: 1, Offset: 3)
	pub fn set_tamp2e(value: u8) { ::write(REGISTER_ADDRESS, TAMP2E_BIT_OFFSET, TAMP2E_BIT_WIDTH, value as u32); }

	const TAMP2TRG_BIT_OFFSET: u8 = 4;
	const TAMP2TRG_BIT_WIDTH: u8 = 1;
	/// Active level for tamper 2 (Width: 1, Offset: 4)
	pub fn get_tamp2trg() -> u8 { ::read(REGISTER_ADDRESS, TAMP2TRG_BIT_OFFSET, TAMP2TRG_BIT_WIDTH) as u8 }
	/// Active level for tamper 2 (Width: 1, Offset: 4)
	pub fn set_tamp2trg(value: u8) { ::write(REGISTER_ADDRESS, TAMP2TRG_BIT_OFFSET, TAMP2TRG_BIT_WIDTH, value as u32); }

	const TAMP3E_BIT_OFFSET: u8 = 5;
	const TAMP3E_BIT_WIDTH: u8 = 1;
	/// Tamper 3 detection enable (Width: 1, Offset: 5)
	pub fn get_tamp3e() -> u8 { ::read(REGISTER_ADDRESS, TAMP3E_BIT_OFFSET, TAMP3E_BIT_WIDTH) as u8 }
	/// Tamper 3 detection enable (Width: 1, Offset: 5)
	pub fn set_tamp3e(value: u8) { ::write(REGISTER_ADDRESS, TAMP3E_BIT_OFFSET, TAMP3E_BIT_WIDTH, value as u32); }

	const TAMP3TRG_BIT_OFFSET: u8 = 6;
	const TAMP3TRG_BIT_WIDTH: u8 = 1;
	/// Active level for tamper 3 (Width: 1, Offset: 6)
	pub fn get_tamp3trg() -> u8 { ::read(REGISTER_ADDRESS, TAMP3TRG_BIT_OFFSET, TAMP3TRG_BIT_WIDTH) as u8 }
	/// Active level for tamper 3 (Width: 1, Offset: 6)
	pub fn set_tamp3trg(value: u8) { ::write(REGISTER_ADDRESS, TAMP3TRG_BIT_OFFSET, TAMP3TRG_BIT_WIDTH, value as u32); }

	const TAMPTS_BIT_OFFSET: u8 = 7;
	const TAMPTS_BIT_WIDTH: u8 = 1;
	/// Activate timestamp on tamper detection event (Width: 1, Offset: 7)
	pub fn get_tampts() -> u8 { ::read(REGISTER_ADDRESS, TAMPTS_BIT_OFFSET, TAMPTS_BIT_WIDTH) as u8 }
	/// Activate timestamp on tamper detection event (Width: 1, Offset: 7)
	pub fn set_tampts(value: u8) { ::write(REGISTER_ADDRESS, TAMPTS_BIT_OFFSET, TAMPTS_BIT_WIDTH, value as u32); }

	const TAMPFREQ_BIT_OFFSET: u8 = 8;
	const TAMPFREQ_BIT_WIDTH: u8 = 3;
	/// Tamper sampling frequency (Width: 3, Offset: 8)
	pub fn get_tampfreq() -> u8 { ::read(REGISTER_ADDRESS, TAMPFREQ_BIT_OFFSET, TAMPFREQ_BIT_WIDTH) as u8 }
	/// Tamper sampling frequency (Width: 3, Offset: 8)
	pub fn set_tampfreq(value: u8) { ::write(REGISTER_ADDRESS, TAMPFREQ_BIT_OFFSET, TAMPFREQ_BIT_WIDTH, value as u32); }

	const TAMPFLT_BIT_OFFSET: u8 = 11;
	const TAMPFLT_BIT_WIDTH: u8 = 2;
	/// Tamper filter count (Width: 2, Offset: 11)
	pub fn get_tampflt() -> u8 { ::read(REGISTER_ADDRESS, TAMPFLT_BIT_OFFSET, TAMPFLT_BIT_WIDTH) as u8 }
	/// Tamper filter count (Width: 2, Offset: 11)
	pub fn set_tampflt(value: u8) { ::write(REGISTER_ADDRESS, TAMPFLT_BIT_OFFSET, TAMPFLT_BIT_WIDTH, value as u32); }

	const TAMPPRCH_BIT_OFFSET: u8 = 13;
	const TAMPPRCH_BIT_WIDTH: u8 = 2;
	/// Tamper precharge duration (Width: 2, Offset: 13)
	pub fn get_tampprch() -> u8 { ::read(REGISTER_ADDRESS, TAMPPRCH_BIT_OFFSET, TAMPPRCH_BIT_WIDTH) as u8 }
	/// Tamper precharge duration (Width: 2, Offset: 13)
	pub fn set_tampprch(value: u8) { ::write(REGISTER_ADDRESS, TAMPPRCH_BIT_OFFSET, TAMPPRCH_BIT_WIDTH, value as u32); }

	const TAMPPUDIS_BIT_OFFSET: u8 = 15;
	const TAMPPUDIS_BIT_WIDTH: u8 = 1;
	/// TAMPER pull-up disable (Width: 1, Offset: 15)
	pub fn get_tamppudis() -> u8 { ::read(REGISTER_ADDRESS, TAMPPUDIS_BIT_OFFSET, TAMPPUDIS_BIT_WIDTH) as u8 }
	/// TAMPER pull-up disable (Width: 1, Offset: 15)
	pub fn set_tamppudis(value: u8) { ::write(REGISTER_ADDRESS, TAMPPUDIS_BIT_OFFSET, TAMPPUDIS_BIT_WIDTH, value as u32); }

	const PC13VALUE_BIT_OFFSET: u8 = 18;
	const PC13VALUE_BIT_WIDTH: u8 = 1;
	/// PC13 value (Width: 1, Offset: 18)
	pub fn get_pc13value() -> u8 { ::read(REGISTER_ADDRESS, PC13VALUE_BIT_OFFSET, PC13VALUE_BIT_WIDTH) as u8 }
	/// PC13 value (Width: 1, Offset: 18)
	pub fn set_pc13value(value: u8) { ::write(REGISTER_ADDRESS, PC13VALUE_BIT_OFFSET, PC13VALUE_BIT_WIDTH, value as u32); }

	const PC13MODE_BIT_OFFSET: u8 = 19;
	const PC13MODE_BIT_WIDTH: u8 = 1;
	/// PC13 mode (Width: 1, Offset: 19)
	pub fn get_pc13mode() -> u8 { ::read(REGISTER_ADDRESS, PC13MODE_BIT_OFFSET, PC13MODE_BIT_WIDTH) as u8 }
	/// PC13 mode (Width: 1, Offset: 19)
	pub fn set_pc13mode(value: u8) { ::write(REGISTER_ADDRESS, PC13MODE_BIT_OFFSET, PC13MODE_BIT_WIDTH, value as u32); }

	const PC14VALUE_BIT_OFFSET: u8 = 20;
	const PC14VALUE_BIT_WIDTH: u8 = 1;
	/// PC14 value (Width: 1, Offset: 20)
	pub fn get_pc14value() -> u8 { ::read(REGISTER_ADDRESS, PC14VALUE_BIT_OFFSET, PC14VALUE_BIT_WIDTH) as u8 }
	/// PC14 value (Width: 1, Offset: 20)
	pub fn set_pc14value(value: u8) { ::write(REGISTER_ADDRESS, PC14VALUE_BIT_OFFSET, PC14VALUE_BIT_WIDTH, value as u32); }

	const PC14MODE_BIT_OFFSET: u8 = 21;
	const PC14MODE_BIT_WIDTH: u8 = 1;
	/// PC 14 mode (Width: 1, Offset: 21)
	pub fn get_pc14mode() -> u8 { ::read(REGISTER_ADDRESS, PC14MODE_BIT_OFFSET, PC14MODE_BIT_WIDTH) as u8 }
	/// PC 14 mode (Width: 1, Offset: 21)
	pub fn set_pc14mode(value: u8) { ::write(REGISTER_ADDRESS, PC14MODE_BIT_OFFSET, PC14MODE_BIT_WIDTH, value as u32); }

	const PC15VALUE_BIT_OFFSET: u8 = 22;
	const PC15VALUE_BIT_WIDTH: u8 = 1;
	/// PC15 value (Width: 1, Offset: 22)
	pub fn get_pc15value() -> u8 { ::read(REGISTER_ADDRESS, PC15VALUE_BIT_OFFSET, PC15VALUE_BIT_WIDTH) as u8 }
	/// PC15 value (Width: 1, Offset: 22)
	pub fn set_pc15value(value: u8) { ::write(REGISTER_ADDRESS, PC15VALUE_BIT_OFFSET, PC15VALUE_BIT_WIDTH, value as u32); }

	const PC15MODE_BIT_OFFSET: u8 = 23;
	const PC15MODE_BIT_WIDTH: u8 = 1;
	/// PC15 mode (Width: 1, Offset: 23)
	pub fn get_pc15mode() -> u8 { ::read(REGISTER_ADDRESS, PC15MODE_BIT_OFFSET, PC15MODE_BIT_WIDTH) as u8 }
	/// PC15 mode (Width: 1, Offset: 23)
	pub fn set_pc15mode(value: u8) { ::write(REGISTER_ADDRESS, PC15MODE_BIT_OFFSET, PC15MODE_BIT_WIDTH, value as u32); }
}
/// alarm A sub second register
/// Size: 0x20 bits
pub mod alrmassr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x44;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MASKSS_BIT_OFFSET: u8 = 24;
	const MASKSS_BIT_WIDTH: u8 = 4;
	/// Mask the most-significant bits starting at this bit (Width: 4, Offset: 24)
	pub fn get_maskss() -> u8 { ::read(REGISTER_ADDRESS, MASKSS_BIT_OFFSET, MASKSS_BIT_WIDTH) as u8 }
	/// Mask the most-significant bits starting at this bit (Width: 4, Offset: 24)
	pub fn set_maskss(value: u8) { ::write(REGISTER_ADDRESS, MASKSS_BIT_OFFSET, MASKSS_BIT_WIDTH, value as u32); }

	const SS_BIT_OFFSET: u8 = 0;
	const SS_BIT_WIDTH: u8 = 15;
	/// Sub seconds value (Width: 15, Offset: 0)
	pub fn get_ss() -> u16 { ::read(REGISTER_ADDRESS, SS_BIT_OFFSET, SS_BIT_WIDTH) as u16 }
	/// Sub seconds value (Width: 15, Offset: 0)
	pub fn set_ss(value: u16) { ::write(REGISTER_ADDRESS, SS_BIT_OFFSET, SS_BIT_WIDTH, value as u32); }
}
/// alarm B sub second register
/// Size: 0x20 bits
pub mod alrmbssr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x48;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MASKSS_BIT_OFFSET: u8 = 24;
	const MASKSS_BIT_WIDTH: u8 = 4;
	/// Mask the most-significant bits starting at this bit (Width: 4, Offset: 24)
	pub fn get_maskss() -> u8 { ::read(REGISTER_ADDRESS, MASKSS_BIT_OFFSET, MASKSS_BIT_WIDTH) as u8 }
	/// Mask the most-significant bits starting at this bit (Width: 4, Offset: 24)
	pub fn set_maskss(value: u8) { ::write(REGISTER_ADDRESS, MASKSS_BIT_OFFSET, MASKSS_BIT_WIDTH, value as u32); }

	const SS_BIT_OFFSET: u8 = 0;
	const SS_BIT_WIDTH: u8 = 15;
	/// Sub seconds value (Width: 15, Offset: 0)
	pub fn get_ss() -> u16 { ::read(REGISTER_ADDRESS, SS_BIT_OFFSET, SS_BIT_WIDTH) as u16 }
	/// Sub seconds value (Width: 15, Offset: 0)
	pub fn set_ss(value: u16) { ::write(REGISTER_ADDRESS, SS_BIT_OFFSET, SS_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp0r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x50;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp1r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x54;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp2r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x58;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp3r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x5C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp4r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x60;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp5r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x64;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp6r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x68;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp7r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x6C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp8r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x70;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp9r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x74;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp10r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x78;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp11r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x7C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp12r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x80;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp13r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x84;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp14r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x88;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp15r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp16r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x90;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp17r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x94;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp18r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x98;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp19r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x9C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp20r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xA0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp21r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xA4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp22r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xA8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp23r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xAC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp24r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xB0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp25r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xB4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp26r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xB8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp27r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xBC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp28r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp29r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp30r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// backup register
/// Size: 0x20 bits
pub mod bkp31r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xCC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BKP_BIT_OFFSET: u8 = 0;
	const BKP_BIT_WIDTH: u8 = 32;
	/// BKP (Width: 32, Offset: 0)
	pub fn get_bkp() -> u32 { ::read(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH) as u32 }
	/// BKP (Width: 32, Offset: 0)
	pub fn set_bkp(value: u32) { ::write(REGISTER_ADDRESS, BKP_BIT_OFFSET, BKP_BIT_WIDTH, value as u32); }
}
/// RTC Wakeup interrupt through the EXTI line
pub const INTERRUPT_RTC_WKUP: u32 = 3;

/// RTC alarm interrupt
pub const INTERRUPT_RTCAlarm: u32 = 41;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>RTC</name>
  <description>Real-time clock</description>
  <groupName>RTC</groupName>
  <baseAddress>0x40002800</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>TR</name>
      <displayName>TR</displayName>
      <description>time register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PM</name>
          <description>AM/PM notation</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HT</name>
          <description>Hour tens in BCD format</description>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>HU</name>
          <description>Hour units in BCD format</description>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>MNT</name>
          <description>Minute tens in BCD format</description>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>MNU</name>
          <description>Minute units in BCD format</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>ST</name>
          <description>Second tens in BCD format</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SU</name>
          <description>Second units in BCD format</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DR</name>
      <displayName>DR</displayName>
      <description>date register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00002101</resetValue>
      <fields>
        <field>
          <name>YT</name>
          <description>Year tens in BCD format</description>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>YU</name>
          <description>Year units in BCD format</description>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>WDU</name>
          <description>Week day units</description>
          <bitOffset>13</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>MT</name>
          <description>Month tens in BCD format</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MU</name>
          <description>Month units in BCD format</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>DT</name>
          <description>Date tens in BCD format</description>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>DU</name>
          <description>Date units in BCD format</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
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
          <name>WCKSEL</name>
          <description>Wakeup clock selection</description>
          <bitOffset>0</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>TSEDGE</name>
          <description>Time-stamp event active
              edge</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>REFCKON</name>
          <description>Reference clock detection enable (50 or
              60 Hz)</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BYPSHAD</name>
          <description>Bypass the shadow
              registers</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>FMT</name>
          <description>Hour format</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ALRAE</name>
          <description>Alarm A enable</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ALRBE</name>
          <description>Alarm B enable</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>WUTE</name>
          <description>Wakeup timer enable</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TSE</name>
          <description>Time stamp enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ALRAIE</name>
          <description>Alarm A interrupt enable</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ALRBIE</name>
          <description>Alarm B interrupt enable</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>WUTIE</name>
          <description>Wakeup timer interrupt
              enable</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TSIE</name>
          <description>Time-stamp interrupt
              enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADD1H</name>
          <description>Add 1 hour (summer time
              change)</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SUB1H</name>
          <description>Subtract 1 hour (winter time
              change)</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BKP</name>
          <description>Backup</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>COSEL</name>
          <description>Calibration output
              selection</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>POL</name>
          <description>Output polarity</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>OSEL</name>
          <description>Output selection</description>
          <bitOffset>21</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>COE</name>
          <description>Calibration output enable</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ISR</name>
      <displayName>ISR</displayName>
      <description>initialization and status
          register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000007</resetValue>
      <fields>
        <field>
          <name>ALRAWF</name>
          <description>Alarm A write flag</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>ALRBWF</name>
          <description>Alarm B write flag</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>WUTWF</name>
          <description>Wakeup timer write flag</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>SHPF</name>
          <description>Shift operation pending</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>INITS</name>
          <description>Initialization status flag</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>RSF</name>
          <description>Registers synchronization
              flag</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>INITF</name>
          <description>Initialization flag</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>INIT</name>
          <description>Initialization mode</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>ALRAF</name>
          <description>Alarm A flag</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>ALRBF</name>
          <description>Alarm B flag</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>WUTF</name>
          <description>Wakeup timer flag</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TSF</name>
          <description>Time-stamp flag</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TSOVF</name>
          <description>Time-stamp overflow flag</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TAMP1F</name>
          <description>Tamper detection flag</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TAMP2F</name>
          <description>RTC_TAMP2 detection flag</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TAMP3F</name>
          <description>RTC_TAMP3 detection flag</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>RECALPF</name>
          <description>Recalibration pending Flag</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
      </fields>
    </register>
    <register>
      <name>PRER</name>
      <displayName>PRER</displayName>
      <description>prescaler register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x007F00FF</resetValue>
      <fields>
        <field>
          <name>PREDIV_A</name>
          <description>Asynchronous prescaler
              factor</description>
          <bitOffset>16</bitOffset>
          <bitWidth>7</bitWidth>
        </field>
        <field>
          <name>PREDIV_S</name>
          <description>Synchronous prescaler
              factor</description>
          <bitOffset>0</bitOffset>
          <bitWidth>15</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>WUTR</name>
      <displayName>WUTR</displayName>
      <description>wakeup timer register</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000FFFF</resetValue>
      <fields>
        <field>
          <name>WUT</name>
          <description>Wakeup auto-reload value
              bits</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ALRMAR</name>
      <displayName>ALRMAR</displayName>
      <description>alarm A register</description>
      <addressOffset>0x1C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MSK4</name>
          <description>Alarm A date mask</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>WDSEL</name>
          <description>Week day selection</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DT</name>
          <description>Date tens in BCD format</description>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>DU</name>
          <description>Date units or day in BCD
              format</description>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>MSK3</name>
          <description>Alarm A hours mask</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PM</name>
          <description>AM/PM notation</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HT</name>
          <description>Hour tens in BCD format</description>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>HU</name>
          <description>Hour units in BCD format</description>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>MSK2</name>
          <description>Alarm A minutes mask</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MNT</name>
          <description>Minute tens in BCD format</description>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>MNU</name>
          <description>Minute units in BCD format</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>MSK1</name>
          <description>Alarm A seconds mask</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ST</name>
          <description>Second tens in BCD format</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SU</name>
          <description>Second units in BCD format</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ALRMBR</name>
      <displayName>ALRMBR</displayName>
      <description>alarm B register</description>
      <addressOffset>0x20</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MSK4</name>
          <description>Alarm B date mask</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>WDSEL</name>
          <description>Week day selection</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DT</name>
          <description>Date tens in BCD format</description>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>DU</name>
          <description>Date units or day in BCD
              format</description>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>MSK3</name>
          <description>Alarm B hours mask</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PM</name>
          <description>AM/PM notation</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>HT</name>
          <description>Hour tens in BCD format</description>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>HU</name>
          <description>Hour units in BCD format</description>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>MSK2</name>
          <description>Alarm B minutes mask</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MNT</name>
          <description>Minute tens in BCD format</description>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>MNU</name>
          <description>Minute units in BCD format</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>MSK1</name>
          <description>Alarm B seconds mask</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ST</name>
          <description>Second tens in BCD format</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>SU</name>
          <description>Second units in BCD format</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>WPR</name>
      <displayName>WPR</displayName>
      <description>write protection register</description>
      <addressOffset>0x24</addressOffset>
      <size>0x20</size>
      <access>write-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>KEY</name>
          <description>Write protection key</description>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SSR</name>
      <displayName>SSR</displayName>
      <description>sub second register</description>
      <addressOffset>0x28</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SS</name>
          <description>Sub second value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>SHIFTR</name>
      <displayName>SHIFTR</displayName>
      <description>shift control register</description>
      <addressOffset>0x2C</addressOffset>
      <size>0x20</size>
      <access>write-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>ADD1S</name>
          <description>Add one second</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SUBFS</name>
          <description>Subtract a fraction of a
              second</description>
          <bitOffset>0</bitOffset>
          <bitWidth>15</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>TSTR</name>
      <displayName>TSTR</displayName>
      <description>time stamp time register</description>
      <addressOffset>0x30</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SU</name>
          <description>Second units in BCD format</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>ST</name>
          <description>Second tens in BCD format</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>MNU</name>
          <description>Minute units in BCD format</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>MNT</name>
          <description>Minute tens in BCD format</description>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>HU</name>
          <description>Hour units in BCD format</description>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>HT</name>
          <description>Hour tens in BCD format</description>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>PM</name>
          <description>AM/PM notation</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>TSDR</name>
      <displayName>TSDR</displayName>
      <description>time stamp date register</description>
      <addressOffset>0x34</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>WDU</name>
          <description>Week day units</description>
          <bitOffset>13</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>MT</name>
          <description>Month tens in BCD format</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>MU</name>
          <description>Month units in BCD format</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>DT</name>
          <description>Date tens in BCD format</description>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>DU</name>
          <description>Date units in BCD format</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>TSSSR</name>
      <displayName>TSSSR</displayName>
      <description>timestamp sub second register</description>
      <addressOffset>0x38</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SS</name>
          <description>Sub second value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CALR</name>
      <displayName>CALR</displayName>
      <description>calibration register</description>
      <addressOffset>0x3C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>CALP</name>
          <description>Increase frequency of RTC by 488.5
              ppm</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CALW8</name>
          <description>Use an 8-second calibration cycle
              period</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CALW16</name>
          <description>Use a 16-second calibration cycle
              period</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CALM</name>
          <description>Calibration minus</description>
          <bitOffset>0</bitOffset>
          <bitWidth>9</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>TAFCR</name>
      <displayName>TAFCR</displayName>
      <description>tamper and alternate function configuration
          register</description>
      <addressOffset>0x40</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>TAMP1E</name>
          <description>Tamper 1 detection enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TAMP1TRG</name>
          <description>Active level for tamper 1</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TAMPIE</name>
          <description>Tamper interrupt enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TAMP2E</name>
          <description>Tamper 2 detection enable</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TAMP2TRG</name>
          <description>Active level for tamper 2</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TAMP3E</name>
          <description>Tamper 3 detection enable</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TAMP3TRG</name>
          <description>Active level for tamper 3</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TAMPTS</name>
          <description>Activate timestamp on tamper detection
              event</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TAMPFREQ</name>
          <description>Tamper sampling frequency</description>
          <bitOffset>8</bitOffset>
          <bitWidth>3</bitWidth>
        </field>
        <field>
          <name>TAMPFLT</name>
          <description>Tamper filter count</description>
          <bitOffset>11</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>TAMPPRCH</name>
          <description>Tamper precharge duration</description>
          <bitOffset>13</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>TAMPPUDIS</name>
          <description>TAMPER pull-up disable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PC13VALUE</name>
          <description>PC13 value</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PC13MODE</name>
          <description>PC13 mode</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PC14VALUE</name>
          <description>PC14 value</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PC14MODE</name>
          <description>PC 14 mode</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PC15VALUE</name>
          <description>PC15 value</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PC15MODE</name>
          <description>PC15 mode</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ALRMASSR</name>
      <displayName>ALRMASSR</displayName>
      <description>alarm A sub second register</description>
      <addressOffset>0x44</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MASKSS</name>
          <description>Mask the most-significant bits starting
              at this bit</description>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>SS</name>
          <description>Sub seconds value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>15</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ALRMBSSR</name>
      <displayName>ALRMBSSR</displayName>
      <description>alarm B sub second register</description>
      <addressOffset>0x48</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MASKSS</name>
          <description>Mask the most-significant bits starting
              at this bit</description>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>SS</name>
          <description>Sub seconds value</description>
          <bitOffset>0</bitOffset>
          <bitWidth>15</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP0R</name>
      <displayName>BKP0R</displayName>
      <description>backup register</description>
      <addressOffset>0x50</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP1R</name>
      <displayName>BKP1R</displayName>
      <description>backup register</description>
      <addressOffset>0x54</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP2R</name>
      <displayName>BKP2R</displayName>
      <description>backup register</description>
      <addressOffset>0x58</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP3R</name>
      <displayName>BKP3R</displayName>
      <description>backup register</description>
      <addressOffset>0x5C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP4R</name>
      <displayName>BKP4R</displayName>
      <description>backup register</description>
      <addressOffset>0x60</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP5R</name>
      <displayName>BKP5R</displayName>
      <description>backup register</description>
      <addressOffset>0x64</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP6R</name>
      <displayName>BKP6R</displayName>
      <description>backup register</description>
      <addressOffset>0x68</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP7R</name>
      <displayName>BKP7R</displayName>
      <description>backup register</description>
      <addressOffset>0x6C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP8R</name>
      <displayName>BKP8R</displayName>
      <description>backup register</description>
      <addressOffset>0x70</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP9R</name>
      <displayName>BKP9R</displayName>
      <description>backup register</description>
      <addressOffset>0x74</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP10R</name>
      <displayName>BKP10R</displayName>
      <description>backup register</description>
      <addressOffset>0x78</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP11R</name>
      <displayName>BKP11R</displayName>
      <description>backup register</description>
      <addressOffset>0x7C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP12R</name>
      <displayName>BKP12R</displayName>
      <description>backup register</description>
      <addressOffset>0x80</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP13R</name>
      <displayName>BKP13R</displayName>
      <description>backup register</description>
      <addressOffset>0x84</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP14R</name>
      <displayName>BKP14R</displayName>
      <description>backup register</description>
      <addressOffset>0x88</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP15R</name>
      <displayName>BKP15R</displayName>
      <description>backup register</description>
      <addressOffset>0x8C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP16R</name>
      <displayName>BKP16R</displayName>
      <description>backup register</description>
      <addressOffset>0x90</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP17R</name>
      <displayName>BKP17R</displayName>
      <description>backup register</description>
      <addressOffset>0x94</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP18R</name>
      <displayName>BKP18R</displayName>
      <description>backup register</description>
      <addressOffset>0x98</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP19R</name>
      <displayName>BKP19R</displayName>
      <description>backup register</description>
      <addressOffset>0x9C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP20R</name>
      <displayName>BKP20R</displayName>
      <description>backup register</description>
      <addressOffset>0xA0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP21R</name>
      <displayName>BKP21R</displayName>
      <description>backup register</description>
      <addressOffset>0xA4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP22R</name>
      <displayName>BKP22R</displayName>
      <description>backup register</description>
      <addressOffset>0xA8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP23R</name>
      <displayName>BKP23R</displayName>
      <description>backup register</description>
      <addressOffset>0xAC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP24R</name>
      <displayName>BKP24R</displayName>
      <description>backup register</description>
      <addressOffset>0xB0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP25R</name>
      <displayName>BKP25R</displayName>
      <description>backup register</description>
      <addressOffset>0xB4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP26R</name>
      <displayName>BKP26R</displayName>
      <description>backup register</description>
      <addressOffset>0xB8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP27R</name>
      <displayName>BKP27R</displayName>
      <description>backup register</description>
      <addressOffset>0xBC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP28R</name>
      <displayName>BKP28R</displayName>
      <description>backup register</description>
      <addressOffset>0xC0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP29R</name>
      <displayName>BKP29R</displayName>
      <description>backup register</description>
      <addressOffset>0xC4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP30R</name>
      <displayName>BKP30R</displayName>
      <description>backup register</description>
      <addressOffset>0xC8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BKP31R</name>
      <displayName>BKP31R</displayName>
      <description>backup register</description>
      <addressOffset>0xCC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>BKP</name>
          <description>BKP</description>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>RTC_WKUP</name>
    <description>RTC Wakeup interrupt through the EXTI
        line</description>
    <value>3</value>
  </interrupt>
  <interrupt>
    <name>RTCAlarm</name>
    <description>RTC alarm interrupt</description>
    <value>41</value>
  </interrupt>
</peripheral>*/
