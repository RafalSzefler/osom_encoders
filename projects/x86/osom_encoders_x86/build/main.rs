mod generator;
mod rustfmt;

use std::env;
use std::path::Path;
use osom_encoders_x86_ref_models::read_x86_xml;



fn check_rustfmt() -> bool {
    let result = rustfmt::rustfmt_command().arg("--help").output();
    result.is_ok()
}

fn main() {
    if !check_rustfmt() {
        panic!("rustfmt not found");
    }

    let workspace_dir = env::var("CARGO_WORKSPACE_DIR").unwrap();
    let workspace_dir = Path::new(&workspace_dir);
    let x86_resources_dir = workspace_dir.join("resources").join("x86");
    let x86_xml = x86_resources_dir.join("x86reference.xml");

    println!("cargo::rerun-if-changed={}", x86_xml.to_str().unwrap());

    let x86_reference = read_x86_xml(&x86_xml).unwrap();
    generator::generate_code(&x86_reference).unwrap();
}
