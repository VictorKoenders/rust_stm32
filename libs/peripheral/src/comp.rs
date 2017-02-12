/// MOD COMP
/// Comparator
const BASE_ADDRESS: u32 = 0x4001001C;
/// control and status register
/// Size: 0x20 bits
pub mod comp1_csr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const COMP1EN_BIT_OFFSET: u8 = 0;
	const COMP1EN_BIT_WIDTH: u8 = 1;
	/// Comparator 1 enable (Width: 1, Offset: 0)
	pub fn get_comp1en() -> u8 { ::read(REGISTER_ADDRESS, COMP1EN_BIT_OFFSET, COMP1EN_BIT_WIDTH) as u8 }
	/// Comparator 1 enable (Width: 1, Offset: 0)
	pub fn set_comp1en(value: u8) { ::write(REGISTER_ADDRESS, COMP1EN_BIT_OFFSET, COMP1EN_BIT_WIDTH, value as u32); }

	const COMP1_INP_DAC_BIT_OFFSET: u8 = 1;
	const COMP1_INP_DAC_BIT_WIDTH: u8 = 1;
	/// COMP1_INP_DAC (Width: 1, Offset: 1)
	pub fn get_comp1_inp_dac() -> u8 { ::read(REGISTER_ADDRESS, COMP1_INP_DAC_BIT_OFFSET, COMP1_INP_DAC_BIT_WIDTH) as u8 }
	/// COMP1_INP_DAC (Width: 1, Offset: 1)
	pub fn set_comp1_inp_dac(value: u8) { ::write(REGISTER_ADDRESS, COMP1_INP_DAC_BIT_OFFSET, COMP1_INP_DAC_BIT_WIDTH, value as u32); }

	const COMP1MODE_BIT_OFFSET: u8 = 2;
	const COMP1MODE_BIT_WIDTH: u8 = 2;
	/// Comparator 1 mode (Width: 2, Offset: 2)
	pub fn get_comp1mode() -> u8 { ::read(REGISTER_ADDRESS, COMP1MODE_BIT_OFFSET, COMP1MODE_BIT_WIDTH) as u8 }
	/// Comparator 1 mode (Width: 2, Offset: 2)
	pub fn set_comp1mode(value: u8) { ::write(REGISTER_ADDRESS, COMP1MODE_BIT_OFFSET, COMP1MODE_BIT_WIDTH, value as u32); }

	const COMP1INSEL_BIT_OFFSET: u8 = 4;
	const COMP1INSEL_BIT_WIDTH: u8 = 3;
	/// Comparator 1 inverting input selection (Width: 3, Offset: 4)
	pub fn get_comp1insel() -> u8 { ::read(REGISTER_ADDRESS, COMP1INSEL_BIT_OFFSET, COMP1INSEL_BIT_WIDTH) as u8 }
	/// Comparator 1 inverting input selection (Width: 3, Offset: 4)
	pub fn set_comp1insel(value: u8) { ::write(REGISTER_ADDRESS, COMP1INSEL_BIT_OFFSET, COMP1INSEL_BIT_WIDTH, value as u32); }

	const COMP1_OUT_SEL_BIT_OFFSET: u8 = 10;
	const COMP1_OUT_SEL_BIT_WIDTH: u8 = 4;
	/// Comparator 1 output selection (Width: 4, Offset: 10)
	pub fn get_comp1_out_sel() -> u8 { ::read(REGISTER_ADDRESS, COMP1_OUT_SEL_BIT_OFFSET, COMP1_OUT_SEL_BIT_WIDTH) as u8 }
	/// Comparator 1 output selection (Width: 4, Offset: 10)
	pub fn set_comp1_out_sel(value: u8) { ::write(REGISTER_ADDRESS, COMP1_OUT_SEL_BIT_OFFSET, COMP1_OUT_SEL_BIT_WIDTH, value as u32); }

	const COMP1POL_BIT_OFFSET: u8 = 15;
	const COMP1POL_BIT_WIDTH: u8 = 1;
	/// Comparator 1 output polarity (Width: 1, Offset: 15)
	pub fn get_comp1pol() -> u8 { ::read(REGISTER_ADDRESS, COMP1POL_BIT_OFFSET, COMP1POL_BIT_WIDTH) as u8 }
	/// Comparator 1 output polarity (Width: 1, Offset: 15)
	pub fn set_comp1pol(value: u8) { ::write(REGISTER_ADDRESS, COMP1POL_BIT_OFFSET, COMP1POL_BIT_WIDTH, value as u32); }

	const COMP1HYST_BIT_OFFSET: u8 = 16;
	const COMP1HYST_BIT_WIDTH: u8 = 2;
	/// Comparator 1 hysteresis (Width: 2, Offset: 16)
	pub fn get_comp1hyst() -> u8 { ::read(REGISTER_ADDRESS, COMP1HYST_BIT_OFFSET, COMP1HYST_BIT_WIDTH) as u8 }
	/// Comparator 1 hysteresis (Width: 2, Offset: 16)
	pub fn set_comp1hyst(value: u8) { ::write(REGISTER_ADDRESS, COMP1HYST_BIT_OFFSET, COMP1HYST_BIT_WIDTH, value as u32); }

	const COMP1_BLANKING_BIT_OFFSET: u8 = 18;
	const COMP1_BLANKING_BIT_WIDTH: u8 = 3;
	/// Comparator 1 blanking source (Width: 3, Offset: 18)
	pub fn get_comp1_blanking() -> u8 { ::read(REGISTER_ADDRESS, COMP1_BLANKING_BIT_OFFSET, COMP1_BLANKING_BIT_WIDTH) as u8 }
	/// Comparator 1 blanking source (Width: 3, Offset: 18)
	pub fn set_comp1_blanking(value: u8) { ::write(REGISTER_ADDRESS, COMP1_BLANKING_BIT_OFFSET, COMP1_BLANKING_BIT_WIDTH, value as u32); }

	const COMP1OUT_BIT_OFFSET: u8 = 30;
	const COMP1OUT_BIT_WIDTH: u8 = 1;
	/// Comparator 1 output (Width: 1, Offset: 30)
	pub fn get_comp1out() -> u8 { ::read(REGISTER_ADDRESS, COMP1OUT_BIT_OFFSET, COMP1OUT_BIT_WIDTH) as u8 }

	const COMP1LOCK_BIT_OFFSET: u8 = 31;
	const COMP1LOCK_BIT_WIDTH: u8 = 1;
	/// Comparator 1 lock (Width: 1, Offset: 31)
	pub fn get_comp1lock() -> u8 { ::read(REGISTER_ADDRESS, COMP1LOCK_BIT_OFFSET, COMP1LOCK_BIT_WIDTH) as u8 }
	/// Comparator 1 lock (Width: 1, Offset: 31)
	pub fn set_comp1lock(value: u8) { ::write(REGISTER_ADDRESS, COMP1LOCK_BIT_OFFSET, COMP1LOCK_BIT_WIDTH, value as u32); }
}
/// control and status register
/// Size: 0x20 bits
pub mod comp2_csr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const COMP2EN_BIT_OFFSET: u8 = 0;
	const COMP2EN_BIT_WIDTH: u8 = 1;
	/// Comparator 2 enable (Width: 1, Offset: 0)
	pub fn get_comp2en() -> u8 { ::read(REGISTER_ADDRESS, COMP2EN_BIT_OFFSET, COMP2EN_BIT_WIDTH) as u8 }
	/// Comparator 2 enable (Width: 1, Offset: 0)
	pub fn set_comp2en(value: u8) { ::write(REGISTER_ADDRESS, COMP2EN_BIT_OFFSET, COMP2EN_BIT_WIDTH, value as u32); }

	const COMP2MODE_BIT_OFFSET: u8 = 2;
	const COMP2MODE_BIT_WIDTH: u8 = 2;
	/// Comparator 2 mode (Width: 2, Offset: 2)
	pub fn get_comp2mode() -> u8 { ::read(REGISTER_ADDRESS, COMP2MODE_BIT_OFFSET, COMP2MODE_BIT_WIDTH) as u8 }
	/// Comparator 2 mode (Width: 2, Offset: 2)
	pub fn set_comp2mode(value: u8) { ::write(REGISTER_ADDRESS, COMP2MODE_BIT_OFFSET, COMP2MODE_BIT_WIDTH, value as u32); }

	const COMP2INSEL_BIT_OFFSET: u8 = 4;
	const COMP2INSEL_BIT_WIDTH: u8 = 3;
	/// Comparator 2 inverting input selection (Width: 3, Offset: 4)
	pub fn get_comp2insel() -> u8 { ::read(REGISTER_ADDRESS, COMP2INSEL_BIT_OFFSET, COMP2INSEL_BIT_WIDTH) as u8 }
	/// Comparator 2 inverting input selection (Width: 3, Offset: 4)
	pub fn set_comp2insel(value: u8) { ::write(REGISTER_ADDRESS, COMP2INSEL_BIT_OFFSET, COMP2INSEL_BIT_WIDTH, value as u32); }

	const COMP2INPSEL_BIT_OFFSET: u8 = 7;
	const COMP2INPSEL_BIT_WIDTH: u8 = 1;
	/// Comparator 2 non inverted input selection (Width: 1, Offset: 7)
	pub fn get_comp2inpsel() -> u8 { ::read(REGISTER_ADDRESS, COMP2INPSEL_BIT_OFFSET, COMP2INPSEL_BIT_WIDTH) as u8 }
	/// Comparator 2 non inverted input selection (Width: 1, Offset: 7)
	pub fn set_comp2inpsel(value: u8) { ::write(REGISTER_ADDRESS, COMP2INPSEL_BIT_OFFSET, COMP2INPSEL_BIT_WIDTH, value as u32); }

	const COMP2INMSEL_BIT_OFFSET: u8 = 9;
	const COMP2INMSEL_BIT_WIDTH: u8 = 1;
	/// Comparator 1inverting input selection (Width: 1, Offset: 9)
	pub fn get_comp2inmsel() -> u8 { ::read(REGISTER_ADDRESS, COMP2INMSEL_BIT_OFFSET, COMP2INMSEL_BIT_WIDTH) as u8 }
	/// Comparator 1inverting input selection (Width: 1, Offset: 9)
	pub fn set_comp2inmsel(value: u8) { ::write(REGISTER_ADDRESS, COMP2INMSEL_BIT_OFFSET, COMP2INMSEL_BIT_WIDTH, value as u32); }

	const COMP2_OUT_SEL_BIT_OFFSET: u8 = 10;
	const COMP2_OUT_SEL_BIT_WIDTH: u8 = 4;
	/// Comparator 2 output selection (Width: 4, Offset: 10)
	pub fn get_comp2_out_sel() -> u8 { ::read(REGISTER_ADDRESS, COMP2_OUT_SEL_BIT_OFFSET, COMP2_OUT_SEL_BIT_WIDTH) as u8 }
	/// Comparator 2 output selection (Width: 4, Offset: 10)
	pub fn set_comp2_out_sel(value: u8) { ::write(REGISTER_ADDRESS, COMP2_OUT_SEL_BIT_OFFSET, COMP2_OUT_SEL_BIT_WIDTH, value as u32); }

	const COMP2POL_BIT_OFFSET: u8 = 15;
	const COMP2POL_BIT_WIDTH: u8 = 1;
	/// Comparator 2 output polarity (Width: 1, Offset: 15)
	pub fn get_comp2pol() -> u8 { ::read(REGISTER_ADDRESS, COMP2POL_BIT_OFFSET, COMP2POL_BIT_WIDTH) as u8 }
	/// Comparator 2 output polarity (Width: 1, Offset: 15)
	pub fn set_comp2pol(value: u8) { ::write(REGISTER_ADDRESS, COMP2POL_BIT_OFFSET, COMP2POL_BIT_WIDTH, value as u32); }

	const COMP2HYST_BIT_OFFSET: u8 = 16;
	const COMP2HYST_BIT_WIDTH: u8 = 2;
	/// Comparator 2 hysteresis (Width: 2, Offset: 16)
	pub fn get_comp2hyst() -> u8 { ::read(REGISTER_ADDRESS, COMP2HYST_BIT_OFFSET, COMP2HYST_BIT_WIDTH) as u8 }
	/// Comparator 2 hysteresis (Width: 2, Offset: 16)
	pub fn set_comp2hyst(value: u8) { ::write(REGISTER_ADDRESS, COMP2HYST_BIT_OFFSET, COMP2HYST_BIT_WIDTH, value as u32); }

	const COMP2_BLANKING_BIT_OFFSET: u8 = 18;
	const COMP2_BLANKING_BIT_WIDTH: u8 = 3;
	/// Comparator 2 blanking source (Width: 3, Offset: 18)
	pub fn get_comp2_blanking() -> u8 { ::read(REGISTER_ADDRESS, COMP2_BLANKING_BIT_OFFSET, COMP2_BLANKING_BIT_WIDTH) as u8 }
	/// Comparator 2 blanking source (Width: 3, Offset: 18)
	pub fn set_comp2_blanking(value: u8) { ::write(REGISTER_ADDRESS, COMP2_BLANKING_BIT_OFFSET, COMP2_BLANKING_BIT_WIDTH, value as u32); }

	const COMP2OUT_BIT_OFFSET: u8 = 30;
	const COMP2OUT_BIT_WIDTH: u8 = 1;
	/// Comparator 2 output (Width: 1, Offset: 30)
	pub fn get_comp2out() -> u8 { ::read(REGISTER_ADDRESS, COMP2OUT_BIT_OFFSET, COMP2OUT_BIT_WIDTH) as u8 }

	const COMP2LOCK_BIT_OFFSET: u8 = 31;
	const COMP2LOCK_BIT_WIDTH: u8 = 1;
	/// Comparator 2 lock (Width: 1, Offset: 31)
	pub fn get_comp2lock() -> u8 { ::read(REGISTER_ADDRESS, COMP2LOCK_BIT_OFFSET, COMP2LOCK_BIT_WIDTH) as u8 }
	/// Comparator 2 lock (Width: 1, Offset: 31)
	pub fn set_comp2lock(value: u8) { ::write(REGISTER_ADDRESS, COMP2LOCK_BIT_OFFSET, COMP2LOCK_BIT_WIDTH, value as u32); }
}
/// control and status register
/// Size: 0x20 bits
pub mod comp3_csr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const COMP3EN_BIT_OFFSET: u8 = 0;
	const COMP3EN_BIT_WIDTH: u8 = 1;
	/// Comparator 3 enable (Width: 1, Offset: 0)
	pub fn get_comp3en() -> u8 { ::read(REGISTER_ADDRESS, COMP3EN_BIT_OFFSET, COMP3EN_BIT_WIDTH) as u8 }
	/// Comparator 3 enable (Width: 1, Offset: 0)
	pub fn set_comp3en(value: u8) { ::write(REGISTER_ADDRESS, COMP3EN_BIT_OFFSET, COMP3EN_BIT_WIDTH, value as u32); }

	const COMP3MODE_BIT_OFFSET: u8 = 2;
	const COMP3MODE_BIT_WIDTH: u8 = 2;
	/// Comparator 3 mode (Width: 2, Offset: 2)
	pub fn get_comp3mode() -> u8 { ::read(REGISTER_ADDRESS, COMP3MODE_BIT_OFFSET, COMP3MODE_BIT_WIDTH) as u8 }
	/// Comparator 3 mode (Width: 2, Offset: 2)
	pub fn set_comp3mode(value: u8) { ::write(REGISTER_ADDRESS, COMP3MODE_BIT_OFFSET, COMP3MODE_BIT_WIDTH, value as u32); }

	const COMP3INSEL_BIT_OFFSET: u8 = 4;
	const COMP3INSEL_BIT_WIDTH: u8 = 3;
	/// Comparator 3 inverting input selection (Width: 3, Offset: 4)
	pub fn get_comp3insel() -> u8 { ::read(REGISTER_ADDRESS, COMP3INSEL_BIT_OFFSET, COMP3INSEL_BIT_WIDTH) as u8 }
	/// Comparator 3 inverting input selection (Width: 3, Offset: 4)
	pub fn set_comp3insel(value: u8) { ::write(REGISTER_ADDRESS, COMP3INSEL_BIT_OFFSET, COMP3INSEL_BIT_WIDTH, value as u32); }

	const COMP3INPSEL_BIT_OFFSET: u8 = 7;
	const COMP3INPSEL_BIT_WIDTH: u8 = 1;
	/// Comparator 3 non inverted input selection (Width: 1, Offset: 7)
	pub fn get_comp3inpsel() -> u8 { ::read(REGISTER_ADDRESS, COMP3INPSEL_BIT_OFFSET, COMP3INPSEL_BIT_WIDTH) as u8 }
	/// Comparator 3 non inverted input selection (Width: 1, Offset: 7)
	pub fn set_comp3inpsel(value: u8) { ::write(REGISTER_ADDRESS, COMP3INPSEL_BIT_OFFSET, COMP3INPSEL_BIT_WIDTH, value as u32); }

	const COMP3_OUT_SEL_BIT_OFFSET: u8 = 10;
	const COMP3_OUT_SEL_BIT_WIDTH: u8 = 4;
	/// Comparator 3 output selection (Width: 4, Offset: 10)
	pub fn get_comp3_out_sel() -> u8 { ::read(REGISTER_ADDRESS, COMP3_OUT_SEL_BIT_OFFSET, COMP3_OUT_SEL_BIT_WIDTH) as u8 }
	/// Comparator 3 output selection (Width: 4, Offset: 10)
	pub fn set_comp3_out_sel(value: u8) { ::write(REGISTER_ADDRESS, COMP3_OUT_SEL_BIT_OFFSET, COMP3_OUT_SEL_BIT_WIDTH, value as u32); }

	const COMP3POL_BIT_OFFSET: u8 = 15;
	const COMP3POL_BIT_WIDTH: u8 = 1;
	/// Comparator 3 output polarity (Width: 1, Offset: 15)
	pub fn get_comp3pol() -> u8 { ::read(REGISTER_ADDRESS, COMP3POL_BIT_OFFSET, COMP3POL_BIT_WIDTH) as u8 }
	/// Comparator 3 output polarity (Width: 1, Offset: 15)
	pub fn set_comp3pol(value: u8) { ::write(REGISTER_ADDRESS, COMP3POL_BIT_OFFSET, COMP3POL_BIT_WIDTH, value as u32); }

	const COMP3HYST_BIT_OFFSET: u8 = 16;
	const COMP3HYST_BIT_WIDTH: u8 = 2;
	/// Comparator 3 hysteresis (Width: 2, Offset: 16)
	pub fn get_comp3hyst() -> u8 { ::read(REGISTER_ADDRESS, COMP3HYST_BIT_OFFSET, COMP3HYST_BIT_WIDTH) as u8 }
	/// Comparator 3 hysteresis (Width: 2, Offset: 16)
	pub fn set_comp3hyst(value: u8) { ::write(REGISTER_ADDRESS, COMP3HYST_BIT_OFFSET, COMP3HYST_BIT_WIDTH, value as u32); }

	const COMP3_BLANKING_BIT_OFFSET: u8 = 18;
	const COMP3_BLANKING_BIT_WIDTH: u8 = 3;
	/// Comparator 3 blanking source (Width: 3, Offset: 18)
	pub fn get_comp3_blanking() -> u8 { ::read(REGISTER_ADDRESS, COMP3_BLANKING_BIT_OFFSET, COMP3_BLANKING_BIT_WIDTH) as u8 }
	/// Comparator 3 blanking source (Width: 3, Offset: 18)
	pub fn set_comp3_blanking(value: u8) { ::write(REGISTER_ADDRESS, COMP3_BLANKING_BIT_OFFSET, COMP3_BLANKING_BIT_WIDTH, value as u32); }

	const COMP3OUT_BIT_OFFSET: u8 = 30;
	const COMP3OUT_BIT_WIDTH: u8 = 1;
	/// Comparator 3 output (Width: 1, Offset: 30)
	pub fn get_comp3out() -> u8 { ::read(REGISTER_ADDRESS, COMP3OUT_BIT_OFFSET, COMP3OUT_BIT_WIDTH) as u8 }

	const COMP3LOCK_BIT_OFFSET: u8 = 31;
	const COMP3LOCK_BIT_WIDTH: u8 = 1;
	/// Comparator 3 lock (Width: 1, Offset: 31)
	pub fn get_comp3lock() -> u8 { ::read(REGISTER_ADDRESS, COMP3LOCK_BIT_OFFSET, COMP3LOCK_BIT_WIDTH) as u8 }
	/// Comparator 3 lock (Width: 1, Offset: 31)
	pub fn set_comp3lock(value: u8) { ::write(REGISTER_ADDRESS, COMP3LOCK_BIT_OFFSET, COMP3LOCK_BIT_WIDTH, value as u32); }
}
/// control and status register
/// Size: 0x20 bits
pub mod comp4_csr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const COMP4EN_BIT_OFFSET: u8 = 0;
	const COMP4EN_BIT_WIDTH: u8 = 1;
	/// Comparator 4 enable (Width: 1, Offset: 0)
	pub fn get_comp4en() -> u8 { ::read(REGISTER_ADDRESS, COMP4EN_BIT_OFFSET, COMP4EN_BIT_WIDTH) as u8 }
	/// Comparator 4 enable (Width: 1, Offset: 0)
	pub fn set_comp4en(value: u8) { ::write(REGISTER_ADDRESS, COMP4EN_BIT_OFFSET, COMP4EN_BIT_WIDTH, value as u32); }

	const COMP4MODE_BIT_OFFSET: u8 = 2;
	const COMP4MODE_BIT_WIDTH: u8 = 2;
	/// Comparator 4 mode (Width: 2, Offset: 2)
	pub fn get_comp4mode() -> u8 { ::read(REGISTER_ADDRESS, COMP4MODE_BIT_OFFSET, COMP4MODE_BIT_WIDTH) as u8 }
	/// Comparator 4 mode (Width: 2, Offset: 2)
	pub fn set_comp4mode(value: u8) { ::write(REGISTER_ADDRESS, COMP4MODE_BIT_OFFSET, COMP4MODE_BIT_WIDTH, value as u32); }

	const COMP4INSEL_BIT_OFFSET: u8 = 4;
	const COMP4INSEL_BIT_WIDTH: u8 = 3;
	/// Comparator 4 inverting input selection (Width: 3, Offset: 4)
	pub fn get_comp4insel() -> u8 { ::read(REGISTER_ADDRESS, COMP4INSEL_BIT_OFFSET, COMP4INSEL_BIT_WIDTH) as u8 }
	/// Comparator 4 inverting input selection (Width: 3, Offset: 4)
	pub fn set_comp4insel(value: u8) { ::write(REGISTER_ADDRESS, COMP4INSEL_BIT_OFFSET, COMP4INSEL_BIT_WIDTH, value as u32); }

	const COMP4INPSEL_BIT_OFFSET: u8 = 7;
	const COMP4INPSEL_BIT_WIDTH: u8 = 1;
	/// Comparator 4 non inverted input selection (Width: 1, Offset: 7)
	pub fn get_comp4inpsel() -> u8 { ::read(REGISTER_ADDRESS, COMP4INPSEL_BIT_OFFSET, COMP4INPSEL_BIT_WIDTH) as u8 }
	/// Comparator 4 non inverted input selection (Width: 1, Offset: 7)
	pub fn set_comp4inpsel(value: u8) { ::write(REGISTER_ADDRESS, COMP4INPSEL_BIT_OFFSET, COMP4INPSEL_BIT_WIDTH, value as u32); }

	const COM4WINMODE_BIT_OFFSET: u8 = 9;
	const COM4WINMODE_BIT_WIDTH: u8 = 1;
	/// Comparator 4 window mode (Width: 1, Offset: 9)
	pub fn get_com4winmode() -> u8 { ::read(REGISTER_ADDRESS, COM4WINMODE_BIT_OFFSET, COM4WINMODE_BIT_WIDTH) as u8 }
	/// Comparator 4 window mode (Width: 1, Offset: 9)
	pub fn set_com4winmode(value: u8) { ::write(REGISTER_ADDRESS, COM4WINMODE_BIT_OFFSET, COM4WINMODE_BIT_WIDTH, value as u32); }

	const COMP4_OUT_SEL_BIT_OFFSET: u8 = 10;
	const COMP4_OUT_SEL_BIT_WIDTH: u8 = 4;
	/// Comparator 4 output selection (Width: 4, Offset: 10)
	pub fn get_comp4_out_sel() -> u8 { ::read(REGISTER_ADDRESS, COMP4_OUT_SEL_BIT_OFFSET, COMP4_OUT_SEL_BIT_WIDTH) as u8 }
	/// Comparator 4 output selection (Width: 4, Offset: 10)
	pub fn set_comp4_out_sel(value: u8) { ::write(REGISTER_ADDRESS, COMP4_OUT_SEL_BIT_OFFSET, COMP4_OUT_SEL_BIT_WIDTH, value as u32); }

	const COMP4POL_BIT_OFFSET: u8 = 15;
	const COMP4POL_BIT_WIDTH: u8 = 1;
	/// Comparator 4 output polarity (Width: 1, Offset: 15)
	pub fn get_comp4pol() -> u8 { ::read(REGISTER_ADDRESS, COMP4POL_BIT_OFFSET, COMP4POL_BIT_WIDTH) as u8 }
	/// Comparator 4 output polarity (Width: 1, Offset: 15)
	pub fn set_comp4pol(value: u8) { ::write(REGISTER_ADDRESS, COMP4POL_BIT_OFFSET, COMP4POL_BIT_WIDTH, value as u32); }

	const COMP4HYST_BIT_OFFSET: u8 = 16;
	const COMP4HYST_BIT_WIDTH: u8 = 2;
	/// Comparator 4 hysteresis (Width: 2, Offset: 16)
	pub fn get_comp4hyst() -> u8 { ::read(REGISTER_ADDRESS, COMP4HYST_BIT_OFFSET, COMP4HYST_BIT_WIDTH) as u8 }
	/// Comparator 4 hysteresis (Width: 2, Offset: 16)
	pub fn set_comp4hyst(value: u8) { ::write(REGISTER_ADDRESS, COMP4HYST_BIT_OFFSET, COMP4HYST_BIT_WIDTH, value as u32); }

	const COMP4_BLANKING_BIT_OFFSET: u8 = 18;
	const COMP4_BLANKING_BIT_WIDTH: u8 = 3;
	/// Comparator 4 blanking source (Width: 3, Offset: 18)
	pub fn get_comp4_blanking() -> u8 { ::read(REGISTER_ADDRESS, COMP4_BLANKING_BIT_OFFSET, COMP4_BLANKING_BIT_WIDTH) as u8 }
	/// Comparator 4 blanking source (Width: 3, Offset: 18)
	pub fn set_comp4_blanking(value: u8) { ::write(REGISTER_ADDRESS, COMP4_BLANKING_BIT_OFFSET, COMP4_BLANKING_BIT_WIDTH, value as u32); }

	const COMP4OUT_BIT_OFFSET: u8 = 30;
	const COMP4OUT_BIT_WIDTH: u8 = 1;
	/// Comparator 4 output (Width: 1, Offset: 30)
	pub fn get_comp4out() -> u8 { ::read(REGISTER_ADDRESS, COMP4OUT_BIT_OFFSET, COMP4OUT_BIT_WIDTH) as u8 }

	const COMP4LOCK_BIT_OFFSET: u8 = 31;
	const COMP4LOCK_BIT_WIDTH: u8 = 1;
	/// Comparator 4 lock (Width: 1, Offset: 31)
	pub fn get_comp4lock() -> u8 { ::read(REGISTER_ADDRESS, COMP4LOCK_BIT_OFFSET, COMP4LOCK_BIT_WIDTH) as u8 }
	/// Comparator 4 lock (Width: 1, Offset: 31)
	pub fn set_comp4lock(value: u8) { ::write(REGISTER_ADDRESS, COMP4LOCK_BIT_OFFSET, COMP4LOCK_BIT_WIDTH, value as u32); }
}
/// control and status register
/// Size: 0x20 bits
pub mod comp5_csr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const COMP5EN_BIT_OFFSET: u8 = 0;
	const COMP5EN_BIT_WIDTH: u8 = 1;
	/// Comparator 5 enable (Width: 1, Offset: 0)
	pub fn get_comp5en() -> u8 { ::read(REGISTER_ADDRESS, COMP5EN_BIT_OFFSET, COMP5EN_BIT_WIDTH) as u8 }
	/// Comparator 5 enable (Width: 1, Offset: 0)
	pub fn set_comp5en(value: u8) { ::write(REGISTER_ADDRESS, COMP5EN_BIT_OFFSET, COMP5EN_BIT_WIDTH, value as u32); }

	const COMP5MODE_BIT_OFFSET: u8 = 2;
	const COMP5MODE_BIT_WIDTH: u8 = 2;
	/// Comparator 5 mode (Width: 2, Offset: 2)
	pub fn get_comp5mode() -> u8 { ::read(REGISTER_ADDRESS, COMP5MODE_BIT_OFFSET, COMP5MODE_BIT_WIDTH) as u8 }
	/// Comparator 5 mode (Width: 2, Offset: 2)
	pub fn set_comp5mode(value: u8) { ::write(REGISTER_ADDRESS, COMP5MODE_BIT_OFFSET, COMP5MODE_BIT_WIDTH, value as u32); }

	const COMP5INSEL_BIT_OFFSET: u8 = 4;
	const COMP5INSEL_BIT_WIDTH: u8 = 3;
	/// Comparator 5 inverting input selection (Width: 3, Offset: 4)
	pub fn get_comp5insel() -> u8 { ::read(REGISTER_ADDRESS, COMP5INSEL_BIT_OFFSET, COMP5INSEL_BIT_WIDTH) as u8 }
	/// Comparator 5 inverting input selection (Width: 3, Offset: 4)
	pub fn set_comp5insel(value: u8) { ::write(REGISTER_ADDRESS, COMP5INSEL_BIT_OFFSET, COMP5INSEL_BIT_WIDTH, value as u32); }

	const COMP5INPSEL_BIT_OFFSET: u8 = 7;
	const COMP5INPSEL_BIT_WIDTH: u8 = 1;
	/// Comparator 5 non inverted input selection (Width: 1, Offset: 7)
	pub fn get_comp5inpsel() -> u8 { ::read(REGISTER_ADDRESS, COMP5INPSEL_BIT_OFFSET, COMP5INPSEL_BIT_WIDTH) as u8 }
	/// Comparator 5 non inverted input selection (Width: 1, Offset: 7)
	pub fn set_comp5inpsel(value: u8) { ::write(REGISTER_ADDRESS, COMP5INPSEL_BIT_OFFSET, COMP5INPSEL_BIT_WIDTH, value as u32); }

	const COMP5_OUT_SEL_BIT_OFFSET: u8 = 10;
	const COMP5_OUT_SEL_BIT_WIDTH: u8 = 4;
	/// Comparator 5 output selection (Width: 4, Offset: 10)
	pub fn get_comp5_out_sel() -> u8 { ::read(REGISTER_ADDRESS, COMP5_OUT_SEL_BIT_OFFSET, COMP5_OUT_SEL_BIT_WIDTH) as u8 }
	/// Comparator 5 output selection (Width: 4, Offset: 10)
	pub fn set_comp5_out_sel(value: u8) { ::write(REGISTER_ADDRESS, COMP5_OUT_SEL_BIT_OFFSET, COMP5_OUT_SEL_BIT_WIDTH, value as u32); }

	const COMP5POL_BIT_OFFSET: u8 = 15;
	const COMP5POL_BIT_WIDTH: u8 = 1;
	/// Comparator 5 output polarity (Width: 1, Offset: 15)
	pub fn get_comp5pol() -> u8 { ::read(REGISTER_ADDRESS, COMP5POL_BIT_OFFSET, COMP5POL_BIT_WIDTH) as u8 }
	/// Comparator 5 output polarity (Width: 1, Offset: 15)
	pub fn set_comp5pol(value: u8) { ::write(REGISTER_ADDRESS, COMP5POL_BIT_OFFSET, COMP5POL_BIT_WIDTH, value as u32); }

	const COMP5HYST_BIT_OFFSET: u8 = 16;
	const COMP5HYST_BIT_WIDTH: u8 = 2;
	/// Comparator 5 hysteresis (Width: 2, Offset: 16)
	pub fn get_comp5hyst() -> u8 { ::read(REGISTER_ADDRESS, COMP5HYST_BIT_OFFSET, COMP5HYST_BIT_WIDTH) as u8 }
	/// Comparator 5 hysteresis (Width: 2, Offset: 16)
	pub fn set_comp5hyst(value: u8) { ::write(REGISTER_ADDRESS, COMP5HYST_BIT_OFFSET, COMP5HYST_BIT_WIDTH, value as u32); }

	const COMP5_BLANKING_BIT_OFFSET: u8 = 18;
	const COMP5_BLANKING_BIT_WIDTH: u8 = 3;
	/// Comparator 5 blanking source (Width: 3, Offset: 18)
	pub fn get_comp5_blanking() -> u8 { ::read(REGISTER_ADDRESS, COMP5_BLANKING_BIT_OFFSET, COMP5_BLANKING_BIT_WIDTH) as u8 }
	/// Comparator 5 blanking source (Width: 3, Offset: 18)
	pub fn set_comp5_blanking(value: u8) { ::write(REGISTER_ADDRESS, COMP5_BLANKING_BIT_OFFSET, COMP5_BLANKING_BIT_WIDTH, value as u32); }

	const COMP5OUT_BIT_OFFSET: u8 = 30;
	const COMP5OUT_BIT_WIDTH: u8 = 1;
	/// Comparator51 output (Width: 1, Offset: 30)
	pub fn get_comp5out() -> u8 { ::read(REGISTER_ADDRESS, COMP5OUT_BIT_OFFSET, COMP5OUT_BIT_WIDTH) as u8 }

	const COMP5LOCK_BIT_OFFSET: u8 = 31;
	const COMP5LOCK_BIT_WIDTH: u8 = 1;
	/// Comparator 5 lock (Width: 1, Offset: 31)
	pub fn get_comp5lock() -> u8 { ::read(REGISTER_ADDRESS, COMP5LOCK_BIT_OFFSET, COMP5LOCK_BIT_WIDTH) as u8 }
	/// Comparator 5 lock (Width: 1, Offset: 31)
	pub fn set_comp5lock(value: u8) { ::write(REGISTER_ADDRESS, COMP5LOCK_BIT_OFFSET, COMP5LOCK_BIT_WIDTH, value as u32); }
}
/// control and status register
/// Size: 0x20 bits
pub mod comp6_csr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const COMP6EN_BIT_OFFSET: u8 = 0;
	const COMP6EN_BIT_WIDTH: u8 = 1;
	/// Comparator 6 enable (Width: 1, Offset: 0)
	pub fn get_comp6en() -> u8 { ::read(REGISTER_ADDRESS, COMP6EN_BIT_OFFSET, COMP6EN_BIT_WIDTH) as u8 }
	/// Comparator 6 enable (Width: 1, Offset: 0)
	pub fn set_comp6en(value: u8) { ::write(REGISTER_ADDRESS, COMP6EN_BIT_OFFSET, COMP6EN_BIT_WIDTH, value as u32); }

	const COMP6MODE_BIT_OFFSET: u8 = 2;
	const COMP6MODE_BIT_WIDTH: u8 = 2;
	/// Comparator 6 mode (Width: 2, Offset: 2)
	pub fn get_comp6mode() -> u8 { ::read(REGISTER_ADDRESS, COMP6MODE_BIT_OFFSET, COMP6MODE_BIT_WIDTH) as u8 }
	/// Comparator 6 mode (Width: 2, Offset: 2)
	pub fn set_comp6mode(value: u8) { ::write(REGISTER_ADDRESS, COMP6MODE_BIT_OFFSET, COMP6MODE_BIT_WIDTH, value as u32); }

	const COMP6INSEL_BIT_OFFSET: u8 = 4;
	const COMP6INSEL_BIT_WIDTH: u8 = 3;
	/// Comparator 6 inverting input selection (Width: 3, Offset: 4)
	pub fn get_comp6insel() -> u8 { ::read(REGISTER_ADDRESS, COMP6INSEL_BIT_OFFSET, COMP6INSEL_BIT_WIDTH) as u8 }
	/// Comparator 6 inverting input selection (Width: 3, Offset: 4)
	pub fn set_comp6insel(value: u8) { ::write(REGISTER_ADDRESS, COMP6INSEL_BIT_OFFSET, COMP6INSEL_BIT_WIDTH, value as u32); }

	const COMP6INPSEL_BIT_OFFSET: u8 = 7;
	const COMP6INPSEL_BIT_WIDTH: u8 = 1;
	/// Comparator 6 non inverted input selection (Width: 1, Offset: 7)
	pub fn get_comp6inpsel() -> u8 { ::read(REGISTER_ADDRESS, COMP6INPSEL_BIT_OFFSET, COMP6INPSEL_BIT_WIDTH) as u8 }
	/// Comparator 6 non inverted input selection (Width: 1, Offset: 7)
	pub fn set_comp6inpsel(value: u8) { ::write(REGISTER_ADDRESS, COMP6INPSEL_BIT_OFFSET, COMP6INPSEL_BIT_WIDTH, value as u32); }

	const COM6WINMODE_BIT_OFFSET: u8 = 9;
	const COM6WINMODE_BIT_WIDTH: u8 = 1;
	/// Comparator 6 window mode (Width: 1, Offset: 9)
	pub fn get_com6winmode() -> u8 { ::read(REGISTER_ADDRESS, COM6WINMODE_BIT_OFFSET, COM6WINMODE_BIT_WIDTH) as u8 }
	/// Comparator 6 window mode (Width: 1, Offset: 9)
	pub fn set_com6winmode(value: u8) { ::write(REGISTER_ADDRESS, COM6WINMODE_BIT_OFFSET, COM6WINMODE_BIT_WIDTH, value as u32); }

	const COMP6_OUT_SEL_BIT_OFFSET: u8 = 10;
	const COMP6_OUT_SEL_BIT_WIDTH: u8 = 4;
	/// Comparator 6 output selection (Width: 4, Offset: 10)
	pub fn get_comp6_out_sel() -> u8 { ::read(REGISTER_ADDRESS, COMP6_OUT_SEL_BIT_OFFSET, COMP6_OUT_SEL_BIT_WIDTH) as u8 }
	/// Comparator 6 output selection (Width: 4, Offset: 10)
	pub fn set_comp6_out_sel(value: u8) { ::write(REGISTER_ADDRESS, COMP6_OUT_SEL_BIT_OFFSET, COMP6_OUT_SEL_BIT_WIDTH, value as u32); }

	const COMP6POL_BIT_OFFSET: u8 = 15;
	const COMP6POL_BIT_WIDTH: u8 = 1;
	/// Comparator 6 output polarity (Width: 1, Offset: 15)
	pub fn get_comp6pol() -> u8 { ::read(REGISTER_ADDRESS, COMP6POL_BIT_OFFSET, COMP6POL_BIT_WIDTH) as u8 }
	/// Comparator 6 output polarity (Width: 1, Offset: 15)
	pub fn set_comp6pol(value: u8) { ::write(REGISTER_ADDRESS, COMP6POL_BIT_OFFSET, COMP6POL_BIT_WIDTH, value as u32); }

	const COMP6HYST_BIT_OFFSET: u8 = 16;
	const COMP6HYST_BIT_WIDTH: u8 = 2;
	/// Comparator 6 hysteresis (Width: 2, Offset: 16)
	pub fn get_comp6hyst() -> u8 { ::read(REGISTER_ADDRESS, COMP6HYST_BIT_OFFSET, COMP6HYST_BIT_WIDTH) as u8 }
	/// Comparator 6 hysteresis (Width: 2, Offset: 16)
	pub fn set_comp6hyst(value: u8) { ::write(REGISTER_ADDRESS, COMP6HYST_BIT_OFFSET, COMP6HYST_BIT_WIDTH, value as u32); }

	const COMP6_BLANKING_BIT_OFFSET: u8 = 18;
	const COMP6_BLANKING_BIT_WIDTH: u8 = 3;
	/// Comparator 6 blanking source (Width: 3, Offset: 18)
	pub fn get_comp6_blanking() -> u8 { ::read(REGISTER_ADDRESS, COMP6_BLANKING_BIT_OFFSET, COMP6_BLANKING_BIT_WIDTH) as u8 }
	/// Comparator 6 blanking source (Width: 3, Offset: 18)
	pub fn set_comp6_blanking(value: u8) { ::write(REGISTER_ADDRESS, COMP6_BLANKING_BIT_OFFSET, COMP6_BLANKING_BIT_WIDTH, value as u32); }

	const COMP6OUT_BIT_OFFSET: u8 = 30;
	const COMP6OUT_BIT_WIDTH: u8 = 1;
	/// Comparator 6 output (Width: 1, Offset: 30)
	pub fn get_comp6out() -> u8 { ::read(REGISTER_ADDRESS, COMP6OUT_BIT_OFFSET, COMP6OUT_BIT_WIDTH) as u8 }

	const COMP6LOCK_BIT_OFFSET: u8 = 31;
	const COMP6LOCK_BIT_WIDTH: u8 = 1;
	/// Comparator 6 lock (Width: 1, Offset: 31)
	pub fn get_comp6lock() -> u8 { ::read(REGISTER_ADDRESS, COMP6LOCK_BIT_OFFSET, COMP6LOCK_BIT_WIDTH) as u8 }
	/// Comparator 6 lock (Width: 1, Offset: 31)
	pub fn set_comp6lock(value: u8) { ::write(REGISTER_ADDRESS, COMP6LOCK_BIT_OFFSET, COMP6LOCK_BIT_WIDTH, value as u32); }
}
/// control and status register
/// Size: 0x20 bits
pub mod comp7_csr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const COMP7EN_BIT_OFFSET: u8 = 0;
	const COMP7EN_BIT_WIDTH: u8 = 1;
	/// Comparator 7 enable (Width: 1, Offset: 0)
	pub fn get_comp7en() -> u8 { ::read(REGISTER_ADDRESS, COMP7EN_BIT_OFFSET, COMP7EN_BIT_WIDTH) as u8 }
	/// Comparator 7 enable (Width: 1, Offset: 0)
	pub fn set_comp7en(value: u8) { ::write(REGISTER_ADDRESS, COMP7EN_BIT_OFFSET, COMP7EN_BIT_WIDTH, value as u32); }

	const COMP7MODE_BIT_OFFSET: u8 = 2;
	const COMP7MODE_BIT_WIDTH: u8 = 2;
	/// Comparator 7 mode (Width: 2, Offset: 2)
	pub fn get_comp7mode() -> u8 { ::read(REGISTER_ADDRESS, COMP7MODE_BIT_OFFSET, COMP7MODE_BIT_WIDTH) as u8 }
	/// Comparator 7 mode (Width: 2, Offset: 2)
	pub fn set_comp7mode(value: u8) { ::write(REGISTER_ADDRESS, COMP7MODE_BIT_OFFSET, COMP7MODE_BIT_WIDTH, value as u32); }

	const COMP7INSEL_BIT_OFFSET: u8 = 4;
	const COMP7INSEL_BIT_WIDTH: u8 = 3;
	/// Comparator 7 inverting input selection (Width: 3, Offset: 4)
	pub fn get_comp7insel() -> u8 { ::read(REGISTER_ADDRESS, COMP7INSEL_BIT_OFFSET, COMP7INSEL_BIT_WIDTH) as u8 }
	/// Comparator 7 inverting input selection (Width: 3, Offset: 4)
	pub fn set_comp7insel(value: u8) { ::write(REGISTER_ADDRESS, COMP7INSEL_BIT_OFFSET, COMP7INSEL_BIT_WIDTH, value as u32); }

	const COMP7INPSEL_BIT_OFFSET: u8 = 7;
	const COMP7INPSEL_BIT_WIDTH: u8 = 1;
	/// Comparator 7 non inverted input selection (Width: 1, Offset: 7)
	pub fn get_comp7inpsel() -> u8 { ::read(REGISTER_ADDRESS, COMP7INPSEL_BIT_OFFSET, COMP7INPSEL_BIT_WIDTH) as u8 }
	/// Comparator 7 non inverted input selection (Width: 1, Offset: 7)
	pub fn set_comp7inpsel(value: u8) { ::write(REGISTER_ADDRESS, COMP7INPSEL_BIT_OFFSET, COMP7INPSEL_BIT_WIDTH, value as u32); }

	const COMP7_OUT_SEL_BIT_OFFSET: u8 = 10;
	const COMP7_OUT_SEL_BIT_WIDTH: u8 = 4;
	/// Comparator 7 output selection (Width: 4, Offset: 10)
	pub fn get_comp7_out_sel() -> u8 { ::read(REGISTER_ADDRESS, COMP7_OUT_SEL_BIT_OFFSET, COMP7_OUT_SEL_BIT_WIDTH) as u8 }
	/// Comparator 7 output selection (Width: 4, Offset: 10)
	pub fn set_comp7_out_sel(value: u8) { ::write(REGISTER_ADDRESS, COMP7_OUT_SEL_BIT_OFFSET, COMP7_OUT_SEL_BIT_WIDTH, value as u32); }

	const COMP7POL_BIT_OFFSET: u8 = 15;
	const COMP7POL_BIT_WIDTH: u8 = 1;
	/// Comparator 7 output polarity (Width: 1, Offset: 15)
	pub fn get_comp7pol() -> u8 { ::read(REGISTER_ADDRESS, COMP7POL_BIT_OFFSET, COMP7POL_BIT_WIDTH) as u8 }
	/// Comparator 7 output polarity (Width: 1, Offset: 15)
	pub fn set_comp7pol(value: u8) { ::write(REGISTER_ADDRESS, COMP7POL_BIT_OFFSET, COMP7POL_BIT_WIDTH, value as u32); }

	const COMP7HYST_BIT_OFFSET: u8 = 16;
	const COMP7HYST_BIT_WIDTH: u8 = 2;
	/// Comparator 7 hysteresis (Width: 2, Offset: 16)
	pub fn get_comp7hyst() -> u8 { ::read(REGISTER_ADDRESS, COMP7HYST_BIT_OFFSET, COMP7HYST_BIT_WIDTH) as u8 }
	/// Comparator 7 hysteresis (Width: 2, Offset: 16)
	pub fn set_comp7hyst(value: u8) { ::write(REGISTER_ADDRESS, COMP7HYST_BIT_OFFSET, COMP7HYST_BIT_WIDTH, value as u32); }

	const COMP7_BLANKING_BIT_OFFSET: u8 = 18;
	const COMP7_BLANKING_BIT_WIDTH: u8 = 3;
	/// Comparator 7 blanking source (Width: 3, Offset: 18)
	pub fn get_comp7_blanking() -> u8 { ::read(REGISTER_ADDRESS, COMP7_BLANKING_BIT_OFFSET, COMP7_BLANKING_BIT_WIDTH) as u8 }
	/// Comparator 7 blanking source (Width: 3, Offset: 18)
	pub fn set_comp7_blanking(value: u8) { ::write(REGISTER_ADDRESS, COMP7_BLANKING_BIT_OFFSET, COMP7_BLANKING_BIT_WIDTH, value as u32); }

	const COMP7OUT_BIT_OFFSET: u8 = 30;
	const COMP7OUT_BIT_WIDTH: u8 = 1;
	/// Comparator 7 output (Width: 1, Offset: 30)
	pub fn get_comp7out() -> u8 { ::read(REGISTER_ADDRESS, COMP7OUT_BIT_OFFSET, COMP7OUT_BIT_WIDTH) as u8 }

	const COMP7LOCK_BIT_OFFSET: u8 = 31;
	const COMP7LOCK_BIT_WIDTH: u8 = 1;
	/// Comparator 7 lock (Width: 1, Offset: 31)
	pub fn get_comp7lock() -> u8 { ::read(REGISTER_ADDRESS, COMP7LOCK_BIT_OFFSET, COMP7LOCK_BIT_WIDTH) as u8 }
	/// Comparator 7 lock (Width: 1, Offset: 31)
	pub fn set_comp7lock(value: u8) { ::write(REGISTER_ADDRESS, COMP7LOCK_BIT_OFFSET, COMP7LOCK_BIT_WIDTH, value as u32); }
}
/// COMP1 & COMP2 & COMP3 interrupts combined with EXTI Lines 21, 22 and 29 interrupts
pub const INTERRUPT_COMP123: u32 = 64;

/// COMP4 & COMP5 & COMP6 interrupts combined with EXTI Lines 30, 31 and 32 interrupts
pub const INTERRUPT_COMP456: u32 = 65;

/// COMP7 interrupt combined with EXTI Line 33 interrupt
pub const INTERRUPT_COMP7: u32 = 66;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>COMP</name>
  <description>Comparator</description>
  <groupName>COMP</groupName>
  <baseAddress>0x4001001C</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x19</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>COMP1_CSR</name>
      <displayName>COMP1_CSR</displayName>
      <description>control and status register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>COMP1EN</name>
          <description>Comparator 1 enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP1_INP_DAC</name>
          <description>COMP1_INP_DAC</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP1MODE</name>
          <description>Comparator 1 mode</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP1INSEL</name>
          <description>Comparator 1 inverting input
              selection</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP1_OUT_SEL</name>
          <description>Comparator 1 output
              selection</description>
          <bitOffset>10</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP1POL</name>
          <description>Comparator 1 output
              polarity</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP1HYST</name>
          <description>Comparator 1 hysteresis</description>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP1_BLANKING</name>
          <description>Comparator 1 blanking
              source</description>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP1OUT</name>
          <description>Comparator 1 output</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>COMP1LOCK</name>
          <description>Comparator 1 lock</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>COMP2_CSR</name>
      <displayName>COMP2_CSR</displayName>
      <description>control and status register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>COMP2EN</name>
          <description>Comparator 2 enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP2MODE</name>
          <description>Comparator 2 mode</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP2INSEL</name>
          <description>Comparator 2 inverting input
              selection</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP2INPSEL</name>
          <description>Comparator 2 non inverted input
              selection</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP2INMSEL</name>
          <description>Comparator 1inverting input
              selection</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP2_OUT_SEL</name>
          <description>Comparator 2 output
              selection</description>
          <bitOffset>10</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP2POL</name>
          <description>Comparator 2 output
              polarity</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP2HYST</name>
          <description>Comparator 2 hysteresis</description>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP2_BLANKING</name>
          <description>Comparator 2 blanking
              source</description>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP2OUT</name>
          <description>Comparator 2 output</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>COMP2LOCK</name>
          <description>Comparator 2 lock</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>COMP3_CSR</name>
      <displayName>COMP3_CSR</displayName>
      <description>control and status register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>COMP3EN</name>
          <description>Comparator 3 enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP3MODE</name>
          <description>Comparator 3 mode</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP3INSEL</name>
          <description>Comparator 3 inverting input
              selection</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP3INPSEL</name>
          <description>Comparator 3 non inverted input
              selection</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP3_OUT_SEL</name>
          <description>Comparator 3 output
              selection</description>
          <bitOffset>10</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP3POL</name>
          <description>Comparator 3 output
              polarity</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP3HYST</name>
          <description>Comparator 3 hysteresis</description>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP3_BLANKING</name>
          <description>Comparator 3 blanking
              source</description>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP3OUT</name>
          <description>Comparator 3 output</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>COMP3LOCK</name>
          <description>Comparator 3 lock</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>COMP4_CSR</name>
      <displayName>COMP4_CSR</displayName>
      <description>control and status register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>COMP4EN</name>
          <description>Comparator 4 enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP4MODE</name>
          <description>Comparator 4 mode</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP4INSEL</name>
          <description>Comparator 4 inverting input
              selection</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP4INPSEL</name>
          <description>Comparator 4 non inverted input
              selection</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COM4WINMODE</name>
          <description>Comparator 4 window mode</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP4_OUT_SEL</name>
          <description>Comparator 4 output
              selection</description>
          <bitOffset>10</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP4POL</name>
          <description>Comparator 4 output
              polarity</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP4HYST</name>
          <description>Comparator 4 hysteresis</description>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP4_BLANKING</name>
          <description>Comparator 4 blanking
              source</description>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP4OUT</name>
          <description>Comparator 4 output</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>COMP4LOCK</name>
          <description>Comparator 4 lock</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>COMP5_CSR</name>
      <displayName>COMP5_CSR</displayName>
      <description>control and status register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>COMP5EN</name>
          <description>Comparator 5 enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP5MODE</name>
          <description>Comparator 5 mode</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP5INSEL</name>
          <description>Comparator 5 inverting input
              selection</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP5INPSEL</name>
          <description>Comparator 5 non inverted input
              selection</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP5_OUT_SEL</name>
          <description>Comparator 5 output
              selection</description>
          <bitOffset>10</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP5POL</name>
          <description>Comparator 5 output
              polarity</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP5HYST</name>
          <description>Comparator 5 hysteresis</description>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP5_BLANKING</name>
          <description>Comparator 5 blanking
              source</description>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP5OUT</name>
          <description>Comparator51 output</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>COMP5LOCK</name>
          <description>Comparator 5 lock</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>COMP6_CSR</name>
      <displayName>COMP6_CSR</displayName>
      <description>control and status register</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>COMP6EN</name>
          <description>Comparator 6 enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP6MODE</name>
          <description>Comparator 6 mode</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP6INSEL</name>
          <description>Comparator 6 inverting input
              selection</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP6INPSEL</name>
          <description>Comparator 6 non inverted input
              selection</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COM6WINMODE</name>
          <description>Comparator 6 window mode</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP6_OUT_SEL</name>
          <description>Comparator 6 output
              selection</description>
          <bitOffset>10</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP6POL</name>
          <description>Comparator 6 output
              polarity</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP6HYST</name>
          <description>Comparator 6 hysteresis</description>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP6_BLANKING</name>
          <description>Comparator 6 blanking
              source</description>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP6OUT</name>
          <description>Comparator 6 output</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>COMP6LOCK</name>
          <description>Comparator 6 lock</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>COMP7_CSR</name>
      <displayName>COMP7_CSR</displayName>
      <description>control and status register</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>COMP7EN</name>
          <description>Comparator 7 enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP7MODE</name>
          <description>Comparator 7 mode</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP7INSEL</name>
          <description>Comparator 7 inverting input
              selection</description>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP7INPSEL</name>
          <description>Comparator 7 non inverted input
              selection</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP7_OUT_SEL</name>
          <description>Comparator 7 output
              selection</description>
          <bitOffset>10</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP7POL</name>
          <description>Comparator 7 output
              polarity</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP7HYST</name>
          <description>Comparator 7 hysteresis</description>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP7_BLANKING</name>
          <description>Comparator 7 blanking
              source</description>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>COMP7OUT</name>
          <description>Comparator 7 output</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>COMP7LOCK</name>
          <description>Comparator 7 lock</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>COMP123</name>
    <description>COMP1 &amp; COMP2 &amp; COMP3 interrupts
        combined with EXTI Lines 21, 22 and 29
        interrupts</description>
    <value>64</value>
  </interrupt>
  <interrupt>
    <name>COMP456</name>
    <description>COMP4 &amp; COMP5 &amp; COMP6 interrupts
        combined with EXTI Lines 30, 31 and 32
        interrupts</description>
    <value>65</value>
  </interrupt>
  <interrupt>
    <name>COMP7</name>
    <description>COMP7 interrupt combined with EXTI Line 33
        interrupt</description>
    <value>66</value>
  </interrupt>
</peripheral>*/
