use std::io::{self, Read, Write};

use crate::modules::raw_mode::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Self {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        for b in io::stdin().bytes() {
            match b {
                Ok(b) => {
                    let c = b as char;
                    print!("{}", c);
                    io::stdout().flush().unwrap();

                    if c == 'q' {
                        break;
                    }
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        disable_raw_mode().unwrap();
    }
}
