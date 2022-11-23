#[allow(unused_imports)]
use anyhow::{Ok, Result};
#[allow(unused_imports)]
use assert_cmd::prelude::*;
#[allow(unused_imports)]
use assert_fs::prelude::*;
#[allow(unused_imports)]
use predicates::prelude::*;
#[allow(unused_imports)]
use std::process::Command;

#[cfg(not(miri))]
#[test]
fn file_does_not_exist() -> Result<()> {
    let mut cmd = Command::cargo_bin("cli")?;

    cmd.arg("foobar").arg("test/file/does/not/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[cfg(not(miri))]
#[test]
fn find_content_in_file() -> Result<()> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str(
        "\
A test
Actual content
More content
Another test
",
    )?;

    let mut cmd = Command::cargo_bin("cli")?;
    cmd.arg("test").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains(
        "\
A test
Another test",
    ));

    Ok(())
}
