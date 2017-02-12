/// endpoint 0 register
pub mod usb_ep0r {
    /// Endpoint address
    /// Access: read-write, Width: 4, Offset: 0
    /// Set Endpoint address
    pub fn set_ea(value: u8) {
        debug_assert!(value <= 0b1111, "set_ea out of range");
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Endpoint address
    pub fn get_ea() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
    /// Status bits, for transmission transfers
    /// Access: read-write, Width: 2, Offset: 4
    /// Set Status bits, for transmission transfers
    pub fn set_stat_tx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_tx out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Status bits, for transmission transfers
    pub fn get_stat_tx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x0u32) as *mut u32) };
        let value = value & (0b11 << 4);
        value as u8
    }
    /// Data Toggle, for transmission transfers
    /// Access: read-write, Width: 1, Offset: 6
    /// Set Data Toggle, for transmission transfers
    pub fn set_dtog_tx(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Data Toggle, for transmission transfers
    pub fn get_dtog_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Correct Transfer for transmission
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Correct Transfer for transmission
    pub fn set_ctr_tx(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Correct Transfer for transmission
    pub fn get_ctr_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Endpoint kind
    /// Access: read-write, Width: 1, Offset: 8
    /// Set Endpoint kind
    pub fn set_ep_kind(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Endpoint kind
    pub fn get_ep_kind() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Endpoint type
    /// Access: read-write, Width: 2, Offset: 9
    /// Set Endpoint type
    pub fn set_ep_type(value: u8) {
        debug_assert!(value <= 0b11, "set_ep_type out of range");
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Endpoint type
    pub fn get_ep_type() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x0u32) as *mut u32) };
        let value = value & (0b11 << 9);
        value as u8
    }
    /// Setup transaction completed
    /// Access: read-only, Width: 1, Offset: 11
    /// Get Setup transaction completed
    pub fn setup() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Status bits, for reception transfers
    /// Access: read-write, Width: 2, Offset: 12
    /// Set Status bits, for reception transfers
    pub fn set_stat_rx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_rx out of range");
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Status bits, for reception transfers
    pub fn get_stat_rx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x0u32) as *mut u32) };
        let value = value & (0b11 << 12);
        value as u8
    }
    /// Data Toggle, for reception transfers
    /// Access: read-write, Width: 1, Offset: 14
    /// Set Data Toggle, for reception transfers
    pub fn set_dtog_rx(value: bool) {
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Data Toggle, for reception transfers
    pub fn get_dtog_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// Correct transfer for reception
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Correct transfer for reception
    pub fn set_ctr_rx(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x0u32) as *mut u32, value) };
    }
    /// Get Correct transfer for reception
    pub fn get_ctr_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x0u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
}
/// endpoint 1 register
pub mod usb_ep1r {
    /// Endpoint address
    /// Access: read-write, Width: 4, Offset: 0
    /// Set Endpoint address
    pub fn set_ea(value: u8) {
        debug_assert!(value <= 0b1111, "set_ea out of range");
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Endpoint address
    pub fn get_ea() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
    /// Status bits, for transmission transfers
    /// Access: read-write, Width: 2, Offset: 4
    /// Set Status bits, for transmission transfers
    pub fn set_stat_tx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_tx out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Status bits, for transmission transfers
    pub fn get_stat_tx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x4u32) as *mut u32) };
        let value = value & (0b11 << 4);
        value as u8
    }
    /// Data Toggle, for transmission transfers
    /// Access: read-write, Width: 1, Offset: 6
    /// Set Data Toggle, for transmission transfers
    pub fn set_dtog_tx(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Data Toggle, for transmission transfers
    pub fn get_dtog_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Correct Transfer for transmission
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Correct Transfer for transmission
    pub fn set_ctr_tx(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Correct Transfer for transmission
    pub fn get_ctr_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Endpoint kind
    /// Access: read-write, Width: 1, Offset: 8
    /// Set Endpoint kind
    pub fn set_ep_kind(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Endpoint kind
    pub fn get_ep_kind() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Endpoint type
    /// Access: read-write, Width: 2, Offset: 9
    /// Set Endpoint type
    pub fn set_ep_type(value: u8) {
        debug_assert!(value <= 0b11, "set_ep_type out of range");
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Endpoint type
    pub fn get_ep_type() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x4u32) as *mut u32) };
        let value = value & (0b11 << 9);
        value as u8
    }
    /// Setup transaction completed
    /// Access: read-only, Width: 1, Offset: 11
    /// Get Setup transaction completed
    pub fn setup() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Status bits, for reception transfers
    /// Access: read-write, Width: 2, Offset: 12
    /// Set Status bits, for reception transfers
    pub fn set_stat_rx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_rx out of range");
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Status bits, for reception transfers
    pub fn get_stat_rx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x4u32) as *mut u32) };
        let value = value & (0b11 << 12);
        value as u8
    }
    /// Data Toggle, for reception transfers
    /// Access: read-write, Width: 1, Offset: 14
    /// Set Data Toggle, for reception transfers
    pub fn set_dtog_rx(value: bool) {
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Data Toggle, for reception transfers
    pub fn get_dtog_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// Correct transfer for reception
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Correct transfer for reception
    pub fn set_ctr_rx(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x4u32) as *mut u32, value) };
    }
    /// Get Correct transfer for reception
    pub fn get_ctr_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x4u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
}
/// endpoint 2 register
pub mod usb_ep2r {
    /// Endpoint address
    /// Access: read-write, Width: 4, Offset: 0
    /// Set Endpoint address
    pub fn set_ea(value: u8) {
        debug_assert!(value <= 0b1111, "set_ea out of range");
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Endpoint address
    pub fn get_ea() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
    /// Status bits, for transmission transfers
    /// Access: read-write, Width: 2, Offset: 4
    /// Set Status bits, for transmission transfers
    pub fn set_stat_tx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_tx out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Status bits, for transmission transfers
    pub fn get_stat_tx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x8u32) as *mut u32) };
        let value = value & (0b11 << 4);
        value as u8
    }
    /// Data Toggle, for transmission transfers
    /// Access: read-write, Width: 1, Offset: 6
    /// Set Data Toggle, for transmission transfers
    pub fn set_dtog_tx(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Data Toggle, for transmission transfers
    pub fn get_dtog_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Correct Transfer for transmission
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Correct Transfer for transmission
    pub fn set_ctr_tx(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Correct Transfer for transmission
    pub fn get_ctr_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Endpoint kind
    /// Access: read-write, Width: 1, Offset: 8
    /// Set Endpoint kind
    pub fn set_ep_kind(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Endpoint kind
    pub fn get_ep_kind() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Endpoint type
    /// Access: read-write, Width: 2, Offset: 9
    /// Set Endpoint type
    pub fn set_ep_type(value: u8) {
        debug_assert!(value <= 0b11, "set_ep_type out of range");
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Endpoint type
    pub fn get_ep_type() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x8u32) as *mut u32) };
        let value = value & (0b11 << 9);
        value as u8
    }
    /// Setup transaction completed
    /// Access: read-only, Width: 1, Offset: 11
    /// Get Setup transaction completed
    pub fn setup() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Status bits, for reception transfers
    /// Access: read-write, Width: 2, Offset: 12
    /// Set Status bits, for reception transfers
    pub fn set_stat_rx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_rx out of range");
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Status bits, for reception transfers
    pub fn get_stat_rx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x8u32) as *mut u32) };
        let value = value & (0b11 << 12);
        value as u8
    }
    /// Data Toggle, for reception transfers
    /// Access: read-write, Width: 1, Offset: 14
    /// Set Data Toggle, for reception transfers
    pub fn set_dtog_rx(value: bool) {
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Data Toggle, for reception transfers
    pub fn get_dtog_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// Correct transfer for reception
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Correct transfer for reception
    pub fn set_ctr_rx(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x8u32) as *mut u32, value) };
    }
    /// Get Correct transfer for reception
    pub fn get_ctr_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x8u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
}
/// endpoint 3 register
pub mod usb_ep3r {
    /// Endpoint address
    /// Access: read-write, Width: 4, Offset: 0
    /// Set Endpoint address
    pub fn set_ea(value: u8) {
        debug_assert!(value <= 0b1111, "set_ea out of range");
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Endpoint address
    pub fn get_ea() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
    /// Status bits, for transmission transfers
    /// Access: read-write, Width: 2, Offset: 4
    /// Set Status bits, for transmission transfers
    pub fn set_stat_tx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_tx out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Status bits, for transmission transfers
    pub fn get_stat_tx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0xCu32) as *mut u32) };
        let value = value & (0b11 << 4);
        value as u8
    }
    /// Data Toggle, for transmission transfers
    /// Access: read-write, Width: 1, Offset: 6
    /// Set Data Toggle, for transmission transfers
    pub fn set_dtog_tx(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Data Toggle, for transmission transfers
    pub fn get_dtog_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Correct Transfer for transmission
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Correct Transfer for transmission
    pub fn set_ctr_tx(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Correct Transfer for transmission
    pub fn get_ctr_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Endpoint kind
    /// Access: read-write, Width: 1, Offset: 8
    /// Set Endpoint kind
    pub fn set_ep_kind(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Endpoint kind
    pub fn get_ep_kind() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Endpoint type
    /// Access: read-write, Width: 2, Offset: 9
    /// Set Endpoint type
    pub fn set_ep_type(value: u8) {
        debug_assert!(value <= 0b11, "set_ep_type out of range");
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Endpoint type
    pub fn get_ep_type() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0xCu32) as *mut u32) };
        let value = value & (0b11 << 9);
        value as u8
    }
    /// Setup transaction completed
    /// Access: read-only, Width: 1, Offset: 11
    /// Get Setup transaction completed
    pub fn setup() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Status bits, for reception transfers
    /// Access: read-write, Width: 2, Offset: 12
    /// Set Status bits, for reception transfers
    pub fn set_stat_rx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_rx out of range");
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Status bits, for reception transfers
    pub fn get_stat_rx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0xCu32) as *mut u32) };
        let value = value & (0b11 << 12);
        value as u8
    }
    /// Data Toggle, for reception transfers
    /// Access: read-write, Width: 1, Offset: 14
    /// Set Data Toggle, for reception transfers
    pub fn set_dtog_rx(value: bool) {
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Data Toggle, for reception transfers
    pub fn get_dtog_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// Correct transfer for reception
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Correct transfer for reception
    pub fn set_ctr_rx(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0xCu32) as *mut u32, value) };
    }
    /// Get Correct transfer for reception
    pub fn get_ctr_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0xCu32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
}
/// endpoint 4 register
pub mod usb_ep4r {
    /// Endpoint address
    /// Access: read-write, Width: 4, Offset: 0
    /// Set Endpoint address
    pub fn set_ea(value: u8) {
        debug_assert!(value <= 0b1111, "set_ea out of range");
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Endpoint address
    pub fn get_ea() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x10u32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
    /// Status bits, for transmission transfers
    /// Access: read-write, Width: 2, Offset: 4
    /// Set Status bits, for transmission transfers
    pub fn set_stat_tx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_tx out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Status bits, for transmission transfers
    pub fn get_stat_tx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x10u32) as *mut u32) };
        let value = value & (0b11 << 4);
        value as u8
    }
    /// Data Toggle, for transmission transfers
    /// Access: read-write, Width: 1, Offset: 6
    /// Set Data Toggle, for transmission transfers
    pub fn set_dtog_tx(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Data Toggle, for transmission transfers
    pub fn get_dtog_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Correct Transfer for transmission
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Correct Transfer for transmission
    pub fn set_ctr_tx(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Correct Transfer for transmission
    pub fn get_ctr_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Endpoint kind
    /// Access: read-write, Width: 1, Offset: 8
    /// Set Endpoint kind
    pub fn set_ep_kind(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Endpoint kind
    pub fn get_ep_kind() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Endpoint type
    /// Access: read-write, Width: 2, Offset: 9
    /// Set Endpoint type
    pub fn set_ep_type(value: u8) {
        debug_assert!(value <= 0b11, "set_ep_type out of range");
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Endpoint type
    pub fn get_ep_type() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x10u32) as *mut u32) };
        let value = value & (0b11 << 9);
        value as u8
    }
    /// Setup transaction completed
    /// Access: read-only, Width: 1, Offset: 11
    /// Get Setup transaction completed
    pub fn setup() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Status bits, for reception transfers
    /// Access: read-write, Width: 2, Offset: 12
    /// Set Status bits, for reception transfers
    pub fn set_stat_rx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_rx out of range");
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Status bits, for reception transfers
    pub fn get_stat_rx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x10u32) as *mut u32) };
        let value = value & (0b11 << 12);
        value as u8
    }
    /// Data Toggle, for reception transfers
    /// Access: read-write, Width: 1, Offset: 14
    /// Set Data Toggle, for reception transfers
    pub fn set_dtog_rx(value: bool) {
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Data Toggle, for reception transfers
    pub fn get_dtog_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// Correct transfer for reception
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Correct transfer for reception
    pub fn set_ctr_rx(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x10u32) as *mut u32, value) };
    }
    /// Get Correct transfer for reception
    pub fn get_ctr_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x10u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
}
/// endpoint 5 register
pub mod usb_ep5r {
    /// Endpoint address
    /// Access: read-write, Width: 4, Offset: 0
    /// Set Endpoint address
    pub fn set_ea(value: u8) {
        debug_assert!(value <= 0b1111, "set_ea out of range");
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Endpoint address
    pub fn get_ea() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x14u32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
    /// Status bits, for transmission transfers
    /// Access: read-write, Width: 2, Offset: 4
    /// Set Status bits, for transmission transfers
    pub fn set_stat_tx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_tx out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Status bits, for transmission transfers
    pub fn get_stat_tx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x14u32) as *mut u32) };
        let value = value & (0b11 << 4);
        value as u8
    }
    /// Data Toggle, for transmission transfers
    /// Access: read-write, Width: 1, Offset: 6
    /// Set Data Toggle, for transmission transfers
    pub fn set_dtog_tx(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Data Toggle, for transmission transfers
    pub fn get_dtog_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x14u32) as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Correct Transfer for transmission
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Correct Transfer for transmission
    pub fn set_ctr_tx(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Correct Transfer for transmission
    pub fn get_ctr_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x14u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Endpoint kind
    /// Access: read-write, Width: 1, Offset: 8
    /// Set Endpoint kind
    pub fn set_ep_kind(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Endpoint kind
    pub fn get_ep_kind() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x14u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Endpoint type
    /// Access: read-write, Width: 2, Offset: 9
    /// Set Endpoint type
    pub fn set_ep_type(value: u8) {
        debug_assert!(value <= 0b11, "set_ep_type out of range");
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Endpoint type
    pub fn get_ep_type() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x14u32) as *mut u32) };
        let value = value & (0b11 << 9);
        value as u8
    }
    /// Setup transaction completed
    /// Access: read-only, Width: 1, Offset: 11
    /// Get Setup transaction completed
    pub fn setup() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x14u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Status bits, for reception transfers
    /// Access: read-write, Width: 2, Offset: 12
    /// Set Status bits, for reception transfers
    pub fn set_stat_rx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_rx out of range");
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Status bits, for reception transfers
    pub fn get_stat_rx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x14u32) as *mut u32) };
        let value = value & (0b11 << 12);
        value as u8
    }
    /// Data Toggle, for reception transfers
    /// Access: read-write, Width: 1, Offset: 14
    /// Set Data Toggle, for reception transfers
    pub fn set_dtog_rx(value: bool) {
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Data Toggle, for reception transfers
    pub fn get_dtog_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x14u32) as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// Correct transfer for reception
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Correct transfer for reception
    pub fn set_ctr_rx(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x14u32) as *mut u32, value) };
    }
    /// Get Correct transfer for reception
    pub fn get_ctr_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x14u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
}
/// endpoint 6 register
pub mod usb_ep6r {
    /// Endpoint address
    /// Access: read-write, Width: 4, Offset: 0
    /// Set Endpoint address
    pub fn set_ea(value: u8) {
        debug_assert!(value <= 0b1111, "set_ea out of range");
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Endpoint address
    pub fn get_ea() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
    /// Status bits, for transmission transfers
    /// Access: read-write, Width: 2, Offset: 4
    /// Set Status bits, for transmission transfers
    pub fn set_stat_tx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_tx out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Status bits, for transmission transfers
    pub fn get_stat_tx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x18u32) as *mut u32) };
        let value = value & (0b11 << 4);
        value as u8
    }
    /// Data Toggle, for transmission transfers
    /// Access: read-write, Width: 1, Offset: 6
    /// Set Data Toggle, for transmission transfers
    pub fn set_dtog_tx(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Data Toggle, for transmission transfers
    pub fn get_dtog_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Correct Transfer for transmission
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Correct Transfer for transmission
    pub fn set_ctr_tx(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Correct Transfer for transmission
    pub fn get_ctr_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Endpoint kind
    /// Access: read-write, Width: 1, Offset: 8
    /// Set Endpoint kind
    pub fn set_ep_kind(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Endpoint kind
    pub fn get_ep_kind() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Endpoint type
    /// Access: read-write, Width: 2, Offset: 9
    /// Set Endpoint type
    pub fn set_ep_type(value: u8) {
        debug_assert!(value <= 0b11, "set_ep_type out of range");
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Endpoint type
    pub fn get_ep_type() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x18u32) as *mut u32) };
        let value = value & (0b11 << 9);
        value as u8
    }
    /// Setup transaction completed
    /// Access: read-only, Width: 1, Offset: 11
    /// Get Setup transaction completed
    pub fn setup() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Status bits, for reception transfers
    /// Access: read-write, Width: 2, Offset: 12
    /// Set Status bits, for reception transfers
    pub fn set_stat_rx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_rx out of range");
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Status bits, for reception transfers
    pub fn get_stat_rx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x18u32) as *mut u32) };
        let value = value & (0b11 << 12);
        value as u8
    }
    /// Data Toggle, for reception transfers
    /// Access: read-write, Width: 1, Offset: 14
    /// Set Data Toggle, for reception transfers
    pub fn set_dtog_rx(value: bool) {
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Data Toggle, for reception transfers
    pub fn get_dtog_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// Correct transfer for reception
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Correct transfer for reception
    pub fn set_ctr_rx(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x18u32) as *mut u32, value) };
    }
    /// Get Correct transfer for reception
    pub fn get_ctr_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x18u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
}
/// endpoint 7 register
pub mod usb_ep7r {
    /// Endpoint address
    /// Access: read-write, Width: 4, Offset: 0
    /// Set Endpoint address
    pub fn set_ea(value: u8) {
        debug_assert!(value <= 0b1111, "set_ea out of range");
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Get Endpoint address
    pub fn get_ea() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
    /// Status bits, for transmission transfers
    /// Access: read-write, Width: 2, Offset: 4
    /// Set Status bits, for transmission transfers
    pub fn set_stat_tx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_tx out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Get Status bits, for transmission transfers
    pub fn get_stat_tx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b11 << 4);
        value as u8
    }
    /// Data Toggle, for transmission transfers
    /// Access: read-write, Width: 1, Offset: 6
    /// Set Data Toggle, for transmission transfers
    pub fn set_dtog_tx(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Get Data Toggle, for transmission transfers
    pub fn get_dtog_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Correct Transfer for transmission
    /// Access: read-write, Width: 1, Offset: 7
    /// Set Correct Transfer for transmission
    pub fn set_ctr_tx(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Get Correct Transfer for transmission
    pub fn get_ctr_tx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Endpoint kind
    /// Access: read-write, Width: 1, Offset: 8
    /// Set Endpoint kind
    pub fn set_ep_kind(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Get Endpoint kind
    pub fn get_ep_kind() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Endpoint type
    /// Access: read-write, Width: 2, Offset: 9
    /// Set Endpoint type
    pub fn set_ep_type(value: u8) {
        debug_assert!(value <= 0b11, "set_ep_type out of range");
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Get Endpoint type
    pub fn get_ep_type() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b11 << 9);
        value as u8
    }
    /// Setup transaction completed
    /// Access: read-only, Width: 1, Offset: 11
    /// Get Setup transaction completed
    pub fn setup() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Status bits, for reception transfers
    /// Access: read-write, Width: 2, Offset: 12
    /// Set Status bits, for reception transfers
    pub fn set_stat_rx(value: u8) {
        debug_assert!(value <= 0b11, "set_stat_rx out of range");
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Get Status bits, for reception transfers
    pub fn get_stat_rx() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b11 << 12);
        value as u8
    }
    /// Data Toggle, for reception transfers
    /// Access: read-write, Width: 1, Offset: 14
    /// Set Data Toggle, for reception transfers
    pub fn set_dtog_rx(value: bool) {
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Get Data Toggle, for reception transfers
    pub fn get_dtog_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// Correct transfer for reception
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Correct transfer for reception
    pub fn set_ctr_rx(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32, value) };
    }
    /// Get Correct transfer for reception
    pub fn get_ctr_rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
}
/// control register
pub mod usb_cntr {
    pub struct ReadonlyCache {
        /// Force USB Reset
        pub fres: bool,
        /// Power down
        pub pdwn: bool,
        /// Low-power mode
        pub lpmode: bool,
        /// Force suspend
        pub fsusp: bool,
        /// Resume request
        pub resume: bool,
        /// Expected start of frame interrupt mask
        pub esofm: bool,
        /// Start of frame interrupt mask
        pub sofm: bool,
        /// USB reset interrupt mask
        pub resetm: bool,
        /// Suspend mode interrupt mask
        pub suspm: bool,
        /// Wakeup interrupt mask
        pub wkupm: bool,
        /// Error interrupt mask
        pub errm: bool,
        /// Packet memory area over / underrun interrupt mask
        pub pmaovrm: bool,
        /// Correct transfer interrupt mask
        pub ctrm: bool,
    }
    pub struct Cache {
        /// Force USB Reset
        pub fres: bool,
        /// Power down
        pub pdwn: bool,
        /// Low-power mode
        pub lpmode: bool,
        /// Force suspend
        pub fsusp: bool,
        /// Resume request
        pub resume: bool,
        /// Expected start of frame interrupt mask
        pub esofm: bool,
        /// Start of frame interrupt mask
        pub sofm: bool,
        /// USB reset interrupt mask
        pub resetm: bool,
        /// Suspend mode interrupt mask
        pub suspm: bool,
        /// Wakeup interrupt mask
        pub wkupm: bool,
        /// Error interrupt mask
        pub errm: bool,
        /// Packet memory area over / underrun interrupt mask
        pub pmaovrm: bool,
        /// Correct transfer interrupt mask
        pub ctrm: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x40u32) as *mut u32) };
        ReadonlyCache {
            fres: ((value >> 0) & 0b1) > 0,
            pdwn: ((value >> 1) & 0b1) > 0,
            lpmode: ((value >> 2) & 0b1) > 0,
            fsusp: ((value >> 3) & 0b1) > 0,
            resume: ((value >> 4) & 0b1) > 0,
            esofm: ((value >> 8) & 0b1) > 0,
            sofm: ((value >> 9) & 0b1) > 0,
            resetm: ((value >> 10) & 0b1) > 0,
            suspm: ((value >> 11) & 0b1) > 0,
            wkupm: ((value >> 12) & 0b1) > 0,
            errm: ((value >> 13) & 0b1) > 0,
            pmaovrm: ((value >> 14) & 0b1) > 0,
            ctrm: ((value >> 15) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x40u32) as *mut u32) };
        Cache {
            fres: ((value >> 0) & 0b1) > 0,
            pdwn: ((value >> 1) & 0b1) > 0,
            lpmode: ((value >> 2) & 0b1) > 0,
            fsusp: ((value >> 3) & 0b1) > 0,
            resume: ((value >> 4) & 0b1) > 0,
            esofm: ((value >> 8) & 0b1) > 0,
            sofm: ((value >> 9) & 0b1) > 0,
            resetm: ((value >> 10) & 0b1) > 0,
            suspm: ((value >> 11) & 0b1) > 0,
            wkupm: ((value >> 12) & 0b1) > 0,
            errm: ((value >> 13) & 0b1) > 0,
            pmaovrm: ((value >> 14) & 0b1) > 0,
            ctrm: ((value >> 15) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.fres as u32) << 0)
                | ((self.pdwn as u32) << 1)
                | ((self.lpmode as u32) << 2)
                | ((self.fsusp as u32) << 3)
                | ((self.resume as u32) << 4)
                | ((self.esofm as u32) << 8)
                | ((self.sofm as u32) << 9)
                | ((self.resetm as u32) << 10)
                | ((self.suspm as u32) << 11)
                | ((self.wkupm as u32) << 12)
                | ((self.errm as u32) << 13)
                | ((self.pmaovrm as u32) << 14)
                | ((self.ctrm as u32) << 15)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x40u32) as *mut u32, value) };
        }
    }
}
/// interrupt status register
pub mod istr {
    /// Endpoint Identifier
    /// Access: read-only, Width: 4, Offset: 0
    /// Get Endpoint Identifier
    pub fn ep_id() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x44u32) as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
    /// Direction of transaction
    /// Access: read-only, Width: 1, Offset: 4
    /// Get Direction of transaction
    pub fn dir() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x44u32) as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// Expected start frame
    /// Access: read-write, Width: 1, Offset: 8
    /// Set Expected start frame
    pub fn set_esof(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x44u32) as *mut u32, value) };
    }
    /// Get Expected start frame
    pub fn get_esof() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x44u32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// start of frame
    /// Access: read-write, Width: 1, Offset: 9
    /// Set start of frame
    pub fn set_sof(value: bool) {
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x44u32) as *mut u32, value) };
    }
    /// Get start of frame
    pub fn get_sof() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x44u32) as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// reset request
    /// Access: read-write, Width: 1, Offset: 10
    /// Set reset request
    pub fn set_reset(value: bool) {
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x44u32) as *mut u32, value) };
    }
    /// Get reset request
    pub fn get_reset() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x44u32) as *mut u32) };
        let value = value & (0b1 << 10);
        value > 0
    }
    /// Suspend mode request
    /// Access: read-write, Width: 1, Offset: 11
    /// Set Suspend mode request
    pub fn set_susp(value: bool) {
        let value = value as u32;
        let value = value << 11;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x44u32) as *mut u32, value) };
    }
    /// Get Suspend mode request
    pub fn get_susp() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x44u32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Wakeup
    /// Access: read-write, Width: 1, Offset: 12
    /// Set Wakeup
    pub fn set_wkup(value: bool) {
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x44u32) as *mut u32, value) };
    }
    /// Get Wakeup
    pub fn get_wkup() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x44u32) as *mut u32) };
        let value = value & (0b1 << 12);
        value > 0
    }
    /// Error
    /// Access: read-write, Width: 1, Offset: 13
    /// Set Error
    pub fn set_err(value: bool) {
        let value = value as u32;
        let value = value << 13;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x44u32) as *mut u32, value) };
    }
    /// Get Error
    pub fn get_err() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x44u32) as *mut u32) };
        let value = value & (0b1 << 13);
        value > 0
    }
    /// Packet memory area over / underrun
    /// Access: read-write, Width: 1, Offset: 14
    /// Set Packet memory area over / underrun
    pub fn set_pmaovr(value: bool) {
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x44u32) as *mut u32, value) };
    }
    /// Get Packet memory area over / underrun
    pub fn get_pmaovr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x44u32) as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// Correct transfer
    /// Access: read-only, Width: 1, Offset: 15
    /// Get Correct transfer
    pub fn ctr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x44u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
}
/// frame number register
pub mod fnr {
    /// Frame number
    /// Access: read-only, Width: 11, Offset: 0
    /// Get Frame number
    pub fn fnr_fn() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x48u32) as *mut u32) };
        let value = value & (0b11111111111 << 0);
        value as u16
    }
    /// Lost SOF
    /// Access: read-only, Width: 2, Offset: 11
    /// Get Lost SOF
    pub fn lsof() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x48u32) as *mut u32) };
        let value = value & (0b11 << 11);
        value as u8
    }
    /// Locked
    /// Access: read-only, Width: 1, Offset: 13
    /// Get Locked
    pub fn lck() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x48u32) as *mut u32) };
        let value = value & (0b1 << 13);
        value > 0
    }
    /// Receive data - line status
    /// Access: read-only, Width: 1, Offset: 14
    /// Get Receive data - line status
    pub fn rxdm() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x48u32) as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// Receive data + line status
    /// Access: read-only, Width: 1, Offset: 15
    /// Get Receive data + line status
    pub fn rxdp() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x48u32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
}
/// device address
pub mod daddr {
    pub struct ReadonlyCache {
        /// Device address
        pub add: bool,
        /// Device address
        pub add1: bool,
        /// Device address
        pub add2: bool,
        /// Device address
        pub add3: bool,
        /// Device address
        pub add4: bool,
        /// Device address
        pub add5: bool,
        /// Device address
        pub add6: bool,
        /// Enable function
        pub ef: bool,
    }
    pub struct Cache {
        /// Device address
        pub add: bool,
        /// Device address
        pub add1: bool,
        /// Device address
        pub add2: bool,
        /// Device address
        pub add3: bool,
        /// Device address
        pub add4: bool,
        /// Device address
        pub add5: bool,
        /// Device address
        pub add6: bool,
        /// Enable function
        pub ef: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x4Cu32) as *mut u32) };
        ReadonlyCache {
            add: ((value >> 0) & 0b1) > 0,
            add1: ((value >> 1) & 0b1) > 0,
            add2: ((value >> 2) & 0b1) > 0,
            add3: ((value >> 3) & 0b1) > 0,
            add4: ((value >> 4) & 0b1) > 0,
            add5: ((value >> 5) & 0b1) > 0,
            add6: ((value >> 6) & 0b1) > 0,
            ef: ((value >> 7) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x4Cu32) as *mut u32) };
        Cache {
            add: ((value >> 0) & 0b1) > 0,
            add1: ((value >> 1) & 0b1) > 0,
            add2: ((value >> 2) & 0b1) > 0,
            add3: ((value >> 3) & 0b1) > 0,
            add4: ((value >> 4) & 0b1) > 0,
            add5: ((value >> 5) & 0b1) > 0,
            add6: ((value >> 6) & 0b1) > 0,
            ef: ((value >> 7) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.add as u32) << 0)
                | ((self.add1 as u32) << 1)
                | ((self.add2 as u32) << 2)
                | ((self.add3 as u32) << 3)
                | ((self.add4 as u32) << 4)
                | ((self.add5 as u32) << 5)
                | ((self.add6 as u32) << 6)
                | ((self.ef as u32) << 7)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x4Cu32) as *mut u32, value) };
        }
    }
}
/// Buffer table address
pub mod btable {
    pub struct ReadonlyCache {
        /// Buffer table
        pub btable: u16,
    }
    pub struct Cache {
        /// Buffer table
        pub btable: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x50u32) as *mut u32) };
        ReadonlyCache {
            btable: ((value >> 3) & 0b1111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005C00u32 + 0x50u32) as *mut u32) };
        Cache {
            btable: ((value >> 3) & 0b1111111111111) as u16,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.btable as u32) << 3)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005C00u32 + 0x50u32) as *mut u32, value) };
        }
    }
}
/// USB wakeup from Suspend
pub const INTERRUPT_USB_WKUP: u32 = 42;
/// USB High priority interrupt
pub const INTERRUPT_USB_HP: u32 = 74;
/// USB Low priority interrupt
pub const INTERRUPT_USB_LP: u32 = 75;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40005C00</baseAddress>
  <description>
                Universal serial bus full-speed device
                interface
            </description>
  <groupName>USB_FS</groupName>
  <interrupt>
    <description>USB wakeup from Suspend</description>
    <name>USB_WKUP</name>
    <value>42</value>
  </interrupt>
  <interrupt>
    <description>USB High priority interrupt</description>
    <name>USB_HP</name>
    <value>74</value>
  </interrupt>
  <interrupt>
    <description>USB Low priority interrupt</description>
    <name>USB_LP</name>
    <value>75</value>
  </interrupt>
  <name>USB_FS</name>
  <registers>
    <register>
      <addressOffset>0x0</addressOffset>
      <description>endpoint 0 register</description>
      <displayName>USB_EP0R</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for transmission
                                transfers
                            </description>
          <name>STAT_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for transmission
                                transfers
                            </description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct Transfer for
                                transmission
                            </description>
          <name>CTR_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Setup transaction
                                completed
                            </description>
          <name>SETUP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for reception
                                transfers
                            </description>
          <name>STAT_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for reception
                                transfers
                            </description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct transfer for
                                reception
                            </description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>USB_EP0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x4</addressOffset>
      <description>endpoint 1 register</description>
      <displayName>USB_EP1R</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for transmission
                                transfers
                            </description>
          <name>STAT_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for transmission
                                transfers
                            </description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct Transfer for
                                transmission
                            </description>
          <name>CTR_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Setup transaction
                                completed
                            </description>
          <name>SETUP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for reception
                                transfers
                            </description>
          <name>STAT_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for reception
                                transfers
                            </description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct transfer for
                                reception
                            </description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>USB_EP1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x8</addressOffset>
      <description>endpoint 2 register</description>
      <displayName>USB_EP2R</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for transmission
                                transfers
                            </description>
          <name>STAT_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for transmission
                                transfers
                            </description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct Transfer for
                                transmission
                            </description>
          <name>CTR_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Setup transaction
                                completed
                            </description>
          <name>SETUP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for reception
                                transfers
                            </description>
          <name>STAT_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for reception
                                transfers
                            </description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct transfer for
                                reception
                            </description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>USB_EP2R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0xC</addressOffset>
      <description>endpoint 3 register</description>
      <displayName>USB_EP3R</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for transmission
                                transfers
                            </description>
          <name>STAT_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for transmission
                                transfers
                            </description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct Transfer for
                                transmission
                            </description>
          <name>CTR_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Setup transaction
                                completed
                            </description>
          <name>SETUP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for reception
                                transfers
                            </description>
          <name>STAT_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for reception
                                transfers
                            </description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct transfer for
                                reception
                            </description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>USB_EP3R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x10</addressOffset>
      <description>endpoint 4 register</description>
      <displayName>USB_EP4R</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for transmission
                                transfers
                            </description>
          <name>STAT_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for transmission
                                transfers
                            </description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct Transfer for
                                transmission
                            </description>
          <name>CTR_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Setup transaction
                                completed
                            </description>
          <name>SETUP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for reception
                                transfers
                            </description>
          <name>STAT_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for reception
                                transfers
                            </description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct transfer for
                                reception
                            </description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>USB_EP4R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x14</addressOffset>
      <description>endpoint 5 register</description>
      <displayName>USB_EP5R</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for transmission
                                transfers
                            </description>
          <name>STAT_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for transmission
                                transfers
                            </description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct Transfer for
                                transmission
                            </description>
          <name>CTR_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Setup transaction
                                completed
                            </description>
          <name>SETUP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for reception
                                transfers
                            </description>
          <name>STAT_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for reception
                                transfers
                            </description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct transfer for
                                reception
                            </description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>USB_EP5R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x18</addressOffset>
      <description>endpoint 6 register</description>
      <displayName>USB_EP6R</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for transmission
                                transfers
                            </description>
          <name>STAT_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for transmission
                                transfers
                            </description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct Transfer for
                                transmission
                            </description>
          <name>CTR_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Setup transaction
                                completed
                            </description>
          <name>SETUP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for reception
                                transfers
                            </description>
          <name>STAT_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for reception
                                transfers
                            </description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct transfer for
                                reception
                            </description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>USB_EP6R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x1C</addressOffset>
      <description>endpoint 7 register</description>
      <displayName>USB_EP7R</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for transmission
                                transfers
                            </description>
          <name>STAT_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for transmission
                                transfers
                            </description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct Transfer for
                                transmission
                            </description>
          <name>CTR_TX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Setup transaction
                                completed
                            </description>
          <name>SETUP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Status bits, for reception
                                transfers
                            </description>
          <name>STAT_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Data Toggle, for reception
                                transfers
                            </description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct transfer for
                                reception
                            </description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>USB_EP7R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x40</addressOffset>
      <description>control register</description>
      <displayName>USB_CNTR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Force USB Reset</description>
          <name>FRES</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Power down</description>
          <name>PDWN</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Low-power mode</description>
          <name>LPMODE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Force suspend</description>
          <name>FSUSP</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Resume request</description>
          <name>RESUME</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Expected start of frame interrupt
                                mask
                            </description>
          <name>ESOFM</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Start of frame interrupt
                                mask
                            </description>
          <name>SOFM</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USB reset interrupt mask</description>
          <name>RESETM</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Suspend mode interrupt
                                mask
                            </description>
          <name>SUSPM</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Wakeup interrupt mask</description>
          <name>WKUPM</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Error interrupt mask</description>
          <name>ERRM</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Packet memory area over / underrun
                                interrupt mask
                            </description>
          <name>PMAOVRM</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Correct transfer interrupt
                                mask
                            </description>
          <name>CTRM</name>
        </field>
      </fields>
      <name>USB_CNTR</name>
      <resetValue>0x00000003</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x44</addressOffset>
      <description>interrupt status register</description>
      <displayName>ISTR</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint Identifier</description>
          <name>EP_ID</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Direction of transaction</description>
          <name>DIR</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Expected start frame</description>
          <name>ESOF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>start of frame</description>
          <name>SOF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>reset request</description>
          <name>RESET</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Suspend mode request</description>
          <name>SUSP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Wakeup</description>
          <name>WKUP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Error</description>
          <name>ERR</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Packet memory area over /
                                underrun
                            </description>
          <name>PMAOVR</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct transfer</description>
          <name>CTR</name>
        </field>
      </fields>
      <name>ISTR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x48</addressOffset>
      <description>frame number register</description>
      <displayName>FNR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>11</bitWidth>
          <description>Frame number</description>
          <name>FN</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Lost SOF</description>
          <name>LSOF</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Locked</description>
          <name>LCK</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receive data - line status</description>
          <name>RXDM</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receive data + line status</description>
          <name>RXDP</name>
        </field>
      </fields>
      <name>FNR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4C</addressOffset>
      <description>device address</description>
      <displayName>DADDR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Device address</description>
          <name>ADD</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Device address</description>
          <name>ADD1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Device address</description>
          <name>ADD2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Device address</description>
          <name>ADD3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Device address</description>
          <name>ADD4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Device address</description>
          <name>ADD5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Device address</description>
          <name>ADD6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Enable function</description>
          <name>EF</name>
        </field>
      </fields>
      <name>DADDR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x50</addressOffset>
      <description>Buffer table address</description>
      <displayName>BTABLE</displayName>
      <fields>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>13</bitWidth>
          <description>Buffer table</description>
          <name>BTABLE</name>
        </field>
      </fields>
      <name>BTABLE</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
