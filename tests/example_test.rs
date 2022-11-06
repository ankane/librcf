use std::io::{self, Write};
use std::process::Command;

#[test]
fn test_example() {
    let status = Command::new("gcc")
        .args([
            "-Wall",
            "-Wextra",
            "-Werror",
            "-o",
            "example/main",
            "example/main.c",
            "-Iinclude",
            "-Ltarget/debug",
            "-Wl,-rpath,target/debug",
            "-lrcf",
        ])
        .status()
        .expect("failed to execute process");
    assert!(status.success());

    let output = Command::new("example/main")
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());
}
