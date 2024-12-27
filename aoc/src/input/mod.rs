use std::error::Error;

mod mmap;
use mmap::Input;

/// Parse an input file line by line to a vector with a given transform
pub fn parse_input_vec<T, F>(day: usize, tfn: F) -> Result<Vec<T>, Box<dyn Error>>
where
    F: FnMut(&str) -> T,
{
    let input = Input::new(day)?;
    Ok(parse_buf_vec(input.lines(), tfn))
}

/// Parse an input file with a single line with a given transform
pub fn parse_input_line<T, F>(day: usize, tfn: F) -> Result<T, Box<dyn Error>>
where
    F: FnMut(&str) -> T,
{
    let input = Input::new(day)?;
    Ok(parse_buf_line(input.lines(), tfn))
}

/// Parse input file sections line by line to a vector with a given transform
pub fn parse_input_sections<T, F>(day: usize, tfn: F) -> Result<Vec<T>, Box<dyn Error>>
where
    F: FnMut(&str) -> T,
{
    let input = Input::new(day)?;
    Ok(parse_buf_vec(input.lines(), tfn))
}

/// Reads an input file to a string
pub fn read_input_file(day: usize) -> Result<String, Box<dyn Error>> {
    Input::new(day)?.to_string()
}

/// Parse an input string to a vector with a given transform
pub fn parse_test_vec<T, F>(test: &str, tfn: F) -> Result<Vec<T>, Box<dyn Error>>
where
    F: FnMut(&str) -> T,
{
    Ok(parse_buf_vec(test.lines(), tfn))
}

/// Parse an test input file to a vector with a given transform
pub fn parse_test_input_vec<T, F>(
    day: usize,
    example: usize,
    tfn: F,
) -> Result<Vec<T>, Box<dyn Error>>
where
    F: FnMut(&str) -> T,
{
    Ok(parse_buf_vec(
        Input::new_example(day, example)?.lines(),
        tfn,
    ))
}

/// Parse a lines iterator to a vector with a given transform
fn parse_buf_vec<'a, T, F>(lines: impl Iterator<Item = &'a str>, tfn: F) -> Vec<T>
where
    F: FnMut(&'a str) -> T,
{
    lines.map(tfn).collect()
}

/// Parse the next line of a line iterator with a given transform
fn parse_buf_line<'a, T, F>(mut lines: impl Iterator<Item = &'a str>, mut tfn: F) -> T
where
    F: FnMut(&'a str) -> T,
{
    tfn(lines.next().expect("No line in input"))
}
