use std::io::{self, Read as _, Result, Write as _};

use crate::modules::raw_mode::RawModeGuard;

#[derive(Default)]
pub struct Editor;

impl Editor {
    pub fn run(&self) -> Result<()> {
        let _raw = RawModeGuard::new()?;
        let mut stdout = io::stdout().lock();
        Self::clear_screen(&mut stdout)?;

        for byte in io::stdin().lock().bytes() {
            let byte = byte?;

            match byte {
                // Carriage return (Enter): raw mode disables \r -> \r\n
                // translation, so emit both ourselves to move to a new line.
                b'\r' => Self::print_byte(&mut stdout, b"\r\n")?,
                _ => Self::print_byte(&mut stdout, &[byte])?,
            }

            if byte == 17 {
                break;
            }
        }
        Self::clear_screen(&mut stdout)?;
        Ok(())
    }

    fn clear_screen(stdout: &mut io::StdoutLock<'_>) -> Result<()> {
        Self::print_byte(stdout, b"\x1b[2J\x1b[H")
    }

    fn print_byte(stdout: &mut io::StdoutLock<'_>, byte: &[u8]) -> Result<()> {
        stdout.write_all(byte)?;
        stdout.flush()?;
        Ok(())
    }
}
