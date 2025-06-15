mod generator;
mod globals;
mod models;
mod rustfmt;

fn main() {
    let instruction_set_path = globals::BUILD_DIR.join("instructions.xml");
    println!("cargo::rerun-if-changed={}", instruction_set_path.to_str().unwrap());

    let instruction_set = models::read_instruction_set(&instruction_set_path);
    generator::generate(&instruction_set);
}
