use address;

const RCC_ADDRESS: u32 = 0x40021000;

macro_rules! implement {
    ($name:ident, $base_addr:expr, $($field:ident = $bit:expr),*) => {
        pub struct $name {
            $(
                pub $field: bool,
            )*
        }

        impl $name {
            pub fn read() -> $name {
                let bits = address::read_u32(RCC_ADDRESS + $base_addr);
                $name {
                    $(
                        $field: bit_is_set(bits, $bit),
                    )*
                }
            }

            pub fn write(self) {
                let bits = 0 $(
                    | ((self.$field as u32) << $bit)
                )*
                ;
                address::write_u32(RCC_ADDRESS + $base_addr, bits);
            }
        }
    }
}

implement!(Side, 0x14, 
    dmaen = 0,
    dma2en = 1,
    sramen = 2,
    flitfen = 4,
    crcen = 6,
    a = 17,
    b = 18,
    c = 19,
    d = 20,
    e = 21,
    f = 22,
    tscen = 24,
    adc12en = 28,
    adc34en = 29
);

implement!(APB1, 0x1C, 
    tim2en = 0,
    tim3en = 1,
    tim4en = 2,
    tim6en = 4,
    tim7en = 5,
    wwdgen = 11,
    spi2en = 14,
    spi3en = 15,
    usart2en = 17,
    i2c1en = 21,
    i2c2en = 22,
    usben = 23,
    canen = 25,
    pwren = 28,
    dacen = 29
);
/*
const SIDE_E_POSITION: u8 = 21;

pub struct Side {
    pub e: bool,
}

impl Side {
    pub fn load() -> Side {
        let bits = address::read_u32(RCC_ADDRESS + AHBENR_OFFSET);
        Side {
            e: bit_is_set(bits, SIDE_E_POSITION)
        }
    }

    pub fn write(self) {
        let bits = 
            (self.e as u32) << SIDE_E_POSITION
        ;

        address::write_u32(RCC_ADDRESS + AHBENR_OFFSET, bits);
    }
}
*/
#[inline]
fn bit_is_set(value: u32, bit: u8) -> bool {
    value & (1 << bit) > 0
}