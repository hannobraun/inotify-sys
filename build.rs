fn main() {
    // On BSD systems, inotify is historically provided by a userspace library
    // (libinotify) rather than the kernel. Link against it when building for
    // those targets.
    //
    // FreeBSD 14.4+ ships in-kernel inotify with libc wrappers in libc.so.7;
    // when the `freebsd-native` feature is enabled we skip linking libinotify
    // and let the extern declarations resolve against libc directly.
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let freebsd_native = std::env::var("CARGO_FEATURE_FREEBSD_NATIVE").is_ok();

    let needs_libinotify = match target_os.as_str() {
        "freebsd" => !freebsd_native,
        "netbsd" | "openbsd" => true,
        _ => false,
    };

    if needs_libinotify {
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
