/*!
The register file holds most of the state of the CPU inside registers.
It contains the 16-bit Program Counter (PC), the 16-bit Stack Pointer (SP),
the 8-bit Accumulator (A), the Flags register (F), general-purpose register pairs
consisting of two 8-bit halves such as BC, DE, HL, and the special-purpose 8-bit registers
Instruction Register (IR) and Interrupt Enable (IE).
https://gekkio.fi/files/gb-docs/gbctr.pdf
*/

use crate::bus::Bus;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Register {
    // u8
    IR,
    // u8
    IE,

    // u8
    A,
    // u8
    B,
    // u8
    C,
    // u8
    D,
    // u8
    E,
    // u8
    F,
    // u8
    H,
    // u8
    L,

    // u8
    Z,
    // u16
    PC,
    // u16
    SP,
}

const REGISTER_COUNT: usize = 13;
const REGISTER_FILE_BYTES: usize = 15;

const DATA_REGISTERS: [Register; 8] = [
    Register::A,
    Register::B,
    Register::C,
    Register::D,
    Register::E,
    Register::F,
    Register::H,
    Register::L,
];

impl Register {
    pub fn data_register(index: u8) -> Register {
        DATA_REGISTERS[index as usize]
    }

    pub fn index(&self) -> usize {
        *self as usize
    }
}

pub struct RegisterFile {
    data: [u8; REGISTER_FILE_BYTES],
    data_bus: Rc<RefCell<Bus<u8>>>,
}

impl Display for RegisterFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "---RegisterFile---")?;
        for register in Register::iter() {
            if register.index() < Register::PC.index() {
                writeln!(f, "{register:?}:\t{}", self.read_u8(register))?;
            } else {
                writeln!(f, "{register:?}:\t{}", self.read_u16(register))?;
            }
        }
        write!(f, "------------------")?;
        Ok(())
    }
}

impl RegisterFile {
    pub fn new(data_bus: Rc<RefCell<Bus<u8>>>) -> RegisterFile {
        RegisterFile {
            data: [0; REGISTER_FILE_BYTES],
            data_bus,
        }
    }

    pub fn read_u8(&self, register: Register) -> u8 {
        assert!(register.index() < Register::PC.index());

        self.data[register.index()]
    }

    pub fn read_u16(&self, register: Register) -> u16 {
        assert!(register.index() >= Register::PC.index());

        let i = 2 * register.index() - Register::PC.index();
        let high = self.data[i + 1] as u16;
        let low = self.data[i] as u16;

        (high << 8) | low
    }

    pub fn write_u8(&mut self, register: Register, value: u8) {
        assert!(register.index() < Register::PC.index());

        self.data[register.index()] = value;
    }

    pub fn write_u16(&mut self, register: Register, value: u16) {
        assert!(register.index() >= Register::PC.index());

        let i = 2 * register.index() - Register::PC.index();
        self.data[i + 1] = (value >> 8) as u8;
        self.data[i] = value as u8;
    }

    pub fn read_data_bus(&mut self, register: Register) {
        let data = self.data_bus.borrow().read().unwrap_or_else(|| {
            println!("WARNING: The data bus should not be empty at this point!");
            0
        });
        self.write_u8(register, data);
    }
}
