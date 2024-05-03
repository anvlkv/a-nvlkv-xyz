#[macro_use]
extern crate rust_i18n;

mod app;

#[cfg(feature = "ssr")]
mod server;

#[cfg(any(feature = "csr", feature = "hydrate"))]
mod bindings;

use cfg_if::cfg_if;

i18n!();

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;

      _ = console_log::init_with_level(log::Level::Debug);
      console_error_panic_hook::set_once();

      log::info!("init client logging");

      leptos::mount_to_body(App);
    }
}
}
