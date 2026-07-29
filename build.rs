fn main() {
    // Rebuild when the embedded default template changes.
    println!("cargo:rerun-if-changed=template.txt");
}
