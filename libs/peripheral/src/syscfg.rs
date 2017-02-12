/// MOD SYSCFG
/// System configuration controller
const BASE_ADDRESS: u32 = 0x40010000;
/// configuration register 1
/// Size: 0x20 bits
pub mod cfgr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const MEM_MODE_BIT_OFFSET: u8 = 0;
	const MEM_MODE_BIT_WIDTH: u8 = 2;
	/// Memory mapping selection bits (Width: 2, Offset: 0)
	pub fn get_mem_mode() -> u8 { ::read(REGISTER_ADDRESS, MEM_MODE_BIT_OFFSET, MEM_MODE_BIT_WIDTH) as u8 }
	/// Memory mapping selection bits (Width: 2, Offset: 0)
	pub fn set_mem_mode(value: u8) { ::write(REGISTER_ADDRESS, MEM_MODE_BIT_OFFSET, MEM_MODE_BIT_WIDTH, value as u32); }

	const USB_IT_RMP_BIT_OFFSET: u8 = 5;
	const USB_IT_RMP_BIT_WIDTH: u8 = 1;
	/// USB interrupt remap (Width: 1, Offset: 5)
	pub fn get_usb_it_rmp() -> u8 { ::read(REGISTER_ADDRESS, USB_IT_RMP_BIT_OFFSET, USB_IT_RMP_BIT_WIDTH) as u8 }
	/// USB interrupt remap (Width: 1, Offset: 5)
	pub fn set_usb_it_rmp(value: u8) { ::write(REGISTER_ADDRESS, USB_IT_RMP_BIT_OFFSET, USB_IT_RMP_BIT_WIDTH, value as u32); }

	const TIM1_ITR_RMP_BIT_OFFSET: u8 = 6;
	const TIM1_ITR_RMP_BIT_WIDTH: u8 = 1;
	/// Timer 1 ITR3 selection (Width: 1, Offset: 6)
	pub fn get_tim1_itr_rmp() -> u8 { ::read(REGISTER_ADDRESS, TIM1_ITR_RMP_BIT_OFFSET, TIM1_ITR_RMP_BIT_WIDTH) as u8 }
	/// Timer 1 ITR3 selection (Width: 1, Offset: 6)
	pub fn set_tim1_itr_rmp(value: u8) { ::write(REGISTER_ADDRESS, TIM1_ITR_RMP_BIT_OFFSET, TIM1_ITR_RMP_BIT_WIDTH, value as u32); }

	const DAC_TRIG_RMP_BIT_OFFSET: u8 = 7;
	const DAC_TRIG_RMP_BIT_WIDTH: u8 = 1;
	/// DAC trigger remap (when TSEL = 001) (Width: 1, Offset: 7)
	pub fn get_dac_trig_rmp() -> u8 { ::read(REGISTER_ADDRESS, DAC_TRIG_RMP_BIT_OFFSET, DAC_TRIG_RMP_BIT_WIDTH) as u8 }
	/// DAC trigger remap (when TSEL = 001) (Width: 1, Offset: 7)
	pub fn set_dac_trig_rmp(value: u8) { ::write(REGISTER_ADDRESS, DAC_TRIG_RMP_BIT_OFFSET, DAC_TRIG_RMP_BIT_WIDTH, value as u32); }

	const ADC24_DMA_RMP_BIT_OFFSET: u8 = 8;
	const ADC24_DMA_RMP_BIT_WIDTH: u8 = 1;
	/// ADC24 DMA remapping bit (Width: 1, Offset: 8)
	pub fn get_adc24_dma_rmp() -> u8 { ::read(REGISTER_ADDRESS, ADC24_DMA_RMP_BIT_OFFSET, ADC24_DMA_RMP_BIT_WIDTH) as u8 }
	/// ADC24 DMA remapping bit (Width: 1, Offset: 8)
	pub fn set_adc24_dma_rmp(value: u8) { ::write(REGISTER_ADDRESS, ADC24_DMA_RMP_BIT_OFFSET, ADC24_DMA_RMP_BIT_WIDTH, value as u32); }

	const TIM16_DMA_RMP_BIT_OFFSET: u8 = 11;
	const TIM16_DMA_RMP_BIT_WIDTH: u8 = 1;
	/// TIM16 DMA request remapping bit (Width: 1, Offset: 11)
	pub fn get_tim16_dma_rmp() -> u8 { ::read(REGISTER_ADDRESS, TIM16_DMA_RMP_BIT_OFFSET, TIM16_DMA_RMP_BIT_WIDTH) as u8 }
	/// TIM16 DMA request remapping bit (Width: 1, Offset: 11)
	pub fn set_tim16_dma_rmp(value: u8) { ::write(REGISTER_ADDRESS, TIM16_DMA_RMP_BIT_OFFSET, TIM16_DMA_RMP_BIT_WIDTH, value as u32); }

	const TIM17_DMA_RMP_BIT_OFFSET: u8 = 12;
	const TIM17_DMA_RMP_BIT_WIDTH: u8 = 1;
	/// TIM17 DMA request remapping bit (Width: 1, Offset: 12)
	pub fn get_tim17_dma_rmp() -> u8 { ::read(REGISTER_ADDRESS, TIM17_DMA_RMP_BIT_OFFSET, TIM17_DMA_RMP_BIT_WIDTH) as u8 }
	/// TIM17 DMA request remapping bit (Width: 1, Offset: 12)
	pub fn set_tim17_dma_rmp(value: u8) { ::write(REGISTER_ADDRESS, TIM17_DMA_RMP_BIT_OFFSET, TIM17_DMA_RMP_BIT_WIDTH, value as u32); }

	const TIM6_DAC1_DMA_RMP_BIT_OFFSET: u8 = 13;
	const TIM6_DAC1_DMA_RMP_BIT_WIDTH: u8 = 1;
	/// TIM6 and DAC1 DMA request remapping bit (Width: 1, Offset: 13)
	pub fn get_tim6_dac1_dma_rmp() -> u8 { ::read(REGISTER_ADDRESS, TIM6_DAC1_DMA_RMP_BIT_OFFSET, TIM6_DAC1_DMA_RMP_BIT_WIDTH) as u8 }
	/// TIM6 and DAC1 DMA request remapping bit (Width: 1, Offset: 13)
	pub fn set_tim6_dac1_dma_rmp(value: u8) { ::write(REGISTER_ADDRESS, TIM6_DAC1_DMA_RMP_BIT_OFFSET, TIM6_DAC1_DMA_RMP_BIT_WIDTH, value as u32); }

	const TIM7_DAC2_DMA_RMP_BIT_OFFSET: u8 = 14;
	const TIM7_DAC2_DMA_RMP_BIT_WIDTH: u8 = 1;
	/// TIM7 and DAC2 DMA request remapping bit (Width: 1, Offset: 14)
	pub fn get_tim7_dac2_dma_rmp() -> u8 { ::read(REGISTER_ADDRESS, TIM7_DAC2_DMA_RMP_BIT_OFFSET, TIM7_DAC2_DMA_RMP_BIT_WIDTH) as u8 }
	/// TIM7 and DAC2 DMA request remapping bit (Width: 1, Offset: 14)
	pub fn set_tim7_dac2_dma_rmp(value: u8) { ::write(REGISTER_ADDRESS, TIM7_DAC2_DMA_RMP_BIT_OFFSET, TIM7_DAC2_DMA_RMP_BIT_WIDTH, value as u32); }

	const I2C_PB6_FM_BIT_OFFSET: u8 = 16;
	const I2C_PB6_FM_BIT_WIDTH: u8 = 1;
	/// Fast Mode Plus (FM+) driving capability activation bits. (Width: 1, Offset: 16)
	pub fn get_i2c_pb6_fm() -> u8 { ::read(REGISTER_ADDRESS, I2C_PB6_FM_BIT_OFFSET, I2C_PB6_FM_BIT_WIDTH) as u8 }
	/// Fast Mode Plus (FM+) driving capability activation bits. (Width: 1, Offset: 16)
	pub fn set_i2c_pb6_fm(value: u8) { ::write(REGISTER_ADDRESS, I2C_PB6_FM_BIT_OFFSET, I2C_PB6_FM_BIT_WIDTH, value as u32); }

	const I2C_PB7_FM_BIT_OFFSET: u8 = 17;
	const I2C_PB7_FM_BIT_WIDTH: u8 = 1;
	/// Fast Mode Plus (FM+) driving capability activation bits. (Width: 1, Offset: 17)
	pub fn get_i2c_pb7_fm() -> u8 { ::read(REGISTER_ADDRESS, I2C_PB7_FM_BIT_OFFSET, I2C_PB7_FM_BIT_WIDTH) as u8 }
	/// Fast Mode Plus (FM+) driving capability activation bits. (Width: 1, Offset: 17)
	pub fn set_i2c_pb7_fm(value: u8) { ::write(REGISTER_ADDRESS, I2C_PB7_FM_BIT_OFFSET, I2C_PB7_FM_BIT_WIDTH, value as u32); }

	const I2C_PB8_FM_BIT_OFFSET: u8 = 18;
	const I2C_PB8_FM_BIT_WIDTH: u8 = 1;
	/// Fast Mode Plus (FM+) driving capability activation bits. (Width: 1, Offset: 18)
	pub fn get_i2c_pb8_fm() -> u8 { ::read(REGISTER_ADDRESS, I2C_PB8_FM_BIT_OFFSET, I2C_PB8_FM_BIT_WIDTH) as u8 }
	/// Fast Mode Plus (FM+) driving capability activation bits. (Width: 1, Offset: 18)
	pub fn set_i2c_pb8_fm(value: u8) { ::write(REGISTER_ADDRESS, I2C_PB8_FM_BIT_OFFSET, I2C_PB8_FM_BIT_WIDTH, value as u32); }

	const I2C_PB9_FM_BIT_OFFSET: u8 = 19;
	const I2C_PB9_FM_BIT_WIDTH: u8 = 1;
	/// Fast Mode Plus (FM+) driving capability activation bits. (Width: 1, Offset: 19)
	pub fn get_i2c_pb9_fm() -> u8 { ::read(REGISTER_ADDRESS, I2C_PB9_FM_BIT_OFFSET, I2C_PB9_FM_BIT_WIDTH) as u8 }
	/// Fast Mode Plus (FM+) driving capability activation bits. (Width: 1, Offset: 19)
	pub fn set_i2c_pb9_fm(value: u8) { ::write(REGISTER_ADDRESS, I2C_PB9_FM_BIT_OFFSET, I2C_PB9_FM_BIT_WIDTH, value as u32); }

	const I2C1_FM_BIT_OFFSET: u8 = 20;
	const I2C1_FM_BIT_WIDTH: u8 = 1;
	/// I2C1 Fast Mode Plus (Width: 1, Offset: 20)
	pub fn get_i2c1_fm() -> u8 { ::read(REGISTER_ADDRESS, I2C1_FM_BIT_OFFSET, I2C1_FM_BIT_WIDTH) as u8 }
	/// I2C1 Fast Mode Plus (Width: 1, Offset: 20)
	pub fn set_i2c1_fm(value: u8) { ::write(REGISTER_ADDRESS, I2C1_FM_BIT_OFFSET, I2C1_FM_BIT_WIDTH, value as u32); }

	const I2C2_FM_BIT_OFFSET: u8 = 21;
	const I2C2_FM_BIT_WIDTH: u8 = 1;
	/// I2C2 Fast Mode Plus (Width: 1, Offset: 21)
	pub fn get_i2c2_fm() -> u8 { ::read(REGISTER_ADDRESS, I2C2_FM_BIT_OFFSET, I2C2_FM_BIT_WIDTH) as u8 }
	/// I2C2 Fast Mode Plus (Width: 1, Offset: 21)
	pub fn set_i2c2_fm(value: u8) { ::write(REGISTER_ADDRESS, I2C2_FM_BIT_OFFSET, I2C2_FM_BIT_WIDTH, value as u32); }

	const ENCODER_MODE_BIT_OFFSET: u8 = 22;
	const ENCODER_MODE_BIT_WIDTH: u8 = 2;
	/// Encoder mode (Width: 2, Offset: 22)
	pub fn get_encoder_mode() -> u8 { ::read(REGISTER_ADDRESS, ENCODER_MODE_BIT_OFFSET, ENCODER_MODE_BIT_WIDTH) as u8 }
	/// Encoder mode (Width: 2, Offset: 22)
	pub fn set_encoder_mode(value: u8) { ::write(REGISTER_ADDRESS, ENCODER_MODE_BIT_OFFSET, ENCODER_MODE_BIT_WIDTH, value as u32); }

	const FPU_IT_BIT_OFFSET: u8 = 26;
	const FPU_IT_BIT_WIDTH: u8 = 6;
	/// Interrupt enable bits from FPU (Width: 6, Offset: 26)
	pub fn get_fpu_it() -> u8 { ::read(REGISTER_ADDRESS, FPU_IT_BIT_OFFSET, FPU_IT_BIT_WIDTH) as u8 }
	/// Interrupt enable bits from FPU (Width: 6, Offset: 26)
	pub fn set_fpu_it(value: u8) { ::write(REGISTER_ADDRESS, FPU_IT_BIT_OFFSET, FPU_IT_BIT_WIDTH, value as u32); }
}
/// external interrupt configuration register 1
/// Size: 0x20 bits
pub mod exticr1 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EXTI3_BIT_OFFSET: u8 = 12;
	const EXTI3_BIT_WIDTH: u8 = 4;
	/// EXTI 3 configuration bits (Width: 4, Offset: 12)
	pub fn get_exti3() -> u8 { ::read(REGISTER_ADDRESS, EXTI3_BIT_OFFSET, EXTI3_BIT_WIDTH) as u8 }
	/// EXTI 3 configuration bits (Width: 4, Offset: 12)
	pub fn set_exti3(value: u8) { ::write(REGISTER_ADDRESS, EXTI3_BIT_OFFSET, EXTI3_BIT_WIDTH, value as u32); }

	const EXTI2_BIT_OFFSET: u8 = 8;
	const EXTI2_BIT_WIDTH: u8 = 4;
	/// EXTI 2 configuration bits (Width: 4, Offset: 8)
	pub fn get_exti2() -> u8 { ::read(REGISTER_ADDRESS, EXTI2_BIT_OFFSET, EXTI2_BIT_WIDTH) as u8 }
	/// EXTI 2 configuration bits (Width: 4, Offset: 8)
	pub fn set_exti2(value: u8) { ::write(REGISTER_ADDRESS, EXTI2_BIT_OFFSET, EXTI2_BIT_WIDTH, value as u32); }

	const EXTI1_BIT_OFFSET: u8 = 4;
	const EXTI1_BIT_WIDTH: u8 = 4;
	/// EXTI 1 configuration bits (Width: 4, Offset: 4)
	pub fn get_exti1() -> u8 { ::read(REGISTER_ADDRESS, EXTI1_BIT_OFFSET, EXTI1_BIT_WIDTH) as u8 }
	/// EXTI 1 configuration bits (Width: 4, Offset: 4)
	pub fn set_exti1(value: u8) { ::write(REGISTER_ADDRESS, EXTI1_BIT_OFFSET, EXTI1_BIT_WIDTH, value as u32); }

	const EXTI0_BIT_OFFSET: u8 = 0;
	const EXTI0_BIT_WIDTH: u8 = 4;
	/// EXTI 0 configuration bits (Width: 4, Offset: 0)
	pub fn get_exti0() -> u8 { ::read(REGISTER_ADDRESS, EXTI0_BIT_OFFSET, EXTI0_BIT_WIDTH) as u8 }
	/// EXTI 0 configuration bits (Width: 4, Offset: 0)
	pub fn set_exti0(value: u8) { ::write(REGISTER_ADDRESS, EXTI0_BIT_OFFSET, EXTI0_BIT_WIDTH, value as u32); }
}
/// external interrupt configuration register 2
/// Size: 0x20 bits
pub mod exticr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EXTI7_BIT_OFFSET: u8 = 12;
	const EXTI7_BIT_WIDTH: u8 = 4;
	/// EXTI 7 configuration bits (Width: 4, Offset: 12)
	pub fn get_exti7() -> u8 { ::read(REGISTER_ADDRESS, EXTI7_BIT_OFFSET, EXTI7_BIT_WIDTH) as u8 }
	/// EXTI 7 configuration bits (Width: 4, Offset: 12)
	pub fn set_exti7(value: u8) { ::write(REGISTER_ADDRESS, EXTI7_BIT_OFFSET, EXTI7_BIT_WIDTH, value as u32); }

	const EXTI6_BIT_OFFSET: u8 = 8;
	const EXTI6_BIT_WIDTH: u8 = 4;
	/// EXTI 6 configuration bits (Width: 4, Offset: 8)
	pub fn get_exti6() -> u8 { ::read(REGISTER_ADDRESS, EXTI6_BIT_OFFSET, EXTI6_BIT_WIDTH) as u8 }
	/// EXTI 6 configuration bits (Width: 4, Offset: 8)
	pub fn set_exti6(value: u8) { ::write(REGISTER_ADDRESS, EXTI6_BIT_OFFSET, EXTI6_BIT_WIDTH, value as u32); }

	const EXTI5_BIT_OFFSET: u8 = 4;
	const EXTI5_BIT_WIDTH: u8 = 4;
	/// EXTI 5 configuration bits (Width: 4, Offset: 4)
	pub fn get_exti5() -> u8 { ::read(REGISTER_ADDRESS, EXTI5_BIT_OFFSET, EXTI5_BIT_WIDTH) as u8 }
	/// EXTI 5 configuration bits (Width: 4, Offset: 4)
	pub fn set_exti5(value: u8) { ::write(REGISTER_ADDRESS, EXTI5_BIT_OFFSET, EXTI5_BIT_WIDTH, value as u32); }

	const EXTI4_BIT_OFFSET: u8 = 0;
	const EXTI4_BIT_WIDTH: u8 = 4;
	/// EXTI 4 configuration bits (Width: 4, Offset: 0)
	pub fn get_exti4() -> u8 { ::read(REGISTER_ADDRESS, EXTI4_BIT_OFFSET, EXTI4_BIT_WIDTH) as u8 }
	/// EXTI 4 configuration bits (Width: 4, Offset: 0)
	pub fn set_exti4(value: u8) { ::write(REGISTER_ADDRESS, EXTI4_BIT_OFFSET, EXTI4_BIT_WIDTH, value as u32); }
}
/// external interrupt configuration register 3
/// Size: 0x20 bits
pub mod exticr3 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EXTI11_BIT_OFFSET: u8 = 12;
	const EXTI11_BIT_WIDTH: u8 = 4;
	/// EXTI 11 configuration bits (Width: 4, Offset: 12)
	pub fn get_exti11() -> u8 { ::read(REGISTER_ADDRESS, EXTI11_BIT_OFFSET, EXTI11_BIT_WIDTH) as u8 }
	/// EXTI 11 configuration bits (Width: 4, Offset: 12)
	pub fn set_exti11(value: u8) { ::write(REGISTER_ADDRESS, EXTI11_BIT_OFFSET, EXTI11_BIT_WIDTH, value as u32); }

	const EXTI10_BIT_OFFSET: u8 = 8;
	const EXTI10_BIT_WIDTH: u8 = 4;
	/// EXTI 10 configuration bits (Width: 4, Offset: 8)
	pub fn get_exti10() -> u8 { ::read(REGISTER_ADDRESS, EXTI10_BIT_OFFSET, EXTI10_BIT_WIDTH) as u8 }
	/// EXTI 10 configuration bits (Width: 4, Offset: 8)
	pub fn set_exti10(value: u8) { ::write(REGISTER_ADDRESS, EXTI10_BIT_OFFSET, EXTI10_BIT_WIDTH, value as u32); }

	const EXTI9_BIT_OFFSET: u8 = 4;
	const EXTI9_BIT_WIDTH: u8 = 4;
	/// EXTI 9 configuration bits (Width: 4, Offset: 4)
	pub fn get_exti9() -> u8 { ::read(REGISTER_ADDRESS, EXTI9_BIT_OFFSET, EXTI9_BIT_WIDTH) as u8 }
	/// EXTI 9 configuration bits (Width: 4, Offset: 4)
	pub fn set_exti9(value: u8) { ::write(REGISTER_ADDRESS, EXTI9_BIT_OFFSET, EXTI9_BIT_WIDTH, value as u32); }

	const EXTI8_BIT_OFFSET: u8 = 0;
	const EXTI8_BIT_WIDTH: u8 = 4;
	/// EXTI 8 configuration bits (Width: 4, Offset: 0)
	pub fn get_exti8() -> u8 { ::read(REGISTER_ADDRESS, EXTI8_BIT_OFFSET, EXTI8_BIT_WIDTH) as u8 }
	/// EXTI 8 configuration bits (Width: 4, Offset: 0)
	pub fn set_exti8(value: u8) { ::write(REGISTER_ADDRESS, EXTI8_BIT_OFFSET, EXTI8_BIT_WIDTH, value as u32); }
}
/// external interrupt configuration register 4
/// Size: 0x20 bits
pub mod exticr4 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const EXTI15_BIT_OFFSET: u8 = 12;
	const EXTI15_BIT_WIDTH: u8 = 4;
	/// EXTI 15 configuration bits (Width: 4, Offset: 12)
	pub fn get_exti15() -> u8 { ::read(REGISTER_ADDRESS, EXTI15_BIT_OFFSET, EXTI15_BIT_WIDTH) as u8 }
	/// EXTI 15 configuration bits (Width: 4, Offset: 12)
	pub fn set_exti15(value: u8) { ::write(REGISTER_ADDRESS, EXTI15_BIT_OFFSET, EXTI15_BIT_WIDTH, value as u32); }

	const EXTI14_BIT_OFFSET: u8 = 8;
	const EXTI14_BIT_WIDTH: u8 = 4;
	/// EXTI 14 configuration bits (Width: 4, Offset: 8)
	pub fn get_exti14() -> u8 { ::read(REGISTER_ADDRESS, EXTI14_BIT_OFFSET, EXTI14_BIT_WIDTH) as u8 }
	/// EXTI 14 configuration bits (Width: 4, Offset: 8)
	pub fn set_exti14(value: u8) { ::write(REGISTER_ADDRESS, EXTI14_BIT_OFFSET, EXTI14_BIT_WIDTH, value as u32); }

	const EXTI13_BIT_OFFSET: u8 = 4;
	const EXTI13_BIT_WIDTH: u8 = 4;
	/// EXTI 13 configuration bits (Width: 4, Offset: 4)
	pub fn get_exti13() -> u8 { ::read(REGISTER_ADDRESS, EXTI13_BIT_OFFSET, EXTI13_BIT_WIDTH) as u8 }
	/// EXTI 13 configuration bits (Width: 4, Offset: 4)
	pub fn set_exti13(value: u8) { ::write(REGISTER_ADDRESS, EXTI13_BIT_OFFSET, EXTI13_BIT_WIDTH, value as u32); }

	const EXTI12_BIT_OFFSET: u8 = 0;
	const EXTI12_BIT_WIDTH: u8 = 4;
	/// EXTI 12 configuration bits (Width: 4, Offset: 0)
	pub fn get_exti12() -> u8 { ::read(REGISTER_ADDRESS, EXTI12_BIT_OFFSET, EXTI12_BIT_WIDTH) as u8 }
	/// EXTI 12 configuration bits (Width: 4, Offset: 0)
	pub fn set_exti12(value: u8) { ::write(REGISTER_ADDRESS, EXTI12_BIT_OFFSET, EXTI12_BIT_WIDTH, value as u32); }
}
/// configuration register 2
/// Size: 0x20 bits
pub mod cfgr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const LOCUP_LOCK_BIT_OFFSET: u8 = 0;
	const LOCUP_LOCK_BIT_WIDTH: u8 = 1;
	/// Cortex-M0 LOCKUP bit enable bit (Width: 1, Offset: 0)
	pub fn get_locup_lock() -> u8 { ::read(REGISTER_ADDRESS, LOCUP_LOCK_BIT_OFFSET, LOCUP_LOCK_BIT_WIDTH) as u8 }
	/// Cortex-M0 LOCKUP bit enable bit (Width: 1, Offset: 0)
	pub fn set_locup_lock(value: u8) { ::write(REGISTER_ADDRESS, LOCUP_LOCK_BIT_OFFSET, LOCUP_LOCK_BIT_WIDTH, value as u32); }

	const SRAM_PARITY_LOCK_BIT_OFFSET: u8 = 1;
	const SRAM_PARITY_LOCK_BIT_WIDTH: u8 = 1;
	/// SRAM parity lock bit (Width: 1, Offset: 1)
	pub fn get_sram_parity_lock() -> u8 { ::read(REGISTER_ADDRESS, SRAM_PARITY_LOCK_BIT_OFFSET, SRAM_PARITY_LOCK_BIT_WIDTH) as u8 }
	/// SRAM parity lock bit (Width: 1, Offset: 1)
	pub fn set_sram_parity_lock(value: u8) { ::write(REGISTER_ADDRESS, SRAM_PARITY_LOCK_BIT_OFFSET, SRAM_PARITY_LOCK_BIT_WIDTH, value as u32); }

	const PVD_LOCK_BIT_OFFSET: u8 = 2;
	const PVD_LOCK_BIT_WIDTH: u8 = 1;
	/// PVD lock enable bit (Width: 1, Offset: 2)
	pub fn get_pvd_lock() -> u8 { ::read(REGISTER_ADDRESS, PVD_LOCK_BIT_OFFSET, PVD_LOCK_BIT_WIDTH) as u8 }
	/// PVD lock enable bit (Width: 1, Offset: 2)
	pub fn set_pvd_lock(value: u8) { ::write(REGISTER_ADDRESS, PVD_LOCK_BIT_OFFSET, PVD_LOCK_BIT_WIDTH, value as u32); }

	const BYP_ADD_PAR_BIT_OFFSET: u8 = 4;
	const BYP_ADD_PAR_BIT_WIDTH: u8 = 1;
	/// Bypass address bit 29 in parity calculation (Width: 1, Offset: 4)
	pub fn get_byp_add_par() -> u8 { ::read(REGISTER_ADDRESS, BYP_ADD_PAR_BIT_OFFSET, BYP_ADD_PAR_BIT_WIDTH) as u8 }
	/// Bypass address bit 29 in parity calculation (Width: 1, Offset: 4)
	pub fn set_byp_add_par(value: u8) { ::write(REGISTER_ADDRESS, BYP_ADD_PAR_BIT_OFFSET, BYP_ADD_PAR_BIT_WIDTH, value as u32); }

	const SRAM_PEF_BIT_OFFSET: u8 = 8;
	const SRAM_PEF_BIT_WIDTH: u8 = 1;
	/// SRAM parity flag (Width: 1, Offset: 8)
	pub fn get_sram_pef() -> u8 { ::read(REGISTER_ADDRESS, SRAM_PEF_BIT_OFFSET, SRAM_PEF_BIT_WIDTH) as u8 }
	/// SRAM parity flag (Width: 1, Offset: 8)
	pub fn set_sram_pef(value: u8) { ::write(REGISTER_ADDRESS, SRAM_PEF_BIT_OFFSET, SRAM_PEF_BIT_WIDTH, value as u32); }
}
/// CCM SRAM protection register
/// Size: 0x20 bits
pub mod rcr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PAGE0_WP_BIT_OFFSET: u8 = 0;
	const PAGE0_WP_BIT_WIDTH: u8 = 1;
	/// CCM SRAM page write protection bit (Width: 1, Offset: 0)
	pub fn get_page0_wp() -> u8 { ::read(REGISTER_ADDRESS, PAGE0_WP_BIT_OFFSET, PAGE0_WP_BIT_WIDTH) as u8 }
	/// CCM SRAM page write protection bit (Width: 1, Offset: 0)
	pub fn set_page0_wp(value: u8) { ::write(REGISTER_ADDRESS, PAGE0_WP_BIT_OFFSET, PAGE0_WP_BIT_WIDTH, value as u32); }

	const PAGE1_WP_BIT_OFFSET: u8 = 1;
	const PAGE1_WP_BIT_WIDTH: u8 = 1;
	/// CCM SRAM page write protection bit (Width: 1, Offset: 1)
	pub fn get_page1_wp() -> u8 { ::read(REGISTER_ADDRESS, PAGE1_WP_BIT_OFFSET, PAGE1_WP_BIT_WIDTH) as u8 }
	/// CCM SRAM page write protection bit (Width: 1, Offset: 1)
	pub fn set_page1_wp(value: u8) { ::write(REGISTER_ADDRESS, PAGE1_WP_BIT_OFFSET, PAGE1_WP_BIT_WIDTH, value as u32); }

	const PAGE2_WP_BIT_OFFSET: u8 = 2;
	const PAGE2_WP_BIT_WIDTH: u8 = 1;
	/// CCM SRAM page write protection bit (Width: 1, Offset: 2)
	pub fn get_page2_wp() -> u8 { ::read(REGISTER_ADDRESS, PAGE2_WP_BIT_OFFSET, PAGE2_WP_BIT_WIDTH) as u8 }
	/// CCM SRAM page write protection bit (Width: 1, Offset: 2)
	pub fn set_page2_wp(value: u8) { ::write(REGISTER_ADDRESS, PAGE2_WP_BIT_OFFSET, PAGE2_WP_BIT_WIDTH, value as u32); }

	const PAGE3_WP_BIT_OFFSET: u8 = 3;
	const PAGE3_WP_BIT_WIDTH: u8 = 1;
	/// CCM SRAM page write protection bit (Width: 1, Offset: 3)
	pub fn get_page3_wp() -> u8 { ::read(REGISTER_ADDRESS, PAGE3_WP_BIT_OFFSET, PAGE3_WP_BIT_WIDTH) as u8 }
	/// CCM SRAM page write protection bit (Width: 1, Offset: 3)
	pub fn set_page3_wp(value: u8) { ::write(REGISTER_ADDRESS, PAGE3_WP_BIT_OFFSET, PAGE3_WP_BIT_WIDTH, value as u32); }

	const PAGE4_WP_BIT_OFFSET: u8 = 4;
	const PAGE4_WP_BIT_WIDTH: u8 = 1;
	/// CCM SRAM page write protection bit (Width: 1, Offset: 4)
	pub fn get_page4_wp() -> u8 { ::read(REGISTER_ADDRESS, PAGE4_WP_BIT_OFFSET, PAGE4_WP_BIT_WIDTH) as u8 }
	/// CCM SRAM page write protection bit (Width: 1, Offset: 4)
	pub fn set_page4_wp(value: u8) { ::write(REGISTER_ADDRESS, PAGE4_WP_BIT_OFFSET, PAGE4_WP_BIT_WIDTH, value as u32); }

	const PAGE5_WP_BIT_OFFSET: u8 = 5;
	const PAGE5_WP_BIT_WIDTH: u8 = 1;
	/// CCM SRAM page write protection bit (Width: 1, Offset: 5)
	pub fn get_page5_wp() -> u8 { ::read(REGISTER_ADDRESS, PAGE5_WP_BIT_OFFSET, PAGE5_WP_BIT_WIDTH) as u8 }
	/// CCM SRAM page write protection bit (Width: 1, Offset: 5)
	pub fn set_page5_wp(value: u8) { ::write(REGISTER_ADDRESS, PAGE5_WP_BIT_OFFSET, PAGE5_WP_BIT_WIDTH, value as u32); }

	const PAGE6_WP_BIT_OFFSET: u8 = 6;
	const PAGE6_WP_BIT_WIDTH: u8 = 1;
	/// CCM SRAM page write protection bit (Width: 1, Offset: 6)
	pub fn get_page6_wp() -> u8 { ::read(REGISTER_ADDRESS, PAGE6_WP_BIT_OFFSET, PAGE6_WP_BIT_WIDTH) as u8 }
	/// CCM SRAM page write protection bit (Width: 1, Offset: 6)
	pub fn set_page6_wp(value: u8) { ::write(REGISTER_ADDRESS, PAGE6_WP_BIT_OFFSET, PAGE6_WP_BIT_WIDTH, value as u32); }

	const PAGE7_WP_BIT_OFFSET: u8 = 7;
	const PAGE7_WP_BIT_WIDTH: u8 = 1;
	/// CCM SRAM page write protection bit (Width: 1, Offset: 7)
	pub fn get_page7_wp() -> u8 { ::read(REGISTER_ADDRESS, PAGE7_WP_BIT_OFFSET, PAGE7_WP_BIT_WIDTH) as u8 }
	/// CCM SRAM page write protection bit (Width: 1, Offset: 7)
	pub fn set_page7_wp(value: u8) { ::write(REGISTER_ADDRESS, PAGE7_WP_BIT_OFFSET, PAGE7_WP_BIT_WIDTH, value as u32); }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>SYSCFG</name>
  <description>System configuration controller</description>
  <groupName>SYSCFG</groupName>
  <baseAddress>0x40010000</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x19</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>CFGR1</name>
      <displayName>CFGR1</displayName>
      <description>configuration register 1</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>MEM_MODE</name>
          <description>Memory mapping selection
              bits</description>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>USB_IT_RMP</name>
          <description>USB interrupt remap</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM1_ITR_RMP</name>
          <description>Timer 1 ITR3 selection</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DAC_TRIG_RMP</name>
          <description>DAC trigger remap (when TSEL =
              001)</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADC24_DMA_RMP</name>
          <description>ADC24 DMA remapping bit</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM16_DMA_RMP</name>
          <description>TIM16 DMA request remapping
              bit</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM17_DMA_RMP</name>
          <description>TIM17 DMA request remapping
              bit</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM6_DAC1_DMA_RMP</name>
          <description>TIM6 and DAC1 DMA request remapping
              bit</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM7_DAC2_DMA_RMP</name>
          <description>TIM7 and DAC2 DMA request remapping
              bit</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2C_PB6_FM</name>
          <description>Fast Mode Plus (FM+) driving capability
              activation bits.</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2C_PB7_FM</name>
          <description>Fast Mode Plus (FM+) driving capability
              activation bits.</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2C_PB8_FM</name>
          <description>Fast Mode Plus (FM+) driving capability
              activation bits.</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2C_PB9_FM</name>
          <description>Fast Mode Plus (FM+) driving capability
              activation bits.</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2C1_FM</name>
          <description>I2C1 Fast Mode Plus</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2C2_FM</name>
          <description>I2C2 Fast Mode Plus</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ENCODER_MODE</name>
          <description>Encoder mode</description>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>FPU_IT</name>
          <description>Interrupt enable bits from
              FPU</description>
          <bitOffset>26</bitOffset>
          <bitWidth>6</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>EXTICR1</name>
      <displayName>EXTICR1</displayName>
      <description>external interrupt configuration register
          1</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>EXTI3</name>
          <description>EXTI 3 configuration bits</description>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>EXTI2</name>
          <description>EXTI 2 configuration bits</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>EXTI1</name>
          <description>EXTI 1 configuration bits</description>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>EXTI0</name>
          <description>EXTI 0 configuration bits</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>EXTICR2</name>
      <displayName>EXTICR2</displayName>
      <description>external interrupt configuration register
          2</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>EXTI7</name>
          <description>EXTI 7 configuration bits</description>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>EXTI6</name>
          <description>EXTI 6 configuration bits</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>EXTI5</name>
          <description>EXTI 5 configuration bits</description>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>EXTI4</name>
          <description>EXTI 4 configuration bits</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>EXTICR3</name>
      <displayName>EXTICR3</displayName>
      <description>external interrupt configuration register
          3</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>EXTI11</name>
          <description>EXTI 11 configuration bits</description>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>EXTI10</name>
          <description>EXTI 10 configuration bits</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>EXTI9</name>
          <description>EXTI 9 configuration bits</description>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>EXTI8</name>
          <description>EXTI 8 configuration bits</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>EXTICR4</name>
      <displayName>EXTICR4</displayName>
      <description>external interrupt configuration register
          4</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>EXTI15</name>
          <description>EXTI 15 configuration bits</description>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>EXTI14</name>
          <description>EXTI 14 configuration bits</description>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>EXTI13</name>
          <description>EXTI 13 configuration bits</description>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>EXTI12</name>
          <description>EXTI 12 configuration bits</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CFGR2</name>
      <displayName>CFGR2</displayName>
      <description>configuration register 2</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>LOCUP_LOCK</name>
          <description>Cortex-M0 LOCKUP bit enable
              bit</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SRAM_PARITY_LOCK</name>
          <description>SRAM parity lock bit</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PVD_LOCK</name>
          <description>PVD lock enable bit</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>BYP_ADD_PAR</name>
          <description>Bypass address bit 29 in parity
              calculation</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SRAM_PEF</name>
          <description>SRAM parity flag</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>RCR</name>
      <displayName>RCR</displayName>
      <description>CCM SRAM protection register</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x0000</resetValue>
      <fields>
        <field>
          <name>PAGE0_WP</name>
          <description>CCM SRAM page write protection
              bit</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PAGE1_WP</name>
          <description>CCM SRAM page write protection
              bit</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PAGE2_WP</name>
          <description>CCM SRAM page write protection
              bit</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PAGE3_WP</name>
          <description>CCM SRAM page write protection
              bit</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PAGE4_WP</name>
          <description>CCM SRAM page write protection
              bit</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PAGE5_WP</name>
          <description>CCM SRAM page write protection
              bit</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PAGE6_WP</name>
          <description>CCM SRAM page write protection
              bit</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PAGE7_WP</name>
          <description>CCM SRAM page write protection
              bit</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
</peripheral>*/
