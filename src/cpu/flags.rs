pub struct Flags {
    data: u8,
}

impl Flags {
    pub fn from_u8(data: u8) -> Flags {
        Flags { data }
    }

    pub fn to_u8(&self) -> u8 {
        self.data
    }

    fn get(&self, idx: usize) -> bool {
        (self.data >> idx) & 0x1 == 0x1
    }

    pub fn get_z(&self) -> bool {
        self.get(0)
    }
    pub fn get_n(&self) -> bool {
        self.get(1)
    }
    pub fn get_h(&self) -> bool {
        self.get(2)
    }
    pub fn get_c(&self) -> bool {
        self.get(3)
    }

    fn set(&mut self, idx: usize, v: bool) {
        self.data = (self.data & !(0x1 >> idx)) | (((v as u8) & 0x1) << idx);
    }

    pub fn set_z(&mut self, v: bool) {
        self.set(0, v)
    }
    pub fn set_n(&mut self, v: bool) {
        self.set(1, v)
    }
    pub fn set_h(&mut self, v: bool) {
        self.set(2, v)
    }
    pub fn set_c(&mut self, v: bool) {
        self.set(3, v)
    }
}
