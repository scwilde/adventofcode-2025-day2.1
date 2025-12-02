use std::error::Error;
use std::fs;
use clap::Parser;

mod cli;

struct Bounds {
    lower: i32,
    upper: i32,
}

fn count_digits<N>(number: N, base: N) -> i32
where f64: From<N>
{ (f64::from(number)).abs().log(f64::from(base)).ceil() as i32 }

fn splitnum(num: i32, rhs_digits: u32) -> (i32, i32) {
    let rhs = num % i32::pow(10, rhs_digits);
    let lhs = num / i32::pow(10, rhs_digits);
    (lhs, rhs)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    cli::print_if_verbose(&args, "Reading puzzle input...");
    let data = fs::read_to_string(&args.input_path)?;

    let mut invalid_id_total = 0;
    for range in data.split(",") {
        let mut splitrange = range.split("-");
        cli::print_if_vverbose(&args, format!("Parsing {}...", range));
        let bounds = Bounds {
            lower: splitrange.next()
                .ok_or(format!("{} not a valid range.", range))?
                .parse().map_err(|err| format!("Problem parsing range {}: {}", range, err))?,
            upper: splitrange.next()
                .ok_or(format!("{} not a valid range.", range))?
                .parse().map_err(|err| format!("Problem parsing range {}: {}", range, err))?
        };
        cli::print_if_vverbose(&args, format!("Parsed to range {}..{}", bounds.lower, bounds.upper));

        let mut invalid_id_count = 0;
        for val in bounds.lower..=bounds.upper {
            let digits = count_digits(val, 10);
            match match digits % 2 {
                0 => {
                    let (lhs, rhs) = splitnum(val, (digits / 2) as u32);
                    lhs == rhs
                },
                _ => {false}
            }{
                true => {
                    cli::print_if_vverbose(&args, format!("{} is not a valid ID!", val));
                    invalid_id_count += 1;
                    invalid_id_total += val;
                },
                false => cli::print_if_vverbose(&args, format!("{} is a valid ID!", val))
            }
        }
    }

    Ok(())
}
