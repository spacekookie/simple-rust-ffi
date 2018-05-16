extern crate cmake;

fn main() {
    let dst = cmake::build("libgreet");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=greet");
}