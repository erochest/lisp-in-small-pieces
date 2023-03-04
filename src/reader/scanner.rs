use std::{
    io::{read_to_string, Read},
    ops::Range,
    rc::Rc,
};

use crate::error::Result;

#[derive(Debug, Clone)]
pub struct Scanner {
    i: usize,
    buffer: Rc<Vec<char>>,
}

#[derive(Debug, Clone)]
pub struct ScanToken {
    range: Range<usize>,
    buffer: Rc<Vec<char>>,
}

impl Scanner {
    pub fn from_str<S: AsRef<str>>(buffer: S) -> Self {
        let buffer = Rc::new(buffer.as_ref().chars().collect());
        Scanner { i: 0, buffer }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.buffer.get(self.i) {
            if !c.is_whitespace() {
                return;
            }
            self.i += 1;
        }
    }

    fn skip_to_end(&mut self) {
        while let Some(c) = self.buffer.get(self.i) {
            if c.is_whitespace() || *c == ')' {
                break;
            }
            self.i += 1;
        }
    }

    fn skip_string(&mut self) {
        assert!(self.buffer.get(self.i) == Some(&'"'));

        self.i += 1;
        while let Some(c) = self.buffer.get(self.i) {
            if *c == '"' {
                self.i += 1;
                return;
            }
            if *c == '\\' {
                self.i += 2;
            } else {
                self.i += 1;
            }
        }
    }

    fn at_end(&self) -> bool {
        self.i >= self.buffer.len()
    }
}

impl Iterator for Scanner {
    type Item = ScanToken;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        if self.at_end() {
            return None;
        }

        let start = self.i;
        if let Some(peek) = self.buffer.get(self.i) {
            if *peek == '\'' {
                self.i += 1;
            } else if *peek == '(' {
                self.i += 1;
            } else if *peek == ')' {
                self.i += 1;
            } else if *peek == '#' && self.buffer.get(self.i+1) == Some(&'\'') {
                self.i += 2;
            } else if *peek == '"' {
                self.skip_string();
            } else {
                self.skip_to_end();
            }
        }

        Some(ScanToken {
            range: start..self.i,
            buffer: Rc::clone(&self.buffer),
        })
    }
}

impl ScanToken {
    pub fn get_string(&self) -> Option<String> {
        self.buffer
            .get(self.range.clone())
            .map(|cs| cs.iter().collect())
    }
}

pub fn scan<R: Read>(reader: R) -> Result<Scanner> {
    let input = read_to_string(reader)?;
    Ok(Scanner::from_str(input))
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use pretty_assertions::assert_eq;

    use crate::reader::scanner::Scanner;

    use super::{scan, ScanToken};

    #[test]
    fn test_get_string() {
        let token = ScanToken {
            range: 4..10,
            buffer: Rc::new("    foobar".chars().collect()),
        };
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

    macro_rules! test_scan {
        ($name:ident, $input:expr, $n:expr, $( $token:expr ),*) => {
            #[test]
            fn $name() {
                let mut input = $input.as_bytes();

                let actual = scan(&mut input);
                assert!(actual.is_ok());

                let scanner: Scanner = actual.unwrap();
                let tokens = scanner
                    .filter_map(|t| t.get_string())
                    .collect::<Vec<_>>();
                assert_eq!(tokens.len(), $n);

                assert_eq!(tokens, vec![$( $token, )*]);
            }
        };
    }

    test_scan!(test_skips_initial_whitespace, "    foobar", 1, "foobar".to_string());
    test_scan!(test_scans_strings, " \"this is a string\" ", 1, "\"this is a string\"".to_string());
    test_scan!(test_scans_empty_strings, " \"\" ", 1, "\"\"".to_string());
    test_scan!(test_scans_strings_with_escapes, " \"this string \\\"contains\\\" a string\" ", 1, "\"this string \\\"contains\\\" a string\"".to_string());

    test_scan!(test_list_start, " ( ", 1, "(".to_string());
    test_scan!(test_list_end, " ) ", 1, ")".to_string());
    test_scan!(test_empty_list, " () ", 2, "(".to_string(), ")".to_string());
    test_scan!(test_integer_list_end, " 42) ", 2, "42".to_string(), ")".to_string());
    test_scan!(test_symbol_list_end_list_end, " foo-bar))", 3, "foo-bar".to_string(), ")".to_string(), ")".to_string());
    test_scan!(test_list_symbol, "(foo-bar)", 3, "(".to_string(), "foo-bar".to_string(), ")".to_string());
    test_scan!(test_quote, "'foo-bar", 2, "'", "foo-bar");
    test_scan!(test_sharp_quote, "#'foo-bar", 2, "#'", "foo-bar");

}
