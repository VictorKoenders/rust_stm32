/// MOD OPAMP
/// Operational amplifier
const BASE_ADDRESS: u32 = 0x40010038;
/// OPAMP1 control register
/// Size: 0x20 bits
pub mod opamp1_cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OPAMP1_EN_BIT_OFFSET: u8 = 0;
	const OPAMP1_EN_BIT_WIDTH: u8 = 1;
	/// OPAMP1 enable (Width: 1, Offset: 0)
	pub fn get_opamp1_en() -> u8 { ::read(REGISTER_ADDRESS, OPAMP1_EN_BIT_OFFSET, OPAMP1_EN_BIT_WIDTH) as u8 }
	/// OPAMP1 enable (Width: 1, Offset: 0)
	pub fn set_opamp1_en(value: u8) { ::write(REGISTER_ADDRESS, OPAMP1_EN_BIT_OFFSET, OPAMP1_EN_BIT_WIDTH, value as u32); }

	const FORCE_VP_BIT_OFFSET: u8 = 1;
	const FORCE_VP_BIT_WIDTH: u8 = 1;
	/// FORCE_VP (Width: 1, Offset: 1)
	pub fn get_force_vp() -> u8 { ::read(REGISTER_ADDRESS, FORCE_VP_BIT_OFFSET, FORCE_VP_BIT_WIDTH) as u8 }
	/// FORCE_VP (Width: 1, Offset: 1)
	pub fn set_force_vp(value: u8) { ::write(REGISTER_ADDRESS, FORCE_VP_BIT_OFFSET, FORCE_VP_BIT_WIDTH, value as u32); }

	const VP_SEL_BIT_OFFSET: u8 = 2;
	const VP_SEL_BIT_WIDTH: u8 = 2;
	/// OPAMP1 Non inverting input selection (Width: 2, Offset: 2)
	pub fn get_vp_sel() -> u8 { ::read(REGISTER_ADDRESS, VP_SEL_BIT_OFFSET, VP_SEL_BIT_WIDTH) as u8 }
	/// OPAMP1 Non inverting input selection (Width: 2, Offset: 2)
	pub fn set_vp_sel(value: u8) { ::write(REGISTER_ADDRESS, VP_SEL_BIT_OFFSET, VP_SEL_BIT_WIDTH, value as u32); }

	const VM_SEL_BIT_OFFSET: u8 = 5;
	const VM_SEL_BIT_WIDTH: u8 = 2;
	/// OPAMP1 inverting input selection (Width: 2, Offset: 5)
	pub fn get_vm_sel() -> u8 { ::read(REGISTER_ADDRESS, VM_SEL_BIT_OFFSET, VM_SEL_BIT_WIDTH) as u8 }
	/// OPAMP1 inverting input selection (Width: 2, Offset: 5)
	pub fn set_vm_sel(value: u8) { ::write(REGISTER_ADDRESS, VM_SEL_BIT_OFFSET, VM_SEL_BIT_WIDTH, value as u32); }

	const TCM_EN_BIT_OFFSET: u8 = 7;
	const TCM_EN_BIT_WIDTH: u8 = 1;
	/// Timer controlled Mux mode enable (Width: 1, Offset: 7)
	pub fn get_tcm_en() -> u8 { ::read(REGISTER_ADDRESS, TCM_EN_BIT_OFFSET, TCM_EN_BIT_WIDTH) as u8 }
	/// Timer controlled Mux mode enable (Width: 1, Offset: 7)
	pub fn set_tcm_en(value: u8) { ::write(REGISTER_ADDRESS, TCM_EN_BIT_OFFSET, TCM_EN_BIT_WIDTH, value as u32); }

	const VMS_SEL_BIT_OFFSET: u8 = 8;
	const VMS_SEL_BIT_WIDTH: u8 = 1;
	/// OPAMP1 inverting input secondary selection (Width: 1, Offset: 8)
	pub fn get_vms_sel() -> u8 { ::read(REGISTER_ADDRESS, VMS_SEL_BIT_OFFSET, VMS_SEL_BIT_WIDTH) as u8 }
	/// OPAMP1 inverting input secondary selection (Width: 1, Offset: 8)
	pub fn set_vms_sel(value: u8) { ::write(REGISTER_ADDRESS, VMS_SEL_BIT_OFFSET, VMS_SEL_BIT_WIDTH, value as u32); }

	const VPS_SEL_BIT_OFFSET: u8 = 9;
	const VPS_SEL_BIT_WIDTH: u8 = 2;
	/// OPAMP1 Non inverting input secondary selection (Width: 2, Offset: 9)
	pub fn get_vps_sel() -> u8 { ::read(REGISTER_ADDRESS, VPS_SEL_BIT_OFFSET, VPS_SEL_BIT_WIDTH) as u8 }
	/// OPAMP1 Non inverting input secondary selection (Width: 2, Offset: 9)
	pub fn set_vps_sel(value: u8) { ::write(REGISTER_ADDRESS, VPS_SEL_BIT_OFFSET, VPS_SEL_BIT_WIDTH, value as u32); }

	const CALON_BIT_OFFSET: u8 = 11;
	const CALON_BIT_WIDTH: u8 = 1;
	/// Calibration mode enable (Width: 1, Offset: 11)
	pub fn get_calon() -> u8 { ::read(REGISTER_ADDRESS, CALON_BIT_OFFSET, CALON_BIT_WIDTH) as u8 }
	/// Calibration mode enable (Width: 1, Offset: 11)
	pub fn set_calon(value: u8) { ::write(REGISTER_ADDRESS, CALON_BIT_OFFSET, CALON_BIT_WIDTH, value as u32); }

	const CALSEL_BIT_OFFSET: u8 = 12;
	const CALSEL_BIT_WIDTH: u8 = 2;
	/// Calibration selection (Width: 2, Offset: 12)
	pub fn get_calsel() -> u8 { ::read(REGISTER_ADDRESS, CALSEL_BIT_OFFSET, CALSEL_BIT_WIDTH) as u8 }
	/// Calibration selection (Width: 2, Offset: 12)
	pub fn set_calsel(value: u8) { ::write(REGISTER_ADDRESS, CALSEL_BIT_OFFSET, CALSEL_BIT_WIDTH, value as u32); }

	const PGA_GAIN_BIT_OFFSET: u8 = 14;
	const PGA_GAIN_BIT_WIDTH: u8 = 4;
	/// Gain in PGA mode (Width: 4, Offset: 14)
	pub fn get_pga_gain() -> u8 { ::read(REGISTER_ADDRESS, PGA_GAIN_BIT_OFFSET, PGA_GAIN_BIT_WIDTH) as u8 }
	/// Gain in PGA mode (Width: 4, Offset: 14)
	pub fn set_pga_gain(value: u8) { ::write(REGISTER_ADDRESS, PGA_GAIN_BIT_OFFSET, PGA_GAIN_BIT_WIDTH, value as u32); }

	const USER_TRIM_BIT_OFFSET: u8 = 18;
	const USER_TRIM_BIT_WIDTH: u8 = 1;
	/// User trimming enable (Width: 1, Offset: 18)
	pub fn get_user_trim() -> u8 { ::read(REGISTER_ADDRESS, USER_TRIM_BIT_OFFSET, USER_TRIM_BIT_WIDTH) as u8 }
	/// User trimming enable (Width: 1, Offset: 18)
	pub fn set_user_trim(value: u8) { ::write(REGISTER_ADDRESS, USER_TRIM_BIT_OFFSET, USER_TRIM_BIT_WIDTH, value as u32); }

	const TRIMOFFSETP_BIT_OFFSET: u8 = 19;
	const TRIMOFFSETP_BIT_WIDTH: u8 = 5;
	/// Offset trimming value (PMOS) (Width: 5, Offset: 19)
	pub fn get_trimoffsetp() -> u8 { ::read(REGISTER_ADDRESS, TRIMOFFSETP_BIT_OFFSET, TRIMOFFSETP_BIT_WIDTH) as u8 }
	/// Offset trimming value (PMOS) (Width: 5, Offset: 19)
	pub fn set_trimoffsetp(value: u8) { ::write(REGISTER_ADDRESS, TRIMOFFSETP_BIT_OFFSET, TRIMOFFSETP_BIT_WIDTH, value as u32); }

	const TRIMOFFSETN_BIT_OFFSET: u8 = 24;
	const TRIMOFFSETN_BIT_WIDTH: u8 = 5;
	/// Offset trimming value (NMOS) (Width: 5, Offset: 24)
	pub fn get_trimoffsetn() -> u8 { ::read(REGISTER_ADDRESS, TRIMOFFSETN_BIT_OFFSET, TRIMOFFSETN_BIT_WIDTH) as u8 }
	/// Offset trimming value (NMOS) (Width: 5, Offset: 24)
	pub fn set_trimoffsetn(value: u8) { ::write(REGISTER_ADDRESS, TRIMOFFSETN_BIT_OFFSET, TRIMOFFSETN_BIT_WIDTH, value as u32); }

	const TSTREF_BIT_OFFSET: u8 = 29;
	const TSTREF_BIT_WIDTH: u8 = 1;
	/// TSTREF (Width: 1, Offset: 29)
	pub fn get_tstref() -> u8 { ::read(REGISTER_ADDRESS, TSTREF_BIT_OFFSET, TSTREF_BIT_WIDTH) as u8 }
	/// TSTREF (Width: 1, Offset: 29)
	pub fn set_tstref(value: u8) { ::write(REGISTER_ADDRESS, TSTREF_BIT_OFFSET, TSTREF_BIT_WIDTH, value as u32); }

	const OUTCAL_BIT_OFFSET: u8 = 30;
	const OUTCAL_BIT_WIDTH: u8 = 1;
	/// OPAMP 1 ouput status flag (Width: 1, Offset: 30)
	pub fn get_outcal() -> u8 { ::read(REGISTER_ADDRESS, OUTCAL_BIT_OFFSET, OUTCAL_BIT_WIDTH) as u8 }

	const LOCK_BIT_OFFSET: u8 = 31;
	const LOCK_BIT_WIDTH: u8 = 1;
	/// OPAMP 1 lock (Width: 1, Offset: 31)
	pub fn get_lock() -> u8 { ::read(REGISTER_ADDRESS, LOCK_BIT_OFFSET, LOCK_BIT_WIDTH) as u8 }
	/// OPAMP 1 lock (Width: 1, Offset: 31)
	pub fn set_lock(value: u8) { ::write(REGISTER_ADDRESS, LOCK_BIT_OFFSET, LOCK_BIT_WIDTH, value as u32); }
}
/// OPAMP2 control register
/// Size: 0x20 bits
pub mod opamp2_cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OPAMP2EN_BIT_OFFSET: u8 = 0;
	const OPAMP2EN_BIT_WIDTH: u8 = 1;
	/// OPAMP2 enable (Width: 1, Offset: 0)
	pub fn get_opamp2en() -> u8 { ::read(REGISTER_ADDRESS, OPAMP2EN_BIT_OFFSET, OPAMP2EN_BIT_WIDTH) as u8 }
	/// OPAMP2 enable (Width: 1, Offset: 0)
	pub fn set_opamp2en(value: u8) { ::write(REGISTER_ADDRESS, OPAMP2EN_BIT_OFFSET, OPAMP2EN_BIT_WIDTH, value as u32); }

	const FORCE_VP_BIT_OFFSET: u8 = 1;
	const FORCE_VP_BIT_WIDTH: u8 = 1;
	/// FORCE_VP (Width: 1, Offset: 1)
	pub fn get_force_vp() -> u8 { ::read(REGISTER_ADDRESS, FORCE_VP_BIT_OFFSET, FORCE_VP_BIT_WIDTH) as u8 }
	/// FORCE_VP (Width: 1, Offset: 1)
	pub fn set_force_vp(value: u8) { ::write(REGISTER_ADDRESS, FORCE_VP_BIT_OFFSET, FORCE_VP_BIT_WIDTH, value as u32); }

	const VP_SEL_BIT_OFFSET: u8 = 2;
	const VP_SEL_BIT_WIDTH: u8 = 2;
	/// OPAMP2 Non inverting input selection (Width: 2, Offset: 2)
	pub fn get_vp_sel() -> u8 { ::read(REGISTER_ADDRESS, VP_SEL_BIT_OFFSET, VP_SEL_BIT_WIDTH) as u8 }
	/// OPAMP2 Non inverting input selection (Width: 2, Offset: 2)
	pub fn set_vp_sel(value: u8) { ::write(REGISTER_ADDRESS, VP_SEL_BIT_OFFSET, VP_SEL_BIT_WIDTH, value as u32); }

	const VM_SEL_BIT_OFFSET: u8 = 5;
	const VM_SEL_BIT_WIDTH: u8 = 2;
	/// OPAMP2 inverting input selection (Width: 2, Offset: 5)
	pub fn get_vm_sel() -> u8 { ::read(REGISTER_ADDRESS, VM_SEL_BIT_OFFSET, VM_SEL_BIT_WIDTH) as u8 }
	/// OPAMP2 inverting input selection (Width: 2, Offset: 5)
	pub fn set_vm_sel(value: u8) { ::write(REGISTER_ADDRESS, VM_SEL_BIT_OFFSET, VM_SEL_BIT_WIDTH, value as u32); }

	const TCM_EN_BIT_OFFSET: u8 = 7;
	const TCM_EN_BIT_WIDTH: u8 = 1;
	/// Timer controlled Mux mode enable (Width: 1, Offset: 7)
	pub fn get_tcm_en() -> u8 { ::read(REGISTER_ADDRESS, TCM_EN_BIT_OFFSET, TCM_EN_BIT_WIDTH) as u8 }
	/// Timer controlled Mux mode enable (Width: 1, Offset: 7)
	pub fn set_tcm_en(value: u8) { ::write(REGISTER_ADDRESS, TCM_EN_BIT_OFFSET, TCM_EN_BIT_WIDTH, value as u32); }

	const VMS_SEL_BIT_OFFSET: u8 = 8;
	const VMS_SEL_BIT_WIDTH: u8 = 1;
	/// OPAMP2 inverting input secondary selection (Width: 1, Offset: 8)
	pub fn get_vms_sel() -> u8 { ::read(REGISTER_ADDRESS, VMS_SEL_BIT_OFFSET, VMS_SEL_BIT_WIDTH) as u8 }
	/// OPAMP2 inverting input secondary selection (Width: 1, Offset: 8)
	pub fn set_vms_sel(value: u8) { ::write(REGISTER_ADDRESS, VMS_SEL_BIT_OFFSET, VMS_SEL_BIT_WIDTH, value as u32); }

	const VPS_SEL_BIT_OFFSET: u8 = 9;
	const VPS_SEL_BIT_WIDTH: u8 = 2;
	/// OPAMP2 Non inverting input secondary selection (Width: 2, Offset: 9)
	pub fn get_vps_sel() -> u8 { ::read(REGISTER_ADDRESS, VPS_SEL_BIT_OFFSET, VPS_SEL_BIT_WIDTH) as u8 }
	/// OPAMP2 Non inverting input secondary selection (Width: 2, Offset: 9)
	pub fn set_vps_sel(value: u8) { ::write(REGISTER_ADDRESS, VPS_SEL_BIT_OFFSET, VPS_SEL_BIT_WIDTH, value as u32); }

	const CALON_BIT_OFFSET: u8 = 11;
	const CALON_BIT_WIDTH: u8 = 1;
	/// Calibration mode enable (Width: 1, Offset: 11)
	pub fn get_calon() -> u8 { ::read(REGISTER_ADDRESS, CALON_BIT_OFFSET, CALON_BIT_WIDTH) as u8 }
	/// Calibration mode enable (Width: 1, Offset: 11)
	pub fn set_calon(value: u8) { ::write(REGISTER_ADDRESS, CALON_BIT_OFFSET, CALON_BIT_WIDTH, value as u32); }

	const CAL_SEL_BIT_OFFSET: u8 = 12;
	const CAL_SEL_BIT_WIDTH: u8 = 2;
	/// Calibration selection (Width: 2, Offset: 12)
	pub fn get_cal_sel() -> u8 { ::read(REGISTER_ADDRESS, CAL_SEL_BIT_OFFSET, CAL_SEL_BIT_WIDTH) as u8 }
	/// Calibration selection (Width: 2, Offset: 12)
	pub fn set_cal_sel(value: u8) { ::write(REGISTER_ADDRESS, CAL_SEL_BIT_OFFSET, CAL_SEL_BIT_WIDTH, value as u32); }

	const PGA_GAIN_BIT_OFFSET: u8 = 14;
	const PGA_GAIN_BIT_WIDTH: u8 = 4;
	/// Gain in PGA mode (Width: 4, Offset: 14)
	pub fn get_pga_gain() -> u8 { ::read(REGISTER_ADDRESS, PGA_GAIN_BIT_OFFSET, PGA_GAIN_BIT_WIDTH) as u8 }
	/// Gain in PGA mode (Width: 4, Offset: 14)
	pub fn set_pga_gain(value: u8) { ::write(REGISTER_ADDRESS, PGA_GAIN_BIT_OFFSET, PGA_GAIN_BIT_WIDTH, value as u32); }

	const USER_TRIM_BIT_OFFSET: u8 = 18;
	const USER_TRIM_BIT_WIDTH: u8 = 1;
	/// User trimming enable (Width: 1, Offset: 18)
	pub fn get_user_trim() -> u8 { ::read(REGISTER_ADDRESS, USER_TRIM_BIT_OFFSET, USER_TRIM_BIT_WIDTH) as u8 }
	/// User trimming enable (Width: 1, Offset: 18)
	pub fn set_user_trim(value: u8) { ::write(REGISTER_ADDRESS, USER_TRIM_BIT_OFFSET, USER_TRIM_BIT_WIDTH, value as u32); }

	const TRIMOFFSETP_BIT_OFFSET: u8 = 19;
	const TRIMOFFSETP_BIT_WIDTH: u8 = 5;
	/// Offset trimming value (PMOS) (Width: 5, Offset: 19)
	pub fn get_trimoffsetp() -> u8 { ::read(REGISTER_ADDRESS, TRIMOFFSETP_BIT_OFFSET, TRIMOFFSETP_BIT_WIDTH) as u8 }
	/// Offset trimming value (PMOS) (Width: 5, Offset: 19)
	pub fn set_trimoffsetp(value: u8) { ::write(REGISTER_ADDRESS, TRIMOFFSETP_BIT_OFFSET, TRIMOFFSETP_BIT_WIDTH, value as u32); }

	const TRIMOFFSETN_BIT_OFFSET: u8 = 24;
	const TRIMOFFSETN_BIT_WIDTH: u8 = 5;
	/// Offset trimming value (NMOS) (Width: 5, Offset: 24)
	pub fn get_trimoffsetn() -> u8 { ::read(REGISTER_ADDRESS, TRIMOFFSETN_BIT_OFFSET, TRIMOFFSETN_BIT_WIDTH) as u8 }
	/// Offset trimming value (NMOS) (Width: 5, Offset: 24)
	pub fn set_trimoffsetn(value: u8) { ::write(REGISTER_ADDRESS, TRIMOFFSETN_BIT_OFFSET, TRIMOFFSETN_BIT_WIDTH, value as u32); }

	const TSTREF_BIT_OFFSET: u8 = 29;
	const TSTREF_BIT_WIDTH: u8 = 1;
	/// TSTREF (Width: 1, Offset: 29)
	pub fn get_tstref() -> u8 { ::read(REGISTER_ADDRESS, TSTREF_BIT_OFFSET, TSTREF_BIT_WIDTH) as u8 }
	/// TSTREF (Width: 1, Offset: 29)
	pub fn set_tstref(value: u8) { ::write(REGISTER_ADDRESS, TSTREF_BIT_OFFSET, TSTREF_BIT_WIDTH, value as u32); }

	const OUTCAL_BIT_OFFSET: u8 = 30;
	const OUTCAL_BIT_WIDTH: u8 = 1;
	/// OPAMP 2 ouput status flag (Width: 1, Offset: 30)
	pub fn get_outcal() -> u8 { ::read(REGISTER_ADDRESS, OUTCAL_BIT_OFFSET, OUTCAL_BIT_WIDTH) as u8 }

	const LOCK_BIT_OFFSET: u8 = 31;
	const LOCK_BIT_WIDTH: u8 = 1;
	/// OPAMP 2 lock (Width: 1, Offset: 31)
	pub fn get_lock() -> u8 { ::read(REGISTER_ADDRESS, LOCK_BIT_OFFSET, LOCK_BIT_WIDTH) as u8 }
	/// OPAMP 2 lock (Width: 1, Offset: 31)
	pub fn set_lock(value: u8) { ::write(REGISTER_ADDRESS, LOCK_BIT_OFFSET, LOCK_BIT_WIDTH, value as u32); }
}
/// OPAMP3 control register
/// Size: 0x20 bits
pub mod opamp3_cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OPAMP3EN_BIT_OFFSET: u8 = 0;
	const OPAMP3EN_BIT_WIDTH: u8 = 1;
	/// OPAMP3 enable (Width: 1, Offset: 0)
	pub fn get_opamp3en() -> u8 { ::read(REGISTER_ADDRESS, OPAMP3EN_BIT_OFFSET, OPAMP3EN_BIT_WIDTH) as u8 }
	/// OPAMP3 enable (Width: 1, Offset: 0)
	pub fn set_opamp3en(value: u8) { ::write(REGISTER_ADDRESS, OPAMP3EN_BIT_OFFSET, OPAMP3EN_BIT_WIDTH, value as u32); }

	const FORCE_VP_BIT_OFFSET: u8 = 1;
	const FORCE_VP_BIT_WIDTH: u8 = 1;
	/// FORCE_VP (Width: 1, Offset: 1)
	pub fn get_force_vp() -> u8 { ::read(REGISTER_ADDRESS, FORCE_VP_BIT_OFFSET, FORCE_VP_BIT_WIDTH) as u8 }
	/// FORCE_VP (Width: 1, Offset: 1)
	pub fn set_force_vp(value: u8) { ::write(REGISTER_ADDRESS, FORCE_VP_BIT_OFFSET, FORCE_VP_BIT_WIDTH, value as u32); }

	const VP_SEL_BIT_OFFSET: u8 = 2;
	const VP_SEL_BIT_WIDTH: u8 = 2;
	/// OPAMP3 Non inverting input selection (Width: 2, Offset: 2)
	pub fn get_vp_sel() -> u8 { ::read(REGISTER_ADDRESS, VP_SEL_BIT_OFFSET, VP_SEL_BIT_WIDTH) as u8 }
	/// OPAMP3 Non inverting input selection (Width: 2, Offset: 2)
	pub fn set_vp_sel(value: u8) { ::write(REGISTER_ADDRESS, VP_SEL_BIT_OFFSET, VP_SEL_BIT_WIDTH, value as u32); }

	const VM_SEL_BIT_OFFSET: u8 = 5;
	const VM_SEL_BIT_WIDTH: u8 = 2;
	/// OPAMP3 inverting input selection (Width: 2, Offset: 5)
	pub fn get_vm_sel() -> u8 { ::read(REGISTER_ADDRESS, VM_SEL_BIT_OFFSET, VM_SEL_BIT_WIDTH) as u8 }
	/// OPAMP3 inverting input selection (Width: 2, Offset: 5)
	pub fn set_vm_sel(value: u8) { ::write(REGISTER_ADDRESS, VM_SEL_BIT_OFFSET, VM_SEL_BIT_WIDTH, value as u32); }

	const TCM_EN_BIT_OFFSET: u8 = 7;
	const TCM_EN_BIT_WIDTH: u8 = 1;
	/// Timer controlled Mux mode enable (Width: 1, Offset: 7)
	pub fn get_tcm_en() -> u8 { ::read(REGISTER_ADDRESS, TCM_EN_BIT_OFFSET, TCM_EN_BIT_WIDTH) as u8 }
	/// Timer controlled Mux mode enable (Width: 1, Offset: 7)
	pub fn set_tcm_en(value: u8) { ::write(REGISTER_ADDRESS, TCM_EN_BIT_OFFSET, TCM_EN_BIT_WIDTH, value as u32); }

	const VMS_SEL_BIT_OFFSET: u8 = 8;
	const VMS_SEL_BIT_WIDTH: u8 = 1;
	/// OPAMP3 inverting input secondary selection (Width: 1, Offset: 8)
	pub fn get_vms_sel() -> u8 { ::read(REGISTER_ADDRESS, VMS_SEL_BIT_OFFSET, VMS_SEL_BIT_WIDTH) as u8 }
	/// OPAMP3 inverting input secondary selection (Width: 1, Offset: 8)
	pub fn set_vms_sel(value: u8) { ::write(REGISTER_ADDRESS, VMS_SEL_BIT_OFFSET, VMS_SEL_BIT_WIDTH, value as u32); }

	const VPS_SEL_BIT_OFFSET: u8 = 9;
	const VPS_SEL_BIT_WIDTH: u8 = 2;
	/// OPAMP3 Non inverting input secondary selection (Width: 2, Offset: 9)
	pub fn get_vps_sel() -> u8 { ::read(REGISTER_ADDRESS, VPS_SEL_BIT_OFFSET, VPS_SEL_BIT_WIDTH) as u8 }
	/// OPAMP3 Non inverting input secondary selection (Width: 2, Offset: 9)
	pub fn set_vps_sel(value: u8) { ::write(REGISTER_ADDRESS, VPS_SEL_BIT_OFFSET, VPS_SEL_BIT_WIDTH, value as u32); }

	const CALON_BIT_OFFSET: u8 = 11;
	const CALON_BIT_WIDTH: u8 = 1;
	/// Calibration mode enable (Width: 1, Offset: 11)
	pub fn get_calon() -> u8 { ::read(REGISTER_ADDRESS, CALON_BIT_OFFSET, CALON_BIT_WIDTH) as u8 }
	/// Calibration mode enable (Width: 1, Offset: 11)
	pub fn set_calon(value: u8) { ::write(REGISTER_ADDRESS, CALON_BIT_OFFSET, CALON_BIT_WIDTH, value as u32); }

	const CALSEL_BIT_OFFSET: u8 = 12;
	const CALSEL_BIT_WIDTH: u8 = 2;
	/// Calibration selection (Width: 2, Offset: 12)
	pub fn get_calsel() -> u8 { ::read(REGISTER_ADDRESS, CALSEL_BIT_OFFSET, CALSEL_BIT_WIDTH) as u8 }
	/// Calibration selection (Width: 2, Offset: 12)
	pub fn set_calsel(value: u8) { ::write(REGISTER_ADDRESS, CALSEL_BIT_OFFSET, CALSEL_BIT_WIDTH, value as u32); }

	const PGA_GAIN_BIT_OFFSET: u8 = 14;
	const PGA_GAIN_BIT_WIDTH: u8 = 4;
	/// Gain in PGA mode (Width: 4, Offset: 14)
	pub fn get_pga_gain() -> u8 { ::read(REGISTER_ADDRESS, PGA_GAIN_BIT_OFFSET, PGA_GAIN_BIT_WIDTH) as u8 }
	/// Gain in PGA mode (Width: 4, Offset: 14)
	pub fn set_pga_gain(value: u8) { ::write(REGISTER_ADDRESS, PGA_GAIN_BIT_OFFSET, PGA_GAIN_BIT_WIDTH, value as u32); }

	const USER_TRIM_BIT_OFFSET: u8 = 18;
	const USER_TRIM_BIT_WIDTH: u8 = 1;
	/// User trimming enable (Width: 1, Offset: 18)
	pub fn get_user_trim() -> u8 { ::read(REGISTER_ADDRESS, USER_TRIM_BIT_OFFSET, USER_TRIM_BIT_WIDTH) as u8 }
	/// User trimming enable (Width: 1, Offset: 18)
	pub fn set_user_trim(value: u8) { ::write(REGISTER_ADDRESS, USER_TRIM_BIT_OFFSET, USER_TRIM_BIT_WIDTH, value as u32); }

	const TRIMOFFSETP_BIT_OFFSET: u8 = 19;
	const TRIMOFFSETP_BIT_WIDTH: u8 = 5;
	/// Offset trimming value (PMOS) (Width: 5, Offset: 19)
	pub fn get_trimoffsetp() -> u8 { ::read(REGISTER_ADDRESS, TRIMOFFSETP_BIT_OFFSET, TRIMOFFSETP_BIT_WIDTH) as u8 }
	/// Offset trimming value (PMOS) (Width: 5, Offset: 19)
	pub fn set_trimoffsetp(value: u8) { ::write(REGISTER_ADDRESS, TRIMOFFSETP_BIT_OFFSET, TRIMOFFSETP_BIT_WIDTH, value as u32); }

	const TRIMOFFSETN_BIT_OFFSET: u8 = 24;
	const TRIMOFFSETN_BIT_WIDTH: u8 = 5;
	/// Offset trimming value (NMOS) (Width: 5, Offset: 24)
	pub fn get_trimoffsetn() -> u8 { ::read(REGISTER_ADDRESS, TRIMOFFSETN_BIT_OFFSET, TRIMOFFSETN_BIT_WIDTH) as u8 }
	/// Offset trimming value (NMOS) (Width: 5, Offset: 24)
	pub fn set_trimoffsetn(value: u8) { ::write(REGISTER_ADDRESS, TRIMOFFSETN_BIT_OFFSET, TRIMOFFSETN_BIT_WIDTH, value as u32); }

	const TSTREF_BIT_OFFSET: u8 = 29;
	const TSTREF_BIT_WIDTH: u8 = 1;
	/// TSTREF (Width: 1, Offset: 29)
	pub fn get_tstref() -> u8 { ::read(REGISTER_ADDRESS, TSTREF_BIT_OFFSET, TSTREF_BIT_WIDTH) as u8 }
	/// TSTREF (Width: 1, Offset: 29)
	pub fn set_tstref(value: u8) { ::write(REGISTER_ADDRESS, TSTREF_BIT_OFFSET, TSTREF_BIT_WIDTH, value as u32); }

	const OUTCAL_BIT_OFFSET: u8 = 30;
	const OUTCAL_BIT_WIDTH: u8 = 1;
	/// OPAMP 3 ouput status flag (Width: 1, Offset: 30)
	pub fn get_outcal() -> u8 { ::read(REGISTER_ADDRESS, OUTCAL_BIT_OFFSET, OUTCAL_BIT_WIDTH) as u8 }

	const LOCK_BIT_OFFSET: u8 = 31;
	const LOCK_BIT_WIDTH: u8 = 1;
	/// OPAMP 3 lock (Width: 1, Offset: 31)
	pub fn get_lock() -> u8 { ::read(REGISTER_ADDRESS, LOCK_BIT_OFFSET, LOCK_BIT_WIDTH) as u8 }
	/// OPAMP 3 lock (Width: 1, Offset: 31)
	pub fn set_lock(value: u8) { ::write(REGISTER_ADDRESS, LOCK_BIT_OFFSET, LOCK_BIT_WIDTH, value as u32); }
}
/// OPAMP4 control register
/// Size: 0x20 bits
pub mod opamp4_cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const OPAMP4EN_BIT_OFFSET: u8 = 0;
	const OPAMP4EN_BIT_WIDTH: u8 = 1;
	/// OPAMP4 enable (Width: 1, Offset: 0)
	pub fn get_opamp4en() -> u8 { ::read(REGISTER_ADDRESS, OPAMP4EN_BIT_OFFSET, OPAMP4EN_BIT_WIDTH) as u8 }
	/// OPAMP4 enable (Width: 1, Offset: 0)
	pub fn set_opamp4en(value: u8) { ::write(REGISTER_ADDRESS, OPAMP4EN_BIT_OFFSET, OPAMP4EN_BIT_WIDTH, value as u32); }

	const FORCE_VP_BIT_OFFSET: u8 = 1;
	const FORCE_VP_BIT_WIDTH: u8 = 1;
	/// FORCE_VP (Width: 1, Offset: 1)
	pub fn get_force_vp() -> u8 { ::read(REGISTER_ADDRESS, FORCE_VP_BIT_OFFSET, FORCE_VP_BIT_WIDTH) as u8 }
	/// FORCE_VP (Width: 1, Offset: 1)
	pub fn set_force_vp(value: u8) { ::write(REGISTER_ADDRESS, FORCE_VP_BIT_OFFSET, FORCE_VP_BIT_WIDTH, value as u32); }

	const VP_SEL_BIT_OFFSET: u8 = 2;
	const VP_SEL_BIT_WIDTH: u8 = 2;
	/// OPAMP4 Non inverting input selection (Width: 2, Offset: 2)
	pub fn get_vp_sel() -> u8 { ::read(REGISTER_ADDRESS, VP_SEL_BIT_OFFSET, VP_SEL_BIT_WIDTH) as u8 }
	/// OPAMP4 Non inverting input selection (Width: 2, Offset: 2)
	pub fn set_vp_sel(value: u8) { ::write(REGISTER_ADDRESS, VP_SEL_BIT_OFFSET, VP_SEL_BIT_WIDTH, value as u32); }

	const VM_SEL_BIT_OFFSET: u8 = 5;
	const VM_SEL_BIT_WIDTH: u8 = 2;
	/// OPAMP4 inverting input selection (Width: 2, Offset: 5)
	pub fn get_vm_sel() -> u8 { ::read(REGISTER_ADDRESS, VM_SEL_BIT_OFFSET, VM_SEL_BIT_WIDTH) as u8 }
	/// OPAMP4 inverting input selection (Width: 2, Offset: 5)
	pub fn set_vm_sel(value: u8) { ::write(REGISTER_ADDRESS, VM_SEL_BIT_OFFSET, VM_SEL_BIT_WIDTH, value as u32); }

	const TCM_EN_BIT_OFFSET: u8 = 7;
	const TCM_EN_BIT_WIDTH: u8 = 1;
	/// Timer controlled Mux mode enable (Width: 1, Offset: 7)
	pub fn get_tcm_en() -> u8 { ::read(REGISTER_ADDRESS, TCM_EN_BIT_OFFSET, TCM_EN_BIT_WIDTH) as u8 }
	/// Timer controlled Mux mode enable (Width: 1, Offset: 7)
	pub fn set_tcm_en(value: u8) { ::write(REGISTER_ADDRESS, TCM_EN_BIT_OFFSET, TCM_EN_BIT_WIDTH, value as u32); }

	const VMS_SEL_BIT_OFFSET: u8 = 8;
	const VMS_SEL_BIT_WIDTH: u8 = 1;
	/// OPAMP4 inverting input secondary selection (Width: 1, Offset: 8)
	pub fn get_vms_sel() -> u8 { ::read(REGISTER_ADDRESS, VMS_SEL_BIT_OFFSET, VMS_SEL_BIT_WIDTH) as u8 }
	/// OPAMP4 inverting input secondary selection (Width: 1, Offset: 8)
	pub fn set_vms_sel(value: u8) { ::write(REGISTER_ADDRESS, VMS_SEL_BIT_OFFSET, VMS_SEL_BIT_WIDTH, value as u32); }

	const VPS_SEL_BIT_OFFSET: u8 = 9;
	const VPS_SEL_BIT_WIDTH: u8 = 2;
	/// OPAMP4 Non inverting input secondary selection (Width: 2, Offset: 9)
	pub fn get_vps_sel() -> u8 { ::read(REGISTER_ADDRESS, VPS_SEL_BIT_OFFSET, VPS_SEL_BIT_WIDTH) as u8 }
	/// OPAMP4 Non inverting input secondary selection (Width: 2, Offset: 9)
	pub fn set_vps_sel(value: u8) { ::write(REGISTER_ADDRESS, VPS_SEL_BIT_OFFSET, VPS_SEL_BIT_WIDTH, value as u32); }

	const CALON_BIT_OFFSET: u8 = 11;
	const CALON_BIT_WIDTH: u8 = 1;
	/// Calibration mode enable (Width: 1, Offset: 11)
	pub fn get_calon() -> u8 { ::read(REGISTER_ADDRESS, CALON_BIT_OFFSET, CALON_BIT_WIDTH) as u8 }
	/// Calibration mode enable (Width: 1, Offset: 11)
	pub fn set_calon(value: u8) { ::write(REGISTER_ADDRESS, CALON_BIT_OFFSET, CALON_BIT_WIDTH, value as u32); }

	const CALSEL_BIT_OFFSET: u8 = 12;
	const CALSEL_BIT_WIDTH: u8 = 2;
	/// Calibration selection (Width: 2, Offset: 12)
	pub fn get_calsel() -> u8 { ::read(REGISTER_ADDRESS, CALSEL_BIT_OFFSET, CALSEL_BIT_WIDTH) as u8 }
	/// Calibration selection (Width: 2, Offset: 12)
	pub fn set_calsel(value: u8) { ::write(REGISTER_ADDRESS, CALSEL_BIT_OFFSET, CALSEL_BIT_WIDTH, value as u32); }

	const PGA_GAIN_BIT_OFFSET: u8 = 14;
	const PGA_GAIN_BIT_WIDTH: u8 = 4;
	/// Gain in PGA mode (Width: 4, Offset: 14)
	pub fn get_pga_gain() -> u8 { ::read(REGISTER_ADDRESS, PGA_GAIN_BIT_OFFSET, PGA_GAIN_BIT_WIDTH) as u8 }
	/// Gain in PGA mode (Width: 4, Offset: 14)
	pub fn set_pga_gain(value: u8) { ::write(REGISTER_ADDRESS, PGA_GAIN_BIT_OFFSET, PGA_GAIN_BIT_WIDTH, value as u32); }

	const USER_TRIM_BIT_OFFSET: u8 = 18;
	const USER_TRIM_BIT_WIDTH: u8 = 1;
	/// User trimming enable (Width: 1, Offset: 18)
	pub fn get_user_trim() -> u8 { ::read(REGISTER_ADDRESS, USER_TRIM_BIT_OFFSET, USER_TRIM_BIT_WIDTH) as u8 }
	/// User trimming enable (Width: 1, Offset: 18)
	pub fn set_user_trim(value: u8) { ::write(REGISTER_ADDRESS, USER_TRIM_BIT_OFFSET, USER_TRIM_BIT_WIDTH, value as u32); }

	const TRIMOFFSETP_BIT_OFFSET: u8 = 19;
	const TRIMOFFSETP_BIT_WIDTH: u8 = 5;
	/// Offset trimming value (PMOS) (Width: 5, Offset: 19)
	pub fn get_trimoffsetp() -> u8 { ::read(REGISTER_ADDRESS, TRIMOFFSETP_BIT_OFFSET, TRIMOFFSETP_BIT_WIDTH) as u8 }
	/// Offset trimming value (PMOS) (Width: 5, Offset: 19)
	pub fn set_trimoffsetp(value: u8) { ::write(REGISTER_ADDRESS, TRIMOFFSETP_BIT_OFFSET, TRIMOFFSETP_BIT_WIDTH, value as u32); }

	const TRIMOFFSETN_BIT_OFFSET: u8 = 24;
	const TRIMOFFSETN_BIT_WIDTH: u8 = 5;
	/// Offset trimming value (NMOS) (Width: 5, Offset: 24)
	pub fn get_trimoffsetn() -> u8 { ::read(REGISTER_ADDRESS, TRIMOFFSETN_BIT_OFFSET, TRIMOFFSETN_BIT_WIDTH) as u8 }
	/// Offset trimming value (NMOS) (Width: 5, Offset: 24)
	pub fn set_trimoffsetn(value: u8) { ::write(REGISTER_ADDRESS, TRIMOFFSETN_BIT_OFFSET, TRIMOFFSETN_BIT_WIDTH, value as u32); }

	const TSTREF_BIT_OFFSET: u8 = 29;
	const TSTREF_BIT_WIDTH: u8 = 1;
	/// TSTREF (Width: 1, Offset: 29)
	pub fn get_tstref() -> u8 { ::read(REGISTER_ADDRESS, TSTREF_BIT_OFFSET, TSTREF_BIT_WIDTH) as u8 }
	/// TSTREF (Width: 1, Offset: 29)
	pub fn set_tstref(value: u8) { ::write(REGISTER_ADDRESS, TSTREF_BIT_OFFSET, TSTREF_BIT_WIDTH, value as u32); }

	const OUTCAL_BIT_OFFSET: u8 = 30;
	const OUTCAL_BIT_WIDTH: u8 = 1;
	/// OPAMP 4 ouput status flag (Width: 1, Offset: 30)
	pub fn get_outcal() -> u8 { ::read(REGISTER_ADDRESS, OUTCAL_BIT_OFFSET, OUTCAL_BIT_WIDTH) as u8 }

	const LOCK_BIT_OFFSET: u8 = 31;
	const LOCK_BIT_WIDTH: u8 = 1;
	/// OPAMP 4 lock (Width: 1, Offset: 31)
	pub fn get_lock() -> u8 { ::read(REGISTER_ADDRESS, LOCK_BIT_OFFSET, LOCK_BIT_WIDTH) as u8 }
	/// OPAMP 4 lock (Width: 1, Offset: 31)
	pub fn set_lock(value: u8) { ::write(REGISTER_ADDRESS, LOCK_BIT_OFFSET, LOCK_BIT_WIDTH, value as u32); }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>OPAMP</name>
  <description>Operational amplifier</description>
  <groupName>OPAMP</groupName>
  <baseAddress>0x40010038</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x3C8</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>OPAMP1_CR</name>
      <displayName>OPAMP1_CR</displayName>
      <description>OPAMP1 control register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OPAMP1_EN</name>
          <description>OPAMP1 enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>FORCE_VP</name>
          <description>FORCE_VP</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VP_SEL</name>
          <description>OPAMP1 Non inverting input
              selection</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VM_SEL</name>
          <description>OPAMP1 inverting input
              selection</description>
          <bitOffset>5</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TCM_EN</name>
          <description>Timer controlled Mux mode
              enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VMS_SEL</name>
          <description>OPAMP1 inverting input secondary
              selection</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VPS_SEL</name>
          <description>OPAMP1 Non inverting input secondary
              selection</description>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CALON</name>
          <description>Calibration mode enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CALSEL</name>
          <description>Calibration selection</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PGA_GAIN</name>
          <description>Gain in PGA mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>USER_TRIM</name>
          <description>User trimming enable</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TRIMOFFSETP</name>
          <description>Offset trimming value
              (PMOS)</description>
          <bitOffset>19</bitOffset>
          <bitWidth>5</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TRIMOFFSETN</name>
          <description>Offset trimming value
              (NMOS)</description>
          <bitOffset>24</bitOffset>
          <bitWidth>5</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TSTREF</name>
          <description>TSTREF</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>OUTCAL</name>
          <description>OPAMP 1 ouput status flag</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>LOCK</name>
          <description>OPAMP 1 lock</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>OPAMP2_CR</name>
      <displayName>OPAMP2_CR</displayName>
      <description>OPAMP2 control register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OPAMP2EN</name>
          <description>OPAMP2 enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>FORCE_VP</name>
          <description>FORCE_VP</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VP_SEL</name>
          <description>OPAMP2 Non inverting input
              selection</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VM_SEL</name>
          <description>OPAMP2 inverting input
              selection</description>
          <bitOffset>5</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TCM_EN</name>
          <description>Timer controlled Mux mode
              enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VMS_SEL</name>
          <description>OPAMP2 inverting input secondary
              selection</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VPS_SEL</name>
          <description>OPAMP2 Non inverting input secondary
              selection</description>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CALON</name>
          <description>Calibration mode enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CAL_SEL</name>
          <description>Calibration selection</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PGA_GAIN</name>
          <description>Gain in PGA mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>USER_TRIM</name>
          <description>User trimming enable</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TRIMOFFSETP</name>
          <description>Offset trimming value
              (PMOS)</description>
          <bitOffset>19</bitOffset>
          <bitWidth>5</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TRIMOFFSETN</name>
          <description>Offset trimming value
              (NMOS)</description>
          <bitOffset>24</bitOffset>
          <bitWidth>5</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TSTREF</name>
          <description>TSTREF</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>OUTCAL</name>
          <description>OPAMP 2 ouput status flag</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>LOCK</name>
          <description>OPAMP 2 lock</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>OPAMP3_CR</name>
      <displayName>OPAMP3_CR</displayName>
      <description>OPAMP3 control register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OPAMP3EN</name>
          <description>OPAMP3 enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>FORCE_VP</name>
          <description>FORCE_VP</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VP_SEL</name>
          <description>OPAMP3 Non inverting input
              selection</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VM_SEL</name>
          <description>OPAMP3 inverting input
              selection</description>
          <bitOffset>5</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TCM_EN</name>
          <description>Timer controlled Mux mode
              enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VMS_SEL</name>
          <description>OPAMP3 inverting input secondary
              selection</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VPS_SEL</name>
          <description>OPAMP3 Non inverting input secondary
              selection</description>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CALON</name>
          <description>Calibration mode enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CALSEL</name>
          <description>Calibration selection</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PGA_GAIN</name>
          <description>Gain in PGA mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>USER_TRIM</name>
          <description>User trimming enable</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TRIMOFFSETP</name>
          <description>Offset trimming value
              (PMOS)</description>
          <bitOffset>19</bitOffset>
          <bitWidth>5</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TRIMOFFSETN</name>
          <description>Offset trimming value
              (NMOS)</description>
          <bitOffset>24</bitOffset>
          <bitWidth>5</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TSTREF</name>
          <description>TSTREF</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>OUTCAL</name>
          <description>OPAMP 3 ouput status flag</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>LOCK</name>
          <description>OPAMP 3 lock</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>OPAMP4_CR</name>
      <displayName>OPAMP4_CR</displayName>
      <description>OPAMP4 control register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>OPAMP4EN</name>
          <description>OPAMP4 enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>FORCE_VP</name>
          <description>FORCE_VP</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VP_SEL</name>
          <description>OPAMP4 Non inverting input
              selection</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VM_SEL</name>
          <description>OPAMP4 inverting input
              selection</description>
          <bitOffset>5</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TCM_EN</name>
          <description>Timer controlled Mux mode
              enable</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VMS_SEL</name>
          <description>OPAMP4 inverting input secondary
              selection</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>VPS_SEL</name>
          <description>OPAMP4 Non inverting input secondary
              selection</description>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CALON</name>
          <description>Calibration mode enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CALSEL</name>
          <description>Calibration selection</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PGA_GAIN</name>
          <description>Gain in PGA mode</description>
          <bitOffset>14</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>USER_TRIM</name>
          <description>User trimming enable</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TRIMOFFSETP</name>
          <description>Offset trimming value
              (PMOS)</description>
          <bitOffset>19</bitOffset>
          <bitWidth>5</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TRIMOFFSETN</name>
          <description>Offset trimming value
              (NMOS)</description>
          <bitOffset>24</bitOffset>
          <bitWidth>5</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>TSTREF</name>
          <description>TSTREF</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>OUTCAL</name>
          <description>OPAMP 4 ouput status flag</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>LOCK</name>
          <description>OPAMP 4 lock</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
  </registers>
</peripheral>*/
