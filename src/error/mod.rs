//! # Errors module `rustgames`
//!
//! Types of errors:
//! Every error includes:
//! - `what` — what going wrong
//! - `why` — reason
//! - `fix` — how to fix

mod audio_error;
mod graphics_error;
mod text_error;

pub use audio_error::AudioError;
pub use graphics_error::GraphicsError;
pub use text_error::TextError;

use std::fmt;
use thiserror::Error;

/// Main error type for `RsGames`, encompassing all subsystems (audio, graphics, text, IO).
///
/// # EXAMPLES
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
    /// Short error code, e.g. `"A001"`.
    pub code: &'a str,
    /// One-line human-readable title of the error.
    pub title: &'a str,
    /// Source location where the error originates, e.g. `"AudioSystem::new()"`.
    pub location: &'a str,
    /// Description of what went wrong.
    pub what: &'a str,
    /// Root cause explanation.
    pub why: &'a str,
    /// Suggested remediation steps.
    pub fix: &'a str,
    /// Optional extra context, e.g. the underlying OS error message.
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

/// Implemented by error types that can render a full [`Diagnostic`] string.
pub trait IntoDiagnostic {
    /// Formats the error as a rich, color-coded diagnostic string.
    fn diagnostic(&self) -> String;
}

