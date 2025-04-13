use crate::bus::Bus;
use crate::cpu::alu::ALU;
use crate::cpu::control_unit::ControlUnit;
use crate::cpu::idu::IDU;
use crate::cpu::instruction::Instruction;
use crate::cpu::register_file::{Register, RegisterFile};
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct CPU {
    control_unit: ControlUnit,
    data_bus: Rc<RefCell<Bus<u8>>>,
    register_file: Rc<RefCell<RegisterFile>>,
    address_bus: Rc<RefCell<Bus<u16>>>,
    alu: ALU,
    idu: IDU,

    current_instruction: Option<Instruction>,
    instruction_counter: u8,
}

impl CPU {
    pub fn new() -> CPU {
        let data_bus = Rc::new(RefCell::new(Bus::<u8>::new()));
        let address_bus = Rc::new(RefCell::new(Bus::<u16>::new()));
        let register_file = Rc::new(RefCell::new(RegisterFile::new(Rc::clone(&data_bus))));

        CPU {
            control_unit: ControlUnit::new(),
            data_bus: Rc::clone(&data_bus),
            register_file: Rc::clone(&register_file),
            address_bus: Rc::clone(&address_bus),
            alu: ALU::new(Rc::clone(&data_bus), Rc::clone(&register_file)),
            idu: IDU::new(Rc::clone(&address_bus)),
            current_instruction: None,
            instruction_counter: 0,
        }
    }

    pub fn data_bus_mut(&mut self) -> RefMut<'_, Bus<u8>> {
        self.data_bus.borrow_mut()
    }

    pub fn register_file(&self) -> Ref<'_, RegisterFile> {
        self.register_file.borrow()
    }

    pub fn clock_cycle(&mut self) {
        // Decode the next instruction if we're not in the middle of one
        if self.current_instruction.is_none() {
            self.current_instruction = Some(self.decode());
        }

        // Execute the current instruction
        match self.current_instruction {
            Some(current_instruction) => self.execute(current_instruction),
            None => panic!("This shouldbe impossible to happen"),
        }

        // If the instruction is fully executed, prepare the next one
        if self.current_instruction.is_none() {
            self.instruction_counter = 0;

            // Read the next instruction from the data bus
            self.register_file.borrow_mut().read_data_bus(Register::IR);
        }

        // Always increment the PC via the IDU
        self.address_bus
            .borrow_mut()
            .write(self.register_file.borrow().read_u16(Register::PC));
        let next_pc = self.idu.increment();
        self.register_file
            .borrow_mut()
            .write_u16(Register::PC, next_pc);
    }

    pub fn decode(&mut self) -> Instruction {
        let instruction = self.register_file.borrow().read_u8(Register::IR);
        let instruction_header = instruction >> 6;
        let instruction_body_1 = (instruction >> 3) & 0x7;
        let instruction_body_2 = instruction & 0x7;

        match instruction_header {
            0b01 => {
                if instruction_body_1 < 8 && instruction_body_2 < 8 {
                    Instruction::LD(
                        Register::data_register(instruction_body_1),
                        Register::data_register(instruction_body_2),
                    )
                } else {
                    Instruction::NOP()
                }
            }
            0b00 => {
                if instruction_body_2 == 6 {
                    assert!(instruction_body_1 < 8);
                    Instruction::LDI(Register::data_register(instruction_body_1))
                } else {
                    Instruction::NOP()
                }
            }
            _ => {
                panic!("Unimplemented or invalid instruction");
            }
        }
    }

    pub fn execute(&mut self, current_instruction: Instruction) {
        println!("Executing {current_instruction:?}");

        match current_instruction {
            Instruction::LD(r, r_) => {
                self.alu.read_data_register(r);
                self.alu.write_data_register(r_);
                self.current_instruction = None;
            }

            Instruction::LDI(r) => match self.instruction_counter {
                0 => {
                    self.register_file.borrow_mut().read_data_bus(Register::Z);
                    self.instruction_counter += 1;
                }
                1 => {
                    self.alu.read_data_register(Register::Z);
                    self.alu.write_data_register(r);
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction LDI");
                }
            },

            Instruction::NOP() => {
                self.current_instruction = None;
            }
        }
    }
}
