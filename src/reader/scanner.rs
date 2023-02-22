use std::{io::{Read, read_to_string}, ops::Range};

use crate::error::Result;

struct ScanRanges {
    i: usize,
    buffer: String,
}

impl Iterator for ScanRanges {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.buffer.len() {
            self.i = self.buffer.len();
            Some(0..self.buffer.len())
        } else {
            None
        }
    }
}

fn scan<R: Read>(reader: R) -> Result<impl Iterator<Item = Range<usize>>> {
    let buffer = read_to_string(reader)?;
    Ok(ScanRanges {
        i: 0,
        buffer,
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::scan;

    #[test]
    fn test_scans_empty() {
        let mut input = "".as_bytes();

        let result = scan(&mut input);
        assert!(result.is_ok());

        let ranges = result.unwrap().collect::<Vec<_>>();
        assert!(ranges.is_empty());
    }

    #[test]
    fn test_scans_single_word() {
        let mut input = "foobar".as_bytes();

        let result = scan(&mut input);
        assert!(result.is_ok());

        let ranges = result.unwrap().collect::<Vec<_>>();
        assert_eq!(ranges.len(), 1);
    }
}