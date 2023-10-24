use assert_cmd::prelude::*;
use pretty_assertions::*;

use std::{path::Path, process::Command};

#[test]
fn test_main() {
    // Requires an input parameter.
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["--help"])
        .assert()
        .success();
}

fn test_parse_file<P: AsRef<Path>, S: AsRef<str>>(input: P, expected: S) {
    let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[
            "parse".to_string(),
            "--input".to_string(),
            input.as_ref().to_string_lossy().to_string(),
        ])
        .assert()
        .success();
    let output = assert.get_output();
    let actual = String::from_utf8(output.stdout.clone())
        .unwrap()
        .trim()
        .to_string();
    assert_str_eq!(actual, expected.as_ref().to_string());
}

#[test]
fn test_empty_file() {
    test_parse_file("tests/data/empty.lisp", "");
}

#[test]
fn test_parse_integer() {
    test_parse_file(
        "tests/data/integer.lisp",
        "{\"type\":\"Integer\",\"value\":42}",
    );
}

#[test]
fn test_parse_symbol() {
    test_parse_file(
        "tests/data/symbol.lisp",
        "{\"type\":\"Symbol\",\"value\":\"foobar\"}",
    );
}

#[test]
fn test_parse_float() {
    test_parse_file(
        "tests/data/float.lisp",
        "{\"type\":\"Float\",\"value\":3.14159}",
    );
}
