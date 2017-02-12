/// control and status register
pub mod comp1_csr {
    /// Comparator 1 enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set Comparator 1 enable
    pub fn set_comp1en(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Comparator 1 enable
    pub fn get_comp1en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// COMP1_INP_DAC
    /// Access: read-write, Width: 1, Offset: 1
    /// Set COMP1_INP_DAC
    pub fn set_comp1_inp_dac(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x0u32) as *mut u32, value) };
    }
    /// Get COMP1_INP_DAC
    pub fn get_comp1_inp_dac() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// Comparator 1 mode
    /// Access: read-write, Width: 2, Offset: 2
    /// Set Comparator 1 mode
    pub fn set_comp1mode(value: u8) {
        debug_assert!(value <= 0b11, "set_comp1mode out of range");
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Comparator 1 mode
    pub fn get_comp1mode() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x0u32) as *mut u32) };
        let value = value & (0b11 << 2);
        value as u8
    }
    /// Comparator 1 inverting input selection
    /// Access: read-write, Width: 3, Offset: 4
    /// Set Comparator 1 inverting input selection
    pub fn set_comp1insel(value: u8) {
        debug_assert!(value <= 0b111, "set_comp1insel out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Comparator 1 inverting input selection
    pub fn get_comp1insel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x0u32) as *mut u32) };
        let value = value & (0b111 << 4);
        value as u8
    }
    /// Comparator 1 output selection
    /// Access: read-write, Width: 4, Offset: 10
    /// Set Comparator 1 output selection
    pub fn set_comp1_out_sel(value: u8) {
        debug_assert!(value <= 0b1111, "set_comp1_out_sel out of range");
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Comparator 1 output selection
    pub fn get_comp1_out_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x0u32) as *mut u32) };
        let value = value & (0b1111 << 10);
        value as u8
    }
    /// Comparator 1 output polarity
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Comparator 1 output polarity
    pub fn set_comp1pol(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Comparator 1 output polarity
    pub fn get_comp1pol() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// Comparator 1 hysteresis
    /// Access: read-write, Width: 2, Offset: 16
    /// Set Comparator 1 hysteresis
    pub fn set_comp1hyst(value: u8) {
        debug_assert!(value <= 0b11, "set_comp1hyst out of range");
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Comparator 1 hysteresis
    pub fn get_comp1hyst() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x0u32) as *mut u32) };
        let value = value & (0b11 << 16);
        value as u8
    }
    /// Comparator 1 blanking source
    /// Access: read-write, Width: 3, Offset: 18
    /// Set Comparator 1 blanking source
    pub fn set_comp1_blanking(value: u8) {
        debug_assert!(value <= 0b111, "set_comp1_blanking out of range");
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Comparator 1 blanking source
    pub fn get_comp1_blanking() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x0u32) as *mut u32) };
        let value = value & (0b111 << 18);
        value as u8
    }
    /// Comparator 1 output
    /// Access: read-only, Width: 1, Offset: 30
    /// Get Comparator 1 output
    pub fn comp1out() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 30);
        value > 0
    }
    /// Comparator 1 lock
    /// Access: read-write, Width: 1, Offset: 31
    /// Set Comparator 1 lock
    pub fn set_comp1lock(value: bool) {
        let value = value as u32;
        let value = value << 31;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Comparator 1 lock
    pub fn get_comp1lock() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
}
/// control and status register
pub mod comp2_csr {
    /// Comparator 2 enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set Comparator 2 enable
    pub fn set_comp2en(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Comparator 2 enable
    pub fn get_comp2en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Comparator 2 mode
    /// Access: read-write, Width: 2, Offset: 2
    /// Set Comparator 2 mode
    pub fn set_comp2mode(value: u8) {
        debug_assert!(value <= 0b11, "set_comp2mode out of range");
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Comparator 2 mode
    pub fn get_comp2mode() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x4u32) as *mut u32) };
        let value = value & (0b11 << 2);
        value as u8
    }
    /// Comparator 2 inverting input selection
    /// Access: read-write, Width: 3, Offset: 4
    /// Set Comparator 2 inverting input selection
    pub fn set_comp2insel(value: u8) {
        debug_assert!(value <= 0b111, "set_comp2insel out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Comparator 2 inverting input selection
    pub fn get_comp2insel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x4u32) as *mut u32) };
        let value = value & (0b111 << 4);
        value as u8
    }
    /// Comparator 2 non inverted input selection
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Comparator 2 non inverted input selection
    pub fn set_comp2inpsel(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Comparator 2 non inverted input selection
    pub fn get_comp2inpsel() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Comparator 1inverting input selection
    /// Access: read-write, Width: 1, Offset: 9
    /// Set Comparator 1inverting input selection
    pub fn set_comp2inmsel(value: bool) {
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Comparator 1inverting input selection
    pub fn get_comp2inmsel() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// Comparator 2 output selection
    /// Access: read-write, Width: 4, Offset: 10
    /// Set Comparator 2 output selection
    pub fn set_comp2_out_sel(value: u8) {
        debug_assert!(value <= 0b1111, "set_comp2_out_sel out of range");
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Comparator 2 output selection
    pub fn get_comp2_out_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x4u32) as *mut u32) };
        let value = value & (0b1111 << 10);
        value as u8
    }
    /// Comparator 2 output polarity
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Comparator 2 output polarity
    pub fn set_comp2pol(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Comparator 2 output polarity
    pub fn get_comp2pol() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// Comparator 2 hysteresis
    /// Access: read-write, Width: 2, Offset: 16
    /// Set Comparator 2 hysteresis
    pub fn set_comp2hyst(value: u8) {
        debug_assert!(value <= 0b11, "set_comp2hyst out of range");
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Comparator 2 hysteresis
    pub fn get_comp2hyst() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x4u32) as *mut u32) };
        let value = value & (0b11 << 16);
        value as u8
    }
    /// Comparator 2 blanking source
    /// Access: read-write, Width: 3, Offset: 18
    /// Set Comparator 2 blanking source
    pub fn set_comp2_blanking(value: u8) {
        debug_assert!(value <= 0b111, "set_comp2_blanking out of range");
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Comparator 2 blanking source
    pub fn get_comp2_blanking() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x4u32) as *mut u32) };
        let value = value & (0b111 << 18);
        value as u8
    }
    /// Comparator 2 output
    /// Access: read-only, Width: 1, Offset: 30
    /// Get Comparator 2 output
    pub fn comp2out() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 30);
        value > 0
    }
    /// Comparator 2 lock
    /// Access: read-write, Width: 1, Offset: 31
    /// Set Comparator 2 lock
    pub fn set_comp2lock(value: bool) {
        let value = value as u32;
        let value = value << 31;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Comparator 2 lock
    pub fn get_comp2lock() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
}
/// control and status register
pub mod comp3_csr {
    /// Comparator 3 enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set Comparator 3 enable
    pub fn set_comp3en(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Comparator 3 enable
    pub fn get_comp3en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Comparator 3 mode
    /// Access: read-write, Width: 2, Offset: 2
    /// Set Comparator 3 mode
    pub fn set_comp3mode(value: u8) {
        debug_assert!(value <= 0b11, "set_comp3mode out of range");
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Comparator 3 mode
    pub fn get_comp3mode() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x8u32) as *mut u32) };
        let value = value & (0b11 << 2);
        value as u8
    }
    /// Comparator 3 inverting input selection
    /// Access: read-write, Width: 3, Offset: 4
    /// Set Comparator 3 inverting input selection
    pub fn set_comp3insel(value: u8) {
        debug_assert!(value <= 0b111, "set_comp3insel out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Comparator 3 inverting input selection
    pub fn get_comp3insel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x8u32) as *mut u32) };
        let value = value & (0b111 << 4);
        value as u8
    }
    /// Comparator 3 non inverted input selection
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Comparator 3 non inverted input selection
    pub fn set_comp3inpsel(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Comparator 3 non inverted input selection
    pub fn get_comp3inpsel() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Comparator 3 output selection
    /// Access: read-write, Width: 4, Offset: 10
    /// Set Comparator 3 output selection
    pub fn set_comp3_out_sel(value: u8) {
        debug_assert!(value <= 0b1111, "set_comp3_out_sel out of range");
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Comparator 3 output selection
    pub fn get_comp3_out_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x8u32) as *mut u32) };
        let value = value & (0b1111 << 10);
        value as u8
    }
    /// Comparator 3 output polarity
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Comparator 3 output polarity
    pub fn set_comp3pol(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Comparator 3 output polarity
    pub fn get_comp3pol() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// Comparator 3 hysteresis
    /// Access: read-write, Width: 2, Offset: 16
    /// Set Comparator 3 hysteresis
    pub fn set_comp3hyst(value: u8) {
        debug_assert!(value <= 0b11, "set_comp3hyst out of range");
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Comparator 3 hysteresis
    pub fn get_comp3hyst() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x8u32) as *mut u32) };
        let value = value & (0b11 << 16);
        value as u8
    }
    /// Comparator 3 blanking source
    /// Access: read-write, Width: 3, Offset: 18
    /// Set Comparator 3 blanking source
    pub fn set_comp3_blanking(value: u8) {
        debug_assert!(value <= 0b111, "set_comp3_blanking out of range");
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Comparator 3 blanking source
    pub fn get_comp3_blanking() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x8u32) as *mut u32) };
        let value = value & (0b111 << 18);
        value as u8
    }
    /// Comparator 3 output
    /// Access: read-only, Width: 1, Offset: 30
    /// Get Comparator 3 output
    pub fn comp3out() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 30);
        value > 0
    }
    /// Comparator 3 lock
    /// Access: read-write, Width: 1, Offset: 31
    /// Set Comparator 3 lock
    pub fn set_comp3lock(value: bool) {
        let value = value as u32;
        let value = value << 31;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Comparator 3 lock
    pub fn get_comp3lock() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
}
/// control and status register
pub mod comp4_csr {
    /// Comparator 4 enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set Comparator 4 enable
    pub fn set_comp4en(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Comparator 4 enable
    pub fn get_comp4en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Comparator 4 mode
    /// Access: read-write, Width: 2, Offset: 2
    /// Set Comparator 4 mode
    pub fn set_comp4mode(value: u8) {
        debug_assert!(value <= 0b11, "set_comp4mode out of range");
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Comparator 4 mode
    pub fn get_comp4mode() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0xCu32) as *mut u32) };
        let value = value & (0b11 << 2);
        value as u8
    }
    /// Comparator 4 inverting input selection
    /// Access: read-write, Width: 3, Offset: 4
    /// Set Comparator 4 inverting input selection
    pub fn set_comp4insel(value: u8) {
        debug_assert!(value <= 0b111, "set_comp4insel out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Comparator 4 inverting input selection
    pub fn get_comp4insel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0xCu32) as *mut u32) };
        let value = value & (0b111 << 4);
        value as u8
    }
    /// Comparator 4 non inverted input selection
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Comparator 4 non inverted input selection
    pub fn set_comp4inpsel(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Comparator 4 non inverted input selection
    pub fn get_comp4inpsel() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Comparator 4 window mode
    /// Access: read-write, Width: 1, Offset: 9
    /// Set Comparator 4 window mode
    pub fn set_com4winmode(value: bool) {
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Comparator 4 window mode
    pub fn get_com4winmode() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// Comparator 4 output selection
    /// Access: read-write, Width: 4, Offset: 10
    /// Set Comparator 4 output selection
    pub fn set_comp4_out_sel(value: u8) {
        debug_assert!(value <= 0b1111, "set_comp4_out_sel out of range");
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Comparator 4 output selection
    pub fn get_comp4_out_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0xCu32) as *mut u32) };
        let value = value & (0b1111 << 10);
        value as u8
    }
    /// Comparator 4 output polarity
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Comparator 4 output polarity
    pub fn set_comp4pol(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Comparator 4 output polarity
    pub fn get_comp4pol() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// Comparator 4 hysteresis
    /// Access: read-write, Width: 2, Offset: 16
    /// Set Comparator 4 hysteresis
    pub fn set_comp4hyst(value: u8) {
        debug_assert!(value <= 0b11, "set_comp4hyst out of range");
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Comparator 4 hysteresis
    pub fn get_comp4hyst() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0xCu32) as *mut u32) };
        let value = value & (0b11 << 16);
        value as u8
    }
    /// Comparator 4 blanking source
    /// Access: read-write, Width: 3, Offset: 18
    /// Set Comparator 4 blanking source
    pub fn set_comp4_blanking(value: u8) {
        debug_assert!(value <= 0b111, "set_comp4_blanking out of range");
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Comparator 4 blanking source
    pub fn get_comp4_blanking() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0xCu32) as *mut u32) };
        let value = value & (0b111 << 18);
        value as u8
    }
    /// Comparator 4 output
    /// Access: read-only, Width: 1, Offset: 30
    /// Get Comparator 4 output
    pub fn comp4out() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 30);
        value > 0
    }
    /// Comparator 4 lock
    /// Access: read-write, Width: 1, Offset: 31
    /// Set Comparator 4 lock
    pub fn set_comp4lock(value: bool) {
        let value = value as u32;
        let value = value << 31;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Comparator 4 lock
    pub fn get_comp4lock() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
}
/// control and status register
pub mod comp5_csr {
    /// Comparator 5 enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set Comparator 5 enable
    pub fn set_comp5en(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Comparator 5 enable
    pub fn get_comp5en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Comparator 5 mode
    /// Access: read-write, Width: 2, Offset: 2
    /// Set Comparator 5 mode
    pub fn set_comp5mode(value: u8) {
        debug_assert!(value <= 0b11, "set_comp5mode out of range");
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Comparator 5 mode
    pub fn get_comp5mode() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x10u32) as *mut u32) };
        let value = value & (0b11 << 2);
        value as u8
    }
    /// Comparator 5 inverting input selection
    /// Access: read-write, Width: 3, Offset: 4
    /// Set Comparator 5 inverting input selection
    pub fn set_comp5insel(value: u8) {
        debug_assert!(value <= 0b111, "set_comp5insel out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Comparator 5 inverting input selection
    pub fn get_comp5insel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x10u32) as *mut u32) };
        let value = value & (0b111 << 4);
        value as u8
    }
    /// Comparator 5 non inverted input selection
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Comparator 5 non inverted input selection
    pub fn set_comp5inpsel(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Comparator 5 non inverted input selection
    pub fn get_comp5inpsel() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Comparator 5 output selection
    /// Access: read-write, Width: 4, Offset: 10
    /// Set Comparator 5 output selection
    pub fn set_comp5_out_sel(value: u8) {
        debug_assert!(value <= 0b1111, "set_comp5_out_sel out of range");
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Comparator 5 output selection
    pub fn get_comp5_out_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x10u32) as *mut u32) };
        let value = value & (0b1111 << 10);
        value as u8
    }
    /// Comparator 5 output polarity
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Comparator 5 output polarity
    pub fn set_comp5pol(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Comparator 5 output polarity
    pub fn get_comp5pol() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// Comparator 5 hysteresis
    /// Access: read-write, Width: 2, Offset: 16
    /// Set Comparator 5 hysteresis
    pub fn set_comp5hyst(value: u8) {
        debug_assert!(value <= 0b11, "set_comp5hyst out of range");
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Comparator 5 hysteresis
    pub fn get_comp5hyst() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x10u32) as *mut u32) };
        let value = value & (0b11 << 16);
        value as u8
    }
    /// Comparator 5 blanking source
    /// Access: read-write, Width: 3, Offset: 18
    /// Set Comparator 5 blanking source
    pub fn set_comp5_blanking(value: u8) {
        debug_assert!(value <= 0b111, "set_comp5_blanking out of range");
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Comparator 5 blanking source
    pub fn get_comp5_blanking() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x10u32) as *mut u32) };
        let value = value & (0b111 << 18);
        value as u8
    }
    /// Comparator51 output
    /// Access: read-only, Width: 1, Offset: 30
    /// Get Comparator51 output
    pub fn comp5out() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << 30);
        value > 0
    }
    /// Comparator 5 lock
    /// Access: read-write, Width: 1, Offset: 31
    /// Set Comparator 5 lock
    pub fn set_comp5lock(value: bool) {
        let value = value as u32;
        let value = value << 31;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Comparator 5 lock
    pub fn get_comp5lock() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
}
/// control and status register
pub mod comp6_csr {
    /// Comparator 6 enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set Comparator 6 enable
    pub fn set_comp6en(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Comparator 6 enable
    pub fn get_comp6en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x14u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Comparator 6 mode
    /// Access: read-write, Width: 2, Offset: 2
    /// Set Comparator 6 mode
    pub fn set_comp6mode(value: u8) {
        debug_assert!(value <= 0b11, "set_comp6mode out of range");
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Comparator 6 mode
    pub fn get_comp6mode() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x14u32) as *mut u32) };
        let value = value & (0b11 << 2);
        value as u8
    }
    /// Comparator 6 inverting input selection
    /// Access: read-write, Width: 3, Offset: 4
    /// Set Comparator 6 inverting input selection
    pub fn set_comp6insel(value: u8) {
        debug_assert!(value <= 0b111, "set_comp6insel out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Comparator 6 inverting input selection
    pub fn get_comp6insel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x14u32) as *mut u32) };
        let value = value & (0b111 << 4);
        value as u8
    }
    /// Comparator 6 non inverted input selection
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Comparator 6 non inverted input selection
    pub fn set_comp6inpsel(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Comparator 6 non inverted input selection
    pub fn get_comp6inpsel() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x14u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Comparator 6 window mode
    /// Access: read-write, Width: 1, Offset: 9
    /// Set Comparator 6 window mode
    pub fn set_com6winmode(value: bool) {
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Comparator 6 window mode
    pub fn get_com6winmode() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x14u32) as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// Comparator 6 output selection
    /// Access: read-write, Width: 4, Offset: 10
    /// Set Comparator 6 output selection
    pub fn set_comp6_out_sel(value: u8) {
        debug_assert!(value <= 0b1111, "set_comp6_out_sel out of range");
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Comparator 6 output selection
    pub fn get_comp6_out_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x14u32) as *mut u32) };
        let value = value & (0b1111 << 10);
        value as u8
    }
    /// Comparator 6 output polarity
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Comparator 6 output polarity
    pub fn set_comp6pol(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Comparator 6 output polarity
    pub fn get_comp6pol() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x14u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// Comparator 6 hysteresis
    /// Access: read-write, Width: 2, Offset: 16
    /// Set Comparator 6 hysteresis
    pub fn set_comp6hyst(value: u8) {
        debug_assert!(value <= 0b11, "set_comp6hyst out of range");
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Comparator 6 hysteresis
    pub fn get_comp6hyst() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x14u32) as *mut u32) };
        let value = value & (0b11 << 16);
        value as u8
    }
    /// Comparator 6 blanking source
    /// Access: read-write, Width: 3, Offset: 18
    /// Set Comparator 6 blanking source
    pub fn set_comp6_blanking(value: u8) {
        debug_assert!(value <= 0b111, "set_comp6_blanking out of range");
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Comparator 6 blanking source
    pub fn get_comp6_blanking() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x14u32) as *mut u32) };
        let value = value & (0b111 << 18);
        value as u8
    }
    /// Comparator 6 output
    /// Access: read-only, Width: 1, Offset: 30
    /// Get Comparator 6 output
    pub fn comp6out() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x14u32) as *mut u32) };
        let value = value & (0b1 << 30);
        value > 0
    }
    /// Comparator 6 lock
    /// Access: read-write, Width: 1, Offset: 31
    /// Set Comparator 6 lock
    pub fn set_comp6lock(value: bool) {
        let value = value as u32;
        let value = value << 31;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Comparator 6 lock
    pub fn get_comp6lock() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x14u32) as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
}
/// control and status register
pub mod comp7_csr {
    /// Comparator 7 enable
    /// Access: read-write, Width: 1, Offset: 0
    /// Set Comparator 7 enable
    pub fn set_comp7en(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Comparator 7 enable
    pub fn get_comp7en() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Comparator 7 mode
    /// Access: read-write, Width: 2, Offset: 2
    /// Set Comparator 7 mode
    pub fn set_comp7mode(value: u8) {
        debug_assert!(value <= 0b11, "set_comp7mode out of range");
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Comparator 7 mode
    pub fn get_comp7mode() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x18u32) as *mut u32) };
        let value = value & (0b11 << 2);
        value as u8
    }
    /// Comparator 7 inverting input selection
    /// Access: read-write, Width: 3, Offset: 4
    /// Set Comparator 7 inverting input selection
    pub fn set_comp7insel(value: u8) {
        debug_assert!(value <= 0b111, "set_comp7insel out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Comparator 7 inverting input selection
    pub fn get_comp7insel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x18u32) as *mut u32) };
        let value = value & (0b111 << 4);
        value as u8
    }
    /// Comparator 7 non inverted input selection
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Comparator 7 non inverted input selection
    pub fn set_comp7inpsel(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Comparator 7 non inverted input selection
    pub fn get_comp7inpsel() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Comparator 7 output selection
    /// Access: read-write, Width: 4, Offset: 10
    /// Set Comparator 7 output selection
    pub fn set_comp7_out_sel(value: u8) {
        debug_assert!(value <= 0b1111, "set_comp7_out_sel out of range");
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Comparator 7 output selection
    pub fn get_comp7_out_sel() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x18u32) as *mut u32) };
        let value = value & (0b1111 << 10);
        value as u8
    }
    /// Comparator 7 output polarity
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Comparator 7 output polarity
    pub fn set_comp7pol(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Comparator 7 output polarity
    pub fn get_comp7pol() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// Comparator 7 hysteresis
    /// Access: read-write, Width: 2, Offset: 16
    /// Set Comparator 7 hysteresis
    pub fn set_comp7hyst(value: u8) {
        debug_assert!(value <= 0b11, "set_comp7hyst out of range");
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Comparator 7 hysteresis
    pub fn get_comp7hyst() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x18u32) as *mut u32) };
        let value = value & (0b11 << 16);
        value as u8
    }
    /// Comparator 7 blanking source
    /// Access: read-write, Width: 3, Offset: 18
    /// Set Comparator 7 blanking source
    pub fn set_comp7_blanking(value: u8) {
        debug_assert!(value <= 0b111, "set_comp7_blanking out of range");
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Comparator 7 blanking source
    pub fn get_comp7_blanking() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x18u32) as *mut u32) };
        let value = value & (0b111 << 18);
        value as u8
    }
    /// Comparator 7 output
    /// Access: read-only, Width: 1, Offset: 30
    /// Get Comparator 7 output
    pub fn comp7out() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 30);
        value > 0
    }
    /// Comparator 7 lock
    /// Access: read-write, Width: 1, Offset: 31
    /// Set Comparator 7 lock
    pub fn set_comp7lock(value: bool) {
        let value = value as u32;
        let value = value << 31;
        unsafe { ::core::ptr::write_volatile((0x4001001Cu32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Comparator 7 lock
    pub fn get_comp7lock() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x4001001Cu32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
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
  <addressBlock>
    <offset>0x0</offset>
    <size>0x19</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x4001001C</baseAddress>
  <description>Comparator</description>
  <groupName>COMP</groupName>
  <interrupt>
    <description>
                    COMP1 &amp; COMP2 &amp; COMP3 interrupts
                    combined with EXTI Lines 21, 22 and 29
                    interrupts
                </description>
    <name>COMP123</name>
    <value>64</value>
  </interrupt>
  <interrupt>
    <description>
                    COMP4 &amp; COMP5 &amp; COMP6 interrupts
                    combined with EXTI Lines 30, 31 and 32
                    interrupts
                </description>
    <name>COMP456</name>
    <value>65</value>
  </interrupt>
  <interrupt>
    <description>
                    COMP7 interrupt combined with EXTI Line 33
                    interrupt
                </description>
    <name>COMP7</name>
    <value>66</value>
  </interrupt>
  <name>COMP</name>
  <registers>
    <register>
      <addressOffset>0x0</addressOffset>
      <description>control and status register</description>
      <displayName>COMP1_CSR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 1 enable</description>
          <name>COMP1EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>COMP1_INP_DAC</description>
          <name>COMP1_INP_DAC</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Comparator 1 mode</description>
          <name>COMP1MODE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Comparator 1 inverting input
                                selection
                            </description>
          <name>COMP1INSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>10</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Comparator 1 output
                                selection
                            </description>
          <name>COMP1_OUT_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Comparator 1 output
                                polarity
                            </description>
          <name>COMP1POL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Comparator 1 hysteresis</description>
          <name>COMP1HYST</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Comparator 1 blanking
                                source
                            </description>
          <name>COMP1_BLANKING</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 1 output</description>
          <name>COMP1OUT</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 1 lock</description>
          <name>COMP1LOCK</name>
        </field>
      </fields>
      <name>COMP1_CSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x4</addressOffset>
      <description>control and status register</description>
      <displayName>COMP2_CSR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 2 enable</description>
          <name>COMP2EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Comparator 2 mode</description>
          <name>COMP2MODE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Comparator 2 inverting input
                                selection
                            </description>
          <name>COMP2INSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Comparator 2 non inverted input
                                selection
                            </description>
          <name>COMP2INPSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Comparator 1inverting input
                                selection
                            </description>
          <name>COMP2INMSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>10</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Comparator 2 output
                                selection
                            </description>
          <name>COMP2_OUT_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Comparator 2 output
                                polarity
                            </description>
          <name>COMP2POL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Comparator 2 hysteresis</description>
          <name>COMP2HYST</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Comparator 2 blanking
                                source
                            </description>
          <name>COMP2_BLANKING</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 2 output</description>
          <name>COMP2OUT</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 2 lock</description>
          <name>COMP2LOCK</name>
        </field>
      </fields>
      <name>COMP2_CSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x8</addressOffset>
      <description>control and status register</description>
      <displayName>COMP3_CSR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 3 enable</description>
          <name>COMP3EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Comparator 3 mode</description>
          <name>COMP3MODE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Comparator 3 inverting input
                                selection
                            </description>
          <name>COMP3INSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Comparator 3 non inverted input
                                selection
                            </description>
          <name>COMP3INPSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>10</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Comparator 3 output
                                selection
                            </description>
          <name>COMP3_OUT_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Comparator 3 output
                                polarity
                            </description>
          <name>COMP3POL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Comparator 3 hysteresis</description>
          <name>COMP3HYST</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Comparator 3 blanking
                                source
                            </description>
          <name>COMP3_BLANKING</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 3 output</description>
          <name>COMP3OUT</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 3 lock</description>
          <name>COMP3LOCK</name>
        </field>
      </fields>
      <name>COMP3_CSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0xC</addressOffset>
      <description>control and status register</description>
      <displayName>COMP4_CSR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 4 enable</description>
          <name>COMP4EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Comparator 4 mode</description>
          <name>COMP4MODE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Comparator 4 inverting input
                                selection
                            </description>
          <name>COMP4INSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Comparator 4 non inverted input
                                selection
                            </description>
          <name>COMP4INPSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 4 window mode</description>
          <name>COM4WINMODE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>10</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Comparator 4 output
                                selection
                            </description>
          <name>COMP4_OUT_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Comparator 4 output
                                polarity
                            </description>
          <name>COMP4POL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Comparator 4 hysteresis</description>
          <name>COMP4HYST</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Comparator 4 blanking
                                source
                            </description>
          <name>COMP4_BLANKING</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 4 output</description>
          <name>COMP4OUT</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 4 lock</description>
          <name>COMP4LOCK</name>
        </field>
      </fields>
      <name>COMP4_CSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x10</addressOffset>
      <description>control and status register</description>
      <displayName>COMP5_CSR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 5 enable</description>
          <name>COMP5EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Comparator 5 mode</description>
          <name>COMP5MODE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Comparator 5 inverting input
                                selection
                            </description>
          <name>COMP5INSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Comparator 5 non inverted input
                                selection
                            </description>
          <name>COMP5INPSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>10</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Comparator 5 output
                                selection
                            </description>
          <name>COMP5_OUT_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Comparator 5 output
                                polarity
                            </description>
          <name>COMP5POL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Comparator 5 hysteresis</description>
          <name>COMP5HYST</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Comparator 5 blanking
                                source
                            </description>
          <name>COMP5_BLANKING</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator51 output</description>
          <name>COMP5OUT</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 5 lock</description>
          <name>COMP5LOCK</name>
        </field>
      </fields>
      <name>COMP5_CSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x14</addressOffset>
      <description>control and status register</description>
      <displayName>COMP6_CSR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 6 enable</description>
          <name>COMP6EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Comparator 6 mode</description>
          <name>COMP6MODE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Comparator 6 inverting input
                                selection
                            </description>
          <name>COMP6INSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Comparator 6 non inverted input
                                selection
                            </description>
          <name>COMP6INPSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 6 window mode</description>
          <name>COM6WINMODE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>10</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Comparator 6 output
                                selection
                            </description>
          <name>COMP6_OUT_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Comparator 6 output
                                polarity
                            </description>
          <name>COMP6POL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Comparator 6 hysteresis</description>
          <name>COMP6HYST</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Comparator 6 blanking
                                source
                            </description>
          <name>COMP6_BLANKING</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 6 output</description>
          <name>COMP6OUT</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 6 lock</description>
          <name>COMP6LOCK</name>
        </field>
      </fields>
      <name>COMP6_CSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x18</addressOffset>
      <description>control and status register</description>
      <displayName>COMP7_CSR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 7 enable</description>
          <name>COMP7EN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Comparator 7 mode</description>
          <name>COMP7MODE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Comparator 7 inverting input
                                selection
                            </description>
          <name>COMP7INSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Comparator 7 non inverted input
                                selection
                            </description>
          <name>COMP7INPSEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>10</bitOffset>
          <bitWidth>4</bitWidth>
          <description>
                                Comparator 7 output
                                selection
                            </description>
          <name>COMP7_OUT_SEL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Comparator 7 output
                                polarity
                            </description>
          <name>COMP7POL</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Comparator 7 hysteresis</description>
          <name>COMP7HYST</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Comparator 7 blanking
                                source
                            </description>
          <name>COMP7_BLANKING</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 7 output</description>
          <name>COMP7OUT</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Comparator 7 lock</description>
          <name>COMP7LOCK</name>
        </field>
      </fields>
      <name>COMP7_CSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
