extern crate cmake;
use cmake::Config;

fn main() {
    let dst = Config::new("vendor/libdecklink_c").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=decklink_c");
    println!("cargo:rustc-link-lib=stdc++");
}
