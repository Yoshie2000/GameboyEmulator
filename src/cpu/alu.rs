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

    pub fn write_data_register(&mut self, register: Register) {
        self.register_file
            .borrow_mut()
            .write_u8(register, self.buffer);
    }

    pub fn add_with_bitwise_carry(&self, a: u8, b: u8, c: bool) -> (u8, u8) {
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

    pub fn sub_with_bitwise_carry(&self, a: u8, b: u8, c: bool) -> (u8, u8) {
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
