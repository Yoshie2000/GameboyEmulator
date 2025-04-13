use crate::cpu::register_file::Register;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    // LD r, r'
    // Load to the 8-bit register r, data from the 8-bit register r'
    // Opcode 0b01xxxyyy, 1 byte, 1 cycle
    LDR(Register, Register),

    // LD r, n
    // Load to the 8-bit register r, the immediate data n
    // Opcode 0b00xxx110, 2 bytes, 2 cycles
    LDI(Register),

    // LD r, (HL)
    // Load to the 8-bit register r, data from the absolute address specified by the 16-bit register HL
    // Opcode 0b01xxx110, 1 byte, 2 cycles
    LD(Register),

    // LD (HL), r
    // Load to the absolute address specified by the 16-bit register HL, data from the 8-bit register r
    // Opcode 0b01110xxx, 1 byte, 2 cycles
    LDM(Register),

    // LD (HL), n
    // Load to the absolute address specified by the 16-bit register HL, the immediate data n
    // Opcode 0b00110110, 2 bytes, 3 cycles
    LDMI(),

    // LD A, (BC)
    // Load to the 8-bit A register, data from the absolute address specified by the 16-bit register BC
    // Opcode 0b00001010, 1 byte, 2 cycles
    LDA_BC(),

    // LD A, (DE)
    // Load to the 8-bit A register, data from the absolute address specified by the 16-bit register DE
    // Opcode 0b00011010, 1 byte, 2 cycles
    LDA_DE(),

    // LD (BC), A
    // Load to the absolute address specified by the 16-bit register BC, data from the 8-bit register A
    // Opcode 0b00000010, 1 byte, 2 cycles
    LDAM_BC(),

    // LD (DE), A
    // Load to the absolute address specified by the 16-bit register DE, data from the 8-bit register A
    // Opcode 0b00010010, 1 byte, 2 cycles
    LDAM_DE(),

    // LD A, (nn)
    // Load to the 8-bit register A, data from the absolute address specified by the 16-bit operand nn
    // Opcode 0b11111010, 3 bytes, 4 cycles
    LDAD(),

    // LD (nn), A
    // Load to the absolute address specified by the 16-bit operand nn, data from the 8-bit register A
    // Opcode 0b11101010, 3 bytes, 4 cycles
    LDAMD(),

    // LDH A, (C)
    // Load to the 8-bit register A, data from the address specified by the 8-bit C register. The full
    // 16-bit absolute address is obtained by setting the most significant byte to 0xFF and the least
    // significant byte to the value of C, so the possible range is 0xFF00-0xFFFF
    // Opcode 0b11110010, 1 byte, 2 cycles
    LDH(),

    // LDH (C), A
    // Load to the address specified by the 8-bit C register, data from the 8-bit register A. The full
    // 16-bit absolute address is obtained by setting the most significant byte to 0xFF and the least
    // significant byte to the value of C, so the possible range is 0xFF00-0xFFFF
    // Opcode 0b11100010, 1 byte, 2 cycles
    LDHM(),

    // LDH A, (n)
    // Load to the 8-bit register A, data from the address specified by the 8-bit immediate data n. The
    // full 16-bit absolute address is obtained by setting the most significant byte to 0xFF and the
    // least significant byte to the value of n, so the possible range is 0xFF00-0xFFFF
    // Opcode 0b11110000, 2 bytes, 3 cycles
    LDHD(),

    // LDH (n), A
    // Load to the address specified by the 8-bit immediate data n, data from the 8-bit register A. The
    // full 16-bit absolute address is obtained by setting the most significant byte to 0xFF and the
    // least significant byte to the value of n, so the possible range is 0xFF00-0xFFFF
    // Opcode 0b11100000, 2 bytes, 3 cycles
    LDHDM(),

    // NOP
    // No operation. Can be used to add a delay of one machine cycle.
    // Opcode 0b00000000, 1 byte, 1 cycle
    NOP(),
}
