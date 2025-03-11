struct Register {
    name: char,
    value: u64,
}

impl Register {
    /// Returns what's in the current register as a `u64`.
    fn read(&self) -> u64 {
        self.value
    }

    /// Writes a new `u64` value to the register.
    fn write(&mut self, n: u64) {
        self.value = n
    }

    /// Returns the name of the register which is a `char`.
    fn name(&self) -> char {
        self.name
    }
}
