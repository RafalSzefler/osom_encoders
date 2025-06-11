use std::io::Write;
use std::process::Stdio;
use std::sync::LazyLock;
use std::{env, process::Command};
use std::path::Path;
use xsd_parser::{generate, Config, config::{Schema, InterpreterFlags, OptimizerFlags, GeneratorFlags, Renderer}};


static RUSTFMT_COMMAND: LazyLock<String> = LazyLock::new(|| {
    match env::var("RUSTFMT") {
        Ok(var) => var,
        Err(_) => "rustfmt".to_string(),
    }
});

fn rustfmt_command() -> Command {
    Command::new(RUSTFMT_COMMAND.as_str())
}

fn check_rustfmt() -> bool {
    let result = rustfmt_command().arg("--help").output();
    result.is_ok()
}

fn main() {
    if !check_rustfmt() {
        panic!("rustfmt not found");
    }

    let workspace_dir = env::var("CARGO_WORKSPACE_DIR").unwrap();
    let workspace_dir = Path::new(&workspace_dir);
    let x86_resources_dir = workspace_dir.join("resources").join("x86");
    let x86_xsd = x86_resources_dir.join("x86reference.xsd");

    println!("cargo::rerun-if-changed={}", x86_xsd.to_str().unwrap());

    let mut config = Config::default();
    config.parser.schemas = vec![Schema::File(x86_xsd)];
    config.interpreter.flags = InterpreterFlags::all();
    config.optimizer.flags = OptimizerFlags::all();
    config.generator.flags = GeneratorFlags::all();

    let config = config.with_renderers([
        Renderer::Types,
        Renderer::Defaults,
        Renderer::NamespaceConstants,
        Renderer::QuickXmlDeserialize {
            boxed_deserializer: false,
        },
    ]);

    let code = generate(config).unwrap();
    let code = "// Auto-generated from x86reference.xsd.\n#![allow(unused)]\n#![allow(dead_code)]\n#![allow(clippy::all)]\n".to_string() + &code.to_string();

    let current_project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let current_project_dir = Path::new(&current_project_dir);
    let output_file = current_project_dir.join("src").join("models.rs");
    println!("cargo::rerun-if-changed={}", output_file.to_str().unwrap());

    let mut cmd = rustfmt_command();
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

    let mut child = cmd.spawn().unwrap();
    let mut child_stdin = child.stdin.take().unwrap();
    let mut child_stdout = child.stdout.take().unwrap();

    let stdin_handle = ::std::thread::spawn(move || {
        child_stdin.write_all(code.as_bytes())
    });

    let mut output = vec![];
    std::io::copy(&mut child_stdout, &mut output).unwrap();

    stdin_handle.join().unwrap().unwrap();

    child.wait().unwrap();

    let code = match String::from_utf8(output) {
        Ok(code) => code,
        Err(_) => panic!("rustfmt output is not valid UTF-8"),
    };

    std::fs::write(output_file.clone(), code).unwrap();
}