use std::error::Error;
use std::fs::File;
#[cfg(miri)]
use std::io::Read;
use std::io::{BufRead, BufReader, Lines};

#[cfg(not(miri))]
use memmap2::Mmap;

/// Parse an input file by line to a vector with a given transform
pub fn parse_input_vec<T, F>(day: usize, tfn: F) -> Result<Vec<T>, Box<dyn Error>>
where
    F: FnMut(String) -> T,
{
    let input = Input::new(day)?;
    parse_buf_vec(input.lines(), tfn)
}

/// Parse an input file with a single line with a given transform
pub fn parse_input_line<T, F>(day: usize, tfn: F) -> Result<T, Box<dyn Error>>
where
    F: FnMut(String) -> T,
{
    let input = Input::new(day)?;
    parse_buf_line(input.lines(), tfn)
}

/// Reads an input file to a string
pub fn read_input_file(day: usize) -> Result<String, Box<dyn Error>> {
    let input = Input::new(day)?;
    input.string()
}

/// Parse an input string to a vector with a given transform
pub fn parse_test_vec<T, F>(test: &str, tfn: F) -> Result<Vec<T>, Box<dyn Error>>
where
    F: FnMut(String) -> T,
{
    let buf = BufReader::new(test.as_bytes());
    parse_buf_vec(buf.lines(), tfn)
}

/// Parse an test input file to a vector with a given transform
pub fn parse_test_input_vec<T, F>(
    day: usize,
    example: usize,
    tfn: F,
) -> Result<Vec<T>, Box<dyn Error>>
where
    F: FnMut(String) -> T,
{
    let input = Input::new_example(day, example)?;
    parse_buf_vec(input.lines(), tfn)
}

/// Memory mapped input (string under miri)
struct Input {
    #[cfg(not(miri))]
    mmap: Mmap,
    #[cfg(miri)]
    string: String,
}

impl Input {
    fn new(day: usize) -> Result<Self, Box<dyn Error>> {
        // Open the file
        let file = Self::open(&format!("day{day:02}.txt"))?;

        Self::new_from_file(file)
    }

    fn new_example(day: usize, example: usize) -> Result<Self, Box<dyn Error>> {
        // Open the file
        let file = Self::open(&format!("example{day:02}-{example}.txt"))?;

        Self::new_from_file(file)
    }

    fn open(file: &str) -> std::io::Result<File> {
        match File::open(format!("inputs/{file}")) {
            Err(_) => File::open(format!("../inputs/{file}")),
            f => f,
        }
    }

    #[cfg(not(miri))]
    fn new_from_file(file: File) -> Result<Self, Box<dyn Error>> {
        // Memory map it
        let mmap = unsafe { Mmap::map(&file)? };

        Ok(Self { mmap })
    }

    #[cfg(miri)]
    fn new_from_file(mut file: File) -> Result<Self, Box<dyn Error>> {
        // Read to string
        let mut string = String::new();
        file.read_to_string(&mut string)?;

        Ok(Self { string })
    }

    fn lines(&self) -> Lines<BufReader<&[u8]>> {
        #[cfg(not(miri))]
        let buf_reader = BufReader::new(self.mmap.as_ref());

        #[cfg(miri)]
        let buf_reader = BufReader::new(self.string.as_ref());

        buf_reader.lines()
    }

    fn string(&self) -> Result<String, Box<dyn Error>> {
        #[cfg(not(miri))]
        let s = String::from_utf8(self.mmap.as_ref().to_vec())?;
        #[cfg(miri)]
        let s = self.string.clone();

        Ok(s)
    }
}

/// Parse a lines iterator to a vector with a given transform
fn parse_buf_vec<T, F>(lines: Lines<BufReader<&[u8]>>, mut tfn: F) -> Result<Vec<T>, Box<dyn Error>>
where
    F: FnMut(String) -> T,
{
    let mut result = Vec::new();

    for l in lines {
        let line = l?;

        result.push(tfn(line));
    }

    Ok(result)
}

/// Parse the next line of a line iterator with a given transform
fn parse_buf_line<T, F>(mut lines: Lines<BufReader<&[u8]>>, mut tfn: F) -> Result<T, Box<dyn Error>>
where
    F: FnMut(String) -> T,
{
    let line = lines.next().expect("No line found in input")?;

    Ok(tfn(line))
}
