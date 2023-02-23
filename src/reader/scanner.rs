use std::{io::{Read, read_to_string}, ops::Range};

use crate::error::Result;

struct ScanRanges {
    i: usize,
    buffer: Vec<char>,
}

impl Iterator for ScanRanges {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {

        while self.buffer.get(self.i).map_or(false, |c| c.is_whitespace()) {
            self.i += 1;
        }

        if self.i >= self.buffer.len() {
            return None
        }

        let start = self.i;
        self.i = self.buffer.len();

        Some(start..self.buffer.len())
    }
}

fn scan<R: Read>(reader: R) -> Result<ScanRanges> {
    let buffer = read_to_string(reader)?.chars().collect::<Vec<_>>();
    Ok(ScanRanges {
        i: 0,
        buffer,
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::reader::scanner::ScanRanges;

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

    // TODO: only whitespace

    #[test]
    fn test_skips_initial_whitespace() {
        let mut input = "    foobar".as_bytes();

        let result = scan(&mut input);
        assert!(result.is_ok());

        let scanner: ScanRanges = result.unwrap();
        let buffer = scanner.buffer.clone();
        let ranges = scanner.collect::<Vec<_>>();
        assert_eq!(ranges.len(), 1);

        let token = buffer.get(ranges[0].clone()).map(|cs| cs.iter().collect::<String>());
        assert_eq!(token, Some("foobar".to_string()));
    }
}