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

    /**
     * Initializes CPU registers based on whether the mode is set to CGB or DMG, and sets the B
     * register value to the provided number
     */
    pub fn initialize_registers(&mut self, dmg_mode: bool, reg_b_value: u8) {

        /*
         * TODO: Instead of passing in reg_b_value, I should calculate the b register's initial
         * value in this function. I can do this by instead passing in the old licensee and new
         * licensee codes and calculating their sum in this function.
         */

        self.registers.a = 0x11;
        self.registers.f = 0x80; // ZERO=1, SUBTRACT=0, HALF_CARRY=0, CARRY=0
        self.registers.b = reg_b_value;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_registers_cgb() {
        let mut cpu = CPU::new();
        cpu.initialize_registers(false, 0x00);

        let reg = Registers {
            a: 0x11, f: 0x80, b: 0x00, c: 0x00, d: 0xFF, e: 0x56, h: 0x00, l: 0x0D, pc: 0x0100, sp: 0xFFFE
        };

        assert_eq!(cpu.registers, reg);
    }

    #[test]
    fn initialize_registers_dmg_case_one() {
        let mut cpu = CPU::new();
        cpu.initialize_registers(true, 0x43);

        let reg = Registers {
            a: 0x11, f: 0x80, b: 0x43, c: 0x00, d: 0x00, e: 0x08, h: 0x99, l: 0x1A, pc: 0x0100, sp: 0xFFFE
        };

        assert_eq!(cpu.registers, reg);
    }


    #[test]
    fn initialize_registers_dmg_case_two() {
        let mut cpu = CPU::new();
        cpu.initialize_registers(true, 0x58);

        let reg = Registers {
            a: 0x11, f: 0x80, b: 0x58, c: 0x00, d: 0x00, e: 0x08, h: 0x99, l: 0x1A, pc: 0x0100, sp: 0xFFFE
        };

        assert_eq!(cpu.registers, reg);
    }

    #[test]
    fn initialize_registers_dmg_case_three() {
        let mut cpu = CPU::new();
        cpu.initialize_registers(true, 0x11);

        let reg = Registers {
            a: 0x11, f: 0x80, b: 0x11, c: 0x00, d: 0x00, e: 0x08, h: 0x00, l: 0x7C, pc: 0x0100, sp: 0xFFFE
        };

        assert_eq!(cpu.registers, reg);
    }
}

