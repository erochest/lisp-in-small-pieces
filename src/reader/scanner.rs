use std::{io::Read, ops::Range};

struct ScanRanges {}

impl Iterator for ScanRanges {
    type Item = Range<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn scan<R: Read>(reader: &mut R) -> impl Iterator<Item = Range<isize>> {
    ScanRanges {}
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::scan;

    #[test]
    fn test_scans_empty() {
        let mut input = "".as_bytes();
        let ranges = scan(&mut input).collect::<Vec<_>>();
        assert!(ranges.is_empty());
    }
}