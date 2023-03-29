use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exists() -> Result<()> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/d/e");
    cmd.assert().failure().stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<()> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}
