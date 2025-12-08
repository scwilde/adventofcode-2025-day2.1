use clap::Parser;

/// Advent of code 2025, Day 1, Puzzle 1:
/// Counts the number of zeroes while doing a series of modulo arithmetic operations
#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    /// Logging level to print. -v, -vv, or -vvv for levels 1, 2, or 3
    #[arg(short, action = clap::ArgAction::Count)]
    pub verbosity: u8,

    /// Text file containing puzzle input
    pub input_path: String
}

impl Cli {
    pub fn log<S: AsRef<str>>(&self, level: u8, msg: S) {
        if self.verbosity >= level {
            eprintln!("{}{}", "   ".repeat((level-1).into()), msg.as_ref());
        }
    }
}