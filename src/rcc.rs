use address;

const RCC_ADDRESS: u32 = 0x40021000;
const AHBENR_OFFSET: u32 = 0x14;

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

#[inline]
fn bit_is_set(value: u32, bit: u8) -> bool {
    value & (1 << bit) > 0
}