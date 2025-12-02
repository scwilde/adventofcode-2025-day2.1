use clap::Parser;

/// Advent of code 2025, Day 1, Puzzle 1:
/// Counts the number of zeroes while doing a series of modulo arithmetic operations
#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Args {
    /// -v will print when each task begins.
    /// -vv will print each operation and its result.
    /// -vvv will print each operation, its result, and the execution time
    #[arg(short, action = clap::ArgAction::Count)]
    pub verbosity: u8,

    /// Text file containing puzzle input
    pub input_path: String
}

pub fn print_if_verbose<S: AsRef<str>>(args: &Args, msg: S) {
    if args.verbosity >= 1 {
        eprintln!("{}", msg.as_ref());
    }
}

pub fn print_if_vverbose<S: AsRef<str>>(args: &Args, msg: S) {
    if args.verbosity >= 2 {
        eprintln!("{}", msg.as_ref());
    }
}

pub fn time_if_vvverbose() {}