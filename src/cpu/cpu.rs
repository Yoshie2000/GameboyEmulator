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
            idu: IDU::new(Rc::clone(&address_bus), Rc::clone(&register_file)),
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
            None => panic!("This should be impossible to happen"),
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
            self.idu.increment_into(Register::PC);
        }
        self.skip_pc_increment = false;
    }

    pub fn decode(&mut self) -> Instruction {
        let instruction = self.register_file.borrow().read_u8(Register::IR);
        let instruction_header = instruction >> 6;
        let instruction_body_1 = (instruction >> 3) & 0x7;
        let instruction_body_2 = instruction & 0x7;

        match instruction_header {
            0b00 => {
                if instruction_body_1 == 1 && instruction_body_2 == 0 {
                    Instruction::LD_SP()
                } else if (instruction_body_1 & 0x1) == 0 && instruction_body_2 == 1 {
                    Instruction::LD_RR(Register::register_pair(instruction_body_1 >> 1))
                } else if instruction_body_1 == 0 && instruction_body_2 == 2 {
                    Instruction::LDAM_BC()
                } else if instruction_body_1 == 1 && instruction_body_2 == 2 {
                    Instruction::LDA_BC()
                } else if instruction_body_1 == 2 && instruction_body_2 == 2 {
                    Instruction::LDAM_DE()
                } else if instruction_body_1 == 3 && instruction_body_2 == 2 {
                    Instruction::LDA_DE()
                } else if instruction_body_1 == 4 && instruction_body_2 == 2 {
                    Instruction::LDH_HLPM()
                } else if instruction_body_1 == 5 && instruction_body_2 == 2 {
                    Instruction::LDH_HLP()
                } else if instruction_body_1 == 6 && instruction_body_2 == 2 {
                    Instruction::LDH_HLMM()
                } else if instruction_body_1 == 7 && instruction_body_2 == 2 {
                    Instruction::LDH_HLM()
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
            0b11 => {
                if instruction_body_1 == 7 && instruction_body_2 == 0 {
                    Instruction::LD_SPE()
                } else if (instruction_body_1 & 0x1) == 0 && instruction_body_2 == 1 {
                    Instruction::POP(Register::register_pair(instruction_body_1 >> 1))
                } else if (instruction_body_1 & 0x1) == 0 && instruction_body_2 == 5 {
                    Instruction::PUSH(Register::register_pair(instruction_body_1 >> 1))
                } else if instruction_body_1 == 7 && instruction_body_2 == 1 {
                    Instruction::LD_SP_HL()
                } else if instruction_body_1 == 7 && instruction_body_2 == 2 {
                    Instruction::LDAD()
                } else if instruction_body_1 == 6 && instruction_body_2 == 2 {
                    Instruction::LDH()
                } else if instruction_body_1 == 5 && instruction_body_2 == 2 {
                    Instruction::LDAMD()
                } else if instruction_body_1 == 4 && instruction_body_2 == 2 {
                    Instruction::LDH_M()
                } else if instruction_body_1 == 6 && instruction_body_2 == 0 {
                    Instruction::LDH_D()
                } else if instruction_body_1 == 4 && instruction_body_2 == 0 {
                    Instruction::LDH_DM()
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
                    let address = 0xFF00 | (c as u16);
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

            Instruction::LDH_M() => match self.instruction_counter {
                0 => {
                    let c = self.register_file.borrow().read_u8(Register::C);
                    let address = 0xFF00 | (c as u16);
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

            Instruction::LDH_D() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow_mut().read_data_bus(Register::Z);
                    self.instruction_counter += 1;
                }
                1 => {
                    let c = self.register_file.borrow().read_u8(Register::Z);
                    let address = 0xFF00 | (c as u16);
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

            Instruction::LDH_DM() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow_mut().read_data_bus(Register::Z);
                    self.instruction_counter += 1;
                }
                1 => {
                    let c = self.register_file.borrow().read_u8(Register::Z);
                    let address = 0xFF00 | (c as u16);
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

            Instruction::LDH_HLM() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow().write_address_bus(Register::HL);
                    // TODO send read signal to the memory (needs to be synchronous)
                    self.register_file.borrow_mut().read_data_bus(Register::Z);

                    self.idu.decrement_into(Register::HL);

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

            Instruction::LDH_HLMM() => match self.instruction_counter {
                0 => {
                    self.register_file
                        .borrow_mut()
                        .write_address_bus(Register::HL);
                    self.register_file.borrow_mut().write_data_bus(Register::A);

                    self.idu.decrement_into(Register::HL);

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                1 => {
                    // This clock cycle is necessary since the address and data bus were busy in the previous cycle
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LDH_HLP() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow().write_address_bus(Register::HL);
                    // TODO send read signal to the memory (needs to be synchronous)
                    self.register_file.borrow_mut().read_data_bus(Register::Z);

                    self.idu.increment_into(Register::HL);

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

            Instruction::LDH_HLPM() => match self.instruction_counter {
                0 => {
                    self.register_file
                        .borrow_mut()
                        .write_address_bus(Register::HL);
                    self.register_file.borrow_mut().write_data_bus(Register::A);

                    self.idu.increment_into(Register::HL);

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                1 => {
                    // This clock cycle is necessary since the address and data bus were busy in the previous cycle
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LD_RR(r) => match self.instruction_counter {
                0 => {
                    self.register_file.borrow_mut().read_data_bus(Register::Z);
                    self.instruction_counter += 1;
                }
                1 => {
                    self.register_file.borrow_mut().read_data_bus(Register::W);
                    self.instruction_counter += 1;
                }
                2 => {
                    let wz = self.register_file.borrow_mut().read_u16(Register::WZ);
                    self.register_file.borrow_mut().write_u16(r, wz);
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LD_SP() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow_mut().read_data_bus(Register::Z);
                    self.instruction_counter += 1;
                }
                1 => {
                    self.register_file.borrow_mut().read_data_bus(Register::W);
                    self.instruction_counter += 1;
                }
                2 => {
                    self.register_file
                        .borrow_mut()
                        .write_address_bus(Register::WZ);
                    self.data_bus
                        .borrow_mut()
                        .write(self.register_file.borrow_mut().read_u16_low(Register::SP));

                    let updated_wz = self.idu.increment_into(Register::WZ);

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                3 => {
                    self.register_file
                        .borrow_mut()
                        .write_address_bus(Register::WZ);
                    self.data_bus
                        .borrow_mut()
                        .write(self.register_file.borrow_mut().read_u16_low(Register::SP));

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                4 => {
                    // This clock cycle is necessary since the address and data bus were busy in the previous cycle
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LD_SP_HL() => match self.instruction_counter {
                0 => {
                    self.register_file
                        .borrow_mut()
                        .write_address_bus(Register::HL);
                    self.idu.write_into(Register::SP);

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                1 => {
                    // This clock cycle is necessary since the address and data bus were busy in the previous cycle
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::PUSH(r) => match self.instruction_counter {
                0 => {
                    self.register_file
                        .borrow_mut()
                        .write_address_bus(Register::SP);
                    self.idu.decrement_into(Register::SP);

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                1 => {
                    self.register_file
                        .borrow_mut()
                        .write_address_bus(Register::SP);
                    self.data_bus
                        .borrow_mut()
                        .write(self.register_file.borrow_mut().read_u16_high(r));
                    // TODO send memory write signal

                    self.idu.decrement_into(Register::SP);

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                2 => {
                    self.register_file
                        .borrow_mut()
                        .write_address_bus(Register::SP);
                    self.data_bus
                        .borrow_mut()
                        .write(self.register_file.borrow_mut().read_u16_low(r));
                    // TODO send memory write signal

                    // This seems like a no-op, though it's in the datasheet so we'll leave it here for completeness purposes
                    self.idu.write_into(Register::SP);

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

            Instruction::POP(r) => match self.instruction_counter {
                0 => {
                    self.register_file
                        .borrow_mut()
                        .write_address_bus(Register::SP);
                    // TODO Send read signal to memory
                    self.register_file.borrow_mut().read_data_bus(Register::Z);

                    self.idu.increment_into(Register::SP);

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                1 => {
                    self.register_file
                        .borrow_mut()
                        .write_address_bus(Register::SP);
                    // TODO Send read signal to memory
                    self.register_file.borrow_mut().read_data_bus(Register::W);

                    self.idu.increment_into(Register::SP);

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                2 => {
                    self.register_file
                        .borrow_mut()
                        .write_u16(r, self.register_file.borrow_mut().read_u16(Register::WZ));
                    self.current_instruction = None;
                }
                _ => {
                    panic!("Unimplemented instruction counter for instruction");
                }
            },

            Instruction::LD_SPE() => match self.instruction_counter {
                0 => {
                    self.register_file.borrow_mut().read_data_bus(Register::Z);
                    self.instruction_counter += 1;
                }
                1 => {
                    self.address_bus.borrow_mut().write(0x0000);

                    fn add_with_bitwise_carry(a: u8, b: i8) -> (u8, u8) {
                        let b_u8 = b as i16 as u8; // Wraps b as if it's added modulo 256
                        let sum = a.wrapping_add(b_u8);

                        let mut carry = 0u8;
                        let mut carry_bit = 0;

                        for i in 0..8 {
                            let a_bit = (a >> i) & 1;
                            let b_bit = (b_u8 >> i) & 1;

                            let (sum_bit, new_carry) = match (a_bit, b_bit, carry_bit) {
                                (0, 0, 0) => (0, 0),
                                (0, 0, 1) => (1, 0),
                                (0, 1, 0) => (1, 0),
                                (0, 1, 1) => (0, 1),
                                (1, 0, 0) => (1, 0),
                                (1, 0, 1) => (0, 1),
                                (1, 1, 0) => (0, 1),
                                (1, 1, 1) => (1, 1),
                                _ => panic!("This is impossible"),
                            };

                            if new_carry == 1 {
                                carry |= 1 << i;
                            }

                            carry_bit = new_carry;
                        }

                        (sum, carry)
                    }

                    // TODO this operation needs to be done by the ALU, though I'm not sure what to do with the flags yet
                    let (result, bitwise_carry) = add_with_bitwise_carry(
                        self.register_file.borrow().read_u16_low(Register::SP),
                        self.register_file.borrow().read_u8(Register::Z) as i8,
                    );
                    self.register_file
                        .borrow_mut()
                        .write_u8(Register::L, result);
                    let mut flags = self.register_file.borrow().flags();
                    flags.set_z(false);
                    flags.set_n(false);
                    flags.set_h(((bitwise_carry >> 3) & 0x1) == 0x1);
                    flags.set_c(((bitwise_carry >> 7) & 0x1) == 0x1);
                    self.register_file
                        .borrow_mut()
                        .write_u8(Register::F, flags.to_u8());

                    self.skip_pc_increment = true;
                    self.instruction_counter += 1;
                }
                2 => {
                    // TODO This also needs to be done by the ALU
                    let z_sign = self.register_file.borrow().read_u8(Register::Z) >> 7;
                    let adj = if z_sign == 0 { 0x00 } else { 0xFF };
                    let flags = self.register_file.borrow().flags();
                    let updated_h = self
                        .register_file
                        .borrow()
                        .read_u16_high(Register::SP)
                        .overflowing_add(flags.get_c() as u8)
                        .0
                        .overflowing_add(adj)
                        .0;
                    self.register_file
                        .borrow_mut()
                        .write_u8(Register::H, updated_h);

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
