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

pub fn parse_points(string: String, amount: usize) -> Result<Vec<i32>, Error> {
    let numbers: Vec<i32> = string
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect();

    if numbers.len() >= amount * 2 {
        return Ok(numbers[..amount * 2].to_vec());
    }
    else {
        return Err(Error("Ошибка, Вы ввели слишком мало точек!"));
    }
}

pub fn parse_edges(string: String, amount: usize) -> Result<Vec<i32>, Error> {
    let numbers: Vec<i32> = string
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect();

    if numbers.len() >= amount * 2 {
        return Ok(numbers[..amount * 2].to_vec());
    }
    else {
        return Err(Error("Ошибка, Вы ввели слишком мало рёбер!"));
    }
}