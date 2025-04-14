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
    LDH_M(),

    // LDH A, (n)
    // Load to the 8-bit register A, data from the address specified by the 8-bit immediate data n. The
    // full 16-bit absolute address is obtained by setting the most significant byte to 0xFF and the
    // least significant byte to the value of n, so the possible range is 0xFF00-0xFFFF
    // Opcode 0b11110000, 2 bytes, 3 cycles
    LDH_D(),

    // LDH (n), A
    // Load to the address specified by the 8-bit immediate data n, data from the 8-bit register A. The
    // full 16-bit absolute address is obtained by setting the most significant byte to 0xFF and the
    // least significant byte to the value of n, so the possible range is 0xFF00-0xFFFF
    // Opcode 0b11100000, 2 bytes, 3 cycles
    LDH_DM(),

    // LDH A, (HL-)
    // Load to the 8-bit register A, data from the absolute address specified by the 16-bit register HL.
    // The value of HL is decremented after the memory read
    // Opcode 0b00111010, 1 byte, 2 cycles
    LDH_HLM(),

    // LDH (HL-), A
    // Load to the absolute address specified by the 16-bit register HL, data from the 8-bit register A.
    // The value of HL is decremented after the memory write
    // Opcode 0b00110010, 1 byte, 2 cycles
    LDH_HLMM(),

    // LDH A, (HL+)
    // Load to the 8-bit register A, data from the absolute address specified by the 16-bit register HL.
    // The value of HL is incremented after the memory read
    // Opcode 0b00101010, 1 byte, 2 cycles
    LDH_HLP(),

    // LDH (HL+), A
    // Load to the absolute address specified by the 16-bit register HL, data from the 8-bit register A.
    // The value of HL is incremented after the memory write
    // Opcode 0b00100010, 1 byte, 2 cycles
    LDH_HLPM(),

    // LD rr, nn
    // Load to the 16-bit register rr, the immediate 16-bit data nn
    // Opcode 0b00xx0001, 3 bytes, 3 cycles
    LD_RR(Register),

    // LD (nn), SP
    // Load to the absolute address specified by the 16-bit operand nn, data from the 16-bit register SP
    // Opcode 0b00001000, 3 bytes, 5 cycles
    LD_SP(),

    // LD SP, HL
    // Load to the 16-bit register SP, data from the 16-bit register HL
    // Opcode 0b11111001, 1 byte, 2 cycles
    LD_SP_HL(),

    // PUSH rr
    // Push to the stack memory, data from the 16-bit register rr
    // Opcode 0b11xx0101, 1 byte, 4 cycles
    PUSH(Register),

    // POP rr
    // Pops to the 16-bit register rr, data from the stack memory
    // This instruction does not do calculations that affect flags, but POP AF completely replaces the
    // F register value, so all flags are changed based on the 8-bit data that is read from memory
    // Opcode 0b11xx0001, 1 byte, 3 cycles
    POP(Register),

    // LD HL, SP+e
    // Load to the HL register, 16-bit data calculated by adding the signed 8-bit operand e to the 16-
    // bit value of the SP register
    // Opcode 0b11111000, 2 bytes, 3 cycles
    LD_SPE(),

    // ADD r
    // Adds to the 8-bit register A, the 8-bit register r, and stores the result back into the A register
    // Opcode 0b10000xxx, 1 byte, 1 cycle
    ADD(Register),

    // ADD (HL)
    // Adds to the 8-bit A register, data from the absolute address specified by the 16-bit register HL,
    // and stores the result back into the A register
    // Opcode 0b10000110, 1 byte, 2 cycles
    ADD_HL(),

    // ADD n
    // Adds to the 8-bit A register, the immediate data n, and stores the result back into the A register
    // Opcode 0b11000110, 2 bytes, 2 cycles
    ADDI(),

    // ADC r
    // Adds to the 8-bit A register, the carry flag and the 8-bit register r, and stores the result back
    // into the A register
    // Opcode 0b10001xxx, 1 byte, 1 cycle
    ADC(Register),

    // ADC (HL)
    // Adds to the 8-bit A register, the carry flag and data from the absolute address specified by the
    // 16-bit register HL, and stores the result back into the A register
    // Opcode 0b10001110, 1 byte, 2 cycles
    ADC_HL(),

    // ADC n
    // Adds to the 8-bit A register, the carry flag and the immediate data n, and stores the result back
    // into the A register
    // Opcode 0b11001111, 2 bytes, 2 cycles
    ADCI(),

    // NOP
    // No operation. Can be used to add a delay of one machine cycle.
    // Opcode 0b00000000, 1 byte, 1 cycle
    NOP(),
}
