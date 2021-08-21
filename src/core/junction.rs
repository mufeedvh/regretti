use std::fs::File;
use std::str;
use std::process;

use memmap::Mmap;

use super::{
    messages,
    lexer::{Lexer, Analyser},
};

pub fn execute(script_path: &String) {
    let file = File::open(script_path);

    // verify open file
    let file = match &file {
        Ok(file) => file,
        Err(error) => {
            messages::push_error(format!("Failed to open script `{}` due to ↓ \n{:?}\n", script_path, error));
            process::exit(1)
        }
    };

    // read the script into a memory map
    let script_source = unsafe {
        Mmap::map(&file)
            .expect("Failed to load script")
    };

    // start lexical analysis
    Lexer::lexerize(
        &Lexer, str::from_utf8(script_source.get(..).unwrap()).unwrap()
    )
}