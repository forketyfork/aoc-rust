fn main() {
    println!("cargo::rustc-check-cfg=cfg(ci_environment)");
    if std::env::var("CI").is_ok() {
        println!("cargo:rustc-cfg=ci_environment");
    }
}
