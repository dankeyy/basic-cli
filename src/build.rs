fn main() {
    #[cfg(not(windows))]
    println!("cargo:rustc-link-lib=dylib=app");

    #[cfg(windows)]
    println!("cargo:rustc-link-lib=dylib=libapp");

    println!("cargo:rustc-link-search=.");

    #[cfg(target_os = "linux")]
    {
        if std::env::var_os("CARGO_BUILD_TARGET").is_none() {
            let out = std::process::Command::new("cargo")
                .arg("build")
                .args(["--target-dir", "target_musl"])
                .env("RUST_FLAGS", "-L . -l dylib=app")
                .env(
                    "CARGO_BUILD_TARGET",
                    format!("{}-unknown-linux-musl", std::env::consts::ARCH)
                ).output();

            eprintln!("{:?}", out);
        }
    }
}
