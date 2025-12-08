use clap::Parser;

/// Advent of code 2025, Day 1, Puzzle 1:
/// Counts the number of zeroes while doing a series of modulo arithmetic operations
#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Args {
    /// Verbosity level. Valid levels: 1, 2, 3
    #[arg(short, action = clap::ArgAction::Count)]
    pub verbosity: u8,

    /// Text file containing puzzle input
    pub input_path: String
}

impl Args {
    pub fn print_if_verbosity<S: AsRef<str>>(&self, v: u8, msg: S) {
        if self.verbosity >= v {
            eprintln!("{}{}", "   ".repeat(v.into()), msg.as_ref());
        }
    }
}