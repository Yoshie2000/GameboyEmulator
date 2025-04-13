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
}
