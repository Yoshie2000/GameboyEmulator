/*!
The control unit decodes the executed instructions and generates control signals for the rest of
the CPU core. It is also responsible for checking and dispatching interrupts.
https://gekkio.fi/files/gb-docs/gbctr.pdf
*/
pub struct ControlUnit {
    ime: bool,
}

impl ControlUnit {
    pub fn new() -> ControlUnit {
        ControlUnit { ime: false }
    }

    pub fn enable_interrupts(&mut self) {
        self.ime = true;
    }

    pub fn disable_interrupts(&mut self) {
        self.ime = false;
    }
}
