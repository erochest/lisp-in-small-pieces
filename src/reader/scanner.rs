use std::{io::{Read, read_to_string}, ops::Range, marker::PhantomData, rc::Rc, slice::SliceIndex};

use crate::error::Result;

#[derive(Debug, Clone)]
struct Scanner {
    i: usize,
    buffer: Rc<Vec<char>>,
}

#[derive(Debug, Clone)]
struct ScanToken {
    range: Range<usize>,
    buffer: Rc<Vec<char>>,
}

impl Iterator for Scanner {
    type Item = ScanToken;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: refactor this
        while let Some(c) = self.buffer.get(self.i) {
            if !c.is_whitespace() {
                break;
            }
            self.i += 1;
        }

        if self.i >= self.buffer.len() {
            return None
        }

        let start = self.i;
        if let Some(peek) = self.buffer.get(self.i) {
            if *peek == '"' {
                self.i += 1;
                while let Some(c) = self.buffer.get(self.i) {
                    if *c == '"' {
                        self.i += 1;
                        break;
                    }
                    if *c == '\\' {
                        self.i += 2;
                    } else {
                        self.i += 1;
                    }
                }
            } else {
                while let Some(c) = self.buffer.get(self.i) {
                    if c.is_whitespace() {
                        break;
                    }
                    self.i += 1;
                }
            }
        }

        Some(ScanToken {
            range: start..self.i,
            buffer: Rc::clone(&self.buffer),
        })
    }
}

impl ScanToken {
    fn get_string(&self) -> Option<String> {
        self.buffer.get(self.range.clone()).map(|cs| cs.iter().collect())
    }
}

fn scan<'a, R: Read>(reader: R) -> Result<Scanner> {
    let buffer = read_to_string(reader)?.chars().collect::<Vec<_>>();
    let buffer = Rc::new(buffer);
    Ok(Scanner {
        i: 0,
        buffer,
    })
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use pretty_assertions::assert_eq;

    use crate::reader::scanner::Scanner;

    use super::{scan, ScanToken};

    #[test]
    fn test_get_string() {
        let token = ScanToken { range: 4..10, buffer: Rc::new("    foobar".chars().collect()) };
        assert_eq!(token.get_string(), Some("foobar".to_string()));
    }

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

    #[test]
    fn test_only_whitespace() {
        let mut input = "    ".as_bytes();

        let result = scan(&mut input);
        assert!(result.is_ok());

        let tokens = result.unwrap().collect::<Vec<_>>();
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_multiple_tokens() {
        let mut input = "  one two three  ".as_bytes();

        let result = scan(&mut input);
        assert!(result.is_ok());

        let tokens = result.unwrap().collect::<Vec<_>>();
        assert_eq!(tokens.len(), 3);
    }

    #[test]
    fn test_skips_initial_whitespace() {
        let mut input = "    foobar".as_bytes();

        let result = scan(&mut input);
        assert!(result.is_ok());

        let scanner: Scanner = result.unwrap();
        let tokens = scanner.collect::<Vec<_>>();
        assert_eq!(tokens.len(), 1);

        let token = tokens[0].get_string();
        assert_eq!(token, Some("foobar".to_string()));
    }

    #[test]
    fn test_scans_strings() {
        let mut input = " \"this is a string\" ".as_bytes();

        let result = scan(&mut input);
        assert!(result.is_ok());

        let scanner = result.unwrap();
        let tokens: Vec<_> = scanner.collect();
        assert_eq!(tokens.len(), 1);

        let token = tokens[0].get_string();
        assert_eq!(token, Some("\"this is a string\"".to_string()));
    }

    #[test]
    fn test_scans_empty_strings() {
        let mut input = " \"\" ".as_bytes();

        let result = scan(&mut input);
        assert!(result.is_ok());

        let scanner = result.unwrap();
        let tokens: Vec<_> = scanner.collect();
        assert_eq!(tokens.len(), 1);

        let token = tokens[0].get_string();
        assert_eq!(token, Some("\"\"".to_string()));
    }

    #[test]
    fn test_scans_strings_with_escapes() {
        let mut input = " \"this string \\\"contains\\\" a string\" ".as_bytes();

        let result = scan(&mut input);
        assert!(result.is_ok());

        let scanner = result.unwrap();
        let tokens: Vec<_> = scanner.collect();
        assert_eq!(tokens.len(), 1);

        let token = tokens[0].get_string();
        assert_eq!(token, Some("\"this string \\\"contains\\\" a string\"".to_string()));
    }

}