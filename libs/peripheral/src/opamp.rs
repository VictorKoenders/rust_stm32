/// OPAMP1 control register
pub mod opamp1_cr {
    /// OPAMP1 enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set OPAMP1 enable
    pub fn set_opamp1_en(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get OPAMP1 enable
    pub fn get_opamp1_en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// FORCE_VP
    /// Access: read-write, Width: 1, Offset: 1
    /// Set FORCE_VP
    pub fn set_force_vp(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get FORCE_VP
    pub fn get_force_vp() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// OPAMP1 Non inverting input selection
    /// Access: read-write, Width: 2, Offset: 2
    /// Set OPAMP1 Non inverting input selection
    pub fn set_vp_sel(value: u8) {
        debug_assert!(value <= 0b11, "set_vp_sel out of range");
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get OPAMP1 Non inverting input selection
    pub fn get_vp_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b11 << 2);
        value as u8
    }
    /// OPAMP1 inverting input selection
    /// Access: read-write, Width: 2, Offset: 5
    /// Set OPAMP1 inverting input selection
    pub fn set_vm_sel(value: u8) {
        debug_assert!(value <= 0b11, "set_vm_sel out of range");
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get OPAMP1 inverting input selection
    pub fn get_vm_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b11 << 5);
        value as u8
    }
    /// Timer controlled Mux mode enable
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Timer controlled Mux mode enable
    pub fn set_tcm_en(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Timer controlled Mux mode enable
    pub fn get_tcm_en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// OPAMP1 inverting input secondary selection
    /// Access: read-write, Width: 1, Offset: 8
    /// Set OPAMP1 inverting input secondary selection
    pub fn set_vms_sel(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get OPAMP1 inverting input secondary selection
    pub fn get_vms_sel() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// OPAMP1 Non inverting input secondary selection
    /// Access: read-write, Width: 2, Offset: 9
    /// Set OPAMP1 Non inverting input secondary selection
    pub fn set_vps_sel(value: u8) {
        debug_assert!(value <= 0b11, "set_vps_sel out of range");
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get OPAMP1 Non inverting input secondary selection
    pub fn get_vps_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b11 << 9);
        value as u8
    }
    /// Calibration mode enable
    /// Access: read-write, Width: 1, Offset: 11
    /// Set Calibration mode enable
    pub fn set_calon(value: bool) {
        let value = value as u32;
        let value = value << 11;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Calibration mode enable
    pub fn get_calon() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Calibration selection
    /// Access: read-write, Width: 2, Offset: 12
    /// Set Calibration selection
    pub fn set_calsel(value: u8) {
        debug_assert!(value <= 0b11, "set_calsel out of range");
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Calibration selection
    pub fn get_calsel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b11 << 12);
        value as u8
    }
    /// Gain in PGA mode
    /// Access: read-write, Width: 4, Offset: 14
    /// Set Gain in PGA mode
    pub fn set_pga_gain(value: u8) {
        debug_assert!(value <= 0b1111, "set_pga_gain out of range");
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Gain in PGA mode
    pub fn get_pga_gain() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1111 << 14);
        value as u8
    }
    /// User trimming enable
    /// Access: read-write, Width: 1, Offset: 18
    /// Set User trimming enable
    pub fn set_user_trim(value: bool) {
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get User trimming enable
    pub fn get_user_trim() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 18);
        value > 0
    }
    /// Offset trimming value (PMOS)
    /// Access: read-write, Width: 5, Offset: 19
    /// Set Offset trimming value (PMOS)
    pub fn set_trimoffsetp(value: u8) {
        debug_assert!(value <= 0b11111, "set_trimoffsetp out of range");
        let value = value as u32;
        let value = value << 19;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Offset trimming value (PMOS)
    pub fn get_trimoffsetp() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b11111 << 19);
        value as u8
    }
    /// Offset trimming value (NMOS)
    /// Access: read-write, Width: 5, Offset: 24
    /// Set Offset trimming value (NMOS)
    pub fn set_trimoffsetn(value: u8) {
        debug_assert!(value <= 0b11111, "set_trimoffsetn out of range");
        let value = value as u32;
        let value = value << 24;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Offset trimming value (NMOS)
    pub fn get_trimoffsetn() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b11111 << 24);
        value as u8
    }
    /// TSTREF
    /// Access: read-write, Width: 1, Offset: 29
    /// Set TSTREF
    pub fn set_tstref(value: bool) {
        let value = value as u32;
        let value = value << 29;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get TSTREF
    pub fn get_tstref() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 29);
        value > 0
    }
    /// OPAMP 1 ouput status flag
    /// Access: read-only, Width: 1, Offset: 30
    /// Get OPAMP 1 ouput status flag
    pub fn outcal() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 30);
        value > 0
    }
    /// OPAMP 1 lock
    /// Access: read-write, Width: 1, Offset: 31
    /// Set OPAMP 1 lock
    pub fn set_lock(value: bool) {
        let value = value as u32;
        let value = value << 31;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get OPAMP 1 lock
    pub fn get_lock() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
}
/// OPAMP2 control register
pub mod opamp2_cr {
    /// OPAMP2 enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set OPAMP2 enable
    pub fn set_opamp2en(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get OPAMP2 enable
    pub fn get_opamp2en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// FORCE_VP
    /// Access: read-write, Width: 1, Offset: 1
    /// Set FORCE_VP
    pub fn set_force_vp(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get FORCE_VP
    pub fn get_force_vp() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// OPAMP2 Non inverting input selection
    /// Access: read-write, Width: 2, Offset: 2
    /// Set OPAMP2 Non inverting input selection
    pub fn set_vp_sel(value: u8) {
        debug_assert!(value <= 0b11, "set_vp_sel out of range");
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get OPAMP2 Non inverting input selection
    pub fn get_vp_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b11 << 2);
        value as u8
    }
    /// OPAMP2 inverting input selection
    /// Access: read-write, Width: 2, Offset: 5
    /// Set OPAMP2 inverting input selection
    pub fn set_vm_sel(value: u8) {
        debug_assert!(value <= 0b11, "set_vm_sel out of range");
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get OPAMP2 inverting input selection
    pub fn get_vm_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b11 << 5);
        value as u8
    }
    /// Timer controlled Mux mode enable
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Timer controlled Mux mode enable
    pub fn set_tcm_en(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Timer controlled Mux mode enable
    pub fn get_tcm_en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// OPAMP2 inverting input secondary selection
    /// Access: read-write, Width: 1, Offset: 8
    /// Set OPAMP2 inverting input secondary selection
    pub fn set_vms_sel(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get OPAMP2 inverting input secondary selection
    pub fn get_vms_sel() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// OPAMP2 Non inverting input secondary selection
    /// Access: read-write, Width: 2, Offset: 9
    /// Set OPAMP2 Non inverting input secondary selection
    pub fn set_vps_sel(value: u8) {
        debug_assert!(value <= 0b11, "set_vps_sel out of range");
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get OPAMP2 Non inverting input secondary selection
    pub fn get_vps_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b11 << 9);
        value as u8
    }
    /// Calibration mode enable
    /// Access: read-write, Width: 1, Offset: 11
    /// Set Calibration mode enable
    pub fn set_calon(value: bool) {
        let value = value as u32;
        let value = value << 11;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Calibration mode enable
    pub fn get_calon() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Calibration selection
    /// Access: read-write, Width: 2, Offset: 12
    /// Set Calibration selection
    pub fn set_cal_sel(value: u8) {
        debug_assert!(value <= 0b11, "set_cal_sel out of range");
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Calibration selection
    pub fn get_cal_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b11 << 12);
        value as u8
    }
    /// Gain in PGA mode
    /// Access: read-write, Width: 4, Offset: 14
    /// Set Gain in PGA mode
    pub fn set_pga_gain(value: u8) {
        debug_assert!(value <= 0b1111, "set_pga_gain out of range");
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Gain in PGA mode
    pub fn get_pga_gain() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1111 << 14);
        value as u8
    }
    /// User trimming enable
    /// Access: read-write, Width: 1, Offset: 18
    /// Set User trimming enable
    pub fn set_user_trim(value: bool) {
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get User trimming enable
    pub fn get_user_trim() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 18);
        value > 0
    }
    /// Offset trimming value (PMOS)
    /// Access: read-write, Width: 5, Offset: 19
    /// Set Offset trimming value (PMOS)
    pub fn set_trimoffsetp(value: u8) {
        debug_assert!(value <= 0b11111, "set_trimoffsetp out of range");
        let value = value as u32;
        let value = value << 19;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Offset trimming value (PMOS)
    pub fn get_trimoffsetp() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b11111 << 19);
        value as u8
    }
    /// Offset trimming value (NMOS)
    /// Access: read-write, Width: 5, Offset: 24
    /// Set Offset trimming value (NMOS)
    pub fn set_trimoffsetn(value: u8) {
        debug_assert!(value <= 0b11111, "set_trimoffsetn out of range");
        let value = value as u32;
        let value = value << 24;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Offset trimming value (NMOS)
    pub fn get_trimoffsetn() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b11111 << 24);
        value as u8
    }
    /// TSTREF
    /// Access: read-write, Width: 1, Offset: 29
    /// Set TSTREF
    pub fn set_tstref(value: bool) {
        let value = value as u32;
        let value = value << 29;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get TSTREF
    pub fn get_tstref() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 29);
        value > 0
    }
    /// OPAMP 2 ouput status flag
    /// Access: read-only, Width: 1, Offset: 30
    /// Get OPAMP 2 ouput status flag
    pub fn outcal() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 30);
        value > 0
    }
    /// OPAMP 2 lock
    /// Access: read-write, Width: 1, Offset: 31
    /// Set OPAMP 2 lock
    pub fn set_lock(value: bool) {
        let value = value as u32;
        let value = value << 31;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get OPAMP 2 lock
    pub fn get_lock() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
}
/// OPAMP3 control register
pub mod opamp3_cr {
    /// OPAMP3 enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set OPAMP3 enable
    pub fn set_opamp3en(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get OPAMP3 enable
    pub fn get_opamp3en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// FORCE_VP
    /// Access: read-write, Width: 1, Offset: 1
    /// Set FORCE_VP
    pub fn set_force_vp(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get FORCE_VP
    pub fn get_force_vp() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// OPAMP3 Non inverting input selection
    /// Access: read-write, Width: 2, Offset: 2
    /// Set OPAMP3 Non inverting input selection
    pub fn set_vp_sel(value: u8) {
        debug_assert!(value <= 0b11, "set_vp_sel out of range");
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get OPAMP3 Non inverting input selection
    pub fn get_vp_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b11 << 2);
        value as u8
    }
    /// OPAMP3 inverting input selection
    /// Access: read-write, Width: 2, Offset: 5
    /// Set OPAMP3 inverting input selection
    pub fn set_vm_sel(value: u8) {
        debug_assert!(value <= 0b11, "set_vm_sel out of range");
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get OPAMP3 inverting input selection
    pub fn get_vm_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b11 << 5);
        value as u8
    }
    /// Timer controlled Mux mode enable
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Timer controlled Mux mode enable
    pub fn set_tcm_en(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Timer controlled Mux mode enable
    pub fn get_tcm_en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// OPAMP3 inverting input secondary selection
    /// Access: read-write, Width: 1, Offset: 8
    /// Set OPAMP3 inverting input secondary selection
    pub fn set_vms_sel(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get OPAMP3 inverting input secondary selection
    pub fn get_vms_sel() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// OPAMP3 Non inverting input secondary selection
    /// Access: read-write, Width: 2, Offset: 9
    /// Set OPAMP3 Non inverting input secondary selection
    pub fn set_vps_sel(value: u8) {
        debug_assert!(value <= 0b11, "set_vps_sel out of range");
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get OPAMP3 Non inverting input secondary selection
    pub fn get_vps_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b11 << 9);
        value as u8
    }
    /// Calibration mode enable
    /// Access: read-write, Width: 1, Offset: 11
    /// Set Calibration mode enable
    pub fn set_calon(value: bool) {
        let value = value as u32;
        let value = value << 11;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Calibration mode enable
    pub fn get_calon() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Calibration selection
    /// Access: read-write, Width: 2, Offset: 12
    /// Set Calibration selection
    pub fn set_calsel(value: u8) {
        debug_assert!(value <= 0b11, "set_calsel out of range");
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Calibration selection
    pub fn get_calsel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b11 << 12);
        value as u8
    }
    /// Gain in PGA mode
    /// Access: read-write, Width: 4, Offset: 14
    /// Set Gain in PGA mode
    pub fn set_pga_gain(value: u8) {
        debug_assert!(value <= 0b1111, "set_pga_gain out of range");
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Gain in PGA mode
    pub fn get_pga_gain() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1111 << 14);
        value as u8
    }
    /// User trimming enable
    /// Access: read-write, Width: 1, Offset: 18
    /// Set User trimming enable
    pub fn set_user_trim(value: bool) {
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get User trimming enable
    pub fn get_user_trim() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 18);
        value > 0
    }
    /// Offset trimming value (PMOS)
    /// Access: read-write, Width: 5, Offset: 19
    /// Set Offset trimming value (PMOS)
    pub fn set_trimoffsetp(value: u8) {
        debug_assert!(value <= 0b11111, "set_trimoffsetp out of range");
        let value = value as u32;
        let value = value << 19;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Offset trimming value (PMOS)
    pub fn get_trimoffsetp() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b11111 << 19);
        value as u8
    }
    /// Offset trimming value (NMOS)
    /// Access: read-write, Width: 5, Offset: 24
    /// Set Offset trimming value (NMOS)
    pub fn set_trimoffsetn(value: u8) {
        debug_assert!(value <= 0b11111, "set_trimoffsetn out of range");
        let value = value as u32;
        let value = value << 24;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Offset trimming value (NMOS)
    pub fn get_trimoffsetn() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b11111 << 24);
        value as u8
    }
    /// TSTREF
    /// Access: read-write, Width: 1, Offset: 29
    /// Set TSTREF
    pub fn set_tstref(value: bool) {
        let value = value as u32;
        let value = value << 29;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get TSTREF
    pub fn get_tstref() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 29);
        value > 0
    }
    /// OPAMP 3 ouput status flag
    /// Access: read-only, Width: 1, Offset: 30
    /// Get OPAMP 3 ouput status flag
    pub fn outcal() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 30);
        value > 0
    }
    /// OPAMP 3 lock
    /// Access: read-write, Width: 1, Offset: 31
    /// Set OPAMP 3 lock
    pub fn set_lock(value: bool) {
        let value = value as u32;
        let value = value << 31;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get OPAMP 3 lock
    pub fn get_lock() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
}
/// OPAMP4 control register
pub mod opamp4_cr {
    /// OPAMP4 enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set OPAMP4 enable
    pub fn set_opamp4en(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get OPAMP4 enable
    pub fn get_opamp4en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// FORCE_VP
    /// Access: read-write, Width: 1, Offset: 1
    /// Set FORCE_VP
    pub fn set_force_vp(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get FORCE_VP
    pub fn get_force_vp() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// OPAMP4 Non inverting input selection
    /// Access: read-write, Width: 2, Offset: 2
    /// Set OPAMP4 Non inverting input selection
    pub fn set_vp_sel(value: u8) {
        debug_assert!(value <= 0b11, "set_vp_sel out of range");
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get OPAMP4 Non inverting input selection
    pub fn get_vp_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b11 << 2);
        value as u8
    }
    /// OPAMP4 inverting input selection
    /// Access: read-write, Width: 2, Offset: 5
    /// Set OPAMP4 inverting input selection
    pub fn set_vm_sel(value: u8) {
        debug_assert!(value <= 0b11, "set_vm_sel out of range");
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get OPAMP4 inverting input selection
    pub fn get_vm_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b11 << 5);
        value as u8
    }
    /// Timer controlled Mux mode enable
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Timer controlled Mux mode enable
    pub fn set_tcm_en(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Timer controlled Mux mode enable
    pub fn get_tcm_en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// OPAMP4 inverting input secondary selection
    /// Access: read-write, Width: 1, Offset: 8
    /// Set OPAMP4 inverting input secondary selection
    pub fn set_vms_sel(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get OPAMP4 inverting input secondary selection
    pub fn get_vms_sel() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// OPAMP4 Non inverting input secondary selection
    /// Access: read-write, Width: 2, Offset: 9
    /// Set OPAMP4 Non inverting input secondary selection
    pub fn set_vps_sel(value: u8) {
        debug_assert!(value <= 0b11, "set_vps_sel out of range");
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get OPAMP4 Non inverting input secondary selection
    pub fn get_vps_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b11 << 9);
        value as u8
    }
    /// Calibration mode enable
    /// Access: read-write, Width: 1, Offset: 11
    /// Set Calibration mode enable
    pub fn set_calon(value: bool) {
        let value = value as u32;
        let value = value << 11;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Calibration mode enable
    pub fn get_calon() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Calibration selection
    /// Access: read-write, Width: 2, Offset: 12
    /// Set Calibration selection
    pub fn set_calsel(value: u8) {
        debug_assert!(value <= 0b11, "set_calsel out of range");
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Calibration selection
    pub fn get_calsel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b11 << 12);
        value as u8
    }
    /// Gain in PGA mode
    /// Access: read-write, Width: 4, Offset: 14
    /// Set Gain in PGA mode
    pub fn set_pga_gain(value: u8) {
        debug_assert!(value <= 0b1111, "set_pga_gain out of range");
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Gain in PGA mode
    pub fn get_pga_gain() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1111 << 14);
        value as u8
    }
    /// User trimming enable
    /// Access: read-write, Width: 1, Offset: 18
    /// Set User trimming enable
    pub fn set_user_trim(value: bool) {
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get User trimming enable
    pub fn get_user_trim() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 18);
        value > 0
    }
    /// Offset trimming value (PMOS)
    /// Access: read-write, Width: 5, Offset: 19
    /// Set Offset trimming value (PMOS)
    pub fn set_trimoffsetp(value: u8) {
        debug_assert!(value <= 0b11111, "set_trimoffsetp out of range");
        let value = value as u32;
        let value = value << 19;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Offset trimming value (PMOS)
    pub fn get_trimoffsetp() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b11111 << 19);
        value as u8
    }
    /// Offset trimming value (NMOS)
    /// Access: read-write, Width: 5, Offset: 24
    /// Set Offset trimming value (NMOS)
    pub fn set_trimoffsetn(value: u8) {
        debug_assert!(value <= 0b11111, "set_trimoffsetn out of range");
        let value = value as u32;
        let value = value << 24;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Offset trimming value (NMOS)
    pub fn get_trimoffsetn() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b11111 << 24);
        value as u8
    }
    /// TSTREF
    /// Access: read-write, Width: 1, Offset: 29
    /// Set TSTREF
    pub fn set_tstref(value: bool) {
        let value = value as u32;
        let value = value << 29;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get TSTREF
    pub fn get_tstref() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 29);
        value > 0
    }
    /// OPAMP 4 ouput status flag
    /// Access: read-only, Width: 1, Offset: 30
    /// Get OPAMP 4 ouput status flag
    pub fn outcal() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 30);
        value > 0
    }
    /// OPAMP 4 lock
    /// Access: read-write, Width: 1, Offset: 31
    /// Set OPAMP 4 lock
    pub fn set_lock(value: bool) {
        let value = value as u32;
        let value = value << 31;
        unsafe { ::core::ptr::write_volatile((0x40010038u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get OPAMP 4 lock
    pub fn get_lock() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40010038u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x3C8</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40010038</baseAddress>
  <description>Operational amplifier</description>
  <groupName>OPAMP</groupName>
  <name>OPAMP</name>
  <registers>
    <register>
      <addressOffset>0x0</addressOffset>
      <description>OPAMP1 control register</description>
      <displayName>OPAMP1_CR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OPAMP1 enable</description>
          <name>OPAMP1_EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FORCE_VP</description>
          <name>FORCE_VP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                OPAMP1 Non inverting input
                                selection
                            </description>
          <name>VP_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                OPAMP1 inverting input
                                selection
                            </description>
          <name>VM_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Timer controlled Mux mode
                                enable
                            </description>
          <name>TCM_EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                OPAMP1 inverting input secondary
                                selection
                            </description>
          <name>VMS_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                OPAMP1 Non inverting input secondary
                                selection
                            </description>
          <name>VPS_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Calibration mode enable</description>
          <name>CALON</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Calibration selection</description>
          <name>CALSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Gain in PGA mode</description>
          <name>PGA_GAIN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>User trimming enable</description>
          <name>USER_TRIM</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>19</bitOffset>
          <bitWidth>5</bitWidth>
          <description>
                                Offset trimming value
                                (PMOS)
                            </description>
          <name>TRIMOFFSETP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>24</bitOffset>
          <bitWidth>5</bitWidth>
          <description>
                                Offset trimming value
                                (NMOS)
                            </description>
          <name>TRIMOFFSETN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TSTREF</description>
          <name>TSTREF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OPAMP 1 ouput status flag</description>
          <name>OUTCAL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OPAMP 1 lock</description>
          <name>LOCK</name>
        </field>
      </fields>
      <name>OPAMP1_CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x4</addressOffset>
      <description>OPAMP2 control register</description>
      <displayName>OPAMP2_CR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OPAMP2 enable</description>
          <name>OPAMP2EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FORCE_VP</description>
          <name>FORCE_VP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                OPAMP2 Non inverting input
                                selection
                            </description>
          <name>VP_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                OPAMP2 inverting input
                                selection
                            </description>
          <name>VM_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Timer controlled Mux mode
                                enable
                            </description>
          <name>TCM_EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                OPAMP2 inverting input secondary
                                selection
                            </description>
          <name>VMS_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                OPAMP2 Non inverting input secondary
                                selection
                            </description>
          <name>VPS_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Calibration mode enable</description>
          <name>CALON</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Calibration selection</description>
          <name>CAL_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Gain in PGA mode</description>
          <name>PGA_GAIN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>User trimming enable</description>
          <name>USER_TRIM</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>19</bitOffset>
          <bitWidth>5</bitWidth>
          <description>
                                Offset trimming value
                                (PMOS)
                            </description>
          <name>TRIMOFFSETP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>24</bitOffset>
          <bitWidth>5</bitWidth>
          <description>
                                Offset trimming value
                                (NMOS)
                            </description>
          <name>TRIMOFFSETN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TSTREF</description>
          <name>TSTREF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OPAMP 2 ouput status flag</description>
          <name>OUTCAL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OPAMP 2 lock</description>
          <name>LOCK</name>
        </field>
      </fields>
      <name>OPAMP2_CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x8</addressOffset>
      <description>OPAMP3 control register</description>
      <displayName>OPAMP3_CR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OPAMP3 enable</description>
          <name>OPAMP3EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FORCE_VP</description>
          <name>FORCE_VP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                OPAMP3 Non inverting input
                                selection
                            </description>
          <name>VP_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                OPAMP3 inverting input
                                selection
                            </description>
          <name>VM_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Timer controlled Mux mode
                                enable
                            </description>
          <name>TCM_EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                OPAMP3 inverting input secondary
                                selection
                            </description>
          <name>VMS_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                OPAMP3 Non inverting input secondary
                                selection
                            </description>
          <name>VPS_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Calibration mode enable</description>
          <name>CALON</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Calibration selection</description>
          <name>CALSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Gain in PGA mode</description>
          <name>PGA_GAIN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>User trimming enable</description>
          <name>USER_TRIM</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>19</bitOffset>
          <bitWidth>5</bitWidth>
          <description>
                                Offset trimming value
                                (PMOS)
                            </description>
          <name>TRIMOFFSETP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>24</bitOffset>
          <bitWidth>5</bitWidth>
          <description>
                                Offset trimming value
                                (NMOS)
                            </description>
          <name>TRIMOFFSETN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TSTREF</description>
          <name>TSTREF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OPAMP 3 ouput status flag</description>
          <name>OUTCAL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OPAMP 3 lock</description>
          <name>LOCK</name>
        </field>
      </fields>
      <name>OPAMP3_CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0xC</addressOffset>
      <description>OPAMP4 control register</description>
      <displayName>OPAMP4_CR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OPAMP4 enable</description>
          <name>OPAMP4EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FORCE_VP</description>
          <name>FORCE_VP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                OPAMP4 Non inverting input
                                selection
                            </description>
          <name>VP_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                OPAMP4 inverting input
                                selection
                            </description>
          <name>VM_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Timer controlled Mux mode
                                enable
                            </description>
          <name>TCM_EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                OPAMP4 inverting input secondary
                                selection
                            </description>
          <name>VMS_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                OPAMP4 Non inverting input secondary
                                selection
                            </description>
          <name>VPS_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Calibration mode enable</description>
          <name>CALON</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Calibration selection</description>
          <name>CALSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Gain in PGA mode</description>
          <name>PGA_GAIN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>User trimming enable</description>
          <name>USER_TRIM</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>19</bitOffset>
          <bitWidth>5</bitWidth>
          <description>
                                Offset trimming value
                                (PMOS)
                            </description>
          <name>TRIMOFFSETP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>24</bitOffset>
          <bitWidth>5</bitWidth>
          <description>
                                Offset trimming value
                                (NMOS)
                            </description>
          <name>TRIMOFFSETN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TSTREF</description>
          <name>TSTREF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OPAMP 4 ouput status flag</description>
          <name>OUTCAL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OPAMP 4 lock</description>
          <name>LOCK</name>
        </field>
      </fields>
      <name>OPAMP4_CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
