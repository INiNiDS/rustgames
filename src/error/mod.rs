//! # Модуль ошибок RsGames
//!
//! Типизированные ошибки с подробными описаниями и советами по исправлению.
//! Формат сообщений вдохновлён `cargo check` — каждая ошибка содержит:
//! - `what`  — что именно пошло не так
//! - `why`   — возможная причина
//! - `fix`   — как исправить (и программное исправление где возможно)

mod audio_error;
mod graphics_error;
mod text_error;

pub use audio_error::AudioError;
pub use graphics_error::GraphicsError;
pub use text_error::TextError;

use std::fmt;
use thiserror::Error;

/// Main error type for RsGames, encompassing all subsystems (audio, graphics, text, IO).
///
/// # Примеры
/// ```rust,ignore
/// match err {
///     GameError::Audio(e)    => eprintln!("{e}"),
///     GameError::Graphics(e) => eprintln!("{e}"),
///     GameError::Text(e)     => eprintln!("{e}"),
///     GameError::Io(e)       => eprintln!("{e}"),
/// }
/// ```
#[derive(Debug, Error)]
pub enum GameError {
    #[error("{0}")]
    Audio(#[from] AudioError),

    #[error("{0}")]
    Graphics(#[from] GraphicsError),

    #[error("{0}")]
    Text(#[from] TextError),

    #[error("{0}")]
    Io(#[from] std::io::Error),
}


///
/// ```text
/// error[E001]: Failed to initialize audio backend
///  --> AudioSystem::new()
///   |
///   = what: the Kira audio manager could not start
///   = why:  no audio device is available or the driver crashed
///   = fix:  check that your system has a working audio device,
///           or call `AudioSystem::new_silent()` to run without audio
///   = note: DefaultBackend error: ...
/// ```
pub struct Diagnostic<'a> {
    pub code: &'a str,
    pub title: &'a str,
    pub location: &'a str,
    pub what: &'a str,
    pub why: &'a str,
    pub fix: &'a str,
    pub note: Option<String>,
}

impl fmt::Display for Diagnostic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\x1b[1;31merror[{code}]\x1b[0m\x1b[1m: {title}\x1b[0m",
            code = self.code, title = self.title)?;
        writeln!(f, " \x1b[1;34m-->\x1b[0m {}", self.location)?;
        writeln!(f, "  \x1b[1;34m|\x1b[0m")?;
        writeln!(f, "  \x1b[1;34m=\x1b[0m \x1b[1mwhat\x1b[0m: {}", self.what)?;
        writeln!(f, "  \x1b[1;34m=\x1b[0m \x1b[1mwhy\x1b[0m:  {}", self.why)?;
        writeln!(f, "  \x1b[1;34m=\x1b[0m \x1b[1mfix\x1b[0m:  {}", self.fix)?;
        if let Some(note) = &self.note {
            writeln!(f, "  \x1b[1;34m=\x1b[0m \x1b[1mnote\x1b[0m: {note}")?;
        }
        Ok(())
    }
}

pub trait IntoDiagnostic {
    fn diagnostic(&self) -> String;
}

