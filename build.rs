fn main() {
    // On BSD systems, inotify is provided by a userspace library (libinotify)
    // rather than the kernel. Link against it when building for those targets.
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if matches!(target_os.as_str(), "freebsd" | "netbsd" | "openbsd") {
        // Use pkg-config to find the library search path if available,
        // otherwise fall back to /usr/local/lib (the conventional BSD prefix).
        let lib_dir = std::process::Command::new("pkg-config")
            .args(["--variable=libdir", "libinotify"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_owned())
            .unwrap_or_else(|| "/usr/local/lib".to_owned());

        println!("cargo:rustc-link-search=native={lib_dir}");
        println!("cargo:rustc-link-lib=inotify");
    }
}
