use std::{error::Error, fs::File};

use memmap2::Mmap;

/// Memory mapped input
pub struct Input {
    mmap: Mmap,
}

impl Input {
    /// Opens and memory maps an input file for a given day
    pub fn new(day: usize) -> Result<Self, Box<dyn Error>> {
        let file = Self::open(&format!("day{day:02}.txt"))?;

        Self::new_from_file(file)
    }

    /// Returns the memory mapped file as a lines iterator
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.mmap
            .as_ref()
            .split(|&b| b == b'\n')
            .map(|line| line.strip_suffix(b"\r").unwrap_or(line))
            .filter(|line| !line.is_empty())
            .map(|line| {
                #[cfg(debug_assertions)]
                let str = std::str::from_utf8(line).expect("Line is not valid UTF-8");

                #[cfg(not(debug_assertions))]
                let str = unsafe { std::str::from_utf8_unchecked(line) };

                str
            })
    }

    /// Returns the memory mapped file as a string slice
    pub fn as_str(&self) -> Result<&str, Box<dyn Error>> {
        #[cfg(debug_assertions)]
        let str = std::str::from_utf8(self.mmap.as_ref())?;

        #[cfg(not(debug_assertions))]
        let str = unsafe { std::str::from_utf8_unchecked(self.mmap.as_ref()) };

        Ok(str)
    }

    /// Opens the input file
    fn open(file: &str) -> std::io::Result<File> {
        match File::open(format!("inputs/{file}")) {
            Err(_) => File::open(format!("../inputs/{file}")),
            f => f,
        }
    }

    /// Memory maps an open file
    fn new_from_file(file: File) -> Result<Self, Box<dyn Error>> {
        let mmap = unsafe { Mmap::map(&file)? };

        Ok(Self { mmap })
    }
}
