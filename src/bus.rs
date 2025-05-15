use std::cell::Cell;

pub struct Bus<T: Copy> {
    value: Cell<Option<T>>,
}

impl<T: Copy> Default for Bus<T> {
    fn default() -> Bus<T> {
        Self {
            value: Cell::new(None),
        }
    }
}

impl<T: Copy> Bus<T> {
    pub fn new() -> Self {
        Bus {
            value: Cell::new(None),
        }
    }

    pub fn write(&self, data: T) {
        self.value.set(Some(data));
    }

    pub fn clear(&self) {
        self.value.set(None);
    }

    pub fn read(&self) -> Option<T> {
        self.value.get()
    }
}
