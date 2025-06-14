use std::{
    io::{Write, copy},
    process::{Command, Stdio},
};

use crate::globals::WORKSPACE_DIR;

pub fn prettify(text_code: String) -> String {
    let mut cmd = Command::new("rustfmt");
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());
    let config_path = WORKSPACE_DIR.join("rustfmt.toml");
    cmd.args(["--config-path", config_path.to_str().unwrap()]);

    let mut child = cmd.spawn().unwrap();
    let mut child_stdin = child.stdin.take().unwrap();
    let mut child_stdout = child.stdout.take().unwrap();

    let stdin_handle = ::std::thread::spawn(move || {
        let _ = child_stdin.write_all(text_code.as_bytes());
    });

    let mut output = vec![];
    copy(&mut child_stdout, &mut output).unwrap();
    child.wait().unwrap();
    stdin_handle.join().unwrap();

    String::from_utf8(output).unwrap()
}
