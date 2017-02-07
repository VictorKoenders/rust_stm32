use f3::peripheral;
use peripheral::usart::Usart;

pub struct USB {
    usart: &'static mut Usart,
}

impl USB {
    pub fn new() -> USB {
        USB {
            usart: unsafe { peripheral::usart2_mut() },
        }
    }

    pub fn read_byte(&self) -> u8 {
        while !self.usart.isr.read().rxne() {}
        self.usart.rdr.read().rdr() as u8
    }

    pub fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            while !self.usart.isr.read().txe() {}
            self.usart.tdr.write(|w| w.tdr(u16::from(*byte)));
        }
    }
}
