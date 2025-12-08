pub struct Range {
    pub lower: u64,
    pub upper: u64,
}

pub fn count_digits(number: u64, base: u32) -> u32
{(number as f64).abs().log(base as f64).ceil() as u32}

pub fn splitnum(num: u64, rhs_digits: u32) -> (u64, u64) {
    let rhs = num % u64::pow(10, rhs_digits);
    let lhs = num / u64::pow(10, rhs_digits);
    (lhs, rhs)
}