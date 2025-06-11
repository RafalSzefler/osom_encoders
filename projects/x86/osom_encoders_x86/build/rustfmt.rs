use std::{env, process::Command, sync::LazyLock};


static RUSTFMT_COMMAND: LazyLock<String> = LazyLock::new(|| {
    match env::var("RUSTFMT") {
        Ok(var) => var,
        Err(_) => "rustfmt".to_string(),
    }
});

pub fn rustfmt_command() -> Command {
    Command::new(RUSTFMT_COMMAND.as_str())
}
