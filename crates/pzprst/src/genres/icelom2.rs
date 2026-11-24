//! Icelom2 is the skip-white Icelom variant.
use super::{g_icelom::Icelom, Genre};
pub(crate) static GENRE: &dyn Genre = &Icelom("icelom2");
