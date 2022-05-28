#[repr(C)]
pub struct Byte(u8);
impl Byte {
    pub fn is_set(&self, index: u8) -> bool {
        if index < 8 {
            return self.0 & 1 << index != 0;
        }
        false
    }
    pub fn set(&mut self, index: u8) {
        if index < 8 {
            self.0 = self.0 | 1 << index;
        }
    }
    pub fn write(&mut self, data: u8) {
        self.0 = data
    }
}
