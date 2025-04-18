/*!
A dedicated 16-bit Increment/Decrement Unit (IDU) is capable of performing only simple increment/decrement
operations on the 16-bit address bus value, but they can be performed independently of the ALU, improving
maximum performance of the CPU core. The IDU always outputs its result back to the register file, where it can
be written to a register pair or a 16-bit register.
https://gekkio.fi/files/gb-docs/gbctr.pdf
*/

use crate::bus::Bus;
use crate::cpu::register_file::{Register, RegisterFile};
use std::cell::RefCell;
use std::rc::Rc;

pub struct IDU {
    address_bus: Rc<RefCell<Bus<u16>>>,
    register_file: Rc<RefCell<RegisterFile>>,
}

impl IDU {
    pub fn new(
        address_bus: Rc<RefCell<Bus<u16>>>,
        register_file: Rc<RefCell<RegisterFile>>,
    ) -> IDU {
        IDU {
            address_bus,
            register_file,
        }
    }

    pub fn write_into(&self, register: Register) {
        let address = self.address_bus.borrow().read().unwrap_or_else(|| {
            println!("WARNING: The address bus should not be empty at this point!");
            0
        });
        self.register_file.borrow_mut().write_u16(register, address)
    }

    pub fn increment_into(&self, register: Register) {
        let address = self.address_bus.borrow().read().unwrap_or_else(|| {
            println!("WARNING: The address bus should not be empty at this point!");
            0
        });
        self.register_file
            .borrow_mut()
            .write_u16(register, address + 1)
    }

    pub fn decrement_into(&self, register: Register) {
        let address = self.address_bus.borrow().read().unwrap_or_else(|| {
            println!("WARNING: The address bus should not be empty at this point!");
            0
        });
        self.register_file
            .borrow_mut()
            .write_u16(register, address - 1)
    }

    pub fn adjust_u8_into(&self, register: Register, adjustment: i32) {
        let address = self.address_bus.borrow().read().unwrap_or_else(|| {
            println!("WARNING: The address bus should not be empty at this point!");
            0
        }) as u8;

        let adjusted_address = match adjustment {
            1 => address + 1,
            -1 => address - 1,
            0 => address,
            _ => panic!("Illegal adjustment"),
        };

        self.register_file
            .borrow_mut()
            .write_u8(register, adjusted_address)
    }
}
