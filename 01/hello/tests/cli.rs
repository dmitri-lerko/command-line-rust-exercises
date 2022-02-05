// use std::process::Command;
use assert_cmd::Command;

#[test]
fn runs() {
  let mut cmd = Command::cargo_bin("hello").unwrap();
  cmd.assert().success().stdout("Hello, world!\n");
}

fn true_ok() {
  let mut cmd = Command::cargo_bin("true").unwrap();
  cmd.assert().success();
}

fn false_not_ok() {
  let mut cmd = Command::cargo_bin("false").unwrap();
  cmd.assert().failure();
}