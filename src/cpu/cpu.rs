use crate::bus::Bus;
use crate::cpu::alu::ALU;
use crate::cpu::control_unit::ControlUnit;
use crate::cpu::idu::IDU;
use crate::cpu::register_file::RegisterFile;

pub struct CPU {
    control_unit: ControlUnit,
    data_bus: Bus<u8>,
    register_file: RegisterFile,
    address_bus: Bus<u16>,
    alu: ALU,
    idu: IDU,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            control_unit: ControlUnit::new(),
            data_bus: Bus::new(),
            register_file: RegisterFile::new(),
            address_bus: Bus::new(),
            alu: ALU::new(),
            idu: IDU::new(),
        }
    }

    pub fn clock_cycle(&mut self) {
        // Read the next instruction from the data bus
        self.register_file.ir = self.data_bus.read().unwrap_or_else(|| {
            println!("WARNING: The data bus should not be empty at this point!");
            0
        });
    }

    pub fn fetch(&mut self) {
        // Write the PC so the IDU and memory can read it
        self.address_bus.write(self.register_file.pc);
    }
}
