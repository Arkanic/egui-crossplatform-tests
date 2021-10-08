#![warn(clippy::all, rust_2018_idioms)]

// Local executable entrypoint

#[cfg(not(target_arch = "wasm32"))]
mod app;
#[cfg(not(target_arch = "wasm32"))]
pub use app::App;


#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = App::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}