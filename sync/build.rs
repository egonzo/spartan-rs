fn main() {
    let v = std::env::var("VERSION").unwrap_or_else(|_| "None".to_string());

    built::write_built_file().expect("Failed to acquire build-time information");

    println!("cargo:rustc-env=VERSION={}", v);
}
