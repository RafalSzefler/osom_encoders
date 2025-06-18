use std::{
    io::{Write, copy},
    process::{Command, Stdio},
    sync::LazyLock,
};

use crate::globals::WORKSPACE_DIR;

static RUSTFMT_COMMAND: LazyLock<&str> = LazyLock::new(|| {
    const CMD_TEXT: &str = "rustfmt";
    let mut cmd = Command::new(CMD_TEXT);
    cmd.args(["--version"]);
    let mut child = cmd.spawn().unwrap();
    let exit_status = child.wait().unwrap();
    if !exit_status.success() {
        println!("cargo::warning=Couldn't execute rustfmt command. Skipping code prettification.");
        return "";
    }

    CMD_TEXT
});

pub fn prettify(text_code: String) -> String {
    if RUSTFMT_COMMAND.is_empty() {
        return text_code;
    }

    let mut cmd = Command::new(*RUSTFMT_COMMAND);

    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

    let config_path = WORKSPACE_DIR.join("rustfmt.toml");
    if config_path.exists() {
        cmd.args(["--config-path", config_path.to_str().unwrap()]);
    }

    let mut child = cmd.spawn().unwrap();
    let mut child_stdin = child.stdin.take().unwrap();
    let mut child_stdout = child.stdout.take().unwrap();

    let stdin_handle = ::std::thread::spawn(move || {
        let _ = child_stdin.write_all(text_code.as_bytes());
    });

    let mut output = vec![];
    copy(&mut child_stdout, &mut output).unwrap();
    let _ = child.wait().unwrap();
    stdin_handle.join().unwrap();

    String::from_utf8(output).unwrap()
}
