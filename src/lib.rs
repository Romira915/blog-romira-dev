pub mod app;
pub(crate) mod components;
mod routes;
#[cfg(not(target_arch = "wasm32"))]
pub mod settings;
