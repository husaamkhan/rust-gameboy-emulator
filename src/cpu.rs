/**
 * Contains all CPU registers.
 *
 * | 16-bit | Hi | Lo |   Name/Function   |
 * |--------|----|----|-------------------|
 * |   AF   | A  | -- |Accumulator & Flags|
 * |   BC   | B  | C  |        BC         |
 * |   DE   | D  | E  |        DE         |
 * |   HL   | H  | L  |        HL         |
 * |   SP   | -- | -- |   Stack Pointer   |
 * |   PC   | -- | -- |  Program Counter  |
 * From Gameboy Pandocs (https://gbdev.io/pandocs/CPU_Registers_and_Flags.html)
 */
#[derive(Debug, PartialEq)]
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
 * Contains bit masks that will be applied to CPU register f (flag register) to get the values of
 * various flag bits.
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

    pub fn initialize_registers(&mut self, dmg_mode: bool) {
        self.registers.a = 0x11;
        self.registers.f = 0x80; // ZERO=1, SUBTRACT=0, HALF_CARRY=0, CARRY=0
        self.registers.b = 0x00;
        self.registers.c = 0x00;
        self.registers.d = 0xFF;
        self.registers.e = 0x56;
        self.registers.h = 0x00;
        self.registers.l = 0x0D;
        self.registers.pc = 0x0100;
        self.registers.sp = 0xFFFE;

        if dmg_mode {
            self.registers.d = 0x00;
            self.registers.e = 0x08;
            self.registers.h = 0x00;
            self.registers.l = 0x7c;
        }
    }

    fn get_af(self) -> u16 {
        ((self.registers.a as u16) << 8) | (self.registers.a as u16)
    }

    fn get_bc(self) -> u16 {
        ((self.registers.b as u16) << 8) | (self.registers.c as u16)
    }

    fn get_de(self) -> u16 {
        ((self.registers.d as u16) << 8) | (self.registers.e as u16)
    }

    fn get_hl(self) -> u16 {
        ((self.registers.h as u16) << 8) | (self.registers.l as u16)
    }

    fn set_af(&mut self, value: u16) {
        self.registers.a = (value >> 8) as u8;
        self.registers.f = (value | 0x0F) as u8;
    }

    fn set_bc(&mut self, value: u16) {
        self.registers.b = (value >> 8) as u8;
        self.registers.c = (value | 0x0F) as u8;
    }

    fn set_de(&mut self, value: u16) {
        self.registers.d = (value >> 8) as u8;
        self.registers.e = (value | 0x0F) as u8;
    }

    fn set_hl(&mut self, value: u16) {
        self.registers.h = (value >> 8) as u8;
        self.registers.l = (value | 0x0F) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_registers_cgb() {
        let mut cpu = CPU::new();
        cpu.initialize_registers(false);

        let reg = Registers {
            a: 0x11, f: 0x80, b: 0x00, c: 0x00, d: 0xFF, e: 0x56, h: 0x00, l: 0x0D, pc: 0x0100, sp: 0xFFFE
        };

        assert_eq!(cpu.registers, reg);
    }

    #[test]
    fn initialize_registers_dmg() {
        let mut cpu = CPU::new();
        cpu.initialize_registers(true);

        let reg = Registers {
            a: 0x11, f: 0x80, b: 0x00, c: 0x00, d: 0x00, e: 0x08, h: 0x00, l: 0x7C, pc: 0x0100, sp: 0xFFFE
        };

        assert_eq!(cpu.registers, reg);
    }
}

