
use assert_cmd::Command;

#[test]
fn test_main() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let output = cmd.unwrap();
    assert!(output.status.success());
}

#[test]
fn test_empty_file() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let cmd = cmd.args(&["--input".to_string(), "tests/data/empty.fth".to_string()]);
    let output = cmd.unwrap();
    assert!(output.status.success());
}

