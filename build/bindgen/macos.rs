extern crate bindgen;

use std::env;
use std::error::Error;
use std::path::PathBuf;

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
    generate_meminfo_bindings()
}

fn generate_meminfo_bindings() -> Result<(), Box<dyn Error>> {
    let relative_path = "ffi/macos";
    let header_name = "meminfo.h";
    let bindings_name = "meminfo.rs";

    let cargo_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    let bindings = bindgen::Builder::default()
        .headers([cargo_root
            .join("src")
            .join(relative_path)
            .join(header_name)
            .to_string_lossy()])
        .generate()
        .expect(&format!(
            "Unable to generate bindings for src/{}",
            relative_path
        ));

    let out_path = PathBuf::from(env::var("OUT_DIR")?).join(relative_path);
    std::fs::create_dir_all(&out_path)?;
    bindings
        .write_to_file(out_path.join(bindings_name))
        .expect(&format!("Couldn't write bindings: {}", relative_path));

    Ok(())
}
