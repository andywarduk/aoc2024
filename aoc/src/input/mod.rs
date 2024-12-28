use std::error::Error;

mod mmap;
use mmap::Input;

/// Parse whole input file with with a given transform
pub fn parse_input<T, F>(day: usize, mut tfn: F) -> Result<T, Box<dyn Error>>
where
    F: FnMut(&str) -> T,
{
    Ok(tfn(Input::new(day)?.as_str()?))
}

/// Parse an input file line by line to a vector with a given transform
pub fn parse_input_vec<T, F>(day: usize, tfn: F) -> Result<Vec<T>, Box<dyn Error>>
where
    F: FnMut(&str) -> T,
{
    Ok(Input::new(day)?.lines().map(tfn).collect())
}

/// Parse an input file with a single line with a given transform
pub fn parse_input_line<T, F>(day: usize, mut tfn: F) -> Result<T, Box<dyn Error>>
where
    F: FnMut(&str) -> T,
{
    Ok(tfn(Input::new(day)?
        .lines()
        .next()
        .expect("No line in input")))
}

/// Parse an input string to a vector with a given transform
pub fn parse_test_vec<T, F>(test: &str, tfn: F) -> Result<Vec<T>, Box<dyn Error>>
where
    F: FnMut(&str) -> T,
{
    Ok(test.lines().map(tfn).collect())
}
