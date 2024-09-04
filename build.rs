fn main() {
    // Once the MSRV is 1.74, we can replace this with a lint in Cargo.toml instead.
    println!("cargo:rustc-check-cfg=cfg(doc_cfg)");
}
