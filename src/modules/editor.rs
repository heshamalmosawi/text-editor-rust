use std::io::{self, Read, Result, Write};

use crate::modules::raw_mode::RawModeGuard;

#[derive(Default)]
pub struct Editor;

impl Editor {
    pub fn run(&self) -> Result<()> {
        let _raw = RawModeGuard::new()?;
        let mut stdout = io::stdout().lock();

        for byte in io::stdin().bytes() {
            let byte = byte?;
            stdout.write_all(&[byte])?;
            stdout.flush()?;

            if byte == b'q' {
                break;
            }
        }
        Ok(())
    }
}
