use std::{env, path::PathBuf};

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let mut storage_template = std::fs::read_to_string("storage.ld.in").unwrap();
    storage_template = storage_template.replace("${ALIGNMENT}", "0x1000");
    storage_template = storage_template.replace("${SIZE}", "0x2000");

    std::fs::write(out.join("storage.x"), &storage_template).unwrap();

    println!("cargo:rerun-if-changed=storage.ld.in");
    println!("cargo:rustc-link-search={}", out.display());
}
