//! # `RsGames`
//!
//! A 2D game engine built on top of `wgpu`, `winit`, and `kira`.
//!
//! ## Quick Start
//! ```rust,ignore
//! use rustgames::prelude::*;
//! use rustgames::core::app;
//!
//! struct MyGame;
//!
//! impl Game for MyGame {
//!     fn init(&mut self, engine: &mut Engine) {}
//!     fn update(&mut self, engine: &mut Engine) {}
//!     fn handle_update(&mut self, engine: &mut Engine) {}
//! }
//!
//! fn main() {
//!     app::run(WindowConfig::default(), Box::new(MyGame)).unwrap();
//! }
//! ```
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::complexity,
    clippy::perf
)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::type_complexity)]
pub mod audio;
pub mod core;
pub mod error;
pub mod graphics;
pub mod prelude;
pub mod text;
pub mod translation;
pub mod utils;
pub mod window;
