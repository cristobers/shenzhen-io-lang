/*
    Definition for the registers to be used witin the machines.
*/

#[derive(Debug)]
pub struct Register {
    pub value: u64,
}

impl Register {
    /// Writes a new `u64` value to the register.
    pub fn write(&mut self, n: u64) {
        self.value = n;
    }
}
