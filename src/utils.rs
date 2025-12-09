pub struct Range {
    pub lower: u64,
    pub upper: u64,
}

pub fn count_digits<N>(number: N, base: u8) -> Result<u32, String>
where u64: From<N>
{
    let number = u64::from(number);
    if number >= 9_007_199_254_740_992 {
        return Err(format!("{} is too large to have its digits accurately counted", number));
    }
    Ok((number as f64).abs().log(base as f64).ceil() as u32)
}

pub fn splitnum(num: u64, rhs_digits: u32) -> (u64, u64) {
    let rhs = num % u64::pow(10, rhs_digits);
    let lhs = num / u64::pow(10, rhs_digits);
    (lhs, rhs)
}