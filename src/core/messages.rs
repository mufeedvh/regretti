use std::io::{BufWriter, Write};
use std::io::Result;

use colored::*;

pub fn put_message(messages: Vec<String>) -> Result<()> {
    let stdout = std::io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    for message in messages {
        let writeln = writeln!(stdout, "{}", message);

        match writeln {
            Ok(()) => (),
            Err(error) => panic!("Couldn't write to `stdout`: {:?}", error),
        };
    }

    stdout.flush()?;

    Ok(())
}

pub fn push_error(message: String) {
    let stdout = std::io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    
    let writeln = writeln!(
        stdout,
        "{} {}",
        "[!] ERROR:".red().bold(), message.red()
    );

    match writeln {
        Ok(()) => (),
        Err(error) => panic!("Couldn't write to `stdout`: {:?}", error),
    };
}

pub fn push_warning(message: String) {
    let stdout = std::io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let writeln = writeln!(
        stdout,
        "{} {}",
        "[!] WARNING:".yellow().bold(), message.yellow()
    );    

    match writeln {
        Ok(()) => (),
        Err(error) => panic!("Couldn't write to `stdout`: {:?}", error),
    };
}