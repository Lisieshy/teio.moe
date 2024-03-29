use cfg_if::cfg_if;
pub mod app;
pub mod counters;
pub mod footer;
pub mod gallery;

// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use leptos::*;
        use wasm_bindgen::prelude::wasm_bindgen;
        use crate::app::*;

        #[wasm_bindgen]
        pub fn hydrate() {
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            mount_to_body(|cx| {
                view! { cx,  <App/> }
            });
        }
    }
}