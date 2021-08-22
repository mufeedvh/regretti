use super::{
    lexer::{Lexer, Analyser},
    tokens::Token,
    messages::*,
};
use std::panic;
use std::process;

pub struct Function {
    pub(super) keyword: String,
    pub(super) value: String,
}

pub trait Builtin {
    fn execute(&self);
    fn print(&self);
    fn math_evaluator(expression: &str) -> f64;
}

use unescape::unescape;
use fasteval::{ez_eval, EmptyNamespace};

impl Builtin for Function {
    fn execute(&self) {
        match self.keyword.as_str() {
            "print" => self.print(),
            _ => (),
        }
    }

    fn print(&self) {
        use std::io::{BufWriter, Write};

        // get a locked buffered writer to stdout
        let stdout = std::io::stdout();
        let mut stdout = BufWriter::new(stdout.lock());
        
        // string formatting
        let mut chars: std::str::Chars;
        let fmt_out: &str;

        // format based on tokens
        match Lexer::tokenize(&Lexer, &self.value)[0] {
            // numbers represent as they are
            Token::Number => fmt_out = &self.value,
            // strings/objects need formatting (avoid pretty print)
            _ => {
                chars = self.value.trim().chars();
                chars.next();
                chars.next_back();
                fmt_out = chars.as_str()
            }
        }

        // escape sequence handling shouldn't panic
        let escaping = panic::catch_unwind(|| {
            unescape(fmt_out).unwrap()
        });

        let out = if escaping.is_ok() {
            unescape(fmt_out).unwrap()
        } else {
            push_error(
                format!(
                    "Panicked because of unsupported escape sequence on string: \"{}\"",
                    fmt_out
                )
            );
            process::exit(1)
        };

        // write formatted string to `stdout`
        let writeln = writeln!(
            stdout,
            "{}",
            out
        );
    
        match writeln {
            Ok(()) => (),
            Err(error) => panic!("Couldn't write to `stdout`: {:?}", error),
        };
    }

    fn math_evaluator(expression: &str) -> f64 {
        let mut ns = EmptyNamespace;

        let result = match ez_eval(expression, &mut ns) {
            Ok(result) => result,
            Err(error) => {
                push_error(
                    format!("Could not evaluate math expression \n\n\t{}\ndue to ↓ \n\n\t{:?}\n", expression, error)
                );
                process::exit(1)
            }
        };

        result
    }
}