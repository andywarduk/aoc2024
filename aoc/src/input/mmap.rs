use std::{error::Error, fs::File};

use memmap2::Mmap;

/// Memory mapped input
pub struct Input {
    mmap: Mmap,
}

impl Input {
    pub fn new(day: usize) -> Result<Self, Box<dyn Error>> {
        // Open the file
        let file = Self::open(&format!("day{day:02}.txt"))?;

        Self::new_from_file(file)
    }

    pub fn new_example(day: usize, example: usize) -> Result<Self, Box<dyn Error>> {
        // Open the file
        let file = Self::open(&format!("example{day:02}-{example}.txt"))?;

        Self::new_from_file(file)
    }

    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.mmap
            .as_ref()
            .split(|&b| b == b'\n')
            .map(|line| line.strip_suffix(b"\r").unwrap_or(line))
            .filter(|line| !line.is_empty())
            .map(|line| std::str::from_utf8(line).expect("Line is not valid UTF-8"))
    }

    pub fn as_str(&self) -> Result<&str, Box<dyn Error>> {
        Ok(std::str::from_utf8(self.mmap.as_ref())?)
    }

    fn open(file: &str) -> std::io::Result<File> {
        match File::open(format!("inputs/{file}")) {
            Err(_) => File::open(format!("../inputs/{file}")),
            f => f,
        }
    }

    fn new_from_file(file: File) -> Result<Self, Box<dyn Error>> {
        // Memory map it
        let mmap = unsafe { Mmap::map(&file)? };

        Ok(Self { mmap })
    }
}
