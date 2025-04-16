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

    // SUB r
    // Subtracts from the 8-bit register A, the 8-bit register r, and stores the result back into the A register
    // Opcode 0b10010xxx, 1 byte, 1 cycle
    SUB(Register),

    // SUB (HL)
    // Subtracts from the 8-bit A register, data from the absolute address specified by the 16-bit register HL,
    // and stores the result back into the A register
    // Opcode 0b10010110, 1 byte, 2 cycles
    SUB_HL(),

    // SUB n
    // Subtracts from the 8-bit A register, the immediate data n, and stores the result back into the A register
    // Opcode 0b11010110, 2 bytes, 2 cycles
    SUBI(),

    // SBC r
    // Subtracts from the 8-bit A register, the carry flag and the 8-bit register r, and stores the result back
    // into the A register
    // Opcode 0b10011xxx, 1 byte, 1 cycle
    SBC(Register),

    // SBC (HL)
    // Subtracts from the 8-bit A register, the carry flag and data from the absolute address specified by the
    // 16-bit register HL, and stores the result back into the A register
    // Opcode 0b10011110, 1 byte, 2 cycles
    SBC_HL(),

    // SBC n
    // Subtracts from the 8-bit A register, the carry flag and the immediate data n, and stores the result back
    // into the A register
    // Opcode 0b11011111, 2 bytes, 2 cycles
    SBCI(),

    // CP r
    // Subtracts from the 8-bit A register, the 8-bit register r, and updates flags based on the result.
    // This instruction is basically identical to SUB r, but does not update the A register
    // Opcode 0b10111xxx, 1 byte, 1 cycle
    CP(Register),

    // CP (HL)
    // Subtracts from the 8-bit A register, data from the absolute address specified by the 16-bit
    // register HL, and updates flags based on the result. This instruction is basically identical to SUB
    // (HL), but does not update the A register
    // Opcode 0b10111110, 1 byte, 2 cycles
    CP_HL(),

    // CP n
    // Subtracts from the 8-bit A register, the immediate data n, and updates flags based on the result.
    // This instruction is basically identical to SUB n, but does not update the A register
    // Opcode 0b11111110, 2 byte, 2 cycles
    CPI(),

    // INC r
    // Increments data in the 8-bit register r
    // Opcode 0b00xxx100, 1 byte, 1 cycle
    INC(Register),

    // INC (HL)
    // Increments data at the absolute address specified by the 16-bit register HL
    // Opcode 0b00110100, 1 byte, 3 cycles
    INC_HL(),

    // DEC r
    // Decrements data in the 8-bit register r
    // Opcode 0b00xxx101, 1 byte, 1 cycle
    DEC(Register),

    // DEC (HL)
    // Decrements data at the absolute address specified by the 16-bit register HL
    // Opcode 0b00110101, 1 byte, 3 cycles
    DEC_HL(),

    // AND r
    // Performs a bitwise AND operation between the 8-bit register A and the 8-bit register r, and
    // stores the result back into the A register
    // Opcode 0b10100xxx, 1 byte, 1 cycle
    AND(Register),

    // AND (HL)
    // Performs a bitwise AND operation between the 8-bit A register and data from the absolute
    // address specified by the 16-bit register HL, and stores the result back into the A register
    // Opcode 0b10100110, 1 byte, 2 cycles
    AND_HL(),

    // AND n
    // Performs a bitwise AND operation between the 8-bit A register and immediate data n, and
    // stores the result back into the A register
    // Opcode 0b11100110, 2 bytes, 2 cycles
    ANDI(),

    // OR r
    // Performs a bitwise OR operation between the 8-bit register A and the 8-bit register r, and
    // stores the result back into the A register
    // Opcode 0b10110xxx, 1 byte, 1 cycle
    OR(Register),

    // OR (HL)
    // Performs a bitwise OR operation between the 8-bit A register and data from the absolute
    // address specified by the 16-bit register HL, and stores the result back into the A register
    // Opcode 0b10110110, 1 byte, 2 cycles
    OR_HL(),

    // OR n
    // Performs a bitwise OR operation between the 8-bit A register and immediate data n, and
    // stores the result back into the A register
    // Opcode 0b11110110, 2 bytes, 2 cycles
    ORI(),

    // XOR r
    // Performs a bitwise XOR operation between the 8-bit register A and the 8-bit register r, and
    // stores the result back into the A register
    // Opcode 0b10101xxx, 1 byte, 1 cycle
    XOR(Register),

    // XOR (HL)
    // Performs a bitwise XOR operation between the 8-bit A register and data from the absolute
    // address specified by the 16-bit register HL, and stores the result back into the A register
    // Opcode 0b10101110, 1 byte, 2 cycles
    XOR_HL(),

    // XOR n
    // Performs a bitwise XOR operation between the 8-bit A register and immediate data n, and
    // stores the result back into the A register
    // Opcode 0b11101110, 2 bytes, 2 cycles
    XORI(),

    // CCF
    // Flips the carry flag, and clears the N and H flags
    // Opcode 0b00111111, 1 byte, 1 cycle
    CCF(),

    // SCF
    // Sets the carry flag, and clears the N and H flags
    // Opcode 0b00110111, 1 byte, 1 cycle
    SCF(),

    // DAA
    // Adjusts register A so that the correct representation of Binary Coded Decimal (BCD) is obtained
    // Opcode 0b0010011, 1 byte, 1 cycle
    DAA(),

    // CPL
    // Flips all the bits in the 8-bit register A, and sets the N and H flags
    // Opcode 0b00101111, 1 byte, 1 cycle
    CPL(),

    // INC rr
    // Increments data in the 16-bit register rr
    // Opcode 0b00xx0011, 1 byte, 2 cycles
    INC_16(Register),

    // DEC rr
    // Decrements data in the 16-bit register rr
    // Opcode 0b00xx1011, 1 byte, 2 cycles
    DEC_16(Register),

    // ADD HL, rr
    // Adds to the 16-bit HL register pair, the 16-bit register rr, and stores the result back into the HL
    // register pair
    // Opcode 0b00xx1001, 1 byte, 2 cycles
    ADD_HL_16(Register),

    // ADD SP, e
    // Loads to the 16-bit register SP, 16-bit data calculated by adding the signed 8-bit operand e to
    // the 16-bit value of the register SP
    // Opcode 0b11101000, 2 bytes, 4 cycles
    ADD_SPE(),

    // RLCA
    // Rotates the 8-bit value of register A to the left, setting the carry flag and the
    // rightmost bit to the lost bit
    // Opcode 0b00000111, 1 byte, 1 cycle
    RLCA(),

    // RRCA
    // Rotates the 8-bit value of register A to the right, setting the carry flag and the
    // leftmost bit to the lost bit.
    // Opcode 0b00001111, 1 byte, 1 cycle
    RRCA(),

    // RLA
    // Rotates the 8-bit value of register A to the left, setting the rightmost bit to the carry
    // flag and the carry flag to the leftmost bit
    // Opcode 0b00010111, 1 byte, 1 cycle
    RLA(),

    // RRA
    // Rotates the 8-bit value of register A to the right, setting the leftmost bit to the carry
    // flag and the carry flag to the rightmost bit
    // Opcode 0b00011111, 1 byte, 1 cycle
    RRA(),

    // CB
    // Instructions prefixed with CB are at least 2 bytes long, meaning a second byte has to be read
    // in order to determine the true instruction behind this opcode
    // Opcode 0b11001011, 2+ bytes, 2+ cycles
    CB(),

    // RLC r
    // Rotates the 8-bit value of register r to the left, setting the carry flag and the
    // rightmost bit to the lost bit
    // Opcode CB 0b00000xxx, 2 bytes, 2 cycles
    RLC(Register),

    // RLC (HL)
    // Rotates 8-bit value at the absolute address specified by the 16-bit register HL to the left,
    // setting the carry flag and the rightmost bit to the lost bit
    // Opcode CB 0b00000110, 2 bytes, 4 cycles
    RLC_HL(),

    // RRC r
    // Rotates the 8-bit value of register r to the right, setting the carry flag and the
    // rightmost bit to the lost bit
    // Opcode CB 0b00001xxx, 2 bytes, 2 cycles
    RRC(Register),

    // RRC (HL)
    // Rotates 8-bit value at the absolute address specified by the 16-bit register HL to the right,
    // setting the carry flag and the rightmost bit to the lost bit
    // Opcode CB 0b00001110, 2 bytes, 4 cycles
    RRC_HL(),

    // RL r
    // Rotates the 8-bit value of register r to the left, setting the rightmost bit to the carry
    // flag and the carry flag to the leftmost bit
    // Opcode CB 0b00010xxx, 2 bytes, 2 cycles
    RL(Register),

    // RL (HL)
    // Rotates 8-bit value at the absolute address specified by the 16-bit register HL to the left,
    // setting the carry flag and the rightmost bit to the lost bit
    // Opcode CB 0b00010110, 2 bytes, 2 cycles
    RL_HL(),

    // RR r
    // Rotates the 8-bit value of register r to the right, setting the leftmost bit to the carry
    // flag and the carry flag to the rightmost bit
    // Opcode CB 0b00011xxx, 2 bytes, 2 cycles
    RR(Register),

    // RR (HL)
    // Rotates 8-bit value at the absolute address specified by the 16-bit register HL to the right,
    // setting the carry flag and the leftmost bit to the lost bit
    // Opcode CB 0b00011110, 2 bytes, 2 cycles
    RR_HL(),

    // SLA r
    // Arithmetically shifts the 8-bit value of register r to the left, setting the carry to the
    // leftmost bit
    // Opcode CB 0b00100xxx, 2 bytes, 2 cycles
    SLA(Register),

    // SLA (HL)
    // Arithmetically shifts the 8-bit value at the absolute address specified by the 16-bit
    // register HL to the left, setting the carry to the leftmost bit
    // Opcode CB 0b00100110, 2 bytes, 4 cycles
    SLA_HL(),

    // SRA r
    // Arithmetically shifts the 8-bit value of register r to the right, setting the carry to the
    // rightmost bit
    // Opcode CB 0b00101xxx, 2 bytes, 2 cycles
    SRA(Register),

    // SRA (HL)
    // Arithmetically shifts the 8-bit value at the absolute address specified by the 16-bit
    // register HL to the right, setting the carry to the rightmost bit
    // Opcode CB 0b00101110, 2 bytes, 4 cycles
    SRA_HL(),

    // SRL r
    // Logically shifts the 8-bit value of register r to the right, setting the carry to the
    // rightmost bit
    // Opcode CB 0b00111xxx, 2 bytes, 2 cycles
    SRL(Register),

    // SRL (HL)
    // Logically shifts the 8-bit value at the absolute address specified by the 16-bit
    // register HL to the right, setting the carry to the rightmost bit
    // Opcode CB 0b00111110, 2 bytes, 4 cycles
    SRL_HL(),

    // NOP
    // No operation. Can be used to add a delay of one machine cycle.
    // Opcode 0b00000000, 1 byte, 1 cycle
    NOP(),
}
