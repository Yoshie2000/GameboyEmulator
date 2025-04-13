/*!
A dedicated 16-bit Increment/Decrement Unit (IDU) is capable of performing only simple increment/decrement
operations on the 16-bit address bus value, but they can be performed independently of the ALU, improving
maximum performance of the CPU core. The IDU always outputs its result back to the register file, where it can
be written to a register pair or a 16-bit register.
https://gekkio.fi/files/gb-docs/gbctr.pdf
*/

use crate::bus::Bus;
use std::cell::RefCell;
use std::rc::Rc;

pub struct IDU {
    address_bus: Rc<RefCell<Bus<u16>>>,
}

impl IDU {
    pub fn new(address_bus: Rc<RefCell<Bus<u16>>>) -> IDU {
        IDU { address_bus }
    }

    pub fn increment(&self) -> u16 {
        let address = self.address_bus.borrow().read().unwrap_or_else(|| {
            println!("WARNING: The address bus should not be empty at this point!");
            0
        });
        address + 1
    }

    pub fn decrement(&self) -> u16 {
        let address = self.address_bus.borrow().read().unwrap_or_else(|| {
            println!("WARNING: The address bus should not be empty at this point!");
            0
        });
        address - 1
    }
}
