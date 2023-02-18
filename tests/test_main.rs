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
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["--input".to_string(), "tests/data/empty.fth".to_string()])
        .assert()
        .success();
}