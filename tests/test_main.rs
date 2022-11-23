
use assert_cmd::Command;

#[test]
fn test_main() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let output = cmd.unwrap();
    assert!(output.status.success());
}