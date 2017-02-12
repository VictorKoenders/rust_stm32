/// interrupt and status register
pub mod isr {
    pub struct ReadonlyCache {
        /// JQOVF
        pub jqovf: bool,
        /// AWD3
        pub awd3: bool,
        /// AWD2
        pub awd2: bool,
        /// AWD1
        pub awd1: bool,
        /// JEOS
        pub jeos: bool,
        /// JEOC
        pub jeoc: bool,
        /// OVR
        pub ovr: bool,
        /// EOS
        pub eos: bool,
        /// EOC
        pub eoc: bool,
        /// EOSMP
        pub eosmp: bool,
        /// ADRDY
        pub adrdy: bool,
    }
    pub struct Cache {
        /// JQOVF
        pub jqovf: bool,
        /// AWD3
        pub awd3: bool,
        /// AWD2
        pub awd2: bool,
        /// AWD1
        pub awd1: bool,
        /// JEOS
        pub jeos: bool,
        /// JEOC
        pub jeoc: bool,
        /// OVR
        pub ovr: bool,
        /// EOS
        pub eos: bool,
        /// EOC
        pub eoc: bool,
        /// EOSMP
        pub eosmp: bool,
        /// ADRDY
        pub adrdy: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x0u32) as *mut u32) };
        ReadonlyCache {
            jqovf: ((value >> 10) & 0b1) > 0,
            awd3: ((value >> 9) & 0b1) > 0,
            awd2: ((value >> 8) & 0b1) > 0,
            awd1: ((value >> 7) & 0b1) > 0,
            jeos: ((value >> 6) & 0b1) > 0,
            jeoc: ((value >> 5) & 0b1) > 0,
            ovr: ((value >> 4) & 0b1) > 0,
            eos: ((value >> 3) & 0b1) > 0,
            eoc: ((value >> 2) & 0b1) > 0,
            eosmp: ((value >> 1) & 0b1) > 0,
            adrdy: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x0u32) as *mut u32) };
        Cache {
            jqovf: ((value >> 10) & 0b1) > 0,
            awd3: ((value >> 9) & 0b1) > 0,
            awd2: ((value >> 8) & 0b1) > 0,
            awd1: ((value >> 7) & 0b1) > 0,
            jeos: ((value >> 6) & 0b1) > 0,
            jeoc: ((value >> 5) & 0b1) > 0,
            ovr: ((value >> 4) & 0b1) > 0,
            eos: ((value >> 3) & 0b1) > 0,
            eoc: ((value >> 2) & 0b1) > 0,
            eosmp: ((value >> 1) & 0b1) > 0,
            adrdy: ((value >> 0) & 0b1) > 0,
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
                | ((self.jqovf as u32) << 10)
                | ((self.awd3 as u32) << 9)
                | ((self.awd2 as u32) << 8)
                | ((self.awd1 as u32) << 7)
                | ((self.jeos as u32) << 6)
                | ((self.jeoc as u32) << 5)
                | ((self.ovr as u32) << 4)
                | ((self.eos as u32) << 3)
                | ((self.eoc as u32) << 2)
                | ((self.eosmp as u32) << 1)
                | ((self.adrdy as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x0u32) as *mut u32, value) };
        }
    }
}
/// interrupt enable register
pub mod ier {
    pub struct ReadonlyCache {
        /// JQOVFIE
        pub jqovfie: bool,
        /// AWD3IE
        pub awd3ie: bool,
        /// AWD2IE
        pub awd2ie: bool,
        /// AWD1IE
        pub awd1ie: bool,
        /// JEOSIE
        pub jeosie: bool,
        /// JEOCIE
        pub jeocie: bool,
        /// OVRIE
        pub ovrie: bool,
        /// EOSIE
        pub eosie: bool,
        /// EOCIE
        pub eocie: bool,
        /// EOSMPIE
        pub eosmpie: bool,
        /// ADRDYIE
        pub adrdyie: bool,
    }
    pub struct Cache {
        /// JQOVFIE
        pub jqovfie: bool,
        /// AWD3IE
        pub awd3ie: bool,
        /// AWD2IE
        pub awd2ie: bool,
        /// AWD1IE
        pub awd1ie: bool,
        /// JEOSIE
        pub jeosie: bool,
        /// JEOCIE
        pub jeocie: bool,
        /// OVRIE
        pub ovrie: bool,
        /// EOSIE
        pub eosie: bool,
        /// EOCIE
        pub eocie: bool,
        /// EOSMPIE
        pub eosmpie: bool,
        /// ADRDYIE
        pub adrdyie: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x4u32) as *mut u32) };
        ReadonlyCache {
            jqovfie: ((value >> 10) & 0b1) > 0,
            awd3ie: ((value >> 9) & 0b1) > 0,
            awd2ie: ((value >> 8) & 0b1) > 0,
            awd1ie: ((value >> 7) & 0b1) > 0,
            jeosie: ((value >> 6) & 0b1) > 0,
            jeocie: ((value >> 5) & 0b1) > 0,
            ovrie: ((value >> 4) & 0b1) > 0,
            eosie: ((value >> 3) & 0b1) > 0,
            eocie: ((value >> 2) & 0b1) > 0,
            eosmpie: ((value >> 1) & 0b1) > 0,
            adrdyie: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x4u32) as *mut u32) };
        Cache {
            jqovfie: ((value >> 10) & 0b1) > 0,
            awd3ie: ((value >> 9) & 0b1) > 0,
            awd2ie: ((value >> 8) & 0b1) > 0,
            awd1ie: ((value >> 7) & 0b1) > 0,
            jeosie: ((value >> 6) & 0b1) > 0,
            jeocie: ((value >> 5) & 0b1) > 0,
            ovrie: ((value >> 4) & 0b1) > 0,
            eosie: ((value >> 3) & 0b1) > 0,
            eocie: ((value >> 2) & 0b1) > 0,
            eosmpie: ((value >> 1) & 0b1) > 0,
            adrdyie: ((value >> 0) & 0b1) > 0,
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
                | ((self.jqovfie as u32) << 10)
                | ((self.awd3ie as u32) << 9)
                | ((self.awd2ie as u32) << 8)
                | ((self.awd1ie as u32) << 7)
                | ((self.jeosie as u32) << 6)
                | ((self.jeocie as u32) << 5)
                | ((self.ovrie as u32) << 4)
                | ((self.eosie as u32) << 3)
                | ((self.eocie as u32) << 2)
                | ((self.eosmpie as u32) << 1)
                | ((self.adrdyie as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x4u32) as *mut u32, value) };
        }
    }
}
/// control register
pub mod cr {
    pub struct ReadonlyCache {
        /// ADCAL
        pub adcal: bool,
        /// ADCALDIF
        pub adcaldif: bool,
        /// DEEPPWD
        pub deeppwd: bool,
        /// ADVREGEN
        pub advregen: bool,
        /// JADSTP
        pub jadstp: bool,
        /// ADSTP
        pub adstp: bool,
        /// JADSTART
        pub jadstart: bool,
        /// ADSTART
        pub adstart: bool,
        /// ADDIS
        pub addis: bool,
        /// ADEN
        pub aden: bool,
    }
    pub struct Cache {
        /// ADCAL
        pub adcal: bool,
        /// ADCALDIF
        pub adcaldif: bool,
        /// DEEPPWD
        pub deeppwd: bool,
        /// ADVREGEN
        pub advregen: bool,
        /// JADSTP
        pub jadstp: bool,
        /// ADSTP
        pub adstp: bool,
        /// JADSTART
        pub jadstart: bool,
        /// ADSTART
        pub adstart: bool,
        /// ADDIS
        pub addis: bool,
        /// ADEN
        pub aden: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x8u32) as *mut u32) };
        ReadonlyCache {
            adcal: ((value >> 31) & 0b1) > 0,
            adcaldif: ((value >> 30) & 0b1) > 0,
            deeppwd: ((value >> 29) & 0b1) > 0,
            advregen: ((value >> 28) & 0b1) > 0,
            jadstp: ((value >> 5) & 0b1) > 0,
            adstp: ((value >> 4) & 0b1) > 0,
            jadstart: ((value >> 3) & 0b1) > 0,
            adstart: ((value >> 2) & 0b1) > 0,
            addis: ((value >> 1) & 0b1) > 0,
            aden: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x8u32) as *mut u32) };
        Cache {
            adcal: ((value >> 31) & 0b1) > 0,
            adcaldif: ((value >> 30) & 0b1) > 0,
            deeppwd: ((value >> 29) & 0b1) > 0,
            advregen: ((value >> 28) & 0b1) > 0,
            jadstp: ((value >> 5) & 0b1) > 0,
            adstp: ((value >> 4) & 0b1) > 0,
            jadstart: ((value >> 3) & 0b1) > 0,
            adstart: ((value >> 2) & 0b1) > 0,
            addis: ((value >> 1) & 0b1) > 0,
            aden: ((value >> 0) & 0b1) > 0,
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
                | ((self.adcal as u32) << 31)
                | ((self.adcaldif as u32) << 30)
                | ((self.deeppwd as u32) << 29)
                | ((self.advregen as u32) << 28)
                | ((self.jadstp as u32) << 5)
                | ((self.adstp as u32) << 4)
                | ((self.jadstart as u32) << 3)
                | ((self.adstart as u32) << 2)
                | ((self.addis as u32) << 1)
                | ((self.aden as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x8u32) as *mut u32, value) };
        }
    }
}
/// configuration register
pub mod cfgr {
    pub struct ReadonlyCache {
        /// AWDCH1CH
        pub awdch1ch: u8,
        /// JAUTO
        pub jauto: u8,
        /// JAWD1EN
        pub jawd1en: u8,
        /// AWD1EN
        pub awd1en: u8,
        /// AWD1SGL
        pub awd1sgl: u8,
        /// JQM
        pub jqm: u8,
        /// JDISCEN
        pub jdiscen: u8,
        /// DISCNUM
        pub discnum: u8,
        /// DISCEN
        pub discen: u8,
        /// AUTOFF
        pub autoff: u8,
        /// AUTDLY
        pub autdly: u8,
        /// CONT
        pub cont: u8,
        /// OVRMOD
        pub ovrmod: u8,
        /// EXTEN
        pub exten: u8,
        /// EXTSEL
        pub extsel: u8,
        /// ALIGN
        pub align: u8,
        /// RES
        pub res: u8,
        /// DMACFG
        pub dmacfg: u8,
        /// DMAEN
        pub dmaen: u8,
    }
    pub struct Cache {
        /// AWDCH1CH
        pub awdch1ch: u8,
        /// JAUTO
        pub jauto: u8,
        /// JAWD1EN
        pub jawd1en: u8,
        /// AWD1EN
        pub awd1en: u8,
        /// AWD1SGL
        pub awd1sgl: u8,
        /// JQM
        pub jqm: u8,
        /// JDISCEN
        pub jdiscen: u8,
        /// DISCNUM
        pub discnum: u8,
        /// DISCEN
        pub discen: u8,
        /// AUTOFF
        pub autoff: u8,
        /// AUTDLY
        pub autdly: u8,
        /// CONT
        pub cont: u8,
        /// OVRMOD
        pub ovrmod: u8,
        /// EXTEN
        pub exten: u8,
        /// EXTSEL
        pub extsel: u8,
        /// ALIGN
        pub align: u8,
        /// RES
        pub res: u8,
        /// DMACFG
        pub dmacfg: u8,
        /// DMAEN
        pub dmaen: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0xCu32) as *mut u32) };
        ReadonlyCache {
            awdch1ch: ((value >> 26) & 0b11111) as u8,
            jauto: ((value >> 25) & 0b11111) as u8,
            jawd1en: ((value >> 24) & 0b11111) as u8,
            awd1en: ((value >> 23) & 0b11111) as u8,
            awd1sgl: ((value >> 22) & 0b11111) as u8,
            jqm: ((value >> 21) & 0b11111) as u8,
            jdiscen: ((value >> 20) & 0b11111) as u8,
            discnum: ((value >> 17) & 0b11111) as u8,
            discen: ((value >> 16) & 0b11111) as u8,
            autoff: ((value >> 15) & 0b11111) as u8,
            autdly: ((value >> 14) & 0b11111) as u8,
            cont: ((value >> 13) & 0b11111) as u8,
            ovrmod: ((value >> 12) & 0b11111) as u8,
            exten: ((value >> 10) & 0b11111) as u8,
            extsel: ((value >> 6) & 0b11111) as u8,
            align: ((value >> 5) & 0b11111) as u8,
            res: ((value >> 3) & 0b11111) as u8,
            dmacfg: ((value >> 1) & 0b11111) as u8,
            dmaen: ((value >> 0) & 0b11111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0xCu32) as *mut u32) };
        Cache {
            awdch1ch: ((value >> 26) & 0b11111) as u8,
            jauto: ((value >> 25) & 0b11111) as u8,
            jawd1en: ((value >> 24) & 0b11111) as u8,
            awd1en: ((value >> 23) & 0b11111) as u8,
            awd1sgl: ((value >> 22) & 0b11111) as u8,
            jqm: ((value >> 21) & 0b11111) as u8,
            jdiscen: ((value >> 20) & 0b11111) as u8,
            discnum: ((value >> 17) & 0b11111) as u8,
            discen: ((value >> 16) & 0b11111) as u8,
            autoff: ((value >> 15) & 0b11111) as u8,
            autdly: ((value >> 14) & 0b11111) as u8,
            cont: ((value >> 13) & 0b11111) as u8,
            ovrmod: ((value >> 12) & 0b11111) as u8,
            exten: ((value >> 10) & 0b11111) as u8,
            extsel: ((value >> 6) & 0b11111) as u8,
            align: ((value >> 5) & 0b11111) as u8,
            res: ((value >> 3) & 0b11111) as u8,
            dmacfg: ((value >> 1) & 0b11111) as u8,
            dmaen: ((value >> 0) & 0b11111) as u8,
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
                | ((self.awdch1ch as u32) << 26)
                | ((self.jauto as u32) << 25)
                | ((self.jawd1en as u32) << 24)
                | ((self.awd1en as u32) << 23)
                | ((self.awd1sgl as u32) << 22)
                | ((self.jqm as u32) << 21)
                | ((self.jdiscen as u32) << 20)
                | ((self.discnum as u32) << 17)
                | ((self.discen as u32) << 16)
                | ((self.autoff as u32) << 15)
                | ((self.autdly as u32) << 14)
                | ((self.cont as u32) << 13)
                | ((self.ovrmod as u32) << 12)
                | ((self.exten as u32) << 10)
                | ((self.extsel as u32) << 6)
                | ((self.align as u32) << 5)
                | ((self.res as u32) << 3)
                | ((self.dmacfg as u32) << 1)
                | ((self.dmaen as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0xCu32) as *mut u32, value) };
        }
    }
}
/// sample time register 1
pub mod smpr1 {
    pub struct ReadonlyCache {
        /// SMP9
        pub smp9: u8,
        /// SMP8
        pub smp8: u8,
        /// SMP7
        pub smp7: u8,
        /// SMP6
        pub smp6: u8,
        /// SMP5
        pub smp5: u8,
        /// SMP4
        pub smp4: u8,
        /// SMP3
        pub smp3: u8,
        /// SMP2
        pub smp2: u8,
        /// SMP1
        pub smp1: u8,
    }
    pub struct Cache {
        /// SMP9
        pub smp9: u8,
        /// SMP8
        pub smp8: u8,
        /// SMP7
        pub smp7: u8,
        /// SMP6
        pub smp6: u8,
        /// SMP5
        pub smp5: u8,
        /// SMP4
        pub smp4: u8,
        /// SMP3
        pub smp3: u8,
        /// SMP2
        pub smp2: u8,
        /// SMP1
        pub smp1: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x14u32) as *mut u32) };
        ReadonlyCache {
            smp9: ((value >> 27) & 0b111) as u8,
            smp8: ((value >> 24) & 0b111) as u8,
            smp7: ((value >> 21) & 0b111) as u8,
            smp6: ((value >> 18) & 0b111) as u8,
            smp5: ((value >> 15) & 0b111) as u8,
            smp4: ((value >> 12) & 0b111) as u8,
            smp3: ((value >> 9) & 0b111) as u8,
            smp2: ((value >> 6) & 0b111) as u8,
            smp1: ((value >> 3) & 0b111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x14u32) as *mut u32) };
        Cache {
            smp9: ((value >> 27) & 0b111) as u8,
            smp8: ((value >> 24) & 0b111) as u8,
            smp7: ((value >> 21) & 0b111) as u8,
            smp6: ((value >> 18) & 0b111) as u8,
            smp5: ((value >> 15) & 0b111) as u8,
            smp4: ((value >> 12) & 0b111) as u8,
            smp3: ((value >> 9) & 0b111) as u8,
            smp2: ((value >> 6) & 0b111) as u8,
            smp1: ((value >> 3) & 0b111) as u8,
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
                | ((self.smp9 as u32) << 27)
                | ((self.smp8 as u32) << 24)
                | ((self.smp7 as u32) << 21)
                | ((self.smp6 as u32) << 18)
                | ((self.smp5 as u32) << 15)
                | ((self.smp4 as u32) << 12)
                | ((self.smp3 as u32) << 9)
                | ((self.smp2 as u32) << 6)
                | ((self.smp1 as u32) << 3)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x14u32) as *mut u32, value) };
        }
    }
}
/// sample time register 2
pub mod smpr2 {
    pub struct ReadonlyCache {
        /// SMP18
        pub smp18: u8,
        /// SMP17
        pub smp17: u8,
        /// SMP16
        pub smp16: u8,
        /// SMP15
        pub smp15: u8,
        /// SMP14
        pub smp14: u8,
        /// SMP13
        pub smp13: u8,
        /// SMP12
        pub smp12: u8,
        /// SMP11
        pub smp11: u8,
        /// SMP10
        pub smp10: u8,
    }
    pub struct Cache {
        /// SMP18
        pub smp18: u8,
        /// SMP17
        pub smp17: u8,
        /// SMP16
        pub smp16: u8,
        /// SMP15
        pub smp15: u8,
        /// SMP14
        pub smp14: u8,
        /// SMP13
        pub smp13: u8,
        /// SMP12
        pub smp12: u8,
        /// SMP11
        pub smp11: u8,
        /// SMP10
        pub smp10: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x18u32) as *mut u32) };
        ReadonlyCache {
            smp18: ((value >> 24) & 0b111) as u8,
            smp17: ((value >> 21) & 0b111) as u8,
            smp16: ((value >> 18) & 0b111) as u8,
            smp15: ((value >> 15) & 0b111) as u8,
            smp14: ((value >> 12) & 0b111) as u8,
            smp13: ((value >> 9) & 0b111) as u8,
            smp12: ((value >> 6) & 0b111) as u8,
            smp11: ((value >> 3) & 0b111) as u8,
            smp10: ((value >> 0) & 0b111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x18u32) as *mut u32) };
        Cache {
            smp18: ((value >> 24) & 0b111) as u8,
            smp17: ((value >> 21) & 0b111) as u8,
            smp16: ((value >> 18) & 0b111) as u8,
            smp15: ((value >> 15) & 0b111) as u8,
            smp14: ((value >> 12) & 0b111) as u8,
            smp13: ((value >> 9) & 0b111) as u8,
            smp12: ((value >> 6) & 0b111) as u8,
            smp11: ((value >> 3) & 0b111) as u8,
            smp10: ((value >> 0) & 0b111) as u8,
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
                | ((self.smp18 as u32) << 24)
                | ((self.smp17 as u32) << 21)
                | ((self.smp16 as u32) << 18)
                | ((self.smp15 as u32) << 15)
                | ((self.smp14 as u32) << 12)
                | ((self.smp13 as u32) << 9)
                | ((self.smp12 as u32) << 6)
                | ((self.smp11 as u32) << 3)
                | ((self.smp10 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x18u32) as *mut u32, value) };
        }
    }
}
/// watchdog threshold register 1
pub mod tr1 {
    pub struct ReadonlyCache {
        /// HT1
        pub ht1: u16,
        /// LT1
        pub lt1: u16,
    }
    pub struct Cache {
        /// HT1
        pub ht1: u16,
        /// LT1
        pub lt1: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x20u32) as *mut u32) };
        ReadonlyCache {
            ht1: ((value >> 16) & 0b111111111111) as u16,
            lt1: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x20u32) as *mut u32) };
        Cache {
            ht1: ((value >> 16) & 0b111111111111) as u16,
            lt1: ((value >> 0) & 0b111111111111) as u16,
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
                | ((self.ht1 as u32) << 16)
                | ((self.lt1 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x20u32) as *mut u32, value) };
        }
    }
}
/// watchdog threshold register
pub mod tr2 {
    pub struct ReadonlyCache {
        /// HT2
        pub ht2: u8,
        /// LT2
        pub lt2: u8,
    }
    pub struct Cache {
        /// HT2
        pub ht2: u8,
        /// LT2
        pub lt2: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x24u32) as *mut u32) };
        ReadonlyCache {
            ht2: ((value >> 16) & 0b11111111) as u8,
            lt2: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x24u32) as *mut u32) };
        Cache {
            ht2: ((value >> 16) & 0b11111111) as u8,
            lt2: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.ht2 as u32) << 16)
                | ((self.lt2 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x24u32) as *mut u32, value) };
        }
    }
}
/// watchdog threshold register 3
pub mod tr3 {
    pub struct ReadonlyCache {
        /// HT3
        pub ht3: u8,
        /// LT3
        pub lt3: u8,
    }
    pub struct Cache {
        /// HT3
        pub ht3: u8,
        /// LT3
        pub lt3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x28u32) as *mut u32) };
        ReadonlyCache {
            ht3: ((value >> 16) & 0b11111111) as u8,
            lt3: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x28u32) as *mut u32) };
        Cache {
            ht3: ((value >> 16) & 0b11111111) as u8,
            lt3: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.ht3 as u32) << 16)
                | ((self.lt3 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x28u32) as *mut u32, value) };
        }
    }
}
/// regular sequence register 1
pub mod sqr1 {
    pub struct ReadonlyCache {
        /// SQ4
        pub sq4: u8,
        /// SQ3
        pub sq3: u8,
        /// SQ2
        pub sq2: u8,
        /// SQ1
        pub sq1: u8,
        /// L3
        pub l3: u8,
    }
    pub struct Cache {
        /// SQ4
        pub sq4: u8,
        /// SQ3
        pub sq3: u8,
        /// SQ2
        pub sq2: u8,
        /// SQ1
        pub sq1: u8,
        /// L3
        pub l3: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x30u32) as *mut u32) };
        ReadonlyCache {
            sq4: ((value >> 24) & 0b11111) as u8,
            sq3: ((value >> 18) & 0b11111) as u8,
            sq2: ((value >> 12) & 0b11111) as u8,
            sq1: ((value >> 6) & 0b11111) as u8,
            l3: ((value >> 0) & 0b11111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x30u32) as *mut u32) };
        Cache {
            sq4: ((value >> 24) & 0b11111) as u8,
            sq3: ((value >> 18) & 0b11111) as u8,
            sq2: ((value >> 12) & 0b11111) as u8,
            sq1: ((value >> 6) & 0b11111) as u8,
            l3: ((value >> 0) & 0b11111) as u8,
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
                | ((self.sq4 as u32) << 24)
                | ((self.sq3 as u32) << 18)
                | ((self.sq2 as u32) << 12)
                | ((self.sq1 as u32) << 6)
                | ((self.l3 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x30u32) as *mut u32, value) };
        }
    }
}
/// regular sequence register 2
pub mod sqr2 {
    pub struct ReadonlyCache {
        /// SQ9
        pub sq9: u8,
        /// SQ8
        pub sq8: u8,
        /// SQ7
        pub sq7: u8,
        /// SQ6
        pub sq6: u8,
        /// SQ5
        pub sq5: u8,
    }
    pub struct Cache {
        /// SQ9
        pub sq9: u8,
        /// SQ8
        pub sq8: u8,
        /// SQ7
        pub sq7: u8,
        /// SQ6
        pub sq6: u8,
        /// SQ5
        pub sq5: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x34u32) as *mut u32) };
        ReadonlyCache {
            sq9: ((value >> 24) & 0b11111) as u8,
            sq8: ((value >> 18) & 0b11111) as u8,
            sq7: ((value >> 12) & 0b11111) as u8,
            sq6: ((value >> 6) & 0b11111) as u8,
            sq5: ((value >> 0) & 0b11111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x34u32) as *mut u32) };
        Cache {
            sq9: ((value >> 24) & 0b11111) as u8,
            sq8: ((value >> 18) & 0b11111) as u8,
            sq7: ((value >> 12) & 0b11111) as u8,
            sq6: ((value >> 6) & 0b11111) as u8,
            sq5: ((value >> 0) & 0b11111) as u8,
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
                | ((self.sq9 as u32) << 24)
                | ((self.sq8 as u32) << 18)
                | ((self.sq7 as u32) << 12)
                | ((self.sq6 as u32) << 6)
                | ((self.sq5 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x34u32) as *mut u32, value) };
        }
    }
}
/// regular sequence register 3
pub mod sqr3 {
    pub struct ReadonlyCache {
        /// SQ14
        pub sq14: u8,
        /// SQ13
        pub sq13: u8,
        /// SQ12
        pub sq12: u8,
        /// SQ11
        pub sq11: u8,
        /// SQ10
        pub sq10: u8,
    }
    pub struct Cache {
        /// SQ14
        pub sq14: u8,
        /// SQ13
        pub sq13: u8,
        /// SQ12
        pub sq12: u8,
        /// SQ11
        pub sq11: u8,
        /// SQ10
        pub sq10: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x38u32) as *mut u32) };
        ReadonlyCache {
            sq14: ((value >> 24) & 0b11111) as u8,
            sq13: ((value >> 18) & 0b11111) as u8,
            sq12: ((value >> 12) & 0b11111) as u8,
            sq11: ((value >> 6) & 0b11111) as u8,
            sq10: ((value >> 0) & 0b11111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x38u32) as *mut u32) };
        Cache {
            sq14: ((value >> 24) & 0b11111) as u8,
            sq13: ((value >> 18) & 0b11111) as u8,
            sq12: ((value >> 12) & 0b11111) as u8,
            sq11: ((value >> 6) & 0b11111) as u8,
            sq10: ((value >> 0) & 0b11111) as u8,
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
                | ((self.sq14 as u32) << 24)
                | ((self.sq13 as u32) << 18)
                | ((self.sq12 as u32) << 12)
                | ((self.sq11 as u32) << 6)
                | ((self.sq10 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x38u32) as *mut u32, value) };
        }
    }
}
/// regular sequence register 4
pub mod sqr4 {
    pub struct ReadonlyCache {
        /// SQ16
        pub sq16: u8,
        /// SQ15
        pub sq15: u8,
    }
    pub struct Cache {
        /// SQ16
        pub sq16: u8,
        /// SQ15
        pub sq15: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x3Cu32) as *mut u32) };
        ReadonlyCache {
            sq16: ((value >> 6) & 0b11111) as u8,
            sq15: ((value >> 0) & 0b11111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x3Cu32) as *mut u32) };
        Cache {
            sq16: ((value >> 6) & 0b11111) as u8,
            sq15: ((value >> 0) & 0b11111) as u8,
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
                | ((self.sq16 as u32) << 6)
                | ((self.sq15 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x3Cu32) as *mut u32, value) };
        }
    }
}
/// regular Data Register
pub mod dr {
    /// regularDATA
    /// Access: read-only, Width: 16, Offset: 0
    /// Get regularDATA
    pub fn regulardata() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x40u32) as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// injected sequence register
pub mod jsqr {
    pub struct ReadonlyCache {
        /// JSQ4
        pub jsq4: u8,
        /// JSQ3
        pub jsq3: u8,
        /// JSQ2
        pub jsq2: u8,
        /// JSQ1
        pub jsq1: u8,
        /// JEXTEN
        pub jexten: u8,
        /// JEXTSEL
        pub jextsel: u8,
        /// JL
        pub jl: u8,
    }
    pub struct Cache {
        /// JSQ4
        pub jsq4: u8,
        /// JSQ3
        pub jsq3: u8,
        /// JSQ2
        pub jsq2: u8,
        /// JSQ1
        pub jsq1: u8,
        /// JEXTEN
        pub jexten: u8,
        /// JEXTSEL
        pub jextsel: u8,
        /// JL
        pub jl: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x4Cu32) as *mut u32) };
        ReadonlyCache {
            jsq4: ((value >> 26) & 0b11111) as u8,
            jsq3: ((value >> 20) & 0b11111) as u8,
            jsq2: ((value >> 14) & 0b11111) as u8,
            jsq1: ((value >> 8) & 0b11111) as u8,
            jexten: ((value >> 6) & 0b11111) as u8,
            jextsel: ((value >> 2) & 0b11111) as u8,
            jl: ((value >> 0) & 0b11111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x4Cu32) as *mut u32) };
        Cache {
            jsq4: ((value >> 26) & 0b11111) as u8,
            jsq3: ((value >> 20) & 0b11111) as u8,
            jsq2: ((value >> 14) & 0b11111) as u8,
            jsq1: ((value >> 8) & 0b11111) as u8,
            jexten: ((value >> 6) & 0b11111) as u8,
            jextsel: ((value >> 2) & 0b11111) as u8,
            jl: ((value >> 0) & 0b11111) as u8,
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
                | ((self.jsq4 as u32) << 26)
                | ((self.jsq3 as u32) << 20)
                | ((self.jsq2 as u32) << 14)
                | ((self.jsq1 as u32) << 8)
                | ((self.jexten as u32) << 6)
                | ((self.jextsel as u32) << 2)
                | ((self.jl as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x4Cu32) as *mut u32, value) };
        }
    }
}
/// offset register 1
pub mod ofr1 {
    pub struct ReadonlyCache {
        /// OFFSET1_EN
        pub offset1_en: bool,
        /// OFFSET1_CH
        pub offset1_ch: bool,
        /// OFFSET1
        pub offset1: bool,
    }
    pub struct Cache {
        /// OFFSET1_EN
        pub offset1_en: bool,
        /// OFFSET1_CH
        pub offset1_ch: bool,
        /// OFFSET1
        pub offset1: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x60u32) as *mut u32) };
        ReadonlyCache {
            offset1_en: ((value >> 31) & 0b1) > 0,
            offset1_ch: ((value >> 26) & 0b1) > 0,
            offset1: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x60u32) as *mut u32) };
        Cache {
            offset1_en: ((value >> 31) & 0b1) > 0,
            offset1_ch: ((value >> 26) & 0b1) > 0,
            offset1: ((value >> 0) & 0b1) > 0,
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
                | ((self.offset1_en as u32) << 31)
                | ((self.offset1_ch as u32) << 26)
                | ((self.offset1 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x60u32) as *mut u32, value) };
        }
    }
}
/// offset register 2
pub mod ofr2 {
    pub struct ReadonlyCache {
        /// OFFSET2_EN
        pub offset2_en: bool,
        /// OFFSET2_CH
        pub offset2_ch: bool,
        /// OFFSET2
        pub offset2: bool,
    }
    pub struct Cache {
        /// OFFSET2_EN
        pub offset2_en: bool,
        /// OFFSET2_CH
        pub offset2_ch: bool,
        /// OFFSET2
        pub offset2: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x64u32) as *mut u32) };
        ReadonlyCache {
            offset2_en: ((value >> 31) & 0b1) > 0,
            offset2_ch: ((value >> 26) & 0b1) > 0,
            offset2: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x64u32) as *mut u32) };
        Cache {
            offset2_en: ((value >> 31) & 0b1) > 0,
            offset2_ch: ((value >> 26) & 0b1) > 0,
            offset2: ((value >> 0) & 0b1) > 0,
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
                | ((self.offset2_en as u32) << 31)
                | ((self.offset2_ch as u32) << 26)
                | ((self.offset2 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x64u32) as *mut u32, value) };
        }
    }
}
/// offset register 3
pub mod ofr3 {
    pub struct ReadonlyCache {
        /// OFFSET3_EN
        pub offset3_en: bool,
        /// OFFSET3_CH
        pub offset3_ch: bool,
        /// OFFSET3
        pub offset3: bool,
    }
    pub struct Cache {
        /// OFFSET3_EN
        pub offset3_en: bool,
        /// OFFSET3_CH
        pub offset3_ch: bool,
        /// OFFSET3
        pub offset3: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x68u32) as *mut u32) };
        ReadonlyCache {
            offset3_en: ((value >> 31) & 0b1) > 0,
            offset3_ch: ((value >> 26) & 0b1) > 0,
            offset3: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x68u32) as *mut u32) };
        Cache {
            offset3_en: ((value >> 31) & 0b1) > 0,
            offset3_ch: ((value >> 26) & 0b1) > 0,
            offset3: ((value >> 0) & 0b1) > 0,
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
                | ((self.offset3_en as u32) << 31)
                | ((self.offset3_ch as u32) << 26)
                | ((self.offset3 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x68u32) as *mut u32, value) };
        }
    }
}
/// offset register 4
pub mod ofr4 {
    pub struct ReadonlyCache {
        /// OFFSET4_EN
        pub offset4_en: bool,
        /// OFFSET4_CH
        pub offset4_ch: bool,
        /// OFFSET4
        pub offset4: bool,
    }
    pub struct Cache {
        /// OFFSET4_EN
        pub offset4_en: bool,
        /// OFFSET4_CH
        pub offset4_ch: bool,
        /// OFFSET4
        pub offset4: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x6Cu32) as *mut u32) };
        ReadonlyCache {
            offset4_en: ((value >> 31) & 0b1) > 0,
            offset4_ch: ((value >> 26) & 0b1) > 0,
            offset4: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x6Cu32) as *mut u32) };
        Cache {
            offset4_en: ((value >> 31) & 0b1) > 0,
            offset4_ch: ((value >> 26) & 0b1) > 0,
            offset4: ((value >> 0) & 0b1) > 0,
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
                | ((self.offset4_en as u32) << 31)
                | ((self.offset4_ch as u32) << 26)
                | ((self.offset4 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0x6Cu32) as *mut u32, value) };
        }
    }
}
/// injected data register 1
pub mod jdr1 {
    /// JDATA1
    /// Access: read-only, Width: 16, Offset: 0
    /// Get JDATA1
    pub fn jdata1() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x80u32) as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// injected data register 2
pub mod jdr2 {
    /// JDATA2
    /// Access: read-only, Width: 16, Offset: 0
    /// Get JDATA2
    pub fn jdata2() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x84u32) as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// injected data register 3
pub mod jdr3 {
    /// JDATA3
    /// Access: read-only, Width: 16, Offset: 0
    /// Get JDATA3
    pub fn jdata3() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x88u32) as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// injected data register 4
pub mod jdr4 {
    /// JDATA4
    /// Access: read-only, Width: 16, Offset: 0
    /// Get JDATA4
    pub fn jdata4() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0x8Cu32) as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// Analog Watchdog 2 Configuration Register
pub mod awd2cr {
    pub struct ReadonlyCache {
        /// AWD2CH
        pub awd2ch: u32,
    }
    pub struct Cache {
        /// AWD2CH
        pub awd2ch: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0xA0u32) as *mut u32) };
        ReadonlyCache {
            awd2ch: ((value >> 1) & 0b111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0xA0u32) as *mut u32) };
        Cache {
            awd2ch: ((value >> 1) & 0b111111111111111111) as u32,
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
                | ((self.awd2ch as u32) << 1)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0xA0u32) as *mut u32, value) };
        }
    }
}
/// Analog Watchdog 3 Configuration Register
pub mod awd3cr {
    pub struct ReadonlyCache {
        /// AWD3CH
        pub awd3ch: u32,
    }
    pub struct Cache {
        /// AWD3CH
        pub awd3ch: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0xA4u32) as *mut u32) };
        ReadonlyCache {
            awd3ch: ((value >> 1) & 0b111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0xA4u32) as *mut u32) };
        Cache {
            awd3ch: ((value >> 1) & 0b111111111111111111) as u32,
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
                | ((self.awd3ch as u32) << 1)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0xA4u32) as *mut u32, value) };
        }
    }
}
/// Differential Mode Selection Register 2
pub mod difsel {
    /// Set Differential mode for channels 15 to 1
    pub fn set_difsel__(index: u8, value: u16) {
        debug_assert!(index < 2, "set_difsel__ out of range");
        debug_assert!(value <= 0b111111111111111, "set_difsel__ out of range");
        let value = value as u32;
        let value = value << (1 + index * 15);
        unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0xB0u32) as *mut u32, value) };
    }
    /// Get Differential mode for channels 15 to 1
    pub fn get_difsel__(index: u8) -> u16 {
        debug_assert!(index < 2, "get_difsel__ out of range");
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0xB0u32) as *mut u32) };
        let value = value & (0b111111111111111 << (1 + index * 15));
        value as u16
    }
}
/// Calibration Factors
pub mod calfact {
    pub struct ReadonlyCache {
        /// CALFACT_D
        pub calfact_d: u8,
        /// CALFACT_S
        pub calfact_s: u8,
    }
    pub struct Cache {
        /// CALFACT_D
        pub calfact_d: u8,
        /// CALFACT_S
        pub calfact_s: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0xB4u32) as *mut u32) };
        ReadonlyCache {
            calfact_d: ((value >> 16) & 0b1111111) as u8,
            calfact_s: ((value >> 0) & 0b1111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile((0x50000100u32 + 0xB4u32) as *mut u32) };
        Cache {
            calfact_d: ((value >> 16) & 0b1111111) as u8,
            calfact_s: ((value >> 0) & 0b1111111) as u8,
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
                | ((self.calfact_d as u32) << 16)
                | ((self.calfact_s as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile((0x50000100u32 + 0xB4u32) as *mut u32, value) };
        }
    }
}
/// ADC1 and ADC2 global interrupt
pub const INTERRUPT_ADC1_2: u32 = 18;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" derivedFrom="ADC1">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x100</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x50000100</baseAddress>
  <description>Analog-to-Digital Converter</description>
  <groupName>ADC</groupName>
  <interrupt>
    <description>ADC1 and ADC2 global interrupt</description>
    <name>ADC1_2</name>
    <value>18</value>
  </interrupt>
  <name>ADC2</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>interrupt and status register</description>
      <displayName>ISR</displayName>
      <fields>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JQOVF</description>
          <name>JQOVF</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AWD3</description>
          <name>AWD3</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AWD2</description>
          <name>AWD2</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AWD1</description>
          <name>AWD1</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JEOS</description>
          <name>JEOS</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JEOC</description>
          <name>JEOC</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OVR</description>
          <name>OVR</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EOS</description>
          <name>EOS</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EOC</description>
          <name>EOC</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EOSMP</description>
          <name>EOSMP</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADRDY</description>
          <name>ADRDY</name>
        </field>
      </fields>
      <name>ISR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>interrupt enable register</description>
      <displayName>IER</displayName>
      <fields>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JQOVFIE</description>
          <name>JQOVFIE</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AWD3IE</description>
          <name>AWD3IE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AWD2IE</description>
          <name>AWD2IE</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AWD1IE</description>
          <name>AWD1IE</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JEOSIE</description>
          <name>JEOSIE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JEOCIE</description>
          <name>JEOCIE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OVRIE</description>
          <name>OVRIE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EOSIE</description>
          <name>EOSIE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EOCIE</description>
          <name>EOCIE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EOSMPIE</description>
          <name>EOSMPIE</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADRDYIE</description>
          <name>ADRDYIE</name>
        </field>
      </fields>
      <name>IER</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>control register</description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADCAL</description>
          <name>ADCAL</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADCALDIF</description>
          <name>ADCALDIF</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DEEPPWD</description>
          <name>DEEPPWD</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADVREGEN</description>
          <name>ADVREGEN</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JADSTP</description>
          <name>JADSTP</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADSTP</description>
          <name>ADSTP</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JADSTART</description>
          <name>JADSTART</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADSTART</description>
          <name>ADSTART</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADDIS</description>
          <name>ADDIS</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADEN</description>
          <name>ADEN</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>configuration register</description>
      <displayName>CFGR</displayName>
      <fields>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>5</bitWidth>
          <description>AWDCH1CH</description>
          <name>AWDCH1CH</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JAUTO</description>
          <name>JAUTO</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JAWD1EN</description>
          <name>JAWD1EN</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AWD1EN</description>
          <name>AWD1EN</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AWD1SGL</description>
          <name>AWD1SGL</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JQM</description>
          <name>JQM</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>JDISCEN</description>
          <name>JDISCEN</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>3</bitWidth>
          <description>DISCNUM</description>
          <name>DISCNUM</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DISCEN</description>
          <name>DISCEN</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AUTOFF</description>
          <name>AUTOFF</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AUTDLY</description>
          <name>AUTDLY</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CONT</description>
          <name>CONT</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OVRMOD</description>
          <name>OVRMOD</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>EXTEN</description>
          <name>EXTEN</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTSEL</description>
          <name>EXTSEL</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ALIGN</description>
          <name>ALIGN</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>2</bitWidth>
          <description>RES</description>
          <name>RES</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DMACFG</description>
          <name>DMACFG</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DMAEN</description>
          <name>DMAEN</name>
        </field>
      </fields>
      <name>CFGR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>sample time register 1</description>
      <displayName>SMPR1</displayName>
      <fields>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP9</description>
          <name>SMP9</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP8</description>
          <name>SMP8</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP7</description>
          <name>SMP7</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP6</description>
          <name>SMP6</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP5</description>
          <name>SMP5</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP4</description>
          <name>SMP4</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP3</description>
          <name>SMP3</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP2</description>
          <name>SMP2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP1</description>
          <name>SMP1</name>
        </field>
      </fields>
      <name>SMPR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>sample time register 2</description>
      <displayName>SMPR2</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP18</description>
          <name>SMP18</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP17</description>
          <name>SMP17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP16</description>
          <name>SMP16</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP15</description>
          <name>SMP15</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP14</description>
          <name>SMP14</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP13</description>
          <name>SMP13</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP12</description>
          <name>SMP12</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP11</description>
          <name>SMP11</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>3</bitWidth>
          <description>SMP10</description>
          <name>SMP10</name>
        </field>
      </fields>
      <name>SMPR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>watchdog threshold register 1</description>
      <displayName>TR1</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>12</bitWidth>
          <description>HT1</description>
          <name>HT1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>LT1</description>
          <name>LT1</name>
        </field>
      </fields>
      <name>TR1</name>
      <resetValue>0x0FFF0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x24</addressOffset>
      <description>watchdog threshold register</description>
      <displayName>TR2</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>HT2</description>
          <name>HT2</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>LT2</description>
          <name>LT2</name>
        </field>
      </fields>
      <name>TR2</name>
      <resetValue>0x0FFF0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28</addressOffset>
      <description>watchdog threshold register 3</description>
      <displayName>TR3</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>HT3</description>
          <name>HT3</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>LT3</description>
          <name>LT3</name>
        </field>
      </fields>
      <name>TR3</name>
      <resetValue>0x0FFF0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x30</addressOffset>
      <description>regular sequence register 1</description>
      <displayName>SQR1</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ4</description>
          <name>SQ4</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ3</description>
          <name>SQ3</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ2</description>
          <name>SQ2</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ1</description>
          <name>SQ1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>L3</description>
          <name>L3</name>
        </field>
      </fields>
      <name>SQR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x34</addressOffset>
      <description>regular sequence register 2</description>
      <displayName>SQR2</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ9</description>
          <name>SQ9</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ8</description>
          <name>SQ8</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ7</description>
          <name>SQ7</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ6</description>
          <name>SQ6</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ5</description>
          <name>SQ5</name>
        </field>
      </fields>
      <name>SQR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x38</addressOffset>
      <description>regular sequence register 3</description>
      <displayName>SQR3</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ14</description>
          <name>SQ14</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ13</description>
          <name>SQ13</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ12</description>
          <name>SQ12</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ11</description>
          <name>SQ11</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ10</description>
          <name>SQ10</name>
        </field>
      </fields>
      <name>SQR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x3C</addressOffset>
      <description>regular sequence register 4</description>
      <displayName>SQR4</displayName>
      <fields>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ16</description>
          <name>SQ16</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
          <description>SQ15</description>
          <name>SQ15</name>
        </field>
      </fields>
      <name>SQR4</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x40</addressOffset>
      <description>regular Data Register</description>
      <displayName>DR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>regularDATA</description>
          <name>regularDATA</name>
        </field>
      </fields>
      <name>DR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4C</addressOffset>
      <description>injected sequence register</description>
      <displayName>JSQR</displayName>
      <fields>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>5</bitWidth>
          <description>JSQ4</description>
          <name>JSQ4</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>5</bitWidth>
          <description>JSQ3</description>
          <name>JSQ3</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>5</bitWidth>
          <description>JSQ2</description>
          <name>JSQ2</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>5</bitWidth>
          <description>JSQ1</description>
          <name>JSQ1</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
          <description>JEXTEN</description>
          <name>JEXTEN</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>4</bitWidth>
          <description>JEXTSEL</description>
          <name>JEXTSEL</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>JL</description>
          <name>JL</name>
        </field>
      </fields>
      <name>JSQR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x60</addressOffset>
      <description>offset register 1</description>
      <displayName>OFR1</displayName>
      <fields>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OFFSET1_EN</description>
          <name>OFFSET1_EN</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>5</bitWidth>
          <description>OFFSET1_CH</description>
          <name>OFFSET1_CH</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>OFFSET1</description>
          <name>OFFSET1</name>
        </field>
      </fields>
      <name>OFR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x64</addressOffset>
      <description>offset register 2</description>
      <displayName>OFR2</displayName>
      <fields>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OFFSET2_EN</description>
          <name>OFFSET2_EN</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>5</bitWidth>
          <description>OFFSET2_CH</description>
          <name>OFFSET2_CH</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>OFFSET2</description>
          <name>OFFSET2</name>
        </field>
      </fields>
      <name>OFR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x68</addressOffset>
      <description>offset register 3</description>
      <displayName>OFR3</displayName>
      <fields>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OFFSET3_EN</description>
          <name>OFFSET3_EN</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>5</bitWidth>
          <description>OFFSET3_CH</description>
          <name>OFFSET3_CH</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>OFFSET3</description>
          <name>OFFSET3</name>
        </field>
      </fields>
      <name>OFR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x6C</addressOffset>
      <description>offset register 4</description>
      <displayName>OFR4</displayName>
      <fields>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>OFFSET4_EN</description>
          <name>OFFSET4_EN</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>5</bitWidth>
          <description>OFFSET4_CH</description>
          <name>OFFSET4_CH</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>OFFSET4</description>
          <name>OFFSET4</name>
        </field>
      </fields>
      <name>OFR4</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x80</addressOffset>
      <description>injected data register 1</description>
      <displayName>JDR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>JDATA1</description>
          <name>JDATA1</name>
        </field>
      </fields>
      <name>JDR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x84</addressOffset>
      <description>injected data register 2</description>
      <displayName>JDR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>JDATA2</description>
          <name>JDATA2</name>
        </field>
      </fields>
      <name>JDR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x88</addressOffset>
      <description>injected data register 3</description>
      <displayName>JDR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>JDATA3</description>
          <name>JDATA3</name>
        </field>
      </fields>
      <name>JDR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x8C</addressOffset>
      <description>injected data register 4</description>
      <displayName>JDR4</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>JDATA4</description>
          <name>JDATA4</name>
        </field>
      </fields>
      <name>JDR4</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xA0</addressOffset>
      <description>
                        Analog Watchdog 2 Configuration
                        Register
                    </description>
      <displayName>AWD2CR</displayName>
      <fields>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>18</bitWidth>
          <description>AWD2CH</description>
          <name>AWD2CH</name>
        </field>
      </fields>
      <name>AWD2CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xA4</addressOffset>
      <description>
                        Analog Watchdog 3 Configuration
                        Register
                    </description>
      <displayName>AWD3CR</displayName>
      <fields>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>18</bitWidth>
          <description>AWD3CH</description>
          <name>AWD3CH</name>
        </field>
      </fields>
      <name>AWD3CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0xB0</addressOffset>
      <description>
                        Differential Mode Selection Register
                        2
                    </description>
      <displayName>DIFSEL</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>15</bitWidth>
          <description>
                                Differential mode for channels 15 to
                                1
                            </description>
          <name>DIFSEL_1_15</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>16</bitOffset>
          <bitWidth>3</bitWidth>
          <description>
                                Differential mode for channels 18 to
                                16
                            </description>
          <name>DIFSEL_16_18</name>
        </field>
      </fields>
      <name>DIFSEL</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xB4</addressOffset>
      <description>Calibration Factors</description>
      <displayName>CALFACT</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>7</bitWidth>
          <description>CALFACT_D</description>
          <name>CALFACT_D</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>7</bitWidth>
          <description>CALFACT_S</description>
          <name>CALFACT_S</name>
        </field>
      </fields>
      <name>CALFACT</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
