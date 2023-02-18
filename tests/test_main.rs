use assert_cmd::prelude::*;

use std::process::Command;

#[test]
fn test_main() {
    // Requires an input parameter.
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn test_empty_file() {
    let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["--input".to_string(), "tests/data/empty.fth".to_string()])
        .assert()
        .success();
    let output = assert.get_output();
    assert!(String::from_utf8(output.stdout.clone()).unwrap() == "".to_string());
}