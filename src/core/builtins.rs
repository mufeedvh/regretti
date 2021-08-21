pub struct Function {
    pub(super) keyword: String,
    pub(super) value: String,
}

pub trait Builtin {
    fn execute(&self);
    fn print(&self);
}

impl Builtin for Function {
    fn execute(&self) {
        match self.keyword.as_str() {
            "print" => self.print(),
            _ => (),
        }
    }

    fn print(&self) {
        use std::io::{BufWriter, Write};

        let stdout = std::io::stdout();
        let mut stdout = BufWriter::new(stdout.lock());
        
        let mut chars = self.value.trim().chars();
        chars.next();
        chars.next_back();
        let fmt_out = chars.as_str();

        let writeln = writeln!(stdout, "{}", fmt_out);
    
        match writeln {
            Ok(()) => (),
            Err(error) => panic!("Couldn't write to `stdout`: {:?}", error),
        };
    }
}