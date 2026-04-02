use crate::memory::Memory;
use std::{
    rc::Rc,
    cell::RefCell
};

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
    registers: Registers,
    memory: Rc<RefCell<Memory>>,
    stall_cycles: u8 // Used to skip T-states to replicate instructions taking multiple cpu cycles to complete
}

impl CPU {
    pub fn new(mem: Rc<RefCell<Memory>>) -> CPU { 
        let r = Registers {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0
        };

        CPU {
            registers: r,
            memory: mem,
            stall_cycles: 0
        }
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

    pub fn cycle(&mut self) {
        if self.stall_cycles > 0 {
            self.stall_cycles -= 1;
            return;
        }

        let mut fetch_next_byte = || -> u8 {
            let byte = self.memory.
                as_ref().
                borrow_mut().
                fetch_byte(self.registers.pc);

            self.registers.pc += 1;
            byte
        };

        let opcode = fetch_next_byte();

        match opcode {
            0x0 => { // NO-OP, just stalls the cpu for 4 T states
                self.stall_cycles = 3;
            }

            0x1 => { // LD BC,n16
                self.stall_cycles = 11;

                // Gameboy colour is little-endian, lower byte comes first
                let low_byte = fetch_next_byte();
                let high_byte = fetch_next_byte();
                
                let value = ((high_byte as u16) << 8) | low_byte as u16;
                self.set_bc(value);
            }

            0x2 => {
                self.stall_cycles = 7;
                
            }

            _ => { // Handles unknown opcodes
                panic!("Error: couldn't decode instruction: {opcode}");
            }

        }
    }

    fn get_af(&self) -> u16 {
        ((self.registers.a as u16) << 8) | (self.registers.f as u16)
    }

    fn get_bc(&self) -> u16 {
        ((self.registers.b as u16) << 8) | (self.registers.c as u16)
    }

    fn get_de(&self) -> u16 {
        ((self.registers.d as u16) << 8) | (self.registers.e as u16)
    }

    fn get_hl(&self) -> u16 {
        ((self.registers.h as u16) << 8) | (self.registers.l as u16)
    }

    fn set_af(&mut self, value: u16) {
        self.registers.a = (value >> 8) as u8;
        self.registers.f = (value & 0x00FF) as u8;
    }

    fn set_bc(&mut self, value: u16) {
        self.registers.b = (value >> 8) as u8;
        self.registers.c = (value & 0x00FF) as u8;
    }

    fn set_de(&mut self, value: u16) {
        self.registers.d = (value >> 8) as u8;
        self.registers.e = (value & 0x00FF) as u8;
    }

    fn set_hl(&mut self, value: u16) {
        self.registers.h = (value >> 8) as u8;
        self.registers.l = (value & 0x00FF) as u8;
    }

    fn get_zero_bit(&self) -> u8 {
        (self.registers.f & FlagBitMasks::ZERO) >> 7
    }

    fn get_subtract_bit(&self) -> u8 {
        (self.registers.f & FlagBitMasks::SUBTRACT) >> 6
    }

    fn get_half_carry_bit(&self) -> u8 {
        (self.registers.f & FlagBitMasks::HALF_CARRY) >> 5
    }

    fn get_carry_bit(&self) -> u8 {
        (self.registers.f & FlagBitMasks::CARRY) >> 4
    }

    fn set_zero_bit(&mut self, bit: u8) {
        if bit == 1 {
            self.registers.f |= FlagBitMasks::ZERO;
        } else {
            self.registers.f &= !FlagBitMasks::ZERO;
        }
    }

    fn set_subtract_bit(&mut self, bit: u8) {
        if bit == 1 {
            self.registers.f |= FlagBitMasks::SUBTRACT;
        } else {
            self.registers.f &= !FlagBitMasks::SUBTRACT;
        }
    }

    fn set_half_carry_bit(&mut self, bit: u8) {
        if bit == 1 {
            self.registers.f |= FlagBitMasks::HALF_CARRY;
        } else {
            self.registers.f &= !FlagBitMasks::HALF_CARRY;
        }
    }
    
    fn set_carry_bit(&mut self, bit: u8) {
        if bit == 1 {
            self.registers.f |= FlagBitMasks::CARRY;
        } else {
            self.registers.f &= !FlagBitMasks::CARRY;
        }
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

    #[test]
    fn set_get_registers() {
        let mut cpu = CPU::new();

        cpu.set_af(0xAAFF);
        cpu.set_bc(0xBBCC);
        cpu.set_de(0xDDEE);
        cpu.set_hl(0x1122);

        assert_eq!(cpu.get_af(), 0xAAFF);
        assert_eq!(cpu.get_bc(), 0xBBCC);
        assert_eq!(cpu.get_de(), 0xDDEE);
        assert_eq!(cpu.get_hl(), 0x1122);
    }

    #[test]
    fn set_get_flags() {
        let mut cpu = CPU::new();

        cpu.registers.f = 0;

        cpu.set_zero_bit(1);
        assert_eq!(cpu.get_zero_bit(), 1);
        assert_eq!(cpu.get_subtract_bit(), 0);
        assert_eq!(cpu.get_half_carry_bit(), 0);
        assert_eq!(cpu.get_carry_bit(), 0);

        cpu.registers.f = 0;

        cpu.set_subtract_bit(1);
        assert_eq!(cpu.get_zero_bit(), 0);
        assert_eq!(cpu.get_subtract_bit(), 1);
        assert_eq!(cpu.get_half_carry_bit(), 0);
        assert_eq!(cpu.get_carry_bit(), 0);

        cpu.registers.f = 0;

        cpu.set_half_carry_bit(1);
        assert_eq!(cpu.get_zero_bit(), 0);
        assert_eq!(cpu.get_subtract_bit(), 0);
        assert_eq!(cpu.get_half_carry_bit(), 1);
        assert_eq!(cpu.get_carry_bit(), 0);

        cpu.registers.f = 0;

        cpu.set_carry_bit(1);
        assert_eq!(cpu.get_zero_bit(), 0);
        assert_eq!(cpu.get_subtract_bit(), 0);
        assert_eq!(cpu.get_half_carry_bit(), 0);
        assert_eq!(cpu.get_carry_bit(), 1);
    }
}

