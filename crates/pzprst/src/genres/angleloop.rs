//! Angle Loop uses the Kouchoku cross and segment formats.
use super::{g_kouchoku::Puzzle, Genre};
pub(crate) static GENRE: &dyn Genre = &Puzzle("angleloop");
