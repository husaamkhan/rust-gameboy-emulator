use crate::memory::Memory;
use std::{
    rc::Rc,
    cell::RefCell
};

const HALF_CARRY_THRESHOLD_U8: u8 =     16;
const HALF_CARRY_THRESHOLD_U16: u16 =   256;

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
#[allow(dead_code)] // TODO: remove this later
mod flag_bit_masks {
    pub const ZERO: u8 = 0x80;
    pub const SUBTRACT: u8 = 0x40;
    pub const HALF_CARRY: u8 = 0x20;
    pub const CARRY: u8 = 0x10;
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
                fetch_byte_from_rom(self.registers.pc);

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

            0x2 => { // LD [BC], A
                self.stall_cycles = 7;
                self.memory.borrow_mut().write(self.get_bc(), self.registers.a);
            }

            0x3 => { // INC BC
                self.stall_cycles = 1;
                self.set_bc(self.get_bc()+1);
            }

            0x4 => { // INC B
                self.registers.b = self.add_u8(self.registers.b, 1);
            }

            0x5 => { // DEC B
                self.registers.b = self.sub_u8(self.registers.b, 1);
            }

            0x6 => { // LD B,n8
                self.stall_cycles = 1;
                self.registers.b = fetch_next_byte();
            }

            0x7 => { // RLCA                
                // Carry flag is set if the 7th bit is 1, as rotating A in that case would cause a
                // carry
                self.set_carry_bit(self.registers.a >> 7);
                self.registers.a = self.registers.a << 1;
            }

            0x8 => { // LD [a16],SP
                self.stall_cycles = 4;

                let low_byte = fetch_next_byte();
                let high_byte = fetch_next_byte();
                let addr = ((high_byte as u16) << 8) | low_byte as u16;

                // truncate u16 value to u8 to get low_byte
                let sp_low_byte = self.registers.sp as u8; 
                let sp_high_byte = (self.registers.sp >> 8) as u8;

                self.memory.borrow_mut().write(addr, sp_low_byte);
                self.memory.borrow_mut().write(addr+1, sp_high_byte);
            }

            0x9 => { // ADD HL,BC
                
            }

            _ => { // Handles unknown opcodes
                panic!("Error: couldn't decode instruction: {opcode}");
            }

        }
    }

    /**
     * Following add_u8 and sub_u8 functions are helpers for 8-bit arithmetic
     * Set the necessary flags, and return the result
     * NOTE: Do NOT set the carry flag
     *
     * To be used for various CPU instructions.
     */
    fn add_u8(&mut self, a: u8, b: u8) -> u8 {
        let result = a.wrapping_add(b);
        self.set_zero_bit(if result == 0 { 1 } else { 0 });
        self.set_subtract_bit(0);
        self.set_half_carry_bit(if CPU::check_for_half_carry_u8(a, b) { 1 } else { 0 });
        result
    }

    fn sub_u8(&mut self, a: u8, b: u8) -> u8 {
        let result = a.wrapping_sub(b);
        self.set_zero_bit(if result == 0 { 1 } else { 0 });
        self.set_subtract_bit(1);
        self.set_half_carry_bit(if CPU::check_for_borrow_u8(a, b) { 1 } else { 0 });
        result
    }

    /** 
     * Used to check if a half carry occured during 8-bit addition.
     * Checks if a 1 is carried from bit 3 to bit 4.
     */
    fn check_for_half_carry_u8(operand1: u8, operand2: u8) -> bool {
        if (((operand1 & 0xF) + (operand2 & 0xF)) & 0x10) == 0x10 {
            return true;
        }

        false
    }

    /**
     * Used to check if a borrow occured during 8-bit subtraction.
     * Checks if the lower nibble of the calculated difference will be less than zero.
     */
    fn check_for_borrow_u8(operand1: u8, operand2: u8) -> bool {
        if (operand1 & 0xF) < (operand2 & 0xF) {
            return true;
        }

        false
    }

    /** 
     * Used to check if a half carry occured during 16-bit addition.
     * Checks if a 1 is carried from bit 11 to 12
     */
    fn check_for_half_carry_u16(operand1: u16, operand2: u16) -> bool {
        if ((operand1 & 0xFFF) + (operand2 & 0xFFF)) as u32 & 0x1000 == 0x1000 {
            return true;
        }

        false
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
        (self.registers.f & flag_bit_masks::ZERO) >> 7
    }

    fn get_subtract_bit(&self) -> u8 {
        (self.registers.f & flag_bit_masks::SUBTRACT) >> 6
    }

    fn get_half_carry_bit(&self) -> u8 {
        (self.registers.f & flag_bit_masks::HALF_CARRY) >> 5
    }

    fn get_carry_bit(&self) -> u8 {
        (self.registers.f & flag_bit_masks::CARRY) >> 4
    }

    fn set_zero_bit(&mut self, bit: u8) {
        if bit == 1 {
            self.registers.f |= flag_bit_masks::ZERO;
        } else {
            self.registers.f &= !flag_bit_masks::ZERO;
        }
    }

    fn set_subtract_bit(&mut self, bit: u8) {
        if bit == 1 {
            self.registers.f |= flag_bit_masks::SUBTRACT;
        } else {
            self.registers.f &= !flag_bit_masks::SUBTRACT;
        }
    }

    fn set_half_carry_bit(&mut self, bit: u8) {
        if bit == 1 {
            self.registers.f |= flag_bit_masks::HALF_CARRY;
        } else {
            self.registers.f &= !flag_bit_masks::HALF_CARRY;
        }
    }
    
    fn set_carry_bit(&mut self, bit: u8) {
        if bit == 1 {
            self.registers.f |= flag_bit_masks::CARRY;
        } else {
            self.registers.f &= !flag_bit_masks::CARRY;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- CPU INITIALIZATION TESTS ----
    #[test]
    fn initialize_registers_cgb() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(Memory::new())));
        cpu.initialize_registers(false);

        let reg = Registers {
            a: 0x11, f: 0x80, b: 0x00, c: 0x00, d: 0xFF, e: 0x56, h: 0x00, l: 0x0D, pc: 0x0100, sp: 0xFFFE
        };

        assert_eq!(cpu.registers, reg);
    }

    #[test]
    fn initialize_registers_dmg() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(Memory::new())));
        cpu.initialize_registers(true);

        let reg = Registers {
            a: 0x11, f: 0x80, b: 0x00, c: 0x00, d: 0x00, e: 0x08, h: 0x00, l: 0x7C, pc: 0x0100, sp: 0xFFFE
        };

        assert_eq!(cpu.registers, reg);
    }

    // ---- REGISTERS AND FLAGS TESTS ----
    #[test]
    fn set_get_registers() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(Memory::new())));

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
        let mut cpu = CPU::new(Rc::new(RefCell::new(Memory::new())));

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

    // ---- BINARY ARITHMETIC CARRY AND BORROW TESTS ----
    // *** 8-bit arithmetic ***
    // Half-carry tests 
    // Case 1: Half carry occurs
    #[test]
    fn half_carry_u8_carry_occurs() {
        let mut operand1 = 0b1111; // 15
        let mut operand2 = 0b0100; // 8
        assert!(CPU::check_for_half_carry_u8(operand1, operand2));
    }
    
    // Case 2: Half carry does not occur
    #[test]
    fn half_carry_u8_no_carry() {
        let operand1 = 0b00001010; // 10
        let operand2 = 0b00000101; // 5
        assert!(!CPU::check_for_half_carry_u8(operand1, operand2));
    }

    // Borrow tests
    // Case 1: Borrow occurs
    #[test]
    fn borrow_u8_borrow_occurs() {
        // Case 1: Borrow occurs
        let operand1 = 0b0001; // 1
        let operand2 = 0b1111; // 15
        assert!(CPU::check_for_borrow_u8(operand1, operand2));
    }

    // Case 2: Borrow does not occur
    #[test]
    fn borrow_u8_no_borrow() {
        let operand1 = 0b1111; // 15
        let operand2 = 0b0001; // 1
        assert!(!CPU::check_for_borrow_u8(operand1, operand2));
    }

    // ** 16-bit arithmetic **
    // Case 1: Half carry occurs
    #[test]
    fn check_for_half_carry_u16() {
        let operand1 = 0b111111111111;  // 4095
        let operand2 = 0b100000000000;  // 2048
        assert!(CPU::check_for_half_carry_u16(operand1, operand2));
    }

    // Case 2: Half carry does not occur
    #[test]
    fn half_carry_u16() {
        let operand1 = 0b1000000000000000; // 4095
        let operand2 = 0b0100000000000000; // 2047
        assert!(!CPU::check_for_half_carry_u16(operand1, operand2));
    }
}

