use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process;

mod argument_parsing;
mod config_constants;
mod number_processor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start-up storage regions
    let mut vm_memory: [u16; config_constants::MEMORY_SIZE] = [0; config_constants::MEMORY_SIZE];
    let mut vm_stack: Vec<u16> = Vec::new();
    let mut vm_registers: [u16; config_constants::NUM_OF_REGISTERS] =
        [0; config_constants::NUM_OF_REGISTERS];
    let mut program_counter: usize = 0;

    // Start-up variables for stdin read.
    let mut input_len = 0;
    let mut input_string = String::new();
    let input_stdin = io::stdin();
    let mut stdin_iterator = input_stdin.lock().lines();

    let parsed_options = argument_parsing::parse_arguments();

    load_program(&mut vm_memory, parsed_options)?;

    loop {
        // if vm_memory[program_counter] != 21 && vm_memory[program_counter] != 19 {
        //     println!("OPCODE: {}", vm_memory[program_counter]);
        // }
        match vm_memory[program_counter] {
            0 => {
                // Halt.
                println!("*** Halting!");
                process::exit(0);
            }
            1 => {
                // set: 1 a b
                //   set register <a> to the value of <b>
                let param_a = vm_memory[program_counter + 1];
                let param_b =
                    number_processor::get_value(vm_memory[program_counter + 2], &vm_registers);

                number_processor::set_value(param_a, param_b, &mut vm_registers);

                program_counter += 2;
            }
            2 => {
                //push: 2 a
                //  push <a> onto the stack
                let param_a =
                    number_processor::get_value(vm_memory[program_counter + 1], &vm_registers);

                vm_stack.push(param_a);

                program_counter += 1;
            }
            3 => {
                //pop: 3 a
                //  remove the top element from the stack and write it into <a>; empty stack = error
                let param_a = vm_memory[program_counter + 1];

                if vm_stack.is_empty() {
                    panic!("FATAL: Attempting to pop with the stack empty.");
                }

                number_processor::set_value(param_a, vm_stack.pop().unwrap(), &mut vm_registers);

                program_counter += 1;
            }
            4 => {
                //eq: 4 a b c
                //  set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
                let param_a = vm_memory[program_counter + 1];
                let param_b =
                    number_processor::get_value(vm_memory[program_counter + 2], &vm_registers);
                let param_c =
                    number_processor::get_value(vm_memory[program_counter + 3], &vm_registers);

                number_processor::set_value(
                    param_a,
                    if param_b == param_c { 1 } else { 0 },
                    &mut vm_registers,
                );

                program_counter += 3;
            }
            5 => {
                //gt: 5 a b c
                //  set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
                let param_a = vm_memory[program_counter + 1];
                let param_b =
                    number_processor::get_value(vm_memory[program_counter + 2], &vm_registers);
                let param_c =
                    number_processor::get_value(vm_memory[program_counter + 3], &vm_registers);

                number_processor::set_value(
                    param_a,
                    if param_b > param_c { 1 } else { 0 },
                    &mut vm_registers,
                );

                program_counter += 3;
            }
            6 => {
                // jump to <a>
                let param =
                    number_processor::get_value(vm_memory[program_counter + 1], &vm_registers);

                // Since PC is going to increment at the end of this match, we set PC to
                // destination_address - 1.
                program_counter = (param as usize) - 1;
            }
            7 => {
                // if <a> is nonzero, jump to <b>
                let param_a =
                    number_processor::get_value(vm_memory[program_counter + 1], &vm_registers);
                let param_b =
                    number_processor::get_value(vm_memory[program_counter + 2], &vm_registers);

                if param_a != 0 {
                    // Since PC is going to increment at the end of this match, we set PC to
                    // destination_address - 1.
                    program_counter = (param_b as usize) - 1;
                } else {
                    // If we're not jumping, simply increment PC by two since we had two parameters
                    // here.
                    program_counter += 2;
                }
            }
            8 => {
                //jf: 8 a b
                //  if <a> is zero, jump to <b>
                let param_a =
                    number_processor::get_value(vm_memory[program_counter + 1], &vm_registers);
                let param_b =
                    number_processor::get_value(vm_memory[program_counter + 2], &vm_registers);

                if param_a == 0 {
                    // Since PC is going to increment at the end of this match, we set PC to
                    // destination_address - 1.
                    program_counter = (param_b as usize) - 1;
                } else {
                    // If we're not jumping, simply increment PC by two since we had two parameters
                    // here.
                    program_counter += 2;
                }
            }
            9 => {
                //add: 9 a b c
                //  assign into <a> the sum of <b> and <c> (modulo 32768)
                let param_a = vm_memory[program_counter + 1];
                let param_b =
                    number_processor::get_value(vm_memory[program_counter + 2], &vm_registers);
                let param_c =
                    number_processor::get_value(vm_memory[program_counter + 3], &vm_registers);

                number_processor::set_value(
                    param_a,
                    (param_b + param_c) % 0x8000,
                    &mut vm_registers,
                );

                program_counter += 3;
            }
            10 => {
                //mult: 10 a b c
                //  store into <a> the product of <b> and <c> (modulo 32768)
                let param_a = vm_memory[program_counter + 1];
                let param_b =
                    number_processor::get_value(vm_memory[program_counter + 2], &vm_registers);
                let param_c =
                    number_processor::get_value(vm_memory[program_counter + 3], &vm_registers);

                number_processor::set_value(
                    param_a,
                    param_b.wrapping_mul(param_c) % 0x8000,
                    &mut vm_registers,
                );

                program_counter += 3;
            }
            11 => {
                //mod: 11 a b c
                //  store into <a> the remainder of <b> divided by <c>
                let param_a = vm_memory[program_counter + 1];
                let param_b =
                    number_processor::get_value(vm_memory[program_counter + 2], &vm_registers);
                let param_c =
                    number_processor::get_value(vm_memory[program_counter + 3], &vm_registers);

                number_processor::set_value(param_a, param_b % param_c, &mut vm_registers);

                program_counter += 3;
            }
            12 => {
                //and: 12 a b c
                //  stores into <a> the bitwise and of <b> and <c>
                let param_a = vm_memory[program_counter + 1];
                let param_b =
                    number_processor::get_value(vm_memory[program_counter + 2], &vm_registers);
                let param_c =
                    number_processor::get_value(vm_memory[program_counter + 3], &vm_registers);

                number_processor::set_value(param_a, param_b & param_c, &mut vm_registers);

                program_counter += 3;
            }
            13 => {
                //or: 13 a b c
                //  stores into <a> the bitwise or of <b> and <c>
                let param_a = vm_memory[program_counter + 1];
                let param_b =
                    number_processor::get_value(vm_memory[program_counter + 2], &vm_registers);
                let param_c =
                    number_processor::get_value(vm_memory[program_counter + 3], &vm_registers);

                number_processor::set_value(param_a, param_b | param_c, &mut vm_registers);

                program_counter += 3;
            }
            14 => {
                //not: 14 a b
                //  stores 15-bit bitwise inverse of <b> in <a>
                let param_a = vm_memory[program_counter + 1];
                let param_b =
                    number_processor::get_value(vm_memory[program_counter + 2], &vm_registers);

                // & 0x7fff since our words are actually 15-bits.
                number_processor::set_value(param_a, (!param_b) & 0x7fff, &mut vm_registers);

                program_counter += 2;
            }
            15 => {
                //rmem: 15 a b
                //  read memory at address <b> and write it to <a>
                let param_a = vm_memory[program_counter + 1];
                let param_b =
                    number_processor::get_value(vm_memory[program_counter + 2], &vm_registers);

                let value_from_memory = vm_memory[param_b as usize];
                number_processor::set_value(param_a, value_from_memory, &mut vm_registers);

                program_counter += 2;
            }
            16 => {
                //wmem: 16 a b
                //  write the value from <b> into memory at address <a>
                let param_a =
                    number_processor::get_value(vm_memory[program_counter + 1], &vm_registers);
                let param_b =
                    number_processor::get_value(vm_memory[program_counter + 2], &vm_registers);

                vm_memory[param_a as usize] = param_b;

                program_counter += 2;
            }
            17 => {
                //call: 17 a
                //  write the address of the next instruction to the stack and jump to <a>
                let param_a =
                    number_processor::get_value(vm_memory[program_counter + 1], &vm_registers);

                vm_stack.push(program_counter as u16 + 2);

                // Set to a - 1 since PC is going to be incremented after this.
                program_counter = param_a as usize - 1;
            }
            18 => {
                //ret: 18
                //  remove the top element from the stack and jump to it; empty stack = halt
                if vm_stack.is_empty() {
                    panic!("FATAL: Attempting to ret with an empty stack.");
                }

                let dest_address = vm_stack.pop().unwrap();

                program_counter = dest_address as usize - 1;
            }
            19 => {
                // write the character represented by ascii code <a> to the terminal
                let param =
                    number_processor::get_value(vm_memory[program_counter + 1], &vm_registers);

                // Would love to find a less hacky way to do this.
                let char = param as u8 as char;

                print!("{}", char);
                let _ = std::io::stdout().flush();

                program_counter += 1;
            }
            20 => {
                //in: 20 a
                //  read a character from the terminal and write its ascii code to <a>; it can be assumed that once input starts, it will continue until a newline is encountered; this means that you can safely read whole lines from the keyboard instead of having to figure out how to read individual characters
                while input_len == input_string.len() {
                    // We append a newline since the program expects it.
                    let next_iterator = stdin_iterator.next();

                    if let Some(Ok(read_line)) = next_iterator {
                        // We append a new-line since our iterator strips them.
                        input_string = read_line + "\n";
                    } else {
                        panic!("FATAL: Couldn't read new line.");
                    }

                    input_len = 0;
                }

                let param_a = vm_memory[program_counter + 1];

                number_processor::set_value(
                    param_a,
                    input_string.as_bytes()[input_len] as u16,
                    &mut vm_registers,
                );

                input_len += 1;

                program_counter += 1;
            }
            21 => {
                // no-op.
            }
            _ => {
                panic!("FATAL: Undefined opcode {}.", vm_memory[program_counter]);
            }
        }

        program_counter += 1;
    }

    //Ok(())
}

fn load_program(
    vm_memory: &mut [u16; 32768],
    parsed_options: argument_parsing::ArgumentOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = BufReader::new(File::open(parsed_options.program_path)?).bytes();
    let mut word_index = 0;
    let mut is_low_byte = true;

    for byte in file {
        if byte.is_err() {
            panic!("Error reading program.");
        }

        let byte = byte?;

        if is_low_byte {
            vm_memory[word_index] = byte as u16;
        } else {
            vm_memory[word_index] += (byte as u16) << 8;
            word_index += 1;
        }

        is_low_byte = !is_low_byte;
    }

    if !is_low_byte {
        panic!("Memory load ended in invalid state?");
    }

    if parsed_options.verbose {
        println!("Loaded {} words from the program into memory.", word_index);
    }

    Ok(())
}
