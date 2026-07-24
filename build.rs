use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(&manifest_dir).join("target"));
    let profile = env::var("PROFILE").unwrap();
    let output_dir = target_dir.join(profile);

    let template_src = PathBuf::from(&manifest_dir).join("template.txt");
    let template_dst = output_dir.join("template.txt");

    fs::copy(&template_src, &template_dst).expect("Failed to copy template.txt to output directory");
    println!("cargo:rerun-if-changed=template.txt");
}
