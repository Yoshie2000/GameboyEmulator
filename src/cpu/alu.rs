/*!
An 8-bit Arithmetic Logic Unit (ALU) has two 8-bit input ports and is capable of
performing various calculations. The ALU outputs its result either to the register
file or the CPU data bus.
https://gekkio.fi/files/gb-docs/gbctr.pdf
*/

use crate::bus::Bus;
use crate::cpu::register_file::{Register, RegisterFile};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ALU {
    data_bus: Rc<RefCell<Bus<u8>>>,
    register_file: Rc<RefCell<RegisterFile>>,
    buffer: u8,
}

impl ALU {
    pub fn new(data_bus: Rc<RefCell<Bus<u8>>>, register_file: Rc<RefCell<RegisterFile>>) -> ALU {
        ALU {
            data_bus,
            register_file,
            buffer: 0,
        }
    }

    pub fn clock_cycle(&mut self) {}

    pub fn read_data_register(&mut self, register: Register) {
        self.buffer = self.register_file.borrow().read_u8(register);
    }

    pub fn read_register_pair_low(&mut self, register: Register) {
        self.buffer = self.register_file.borrow().read_u16_low(register);
    }

    pub fn read_register_pair_high(&mut self, register: Register) {
        self.buffer = self.register_file.borrow().read_u16_high(register);
    }

    pub fn write_data_register(&mut self, register: Register) {
        self.register_file
            .borrow_mut()
            .write_u8(register, self.buffer);
    }

    pub fn write_register_pair_low(&mut self, register: Register) {
        self.register_file
            .borrow_mut()
            .write_u16_low(register, self.buffer);
    }

    pub fn write_register_pair_high(&mut self, register: Register) {
        self.register_file
            .borrow_mut()
            .write_u16_high(register, self.buffer);
    }

    pub fn write_data_bus(&self) {
        self.data_bus.borrow_mut().write(self.buffer);
    }

    pub fn addi_register_16_low(&mut self, register: Register) {
        let (result, bitwise_carry) = self.add_with_bitwise_carry(
            self.buffer,
            self.register_file.borrow().read_u16_low(register),
            false,
        );
        self.buffer = result;

        let mut flags = self.register_file.borrow().flags();
        flags.set_z(false);
        flags.set_n(false);
        flags.set_h(((bitwise_carry >> 3) & 0x1) == 0x1);
        flags.set_c(((bitwise_carry >> 7) & 0x1) == 0x1);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn addi_register_16_high(&mut self, register: Register) {
        let adj = if (self.buffer >> 7) == 0 { 0x00 } else { 0xFF };
        let flags = self.register_file.borrow().flags();
        self.buffer = self
            .register_file
            .borrow()
            .read_u16_high(register)
            .overflowing_add(flags.get_c() as u8)
            .0
            .overflowing_add(adj)
            .0;
    }

    pub fn add_register_16_low(&mut self, register: Register) {
        let (result, bitwise_carry) = self.add_with_bitwise_carry(
            self.buffer,
            self.register_file.borrow().read_u16_low(register),
            false,
        );
        self.buffer = result;

        let mut flags = self.register_file.borrow().flags();
        flags.set_n(false);
        flags.set_h(((bitwise_carry >> 3) & 0x1) == 0x1);
        flags.set_c(((bitwise_carry >> 7) & 0x1) == 0x1);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn add_register_16_high(&mut self, register: Register) {
        let (result, bitwise_carry) = self.add_with_bitwise_carry(
            self.buffer,
            self.register_file.borrow().read_u16_high(register),
            true,
        );
        self.buffer = result;

        let mut flags = self.register_file.borrow().flags();
        flags.set_n(false);
        flags.set_h(((bitwise_carry >> 3) & 0x1) == 0x1);
        flags.set_c(((bitwise_carry >> 7) & 0x1) == 0x1);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn add_register(&mut self, register: Register, consider_carry: bool) {
        let mut flags = self.register_file.borrow().flags();
        let carry = if !consider_carry {
            false
        } else {
            flags.get_c()
        };

        let (result, bitwise_carry) = self.add_with_bitwise_carry(
            self.buffer,
            self.register_file.borrow().read_u8(register),
            carry,
        );
        self.buffer = result;

        flags.set_z(result == 0);
        flags.set_n(false);
        flags.set_h(((bitwise_carry >> 3) & 0x1) == 0x1);
        flags.set_c(((bitwise_carry >> 7) & 0x1) == 0x1);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn sub_register(&mut self, register: Register, consider_carry: bool) {
        let mut flags = self.register_file.borrow().flags();
        let carry = if !consider_carry {
            false
        } else {
            flags.get_c()
        };

        let (result, bitwise_carry) = self.sub_with_bitwise_carry(
            self.buffer,
            self.register_file.borrow().read_u8(register),
            carry,
        );
        self.buffer = result;

        flags.set_z(result == 0);
        flags.set_n(true);
        flags.set_h(((bitwise_carry >> 3) & 0x1) == 0x1);
        flags.set_c(((bitwise_carry >> 7) & 0x1) == 0x1);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn increment(&mut self) {
        let (result, bitwise_carry) = self.add_with_bitwise_carry(self.buffer, 1, false);
        self.buffer = result;

        let mut flags = self.register_file.borrow().flags();
        flags.set_z(result == 0);
        flags.set_n(false);
        flags.set_h(((bitwise_carry >> 3) & 0x1) == 0x1);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn decrement(&mut self) {
        let (result, bitwise_carry) = self.sub_with_bitwise_carry(self.buffer, 1, false);
        self.buffer = result;

        let mut flags = self.register_file.borrow().flags();
        flags.set_z(result == 0);
        flags.set_n(false);
        flags.set_h(((bitwise_carry >> 3) & 0x1) == 0x1);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn and(&mut self, register: Register) {
        let result = self.buffer & self.register_file.borrow().read_u8(register);
        self.buffer = result;

        let mut flags = self.register_file.borrow().flags();
        flags.set_z(result == 0);
        flags.set_n(false);
        flags.set_h(true);
        flags.set_c(false);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn or(&mut self, register: Register) {
        let result = self.buffer | self.register_file.borrow().read_u8(register);
        self.buffer = result;

        let mut flags = self.register_file.borrow().flags();
        flags.set_z(result == 0);
        flags.set_n(false);
        flags.set_h(false);
        flags.set_c(false);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn xor(&mut self, register: Register) {
        let result = self.buffer ^ self.register_file.borrow().read_u8(register);
        self.buffer = result;

        let mut flags = self.register_file.borrow().flags();
        flags.set_z(result == 0);
        flags.set_n(false);
        flags.set_h(false);
        flags.set_c(false);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn flip_carry(&self) {
        let mut flags = self.register_file.borrow().flags();
        flags.set_n(false);
        flags.set_h(false);
        flags.set_c(!flags.get_c());
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn set_carry(&self) {
        let mut flags = self.register_file.borrow().flags();
        flags.set_n(false);
        flags.set_h(false);
        flags.set_c(true);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn decimal_adjust(&mut self) {
        let mut flags = self.register_file.borrow().flags();

        if !flags.get_n() {
            // After an addition, adjust if (half-)carry occurred or if result is out of bounds
            if flags.get_c() || self.buffer > 0x09 {
                self.buffer = self.buffer.overflowing_add(0x60).0;
                flags.set_c(true);
            }
            if flags.get_h() || (self.buffer & 0x0f) > 0x09 {
                self.buffer = self.buffer.overflowing_add(0x06).0;
            }
        } else {
            // After a subtraction, only adjust if (half-)carry occurred
            if flags.get_c() {
                self.buffer = self.buffer.overflowing_sub(0x60).0;
            }
            if flags.get_h() {
                self.buffer = self.buffer.overflowing_sub(0x06).0;
            }
        }

        flags.set_z(self.buffer == 0);
        flags.set_h(false);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn flip(&mut self) {
        self.buffer = !self.buffer;

        let mut flags = self.register_file.borrow().flags();
        flags.set_n(true);
        flags.set_h(true);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn rotate_left(&mut self, replace_with_carry: bool) {
        let mut flags = self.register_file.borrow().flags();

        let high_bit = self.buffer >> 7;
        let replacement = if replace_with_carry {
            flags.get_c() as u8
        } else {
            high_bit
        };
        self.buffer = (self.buffer << 1) & replacement;

        flags.set_z(self.buffer == 0);
        flags.set_n(false);
        flags.set_h(false);
        flags.set_c(high_bit == 0x1);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn rotate_right(&mut self, replace_with_carry: bool) {
        let mut flags = self.register_file.borrow().flags();

        let low_bit = self.buffer & 0x1;
        let replacement = if replace_with_carry {
            flags.get_c() as u8
        } else {
            low_bit
        };
        self.buffer = (self.buffer >> 1) & (replacement << 7);

        flags.set_z(self.buffer == 0);
        flags.set_n(false);
        flags.set_h(false);
        flags.set_c(low_bit == 0x1);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn shift_left(&mut self) {
        let high_bit = self.buffer >> 7;
        self.buffer = self.buffer << 1;

        let mut flags = self.register_file.borrow().flags();
        flags.set_z(self.buffer == 0);
        flags.set_n(false);
        flags.set_h(false);
        flags.set_c(high_bit == 0x1);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    pub fn shift_right(&mut self, arithmetic: bool) {
        let low_bit = self.buffer & 0x1;
        let replacement = if arithmetic { self.buffer >> 7 } else { 0 };
        self.buffer = (self.buffer >> 1) & (replacement << 7);

        let mut flags = self.register_file.borrow().flags();
        flags.set_z(self.buffer == 0);
        flags.set_n(false);
        flags.set_h(false);
        flags.set_c(low_bit == 0x1);
        self.register_file
            .borrow_mut()
            .write_u8(Register::F, flags.to_u8());
    }

    fn add_with_bitwise_carry(&self, a: u8, b: u8, c: bool) -> (u8, u8) {
        let mut result = 0u8;
        let mut carry_bits = 0u8;
        let mut carry = c as u8;

        for i in 0..8 {
            let bit_a = (a >> i) & 1;
            let bit_b = (b >> i) & 1;

            let sum = bit_a + bit_b + carry;
            let bit_result = sum & 1;
            carry = (sum >> 1) & 1;

            result |= bit_result << i;
            if carry == 1 {
                carry_bits |= 1 << i;
            }
        }

        (result, carry_bits)
    }

    fn sub_with_bitwise_carry(&self, a: u8, b: u8, c: bool) -> (u8, u8) {
        let mut result = 0u8;
        let mut carry_bits = 0u8;
        let mut carry = c as u8;

        for i in 0..8 {
            let bit_a = (a >> i) & 1;
            let bit_b = (b >> i) & 1;
            let mut diff = bit_a as i8 - bit_b as i8 - carry as i8;

            if diff < 0 {
                diff += 2;
                carry = 1;
                carry_bits |= 1 << i;
            } else {
                carry = 0;
            }

            result |= (diff as u8 & 1) << i;
        }

        (result, carry_bits)
    }
}
