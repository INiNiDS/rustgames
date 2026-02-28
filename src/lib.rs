#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::complexity,
    clippy::perf
)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::type_complexity)]
pub mod audio;
pub mod core;
pub mod graphics;
pub mod translation;
pub mod prelude;
pub mod text;
pub mod window;
