mod bindgen;

fn main() {
    bindgen::generate_bindings().unwrap();
}
