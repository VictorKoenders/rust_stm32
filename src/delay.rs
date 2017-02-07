use {address, rcc};

const TIM7:       u32 = 0x4000_1400;
const ARR_OFFSET: u32 =      0x002C;
const PSC_OFFSET: u32 =      0x0028;
const EGR_OFFSET: u32 =      0x0014;
const SR_OFFSET:  u32 =      0x0010;
const CR1_OFFSET: u32 =      0x0000;

#[allow(dead_code)]
pub unsafe fn init(){
    let mut apb1 = rcc::APB1::read();
    apb1.tim7en = true;
    apb1.write();
    
    address::clear_bit_u32(TIM7 + CR1_OFFSET, 0);
    address::set_bit_u32(TIM7 + CR1_OFFSET, 3);
    
    const MASK: u16 = 65535;
    let mut value = address::read_u32(TIM7 + PSC_OFFSET);
    value &= !(MASK as u32);
    value |= (7_999 & MASK) as u32;
    address::write_u32(TIM7 + PSC_OFFSET, value);
}

#[allow(dead_code)]
pub fn ms(interval: u16) {
    // The alarm (the "update event") will set off in `n` "ticks".
    // One tick = 1 ms (see `init`)
    address::write_u16(TIM7 + ARR_OFFSET, interval);

    // Generate an update event to update the autoreload register
    address::write_u32(TIM7 + EGR_OFFSET, 1 << 0u8);

    // Clear any previous "update" event by clearing the update event flag
    let _ = address::read_u32(TIM7 + SR_OFFSET);
    address::write_u32(TIM7 + SR_OFFSET, 0);

    // CEN: Enable the counter
    address::set_bit_u32(TIM7 + CR1_OFFSET, 0);

    // Wait until the alarm goes off (the "update event" occurs)
    while !address::get_bit_u32(TIM7 + SR_OFFSET, 0) {}

    // Clear the "update" flag
    address::write_u32(TIM7 + SR_OFFSET, 0);
}