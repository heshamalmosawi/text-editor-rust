use std::io::{self, Read, Write};
mod modules;

use crate::modules::raw_mode::{disable_raw_mode, enable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();
    for chars in io::stdin().bytes() {
        let input = chars.unwrap() as char;
        if input == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
        print!("{}", input);
        io::stdout().flush().unwrap();
    }
}
