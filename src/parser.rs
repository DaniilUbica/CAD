#[derive(Clone, Debug)]
pub struct Error(pub &'static str);

impl std::fmt::Display for Error {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for Error {
    #[inline]
    fn description(&self) -> &str {
        &self.0
    }
}

pub fn parse_numbers(string: String, amount: usize, err_msg: &'static str) -> Result<Vec<(usize, usize)>, Error> {
    let numbers: Vec<usize> = string
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect();

    let pairs: Vec<(usize, usize)> = numbers.chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    if pairs.len() >= amount {
        return Ok(pairs[..amount].to_vec());
    }
    else {
        return Err(Error(err_msg));
    }
}