use std::fs::read_to_string;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], path: &str) -> TestResult {
    let expected = read_to_string(path)?;
    let mut cmd = Command::cargo_bin("echors")?;
    cmd.args(args).assert().success().stdout(expected);

    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echors")?;
    cmd.arg("hello world").assert().success();

    Ok(())
}

#[test]
fn dies_no_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("echors")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello there", "-n"], "tests/expected/hello2.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
