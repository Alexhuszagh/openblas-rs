pub fn main() {
    let mut config = cmake::Config::new("OpenBLAS-0.3.21");
    config.define("BUILD_TESTING", "OFF");
    if cfg!(not(feature = "lapack")) {
        config.define("BUILD_WITHOUT_LAPACK", "ON");
    }
    let dst = config.build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=openblas");
}
