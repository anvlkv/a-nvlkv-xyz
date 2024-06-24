#[macro_use]
extern crate rust_i18n;

i18n!();

#[cfg(feature = "ssr")]
fn main() {
    // Unused; required for cargo-leptos to build the front as it also wants to build a bin
}
