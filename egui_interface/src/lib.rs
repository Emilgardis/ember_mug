#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod repaint_on_drop;
pub mod runtime;
pub use app::EmberMugApp;
pub mod events;
