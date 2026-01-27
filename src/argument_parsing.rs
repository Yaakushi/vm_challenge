use std::env;

#[derive(Debug)]
pub struct ArgumentOptions {
    pub program_path: String, // Path to the program to be loaded in the VM.
    pub verbose: bool,        // Whether or not program should be in verbose mode.
}

pub fn parse_arguments() -> ArgumentOptions {
    let mut program_path: Option<String> = None;
    let mut is_verbose = false;
    let mut is_parsing_flags = true;

    for argument in env::args().skip(1) {
        let arg = argument.trim();

        if is_parsing_flags && argument.starts_with("--") {
            let long_flag = &arg[2..];
            match long_flag {
                "verbose" => {
                    is_verbose = true;
                }
                // This deals with "--" - Disable flag parsing.
                "" => {
                    is_parsing_flags = false;
                }
                _ => {
                    panic!("Unrecognized flag {arg}");
                }
            }
        } else if is_parsing_flags && argument.starts_with("-") {
            // String with all the letters grouped in a single '-'
            let short_flag_string = &arg[1..];

            for short_flag in short_flag_string.chars() {
                match short_flag {
                    'v' => {
                        is_verbose = true;
                    }
                    _ => {
                        panic!("Unrecognized flag {short_flag} (from {short_flag_string})");
                    }
                }
            }
        } else {
            // Assume anything that isn't prefixed by a - (or after a '--') is the path to the
            // program to be loaded in the VM.

            // If program_path is already set, panic.
            if program_path.is_some() {
                panic!("Attempting to reset program path.");
            }

            program_path = Some(argument);
        }
    }

    if let Some(path) = program_path {
        ArgumentOptions {
            program_path: path,
            verbose: is_verbose,
        }
    } else {
        panic!("Program path not provided.");
    }
}
