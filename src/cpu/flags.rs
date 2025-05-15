pub struct Flags {
    data: u8,
}

impl Flags {
    const Z: u8 = 0b0001;
    const N: u8 = 0b0010;
    const H: u8 = 0b0100;
    const C: u8 = 0b1000;

    pub fn from_u8(data: u8) -> Flags {
        Flags { data }
    }

    pub fn to_u8(&self) -> u8 {
        self.data
    }


    pub fn get_z(&self) -> bool {
        self.data & Self::Z != 0
    }

    pub fn get_n(&self) -> bool {
        self.data & Self::N != 0
    }

    pub fn get_h(&self) -> bool {
        self.data & Self::H != 0
    }

    pub fn get_c(&self) -> bool {
        self.data & Self::C != 0
    }

    fn set(&mut self, mask: u8, v: bool) {
        let set = self.data | mask;
        let unset = self.data & !mask;
        // generate a cmove instruction to set data
        self.data = if v { set } else { unset };
    }

    pub fn set_z(&mut self, v: bool) {
        self.set(Self::Z, v)
    }
    pub fn set_n(&mut self, v: bool) {
        self.set(Self::N, v)
    }
    pub fn set_h(&mut self, v: bool) {
        self.set(Self::H, v)
    }
    pub fn set_c(&mut self, v: bool) {
        self.set(Self::C, v)
    }
}
