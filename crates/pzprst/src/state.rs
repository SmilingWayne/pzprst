//! pzpr-trace `state`: FileIO text, raw DEFLATE, unpadded URL-safe base64.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use miniz_oxide::inflate::{decompress_to_vec_with_limit, TINFLStatus};

use crate::board::Board;
use crate::error::{Error, Result};

pub fn encode(fileio: &str) -> String {
    let compressed = miniz_oxide::deflate::compress_to_vec(fileio.as_bytes(), 9);
    URL_SAFE_NO_PAD.encode(compressed)
}

/// pzpr-trace's URL `state` decoding: `[A-Za-z0-9_-]+`, raw DEFLATE, UTF-8.
pub fn decode(state: &str, max_bytes: usize) -> Result<String> {
    let bytes = URL_SAFE_NO_PAD
        .decode(state.trim())
        .map_err(|e| Error::StateDecode(format!("base64url: {e}")))?;
    let inflated = decompress_to_vec_with_limit(&bytes, max_bytes).map_err(|e| {
        if e.status == TINFLStatus::HasMoreOutput {
            Error::ResourceLimit(format!("inflated state exceeds {max_bytes} bytes"))
        } else {
            Error::StateDecode(format!("inflate: {:?}", e.status))
        }
    })?;
    String::from_utf8(inflated).map_err(|_| Error::StateDecode("state is not UTF-8".into()))
}

/// pzpr-trace import check: same type, size, rules and question data.
pub(crate) fn check_match(puzzle: &Board, state: &Board) -> Result<()> {
    let mismatch = |what: String| Err(Error::StateMismatch(what));
    if puzzle.pid() != state.pid() {
        return mismatch(format!("puzzle type {} vs {}", puzzle.pid(), state.pid()));
    }
    if (puzzle.cols(), puzzle.rows()) != (state.cols(), state.rows()) {
        return mismatch(format!(
            "size {}x{} vs {}x{}",
            puzzle.cols(),
            puzzle.rows(),
            state.cols(),
            state.rows()
        ));
    }
    if puzzle.rules != state.rules {
        return mismatch(format!("rules {:?} vs {:?}", puzzle.rules, state.rules));
    }
    for (i, (a, b)) in puzzle.cells.iter().zip(&state.cells).enumerate() {
        if a.question() != b.question() {
            return mismatch(format!(
                "clue at row {}, column {}",
                i / puzzle.cols(),
                i % puzzle.cols()
            ));
        }
    }
    for (i, (a, b)) in puzzle.crosses.iter().zip(&state.crosses).enumerate() {
        if a.question() != b.question() {
            let (bx, by) = puzzle.geom.cross_pos(i).unwrap_or((0, 0));
            return mismatch(format!("cross at ({bx}, {by})"));
        }
    }
    for (i, (a, b)) in puzzle.borders.iter().zip(&state.borders).enumerate() {
        if a.question() != b.question() {
            let (bx, by) = puzzle.geom.border_pos(i);
            return mismatch(format!("border at ({bx}, {by})"));
        }
    }
    Ok(())
}
