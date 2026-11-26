//! pzprst: pzprjs-compatible static puzzle boards.
//!
//! Decodes and encodes pzpr URLs, FileIO text and pzpr-trace states exactly like
//! the pinned pzprjs ([`PZPRJS_COMMIT`]), and adds board symmetries, canonical
//! URLs and board transformations. See `docs/SPEC.md`.
//!
//! ```
//! use pzprst::{canonical_url, Board, Options, Transform};
//! assert_eq!(canonical_url("slither/3/2/h12g3", &Options::default()).unwrap(), "slither/3/2/h17d");
//! let b = Board::from_url("slither/3/2/h17d", &Options::default()).unwrap();
//! assert_eq!(b.transform(Transform::FlipX).to_url().unwrap(), "slither/3/2/b8c");
//! ```

mod bank;
mod board;
mod encode;
mod error;
mod fileio;
mod genres;
mod jsnum;
mod rooms;
mod state;
mod transform;
mod url;
mod variety;
mod variety_table;

pub use bank::{BankPiece, PieceBank};
pub use board::{Board, Group, Obj, Options, Segment};
pub use error::{Diagnostic, DiagnosticCode, Error, Result};
pub use genres::supported;
pub use transform::Transform;
pub use url::MAX_DIM;
pub use variety::{all_pids, to_pid};
pub use variety_table::PZPRJS_COMMIT;

/// Canonical URL body (same orientation) for any supported puzzle URL.
pub fn canonical_url(url: &str, opts: &Options) -> Result<String> {
    Board::from_url(url, opts)?.to_url()
}
