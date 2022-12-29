#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod runtime;
pub use app::EmberMugApp;
pub mod events;
