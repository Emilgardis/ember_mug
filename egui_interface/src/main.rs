#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    use tracing_subscriber::EnvFilter;
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_file(true)
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();
    color_eyre::install().unwrap();

    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    let stop = ember_mug_egui::runtime::start();
    eframe::run_native(
        "Ember Mug",
        native_options,
        Box::new(|cc| Box::new(ember_mug_egui::EmberMugApp::new(cc))),
    );
    stop();
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(ember_mug_egui::EmberMugApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
