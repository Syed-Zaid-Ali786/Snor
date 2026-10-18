fn main() {
    #[cfg(windows)]
    {
        let manifest = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
        let assets = if manifest.join("assets").is_dir() {
            manifest.join("assets")
        } else {
            manifest.join("../assets")
        };
        let icon = assets.join("snor.ico");
        println!("cargo:rerun-if-changed={}", icon.display());
        winresource::WindowsResource::new()
            .set_icon(icon.to_str().expect("icon path must be UTF-8"))
            .set("ProductName", "Snor")
            .set("FileDescription", "Snor")
            .compile()
            .expect("embed Snor's Windows icon");
    }
}
