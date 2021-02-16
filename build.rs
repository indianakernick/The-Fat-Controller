fn main() {
    // https://stackoverflow.com/a/45537237/4093378
    if std::env::var("DISPLAY").is_ok() {
        println!("cargo:rustc-cfg=x11");
    }

    // There's an alternative that's more robust but also more complicated.
    // https://unix.stackexchange.com/a/325972/356153
}
