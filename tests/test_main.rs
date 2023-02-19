use assert_cmd::prelude::*;

use std::process::Command;

#[test]
fn test_main() {
    // Requires an input parameter.
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["--help"])
        .assert()
        .success();
}

#[test]
fn test_empty_file() {
    let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[
            "parse".to_string(),
            "--input".to_string(),
            "tests/data/empty.fth".to_string(),
        ])
        .assert()
        .success();
    let output = assert.get_output();
    assert!(String::from_utf8(output.stdout.clone()).unwrap() == "".to_string());
}

#[test]
fn test_parse_integer() {
    let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&[
            "parse".to_string(),
            "--input".to_string(),
            "tests/data/integer.fth".to_string(),
        ])
        .assert()
        .success();
    let output = assert.get_output();
    let actual = String::from_utf8(output.stdout.clone())
        .unwrap()
        .trim()
        .to_string();
    let expected = "{\"type\":\"Integer\",\"value\":42}".to_string();
    assert!(actual == expected);
}
