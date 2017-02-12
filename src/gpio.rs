pub trait Addressable {
    fn to_int(&self, offset: u32) -> u32;
    fn from_int(value: u32, offset: u32) -> self;
}

impl Addressable for bool {
    fn to_int(&self, offset: u32) -> u32 { 1 << offset }
    fn from_int(value: u32, address: u32) -> self { value & address > 0 }
}

pub enum ModerMode {
    InputMode = 0b00,
    OutputMode = 0b01,
    AlternateMode = 0b10,
    AnalogMode = 0b11,
}

impl Addressable for ModerMode {
    fn to_int(&self, offset: u32) -> u32 {
        (self as u32) << offset
    }
    fn from_int(value: u32, address: u32) -> self {
        (value & (0b11 << address)) as ModerMode
    }
}


macro_rules! implement_cached_bools {
    ($name:ident, $address:expr, $type:ident, $($field:ident = $offset:expr), *) => {
        pub struct $name {
            $(
                $field: $type,
            )*,
            __saved: bool
        }

        impl $name {
            pub fn get() -> $name {
                let value = unsafe { ptr::volatile_read($address) };
                $name {
                    $(
                        $field: $type::from_int(value, $offset),
                    )*,
                    __saved: false
                }
            }

            pub fn save(self) {
                self.__saved = true;
                let value = 0 $(
                    | self.$field.to_int($offset)
                )*;
                unsafe { ptr::volatile_write($address, value) };
            }
        } 

        impl ::core::ops::Drop for $name {
            fn drop(self) {
                if !self.__saved {
                    panic!("{} is not saved!", stringify!($name));
                }
            }
        }
    }
}

macro_rules! implement_gpio {
    ($side:ident, $offset:expr) => {
        pub mod $side {
            const ADDRESS: u32 = $offset;
            implement_cached_bools!(Moder, ADDRESS + 0x00, ModerMode, 
                moder15 = 30,
            );
        }
    }
}

implement_gpio!(a, 0x48000000);