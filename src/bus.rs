pub struct Bus<T: Copy> {
    value: Option<T>,
}

impl<T: Copy> Bus<T> {
    pub fn new() -> Self {
        Bus { value: None }
    }

    pub fn write(&mut self, data: T) {
        self.value = Some(data)
    }

    pub fn clear(&mut self) {
        self.value = None
    }

    pub fn read(&self) -> Option<T> {
        self.value
    }
}
