#[repr(C)]
pub struct Stack([usize; 32]);

impl Stack {
    pub fn a0(&self) -> usize {
        self.0[9]
    }
    pub fn a1(&self) -> usize {
        self.0[10]
    }
    pub fn a7(&self) -> usize {
        self.0[16]
    }
    /// Sets the return value.
    pub fn set_ret(&mut self, ret: usize) {
        self.0[9] = ret;
    }
}
