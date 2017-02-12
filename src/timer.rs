use peripheral::tims::*;

pub struct Timer {
    slot: TimerSlot
}

impl Timer {
    pub fn new(slot: TimerSlot) -> Timer {
        match slot {
            TimerSlot::TIM7 => {
                let mut apb1enr = ::peripheral::rcc::apb1enr::modify();
                apb1enr.tim7en = true;
                apb1enr.save();

                let mut cr1 = tim7::cr1::modify();
                cr1.cen = false;
                cr1.opm = true;
                cr1.save();

                let mut psc = tim7::psc::modify();
                psc.psc = 7_999;
                psc.save();
            }
        }
        Timer {
            slot: slot
        }
    }

    pub fn sleep_ms(&self, time: u16) {
        match self.slot {
            TimerSlot::TIM7 => {
                let mut arr = tim7::arr::modify();
                arr.arr = time;
                arr.save();

                tim7::egr::ug(true);

                let _ = tim7::sr::load();
                let mut sr = tim7::sr::modify();
                sr.uif = false;
                sr.save();

                let mut cr1 = tim7::cr1::modify();
                cr1.cen = true;
                cr1.save();

                while !tim7::sr::load().uif {}

                let mut sr = tim7::sr::modify();
                sr.uif = false;
                sr.save();
            }
        }
    }
}

pub enum TimerSlot {
    TIM7
}
