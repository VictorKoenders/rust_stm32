macro_rules! implement_gpio_side {
    ($name:ident, $address:expr, $max_pin:expr) => {
        #[allow(dead_code)]
        pub mod $name {
            use address;

            const ADDRESS: u32 = $address;
            const ODR_OFFSET: u32 = 0x14;
            const BSRR_OFFSET: u32 = 0x18;
            const INIT_OFFSET: u32 = 0;
            const MAX_PIN: u8 = $max_pin;

            pub fn set(pin: u8) {
                debug_assert!(pin < MAX_PIN);
                address::write_u32(ADDRESS + BSRR_OFFSET, 1 << pin);
            }

            pub fn clear(pin: u8) {
                debug_assert!(pin < MAX_PIN);
                address::write_u32(ADDRESS + BSRR_OFFSET, 1 << (pin + MAX_PIN));
            }

            pub fn is_high(pin: u8) -> bool {
                debug_assert!(pin < MAX_PIN);
                let value = address::read_u16(ADDRESS + ODR_OFFSET);
                unsafe { bkpt!(); } 
                value & (1 << pin) != 0
            }

            pub fn configure_pin_as_output(pin: u8){
                debug_assert!(pin < MAX_PIN);
                let mut bits = address::read_u32(ADDRESS + INIT_OFFSET);
                
                let offset: u8 = pin * 2;
                bits &= !(2 << offset);
                bits |= 1 << offset; 

                address::write_u32(ADDRESS + INIT_OFFSET, bits);
            }

            pub fn configure_pin_as_input(pin: u8){
                debug_assert!(pin < MAX_PIN);
                let mut bits = address::read_u32(ADDRESS + INIT_OFFSET);
                
                let offset: u8 = pin * 2;
                bits &= !(1 << offset);
                bits |= 2 << offset; 

                address::write_u32(ADDRESS + INIT_OFFSET, bits);
            }
        }
    }
}

//implement_gpio_side!(b, 0x4800_)
implement_gpio_side!(a, 0x4800_0000, 16);
implement_gpio_side!(b, 0x4800_0400, 16);
implement_gpio_side!(c, 0x4800_0800, 16);
implement_gpio_side!(d, 0x4800_0C00, 16);
implement_gpio_side!(e, 0x4800_1000, 16);
implement_gpio_side!(f, 0x4800_1400, 16);