use assert_cmd::{Command, output};
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_basic_html() {
    let mut cmd = Command::cargo_bin("skp").unwrap();

    let result = cmd.write_stdin("<preserved>suli kamapa. onasu kinlu ponapa.</preserved>");

    let output = result.output();

    insta::assert_debug_snapshot!(output);
}

#[test]
fn test_newlines() {
    let mut cmd = Command::cargo_bin("skp").unwrap();

    let result = cmd.write_stdin(
        "<preserved>\n\n\nsuli kamapa. onasu kinlu ponapa.\nsuli kamapa.\n\nlinja sinsu.</preserved>",
    );

    let output = result.output();

    insta::assert_debug_snapshot!(output);
}
