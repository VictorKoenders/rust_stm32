/// MOD USB_FS
/// Universal serial bus full-speed device interface
const BASE_ADDRESS: u32 = 0x40005C00;
/// endpoint 0 register
/// Size: 0x20 bits
pub mod usb_ep0r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EA_BIT_OFFSET: u8 = 0;
	const EA_BIT_WIDTH: u8 = 4;
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn get_ea() -> u8 { ::read(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH) as u8 }
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn set_ea(value: u8) { ::write(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH, value as u32); }

	const STAT_TX_BIT_OFFSET: u8 = 4;
	const STAT_TX_BIT_WIDTH: u8 = 2;
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn get_stat_tx() -> u8 { ::read(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH) as u8 }
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn set_stat_tx(value: u8) { ::write(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH, value as u32); }

	const DTOG_TX_BIT_OFFSET: u8 = 6;
	const DTOG_TX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn get_dtog_tx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH) as u8 }
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn set_dtog_tx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH, value as u32); }

	const CTR_TX_BIT_OFFSET: u8 = 7;
	const CTR_TX_BIT_WIDTH: u8 = 1;
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn get_ctr_tx() -> u8 { ::read(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH) as u8 }
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn set_ctr_tx(value: u8) { ::write(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH, value as u32); }

	const EP_KIND_BIT_OFFSET: u8 = 8;
	const EP_KIND_BIT_WIDTH: u8 = 1;
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn get_ep_kind() -> u8 { ::read(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH) as u8 }
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn set_ep_kind(value: u8) { ::write(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH, value as u32); }

	const EP_TYPE_BIT_OFFSET: u8 = 9;
	const EP_TYPE_BIT_WIDTH: u8 = 2;
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn get_ep_type() -> u8 { ::read(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH) as u8 }
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn set_ep_type(value: u8) { ::write(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH, value as u32); }

	const SETUP_BIT_OFFSET: u8 = 11;
	const SETUP_BIT_WIDTH: u8 = 1;
	/// Setup transaction completed (Width: 1, Offset: 11)
	pub fn get_setup() -> u8 { ::read(REGISTER_ADDRESS, SETUP_BIT_OFFSET, SETUP_BIT_WIDTH) as u8 }

	const STAT_RX_BIT_OFFSET: u8 = 12;
	const STAT_RX_BIT_WIDTH: u8 = 2;
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn get_stat_rx() -> u8 { ::read(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH) as u8 }
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn set_stat_rx(value: u8) { ::write(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH, value as u32); }

	const DTOG_RX_BIT_OFFSET: u8 = 14;
	const DTOG_RX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn get_dtog_rx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH) as u8 }
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn set_dtog_rx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH, value as u32); }

	const CTR_RX_BIT_OFFSET: u8 = 15;
	const CTR_RX_BIT_WIDTH: u8 = 1;
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn get_ctr_rx() -> u8 { ::read(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH) as u8 }
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn set_ctr_rx(value: u8) { ::write(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH, value as u32); }
}
/// endpoint 1 register
/// Size: 0x20 bits
pub mod usb_ep1r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EA_BIT_OFFSET: u8 = 0;
	const EA_BIT_WIDTH: u8 = 4;
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn get_ea() -> u8 { ::read(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH) as u8 }
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn set_ea(value: u8) { ::write(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH, value as u32); }

	const STAT_TX_BIT_OFFSET: u8 = 4;
	const STAT_TX_BIT_WIDTH: u8 = 2;
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn get_stat_tx() -> u8 { ::read(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH) as u8 }
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn set_stat_tx(value: u8) { ::write(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH, value as u32); }

	const DTOG_TX_BIT_OFFSET: u8 = 6;
	const DTOG_TX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn get_dtog_tx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH) as u8 }
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn set_dtog_tx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH, value as u32); }

	const CTR_TX_BIT_OFFSET: u8 = 7;
	const CTR_TX_BIT_WIDTH: u8 = 1;
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn get_ctr_tx() -> u8 { ::read(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH) as u8 }
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn set_ctr_tx(value: u8) { ::write(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH, value as u32); }

	const EP_KIND_BIT_OFFSET: u8 = 8;
	const EP_KIND_BIT_WIDTH: u8 = 1;
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn get_ep_kind() -> u8 { ::read(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH) as u8 }
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn set_ep_kind(value: u8) { ::write(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH, value as u32); }

	const EP_TYPE_BIT_OFFSET: u8 = 9;
	const EP_TYPE_BIT_WIDTH: u8 = 2;
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn get_ep_type() -> u8 { ::read(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH) as u8 }
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn set_ep_type(value: u8) { ::write(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH, value as u32); }

	const SETUP_BIT_OFFSET: u8 = 11;
	const SETUP_BIT_WIDTH: u8 = 1;
	/// Setup transaction completed (Width: 1, Offset: 11)
	pub fn get_setup() -> u8 { ::read(REGISTER_ADDRESS, SETUP_BIT_OFFSET, SETUP_BIT_WIDTH) as u8 }

	const STAT_RX_BIT_OFFSET: u8 = 12;
	const STAT_RX_BIT_WIDTH: u8 = 2;
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn get_stat_rx() -> u8 { ::read(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH) as u8 }
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn set_stat_rx(value: u8) { ::write(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH, value as u32); }

	const DTOG_RX_BIT_OFFSET: u8 = 14;
	const DTOG_RX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn get_dtog_rx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH) as u8 }
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn set_dtog_rx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH, value as u32); }

	const CTR_RX_BIT_OFFSET: u8 = 15;
	const CTR_RX_BIT_WIDTH: u8 = 1;
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn get_ctr_rx() -> u8 { ::read(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH) as u8 }
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn set_ctr_rx(value: u8) { ::write(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH, value as u32); }
}
/// endpoint 2 register
/// Size: 0x20 bits
pub mod usb_ep2r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EA_BIT_OFFSET: u8 = 0;
	const EA_BIT_WIDTH: u8 = 4;
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn get_ea() -> u8 { ::read(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH) as u8 }
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn set_ea(value: u8) { ::write(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH, value as u32); }

	const STAT_TX_BIT_OFFSET: u8 = 4;
	const STAT_TX_BIT_WIDTH: u8 = 2;
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn get_stat_tx() -> u8 { ::read(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH) as u8 }
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn set_stat_tx(value: u8) { ::write(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH, value as u32); }

	const DTOG_TX_BIT_OFFSET: u8 = 6;
	const DTOG_TX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn get_dtog_tx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH) as u8 }
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn set_dtog_tx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH, value as u32); }

	const CTR_TX_BIT_OFFSET: u8 = 7;
	const CTR_TX_BIT_WIDTH: u8 = 1;
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn get_ctr_tx() -> u8 { ::read(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH) as u8 }
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn set_ctr_tx(value: u8) { ::write(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH, value as u32); }

	const EP_KIND_BIT_OFFSET: u8 = 8;
	const EP_KIND_BIT_WIDTH: u8 = 1;
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn get_ep_kind() -> u8 { ::read(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH) as u8 }
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn set_ep_kind(value: u8) { ::write(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH, value as u32); }

	const EP_TYPE_BIT_OFFSET: u8 = 9;
	const EP_TYPE_BIT_WIDTH: u8 = 2;
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn get_ep_type() -> u8 { ::read(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH) as u8 }
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn set_ep_type(value: u8) { ::write(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH, value as u32); }

	const SETUP_BIT_OFFSET: u8 = 11;
	const SETUP_BIT_WIDTH: u8 = 1;
	/// Setup transaction completed (Width: 1, Offset: 11)
	pub fn get_setup() -> u8 { ::read(REGISTER_ADDRESS, SETUP_BIT_OFFSET, SETUP_BIT_WIDTH) as u8 }

	const STAT_RX_BIT_OFFSET: u8 = 12;
	const STAT_RX_BIT_WIDTH: u8 = 2;
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn get_stat_rx() -> u8 { ::read(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH) as u8 }
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn set_stat_rx(value: u8) { ::write(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH, value as u32); }

	const DTOG_RX_BIT_OFFSET: u8 = 14;
	const DTOG_RX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn get_dtog_rx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH) as u8 }
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn set_dtog_rx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH, value as u32); }

	const CTR_RX_BIT_OFFSET: u8 = 15;
	const CTR_RX_BIT_WIDTH: u8 = 1;
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn get_ctr_rx() -> u8 { ::read(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH) as u8 }
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn set_ctr_rx(value: u8) { ::write(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH, value as u32); }
}
/// endpoint 3 register
/// Size: 0x20 bits
pub mod usb_ep3r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EA_BIT_OFFSET: u8 = 0;
	const EA_BIT_WIDTH: u8 = 4;
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn get_ea() -> u8 { ::read(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH) as u8 }
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn set_ea(value: u8) { ::write(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH, value as u32); }

	const STAT_TX_BIT_OFFSET: u8 = 4;
	const STAT_TX_BIT_WIDTH: u8 = 2;
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn get_stat_tx() -> u8 { ::read(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH) as u8 }
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn set_stat_tx(value: u8) { ::write(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH, value as u32); }

	const DTOG_TX_BIT_OFFSET: u8 = 6;
	const DTOG_TX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn get_dtog_tx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH) as u8 }
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn set_dtog_tx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH, value as u32); }

	const CTR_TX_BIT_OFFSET: u8 = 7;
	const CTR_TX_BIT_WIDTH: u8 = 1;
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn get_ctr_tx() -> u8 { ::read(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH) as u8 }
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn set_ctr_tx(value: u8) { ::write(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH, value as u32); }

	const EP_KIND_BIT_OFFSET: u8 = 8;
	const EP_KIND_BIT_WIDTH: u8 = 1;
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn get_ep_kind() -> u8 { ::read(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH) as u8 }
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn set_ep_kind(value: u8) { ::write(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH, value as u32); }

	const EP_TYPE_BIT_OFFSET: u8 = 9;
	const EP_TYPE_BIT_WIDTH: u8 = 2;
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn get_ep_type() -> u8 { ::read(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH) as u8 }
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn set_ep_type(value: u8) { ::write(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH, value as u32); }

	const SETUP_BIT_OFFSET: u8 = 11;
	const SETUP_BIT_WIDTH: u8 = 1;
	/// Setup transaction completed (Width: 1, Offset: 11)
	pub fn get_setup() -> u8 { ::read(REGISTER_ADDRESS, SETUP_BIT_OFFSET, SETUP_BIT_WIDTH) as u8 }

	const STAT_RX_BIT_OFFSET: u8 = 12;
	const STAT_RX_BIT_WIDTH: u8 = 2;
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn get_stat_rx() -> u8 { ::read(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH) as u8 }
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn set_stat_rx(value: u8) { ::write(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH, value as u32); }

	const DTOG_RX_BIT_OFFSET: u8 = 14;
	const DTOG_RX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn get_dtog_rx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH) as u8 }
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn set_dtog_rx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH, value as u32); }

	const CTR_RX_BIT_OFFSET: u8 = 15;
	const CTR_RX_BIT_WIDTH: u8 = 1;
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn get_ctr_rx() -> u8 { ::read(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH) as u8 }
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn set_ctr_rx(value: u8) { ::write(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH, value as u32); }
}
/// endpoint 4 register
/// Size: 0x20 bits
pub mod usb_ep4r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EA_BIT_OFFSET: u8 = 0;
	const EA_BIT_WIDTH: u8 = 4;
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn get_ea() -> u8 { ::read(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH) as u8 }
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn set_ea(value: u8) { ::write(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH, value as u32); }

	const STAT_TX_BIT_OFFSET: u8 = 4;
	const STAT_TX_BIT_WIDTH: u8 = 2;
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn get_stat_tx() -> u8 { ::read(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH) as u8 }
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn set_stat_tx(value: u8) { ::write(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH, value as u32); }

	const DTOG_TX_BIT_OFFSET: u8 = 6;
	const DTOG_TX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn get_dtog_tx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH) as u8 }
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn set_dtog_tx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH, value as u32); }

	const CTR_TX_BIT_OFFSET: u8 = 7;
	const CTR_TX_BIT_WIDTH: u8 = 1;
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn get_ctr_tx() -> u8 { ::read(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH) as u8 }
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn set_ctr_tx(value: u8) { ::write(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH, value as u32); }

	const EP_KIND_BIT_OFFSET: u8 = 8;
	const EP_KIND_BIT_WIDTH: u8 = 1;
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn get_ep_kind() -> u8 { ::read(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH) as u8 }
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn set_ep_kind(value: u8) { ::write(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH, value as u32); }

	const EP_TYPE_BIT_OFFSET: u8 = 9;
	const EP_TYPE_BIT_WIDTH: u8 = 2;
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn get_ep_type() -> u8 { ::read(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH) as u8 }
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn set_ep_type(value: u8) { ::write(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH, value as u32); }

	const SETUP_BIT_OFFSET: u8 = 11;
	const SETUP_BIT_WIDTH: u8 = 1;
	/// Setup transaction completed (Width: 1, Offset: 11)
	pub fn get_setup() -> u8 { ::read(REGISTER_ADDRESS, SETUP_BIT_OFFSET, SETUP_BIT_WIDTH) as u8 }

	const STAT_RX_BIT_OFFSET: u8 = 12;
	const STAT_RX_BIT_WIDTH: u8 = 2;
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn get_stat_rx() -> u8 { ::read(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH) as u8 }
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn set_stat_rx(value: u8) { ::write(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH, value as u32); }

	const DTOG_RX_BIT_OFFSET: u8 = 14;
	const DTOG_RX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn get_dtog_rx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH) as u8 }
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn set_dtog_rx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH, value as u32); }

	const CTR_RX_BIT_OFFSET: u8 = 15;
	const CTR_RX_BIT_WIDTH: u8 = 1;
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn get_ctr_rx() -> u8 { ::read(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH) as u8 }
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn set_ctr_rx(value: u8) { ::write(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH, value as u32); }
}
/// endpoint 5 register
/// Size: 0x20 bits
pub mod usb_ep5r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EA_BIT_OFFSET: u8 = 0;
	const EA_BIT_WIDTH: u8 = 4;
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn get_ea() -> u8 { ::read(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH) as u8 }
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn set_ea(value: u8) { ::write(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH, value as u32); }

	const STAT_TX_BIT_OFFSET: u8 = 4;
	const STAT_TX_BIT_WIDTH: u8 = 2;
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn get_stat_tx() -> u8 { ::read(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH) as u8 }
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn set_stat_tx(value: u8) { ::write(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH, value as u32); }

	const DTOG_TX_BIT_OFFSET: u8 = 6;
	const DTOG_TX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn get_dtog_tx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH) as u8 }
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn set_dtog_tx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH, value as u32); }

	const CTR_TX_BIT_OFFSET: u8 = 7;
	const CTR_TX_BIT_WIDTH: u8 = 1;
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn get_ctr_tx() -> u8 { ::read(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH) as u8 }
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn set_ctr_tx(value: u8) { ::write(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH, value as u32); }

	const EP_KIND_BIT_OFFSET: u8 = 8;
	const EP_KIND_BIT_WIDTH: u8 = 1;
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn get_ep_kind() -> u8 { ::read(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH) as u8 }
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn set_ep_kind(value: u8) { ::write(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH, value as u32); }

	const EP_TYPE_BIT_OFFSET: u8 = 9;
	const EP_TYPE_BIT_WIDTH: u8 = 2;
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn get_ep_type() -> u8 { ::read(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH) as u8 }
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn set_ep_type(value: u8) { ::write(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH, value as u32); }

	const SETUP_BIT_OFFSET: u8 = 11;
	const SETUP_BIT_WIDTH: u8 = 1;
	/// Setup transaction completed (Width: 1, Offset: 11)
	pub fn get_setup() -> u8 { ::read(REGISTER_ADDRESS, SETUP_BIT_OFFSET, SETUP_BIT_WIDTH) as u8 }

	const STAT_RX_BIT_OFFSET: u8 = 12;
	const STAT_RX_BIT_WIDTH: u8 = 2;
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn get_stat_rx() -> u8 { ::read(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH) as u8 }
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn set_stat_rx(value: u8) { ::write(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH, value as u32); }

	const DTOG_RX_BIT_OFFSET: u8 = 14;
	const DTOG_RX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn get_dtog_rx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH) as u8 }
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn set_dtog_rx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH, value as u32); }

	const CTR_RX_BIT_OFFSET: u8 = 15;
	const CTR_RX_BIT_WIDTH: u8 = 1;
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn get_ctr_rx() -> u8 { ::read(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH) as u8 }
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn set_ctr_rx(value: u8) { ::write(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH, value as u32); }
}
/// endpoint 6 register
/// Size: 0x20 bits
pub mod usb_ep6r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EA_BIT_OFFSET: u8 = 0;
	const EA_BIT_WIDTH: u8 = 4;
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn get_ea() -> u8 { ::read(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH) as u8 }
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn set_ea(value: u8) { ::write(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH, value as u32); }

	const STAT_TX_BIT_OFFSET: u8 = 4;
	const STAT_TX_BIT_WIDTH: u8 = 2;
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn get_stat_tx() -> u8 { ::read(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH) as u8 }
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn set_stat_tx(value: u8) { ::write(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH, value as u32); }

	const DTOG_TX_BIT_OFFSET: u8 = 6;
	const DTOG_TX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn get_dtog_tx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH) as u8 }
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn set_dtog_tx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH, value as u32); }

	const CTR_TX_BIT_OFFSET: u8 = 7;
	const CTR_TX_BIT_WIDTH: u8 = 1;
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn get_ctr_tx() -> u8 { ::read(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH) as u8 }
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn set_ctr_tx(value: u8) { ::write(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH, value as u32); }

	const EP_KIND_BIT_OFFSET: u8 = 8;
	const EP_KIND_BIT_WIDTH: u8 = 1;
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn get_ep_kind() -> u8 { ::read(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH) as u8 }
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn set_ep_kind(value: u8) { ::write(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH, value as u32); }

	const EP_TYPE_BIT_OFFSET: u8 = 9;
	const EP_TYPE_BIT_WIDTH: u8 = 2;
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn get_ep_type() -> u8 { ::read(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH) as u8 }
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn set_ep_type(value: u8) { ::write(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH, value as u32); }

	const SETUP_BIT_OFFSET: u8 = 11;
	const SETUP_BIT_WIDTH: u8 = 1;
	/// Setup transaction completed (Width: 1, Offset: 11)
	pub fn get_setup() -> u8 { ::read(REGISTER_ADDRESS, SETUP_BIT_OFFSET, SETUP_BIT_WIDTH) as u8 }

	const STAT_RX_BIT_OFFSET: u8 = 12;
	const STAT_RX_BIT_WIDTH: u8 = 2;
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn get_stat_rx() -> u8 { ::read(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH) as u8 }
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn set_stat_rx(value: u8) { ::write(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH, value as u32); }

	const DTOG_RX_BIT_OFFSET: u8 = 14;
	const DTOG_RX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn get_dtog_rx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH) as u8 }
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn set_dtog_rx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH, value as u32); }

	const CTR_RX_BIT_OFFSET: u8 = 15;
	const CTR_RX_BIT_WIDTH: u8 = 1;
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn get_ctr_rx() -> u8 { ::read(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH) as u8 }
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn set_ctr_rx(value: u8) { ::write(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH, value as u32); }
}
/// endpoint 7 register
/// Size: 0x20 bits
pub mod usb_ep7r {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EA_BIT_OFFSET: u8 = 0;
	const EA_BIT_WIDTH: u8 = 4;
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn get_ea() -> u8 { ::read(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH) as u8 }
	/// Endpoint address (Width: 4, Offset: 0)
	pub fn set_ea(value: u8) { ::write(REGISTER_ADDRESS, EA_BIT_OFFSET, EA_BIT_WIDTH, value as u32); }

	const STAT_TX_BIT_OFFSET: u8 = 4;
	const STAT_TX_BIT_WIDTH: u8 = 2;
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn get_stat_tx() -> u8 { ::read(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH) as u8 }
	/// Status bits, for transmission transfers (Width: 2, Offset: 4)
	pub fn set_stat_tx(value: u8) { ::write(REGISTER_ADDRESS, STAT_TX_BIT_OFFSET, STAT_TX_BIT_WIDTH, value as u32); }

	const DTOG_TX_BIT_OFFSET: u8 = 6;
	const DTOG_TX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn get_dtog_tx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH) as u8 }
	/// Data Toggle, for transmission transfers (Width: 1, Offset: 6)
	pub fn set_dtog_tx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_TX_BIT_OFFSET, DTOG_TX_BIT_WIDTH, value as u32); }

	const CTR_TX_BIT_OFFSET: u8 = 7;
	const CTR_TX_BIT_WIDTH: u8 = 1;
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn get_ctr_tx() -> u8 { ::read(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH) as u8 }
	/// Correct Transfer for transmission (Width: 1, Offset: 7)
	pub fn set_ctr_tx(value: u8) { ::write(REGISTER_ADDRESS, CTR_TX_BIT_OFFSET, CTR_TX_BIT_WIDTH, value as u32); }

	const EP_KIND_BIT_OFFSET: u8 = 8;
	const EP_KIND_BIT_WIDTH: u8 = 1;
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn get_ep_kind() -> u8 { ::read(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH) as u8 }
	/// Endpoint kind (Width: 1, Offset: 8)
	pub fn set_ep_kind(value: u8) { ::write(REGISTER_ADDRESS, EP_KIND_BIT_OFFSET, EP_KIND_BIT_WIDTH, value as u32); }

	const EP_TYPE_BIT_OFFSET: u8 = 9;
	const EP_TYPE_BIT_WIDTH: u8 = 2;
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn get_ep_type() -> u8 { ::read(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH) as u8 }
	/// Endpoint type (Width: 2, Offset: 9)
	pub fn set_ep_type(value: u8) { ::write(REGISTER_ADDRESS, EP_TYPE_BIT_OFFSET, EP_TYPE_BIT_WIDTH, value as u32); }

	const SETUP_BIT_OFFSET: u8 = 11;
	const SETUP_BIT_WIDTH: u8 = 1;
	/// Setup transaction completed (Width: 1, Offset: 11)
	pub fn get_setup() -> u8 { ::read(REGISTER_ADDRESS, SETUP_BIT_OFFSET, SETUP_BIT_WIDTH) as u8 }

	const STAT_RX_BIT_OFFSET: u8 = 12;
	const STAT_RX_BIT_WIDTH: u8 = 2;
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn get_stat_rx() -> u8 { ::read(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH) as u8 }
	/// Status bits, for reception transfers (Width: 2, Offset: 12)
	pub fn set_stat_rx(value: u8) { ::write(REGISTER_ADDRESS, STAT_RX_BIT_OFFSET, STAT_RX_BIT_WIDTH, value as u32); }

	const DTOG_RX_BIT_OFFSET: u8 = 14;
	const DTOG_RX_BIT_WIDTH: u8 = 1;
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn get_dtog_rx() -> u8 { ::read(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH) as u8 }
	/// Data Toggle, for reception transfers (Width: 1, Offset: 14)
	pub fn set_dtog_rx(value: u8) { ::write(REGISTER_ADDRESS, DTOG_RX_BIT_OFFSET, DTOG_RX_BIT_WIDTH, value as u32); }

	const CTR_RX_BIT_OFFSET: u8 = 15;
	const CTR_RX_BIT_WIDTH: u8 = 1;
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn get_ctr_rx() -> u8 { ::read(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH) as u8 }
	/// Correct transfer for reception (Width: 1, Offset: 15)
	pub fn set_ctr_rx(value: u8) { ::write(REGISTER_ADDRESS, CTR_RX_BIT_OFFSET, CTR_RX_BIT_WIDTH, value as u32); }
}
/// control register
/// Size: 0x20 bits
pub mod usb_cntr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x40;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const FRES_BIT_OFFSET: u8 = 0;
	const FRES_BIT_WIDTH: u8 = 1;
	/// Force USB Reset (Width: 1, Offset: 0)
	pub fn get_fres() -> u8 { ::read(REGISTER_ADDRESS, FRES_BIT_OFFSET, FRES_BIT_WIDTH) as u8 }
	/// Force USB Reset (Width: 1, Offset: 0)
	pub fn set_fres(value: u8) { ::write(REGISTER_ADDRESS, FRES_BIT_OFFSET, FRES_BIT_WIDTH, value as u32); }

	const PDWN_BIT_OFFSET: u8 = 1;
	const PDWN_BIT_WIDTH: u8 = 1;
	/// Power down (Width: 1, Offset: 1)
	pub fn get_pdwn() -> u8 { ::read(REGISTER_ADDRESS, PDWN_BIT_OFFSET, PDWN_BIT_WIDTH) as u8 }
	/// Power down (Width: 1, Offset: 1)
	pub fn set_pdwn(value: u8) { ::write(REGISTER_ADDRESS, PDWN_BIT_OFFSET, PDWN_BIT_WIDTH, value as u32); }

	const LPMODE_BIT_OFFSET: u8 = 2;
	const LPMODE_BIT_WIDTH: u8 = 1;
	/// Low-power mode (Width: 1, Offset: 2)
	pub fn get_lpmode() -> u8 { ::read(REGISTER_ADDRESS, LPMODE_BIT_OFFSET, LPMODE_BIT_WIDTH) as u8 }
	/// Low-power mode (Width: 1, Offset: 2)
	pub fn set_lpmode(value: u8) { ::write(REGISTER_ADDRESS, LPMODE_BIT_OFFSET, LPMODE_BIT_WIDTH, value as u32); }

	const FSUSP_BIT_OFFSET: u8 = 3;
	const FSUSP_BIT_WIDTH: u8 = 1;
	/// Force suspend (Width: 1, Offset: 3)
	pub fn get_fsusp() -> u8 { ::read(REGISTER_ADDRESS, FSUSP_BIT_OFFSET, FSUSP_BIT_WIDTH) as u8 }
	/// Force suspend (Width: 1, Offset: 3)
	pub fn set_fsusp(value: u8) { ::write(REGISTER_ADDRESS, FSUSP_BIT_OFFSET, FSUSP_BIT_WIDTH, value as u32); }

	const RESUME_BIT_OFFSET: u8 = 4;
	const RESUME_BIT_WIDTH: u8 = 1;
	/// Resume request (Width: 1, Offset: 4)
	pub fn get_resume() -> u8 { ::read(REGISTER_ADDRESS, RESUME_BIT_OFFSET, RESUME_BIT_WIDTH) as u8 }
	/// Resume request (Width: 1, Offset: 4)
	pub fn set_resume(value: u8) { ::write(REGISTER_ADDRESS, RESUME_BIT_OFFSET, RESUME_BIT_WIDTH, value as u32); }

	const ESOFM_BIT_OFFSET: u8 = 8;
	const ESOFM_BIT_WIDTH: u8 = 1;
	/// Expected start of frame interrupt mask (Width: 1, Offset: 8)
	pub fn get_esofm() -> u8 { ::read(REGISTER_ADDRESS, ESOFM_BIT_OFFSET, ESOFM_BIT_WIDTH) as u8 }
	/// Expected start of frame interrupt mask (Width: 1, Offset: 8)
	pub fn set_esofm(value: u8) { ::write(REGISTER_ADDRESS, ESOFM_BIT_OFFSET, ESOFM_BIT_WIDTH, value as u32); }

	const SOFM_BIT_OFFSET: u8 = 9;
	const SOFM_BIT_WIDTH: u8 = 1;
	/// Start of frame interrupt mask (Width: 1, Offset: 9)
	pub fn get_sofm() -> u8 { ::read(REGISTER_ADDRESS, SOFM_BIT_OFFSET, SOFM_BIT_WIDTH) as u8 }
	/// Start of frame interrupt mask (Width: 1, Offset: 9)
	pub fn set_sofm(value: u8) { ::write(REGISTER_ADDRESS, SOFM_BIT_OFFSET, SOFM_BIT_WIDTH, value as u32); }

	const RESETM_BIT_OFFSET: u8 = 10;
	const RESETM_BIT_WIDTH: u8 = 1;
	/// USB reset interrupt mask (Width: 1, Offset: 10)
	pub fn get_resetm() -> u8 { ::read(REGISTER_ADDRESS, RESETM_BIT_OFFSET, RESETM_BIT_WIDTH) as u8 }
	/// USB reset interrupt mask (Width: 1, Offset: 10)
	pub fn set_resetm(value: u8) { ::write(REGISTER_ADDRESS, RESETM_BIT_OFFSET, RESETM_BIT_WIDTH, value as u32); }

	const SUSPM_BIT_OFFSET: u8 = 11;
	const SUSPM_BIT_WIDTH: u8 = 1;
	/// Suspend mode interrupt mask (Width: 1, Offset: 11)
	pub fn get_suspm() -> u8 { ::read(REGISTER_ADDRESS, SUSPM_BIT_OFFSET, SUSPM_BIT_WIDTH) as u8 }
	/// Suspend mode interrupt mask (Width: 1, Offset: 11)
	pub fn set_suspm(value: u8) { ::write(REGISTER_ADDRESS, SUSPM_BIT_OFFSET, SUSPM_BIT_WIDTH, value as u32); }

	const WKUPM_BIT_OFFSET: u8 = 12;
	const WKUPM_BIT_WIDTH: u8 = 1;
	/// Wakeup interrupt mask (Width: 1, Offset: 12)
	pub fn get_wkupm() -> u8 { ::read(REGISTER_ADDRESS, WKUPM_BIT_OFFSET, WKUPM_BIT_WIDTH) as u8 }
	/// Wakeup interrupt mask (Width: 1, Offset: 12)
	pub fn set_wkupm(value: u8) { ::write(REGISTER_ADDRESS, WKUPM_BIT_OFFSET, WKUPM_BIT_WIDTH, value as u32); }

	const ERRM_BIT_OFFSET: u8 = 13;
	const ERRM_BIT_WIDTH: u8 = 1;
	/// Error interrupt mask (Width: 1, Offset: 13)
	pub fn get_errm() -> u8 { ::read(REGISTER_ADDRESS, ERRM_BIT_OFFSET, ERRM_BIT_WIDTH) as u8 }
	/// Error interrupt mask (Width: 1, Offset: 13)
	pub fn set_errm(value: u8) { ::write(REGISTER_ADDRESS, ERRM_BIT_OFFSET, ERRM_BIT_WIDTH, value as u32); }

	const PMAOVRM_BIT_OFFSET: u8 = 14;
	const PMAOVRM_BIT_WIDTH: u8 = 1;
	/// Packet memory area over / underrun interrupt mask (Width: 1, Offset: 14)
	pub fn get_pmaovrm() -> u8 { ::read(REGISTER_ADDRESS, PMAOVRM_BIT_OFFSET, PMAOVRM_BIT_WIDTH) as u8 }
	/// Packet memory area over / underrun interrupt mask (Width: 1, Offset: 14)
	pub fn set_pmaovrm(value: u8) { ::write(REGISTER_ADDRESS, PMAOVRM_BIT_OFFSET, PMAOVRM_BIT_WIDTH, value as u32); }

	const CTRM_BIT_OFFSET: u8 = 15;
	const CTRM_BIT_WIDTH: u8 = 1;
	/// Correct transfer interrupt mask (Width: 1, Offset: 15)
	pub fn get_ctrm() -> u8 { ::read(REGISTER_ADDRESS, CTRM_BIT_OFFSET, CTRM_BIT_WIDTH) as u8 }
	/// Correct transfer interrupt mask (Width: 1, Offset: 15)
	pub fn set_ctrm(value: u8) { ::write(REGISTER_ADDRESS, CTRM_BIT_OFFSET, CTRM_BIT_WIDTH, value as u32); }
}
/// interrupt status register
/// Size: 0x20 bits
pub mod istr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x44;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EP_ID_BIT_OFFSET: u8 = 0;
	const EP_ID_BIT_WIDTH: u8 = 4;
	/// Endpoint Identifier (Width: 4, Offset: 0)
	pub fn get_ep_id() -> u8 { ::read(REGISTER_ADDRESS, EP_ID_BIT_OFFSET, EP_ID_BIT_WIDTH) as u8 }

	const DIR_BIT_OFFSET: u8 = 4;
	const DIR_BIT_WIDTH: u8 = 1;
	/// Direction of transaction (Width: 1, Offset: 4)
	pub fn get_dir() -> u8 { ::read(REGISTER_ADDRESS, DIR_BIT_OFFSET, DIR_BIT_WIDTH) as u8 }

	const ESOF_BIT_OFFSET: u8 = 8;
	const ESOF_BIT_WIDTH: u8 = 1;
	/// Expected start frame (Width: 1, Offset: 8)
	pub fn get_esof() -> u8 { ::read(REGISTER_ADDRESS, ESOF_BIT_OFFSET, ESOF_BIT_WIDTH) as u8 }
	/// Expected start frame (Width: 1, Offset: 8)
	pub fn set_esof(value: u8) { ::write(REGISTER_ADDRESS, ESOF_BIT_OFFSET, ESOF_BIT_WIDTH, value as u32); }

	const SOF_BIT_OFFSET: u8 = 9;
	const SOF_BIT_WIDTH: u8 = 1;
	/// start of frame (Width: 1, Offset: 9)
	pub fn get_sof() -> u8 { ::read(REGISTER_ADDRESS, SOF_BIT_OFFSET, SOF_BIT_WIDTH) as u8 }
	/// start of frame (Width: 1, Offset: 9)
	pub fn set_sof(value: u8) { ::write(REGISTER_ADDRESS, SOF_BIT_OFFSET, SOF_BIT_WIDTH, value as u32); }

	const RESET_BIT_OFFSET: u8 = 10;
	const RESET_BIT_WIDTH: u8 = 1;
	/// reset request (Width: 1, Offset: 10)
	pub fn get_reset() -> u8 { ::read(REGISTER_ADDRESS, RESET_BIT_OFFSET, RESET_BIT_WIDTH) as u8 }
	/// reset request (Width: 1, Offset: 10)
	pub fn set_reset(value: u8) { ::write(REGISTER_ADDRESS, RESET_BIT_OFFSET, RESET_BIT_WIDTH, value as u32); }

	const SUSP_BIT_OFFSET: u8 = 11;
	const SUSP_BIT_WIDTH: u8 = 1;
	/// Suspend mode request (Width: 1, Offset: 11)
	pub fn get_susp() -> u8 { ::read(REGISTER_ADDRESS, SUSP_BIT_OFFSET, SUSP_BIT_WIDTH) as u8 }
	/// Suspend mode request (Width: 1, Offset: 11)
	pub fn set_susp(value: u8) { ::write(REGISTER_ADDRESS, SUSP_BIT_OFFSET, SUSP_BIT_WIDTH, value as u32); }

	const WKUP_BIT_OFFSET: u8 = 12;
	const WKUP_BIT_WIDTH: u8 = 1;
	/// Wakeup (Width: 1, Offset: 12)
	pub fn get_wkup() -> u8 { ::read(REGISTER_ADDRESS, WKUP_BIT_OFFSET, WKUP_BIT_WIDTH) as u8 }
	/// Wakeup (Width: 1, Offset: 12)
	pub fn set_wkup(value: u8) { ::write(REGISTER_ADDRESS, WKUP_BIT_OFFSET, WKUP_BIT_WIDTH, value as u32); }

	const ERR_BIT_OFFSET: u8 = 13;
	const ERR_BIT_WIDTH: u8 = 1;
	/// Error (Width: 1, Offset: 13)
	pub fn get_err() -> u8 { ::read(REGISTER_ADDRESS, ERR_BIT_OFFSET, ERR_BIT_WIDTH) as u8 }
	/// Error (Width: 1, Offset: 13)
	pub fn set_err(value: u8) { ::write(REGISTER_ADDRESS, ERR_BIT_OFFSET, ERR_BIT_WIDTH, value as u32); }

	const PMAOVR_BIT_OFFSET: u8 = 14;
	const PMAOVR_BIT_WIDTH: u8 = 1;
	/// Packet memory area over / underrun (Width: 1, Offset: 14)
	pub fn get_pmaovr() -> u8 { ::read(REGISTER_ADDRESS, PMAOVR_BIT_OFFSET, PMAOVR_BIT_WIDTH) as u8 }
	/// Packet memory area over / underrun (Width: 1, Offset: 14)
	pub fn set_pmaovr(value: u8) { ::write(REGISTER_ADDRESS, PMAOVR_BIT_OFFSET, PMAOVR_BIT_WIDTH, value as u32); }

	const CTR_BIT_OFFSET: u8 = 15;
	const CTR_BIT_WIDTH: u8 = 1;
	/// Correct transfer (Width: 1, Offset: 15)
	pub fn get_ctr() -> u8 { ::read(REGISTER_ADDRESS, CTR_BIT_OFFSET, CTR_BIT_WIDTH) as u8 }
}
/// frame number register
/// Size: 0x20 bits
pub mod fnr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x48;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const FN_BIT_OFFSET: u8 = 0;
	const FN_BIT_WIDTH: u8 = 11;
	/// Frame number (Width: 11, Offset: 0)
	pub fn get_fn() -> u16 { ::read(REGISTER_ADDRESS, FN_BIT_OFFSET, FN_BIT_WIDTH) as u16 }

	const LSOF_BIT_OFFSET: u8 = 11;
	const LSOF_BIT_WIDTH: u8 = 2;
	/// Lost SOF (Width: 2, Offset: 11)
	pub fn get_lsof() -> u8 { ::read(REGISTER_ADDRESS, LSOF_BIT_OFFSET, LSOF_BIT_WIDTH) as u8 }

	const LCK_BIT_OFFSET: u8 = 13;
	const LCK_BIT_WIDTH: u8 = 1;
	/// Locked (Width: 1, Offset: 13)
	pub fn get_lck() -> u8 { ::read(REGISTER_ADDRESS, LCK_BIT_OFFSET, LCK_BIT_WIDTH) as u8 }

	const RXDM_BIT_OFFSET: u8 = 14;
	const RXDM_BIT_WIDTH: u8 = 1;
	/// Receive data - line status (Width: 1, Offset: 14)
	pub fn get_rxdm() -> u8 { ::read(REGISTER_ADDRESS, RXDM_BIT_OFFSET, RXDM_BIT_WIDTH) as u8 }

	const RXDP_BIT_OFFSET: u8 = 15;
	const RXDP_BIT_WIDTH: u8 = 1;
	/// Receive data + line status (Width: 1, Offset: 15)
	pub fn get_rxdp() -> u8 { ::read(REGISTER_ADDRESS, RXDP_BIT_OFFSET, RXDP_BIT_WIDTH) as u8 }
}
/// device address
/// Size: 0x20 bits
pub mod daddr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const ADD_BIT_OFFSET: u8 = 0;
	const ADD_BIT_WIDTH: u8 = 1;
	/// Device address (Width: 1, Offset: 0)
	pub fn get_add() -> u8 { ::read(REGISTER_ADDRESS, ADD_BIT_OFFSET, ADD_BIT_WIDTH) as u8 }
	/// Device address (Width: 1, Offset: 0)
	pub fn set_add(value: u8) { ::write(REGISTER_ADDRESS, ADD_BIT_OFFSET, ADD_BIT_WIDTH, value as u32); }

	const ADD1_BIT_OFFSET: u8 = 1;
	const ADD1_BIT_WIDTH: u8 = 1;
	/// Device address (Width: 1, Offset: 1)
	pub fn get_add1() -> u8 { ::read(REGISTER_ADDRESS, ADD1_BIT_OFFSET, ADD1_BIT_WIDTH) as u8 }
	/// Device address (Width: 1, Offset: 1)
	pub fn set_add1(value: u8) { ::write(REGISTER_ADDRESS, ADD1_BIT_OFFSET, ADD1_BIT_WIDTH, value as u32); }

	const ADD2_BIT_OFFSET: u8 = 2;
	const ADD2_BIT_WIDTH: u8 = 1;
	/// Device address (Width: 1, Offset: 2)
	pub fn get_add2() -> u8 { ::read(REGISTER_ADDRESS, ADD2_BIT_OFFSET, ADD2_BIT_WIDTH) as u8 }
	/// Device address (Width: 1, Offset: 2)
	pub fn set_add2(value: u8) { ::write(REGISTER_ADDRESS, ADD2_BIT_OFFSET, ADD2_BIT_WIDTH, value as u32); }

	const ADD3_BIT_OFFSET: u8 = 3;
	const ADD3_BIT_WIDTH: u8 = 1;
	/// Device address (Width: 1, Offset: 3)
	pub fn get_add3() -> u8 { ::read(REGISTER_ADDRESS, ADD3_BIT_OFFSET, ADD3_BIT_WIDTH) as u8 }
	/// Device address (Width: 1, Offset: 3)
	pub fn set_add3(value: u8) { ::write(REGISTER_ADDRESS, ADD3_BIT_OFFSET, ADD3_BIT_WIDTH, value as u32); }

	const ADD4_BIT_OFFSET: u8 = 4;
	const ADD4_BIT_WIDTH: u8 = 1;
	/// Device address (Width: 1, Offset: 4)
	pub fn get_add4() -> u8 { ::read(REGISTER_ADDRESS, ADD4_BIT_OFFSET, ADD4_BIT_WIDTH) as u8 }
	/// Device address (Width: 1, Offset: 4)
	pub fn set_add4(value: u8) { ::write(REGISTER_ADDRESS, ADD4_BIT_OFFSET, ADD4_BIT_WIDTH, value as u32); }

	const ADD5_BIT_OFFSET: u8 = 5;
	const ADD5_BIT_WIDTH: u8 = 1;
	/// Device address (Width: 1, Offset: 5)
	pub fn get_add5() -> u8 { ::read(REGISTER_ADDRESS, ADD5_BIT_OFFSET, ADD5_BIT_WIDTH) as u8 }
	/// Device address (Width: 1, Offset: 5)
	pub fn set_add5(value: u8) { ::write(REGISTER_ADDRESS, ADD5_BIT_OFFSET, ADD5_BIT_WIDTH, value as u32); }

	const ADD6_BIT_OFFSET: u8 = 6;
	const ADD6_BIT_WIDTH: u8 = 1;
	/// Device address (Width: 1, Offset: 6)
	pub fn get_add6() -> u8 { ::read(REGISTER_ADDRESS, ADD6_BIT_OFFSET, ADD6_BIT_WIDTH) as u8 }
	/// Device address (Width: 1, Offset: 6)
	pub fn set_add6(value: u8) { ::write(REGISTER_ADDRESS, ADD6_BIT_OFFSET, ADD6_BIT_WIDTH, value as u32); }

	const EF_BIT_OFFSET: u8 = 7;
	const EF_BIT_WIDTH: u8 = 1;
	/// Enable function (Width: 1, Offset: 7)
	pub fn get_ef() -> u8 { ::read(REGISTER_ADDRESS, EF_BIT_OFFSET, EF_BIT_WIDTH) as u8 }
	/// Enable function (Width: 1, Offset: 7)
	pub fn set_ef(value: u8) { ::write(REGISTER_ADDRESS, EF_BIT_OFFSET, EF_BIT_WIDTH, value as u32); }
}
/// Buffer table address
/// Size: 0x20 bits
pub mod btable {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x50;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const BTABLE_BIT_OFFSET: u8 = 3;
	const BTABLE_BIT_WIDTH: u8 = 13;
	/// Buffer table (Width: 13, Offset: 3)
	pub fn get_btable() -> u16 { ::read(REGISTER_ADDRESS, BTABLE_BIT_OFFSET, BTABLE_BIT_WIDTH) as u16 }
	/// Buffer table (Width: 13, Offset: 3)
	pub fn set_btable(value: u16) { ::write(REGISTER_ADDRESS, BTABLE_BIT_OFFSET, BTABLE_BIT_WIDTH, value as u32); }
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
  <name>USB_FS</name>
  <description>Universal serial bus full-speed device
      interface</description>
  <groupName>USB_FS</groupName>
  <baseAddress>0x40005C00</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>USB_EP0R</name>
      <displayName>USB_EP0R</displayName>
      <description>endpoint 0 register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EA</name>
          <description>Endpoint address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>STAT_TX</name>
          <description>Status bits, for transmission
              transfers</description>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_TX</name>
          <description>Data Toggle, for transmission
              transfers</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_TX</name>
          <description>Correct Transfer for
              transmission</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_KIND</name>
          <description>Endpoint kind</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_TYPE</name>
          <description>Endpoint type</description>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SETUP</name>
          <description>Setup transaction
              completed</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>STAT_RX</name>
          <description>Status bits, for reception
              transfers</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_RX</name>
          <description>Data Toggle, for reception
              transfers</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_RX</name>
          <description>Correct transfer for
              reception</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>USB_EP1R</name>
      <displayName>USB_EP1R</displayName>
      <description>endpoint 1 register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EA</name>
          <description>Endpoint address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>STAT_TX</name>
          <description>Status bits, for transmission
              transfers</description>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_TX</name>
          <description>Data Toggle, for transmission
              transfers</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_TX</name>
          <description>Correct Transfer for
              transmission</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_KIND</name>
          <description>Endpoint kind</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_TYPE</name>
          <description>Endpoint type</description>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SETUP</name>
          <description>Setup transaction
              completed</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>STAT_RX</name>
          <description>Status bits, for reception
              transfers</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_RX</name>
          <description>Data Toggle, for reception
              transfers</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_RX</name>
          <description>Correct transfer for
              reception</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>USB_EP2R</name>
      <displayName>USB_EP2R</displayName>
      <description>endpoint 2 register</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EA</name>
          <description>Endpoint address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>STAT_TX</name>
          <description>Status bits, for transmission
              transfers</description>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_TX</name>
          <description>Data Toggle, for transmission
              transfers</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_TX</name>
          <description>Correct Transfer for
              transmission</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_KIND</name>
          <description>Endpoint kind</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_TYPE</name>
          <description>Endpoint type</description>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SETUP</name>
          <description>Setup transaction
              completed</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>STAT_RX</name>
          <description>Status bits, for reception
              transfers</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_RX</name>
          <description>Data Toggle, for reception
              transfers</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_RX</name>
          <description>Correct transfer for
              reception</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>USB_EP3R</name>
      <displayName>USB_EP3R</displayName>
      <description>endpoint 3 register</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EA</name>
          <description>Endpoint address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>STAT_TX</name>
          <description>Status bits, for transmission
              transfers</description>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_TX</name>
          <description>Data Toggle, for transmission
              transfers</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_TX</name>
          <description>Correct Transfer for
              transmission</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_KIND</name>
          <description>Endpoint kind</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_TYPE</name>
          <description>Endpoint type</description>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SETUP</name>
          <description>Setup transaction
              completed</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>STAT_RX</name>
          <description>Status bits, for reception
              transfers</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_RX</name>
          <description>Data Toggle, for reception
              transfers</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_RX</name>
          <description>Correct transfer for
              reception</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>USB_EP4R</name>
      <displayName>USB_EP4R</displayName>
      <description>endpoint 4 register</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EA</name>
          <description>Endpoint address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>STAT_TX</name>
          <description>Status bits, for transmission
              transfers</description>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_TX</name>
          <description>Data Toggle, for transmission
              transfers</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_TX</name>
          <description>Correct Transfer for
              transmission</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_KIND</name>
          <description>Endpoint kind</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_TYPE</name>
          <description>Endpoint type</description>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SETUP</name>
          <description>Setup transaction
              completed</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>STAT_RX</name>
          <description>Status bits, for reception
              transfers</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_RX</name>
          <description>Data Toggle, for reception
              transfers</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_RX</name>
          <description>Correct transfer for
              reception</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>USB_EP5R</name>
      <displayName>USB_EP5R</displayName>
      <description>endpoint 5 register</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EA</name>
          <description>Endpoint address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>STAT_TX</name>
          <description>Status bits, for transmission
              transfers</description>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_TX</name>
          <description>Data Toggle, for transmission
              transfers</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_TX</name>
          <description>Correct Transfer for
              transmission</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_KIND</name>
          <description>Endpoint kind</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_TYPE</name>
          <description>Endpoint type</description>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SETUP</name>
          <description>Setup transaction
              completed</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>STAT_RX</name>
          <description>Status bits, for reception
              transfers</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_RX</name>
          <description>Data Toggle, for reception
              transfers</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_RX</name>
          <description>Correct transfer for
              reception</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>USB_EP6R</name>
      <displayName>USB_EP6R</displayName>
      <description>endpoint 6 register</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EA</name>
          <description>Endpoint address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>STAT_TX</name>
          <description>Status bits, for transmission
              transfers</description>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_TX</name>
          <description>Data Toggle, for transmission
              transfers</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_TX</name>
          <description>Correct Transfer for
              transmission</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_KIND</name>
          <description>Endpoint kind</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_TYPE</name>
          <description>Endpoint type</description>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SETUP</name>
          <description>Setup transaction
              completed</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>STAT_RX</name>
          <description>Status bits, for reception
              transfers</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_RX</name>
          <description>Data Toggle, for reception
              transfers</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_RX</name>
          <description>Correct transfer for
              reception</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>USB_EP7R</name>
      <displayName>USB_EP7R</displayName>
      <description>endpoint 7 register</description>
      <addressOffset>0x1C</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EA</name>
          <description>Endpoint address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>STAT_TX</name>
          <description>Status bits, for transmission
              transfers</description>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_TX</name>
          <description>Data Toggle, for transmission
              transfers</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_TX</name>
          <description>Correct Transfer for
              transmission</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_KIND</name>
          <description>Endpoint kind</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>EP_TYPE</name>
          <description>Endpoint type</description>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SETUP</name>
          <description>Setup transaction
              completed</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>STAT_RX</name>
          <description>Status bits, for reception
              transfers</description>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>DTOG_RX</name>
          <description>Data Toggle, for reception
              transfers</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR_RX</name>
          <description>Correct transfer for
              reception</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>USB_CNTR</name>
      <displayName>USB_CNTR</displayName>
      <description>control register</description>
      <addressOffset>0x40</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000003</resetValue>
      <fields>
        <field>
          <name>FRES</name>
          <description>Force USB Reset</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PDWN</name>
          <description>Power down</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>LPMODE</name>
          <description>Low-power mode</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>FSUSP</name>
          <description>Force suspend</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RESUME</name>
          <description>Resume request</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ESOFM</name>
          <description>Expected start of frame interrupt
              mask</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SOFM</name>
          <description>Start of frame interrupt
              mask</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RESETM</name>
          <description>USB reset interrupt mask</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SUSPM</name>
          <description>Suspend mode interrupt
              mask</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>WKUPM</name>
          <description>Wakeup interrupt mask</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ERRM</name>
          <description>Error interrupt mask</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PMAOVRM</name>
          <description>Packet memory area over / underrun
              interrupt mask</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CTRM</name>
          <description>Correct transfer interrupt
              mask</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>ISTR</name>
      <displayName>ISTR</displayName>
      <description>interrupt status register</description>
      <addressOffset>0x44</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>EP_ID</name>
          <description>Endpoint Identifier</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>DIR</name>
          <description>Direction of transaction</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>ESOF</name>
          <description>Expected start frame</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SOF</name>
          <description>start of frame</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>RESET</name>
          <description>reset request</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SUSP</name>
          <description>Suspend mode request</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>WKUP</name>
          <description>Wakeup</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>ERR</name>
          <description>Error</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PMAOVR</name>
          <description>Packet memory area over /
              underrun</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CTR</name>
          <description>Correct transfer</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
      </fields>
    </register>
    <register>
      <name>FNR</name>
      <displayName>FNR</displayName>
      <description>frame number register</description>
      <addressOffset>0x48</addressOffset>
      <size>0x20</size>
      <access>read-only</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>FN</name>
          <description>Frame number</description>
          <bitOffset>0</bitOffset>
          <bitWidth>11</bitWidth>
        </field>
        <field>
          <name>LSOF</name>
          <description>Lost SOF</description>
          <bitOffset>11</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>LCK</name>
          <description>Locked</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RXDM</name>
          <description>Receive data - line status</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>RXDP</name>
          <description>Receive data + line status</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>DADDR</name>
      <displayName>DADDR</displayName>
      <description>device address</description>
      <addressOffset>0x4C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>ADD</name>
          <description>Device address</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADD1</name>
          <description>Device address</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADD2</name>
          <description>Device address</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADD3</name>
          <description>Device address</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADD4</name>
          <description>Device address</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADD5</name>
          <description>Device address</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADD6</name>
          <description>Device address</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>EF</name>
          <description>Enable function</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BTABLE</name>
      <displayName>BTABLE</displayName>
      <description>Buffer table address</description>
      <addressOffset>0x50</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>BTABLE</name>
          <description>Buffer table</description>
          <bitOffset>3</bitOffset>
          <bitWidth>13</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>USB_WKUP</name>
    <description>USB wakeup from Suspend</description>
    <value>42</value>
  </interrupt>
  <interrupt>
    <name>USB_HP</name>
    <description>USB High priority interrupt</description>
    <value>74</value>
  </interrupt>
  <interrupt>
    <name>USB_LP</name>
    <description>USB Low priority interrupt</description>
    <value>75</value>
  </interrupt>
</peripheral>*/
