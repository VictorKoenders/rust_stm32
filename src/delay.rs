use address;

const TIM7:       u32 = 0x4000_1400;
const ARR_OFFSET: u32 =      0x002C;
const EGR_OFFSET: u32 =      0x0014;
const SR_OFFSET:  u32 =      0x0010;
const CR1_OFFSET: u32 =      0x0000;

pub unsafe fn init(){
    /*
    let rcc = peripheral::rcc_mut();
    let tim7 = peripheral::tim7_mut();

    // RCC: Enable TIM7
    rcc.apb1enr.modify(|_, w| w.tim7en(true));

    // CEN: Disable the clock
    // OPM. Enable One Pulse Mode. Stop the counter after the next update event.
    tim7.cr1.write(|w| w.cen(false).opm(true));

    // Set pre-scaler to 8_000 -> Frequency = 1 KHz
    tim7.psc.write(|w| w.psc(7_999));
    */
}

pub fn ms(interval: u32) {
    // tim7.arr.write(|w| w.arr(n));
    // The alarm (the "update event") will set off in `n` "ticks".
    // One tick = 1 ms (see `init`)
    address::write_u32(TIM7 + ARR_OFFSET, interval);

    // tim7.egr.write(|w| w.ug(true));
    // Generate an update event to update the autoreload register
    address::write_u32(TIM7 + EGR_OFFSET, 1 << 0u8);

    // tim7.sr.read();
    // tim7.sr.write(|w| w);
    // Clear any previous "update" event by clearing the update event flag
    let _ = ptr::read_volatile((TIM7 + SR_OFFSET) as *mut u32);
    ptr::write_volatile((TIM7 + SR_OFFSET) as *mut u32, 0);

    // tim7.cr1.modify(|_, w| w.cen(true));
    // CEN: Enable the counter
    let mut bits = ptr::read_volatile((TIM7 + CR1_OFFSET) as *mut u32);
    bits |= 1 << 8;
    ptr::write_volatile((TIM7 + CR1_OFFSET) as *mut u32, bits);

    // while !tim7.sr.read().uif() {}
    // Wait until the alarm goes off (the "update event" occurs)
    while ptr::read_volatile((TIM7 + SR_OFFSET) as *mut u32) & (1 << 8) != 0 {}



    /*

    // Wait until the alarm goes off (the "update event" occurs)
    while !tim7.sr.read().uif() {}

    // Clear the "update" flag
    tim7.sr.write(|w| w);
    */
}