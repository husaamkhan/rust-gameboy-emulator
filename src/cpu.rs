struct Flags {
    zero: bool,
    subtraction: bool,
    half_carry: bool,
    carry: bool
}

struct Registers {
    a: u8,
    flags: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16
}

pub struct CPU {
    registers: Registers
}

impl CPU {
    pub fn new() -> CPU {
        CPU {}
    }
}

