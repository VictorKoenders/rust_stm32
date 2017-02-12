/// MOD RCC
/// Reset and clock control
const BASE_ADDRESS: u32 = 0x40021000;
/// Clock control register
/// Size: 0x20 bits
pub mod cr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x0;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const HSION_BIT_OFFSET: u8 = 0;
	const HSION_BIT_WIDTH: u8 = 1;
	/// Internal High Speed clock enable (Width: 1, Offset: 0)
	pub fn get_hsion() -> u8 { ::read(REGISTER_ADDRESS, HSION_BIT_OFFSET, HSION_BIT_WIDTH) as u8 }
	/// Internal High Speed clock enable (Width: 1, Offset: 0)
	pub fn set_hsion(value: u8) { ::write(REGISTER_ADDRESS, HSION_BIT_OFFSET, HSION_BIT_WIDTH, value as u32); }

	const HSIRDY_BIT_OFFSET: u8 = 1;
	const HSIRDY_BIT_WIDTH: u8 = 1;
	/// Internal High Speed clock ready flag (Width: 1, Offset: 1)
	pub fn get_hsirdy() -> u8 { ::read(REGISTER_ADDRESS, HSIRDY_BIT_OFFSET, HSIRDY_BIT_WIDTH) as u8 }

	const HSITRIM_BIT_OFFSET: u8 = 3;
	const HSITRIM_BIT_WIDTH: u8 = 5;
	/// Internal High Speed clock trimming (Width: 5, Offset: 3)
	pub fn get_hsitrim() -> u8 { ::read(REGISTER_ADDRESS, HSITRIM_BIT_OFFSET, HSITRIM_BIT_WIDTH) as u8 }
	/// Internal High Speed clock trimming (Width: 5, Offset: 3)
	pub fn set_hsitrim(value: u8) { ::write(REGISTER_ADDRESS, HSITRIM_BIT_OFFSET, HSITRIM_BIT_WIDTH, value as u32); }

	const HSICAL_BIT_OFFSET: u8 = 8;
	const HSICAL_BIT_WIDTH: u8 = 8;
	/// Internal High Speed clock Calibration (Width: 8, Offset: 8)
	pub fn get_hsical() -> u8 { ::read(REGISTER_ADDRESS, HSICAL_BIT_OFFSET, HSICAL_BIT_WIDTH) as u8 }

	const HSEON_BIT_OFFSET: u8 = 16;
	const HSEON_BIT_WIDTH: u8 = 1;
	/// External High Speed clock enable (Width: 1, Offset: 16)
	pub fn get_hseon() -> u8 { ::read(REGISTER_ADDRESS, HSEON_BIT_OFFSET, HSEON_BIT_WIDTH) as u8 }
	/// External High Speed clock enable (Width: 1, Offset: 16)
	pub fn set_hseon(value: u8) { ::write(REGISTER_ADDRESS, HSEON_BIT_OFFSET, HSEON_BIT_WIDTH, value as u32); }

	const HSERDY_BIT_OFFSET: u8 = 17;
	const HSERDY_BIT_WIDTH: u8 = 1;
	/// External High Speed clock ready flag (Width: 1, Offset: 17)
	pub fn get_hserdy() -> u8 { ::read(REGISTER_ADDRESS, HSERDY_BIT_OFFSET, HSERDY_BIT_WIDTH) as u8 }

	const HSEBYP_BIT_OFFSET: u8 = 18;
	const HSEBYP_BIT_WIDTH: u8 = 1;
	/// External High Speed clock Bypass (Width: 1, Offset: 18)
	pub fn get_hsebyp() -> u8 { ::read(REGISTER_ADDRESS, HSEBYP_BIT_OFFSET, HSEBYP_BIT_WIDTH) as u8 }
	/// External High Speed clock Bypass (Width: 1, Offset: 18)
	pub fn set_hsebyp(value: u8) { ::write(REGISTER_ADDRESS, HSEBYP_BIT_OFFSET, HSEBYP_BIT_WIDTH, value as u32); }

	const CSSON_BIT_OFFSET: u8 = 19;
	const CSSON_BIT_WIDTH: u8 = 1;
	/// Clock Security System enable (Width: 1, Offset: 19)
	pub fn get_csson() -> u8 { ::read(REGISTER_ADDRESS, CSSON_BIT_OFFSET, CSSON_BIT_WIDTH) as u8 }
	/// Clock Security System enable (Width: 1, Offset: 19)
	pub fn set_csson(value: u8) { ::write(REGISTER_ADDRESS, CSSON_BIT_OFFSET, CSSON_BIT_WIDTH, value as u32); }

	const PLLON_BIT_OFFSET: u8 = 24;
	const PLLON_BIT_WIDTH: u8 = 1;
	/// PLL enable (Width: 1, Offset: 24)
	pub fn get_pllon() -> u8 { ::read(REGISTER_ADDRESS, PLLON_BIT_OFFSET, PLLON_BIT_WIDTH) as u8 }
	/// PLL enable (Width: 1, Offset: 24)
	pub fn set_pllon(value: u8) { ::write(REGISTER_ADDRESS, PLLON_BIT_OFFSET, PLLON_BIT_WIDTH, value as u32); }

	const PLLRDY_BIT_OFFSET: u8 = 25;
	const PLLRDY_BIT_WIDTH: u8 = 1;
	/// PLL clock ready flag (Width: 1, Offset: 25)
	pub fn get_pllrdy() -> u8 { ::read(REGISTER_ADDRESS, PLLRDY_BIT_OFFSET, PLLRDY_BIT_WIDTH) as u8 }
}
/// Clock configuration register (RCC_CFGR)
/// Size: 0x20 bits
pub mod cfgr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x4;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SW_BIT_OFFSET: u8 = 0;
	const SW_BIT_WIDTH: u8 = 2;
	/// System clock Switch (Width: 2, Offset: 0)
	pub fn get_sw() -> u8 { ::read(REGISTER_ADDRESS, SW_BIT_OFFSET, SW_BIT_WIDTH) as u8 }
	/// System clock Switch (Width: 2, Offset: 0)
	pub fn set_sw(value: u8) { ::write(REGISTER_ADDRESS, SW_BIT_OFFSET, SW_BIT_WIDTH, value as u32); }

	const SWS_BIT_OFFSET: u8 = 2;
	const SWS_BIT_WIDTH: u8 = 2;
	/// System Clock Switch Status (Width: 2, Offset: 2)
	pub fn get_sws() -> u8 { ::read(REGISTER_ADDRESS, SWS_BIT_OFFSET, SWS_BIT_WIDTH) as u8 }

	const HPRE_BIT_OFFSET: u8 = 4;
	const HPRE_BIT_WIDTH: u8 = 4;
	/// AHB prescaler (Width: 4, Offset: 4)
	pub fn get_hpre() -> u8 { ::read(REGISTER_ADDRESS, HPRE_BIT_OFFSET, HPRE_BIT_WIDTH) as u8 }
	/// AHB prescaler (Width: 4, Offset: 4)
	pub fn set_hpre(value: u8) { ::write(REGISTER_ADDRESS, HPRE_BIT_OFFSET, HPRE_BIT_WIDTH, value as u32); }

	const PPRE1_BIT_OFFSET: u8 = 8;
	const PPRE1_BIT_WIDTH: u8 = 3;
	/// APB Low speed prescaler (APB1) (Width: 3, Offset: 8)
	pub fn get_ppre1() -> u8 { ::read(REGISTER_ADDRESS, PPRE1_BIT_OFFSET, PPRE1_BIT_WIDTH) as u8 }
	/// APB Low speed prescaler (APB1) (Width: 3, Offset: 8)
	pub fn set_ppre1(value: u8) { ::write(REGISTER_ADDRESS, PPRE1_BIT_OFFSET, PPRE1_BIT_WIDTH, value as u32); }

	const PPRE2_BIT_OFFSET: u8 = 11;
	const PPRE2_BIT_WIDTH: u8 = 3;
	/// APB high speed prescaler (APB2) (Width: 3, Offset: 11)
	pub fn get_ppre2() -> u8 { ::read(REGISTER_ADDRESS, PPRE2_BIT_OFFSET, PPRE2_BIT_WIDTH) as u8 }
	/// APB high speed prescaler (APB2) (Width: 3, Offset: 11)
	pub fn set_ppre2(value: u8) { ::write(REGISTER_ADDRESS, PPRE2_BIT_OFFSET, PPRE2_BIT_WIDTH, value as u32); }

	const PLLSRC_BIT_OFFSET: u8 = 16;
	const PLLSRC_BIT_WIDTH: u8 = 1;
	/// PLL entry clock source (Width: 1, Offset: 16)
	pub fn get_pllsrc() -> u8 { ::read(REGISTER_ADDRESS, PLLSRC_BIT_OFFSET, PLLSRC_BIT_WIDTH) as u8 }
	/// PLL entry clock source (Width: 1, Offset: 16)
	pub fn set_pllsrc(value: u8) { ::write(REGISTER_ADDRESS, PLLSRC_BIT_OFFSET, PLLSRC_BIT_WIDTH, value as u32); }

	const PLLXTPRE_BIT_OFFSET: u8 = 17;
	const PLLXTPRE_BIT_WIDTH: u8 = 1;
	/// HSE divider for PLL entry (Width: 1, Offset: 17)
	pub fn get_pllxtpre() -> u8 { ::read(REGISTER_ADDRESS, PLLXTPRE_BIT_OFFSET, PLLXTPRE_BIT_WIDTH) as u8 }
	/// HSE divider for PLL entry (Width: 1, Offset: 17)
	pub fn set_pllxtpre(value: u8) { ::write(REGISTER_ADDRESS, PLLXTPRE_BIT_OFFSET, PLLXTPRE_BIT_WIDTH, value as u32); }

	const PLLMUL_BIT_OFFSET: u8 = 18;
	const PLLMUL_BIT_WIDTH: u8 = 4;
	/// PLL Multiplication Factor (Width: 4, Offset: 18)
	pub fn get_pllmul() -> u8 { ::read(REGISTER_ADDRESS, PLLMUL_BIT_OFFSET, PLLMUL_BIT_WIDTH) as u8 }
	/// PLL Multiplication Factor (Width: 4, Offset: 18)
	pub fn set_pllmul(value: u8) { ::write(REGISTER_ADDRESS, PLLMUL_BIT_OFFSET, PLLMUL_BIT_WIDTH, value as u32); }

	const USBPRES_BIT_OFFSET: u8 = 22;
	const USBPRES_BIT_WIDTH: u8 = 1;
	/// USB prescaler (Width: 1, Offset: 22)
	pub fn get_usbpres() -> u8 { ::read(REGISTER_ADDRESS, USBPRES_BIT_OFFSET, USBPRES_BIT_WIDTH) as u8 }
	/// USB prescaler (Width: 1, Offset: 22)
	pub fn set_usbpres(value: u8) { ::write(REGISTER_ADDRESS, USBPRES_BIT_OFFSET, USBPRES_BIT_WIDTH, value as u32); }

	const MCO_BIT_OFFSET: u8 = 24;
	const MCO_BIT_WIDTH: u8 = 3;
	/// Microcontroller clock output (Width: 3, Offset: 24)
	pub fn get_mco() -> u8 { ::read(REGISTER_ADDRESS, MCO_BIT_OFFSET, MCO_BIT_WIDTH) as u8 }
	/// Microcontroller clock output (Width: 3, Offset: 24)
	pub fn set_mco(value: u8) { ::write(REGISTER_ADDRESS, MCO_BIT_OFFSET, MCO_BIT_WIDTH, value as u32); }

	const MCOF_BIT_OFFSET: u8 = 28;
	const MCOF_BIT_WIDTH: u8 = 1;
	/// Microcontroller Clock Output Flag (Width: 1, Offset: 28)
	pub fn get_mcof() -> u8 { ::read(REGISTER_ADDRESS, MCOF_BIT_OFFSET, MCOF_BIT_WIDTH) as u8 }

	const I2SSRC_BIT_OFFSET: u8 = 23;
	const I2SSRC_BIT_WIDTH: u8 = 1;
	/// I2S external clock source selection (Width: 1, Offset: 23)
	pub fn get_i2ssrc() -> u8 { ::read(REGISTER_ADDRESS, I2SSRC_BIT_OFFSET, I2SSRC_BIT_WIDTH) as u8 }
	/// I2S external clock source selection (Width: 1, Offset: 23)
	pub fn set_i2ssrc(value: u8) { ::write(REGISTER_ADDRESS, I2SSRC_BIT_OFFSET, I2SSRC_BIT_WIDTH, value as u32); }
}
/// Clock interrupt register (RCC_CIR)
/// Size: 0x20 bits
pub mod cir {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x8;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const LSIRDYF_BIT_OFFSET: u8 = 0;
	const LSIRDYF_BIT_WIDTH: u8 = 1;
	/// LSI Ready Interrupt flag (Width: 1, Offset: 0)
	pub fn get_lsirdyf() -> u8 { ::read(REGISTER_ADDRESS, LSIRDYF_BIT_OFFSET, LSIRDYF_BIT_WIDTH) as u8 }

	const LSERDYF_BIT_OFFSET: u8 = 1;
	const LSERDYF_BIT_WIDTH: u8 = 1;
	/// LSE Ready Interrupt flag (Width: 1, Offset: 1)
	pub fn get_lserdyf() -> u8 { ::read(REGISTER_ADDRESS, LSERDYF_BIT_OFFSET, LSERDYF_BIT_WIDTH) as u8 }

	const HSIRDYF_BIT_OFFSET: u8 = 2;
	const HSIRDYF_BIT_WIDTH: u8 = 1;
	/// HSI Ready Interrupt flag (Width: 1, Offset: 2)
	pub fn get_hsirdyf() -> u8 { ::read(REGISTER_ADDRESS, HSIRDYF_BIT_OFFSET, HSIRDYF_BIT_WIDTH) as u8 }

	const HSERDYF_BIT_OFFSET: u8 = 3;
	const HSERDYF_BIT_WIDTH: u8 = 1;
	/// HSE Ready Interrupt flag (Width: 1, Offset: 3)
	pub fn get_hserdyf() -> u8 { ::read(REGISTER_ADDRESS, HSERDYF_BIT_OFFSET, HSERDYF_BIT_WIDTH) as u8 }

	const PLLRDYF_BIT_OFFSET: u8 = 4;
	const PLLRDYF_BIT_WIDTH: u8 = 1;
	/// PLL Ready Interrupt flag (Width: 1, Offset: 4)
	pub fn get_pllrdyf() -> u8 { ::read(REGISTER_ADDRESS, PLLRDYF_BIT_OFFSET, PLLRDYF_BIT_WIDTH) as u8 }

	const CSSF_BIT_OFFSET: u8 = 7;
	const CSSF_BIT_WIDTH: u8 = 1;
	/// Clock Security System Interrupt flag (Width: 1, Offset: 7)
	pub fn get_cssf() -> u8 { ::read(REGISTER_ADDRESS, CSSF_BIT_OFFSET, CSSF_BIT_WIDTH) as u8 }

	const LSIRDYIE_BIT_OFFSET: u8 = 8;
	const LSIRDYIE_BIT_WIDTH: u8 = 1;
	/// LSI Ready Interrupt Enable (Width: 1, Offset: 8)
	pub fn get_lsirdyie() -> u8 { ::read(REGISTER_ADDRESS, LSIRDYIE_BIT_OFFSET, LSIRDYIE_BIT_WIDTH) as u8 }
	/// LSI Ready Interrupt Enable (Width: 1, Offset: 8)
	pub fn set_lsirdyie(value: u8) { ::write(REGISTER_ADDRESS, LSIRDYIE_BIT_OFFSET, LSIRDYIE_BIT_WIDTH, value as u32); }

	const LSERDYIE_BIT_OFFSET: u8 = 9;
	const LSERDYIE_BIT_WIDTH: u8 = 1;
	/// LSE Ready Interrupt Enable (Width: 1, Offset: 9)
	pub fn get_lserdyie() -> u8 { ::read(REGISTER_ADDRESS, LSERDYIE_BIT_OFFSET, LSERDYIE_BIT_WIDTH) as u8 }
	/// LSE Ready Interrupt Enable (Width: 1, Offset: 9)
	pub fn set_lserdyie(value: u8) { ::write(REGISTER_ADDRESS, LSERDYIE_BIT_OFFSET, LSERDYIE_BIT_WIDTH, value as u32); }

	const HSIRDYIE_BIT_OFFSET: u8 = 10;
	const HSIRDYIE_BIT_WIDTH: u8 = 1;
	/// HSI Ready Interrupt Enable (Width: 1, Offset: 10)
	pub fn get_hsirdyie() -> u8 { ::read(REGISTER_ADDRESS, HSIRDYIE_BIT_OFFSET, HSIRDYIE_BIT_WIDTH) as u8 }
	/// HSI Ready Interrupt Enable (Width: 1, Offset: 10)
	pub fn set_hsirdyie(value: u8) { ::write(REGISTER_ADDRESS, HSIRDYIE_BIT_OFFSET, HSIRDYIE_BIT_WIDTH, value as u32); }

	const HSERDYIE_BIT_OFFSET: u8 = 11;
	const HSERDYIE_BIT_WIDTH: u8 = 1;
	/// HSE Ready Interrupt Enable (Width: 1, Offset: 11)
	pub fn get_hserdyie() -> u8 { ::read(REGISTER_ADDRESS, HSERDYIE_BIT_OFFSET, HSERDYIE_BIT_WIDTH) as u8 }
	/// HSE Ready Interrupt Enable (Width: 1, Offset: 11)
	pub fn set_hserdyie(value: u8) { ::write(REGISTER_ADDRESS, HSERDYIE_BIT_OFFSET, HSERDYIE_BIT_WIDTH, value as u32); }

	const PLLRDYIE_BIT_OFFSET: u8 = 12;
	const PLLRDYIE_BIT_WIDTH: u8 = 1;
	/// PLL Ready Interrupt Enable (Width: 1, Offset: 12)
	pub fn get_pllrdyie() -> u8 { ::read(REGISTER_ADDRESS, PLLRDYIE_BIT_OFFSET, PLLRDYIE_BIT_WIDTH) as u8 }
	/// PLL Ready Interrupt Enable (Width: 1, Offset: 12)
	pub fn set_pllrdyie(value: u8) { ::write(REGISTER_ADDRESS, PLLRDYIE_BIT_OFFSET, PLLRDYIE_BIT_WIDTH, value as u32); }

	const LSIRDYC_BIT_OFFSET: u8 = 16;
	const LSIRDYC_BIT_WIDTH: u8 = 1;
	/// LSI Ready Interrupt Clear (Width: 1, Offset: 16)
	pub fn set_lsirdyc(value: u8) { ::write(REGISTER_ADDRESS, LSIRDYC_BIT_OFFSET, LSIRDYC_BIT_WIDTH, value as u32); }

	const LSERDYC_BIT_OFFSET: u8 = 17;
	const LSERDYC_BIT_WIDTH: u8 = 1;
	/// LSE Ready Interrupt Clear (Width: 1, Offset: 17)
	pub fn set_lserdyc(value: u8) { ::write(REGISTER_ADDRESS, LSERDYC_BIT_OFFSET, LSERDYC_BIT_WIDTH, value as u32); }

	const HSIRDYC_BIT_OFFSET: u8 = 18;
	const HSIRDYC_BIT_WIDTH: u8 = 1;
	/// HSI Ready Interrupt Clear (Width: 1, Offset: 18)
	pub fn set_hsirdyc(value: u8) { ::write(REGISTER_ADDRESS, HSIRDYC_BIT_OFFSET, HSIRDYC_BIT_WIDTH, value as u32); }

	const HSERDYC_BIT_OFFSET: u8 = 19;
	const HSERDYC_BIT_WIDTH: u8 = 1;
	/// HSE Ready Interrupt Clear (Width: 1, Offset: 19)
	pub fn set_hserdyc(value: u8) { ::write(REGISTER_ADDRESS, HSERDYC_BIT_OFFSET, HSERDYC_BIT_WIDTH, value as u32); }

	const PLLRDYC_BIT_OFFSET: u8 = 20;
	const PLLRDYC_BIT_WIDTH: u8 = 1;
	/// PLL Ready Interrupt Clear (Width: 1, Offset: 20)
	pub fn set_pllrdyc(value: u8) { ::write(REGISTER_ADDRESS, PLLRDYC_BIT_OFFSET, PLLRDYC_BIT_WIDTH, value as u32); }

	const CSSC_BIT_OFFSET: u8 = 23;
	const CSSC_BIT_WIDTH: u8 = 1;
	/// Clock security system interrupt clear (Width: 1, Offset: 23)
	pub fn set_cssc(value: u8) { ::write(REGISTER_ADDRESS, CSSC_BIT_OFFSET, CSSC_BIT_WIDTH, value as u32); }
}
/// APB2 peripheral reset register (RCC_APB2RSTR)
/// Size: 0x20 bits
pub mod apb2rstr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0xC;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SYSCFGRST_BIT_OFFSET: u8 = 0;
	const SYSCFGRST_BIT_WIDTH: u8 = 1;
	/// SYSCFG and COMP reset (Width: 1, Offset: 0)
	pub fn get_syscfgrst() -> u8 { ::read(REGISTER_ADDRESS, SYSCFGRST_BIT_OFFSET, SYSCFGRST_BIT_WIDTH) as u8 }
	/// SYSCFG and COMP reset (Width: 1, Offset: 0)
	pub fn set_syscfgrst(value: u8) { ::write(REGISTER_ADDRESS, SYSCFGRST_BIT_OFFSET, SYSCFGRST_BIT_WIDTH, value as u32); }

	const TIM1RST_BIT_OFFSET: u8 = 11;
	const TIM1RST_BIT_WIDTH: u8 = 1;
	/// TIM1 timer reset (Width: 1, Offset: 11)
	pub fn get_tim1rst() -> u8 { ::read(REGISTER_ADDRESS, TIM1RST_BIT_OFFSET, TIM1RST_BIT_WIDTH) as u8 }
	/// TIM1 timer reset (Width: 1, Offset: 11)
	pub fn set_tim1rst(value: u8) { ::write(REGISTER_ADDRESS, TIM1RST_BIT_OFFSET, TIM1RST_BIT_WIDTH, value as u32); }

	const SPI1RST_BIT_OFFSET: u8 = 12;
	const SPI1RST_BIT_WIDTH: u8 = 1;
	/// SPI 1 reset (Width: 1, Offset: 12)
	pub fn get_spi1rst() -> u8 { ::read(REGISTER_ADDRESS, SPI1RST_BIT_OFFSET, SPI1RST_BIT_WIDTH) as u8 }
	/// SPI 1 reset (Width: 1, Offset: 12)
	pub fn set_spi1rst(value: u8) { ::write(REGISTER_ADDRESS, SPI1RST_BIT_OFFSET, SPI1RST_BIT_WIDTH, value as u32); }

	const TIM8RST_BIT_OFFSET: u8 = 13;
	const TIM8RST_BIT_WIDTH: u8 = 1;
	/// TIM8 timer reset (Width: 1, Offset: 13)
	pub fn get_tim8rst() -> u8 { ::read(REGISTER_ADDRESS, TIM8RST_BIT_OFFSET, TIM8RST_BIT_WIDTH) as u8 }
	/// TIM8 timer reset (Width: 1, Offset: 13)
	pub fn set_tim8rst(value: u8) { ::write(REGISTER_ADDRESS, TIM8RST_BIT_OFFSET, TIM8RST_BIT_WIDTH, value as u32); }

	const USART1RST_BIT_OFFSET: u8 = 14;
	const USART1RST_BIT_WIDTH: u8 = 1;
	/// USART1 reset (Width: 1, Offset: 14)
	pub fn get_usart1rst() -> u8 { ::read(REGISTER_ADDRESS, USART1RST_BIT_OFFSET, USART1RST_BIT_WIDTH) as u8 }
	/// USART1 reset (Width: 1, Offset: 14)
	pub fn set_usart1rst(value: u8) { ::write(REGISTER_ADDRESS, USART1RST_BIT_OFFSET, USART1RST_BIT_WIDTH, value as u32); }

	const TIM15RST_BIT_OFFSET: u8 = 16;
	const TIM15RST_BIT_WIDTH: u8 = 1;
	/// TIM15 timer reset (Width: 1, Offset: 16)
	pub fn get_tim15rst() -> u8 { ::read(REGISTER_ADDRESS, TIM15RST_BIT_OFFSET, TIM15RST_BIT_WIDTH) as u8 }
	/// TIM15 timer reset (Width: 1, Offset: 16)
	pub fn set_tim15rst(value: u8) { ::write(REGISTER_ADDRESS, TIM15RST_BIT_OFFSET, TIM15RST_BIT_WIDTH, value as u32); }

	const TIM16RST_BIT_OFFSET: u8 = 17;
	const TIM16RST_BIT_WIDTH: u8 = 1;
	/// TIM16 timer reset (Width: 1, Offset: 17)
	pub fn get_tim16rst() -> u8 { ::read(REGISTER_ADDRESS, TIM16RST_BIT_OFFSET, TIM16RST_BIT_WIDTH) as u8 }
	/// TIM16 timer reset (Width: 1, Offset: 17)
	pub fn set_tim16rst(value: u8) { ::write(REGISTER_ADDRESS, TIM16RST_BIT_OFFSET, TIM16RST_BIT_WIDTH, value as u32); }

	const TIM17RST_BIT_OFFSET: u8 = 18;
	const TIM17RST_BIT_WIDTH: u8 = 1;
	/// TIM17 timer reset (Width: 1, Offset: 18)
	pub fn get_tim17rst() -> u8 { ::read(REGISTER_ADDRESS, TIM17RST_BIT_OFFSET, TIM17RST_BIT_WIDTH) as u8 }
	/// TIM17 timer reset (Width: 1, Offset: 18)
	pub fn set_tim17rst(value: u8) { ::write(REGISTER_ADDRESS, TIM17RST_BIT_OFFSET, TIM17RST_BIT_WIDTH, value as u32); }
}
/// APB1 peripheral reset register (RCC_APB1RSTR)
/// Size: 0x20 bits
pub mod apb1rstr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x10;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TIM2RST_BIT_OFFSET: u8 = 0;
	const TIM2RST_BIT_WIDTH: u8 = 1;
	/// Timer 2 reset (Width: 1, Offset: 0)
	pub fn get_tim2rst() -> u8 { ::read(REGISTER_ADDRESS, TIM2RST_BIT_OFFSET, TIM2RST_BIT_WIDTH) as u8 }
	/// Timer 2 reset (Width: 1, Offset: 0)
	pub fn set_tim2rst(value: u8) { ::write(REGISTER_ADDRESS, TIM2RST_BIT_OFFSET, TIM2RST_BIT_WIDTH, value as u32); }

	const TIM3RST_BIT_OFFSET: u8 = 1;
	const TIM3RST_BIT_WIDTH: u8 = 1;
	/// Timer 3 reset (Width: 1, Offset: 1)
	pub fn get_tim3rst() -> u8 { ::read(REGISTER_ADDRESS, TIM3RST_BIT_OFFSET, TIM3RST_BIT_WIDTH) as u8 }
	/// Timer 3 reset (Width: 1, Offset: 1)
	pub fn set_tim3rst(value: u8) { ::write(REGISTER_ADDRESS, TIM3RST_BIT_OFFSET, TIM3RST_BIT_WIDTH, value as u32); }

	const TIM4RST_BIT_OFFSET: u8 = 2;
	const TIM4RST_BIT_WIDTH: u8 = 1;
	/// Timer 14 reset (Width: 1, Offset: 2)
	pub fn get_tim4rst() -> u8 { ::read(REGISTER_ADDRESS, TIM4RST_BIT_OFFSET, TIM4RST_BIT_WIDTH) as u8 }
	/// Timer 14 reset (Width: 1, Offset: 2)
	pub fn set_tim4rst(value: u8) { ::write(REGISTER_ADDRESS, TIM4RST_BIT_OFFSET, TIM4RST_BIT_WIDTH, value as u32); }

	const TIM6RST_BIT_OFFSET: u8 = 4;
	const TIM6RST_BIT_WIDTH: u8 = 1;
	/// Timer 6 reset (Width: 1, Offset: 4)
	pub fn get_tim6rst() -> u8 { ::read(REGISTER_ADDRESS, TIM6RST_BIT_OFFSET, TIM6RST_BIT_WIDTH) as u8 }
	/// Timer 6 reset (Width: 1, Offset: 4)
	pub fn set_tim6rst(value: u8) { ::write(REGISTER_ADDRESS, TIM6RST_BIT_OFFSET, TIM6RST_BIT_WIDTH, value as u32); }

	const TIM7RST_BIT_OFFSET: u8 = 5;
	const TIM7RST_BIT_WIDTH: u8 = 1;
	/// Timer 7 reset (Width: 1, Offset: 5)
	pub fn get_tim7rst() -> u8 { ::read(REGISTER_ADDRESS, TIM7RST_BIT_OFFSET, TIM7RST_BIT_WIDTH) as u8 }
	/// Timer 7 reset (Width: 1, Offset: 5)
	pub fn set_tim7rst(value: u8) { ::write(REGISTER_ADDRESS, TIM7RST_BIT_OFFSET, TIM7RST_BIT_WIDTH, value as u32); }

	const WWDGRST_BIT_OFFSET: u8 = 11;
	const WWDGRST_BIT_WIDTH: u8 = 1;
	/// Window watchdog reset (Width: 1, Offset: 11)
	pub fn get_wwdgrst() -> u8 { ::read(REGISTER_ADDRESS, WWDGRST_BIT_OFFSET, WWDGRST_BIT_WIDTH) as u8 }
	/// Window watchdog reset (Width: 1, Offset: 11)
	pub fn set_wwdgrst(value: u8) { ::write(REGISTER_ADDRESS, WWDGRST_BIT_OFFSET, WWDGRST_BIT_WIDTH, value as u32); }

	const SPI2RST_BIT_OFFSET: u8 = 14;
	const SPI2RST_BIT_WIDTH: u8 = 1;
	/// SPI2 reset (Width: 1, Offset: 14)
	pub fn get_spi2rst() -> u8 { ::read(REGISTER_ADDRESS, SPI2RST_BIT_OFFSET, SPI2RST_BIT_WIDTH) as u8 }
	/// SPI2 reset (Width: 1, Offset: 14)
	pub fn set_spi2rst(value: u8) { ::write(REGISTER_ADDRESS, SPI2RST_BIT_OFFSET, SPI2RST_BIT_WIDTH, value as u32); }

	const SPI3RST_BIT_OFFSET: u8 = 15;
	const SPI3RST_BIT_WIDTH: u8 = 1;
	/// SPI3 reset (Width: 1, Offset: 15)
	pub fn get_spi3rst() -> u8 { ::read(REGISTER_ADDRESS, SPI3RST_BIT_OFFSET, SPI3RST_BIT_WIDTH) as u8 }
	/// SPI3 reset (Width: 1, Offset: 15)
	pub fn set_spi3rst(value: u8) { ::write(REGISTER_ADDRESS, SPI3RST_BIT_OFFSET, SPI3RST_BIT_WIDTH, value as u32); }

	const USART2RST_BIT_OFFSET: u8 = 17;
	const USART2RST_BIT_WIDTH: u8 = 1;
	/// USART 2 reset (Width: 1, Offset: 17)
	pub fn get_usart2rst() -> u8 { ::read(REGISTER_ADDRESS, USART2RST_BIT_OFFSET, USART2RST_BIT_WIDTH) as u8 }
	/// USART 2 reset (Width: 1, Offset: 17)
	pub fn set_usart2rst(value: u8) { ::write(REGISTER_ADDRESS, USART2RST_BIT_OFFSET, USART2RST_BIT_WIDTH, value as u32); }

	const USART3RST_BIT_OFFSET: u8 = 18;
	const USART3RST_BIT_WIDTH: u8 = 1;
	/// USART3 reset (Width: 1, Offset: 18)
	pub fn get_usart3rst() -> u8 { ::read(REGISTER_ADDRESS, USART3RST_BIT_OFFSET, USART3RST_BIT_WIDTH) as u8 }
	/// USART3 reset (Width: 1, Offset: 18)
	pub fn set_usart3rst(value: u8) { ::write(REGISTER_ADDRESS, USART3RST_BIT_OFFSET, USART3RST_BIT_WIDTH, value as u32); }

	const UART4RST_BIT_OFFSET: u8 = 19;
	const UART4RST_BIT_WIDTH: u8 = 1;
	/// UART 4 reset (Width: 1, Offset: 19)
	pub fn get_uart4rst() -> u8 { ::read(REGISTER_ADDRESS, UART4RST_BIT_OFFSET, UART4RST_BIT_WIDTH) as u8 }
	/// UART 4 reset (Width: 1, Offset: 19)
	pub fn set_uart4rst(value: u8) { ::write(REGISTER_ADDRESS, UART4RST_BIT_OFFSET, UART4RST_BIT_WIDTH, value as u32); }

	const UART5RST_BIT_OFFSET: u8 = 20;
	const UART5RST_BIT_WIDTH: u8 = 1;
	/// UART 5 reset (Width: 1, Offset: 20)
	pub fn get_uart5rst() -> u8 { ::read(REGISTER_ADDRESS, UART5RST_BIT_OFFSET, UART5RST_BIT_WIDTH) as u8 }
	/// UART 5 reset (Width: 1, Offset: 20)
	pub fn set_uart5rst(value: u8) { ::write(REGISTER_ADDRESS, UART5RST_BIT_OFFSET, UART5RST_BIT_WIDTH, value as u32); }

	const I2C1RST_BIT_OFFSET: u8 = 21;
	const I2C1RST_BIT_WIDTH: u8 = 1;
	/// I2C1 reset (Width: 1, Offset: 21)
	pub fn get_i2c1rst() -> u8 { ::read(REGISTER_ADDRESS, I2C1RST_BIT_OFFSET, I2C1RST_BIT_WIDTH) as u8 }
	/// I2C1 reset (Width: 1, Offset: 21)
	pub fn set_i2c1rst(value: u8) { ::write(REGISTER_ADDRESS, I2C1RST_BIT_OFFSET, I2C1RST_BIT_WIDTH, value as u32); }

	const I2C2RST_BIT_OFFSET: u8 = 22;
	const I2C2RST_BIT_WIDTH: u8 = 1;
	/// I2C2 reset (Width: 1, Offset: 22)
	pub fn get_i2c2rst() -> u8 { ::read(REGISTER_ADDRESS, I2C2RST_BIT_OFFSET, I2C2RST_BIT_WIDTH) as u8 }
	/// I2C2 reset (Width: 1, Offset: 22)
	pub fn set_i2c2rst(value: u8) { ::write(REGISTER_ADDRESS, I2C2RST_BIT_OFFSET, I2C2RST_BIT_WIDTH, value as u32); }

	const USBRST_BIT_OFFSET: u8 = 23;
	const USBRST_BIT_WIDTH: u8 = 1;
	/// USB reset (Width: 1, Offset: 23)
	pub fn get_usbrst() -> u8 { ::read(REGISTER_ADDRESS, USBRST_BIT_OFFSET, USBRST_BIT_WIDTH) as u8 }
	/// USB reset (Width: 1, Offset: 23)
	pub fn set_usbrst(value: u8) { ::write(REGISTER_ADDRESS, USBRST_BIT_OFFSET, USBRST_BIT_WIDTH, value as u32); }

	const CANRST_BIT_OFFSET: u8 = 25;
	const CANRST_BIT_WIDTH: u8 = 1;
	/// CAN reset (Width: 1, Offset: 25)
	pub fn get_canrst() -> u8 { ::read(REGISTER_ADDRESS, CANRST_BIT_OFFSET, CANRST_BIT_WIDTH) as u8 }
	/// CAN reset (Width: 1, Offset: 25)
	pub fn set_canrst(value: u8) { ::write(REGISTER_ADDRESS, CANRST_BIT_OFFSET, CANRST_BIT_WIDTH, value as u32); }

	const PWRRST_BIT_OFFSET: u8 = 28;
	const PWRRST_BIT_WIDTH: u8 = 1;
	/// Power interface reset (Width: 1, Offset: 28)
	pub fn get_pwrrst() -> u8 { ::read(REGISTER_ADDRESS, PWRRST_BIT_OFFSET, PWRRST_BIT_WIDTH) as u8 }
	/// Power interface reset (Width: 1, Offset: 28)
	pub fn set_pwrrst(value: u8) { ::write(REGISTER_ADDRESS, PWRRST_BIT_OFFSET, PWRRST_BIT_WIDTH, value as u32); }

	const DACRST_BIT_OFFSET: u8 = 29;
	const DACRST_BIT_WIDTH: u8 = 1;
	/// DAC interface reset (Width: 1, Offset: 29)
	pub fn get_dacrst() -> u8 { ::read(REGISTER_ADDRESS, DACRST_BIT_OFFSET, DACRST_BIT_WIDTH) as u8 }
	/// DAC interface reset (Width: 1, Offset: 29)
	pub fn set_dacrst(value: u8) { ::write(REGISTER_ADDRESS, DACRST_BIT_OFFSET, DACRST_BIT_WIDTH, value as u32); }
}
/// AHB Peripheral Clock enable register (RCC_AHBENR)
/// Size: 0x20 bits
pub mod ahbenr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x14;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const DMAEN_BIT_OFFSET: u8 = 0;
	const DMAEN_BIT_WIDTH: u8 = 1;
	/// DMA1 clock enable (Width: 1, Offset: 0)
	pub fn get_dmaen() -> u8 { ::read(REGISTER_ADDRESS, DMAEN_BIT_OFFSET, DMAEN_BIT_WIDTH) as u8 }
	/// DMA1 clock enable (Width: 1, Offset: 0)
	pub fn set_dmaen(value: u8) { ::write(REGISTER_ADDRESS, DMAEN_BIT_OFFSET, DMAEN_BIT_WIDTH, value as u32); }

	const DMA2EN_BIT_OFFSET: u8 = 1;
	const DMA2EN_BIT_WIDTH: u8 = 1;
	/// DMA2 clock enable (Width: 1, Offset: 1)
	pub fn get_dma2en() -> u8 { ::read(REGISTER_ADDRESS, DMA2EN_BIT_OFFSET, DMA2EN_BIT_WIDTH) as u8 }
	/// DMA2 clock enable (Width: 1, Offset: 1)
	pub fn set_dma2en(value: u8) { ::write(REGISTER_ADDRESS, DMA2EN_BIT_OFFSET, DMA2EN_BIT_WIDTH, value as u32); }

	const SRAMEN_BIT_OFFSET: u8 = 2;
	const SRAMEN_BIT_WIDTH: u8 = 1;
	/// SRAM interface clock enable (Width: 1, Offset: 2)
	pub fn get_sramen() -> u8 { ::read(REGISTER_ADDRESS, SRAMEN_BIT_OFFSET, SRAMEN_BIT_WIDTH) as u8 }
	/// SRAM interface clock enable (Width: 1, Offset: 2)
	pub fn set_sramen(value: u8) { ::write(REGISTER_ADDRESS, SRAMEN_BIT_OFFSET, SRAMEN_BIT_WIDTH, value as u32); }

	const FLITFEN_BIT_OFFSET: u8 = 4;
	const FLITFEN_BIT_WIDTH: u8 = 1;
	/// FLITF clock enable (Width: 1, Offset: 4)
	pub fn get_flitfen() -> u8 { ::read(REGISTER_ADDRESS, FLITFEN_BIT_OFFSET, FLITFEN_BIT_WIDTH) as u8 }
	/// FLITF clock enable (Width: 1, Offset: 4)
	pub fn set_flitfen(value: u8) { ::write(REGISTER_ADDRESS, FLITFEN_BIT_OFFSET, FLITFEN_BIT_WIDTH, value as u32); }

	const CRCEN_BIT_OFFSET: u8 = 6;
	const CRCEN_BIT_WIDTH: u8 = 1;
	/// CRC clock enable (Width: 1, Offset: 6)
	pub fn get_crcen() -> u8 { ::read(REGISTER_ADDRESS, CRCEN_BIT_OFFSET, CRCEN_BIT_WIDTH) as u8 }
	/// CRC clock enable (Width: 1, Offset: 6)
	pub fn set_crcen(value: u8) { ::write(REGISTER_ADDRESS, CRCEN_BIT_OFFSET, CRCEN_BIT_WIDTH, value as u32); }

	const IOPAEN_BIT_OFFSET: u8 = 17;
	const IOPAEN_BIT_WIDTH: u8 = 1;
	/// I/O port A clock enable (Width: 1, Offset: 17)
	pub fn get_iopaen() -> u8 { ::read(REGISTER_ADDRESS, IOPAEN_BIT_OFFSET, IOPAEN_BIT_WIDTH) as u8 }
	/// I/O port A clock enable (Width: 1, Offset: 17)
	pub fn set_iopaen(value: u8) { ::write(REGISTER_ADDRESS, IOPAEN_BIT_OFFSET, IOPAEN_BIT_WIDTH, value as u32); }

	const IOPBEN_BIT_OFFSET: u8 = 18;
	const IOPBEN_BIT_WIDTH: u8 = 1;
	/// I/O port B clock enable (Width: 1, Offset: 18)
	pub fn get_iopben() -> u8 { ::read(REGISTER_ADDRESS, IOPBEN_BIT_OFFSET, IOPBEN_BIT_WIDTH) as u8 }
	/// I/O port B clock enable (Width: 1, Offset: 18)
	pub fn set_iopben(value: u8) { ::write(REGISTER_ADDRESS, IOPBEN_BIT_OFFSET, IOPBEN_BIT_WIDTH, value as u32); }

	const IOPCEN_BIT_OFFSET: u8 = 19;
	const IOPCEN_BIT_WIDTH: u8 = 1;
	/// I/O port C clock enable (Width: 1, Offset: 19)
	pub fn get_iopcen() -> u8 { ::read(REGISTER_ADDRESS, IOPCEN_BIT_OFFSET, IOPCEN_BIT_WIDTH) as u8 }
	/// I/O port C clock enable (Width: 1, Offset: 19)
	pub fn set_iopcen(value: u8) { ::write(REGISTER_ADDRESS, IOPCEN_BIT_OFFSET, IOPCEN_BIT_WIDTH, value as u32); }

	const IOPDEN_BIT_OFFSET: u8 = 20;
	const IOPDEN_BIT_WIDTH: u8 = 1;
	/// I/O port D clock enable (Width: 1, Offset: 20)
	pub fn get_iopden() -> u8 { ::read(REGISTER_ADDRESS, IOPDEN_BIT_OFFSET, IOPDEN_BIT_WIDTH) as u8 }
	/// I/O port D clock enable (Width: 1, Offset: 20)
	pub fn set_iopden(value: u8) { ::write(REGISTER_ADDRESS, IOPDEN_BIT_OFFSET, IOPDEN_BIT_WIDTH, value as u32); }

	const IOPEEN_BIT_OFFSET: u8 = 21;
	const IOPEEN_BIT_WIDTH: u8 = 1;
	/// I/O port E clock enable (Width: 1, Offset: 21)
	pub fn get_iopeen() -> u8 { ::read(REGISTER_ADDRESS, IOPEEN_BIT_OFFSET, IOPEEN_BIT_WIDTH) as u8 }
	/// I/O port E clock enable (Width: 1, Offset: 21)
	pub fn set_iopeen(value: u8) { ::write(REGISTER_ADDRESS, IOPEEN_BIT_OFFSET, IOPEEN_BIT_WIDTH, value as u32); }

	const IOPFEN_BIT_OFFSET: u8 = 22;
	const IOPFEN_BIT_WIDTH: u8 = 1;
	/// I/O port F clock enable (Width: 1, Offset: 22)
	pub fn get_iopfen() -> u8 { ::read(REGISTER_ADDRESS, IOPFEN_BIT_OFFSET, IOPFEN_BIT_WIDTH) as u8 }
	/// I/O port F clock enable (Width: 1, Offset: 22)
	pub fn set_iopfen(value: u8) { ::write(REGISTER_ADDRESS, IOPFEN_BIT_OFFSET, IOPFEN_BIT_WIDTH, value as u32); }

	const TSCEN_BIT_OFFSET: u8 = 24;
	const TSCEN_BIT_WIDTH: u8 = 1;
	/// Touch sensing controller clock enable (Width: 1, Offset: 24)
	pub fn get_tscen() -> u8 { ::read(REGISTER_ADDRESS, TSCEN_BIT_OFFSET, TSCEN_BIT_WIDTH) as u8 }
	/// Touch sensing controller clock enable (Width: 1, Offset: 24)
	pub fn set_tscen(value: u8) { ::write(REGISTER_ADDRESS, TSCEN_BIT_OFFSET, TSCEN_BIT_WIDTH, value as u32); }

	const ADC12EN_BIT_OFFSET: u8 = 28;
	const ADC12EN_BIT_WIDTH: u8 = 1;
	/// ADC1 and ADC2 clock enable (Width: 1, Offset: 28)
	pub fn get_adc12en() -> u8 { ::read(REGISTER_ADDRESS, ADC12EN_BIT_OFFSET, ADC12EN_BIT_WIDTH) as u8 }
	/// ADC1 and ADC2 clock enable (Width: 1, Offset: 28)
	pub fn set_adc12en(value: u8) { ::write(REGISTER_ADDRESS, ADC12EN_BIT_OFFSET, ADC12EN_BIT_WIDTH, value as u32); }

	const ADC34EN_BIT_OFFSET: u8 = 29;
	const ADC34EN_BIT_WIDTH: u8 = 1;
	/// ADC3 and ADC4 clock enable (Width: 1, Offset: 29)
	pub fn get_adc34en() -> u8 { ::read(REGISTER_ADDRESS, ADC34EN_BIT_OFFSET, ADC34EN_BIT_WIDTH) as u8 }
	/// ADC3 and ADC4 clock enable (Width: 1, Offset: 29)
	pub fn set_adc34en(value: u8) { ::write(REGISTER_ADDRESS, ADC34EN_BIT_OFFSET, ADC34EN_BIT_WIDTH, value as u32); }
}
/// APB2 peripheral clock enable register (RCC_APB2ENR)
/// Size: 0x20 bits
pub mod apb2enr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x18;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const SYSCFGEN_BIT_OFFSET: u8 = 0;
	const SYSCFGEN_BIT_WIDTH: u8 = 1;
	/// SYSCFG clock enable (Width: 1, Offset: 0)
	pub fn get_syscfgen() -> u8 { ::read(REGISTER_ADDRESS, SYSCFGEN_BIT_OFFSET, SYSCFGEN_BIT_WIDTH) as u8 }
	/// SYSCFG clock enable (Width: 1, Offset: 0)
	pub fn set_syscfgen(value: u8) { ::write(REGISTER_ADDRESS, SYSCFGEN_BIT_OFFSET, SYSCFGEN_BIT_WIDTH, value as u32); }

	const TIM1EN_BIT_OFFSET: u8 = 11;
	const TIM1EN_BIT_WIDTH: u8 = 1;
	/// TIM1 Timer clock enable (Width: 1, Offset: 11)
	pub fn get_tim1en() -> u8 { ::read(REGISTER_ADDRESS, TIM1EN_BIT_OFFSET, TIM1EN_BIT_WIDTH) as u8 }
	/// TIM1 Timer clock enable (Width: 1, Offset: 11)
	pub fn set_tim1en(value: u8) { ::write(REGISTER_ADDRESS, TIM1EN_BIT_OFFSET, TIM1EN_BIT_WIDTH, value as u32); }

	const SPI1EN_BIT_OFFSET: u8 = 12;
	const SPI1EN_BIT_WIDTH: u8 = 1;
	/// SPI 1 clock enable (Width: 1, Offset: 12)
	pub fn get_spi1en() -> u8 { ::read(REGISTER_ADDRESS, SPI1EN_BIT_OFFSET, SPI1EN_BIT_WIDTH) as u8 }
	/// SPI 1 clock enable (Width: 1, Offset: 12)
	pub fn set_spi1en(value: u8) { ::write(REGISTER_ADDRESS, SPI1EN_BIT_OFFSET, SPI1EN_BIT_WIDTH, value as u32); }

	const TIM8EN_BIT_OFFSET: u8 = 13;
	const TIM8EN_BIT_WIDTH: u8 = 1;
	/// TIM8 Timer clock enable (Width: 1, Offset: 13)
	pub fn get_tim8en() -> u8 { ::read(REGISTER_ADDRESS, TIM8EN_BIT_OFFSET, TIM8EN_BIT_WIDTH) as u8 }
	/// TIM8 Timer clock enable (Width: 1, Offset: 13)
	pub fn set_tim8en(value: u8) { ::write(REGISTER_ADDRESS, TIM8EN_BIT_OFFSET, TIM8EN_BIT_WIDTH, value as u32); }

	const USART1EN_BIT_OFFSET: u8 = 14;
	const USART1EN_BIT_WIDTH: u8 = 1;
	/// USART1 clock enable (Width: 1, Offset: 14)
	pub fn get_usart1en() -> u8 { ::read(REGISTER_ADDRESS, USART1EN_BIT_OFFSET, USART1EN_BIT_WIDTH) as u8 }
	/// USART1 clock enable (Width: 1, Offset: 14)
	pub fn set_usart1en(value: u8) { ::write(REGISTER_ADDRESS, USART1EN_BIT_OFFSET, USART1EN_BIT_WIDTH, value as u32); }

	const TIM15EN_BIT_OFFSET: u8 = 16;
	const TIM15EN_BIT_WIDTH: u8 = 1;
	/// TIM15 timer clock enable (Width: 1, Offset: 16)
	pub fn get_tim15en() -> u8 { ::read(REGISTER_ADDRESS, TIM15EN_BIT_OFFSET, TIM15EN_BIT_WIDTH) as u8 }
	/// TIM15 timer clock enable (Width: 1, Offset: 16)
	pub fn set_tim15en(value: u8) { ::write(REGISTER_ADDRESS, TIM15EN_BIT_OFFSET, TIM15EN_BIT_WIDTH, value as u32); }

	const TIM16EN_BIT_OFFSET: u8 = 17;
	const TIM16EN_BIT_WIDTH: u8 = 1;
	/// TIM16 timer clock enable (Width: 1, Offset: 17)
	pub fn get_tim16en() -> u8 { ::read(REGISTER_ADDRESS, TIM16EN_BIT_OFFSET, TIM16EN_BIT_WIDTH) as u8 }
	/// TIM16 timer clock enable (Width: 1, Offset: 17)
	pub fn set_tim16en(value: u8) { ::write(REGISTER_ADDRESS, TIM16EN_BIT_OFFSET, TIM16EN_BIT_WIDTH, value as u32); }

	const TIM17EN_BIT_OFFSET: u8 = 18;
	const TIM17EN_BIT_WIDTH: u8 = 1;
	/// TIM17 timer clock enable (Width: 1, Offset: 18)
	pub fn get_tim17en() -> u8 { ::read(REGISTER_ADDRESS, TIM17EN_BIT_OFFSET, TIM17EN_BIT_WIDTH) as u8 }
	/// TIM17 timer clock enable (Width: 1, Offset: 18)
	pub fn set_tim17en(value: u8) { ::write(REGISTER_ADDRESS, TIM17EN_BIT_OFFSET, TIM17EN_BIT_WIDTH, value as u32); }
}
/// APB1 peripheral clock enable register (RCC_APB1ENR)
/// Size: 0x20 bits
pub mod apb1enr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x1C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const TIM2EN_BIT_OFFSET: u8 = 0;
	const TIM2EN_BIT_WIDTH: u8 = 1;
	/// Timer 2 clock enable (Width: 1, Offset: 0)
	pub fn get_tim2en() -> u8 { ::read(REGISTER_ADDRESS, TIM2EN_BIT_OFFSET, TIM2EN_BIT_WIDTH) as u8 }
	/// Timer 2 clock enable (Width: 1, Offset: 0)
	pub fn set_tim2en(value: u8) { ::write(REGISTER_ADDRESS, TIM2EN_BIT_OFFSET, TIM2EN_BIT_WIDTH, value as u32); }

	const TIM3EN_BIT_OFFSET: u8 = 1;
	const TIM3EN_BIT_WIDTH: u8 = 1;
	/// Timer 3 clock enable (Width: 1, Offset: 1)
	pub fn get_tim3en() -> u8 { ::read(REGISTER_ADDRESS, TIM3EN_BIT_OFFSET, TIM3EN_BIT_WIDTH) as u8 }
	/// Timer 3 clock enable (Width: 1, Offset: 1)
	pub fn set_tim3en(value: u8) { ::write(REGISTER_ADDRESS, TIM3EN_BIT_OFFSET, TIM3EN_BIT_WIDTH, value as u32); }

	const TIM4EN_BIT_OFFSET: u8 = 2;
	const TIM4EN_BIT_WIDTH: u8 = 1;
	/// Timer 4 clock enable (Width: 1, Offset: 2)
	pub fn get_tim4en() -> u8 { ::read(REGISTER_ADDRESS, TIM4EN_BIT_OFFSET, TIM4EN_BIT_WIDTH) as u8 }
	/// Timer 4 clock enable (Width: 1, Offset: 2)
	pub fn set_tim4en(value: u8) { ::write(REGISTER_ADDRESS, TIM4EN_BIT_OFFSET, TIM4EN_BIT_WIDTH, value as u32); }

	const TIM6EN_BIT_OFFSET: u8 = 4;
	const TIM6EN_BIT_WIDTH: u8 = 1;
	/// Timer 6 clock enable (Width: 1, Offset: 4)
	pub fn get_tim6en() -> u8 { ::read(REGISTER_ADDRESS, TIM6EN_BIT_OFFSET, TIM6EN_BIT_WIDTH) as u8 }
	/// Timer 6 clock enable (Width: 1, Offset: 4)
	pub fn set_tim6en(value: u8) { ::write(REGISTER_ADDRESS, TIM6EN_BIT_OFFSET, TIM6EN_BIT_WIDTH, value as u32); }

	const TIM7EN_BIT_OFFSET: u8 = 5;
	const TIM7EN_BIT_WIDTH: u8 = 1;
	/// Timer 7 clock enable (Width: 1, Offset: 5)
	pub fn get_tim7en() -> u8 { ::read(REGISTER_ADDRESS, TIM7EN_BIT_OFFSET, TIM7EN_BIT_WIDTH) as u8 }
	/// Timer 7 clock enable (Width: 1, Offset: 5)
	pub fn set_tim7en(value: u8) { ::write(REGISTER_ADDRESS, TIM7EN_BIT_OFFSET, TIM7EN_BIT_WIDTH, value as u32); }

	const WWDGEN_BIT_OFFSET: u8 = 11;
	const WWDGEN_BIT_WIDTH: u8 = 1;
	/// Window watchdog clock enable (Width: 1, Offset: 11)
	pub fn get_wwdgen() -> u8 { ::read(REGISTER_ADDRESS, WWDGEN_BIT_OFFSET, WWDGEN_BIT_WIDTH) as u8 }
	/// Window watchdog clock enable (Width: 1, Offset: 11)
	pub fn set_wwdgen(value: u8) { ::write(REGISTER_ADDRESS, WWDGEN_BIT_OFFSET, WWDGEN_BIT_WIDTH, value as u32); }

	const SPI2EN_BIT_OFFSET: u8 = 14;
	const SPI2EN_BIT_WIDTH: u8 = 1;
	/// SPI 2 clock enable (Width: 1, Offset: 14)
	pub fn get_spi2en() -> u8 { ::read(REGISTER_ADDRESS, SPI2EN_BIT_OFFSET, SPI2EN_BIT_WIDTH) as u8 }
	/// SPI 2 clock enable (Width: 1, Offset: 14)
	pub fn set_spi2en(value: u8) { ::write(REGISTER_ADDRESS, SPI2EN_BIT_OFFSET, SPI2EN_BIT_WIDTH, value as u32); }

	const SPI3EN_BIT_OFFSET: u8 = 15;
	const SPI3EN_BIT_WIDTH: u8 = 1;
	/// SPI 3 clock enable (Width: 1, Offset: 15)
	pub fn get_spi3en() -> u8 { ::read(REGISTER_ADDRESS, SPI3EN_BIT_OFFSET, SPI3EN_BIT_WIDTH) as u8 }
	/// SPI 3 clock enable (Width: 1, Offset: 15)
	pub fn set_spi3en(value: u8) { ::write(REGISTER_ADDRESS, SPI3EN_BIT_OFFSET, SPI3EN_BIT_WIDTH, value as u32); }

	const USART2EN_BIT_OFFSET: u8 = 17;
	const USART2EN_BIT_WIDTH: u8 = 1;
	/// USART 2 clock enable (Width: 1, Offset: 17)
	pub fn get_usart2en() -> u8 { ::read(REGISTER_ADDRESS, USART2EN_BIT_OFFSET, USART2EN_BIT_WIDTH) as u8 }
	/// USART 2 clock enable (Width: 1, Offset: 17)
	pub fn set_usart2en(value: u8) { ::write(REGISTER_ADDRESS, USART2EN_BIT_OFFSET, USART2EN_BIT_WIDTH, value as u32); }

	const I2C1EN_BIT_OFFSET: u8 = 21;
	const I2C1EN_BIT_WIDTH: u8 = 1;
	/// I2C 1 clock enable (Width: 1, Offset: 21)
	pub fn get_i2c1en() -> u8 { ::read(REGISTER_ADDRESS, I2C1EN_BIT_OFFSET, I2C1EN_BIT_WIDTH) as u8 }
	/// I2C 1 clock enable (Width: 1, Offset: 21)
	pub fn set_i2c1en(value: u8) { ::write(REGISTER_ADDRESS, I2C1EN_BIT_OFFSET, I2C1EN_BIT_WIDTH, value as u32); }

	const I2C2EN_BIT_OFFSET: u8 = 22;
	const I2C2EN_BIT_WIDTH: u8 = 1;
	/// I2C 2 clock enable (Width: 1, Offset: 22)
	pub fn get_i2c2en() -> u8 { ::read(REGISTER_ADDRESS, I2C2EN_BIT_OFFSET, I2C2EN_BIT_WIDTH) as u8 }
	/// I2C 2 clock enable (Width: 1, Offset: 22)
	pub fn set_i2c2en(value: u8) { ::write(REGISTER_ADDRESS, I2C2EN_BIT_OFFSET, I2C2EN_BIT_WIDTH, value as u32); }

	const USBEN_BIT_OFFSET: u8 = 23;
	const USBEN_BIT_WIDTH: u8 = 1;
	/// USB clock enable (Width: 1, Offset: 23)
	pub fn get_usben() -> u8 { ::read(REGISTER_ADDRESS, USBEN_BIT_OFFSET, USBEN_BIT_WIDTH) as u8 }
	/// USB clock enable (Width: 1, Offset: 23)
	pub fn set_usben(value: u8) { ::write(REGISTER_ADDRESS, USBEN_BIT_OFFSET, USBEN_BIT_WIDTH, value as u32); }

	const CANEN_BIT_OFFSET: u8 = 25;
	const CANEN_BIT_WIDTH: u8 = 1;
	/// CAN clock enable (Width: 1, Offset: 25)
	pub fn get_canen() -> u8 { ::read(REGISTER_ADDRESS, CANEN_BIT_OFFSET, CANEN_BIT_WIDTH) as u8 }
	/// CAN clock enable (Width: 1, Offset: 25)
	pub fn set_canen(value: u8) { ::write(REGISTER_ADDRESS, CANEN_BIT_OFFSET, CANEN_BIT_WIDTH, value as u32); }

	const PWREN_BIT_OFFSET: u8 = 28;
	const PWREN_BIT_WIDTH: u8 = 1;
	/// Power interface clock enable (Width: 1, Offset: 28)
	pub fn get_pwren() -> u8 { ::read(REGISTER_ADDRESS, PWREN_BIT_OFFSET, PWREN_BIT_WIDTH) as u8 }
	/// Power interface clock enable (Width: 1, Offset: 28)
	pub fn set_pwren(value: u8) { ::write(REGISTER_ADDRESS, PWREN_BIT_OFFSET, PWREN_BIT_WIDTH, value as u32); }

	const DACEN_BIT_OFFSET: u8 = 29;
	const DACEN_BIT_WIDTH: u8 = 1;
	/// DAC interface clock enable (Width: 1, Offset: 29)
	pub fn get_dacen() -> u8 { ::read(REGISTER_ADDRESS, DACEN_BIT_OFFSET, DACEN_BIT_WIDTH) as u8 }
	/// DAC interface clock enable (Width: 1, Offset: 29)
	pub fn set_dacen(value: u8) { ::write(REGISTER_ADDRESS, DACEN_BIT_OFFSET, DACEN_BIT_WIDTH, value as u32); }
}
/// Backup domain control register (RCC_BDCR)
/// Size: 0x20 bits
pub mod bdcr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x20;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const LSEON_BIT_OFFSET: u8 = 0;
	const LSEON_BIT_WIDTH: u8 = 1;
	/// External Low Speed oscillator enable (Width: 1, Offset: 0)
	pub fn get_lseon() -> u8 { ::read(REGISTER_ADDRESS, LSEON_BIT_OFFSET, LSEON_BIT_WIDTH) as u8 }
	/// External Low Speed oscillator enable (Width: 1, Offset: 0)
	pub fn set_lseon(value: u8) { ::write(REGISTER_ADDRESS, LSEON_BIT_OFFSET, LSEON_BIT_WIDTH, value as u32); }

	const LSERDY_BIT_OFFSET: u8 = 1;
	const LSERDY_BIT_WIDTH: u8 = 1;
	/// External Low Speed oscillator ready (Width: 1, Offset: 1)
	pub fn get_lserdy() -> u8 { ::read(REGISTER_ADDRESS, LSERDY_BIT_OFFSET, LSERDY_BIT_WIDTH) as u8 }

	const LSEBYP_BIT_OFFSET: u8 = 2;
	const LSEBYP_BIT_WIDTH: u8 = 1;
	/// External Low Speed oscillator bypass (Width: 1, Offset: 2)
	pub fn get_lsebyp() -> u8 { ::read(REGISTER_ADDRESS, LSEBYP_BIT_OFFSET, LSEBYP_BIT_WIDTH) as u8 }
	/// External Low Speed oscillator bypass (Width: 1, Offset: 2)
	pub fn set_lsebyp(value: u8) { ::write(REGISTER_ADDRESS, LSEBYP_BIT_OFFSET, LSEBYP_BIT_WIDTH, value as u32); }

	const LSEDRV_BIT_OFFSET: u8 = 3;
	const LSEDRV_BIT_WIDTH: u8 = 2;
	/// LSE oscillator drive capability (Width: 2, Offset: 3)
	pub fn get_lsedrv() -> u8 { ::read(REGISTER_ADDRESS, LSEDRV_BIT_OFFSET, LSEDRV_BIT_WIDTH) as u8 }
	/// LSE oscillator drive capability (Width: 2, Offset: 3)
	pub fn set_lsedrv(value: u8) { ::write(REGISTER_ADDRESS, LSEDRV_BIT_OFFSET, LSEDRV_BIT_WIDTH, value as u32); }

	const RTCSEL_BIT_OFFSET: u8 = 8;
	const RTCSEL_BIT_WIDTH: u8 = 2;
	/// RTC clock source selection (Width: 2, Offset: 8)
	pub fn get_rtcsel() -> u8 { ::read(REGISTER_ADDRESS, RTCSEL_BIT_OFFSET, RTCSEL_BIT_WIDTH) as u8 }
	/// RTC clock source selection (Width: 2, Offset: 8)
	pub fn set_rtcsel(value: u8) { ::write(REGISTER_ADDRESS, RTCSEL_BIT_OFFSET, RTCSEL_BIT_WIDTH, value as u32); }

	const RTCEN_BIT_OFFSET: u8 = 15;
	const RTCEN_BIT_WIDTH: u8 = 1;
	/// RTC clock enable (Width: 1, Offset: 15)
	pub fn get_rtcen() -> u8 { ::read(REGISTER_ADDRESS, RTCEN_BIT_OFFSET, RTCEN_BIT_WIDTH) as u8 }
	/// RTC clock enable (Width: 1, Offset: 15)
	pub fn set_rtcen(value: u8) { ::write(REGISTER_ADDRESS, RTCEN_BIT_OFFSET, RTCEN_BIT_WIDTH, value as u32); }

	const BDRST_BIT_OFFSET: u8 = 16;
	const BDRST_BIT_WIDTH: u8 = 1;
	/// Backup domain software reset (Width: 1, Offset: 16)
	pub fn get_bdrst() -> u8 { ::read(REGISTER_ADDRESS, BDRST_BIT_OFFSET, BDRST_BIT_WIDTH) as u8 }
	/// Backup domain software reset (Width: 1, Offset: 16)
	pub fn set_bdrst(value: u8) { ::write(REGISTER_ADDRESS, BDRST_BIT_OFFSET, BDRST_BIT_WIDTH, value as u32); }
}
/// Control/status register (RCC_CSR)
/// Size: 0x20 bits
pub mod csr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x24;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const LSION_BIT_OFFSET: u8 = 0;
	const LSION_BIT_WIDTH: u8 = 1;
	/// Internal low speed oscillator enable (Width: 1, Offset: 0)
	pub fn get_lsion() -> u8 { ::read(REGISTER_ADDRESS, LSION_BIT_OFFSET, LSION_BIT_WIDTH) as u8 }
	/// Internal low speed oscillator enable (Width: 1, Offset: 0)
	pub fn set_lsion(value: u8) { ::write(REGISTER_ADDRESS, LSION_BIT_OFFSET, LSION_BIT_WIDTH, value as u32); }

	const LSIRDY_BIT_OFFSET: u8 = 1;
	const LSIRDY_BIT_WIDTH: u8 = 1;
	/// Internal low speed oscillator ready (Width: 1, Offset: 1)
	pub fn get_lsirdy() -> u8 { ::read(REGISTER_ADDRESS, LSIRDY_BIT_OFFSET, LSIRDY_BIT_WIDTH) as u8 }

	const RMVF_BIT_OFFSET: u8 = 24;
	const RMVF_BIT_WIDTH: u8 = 1;
	/// Remove reset flag (Width: 1, Offset: 24)
	pub fn get_rmvf() -> u8 { ::read(REGISTER_ADDRESS, RMVF_BIT_OFFSET, RMVF_BIT_WIDTH) as u8 }
	/// Remove reset flag (Width: 1, Offset: 24)
	pub fn set_rmvf(value: u8) { ::write(REGISTER_ADDRESS, RMVF_BIT_OFFSET, RMVF_BIT_WIDTH, value as u32); }

	const OBLRSTF_BIT_OFFSET: u8 = 25;
	const OBLRSTF_BIT_WIDTH: u8 = 1;
	/// Option byte loader reset flag (Width: 1, Offset: 25)
	pub fn get_oblrstf() -> u8 { ::read(REGISTER_ADDRESS, OBLRSTF_BIT_OFFSET, OBLRSTF_BIT_WIDTH) as u8 }
	/// Option byte loader reset flag (Width: 1, Offset: 25)
	pub fn set_oblrstf(value: u8) { ::write(REGISTER_ADDRESS, OBLRSTF_BIT_OFFSET, OBLRSTF_BIT_WIDTH, value as u32); }

	const PINRSTF_BIT_OFFSET: u8 = 26;
	const PINRSTF_BIT_WIDTH: u8 = 1;
	/// PIN reset flag (Width: 1, Offset: 26)
	pub fn get_pinrstf() -> u8 { ::read(REGISTER_ADDRESS, PINRSTF_BIT_OFFSET, PINRSTF_BIT_WIDTH) as u8 }
	/// PIN reset flag (Width: 1, Offset: 26)
	pub fn set_pinrstf(value: u8) { ::write(REGISTER_ADDRESS, PINRSTF_BIT_OFFSET, PINRSTF_BIT_WIDTH, value as u32); }

	const PORRSTF_BIT_OFFSET: u8 = 27;
	const PORRSTF_BIT_WIDTH: u8 = 1;
	/// POR/PDR reset flag (Width: 1, Offset: 27)
	pub fn get_porrstf() -> u8 { ::read(REGISTER_ADDRESS, PORRSTF_BIT_OFFSET, PORRSTF_BIT_WIDTH) as u8 }
	/// POR/PDR reset flag (Width: 1, Offset: 27)
	pub fn set_porrstf(value: u8) { ::write(REGISTER_ADDRESS, PORRSTF_BIT_OFFSET, PORRSTF_BIT_WIDTH, value as u32); }

	const SFTRSTF_BIT_OFFSET: u8 = 28;
	const SFTRSTF_BIT_WIDTH: u8 = 1;
	/// Software reset flag (Width: 1, Offset: 28)
	pub fn get_sftrstf() -> u8 { ::read(REGISTER_ADDRESS, SFTRSTF_BIT_OFFSET, SFTRSTF_BIT_WIDTH) as u8 }
	/// Software reset flag (Width: 1, Offset: 28)
	pub fn set_sftrstf(value: u8) { ::write(REGISTER_ADDRESS, SFTRSTF_BIT_OFFSET, SFTRSTF_BIT_WIDTH, value as u32); }

	const IWDGRSTF_BIT_OFFSET: u8 = 29;
	const IWDGRSTF_BIT_WIDTH: u8 = 1;
	/// Independent watchdog reset flag (Width: 1, Offset: 29)
	pub fn get_iwdgrstf() -> u8 { ::read(REGISTER_ADDRESS, IWDGRSTF_BIT_OFFSET, IWDGRSTF_BIT_WIDTH) as u8 }
	/// Independent watchdog reset flag (Width: 1, Offset: 29)
	pub fn set_iwdgrstf(value: u8) { ::write(REGISTER_ADDRESS, IWDGRSTF_BIT_OFFSET, IWDGRSTF_BIT_WIDTH, value as u32); }

	const WWDGRSTF_BIT_OFFSET: u8 = 30;
	const WWDGRSTF_BIT_WIDTH: u8 = 1;
	/// Window watchdog reset flag (Width: 1, Offset: 30)
	pub fn get_wwdgrstf() -> u8 { ::read(REGISTER_ADDRESS, WWDGRSTF_BIT_OFFSET, WWDGRSTF_BIT_WIDTH) as u8 }
	/// Window watchdog reset flag (Width: 1, Offset: 30)
	pub fn set_wwdgrstf(value: u8) { ::write(REGISTER_ADDRESS, WWDGRSTF_BIT_OFFSET, WWDGRSTF_BIT_WIDTH, value as u32); }

	const LPWRRSTF_BIT_OFFSET: u8 = 31;
	const LPWRRSTF_BIT_WIDTH: u8 = 1;
	/// Low-power reset flag (Width: 1, Offset: 31)
	pub fn get_lpwrrstf() -> u8 { ::read(REGISTER_ADDRESS, LPWRRSTF_BIT_OFFSET, LPWRRSTF_BIT_WIDTH) as u8 }
	/// Low-power reset flag (Width: 1, Offset: 31)
	pub fn set_lpwrrstf(value: u8) { ::write(REGISTER_ADDRESS, LPWRRSTF_BIT_OFFSET, LPWRRSTF_BIT_WIDTH, value as u32); }
}
/// AHB peripheral reset register
/// Size: 0x20 bits
pub mod ahbrstr {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x28;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const IOPARST_BIT_OFFSET: u8 = 17;
	const IOPARST_BIT_WIDTH: u8 = 1;
	/// I/O port A reset (Width: 1, Offset: 17)
	pub fn get_ioparst() -> u8 { ::read(REGISTER_ADDRESS, IOPARST_BIT_OFFSET, IOPARST_BIT_WIDTH) as u8 }
	/// I/O port A reset (Width: 1, Offset: 17)
	pub fn set_ioparst(value: u8) { ::write(REGISTER_ADDRESS, IOPARST_BIT_OFFSET, IOPARST_BIT_WIDTH, value as u32); }

	const IOPBRST_BIT_OFFSET: u8 = 18;
	const IOPBRST_BIT_WIDTH: u8 = 1;
	/// I/O port B reset (Width: 1, Offset: 18)
	pub fn get_iopbrst() -> u8 { ::read(REGISTER_ADDRESS, IOPBRST_BIT_OFFSET, IOPBRST_BIT_WIDTH) as u8 }
	/// I/O port B reset (Width: 1, Offset: 18)
	pub fn set_iopbrst(value: u8) { ::write(REGISTER_ADDRESS, IOPBRST_BIT_OFFSET, IOPBRST_BIT_WIDTH, value as u32); }

	const IOPCRST_BIT_OFFSET: u8 = 19;
	const IOPCRST_BIT_WIDTH: u8 = 1;
	/// I/O port C reset (Width: 1, Offset: 19)
	pub fn get_iopcrst() -> u8 { ::read(REGISTER_ADDRESS, IOPCRST_BIT_OFFSET, IOPCRST_BIT_WIDTH) as u8 }
	/// I/O port C reset (Width: 1, Offset: 19)
	pub fn set_iopcrst(value: u8) { ::write(REGISTER_ADDRESS, IOPCRST_BIT_OFFSET, IOPCRST_BIT_WIDTH, value as u32); }

	const IOPDRST_BIT_OFFSET: u8 = 20;
	const IOPDRST_BIT_WIDTH: u8 = 1;
	/// I/O port D reset (Width: 1, Offset: 20)
	pub fn get_iopdrst() -> u8 { ::read(REGISTER_ADDRESS, IOPDRST_BIT_OFFSET, IOPDRST_BIT_WIDTH) as u8 }
	/// I/O port D reset (Width: 1, Offset: 20)
	pub fn set_iopdrst(value: u8) { ::write(REGISTER_ADDRESS, IOPDRST_BIT_OFFSET, IOPDRST_BIT_WIDTH, value as u32); }

	const IOPERST_BIT_OFFSET: u8 = 21;
	const IOPERST_BIT_WIDTH: u8 = 1;
	/// I/O port E reset (Width: 1, Offset: 21)
	pub fn get_ioperst() -> u8 { ::read(REGISTER_ADDRESS, IOPERST_BIT_OFFSET, IOPERST_BIT_WIDTH) as u8 }
	/// I/O port E reset (Width: 1, Offset: 21)
	pub fn set_ioperst(value: u8) { ::write(REGISTER_ADDRESS, IOPERST_BIT_OFFSET, IOPERST_BIT_WIDTH, value as u32); }

	const IOPFRST_BIT_OFFSET: u8 = 22;
	const IOPFRST_BIT_WIDTH: u8 = 1;
	/// I/O port F reset (Width: 1, Offset: 22)
	pub fn get_iopfrst() -> u8 { ::read(REGISTER_ADDRESS, IOPFRST_BIT_OFFSET, IOPFRST_BIT_WIDTH) as u8 }
	/// I/O port F reset (Width: 1, Offset: 22)
	pub fn set_iopfrst(value: u8) { ::write(REGISTER_ADDRESS, IOPFRST_BIT_OFFSET, IOPFRST_BIT_WIDTH, value as u32); }

	const TSCRST_BIT_OFFSET: u8 = 24;
	const TSCRST_BIT_WIDTH: u8 = 1;
	/// Touch sensing controller reset (Width: 1, Offset: 24)
	pub fn get_tscrst() -> u8 { ::read(REGISTER_ADDRESS, TSCRST_BIT_OFFSET, TSCRST_BIT_WIDTH) as u8 }
	/// Touch sensing controller reset (Width: 1, Offset: 24)
	pub fn set_tscrst(value: u8) { ::write(REGISTER_ADDRESS, TSCRST_BIT_OFFSET, TSCRST_BIT_WIDTH, value as u32); }

	const ADC12RST_BIT_OFFSET: u8 = 28;
	const ADC12RST_BIT_WIDTH: u8 = 1;
	/// ADC1 and ADC2 reset (Width: 1, Offset: 28)
	pub fn get_adc12rst() -> u8 { ::read(REGISTER_ADDRESS, ADC12RST_BIT_OFFSET, ADC12RST_BIT_WIDTH) as u8 }
	/// ADC1 and ADC2 reset (Width: 1, Offset: 28)
	pub fn set_adc12rst(value: u8) { ::write(REGISTER_ADDRESS, ADC12RST_BIT_OFFSET, ADC12RST_BIT_WIDTH, value as u32); }

	const ADC34RST_BIT_OFFSET: u8 = 29;
	const ADC34RST_BIT_WIDTH: u8 = 1;
	/// ADC3 and ADC4 reset (Width: 1, Offset: 29)
	pub fn get_adc34rst() -> u8 { ::read(REGISTER_ADDRESS, ADC34RST_BIT_OFFSET, ADC34RST_BIT_WIDTH) as u8 }
	/// ADC3 and ADC4 reset (Width: 1, Offset: 29)
	pub fn set_adc34rst(value: u8) { ::write(REGISTER_ADDRESS, ADC34RST_BIT_OFFSET, ADC34RST_BIT_WIDTH, value as u32); }
}
/// Clock configuration register 2
/// Size: 0x20 bits
pub mod cfgr2 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x2C;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const PREDIV_BIT_OFFSET: u8 = 0;
	const PREDIV_BIT_WIDTH: u8 = 4;
	/// PREDIV division factor (Width: 4, Offset: 0)
	pub fn get_prediv() -> u8 { ::read(REGISTER_ADDRESS, PREDIV_BIT_OFFSET, PREDIV_BIT_WIDTH) as u8 }
	/// PREDIV division factor (Width: 4, Offset: 0)
	pub fn set_prediv(value: u8) { ::write(REGISTER_ADDRESS, PREDIV_BIT_OFFSET, PREDIV_BIT_WIDTH, value as u32); }

	const ADC12PRES_BIT_OFFSET: u8 = 4;
	const ADC12PRES_BIT_WIDTH: u8 = 5;
	/// ADC1 and ADC2 prescaler (Width: 5, Offset: 4)
	pub fn get_adc12pres() -> u8 { ::read(REGISTER_ADDRESS, ADC12PRES_BIT_OFFSET, ADC12PRES_BIT_WIDTH) as u8 }
	/// ADC1 and ADC2 prescaler (Width: 5, Offset: 4)
	pub fn set_adc12pres(value: u8) { ::write(REGISTER_ADDRESS, ADC12PRES_BIT_OFFSET, ADC12PRES_BIT_WIDTH, value as u32); }

	const ADC34PRES_BIT_OFFSET: u8 = 9;
	const ADC34PRES_BIT_WIDTH: u8 = 5;
	/// ADC3 and ADC4 prescaler (Width: 5, Offset: 9)
	pub fn get_adc34pres() -> u8 { ::read(REGISTER_ADDRESS, ADC34PRES_BIT_OFFSET, ADC34PRES_BIT_WIDTH) as u8 }
	/// ADC3 and ADC4 prescaler (Width: 5, Offset: 9)
	pub fn set_adc34pres(value: u8) { ::write(REGISTER_ADDRESS, ADC34PRES_BIT_OFFSET, ADC34PRES_BIT_WIDTH, value as u32); }
}
/// Clock configuration register 3
/// Size: 0x20 bits
pub mod cfgr3 {
	const REGISTER_ADDRESS_OFFSET: u32 = 0x30;
	const REGISTER_ADDRESS: u32 = super::BASE_ADDRESS + REGISTER_ADDRESS_OFFSET;

	const USART1SW_BIT_OFFSET: u8 = 0;
	const USART1SW_BIT_WIDTH: u8 = 2;
	/// USART1 clock source selection (Width: 2, Offset: 0)
	pub fn get_usart1sw() -> u8 { ::read(REGISTER_ADDRESS, USART1SW_BIT_OFFSET, USART1SW_BIT_WIDTH) as u8 }
	/// USART1 clock source selection (Width: 2, Offset: 0)
	pub fn set_usart1sw(value: u8) { ::write(REGISTER_ADDRESS, USART1SW_BIT_OFFSET, USART1SW_BIT_WIDTH, value as u32); }

	const I2C1SW_BIT_OFFSET: u8 = 4;
	const I2C1SW_BIT_WIDTH: u8 = 1;
	/// I2C1 clock source selection (Width: 1, Offset: 4)
	pub fn get_i2c1sw() -> u8 { ::read(REGISTER_ADDRESS, I2C1SW_BIT_OFFSET, I2C1SW_BIT_WIDTH) as u8 }
	/// I2C1 clock source selection (Width: 1, Offset: 4)
	pub fn set_i2c1sw(value: u8) { ::write(REGISTER_ADDRESS, I2C1SW_BIT_OFFSET, I2C1SW_BIT_WIDTH, value as u32); }

	const I2C2SW_BIT_OFFSET: u8 = 5;
	const I2C2SW_BIT_WIDTH: u8 = 1;
	/// I2C2 clock source selection (Width: 1, Offset: 5)
	pub fn get_i2c2sw() -> u8 { ::read(REGISTER_ADDRESS, I2C2SW_BIT_OFFSET, I2C2SW_BIT_WIDTH) as u8 }
	/// I2C2 clock source selection (Width: 1, Offset: 5)
	pub fn set_i2c2sw(value: u8) { ::write(REGISTER_ADDRESS, I2C2SW_BIT_OFFSET, I2C2SW_BIT_WIDTH, value as u32); }

	const USART2SW_BIT_OFFSET: u8 = 16;
	const USART2SW_BIT_WIDTH: u8 = 2;
	/// USART2 clock source selection (Width: 2, Offset: 16)
	pub fn get_usart2sw() -> u8 { ::read(REGISTER_ADDRESS, USART2SW_BIT_OFFSET, USART2SW_BIT_WIDTH) as u8 }
	/// USART2 clock source selection (Width: 2, Offset: 16)
	pub fn set_usart2sw(value: u8) { ::write(REGISTER_ADDRESS, USART2SW_BIT_OFFSET, USART2SW_BIT_WIDTH, value as u32); }

	const USART3SW_BIT_OFFSET: u8 = 18;
	const USART3SW_BIT_WIDTH: u8 = 2;
	/// USART3 clock source selection (Width: 2, Offset: 18)
	pub fn get_usart3sw() -> u8 { ::read(REGISTER_ADDRESS, USART3SW_BIT_OFFSET, USART3SW_BIT_WIDTH) as u8 }
	/// USART3 clock source selection (Width: 2, Offset: 18)
	pub fn set_usart3sw(value: u8) { ::write(REGISTER_ADDRESS, USART3SW_BIT_OFFSET, USART3SW_BIT_WIDTH, value as u32); }

	const TIM1SW_BIT_OFFSET: u8 = 8;
	const TIM1SW_BIT_WIDTH: u8 = 1;
	/// Timer1 clock source selection (Width: 1, Offset: 8)
	pub fn get_tim1sw() -> u8 { ::read(REGISTER_ADDRESS, TIM1SW_BIT_OFFSET, TIM1SW_BIT_WIDTH) as u8 }
	/// Timer1 clock source selection (Width: 1, Offset: 8)
	pub fn set_tim1sw(value: u8) { ::write(REGISTER_ADDRESS, TIM1SW_BIT_OFFSET, TIM1SW_BIT_WIDTH, value as u32); }

	const TIM8SW_BIT_OFFSET: u8 = 9;
	const TIM8SW_BIT_WIDTH: u8 = 1;
	/// Timer8 clock source selection (Width: 1, Offset: 9)
	pub fn get_tim8sw() -> u8 { ::read(REGISTER_ADDRESS, TIM8SW_BIT_OFFSET, TIM8SW_BIT_WIDTH) as u8 }
	/// Timer8 clock source selection (Width: 1, Offset: 9)
	pub fn set_tim8sw(value: u8) { ::write(REGISTER_ADDRESS, TIM8SW_BIT_OFFSET, TIM8SW_BIT_WIDTH, value as u32); }

	const UART4SW_BIT_OFFSET: u8 = 20;
	const UART4SW_BIT_WIDTH: u8 = 2;
	/// UART4 clock source selection (Width: 2, Offset: 20)
	pub fn get_uart4sw() -> u8 { ::read(REGISTER_ADDRESS, UART4SW_BIT_OFFSET, UART4SW_BIT_WIDTH) as u8 }
	/// UART4 clock source selection (Width: 2, Offset: 20)
	pub fn set_uart4sw(value: u8) { ::write(REGISTER_ADDRESS, UART4SW_BIT_OFFSET, UART4SW_BIT_WIDTH, value as u32); }

	const UART5SW_BIT_OFFSET: u8 = 22;
	const UART5SW_BIT_WIDTH: u8 = 2;
	/// UART5 clock source selection (Width: 2, Offset: 22)
	pub fn get_uart5sw() -> u8 { ::read(REGISTER_ADDRESS, UART5SW_BIT_OFFSET, UART5SW_BIT_WIDTH) as u8 }
	/// UART5 clock source selection (Width: 2, Offset: 22)
	pub fn set_uart5sw(value: u8) { ::write(REGISTER_ADDRESS, UART5SW_BIT_OFFSET, UART5SW_BIT_WIDTH, value as u32); }
}
/// RCC global interrupt
pub const INTERRUPT_RCC: u32 = 5;

/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <name>RCC</name>
  <description>Reset and clock control</description>
  <groupName>RCC</groupName>
  <baseAddress>0x40021000</baseAddress>
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <registers>
    <register>
      <name>CR</name>
      <displayName>CR</displayName>
      <description>Clock control register</description>
      <addressOffset>0x0</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000083</resetValue>
      <fields>
        <field>
          <name>HSION</name>
          <description>Internal High Speed clock
              enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>HSIRDY</name>
          <description>Internal High Speed clock ready
              flag</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>HSITRIM</name>
          <description>Internal High Speed clock
              trimming</description>
          <bitOffset>3</bitOffset>
          <bitWidth>5</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>HSICAL</name>
          <description>Internal High Speed clock
              Calibration</description>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>HSEON</name>
          <description>External High Speed clock
              enable</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>HSERDY</name>
          <description>External High Speed clock ready
              flag</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>HSEBYP</name>
          <description>External High Speed clock
              Bypass</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>CSSON</name>
          <description>Clock Security System
              enable</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PLLON</name>
          <description>PLL enable</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PLLRDY</name>
          <description>PLL clock ready flag</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
      </fields>
    </register>
    <register>
      <name>CFGR</name>
      <displayName>CFGR</displayName>
      <description>Clock configuration register
          (RCC_CFGR)</description>
      <addressOffset>0x4</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SW</name>
          <description>System clock Switch</description>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SWS</name>
          <description>System Clock Switch Status</description>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>HPRE</name>
          <description>AHB prescaler</description>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PPRE1</name>
          <description>APB Low speed prescaler
              (APB1)</description>
          <bitOffset>8</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PPRE2</name>
          <description>APB high speed prescaler
              (APB2)</description>
          <bitOffset>11</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PLLSRC</name>
          <description>PLL entry clock source</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PLLXTPRE</name>
          <description>HSE divider for PLL entry</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PLLMUL</name>
          <description>PLL Multiplication Factor</description>
          <bitOffset>18</bitOffset>
          <bitWidth>4</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>USBPRES</name>
          <description>USB prescaler</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>MCO</name>
          <description>Microcontroller clock
              output</description>
          <bitOffset>24</bitOffset>
          <bitWidth>3</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>MCOF</name>
          <description>Microcontroller Clock Output
              Flag</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>I2SSRC</name>
          <description>I2S external clock source
              selection</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>CIR</name>
      <displayName>CIR</displayName>
      <description>Clock interrupt register
          (RCC_CIR)</description>
      <addressOffset>0x8</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>LSIRDYF</name>
          <description>LSI Ready Interrupt flag</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>LSERDYF</name>
          <description>LSE Ready Interrupt flag</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>HSIRDYF</name>
          <description>HSI Ready Interrupt flag</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>HSERDYF</name>
          <description>HSE Ready Interrupt flag</description>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>PLLRDYF</name>
          <description>PLL Ready Interrupt flag</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>CSSF</name>
          <description>Clock Security System Interrupt
              flag</description>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>LSIRDYIE</name>
          <description>LSI Ready Interrupt Enable</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>LSERDYIE</name>
          <description>LSE Ready Interrupt Enable</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>HSIRDYIE</name>
          <description>HSI Ready Interrupt Enable</description>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>HSERDYIE</name>
          <description>HSE Ready Interrupt Enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PLLRDYIE</name>
          <description>PLL Ready Interrupt Enable</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>LSIRDYC</name>
          <description>LSI Ready Interrupt Clear</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <access>write-only</access>
        </field>
        <field>
          <name>LSERDYC</name>
          <description>LSE Ready Interrupt Clear</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <access>write-only</access>
        </field>
        <field>
          <name>HSIRDYC</name>
          <description>HSI Ready Interrupt Clear</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <access>write-only</access>
        </field>
        <field>
          <name>HSERDYC</name>
          <description>HSE Ready Interrupt Clear</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <access>write-only</access>
        </field>
        <field>
          <name>PLLRDYC</name>
          <description>PLL Ready Interrupt Clear</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <access>write-only</access>
        </field>
        <field>
          <name>CSSC</name>
          <description>Clock security system interrupt
              clear</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <access>write-only</access>
        </field>
      </fields>
    </register>
    <register>
      <name>APB2RSTR</name>
      <displayName>APB2RSTR</displayName>
      <description>APB2 peripheral reset register
          (RCC_APB2RSTR)</description>
      <addressOffset>0xC</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SYSCFGRST</name>
          <description>SYSCFG and COMP reset</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM1RST</name>
          <description>TIM1 timer reset</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SPI1RST</name>
          <description>SPI 1 reset</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM8RST</name>
          <description>TIM8 timer reset</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>USART1RST</name>
          <description>USART1 reset</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM15RST</name>
          <description>TIM15 timer reset</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM16RST</name>
          <description>TIM16 timer reset</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM17RST</name>
          <description>TIM17 timer reset</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>APB1RSTR</name>
      <displayName>APB1RSTR</displayName>
      <description>APB1 peripheral reset register
          (RCC_APB1RSTR)</description>
      <addressOffset>0x10</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>TIM2RST</name>
          <description>Timer 2 reset</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM3RST</name>
          <description>Timer 3 reset</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM4RST</name>
          <description>Timer 14 reset</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM6RST</name>
          <description>Timer 6 reset</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM7RST</name>
          <description>Timer 7 reset</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>WWDGRST</name>
          <description>Window watchdog reset</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SPI2RST</name>
          <description>SPI2 reset</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SPI3RST</name>
          <description>SPI3 reset</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>USART2RST</name>
          <description>USART 2 reset</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>USART3RST</name>
          <description>USART3 reset</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>UART4RST</name>
          <description>UART 4 reset</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>UART5RST</name>
          <description>UART 5 reset</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2C1RST</name>
          <description>I2C1 reset</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2C2RST</name>
          <description>I2C2 reset</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>USBRST</name>
          <description>USB reset</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CANRST</name>
          <description>CAN reset</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PWRRST</name>
          <description>Power interface reset</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DACRST</name>
          <description>DAC interface reset</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>AHBENR</name>
      <displayName>AHBENR</displayName>
      <description>AHB Peripheral Clock enable register
          (RCC_AHBENR)</description>
      <addressOffset>0x14</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000014</resetValue>
      <fields>
        <field>
          <name>DMAEN</name>
          <description>DMA1 clock enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DMA2EN</name>
          <description>DMA2 clock enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SRAMEN</name>
          <description>SRAM interface clock
              enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>FLITFEN</name>
          <description>FLITF clock enable</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CRCEN</name>
          <description>CRC clock enable</description>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IOPAEN</name>
          <description>I/O port A clock enable</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IOPBEN</name>
          <description>I/O port B clock enable</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IOPCEN</name>
          <description>I/O port C clock enable</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IOPDEN</name>
          <description>I/O port D clock enable</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IOPEEN</name>
          <description>I/O port E clock enable</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IOPFEN</name>
          <description>I/O port F clock enable</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TSCEN</name>
          <description>Touch sensing controller clock
              enable</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADC12EN</name>
          <description>ADC1 and ADC2 clock enable</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADC34EN</name>
          <description>ADC3 and ADC4 clock enable</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>APB2ENR</name>
      <displayName>APB2ENR</displayName>
      <description>APB2 peripheral clock enable register
          (RCC_APB2ENR)</description>
      <addressOffset>0x18</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>SYSCFGEN</name>
          <description>SYSCFG clock enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM1EN</name>
          <description>TIM1 Timer clock enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SPI1EN</name>
          <description>SPI 1 clock enable</description>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM8EN</name>
          <description>TIM8 Timer clock enable</description>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>USART1EN</name>
          <description>USART1 clock enable</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM15EN</name>
          <description>TIM15 timer clock enable</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM16EN</name>
          <description>TIM16 timer clock enable</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM17EN</name>
          <description>TIM17 timer clock enable</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>APB1ENR</name>
      <displayName>APB1ENR</displayName>
      <description>APB1 peripheral clock enable register
          (RCC_APB1ENR)</description>
      <addressOffset>0x1C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>TIM2EN</name>
          <description>Timer 2 clock enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM3EN</name>
          <description>Timer 3 clock enable</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM4EN</name>
          <description>Timer 4 clock enable</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM6EN</name>
          <description>Timer 6 clock enable</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM7EN</name>
          <description>Timer 7 clock enable</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>WWDGEN</name>
          <description>Window watchdog clock
              enable</description>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SPI2EN</name>
          <description>SPI 2 clock enable</description>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>SPI3EN</name>
          <description>SPI 3 clock enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>USART2EN</name>
          <description>USART 2 clock enable</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2C1EN</name>
          <description>I2C 1 clock enable</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2C2EN</name>
          <description>I2C 2 clock enable</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>USBEN</name>
          <description>USB clock enable</description>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>CANEN</name>
          <description>CAN clock enable</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>PWREN</name>
          <description>Power interface clock
              enable</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>DACEN</name>
          <description>DAC interface clock enable</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>BDCR</name>
      <displayName>BDCR</displayName>
      <description>Backup domain control register
          (RCC_BDCR)</description>
      <addressOffset>0x20</addressOffset>
      <size>0x20</size>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>LSEON</name>
          <description>External Low Speed oscillator
              enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>LSERDY</name>
          <description>External Low Speed oscillator
              ready</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>LSEBYP</name>
          <description>External Low Speed oscillator
              bypass</description>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>LSEDRV</name>
          <description>LSE oscillator drive
              capability</description>
          <bitOffset>3</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>RTCSEL</name>
          <description>RTC clock source selection</description>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>RTCEN</name>
          <description>RTC clock enable</description>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>BDRST</name>
          <description>Backup domain software
              reset</description>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>CSR</name>
      <displayName>CSR</displayName>
      <description>Control/status register
          (RCC_CSR)</description>
      <addressOffset>0x24</addressOffset>
      <size>0x20</size>
      <resetValue>0x0C000000</resetValue>
      <fields>
        <field>
          <name>LSION</name>
          <description>Internal low speed oscillator
              enable</description>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>LSIRDY</name>
          <description>Internal low speed oscillator
              ready</description>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-only</access>
        </field>
        <field>
          <name>RMVF</name>
          <description>Remove reset flag</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>OBLRSTF</name>
          <description>Option byte loader reset
              flag</description>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PINRSTF</name>
          <description>PIN reset flag</description>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>PORRSTF</name>
          <description>POR/PDR reset flag</description>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>SFTRSTF</name>
          <description>Software reset flag</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>IWDGRSTF</name>
          <description>Independent watchdog reset
              flag</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>WWDGRSTF</name>
          <description>Window watchdog reset flag</description>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
        <field>
          <name>LPWRRSTF</name>
          <description>Low-power reset flag</description>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <access>read-write</access>
        </field>
      </fields>
    </register>
    <register>
      <name>AHBRSTR</name>
      <displayName>AHBRSTR</displayName>
      <description>AHB peripheral reset register</description>
      <addressOffset>0x28</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>IOPARST</name>
          <description>I/O port A reset</description>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IOPBRST</name>
          <description>I/O port B reset</description>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IOPCRST</name>
          <description>I/O port C reset</description>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IOPDRST</name>
          <description>I/O port D reset</description>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IOPERST</name>
          <description>I/O port E reset</description>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>IOPFRST</name>
          <description>I/O port F reset</description>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TSCRST</name>
          <description>Touch sensing controller
              reset</description>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADC12RST</name>
          <description>ADC1 and ADC2 reset</description>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>ADC34RST</name>
          <description>ADC3 and ADC4 reset</description>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CFGR2</name>
      <displayName>CFGR2</displayName>
      <description>Clock configuration register 2</description>
      <addressOffset>0x2C</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>PREDIV</name>
          <description>PREDIV division factor</description>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
        </field>
        <field>
          <name>ADC12PRES</name>
          <description>ADC1 and ADC2 prescaler</description>
          <bitOffset>4</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
        <field>
          <name>ADC34PRES</name>
          <description>ADC3 and ADC4 prescaler</description>
          <bitOffset>9</bitOffset>
          <bitWidth>5</bitWidth>
        </field>
      </fields>
    </register>
    <register>
      <name>CFGR3</name>
      <displayName>CFGR3</displayName>
      <description>Clock configuration register 3</description>
      <addressOffset>0x30</addressOffset>
      <size>0x20</size>
      <access>read-write</access>
      <resetValue>0x00000000</resetValue>
      <fields>
        <field>
          <name>USART1SW</name>
          <description>USART1 clock source
              selection</description>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>I2C1SW</name>
          <description>I2C1 clock source
              selection</description>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>I2C2SW</name>
          <description>I2C2 clock source
              selection</description>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>USART2SW</name>
          <description>USART2 clock source
              selection</description>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>USART3SW</name>
          <description>USART3 clock source
              selection</description>
          <bitOffset>18</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>TIM1SW</name>
          <description>Timer1 clock source
              selection</description>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>TIM8SW</name>
          <description>Timer8 clock source
              selection</description>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
        </field>
        <field>
          <name>UART4SW</name>
          <description>UART4 clock source
              selection</description>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
        <field>
          <name>UART5SW</name>
          <description>UART5 clock source
              selection</description>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
        </field>
      </fields>
    </register>
  </registers>
  <interrupt>
    <name>RCC</name>
    <description>RCC global interrupt</description>
    <value>5</value>
  </interrupt>
</peripheral>*/
