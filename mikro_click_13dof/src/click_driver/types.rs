/// Trait used to represent a readable register value.
pub trait Register {
    // Device id
    const DEVICE: u8;

    // Register value
    const REGISTER: u8;

    // Data size
    const SIZE: u8;
}

/// Trait used to represent a readable value
pub trait ReadRegister {
    // Set raw data (after reading, or instance)
    fn set_raw_data(&mut self, _data: &[u8]);
}

/// Trait used to represent a writable register value.
pub trait WriteRegister {
    // Get the raw data buffer
    fn get_raw_data(&self) -> u8;
}
