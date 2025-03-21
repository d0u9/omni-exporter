extern crate bindgen;

use std::env;
use std::error::Error;
use std::path::PathBuf;

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
    let relative_path = "ffi/macos";
    let header_name = "meminfo.h";
    let bindings_name = "meminfo.rs";
    do_generate_bindings(relative_path, header_name, bindings_name)?;

    let relative_path = "ffi/macos";
    let header_name = "cpu.h";
    let bindings_name = "cpu.rs";
    do_generate_bindings(relative_path, header_name, bindings_name)?;

    Ok(())
}

fn do_generate_bindings(
    relative_path: &str,
    header_name: &str,
    bindings_name: &str,
) -> Result<(), Box<dyn Error>> {
    let cargo_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    let bindings = bindgen::Builder::default()
        .headers([cargo_root
            .join("src")
            .join(relative_path)
            .join(header_name)
            .to_string_lossy()])
        .generate()
        .unwrap_or_else(|_| panic!("Unable to generate bindings for src/{}", relative_path));

    let out_path = PathBuf::from(env::var("OUT_DIR")?).join(relative_path);
    std::fs::create_dir_all(&out_path)?;
    bindings
        .write_to_file(out_path.join(bindings_name))
        .unwrap_or_else(|_| panic!("Couldn't write bindings: {}", relative_path));

    Ok(())
}
