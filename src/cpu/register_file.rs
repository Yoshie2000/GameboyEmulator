/*!
The register file holds most of the state of the CPU inside registers.
It contains the 16-bit Program Counter (PC), the 16-bit Stack Pointer (SP),
the 8-bit Accumulator (A), the Flags register (F), general-purpose register pairs
consisting of two 8-bit halves such as BC, DE, HL, and the special-purpose 8-bit registers
Instruction Register (IR) and Interrupt Enable (IE).
https://gekkio.fi/files/gb-docs/gbctr.pdf
*/
pub struct RegisterFile {
    pub(crate) ir: u8,
    pub(crate) ie: u8,

    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,

    pub(crate) pc: u16,
    pub(crate) sp: u16,
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        RegisterFile {
            ir: 0,
            ie: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        }
    }
}
