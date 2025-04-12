/*!
A dedicated 16-bit Increment/Decrement Unit (IDU) is capable of performing only simple increment/decrement
operations on the 16-bit address bus value, but they can be performed independently of the ALU, improving
maximum performance of the CPU core. The IDU always outputs its result back to the register file, where it can
be written to a register pair or a 16-bit register.
https://gekkio.fi/files/gb-docs/gbctr.pdf
*/

pub struct IDU {}

impl IDU {
    pub fn new() -> IDU {
        IDU {}
    }
}
