use crate::cpu::register_file::Register;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    // LD r, r'
    // Load to the 8-bit register r, data from the 8-bit register r'
    // Opcode 0b01xxxyyy, 1 byte
    LD(Register, Register),

    // LD r, n
    // Load to the 8-bit register r, the immediate data n
    // Opcode 0b00xxx110, 2 bytes
    LDI(Register),

    // NOP
    // No operation. Can be used to add a delay of one machine cycle.
    // Opcode 0b00000000, 1 byte
    NOP(),
}
