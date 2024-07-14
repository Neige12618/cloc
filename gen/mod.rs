use std::path::Path;

mod gen_code;
mod language;

pub fn generate_code_from_json(path: &str) {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("language_type.rs");

    let data = language::parse_json(path).expect("Failed to parse JSON");
    let code = gen_code::generate_code(data);

    std::fs::write(dest_path, code).expect("Unable to write file");
}
