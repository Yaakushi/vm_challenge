pub const NUM_OF_REGISTERS: usize = 8;
pub const START_OF_REGISTER_INDEX: u16 = 1 << 15;

// Our memory is addressable from 0..START_OF_REGISTER_INDEX.
pub const MEMORY_SIZE: usize = START_OF_REGISTER_INDEX as usize;
