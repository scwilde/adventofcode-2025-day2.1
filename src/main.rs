use std::error::Error;
use std::fs;
use clap::Parser;

mod cli;
mod utils;

fn is_id_valid(id: u64) -> Result<bool, String>{
    let digit_count = utils::count_digits(id, 10)?;
    if digit_count % 2 == 0 {
        let (lhs, rhs) = utils::splitnum(id, (digit_count/2) as u32);
        return Ok(lhs != rhs);
    }
    Ok(true)
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::Cli::parse();

    cli.log(1, "Reading puzzle input...");
    let data = fs::read_to_string(&cli.input_path)?;

    cli.log(1, "Parsing ID ranges...");
    let mut parsed_range_count = 0;
    let mut invalid_id_sum = 0;
    for id_range in data.split(",") {
        let mut splitrange = id_range.split("-");
        cli.log(2, format!("Parsing {}...", id_range));
        let id_range = utils::Range {
            lower: splitrange.next()
                .ok_or(format!("{} not a valid range.", id_range))?
                .parse().map_err(|err| format!("Problem parsing range {}: {}", id_range, err))?,
            upper: splitrange.next()
                .ok_or(format!("{} not a valid range.", id_range))?
                .parse().map_err(|err| format!("Problem parsing range {}: {}", id_range, err))?
        };
        
        cli.log(2, format!("Searching range {}..={}...", id_range.lower, id_range.upper));
        let mut invalid_id_count = 0;
        for id in id_range.lower..=id_range.upper {
            match is_id_valid(id)?{
                false => {
                    cli.log(3, format!("{} is not a valid ID!", id));
                    invalid_id_count += 1;
                    invalid_id_sum += id;
                },
                true => cli.log(3, format!("{} is a valid ID!", id))
            }
        }
        cli.log(2, format!(
            "{} invalid IDs; Invalid ID sum: {}",
            invalid_id_count, invalid_id_sum
        ));
        parsed_range_count += 1;
    }

    cli.log(1, format!("Parsed {} ID ranges! Invalid ID sum:", parsed_range_count));
    println!("{}", invalid_id_sum);
    Ok(())
}
