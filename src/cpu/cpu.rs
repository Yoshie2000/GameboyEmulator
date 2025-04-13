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
    skip_pc_increment: bool,
}

impl CPU {
    pub fn new() -> CPU {
        let data_bus = Rc::new(RefCell::new(Bus::<u8>::new()));
        let address_bus = Rc::new(RefCell::new(Bus::<u16>::new()));
        let register_file = Rc::new(RefCell::new(RegisterFile::new(
            Rc::clone(&data_bus),
            Rc::clone(&address_bus),
        )));

        CPU {
            control_unit: ControlUnit::new(),
            data_bus: Rc::clone(&data_bus),
            register_file: Rc::clone(&register_file),
            address_bus: Rc::clone(&address_bus),
            alu: ALU::new(Rc::clone(&data_bus), Rc::clone(&register_file)),
            idu: IDU::new(Rc::clone(&address_bus)),
            current_instruction: None,
            instruction_counter: 0,
            skip_pc_increment: false,
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

        // Increment the PC via the IDU
        if !self.skip_pc_increment {
            self.address_bus
                .borrow_mut()
                .write(self.register_file.borrow().read_u16(Register::PC));
            let next_pc = self.idu.increment();
            self.register_file
                .borrow_mut()
                .write_u16(Register::PC, next_pc);
        }
        self.skip_pc_increment = false;
    }

    pub fn decode(&mut self) -> Instruction {
        let instruction = self.register_file.borrow().read_u8(Register::IR);
        let instruction_header = instruction >> 6;
        let instruction_body_1 = (instruction >> 3) & 0x7;
        let instruction_body_2 = instruction & 0x7;

        match instruction_header {
            0b01 => {
                if instruction_body_1 < 6 && instruction_body_2 < 6 {
                    Instruction::LDR(
                        Register::data_register(instruction_body_1),
                        Register::data_register(instruction_body_2),
                    )
                } else if instruction_body_2 == 6 {
                    Instruction::LD(Register::data_register(instruction_body_1))
                } else if instruction_body_1 == 6 {
                    Instruction::LDM(Register::data_register(instruction_body_2))
                } else {
                    panic!("Unimplemented or invalid instruction {instruction}");
                }
            }
            0b00 => {
                if instruction_body_1 == 0 && instruction_body_2 == 2 {
                    Instruction::LDAM_BC()
                } else if instruction_body_1 == 1 && instruction_body_2 == 2 {
                    Instruction::LDA_BC()
                } else if instruction_body_1 == 2 && instruction_body_2 == 2 {
                    Instruction::LDAM_DE()
                } else if instruction_body_1 == 3 && instruction_body_2 == 2 {
                    Instruction::LDA_DE()
                } else if instruction_body_1 == 6 && instruction_body_2 == 6 {
                    Instruction::LDMI()
                } else if instruction_body_2 == 6 {
                    Instruction::LDI(Register::data_register(instruction_body_1))
                } else if instruction_body_1 == 0 && instruction_body_2 == 0 {
                    Instruction::NOP()
                } else {
                    panic!("Unimplemented or invalid instruction {instruction}");
                }
            }
            11 => {
                if instruction_body_1 == 7 && instruction_body_2 == 2 {
                    Instruction::LDAD()
                } else if instruction_body_1 == 6 && instruction_body_2 == 2 {
                    Instruction::LDH()
                } else if instruction_body_1 == 5 && instruction_body_2 == 2 {
                    Instruction::LDAMD()
                } else if instruction_body_1 == 4 && instruction_body_2 == 2 {
                    Instruction::LDHM()
                } else if instruction_body_1 == 6 && instruction_body_2 == 0 {
                    Instruction::LDHD()
                } else if instruction_body_1 == 4 && instruction_body_2 == 0 {
                    Instruction::LDHDM()
                } else {
                    panic!("Unimplemented or invalid instruction {instruction}");
                }
            }
            _ => {
                panic!("Unimplemented or invalid instruction {instruction}");
            }
        }
    }

    pub fn execute(&mut self, current_instruction: Instruction) {
        println!("Executing {current_instruction:?}");

        match current_instruction {
            Instruction::LDR(r, r_) => {
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
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LD(r) => match self.instruction_counter {
                0 => {
                    self.register_file.borrow().write_address_bus(Register::HL);
                    // TODO send read signal to the memory (needs to be synchronous)
                    self.register_file.borrow_mut().read_data_bus(Register::Z);

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                1 => {
                    self.alu.read_data_register(Register::Z);
                    self.alu.write_data_register(r);
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LDM(r) => match self.instruction_counter {
                0 => {
                    self.register_file.borrow().write_address_bus(Register::HL);
                    self.register_file.borrow().write_data_bus(r);
                    // TODO send write signal to the memory
                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                1 => {
                    // The second clock cycle is necessary since the address and data bus were busy in the previous cycle
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LDMI() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow_mut().read_data_bus(Register::Z);
                    self.instruction_counter += 1;
                }
                1 => {
                    self.register_file.borrow().write_address_bus(Register::HL);
                    self.register_file.borrow().write_data_bus(Register::Z);
                    // TODO send write signal to the memory
                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                2 => {
                    // The second clock cycle is necessary since the address and data bus were busy in the previous cycle
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LDA_BC() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow().write_address_bus(Register::BC);
                    // TODO send read signal to the memory (needs to be synchronous)
                    self.register_file.borrow_mut().read_data_bus(Register::Z);

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                1 => {
                    self.alu.read_data_register(Register::Z);
                    self.alu.write_data_register(Register::A);
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LDA_DE() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow().write_address_bus(Register::DE);
                    // TODO send read signal to the memory (needs to be synchronous)
                    self.register_file.borrow_mut().read_data_bus(Register::Z);

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                1 => {
                    self.alu.read_data_register(Register::Z);
                    self.alu.write_data_register(Register::A);
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LDAM_BC() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow().write_address_bus(Register::BC);
                    self.register_file.borrow().write_data_bus(Register::A);
                    // TODO send write signal to the memory
                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                1 => {
                    // The second clock cycle is necessary since the address and data bus were busy in the previous cycle
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LDAM_DE() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow().write_address_bus(Register::DE);
                    self.register_file.borrow().write_data_bus(Register::A);
                    // TODO send write signal to the memory
                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                1 => {
                    // The second clock cycle is necessary since the address and data bus were busy in the previous cycle
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LDAD() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow_mut().read_data_bus(Register::Z);
                    self.instruction_counter += 1;
                }
                1 => {
                    self.register_file.borrow_mut().read_data_bus(Register::W);
                    self.instruction_counter += 1;
                }
                2 => {
                    self.register_file.borrow().write_address_bus(Register::WZ);
                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                3 => {
                    self.alu.read_data_register(Register::Z);
                    self.alu.write_data_register(Register::A);
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LDAMD() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow_mut().read_data_bus(Register::Z);
                    self.instruction_counter += 1;
                }
                1 => {
                    self.register_file.borrow_mut().read_data_bus(Register::W);
                    self.instruction_counter += 1;
                }
                2 => {
                    self.register_file.borrow().write_address_bus(Register::WZ);
                    self.register_file.borrow().write_data_bus(Register::A);
                    // TODO send write signal to the memory
                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                3 => {
                    // This clock cycle is necessary since the address and data bus were busy in the previous cycle
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LDH() => match self.instruction_counter {
                0 => {
                    let c = self.register_file.borrow().read_u8(Register::C);
                    let address = self.idu.unsigned16(0xFF, c);
                    self.address_bus.borrow_mut().write(address);
                    // TODO send read signal to the memory (needs to be synchronous)
                    self.register_file.borrow_mut().read_data_bus(Register::Z);

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                1 => {
                    self.alu.read_data_register(Register::Z);
                    self.alu.write_data_register(Register::A);
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LDHM() => match self.instruction_counter {
                0 => {
                    let c = self.register_file.borrow().read_u8(Register::C);
                    let address = self.idu.unsigned16(0xFF, c);
                    self.address_bus.borrow_mut().write(address);
                    self.register_file.borrow_mut().write_data_bus(Register::A);
                    // TODO send write signal to the memory
                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                1 => {
                    // The second clock cycle is necessary since the address and data bus were busy in the previous cycle
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LDHD() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow_mut().read_data_bus(Register::Z);
                    self.instruction_counter += 1;
                }
                1 => {
                    let c = self.register_file.borrow().read_u8(Register::Z);
                    let address = self.idu.unsigned16(0xFF, c);
                    self.address_bus.borrow_mut().write(address);
                    // TODO send read signal to the memory (needs to be synchronous)
                    self.register_file.borrow_mut().read_data_bus(Register::Z);

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                2 => {
                    self.alu.read_data_register(Register::Z);
                    self.alu.write_data_register(Register::A);
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LDHDM() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow_mut().read_data_bus(Register::Z);
                    self.instruction_counter += 1;
                }
                1 => {
                    let c = self.register_file.borrow().read_u8(Register::Z);
                    let address = self.idu.unsigned16(0xFF, c);
                    self.address_bus.borrow_mut().write(address);
                    self.register_file.borrow_mut().write_data_bus(Register::A);
                    // TODO send write signal to the memory
                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                2 => {
                    // This clock cycle is necessary since the address and data bus were busy in the previous cycle
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::NOP() => {
                self.current_instruction = None;
            }
        }
    }
}
