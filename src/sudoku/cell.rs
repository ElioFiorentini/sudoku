#[derive(Clone, Copy, Debug)]
pub struct Cell {
    value: u8,
    locked: bool,
}
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value == 0 {
            write!(f, " ")
        } else {
            write!(f, "{}", self.value)
        }
    }
}
impl Cell {
    pub fn new() -> Self {
        Cell {
            value: 0,
            locked: false,
        }
    }
    pub fn from(value: u8) -> Self {
        Cell {
            value,
            locked: false,
        }
    }
    pub fn set(&mut self, value: u8) -> Result<u8, &'static str> {
        if self.locked {
            Err("cell locked")
        } else if value > 9 {
            Err("invalid value")
        } else {
            let original_value = self.value().clone();
            self.value = value;
            Ok(original_value)
        }
    }
    pub fn value(&self) -> u8 {
        self.value
    }
    pub fn is_empty(&self) -> bool {
        self.value == 0
    }
    pub fn is_filled(&self) -> bool {
        self.value != 0
    }
    pub fn is_locked(&self) -> bool {
        self.locked
    }
    pub fn lock(&mut self) {
        self.locked = true;
    }
    pub fn unlock(&mut self) {
        self.locked = false;
    }
}
