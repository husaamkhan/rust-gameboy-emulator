struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16
}

/**
 * Contains bit masks that will be applied to register f (flag register) to get the values of various
 * flag bits.
 */
struct FlagBitMasks;
impl FlagBitMasks {
    const ZERO: u8 = 0x80;
    const SUBTRACT: u8 = 0x40;
    const HALF_CARRY: u8 = 0x20;
    const CARRY: u8 = 0x10;
}

pub struct CPU {
    registers: Registers
}

impl CPU {
    pub fn new() -> CPU { 
        let r = Registers { a: 0, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0 };
        CPU { registers: r }
    }

    fn initialize_registers(mut self, dmg_mode: bool, reg_b_value: u8) {
        self.registers.a = 0x11;
        self.registers.f = 0x80; // ZERO=1, SUBTRACT=0, HALF_CARRY=0, CARRY=0
        self.registers.b = reg_b_value;
        self.registers.c = 0x00;
        self.registers.d = 0xff;
        self.registers.e = 0x56;
        self.registers.h = 0x00;
        self.registers.l = 0x0D;
        self.registers.pc = 0x0100;
        self.registers.sp = 0xFFFE;

        if dmg_mode {
            self.registers.d = 0x00;
            self.registers.e = 0x08;

            if self.registers.b == 0x43 || self.registers.b == 0x58 {
                self.registers.h = 0x99;
                self.registers.l = 0x1A;
            } else {
                self.registers.h = 0x00;
                self.registers.l = 0x7C;
            }
        }
    }
}

