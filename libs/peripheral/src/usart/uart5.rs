/// Control register 1
pub mod cr1 {
    pub struct ReadonlyCache {
        /// End of Block interrupt enable
        pub eobie: bool,
        /// Receiver timeout interrupt enable
        pub rtoie: bool,
        /// Driver Enable assertion time
        pub deat: bool,
        /// Driver Enable deassertion time
        pub dedt: bool,
        /// Oversampling mode
        pub over8: bool,
        /// Character match interrupt enable
        pub cmie: bool,
        /// Mute mode enable
        pub mme: bool,
        /// Word length
        pub m: bool,
        /// Receiver wakeup method
        pub wake: bool,
        /// Parity control enable
        pub pce: bool,
        /// Parity selection
        pub ps: bool,
        /// PE interrupt enable
        pub peie: bool,
        /// interrupt enable
        pub txeie: bool,
        /// Transmission complete interrupt enable
        pub tcie: bool,
        /// RXNE interrupt enable
        pub rxneie: bool,
        /// IDLE interrupt enable
        pub idleie: bool,
        /// Transmitter enable
        pub te: bool,
        /// Receiver enable
        pub re: bool,
        /// USART enable in Stop mode
        pub uesm: bool,
        /// USART enable
        pub ue: bool,
    }
    pub struct Cache {
        /// End of Block interrupt enable
        pub eobie: bool,
        /// Receiver timeout interrupt enable
        pub rtoie: bool,
        /// Driver Enable assertion time
        pub deat: bool,
        /// Driver Enable deassertion time
        pub dedt: bool,
        /// Oversampling mode
        pub over8: bool,
        /// Character match interrupt enable
        pub cmie: bool,
        /// Mute mode enable
        pub mme: bool,
        /// Word length
        pub m: bool,
        /// Receiver wakeup method
        pub wake: bool,
        /// Parity control enable
        pub pce: bool,
        /// Parity selection
        pub ps: bool,
        /// PE interrupt enable
        pub peie: bool,
        /// interrupt enable
        pub txeie: bool,
        /// Transmission complete interrupt enable
        pub tcie: bool,
        /// RXNE interrupt enable
        pub rxneie: bool,
        /// IDLE interrupt enable
        pub idleie: bool,
        /// Transmitter enable
        pub te: bool,
        /// Receiver enable
        pub re: bool,
        /// USART enable in Stop mode
        pub uesm: bool,
        /// USART enable
        pub ue: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            eobie: ((value >> 27) & 0b1) > 0,
            rtoie: ((value >> 26) & 0b1) > 0,
            deat: ((value >> 21) & 0b1) > 0,
            dedt: ((value >> 16) & 0b1) > 0,
            over8: ((value >> 15) & 0b1) > 0,
            cmie: ((value >> 14) & 0b1) > 0,
            mme: ((value >> 13) & 0b1) > 0,
            m: ((value >> 12) & 0b1) > 0,
            wake: ((value >> 11) & 0b1) > 0,
            pce: ((value >> 10) & 0b1) > 0,
            ps: ((value >> 9) & 0b1) > 0,
            peie: ((value >> 8) & 0b1) > 0,
            txeie: ((value >> 7) & 0b1) > 0,
            tcie: ((value >> 6) & 0b1) > 0,
            rxneie: ((value >> 5) & 0b1) > 0,
            idleie: ((value >> 4) & 0b1) > 0,
            te: ((value >> 3) & 0b1) > 0,
            re: ((value >> 2) & 0b1) > 0,
            uesm: ((value >> 1) & 0b1) > 0,
            ue: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x0u32) as *mut u32) };
        Cache {
            eobie: ((value >> 27) & 0b1) > 0,
            rtoie: ((value >> 26) & 0b1) > 0,
            deat: ((value >> 21) & 0b1) > 0,
            dedt: ((value >> 16) & 0b1) > 0,
            over8: ((value >> 15) & 0b1) > 0,
            cmie: ((value >> 14) & 0b1) > 0,
            mme: ((value >> 13) & 0b1) > 0,
            m: ((value >> 12) & 0b1) > 0,
            wake: ((value >> 11) & 0b1) > 0,
            pce: ((value >> 10) & 0b1) > 0,
            ps: ((value >> 9) & 0b1) > 0,
            peie: ((value >> 8) & 0b1) > 0,
            txeie: ((value >> 7) & 0b1) > 0,
            tcie: ((value >> 6) & 0b1) > 0,
            rxneie: ((value >> 5) & 0b1) > 0,
            idleie: ((value >> 4) & 0b1) > 0,
            te: ((value >> 3) & 0b1) > 0,
            re: ((value >> 2) & 0b1) > 0,
            uesm: ((value >> 1) & 0b1) > 0,
            ue: ((value >> 0) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.eobie as u32) << 27)
                | ((self.rtoie as u32) << 26)
                | ((self.deat as u32) << 21)
                | ((self.dedt as u32) << 16)
                | ((self.over8 as u32) << 15)
                | ((self.cmie as u32) << 14)
                | ((self.mme as u32) << 13)
                | ((self.m as u32) << 12)
                | ((self.wake as u32) << 11)
                | ((self.pce as u32) << 10)
                | ((self.ps as u32) << 9)
                | ((self.peie as u32) << 8)
                | ((self.txeie as u32) << 7)
                | ((self.tcie as u32) << 6)
                | ((self.rxneie as u32) << 5)
                | ((self.idleie as u32) << 4)
                | ((self.te as u32) << 3)
                | ((self.re as u32) << 2)
                | ((self.uesm as u32) << 1)
                | ((self.ue as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005000u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// Control register 2
pub mod cr2 {
    pub struct ReadonlyCache {
        /// Address of the USART node
        pub add4: u8,
        /// Address of the USART node
        pub add0: u8,
        /// Receiver timeout enable
        pub rtoen: u8,
        /// Auto baud rate mode
        pub abrmod: u8,
        /// Auto baud rate enable
        pub abren: u8,
        /// Most significant bit first
        pub msbfirst: u8,
        /// Binary data inversion
        pub datainv: u8,
        /// TX pin active level inversion
        pub txinv: u8,
        /// RX pin active level inversion
        pub rxinv: u8,
        /// Swap TX/RX pins
        pub swap: u8,
        /// LIN mode enable
        pub linen: u8,
        /// STOP bits
        pub stop: u8,
        /// Clock enable
        pub clken: u8,
        /// Clock polarity
        pub cpol: u8,
        /// Clock phase
        pub cpha: u8,
        /// Last bit clock pulse
        pub lbcl: u8,
        /// LIN break detection interrupt enable
        pub lbdie: u8,
        /// LIN break detection length
        pub lbdl: u8,
        /// 7-bit Address Detection/4-bit Address Detection
        pub addm7: u8,
    }
    pub struct Cache {
        /// Address of the USART node
        pub add4: u8,
        /// Address of the USART node
        pub add0: u8,
        /// Receiver timeout enable
        pub rtoen: u8,
        /// Auto baud rate mode
        pub abrmod: u8,
        /// Auto baud rate enable
        pub abren: u8,
        /// Most significant bit first
        pub msbfirst: u8,
        /// Binary data inversion
        pub datainv: u8,
        /// TX pin active level inversion
        pub txinv: u8,
        /// RX pin active level inversion
        pub rxinv: u8,
        /// Swap TX/RX pins
        pub swap: u8,
        /// LIN mode enable
        pub linen: u8,
        /// STOP bits
        pub stop: u8,
        /// Clock enable
        pub clken: u8,
        /// Clock polarity
        pub cpol: u8,
        /// Clock phase
        pub cpha: u8,
        /// Last bit clock pulse
        pub lbcl: u8,
        /// LIN break detection interrupt enable
        pub lbdie: u8,
        /// LIN break detection length
        pub lbdl: u8,
        /// 7-bit Address Detection/4-bit Address Detection
        pub addm7: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            add4: ((value >> 28) & 0b1111) as u8,
            add0: ((value >> 24) & 0b1111) as u8,
            rtoen: ((value >> 23) & 0b1111) as u8,
            abrmod: ((value >> 21) & 0b1111) as u8,
            abren: ((value >> 20) & 0b1111) as u8,
            msbfirst: ((value >> 19) & 0b1111) as u8,
            datainv: ((value >> 18) & 0b1111) as u8,
            txinv: ((value >> 17) & 0b1111) as u8,
            rxinv: ((value >> 16) & 0b1111) as u8,
            swap: ((value >> 15) & 0b1111) as u8,
            linen: ((value >> 14) & 0b1111) as u8,
            stop: ((value >> 12) & 0b1111) as u8,
            clken: ((value >> 11) & 0b1111) as u8,
            cpol: ((value >> 10) & 0b1111) as u8,
            cpha: ((value >> 9) & 0b1111) as u8,
            lbcl: ((value >> 8) & 0b1111) as u8,
            lbdie: ((value >> 6) & 0b1111) as u8,
            lbdl: ((value >> 5) & 0b1111) as u8,
            addm7: ((value >> 4) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x4u32) as *mut u32) };
        Cache {
            add4: ((value >> 28) & 0b1111) as u8,
            add0: ((value >> 24) & 0b1111) as u8,
            rtoen: ((value >> 23) & 0b1111) as u8,
            abrmod: ((value >> 21) & 0b1111) as u8,
            abren: ((value >> 20) & 0b1111) as u8,
            msbfirst: ((value >> 19) & 0b1111) as u8,
            datainv: ((value >> 18) & 0b1111) as u8,
            txinv: ((value >> 17) & 0b1111) as u8,
            rxinv: ((value >> 16) & 0b1111) as u8,
            swap: ((value >> 15) & 0b1111) as u8,
            linen: ((value >> 14) & 0b1111) as u8,
            stop: ((value >> 12) & 0b1111) as u8,
            clken: ((value >> 11) & 0b1111) as u8,
            cpol: ((value >> 10) & 0b1111) as u8,
            cpha: ((value >> 9) & 0b1111) as u8,
            lbcl: ((value >> 8) & 0b1111) as u8,
            lbdie: ((value >> 6) & 0b1111) as u8,
            lbdl: ((value >> 5) & 0b1111) as u8,
            addm7: ((value >> 4) & 0b1111) as u8,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.add4 as u32) << 28)
                | ((self.add0 as u32) << 24)
                | ((self.rtoen as u32) << 23)
                | ((self.abrmod as u32) << 21)
                | ((self.abren as u32) << 20)
                | ((self.msbfirst as u32) << 19)
                | ((self.datainv as u32) << 18)
                | ((self.txinv as u32) << 17)
                | ((self.rxinv as u32) << 16)
                | ((self.swap as u32) << 15)
                | ((self.linen as u32) << 14)
                | ((self.stop as u32) << 12)
                | ((self.clken as u32) << 11)
                | ((self.cpol as u32) << 10)
                | ((self.cpha as u32) << 9)
                | ((self.lbcl as u32) << 8)
                | ((self.lbdie as u32) << 6)
                | ((self.lbdl as u32) << 5)
                | ((self.addm7 as u32) << 4)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005000u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// Control register 3
pub mod cr3 {
    pub struct ReadonlyCache {
        /// Wakeup from Stop mode interrupt enable
        pub wufie: bool,
        /// Wakeup from Stop mode interrupt flag selection
        pub wus: bool,
        /// Smartcard auto-retry count
        pub scarcnt: bool,
        /// Driver enable polarity selection
        pub dep: bool,
        /// Driver enable mode
        pub dem: bool,
        /// DMA Disable on Reception Error
        pub ddre: bool,
        /// Overrun Disable
        pub ovrdis: bool,
        /// One sample bit method enable
        pub onebit: bool,
        /// CTS interrupt enable
        pub ctsie: bool,
        /// CTS enable
        pub ctse: bool,
        /// RTS enable
        pub rtse: bool,
        /// DMA enable transmitter
        pub dmat: bool,
        /// DMA enable receiver
        pub dmar: bool,
        /// Smartcard mode enable
        pub scen: bool,
        /// Smartcard NACK enable
        pub nack: bool,
        /// Half-duplex selection
        pub hdsel: bool,
        /// IrDA low-power
        pub irlp: bool,
        /// IrDA mode enable
        pub iren: bool,
        /// Error interrupt enable
        pub eie: bool,
    }
    pub struct Cache {
        /// Wakeup from Stop mode interrupt enable
        pub wufie: bool,
        /// Wakeup from Stop mode interrupt flag selection
        pub wus: bool,
        /// Smartcard auto-retry count
        pub scarcnt: bool,
        /// Driver enable polarity selection
        pub dep: bool,
        /// Driver enable mode
        pub dem: bool,
        /// DMA Disable on Reception Error
        pub ddre: bool,
        /// Overrun Disable
        pub ovrdis: bool,
        /// One sample bit method enable
        pub onebit: bool,
        /// CTS interrupt enable
        pub ctsie: bool,
        /// CTS enable
        pub ctse: bool,
        /// RTS enable
        pub rtse: bool,
        /// DMA enable transmitter
        pub dmat: bool,
        /// DMA enable receiver
        pub dmar: bool,
        /// Smartcard mode enable
        pub scen: bool,
        /// Smartcard NACK enable
        pub nack: bool,
        /// Half-duplex selection
        pub hdsel: bool,
        /// IrDA low-power
        pub irlp: bool,
        /// IrDA mode enable
        pub iren: bool,
        /// Error interrupt enable
        pub eie: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            wufie: ((value >> 22) & 0b1) > 0,
            wus: ((value >> 20) & 0b1) > 0,
            scarcnt: ((value >> 17) & 0b1) > 0,
            dep: ((value >> 15) & 0b1) > 0,
            dem: ((value >> 14) & 0b1) > 0,
            ddre: ((value >> 13) & 0b1) > 0,
            ovrdis: ((value >> 12) & 0b1) > 0,
            onebit: ((value >> 11) & 0b1) > 0,
            ctsie: ((value >> 10) & 0b1) > 0,
            ctse: ((value >> 9) & 0b1) > 0,
            rtse: ((value >> 8) & 0b1) > 0,
            dmat: ((value >> 7) & 0b1) > 0,
            dmar: ((value >> 6) & 0b1) > 0,
            scen: ((value >> 5) & 0b1) > 0,
            nack: ((value >> 4) & 0b1) > 0,
            hdsel: ((value >> 3) & 0b1) > 0,
            irlp: ((value >> 2) & 0b1) > 0,
            iren: ((value >> 1) & 0b1) > 0,
            eie: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x8u32) as *mut u32) };
        Cache {
            wufie: ((value >> 22) & 0b1) > 0,
            wus: ((value >> 20) & 0b1) > 0,
            scarcnt: ((value >> 17) & 0b1) > 0,
            dep: ((value >> 15) & 0b1) > 0,
            dem: ((value >> 14) & 0b1) > 0,
            ddre: ((value >> 13) & 0b1) > 0,
            ovrdis: ((value >> 12) & 0b1) > 0,
            onebit: ((value >> 11) & 0b1) > 0,
            ctsie: ((value >> 10) & 0b1) > 0,
            ctse: ((value >> 9) & 0b1) > 0,
            rtse: ((value >> 8) & 0b1) > 0,
            dmat: ((value >> 7) & 0b1) > 0,
            dmar: ((value >> 6) & 0b1) > 0,
            scen: ((value >> 5) & 0b1) > 0,
            nack: ((value >> 4) & 0b1) > 0,
            hdsel: ((value >> 3) & 0b1) > 0,
            irlp: ((value >> 2) & 0b1) > 0,
            iren: ((value >> 1) & 0b1) > 0,
            eie: ((value >> 0) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.wufie as u32) << 22)
                | ((self.wus as u32) << 20)
                | ((self.scarcnt as u32) << 17)
                | ((self.dep as u32) << 15)
                | ((self.dem as u32) << 14)
                | ((self.ddre as u32) << 13)
                | ((self.ovrdis as u32) << 12)
                | ((self.onebit as u32) << 11)
                | ((self.ctsie as u32) << 10)
                | ((self.ctse as u32) << 9)
                | ((self.rtse as u32) << 8)
                | ((self.dmat as u32) << 7)
                | ((self.dmar as u32) << 6)
                | ((self.scen as u32) << 5)
                | ((self.nack as u32) << 4)
                | ((self.hdsel as u32) << 3)
                | ((self.irlp as u32) << 2)
                | ((self.iren as u32) << 1)
                | ((self.eie as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005000u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// Baud rate register
pub mod brr {
    pub struct ReadonlyCache {
        /// mantissa of USARTDIV
        pub div_mantissa: u16,
        /// fraction of USARTDIV
        pub div_fraction: u16,
    }
    pub struct Cache {
        /// mantissa of USARTDIV
        pub div_mantissa: u16,
        /// fraction of USARTDIV
        pub div_fraction: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            div_mantissa: ((value >> 4) & 0b111111111111) as u16,
            div_fraction: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0xCu32) as *mut u32) };
        Cache {
            div_mantissa: ((value >> 4) & 0b111111111111) as u16,
            div_fraction: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.div_mantissa as u32) << 4)
                | ((self.div_fraction as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005000u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// Guard time and prescaler register
pub mod gtpr {
    pub struct ReadonlyCache {
        /// Guard time value
        pub gt: u8,
        /// Prescaler value
        pub psc: u8,
    }
    pub struct Cache {
        /// Guard time value
        pub gt: u8,
        /// Prescaler value
        pub psc: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x10u32) as *mut u32) };
        ReadonlyCache {
            gt: ((value >> 8) & 0b11111111) as u8,
            psc: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x10u32) as *mut u32) };
        Cache {
            gt: ((value >> 8) & 0b11111111) as u8,
            psc: ((value >> 0) & 0b11111111) as u8,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.gt as u32) << 8)
                | ((self.psc as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005000u32 + 0x10u32) as *mut u32, value) };
        }
    }
}
/// Receiver timeout register
pub mod rtor {
    pub struct ReadonlyCache {
        /// Block Length
        pub blen: u8,
        /// Receiver timeout value
        pub rto: u8,
    }
    pub struct Cache {
        /// Block Length
        pub blen: u8,
        /// Receiver timeout value
        pub rto: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x14u32) as *mut u32) };
        ReadonlyCache {
            blen: ((value >> 24) & 0b11111111) as u8,
            rto: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x14u32) as *mut u32) };
        Cache {
            blen: ((value >> 24) & 0b11111111) as u8,
            rto: ((value >> 0) & 0b11111111) as u8,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.blen as u32) << 24)
                | ((self.rto as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005000u32 + 0x14u32) as *mut u32, value) };
        }
    }
}
/// Request register
pub mod rqr {
    pub struct ReadonlyCache {
        /// Transmit data flush request
        pub txfrq: bool,
        /// Receive data flush request
        pub rxfrq: bool,
        /// Mute mode request
        pub mmrq: bool,
        /// Send break request
        pub sbkrq: bool,
        /// Auto baud rate request
        pub abrrq: bool,
    }
    pub struct Cache {
        /// Transmit data flush request
        pub txfrq: bool,
        /// Receive data flush request
        pub rxfrq: bool,
        /// Mute mode request
        pub mmrq: bool,
        /// Send break request
        pub sbkrq: bool,
        /// Auto baud rate request
        pub abrrq: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x18u32) as *mut u32) };
        ReadonlyCache {
            txfrq: ((value >> 4) & 0b1) > 0,
            rxfrq: ((value >> 3) & 0b1) > 0,
            mmrq: ((value >> 2) & 0b1) > 0,
            sbkrq: ((value >> 1) & 0b1) > 0,
            abrrq: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x18u32) as *mut u32) };
        Cache {
            txfrq: ((value >> 4) & 0b1) > 0,
            rxfrq: ((value >> 3) & 0b1) > 0,
            mmrq: ((value >> 2) & 0b1) > 0,
            sbkrq: ((value >> 1) & 0b1) > 0,
            abrrq: ((value >> 0) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.txfrq as u32) << 4)
                | ((self.rxfrq as u32) << 3)
                | ((self.mmrq as u32) << 2)
                | ((self.sbkrq as u32) << 1)
                | ((self.abrrq as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005000u32 + 0x18u32) as *mut u32, value) };
        }
    }
}
/// Interrupt & status register
pub mod isr {
    /// Receive enable acknowledge flag
    /// Access: read-only, Width: 1, Offset: 22
    /// Get Receive enable acknowledge flag
    pub fn reack() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 22);
        value > 0
    }
    /// Transmit enable acknowledge flag
    /// Access: read-only, Width: 1, Offset: 21
    /// Get Transmit enable acknowledge flag
    pub fn teack() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 21);
        value > 0
    }
    /// Wakeup from Stop mode flag
    /// Access: read-only, Width: 1, Offset: 20
    /// Get Wakeup from Stop mode flag
    pub fn wuf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 20);
        value > 0
    }
    /// Receiver wakeup from Mute mode
    /// Access: read-only, Width: 1, Offset: 19
    /// Get Receiver wakeup from Mute mode
    pub fn rwu() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 19);
        value > 0
    }
    /// Send break flag
    /// Access: read-only, Width: 1, Offset: 18
    /// Get Send break flag
    pub fn sbkf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 18);
        value > 0
    }
    /// character match flag
    /// Access: read-only, Width: 1, Offset: 17
    /// Get character match flag
    pub fn cmf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 17);
        value > 0
    }
    /// Busy flag
    /// Access: read-only, Width: 1, Offset: 16
    /// Get Busy flag
    pub fn busy() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 16);
        value > 0
    }
    /// Auto baud rate flag
    /// Access: read-only, Width: 1, Offset: 15
    /// Get Auto baud rate flag
    pub fn abrf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// Auto baud rate error
    /// Access: read-only, Width: 1, Offset: 14
    /// Get Auto baud rate error
    pub fn abre() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// End of block flag
    /// Access: read-only, Width: 1, Offset: 12
    /// Get End of block flag
    pub fn eobf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 12);
        value > 0
    }
    /// Receiver timeout
    /// Access: read-only, Width: 1, Offset: 11
    /// Get Receiver timeout
    pub fn rtof() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// CTS flag
    /// Access: read-only, Width: 1, Offset: 10
    /// Get CTS flag
    pub fn cts() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 10);
        value > 0
    }
    /// CTS interrupt flag
    /// Access: read-only, Width: 1, Offset: 9
    /// Get CTS interrupt flag
    pub fn ctsif() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// LIN break detection flag
    /// Access: read-only, Width: 1, Offset: 8
    /// Get LIN break detection flag
    pub fn lbdf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Transmit data register empty
    /// Access: read-only, Width: 1, Offset: 7
    /// Get Transmit data register empty
    pub fn txe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Transmission complete
    /// Access: read-only, Width: 1, Offset: 6
    /// Get Transmission complete
    pub fn tc() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Read data register not empty
    /// Access: read-only, Width: 1, Offset: 5
    /// Get Read data register not empty
    pub fn rxne() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// Idle line detected
    /// Access: read-only, Width: 1, Offset: 4
    /// Get Idle line detected
    pub fn idle() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// Overrun error
    /// Access: read-only, Width: 1, Offset: 3
    /// Get Overrun error
    pub fn ore() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// Noise detected flag
    /// Access: read-only, Width: 1, Offset: 2
    /// Get Noise detected flag
    pub fn nf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// Framing error
    /// Access: read-only, Width: 1, Offset: 1
    /// Get Framing error
    pub fn fe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// Parity error
    /// Access: read-only, Width: 1, Offset: 0
    /// Get Parity error
    pub fn pe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x1Cu32) as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// Interrupt flag clear register
pub mod icr {
    pub struct ReadonlyCache {
        /// Wakeup from Stop mode clear flag
        pub wucf: bool,
        /// Character match clear flag
        pub cmcf: bool,
        /// End of timeout clear flag
        pub eobcf: bool,
        /// Receiver timeout clear flag
        pub rtocf: bool,
        /// CTS clear flag
        pub ctscf: bool,
        /// LIN break detection clear flag
        pub lbdcf: bool,
        /// Transmission complete clear flag
        pub tccf: bool,
        /// Idle line detected clear flag
        pub idlecf: bool,
        /// Overrun error clear flag
        pub orecf: bool,
        /// Noise detected clear flag
        pub ncf: bool,
        /// Framing error clear flag
        pub fecf: bool,
        /// Parity error clear flag
        pub pecf: bool,
    }
    pub struct Cache {
        /// Wakeup from Stop mode clear flag
        pub wucf: bool,
        /// Character match clear flag
        pub cmcf: bool,
        /// End of timeout clear flag
        pub eobcf: bool,
        /// Receiver timeout clear flag
        pub rtocf: bool,
        /// CTS clear flag
        pub ctscf: bool,
        /// LIN break detection clear flag
        pub lbdcf: bool,
        /// Transmission complete clear flag
        pub tccf: bool,
        /// Idle line detected clear flag
        pub idlecf: bool,
        /// Overrun error clear flag
        pub orecf: bool,
        /// Noise detected clear flag
        pub ncf: bool,
        /// Framing error clear flag
        pub fecf: bool,
        /// Parity error clear flag
        pub pecf: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x20u32) as *mut u32) };
        ReadonlyCache {
            wucf: ((value >> 20) & 0b1) > 0,
            cmcf: ((value >> 17) & 0b1) > 0,
            eobcf: ((value >> 12) & 0b1) > 0,
            rtocf: ((value >> 11) & 0b1) > 0,
            ctscf: ((value >> 9) & 0b1) > 0,
            lbdcf: ((value >> 8) & 0b1) > 0,
            tccf: ((value >> 6) & 0b1) > 0,
            idlecf: ((value >> 4) & 0b1) > 0,
            orecf: ((value >> 3) & 0b1) > 0,
            ncf: ((value >> 2) & 0b1) > 0,
            fecf: ((value >> 1) & 0b1) > 0,
            pecf: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x20u32) as *mut u32) };
        Cache {
            wucf: ((value >> 20) & 0b1) > 0,
            cmcf: ((value >> 17) & 0b1) > 0,
            eobcf: ((value >> 12) & 0b1) > 0,
            rtocf: ((value >> 11) & 0b1) > 0,
            ctscf: ((value >> 9) & 0b1) > 0,
            lbdcf: ((value >> 8) & 0b1) > 0,
            tccf: ((value >> 6) & 0b1) > 0,
            idlecf: ((value >> 4) & 0b1) > 0,
            orecf: ((value >> 3) & 0b1) > 0,
            ncf: ((value >> 2) & 0b1) > 0,
            fecf: ((value >> 1) & 0b1) > 0,
            pecf: ((value >> 0) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.wucf as u32) << 20)
                | ((self.cmcf as u32) << 17)
                | ((self.eobcf as u32) << 12)
                | ((self.rtocf as u32) << 11)
                | ((self.ctscf as u32) << 9)
                | ((self.lbdcf as u32) << 8)
                | ((self.tccf as u32) << 6)
                | ((self.idlecf as u32) << 4)
                | ((self.orecf as u32) << 3)
                | ((self.ncf as u32) << 2)
                | ((self.fecf as u32) << 1)
                | ((self.pecf as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005000u32 + 0x20u32) as *mut u32, value) };
        }
    }
}
/// Receive data register
pub mod rdr {
    /// Receive data value
    /// Access: read-only, Width: 9, Offset: 0
    /// Get Receive data value
    pub fn rdr() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x24u32) as *mut u32) };
        let value = value & (0b111111111 << 0);
        value as u16
    }
}
/// Transmit data register
pub mod tdr {
    pub struct ReadonlyCache {
        /// Transmit data value
        pub tdr: u16,
    }
    pub struct Cache {
        /// Transmit data value
        pub tdr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x28u32) as *mut u32) };
        ReadonlyCache {
            tdr: ((value >> 0) & 0b111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x40005000u32 + 0x28u32) as *mut u32) };
        Cache {
            tdr: ((value >> 0) & 0b111111111) as u16,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call Cache::drop defined below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.tdr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x40005000u32 + 0x28u32) as *mut u32, value) };
        }
    }
}
/// UART5 global and EXTI Line 35 interrupts
pub const INTERRUPT_UART5_EXTI35: u32 = 53;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" derivedFrom="USART1">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40005000</baseAddress>
  <description>
                Universal synchronous asynchronous receiver
                transmitter
            </description>
  <groupName>USART</groupName>
  <interrupt>
    <description>
                    UART5 global and EXTI Line 35
                    interrupts
                </description>
    <name>UART5_EXTI35</name>
    <value>53</value>
  </interrupt>
  <name>UART5</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>Control register 1</description>
      <displayName>CR1</displayName>
      <fields>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                End of Block interrupt
                                enable
                            </description>
          <name>EOBIE</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Receiver timeout interrupt
                                enable
                            </description>
          <name>RTOIE</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>5</bitWidth>
          <description>
                                Driver Enable assertion
                                time
                            </description>
          <name>DEAT</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>5</bitWidth>
          <description>
                                Driver Enable deassertion
                                time
                            </description>
          <name>DEDT</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Oversampling mode</description>
          <name>OVER8</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Character match interrupt
                                enable
                            </description>
          <name>CMIE</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Mute mode enable</description>
          <name>MME</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Word length</description>
          <name>M</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receiver wakeup method</description>
          <name>WAKE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Parity control enable</description>
          <name>PCE</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Parity selection</description>
          <name>PS</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PE interrupt enable</description>
          <name>PEIE</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>interrupt enable</description>
          <name>TXEIE</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transmission complete interrupt
                                enable
                            </description>
          <name>TCIE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXNE interrupt enable</description>
          <name>RXNEIE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDLE interrupt enable</description>
          <name>IDLEIE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Transmitter enable</description>
          <name>TE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receiver enable</description>
          <name>RE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USART enable in Stop mode</description>
          <name>UESM</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USART enable</description>
          <name>UE</name>
        </field>
      </fields>
      <name>CR1</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>Control register 2</description>
      <displayName>CR2</displayName>
      <fields>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Address of the USART node</description>
          <name>ADD4</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Address of the USART node</description>
          <name>ADD0</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receiver timeout enable</description>
          <name>RTOEN</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Auto baud rate mode</description>
          <name>ABRMOD</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Auto baud rate enable</description>
          <name>ABREN</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Most significant bit first</description>
          <name>MSBFIRST</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Binary data inversion</description>
          <name>DATAINV</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                TX pin active level
                                inversion
                            </description>
          <name>TXINV</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                RX pin active level
                                inversion
                            </description>
          <name>RXINV</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Swap TX/RX pins</description>
          <name>SWAP</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LIN mode enable</description>
          <name>LINEN</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>STOP bits</description>
          <name>STOP</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clock enable</description>
          <name>CLKEN</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clock polarity</description>
          <name>CPOL</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clock phase</description>
          <name>CPHA</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Last bit clock pulse</description>
          <name>LBCL</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                LIN break detection interrupt
                                enable
                            </description>
          <name>LBDIE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LIN break detection length</description>
          <name>LBDL</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                7-bit Address Detection/4-bit Address
                                Detection
                            </description>
          <name>ADDM7</name>
        </field>
      </fields>
      <name>CR2</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>Control register 3</description>
      <displayName>CR3</displayName>
      <fields>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Wakeup from Stop mode interrupt
                                enable
                            </description>
          <name>WUFIE</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
          <description>
                                Wakeup from Stop mode interrupt flag
                                selection
                            </description>
          <name>WUS</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Smartcard auto-retry count</description>
          <name>SCARCNT</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Driver enable polarity
                                selection
                            </description>
          <name>DEP</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Driver enable mode</description>
          <name>DEM</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                DMA Disable on Reception
                                Error
                            </description>
          <name>DDRE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Overrun Disable</description>
          <name>OVRDIS</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                One sample bit method
                                enable
                            </description>
          <name>ONEBIT</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CTS interrupt enable</description>
          <name>CTSIE</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CTS enable</description>
          <name>CTSE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTS enable</description>
          <name>RTSE</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DMA enable transmitter</description>
          <name>DMAT</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DMA enable receiver</description>
          <name>DMAR</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Smartcard mode enable</description>
          <name>SCEN</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Smartcard NACK enable</description>
          <name>NACK</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Half-duplex selection</description>
          <name>HDSEL</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IrDA low-power</description>
          <name>IRLP</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IrDA mode enable</description>
          <name>IREN</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Error interrupt enable</description>
          <name>EIE</name>
        </field>
      </fields>
      <name>CR3</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>Baud rate register</description>
      <displayName>BRR</displayName>
      <fields>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>12</bitWidth>
          <description>mantissa of USARTDIV</description>
          <name>DIV_Mantissa</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>fraction of USARTDIV</description>
          <name>DIV_Fraction</name>
        </field>
      </fields>
      <name>BRR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>
                        Guard time and prescaler
                        register
                    </description>
      <displayName>GTPR</displayName>
      <fields>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Guard time value</description>
          <name>GT</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Prescaler value</description>
          <name>PSC</name>
        </field>
      </fields>
      <name>GTPR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>Receiver timeout register</description>
      <displayName>RTOR</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Block Length</description>
          <name>BLEN</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>24</bitWidth>
          <description>Receiver timeout value</description>
          <name>RTO</name>
        </field>
      </fields>
      <name>RTOR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>Request register</description>
      <displayName>RQR</displayName>
      <fields>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transmit data flush
                                request
                            </description>
          <name>TXFRQ</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receive data flush request</description>
          <name>RXFRQ</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Mute mode request</description>
          <name>MMRQ</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Send break request</description>
          <name>SBKRQ</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Auto baud rate request</description>
          <name>ABRRQ</name>
        </field>
      </fields>
      <name>RQR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1C</addressOffset>
      <description>
                        Interrupt &amp; status
                        register
                    </description>
      <displayName>ISR</displayName>
      <fields>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Receive enable acknowledge
                                flag
                            </description>
          <name>REACK</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transmit enable acknowledge
                                flag
                            </description>
          <name>TEACK</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Wakeup from Stop mode flag</description>
          <name>WUF</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Receiver wakeup from Mute
                                mode
                            </description>
          <name>RWU</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Send break flag</description>
          <name>SBKF</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>character match flag</description>
          <name>CMF</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Busy flag</description>
          <name>BUSY</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Auto baud rate flag</description>
          <name>ABRF</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Auto baud rate error</description>
          <name>ABRE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>End of block flag</description>
          <name>EOBF</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receiver timeout</description>
          <name>RTOF</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CTS flag</description>
          <name>CTS</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CTS interrupt flag</description>
          <name>CTSIF</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LIN break detection flag</description>
          <name>LBDF</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transmit data register
                                empty
                            </description>
          <name>TXE</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Transmission complete</description>
          <name>TC</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Read data register not
                                empty
                            </description>
          <name>RXNE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Idle line detected</description>
          <name>IDLE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Overrun error</description>
          <name>ORE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Noise detected flag</description>
          <name>NF</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Framing error</description>
          <name>FE</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Parity error</description>
          <name>PE</name>
        </field>
      </fields>
      <name>ISR</name>
      <resetValue>0x00C0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>Interrupt flag clear register</description>
      <displayName>ICR</displayName>
      <fields>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Wakeup from Stop mode clear
                                flag
                            </description>
          <name>WUCF</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Character match clear flag</description>
          <name>CMCF</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>End of timeout clear flag</description>
          <name>EOBCF</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Receiver timeout clear
                                flag
                            </description>
          <name>RTOCF</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CTS clear flag</description>
          <name>CTSCF</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                LIN break detection clear
                                flag
                            </description>
          <name>LBDCF</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Transmission complete clear
                                flag
                            </description>
          <name>TCCF</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>
                                Idle line detected clear
                                flag
                            </description>
          <name>IDLECF</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Overrun error clear flag</description>
          <name>ORECF</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Noise detected clear flag</description>
          <name>NCF</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Framing error clear flag</description>
          <name>FECF</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Parity error clear flag</description>
          <name>PECF</name>
        </field>
      </fields>
      <name>ICR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x24</addressOffset>
      <description>Receive data register</description>
      <displayName>RDR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>9</bitWidth>
          <description>Receive data value</description>
          <name>RDR</name>
        </field>
      </fields>
      <name>RDR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28</addressOffset>
      <description>Transmit data register</description>
      <displayName>TDR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>9</bitWidth>
          <description>Transmit data value</description>
          <name>TDR</name>
        </field>
      </fields>
      <name>TDR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
