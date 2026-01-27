use crate::config_constants;

pub fn get_value(value: u16, registers: &[u16]) -> u16 {
    if value < config_constants::START_OF_REGISTER_INDEX {
        value
    } else {
        let reg_index = value - config_constants::START_OF_REGISTER_INDEX;
        if usize::from(reg_index) >= config_constants::NUM_OF_REGISTERS {
            panic!("Attempted to index an inexistent register.");
        } else {
            registers[reg_index as usize]
        }
    }
}

pub fn set_value(address: u16, value: u16, registers: &mut [u16]) {
    if address as usize
        >= config_constants::NUM_OF_REGISTERS + config_constants::START_OF_REGISTER_INDEX as usize
    {
        panic!(
            "Register index bigger than the max register index: {}.",
            address
        );
    }

    if address < config_constants::START_OF_REGISTER_INDEX {
        panic!(
            "Register index smaller than the start of register index: {}.",
            address
        );
    }

    let reg_index = (address - config_constants::START_OF_REGISTER_INDEX) as usize;
    registers[reg_index] = value;
}
