//! Doppelblock (pzprjs `variety/doppelblock.js`, pid `doppelblock`).
//!
//! The URL is the `decodeNumber16ExCell` outer-clue layer, followed by the
//! `decodeNumber16` cell-clue layer only when some cell holds a clue. FileIO
//! stores one combined cell/excell layer (`decodeCellExCellQnumAnumsub`): an
//! excell is its clue number, a cell with a question clue is `q<n>`, otherwise
//! the answer number, `+` (`qsub`) or `-` (`qans`, a black square).
//! `hasexcell` 1 puts the outer clues above and to the left; its
//! `adjustExCellTopLeft_1`/`_2` hooks redistribute them under transforms.

use super::Genre;
use crate::board::{Board, Group};
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};
use crate::transform::Transform;

pub(crate) static GENRE: &dyn Genre = &Doppelblock;

struct Doppelblock;

impl Genre for Doppelblock {
    fn pid(&self) -> &'static str {
        "doppelblock"
    }

    fn hasborder(&self) -> u8 {
        0
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn hasexcell(&self) -> u8 {
        1
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_number16_excell(b, body, d);
        // pzprjs always calls `decodeNumber16`, but `encodePzpr` omits the cell
        // layer when no cell holds a clue, so an empty remainder means "no cell
        // clues", not a cell body that ends too early.
        if rest.is_empty() {
            return rest;
        }
        encode::decode_number16(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut body = encode::encode_number16_excell(b);
        if b.cells.iter().any(|c| c.qnum != -1) {
            body.push_str(&encode::encode_number16(b));
        }
        Ok(body)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        decode_cell_excell_qnum_anumsub(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        encode_cell_excell_qnum_anumsub(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // `Cell.maxnum` is `cols - 2`; `Cell.numberWithMB` also stores the
        // answer marks (`qsub` for `?`, `qans` for a black square).
        let maxnum = b.cols() as i32 - 2;
        if b.cells.iter().any(|c| {
            !(c.qnum == -1 || c.qnum == -2 || (0..=maxnum).contains(&c.qnum))
                || !(c.anum == -1 || (0..=maxnum).contains(&c.anum))
                || !(0..=1).contains(&c.qsub)
                || !(0..=1).contains(&c.qans)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("doppelblock cell value outside empty / ? / clue 0-{maxnum} or answer 0-{maxnum}"),
            );
        }
        // `ExCell.maxnum` is `n * (n + 1) / 2` with `n = cols - 2`.
        let n = b.cols() as i32 - 2;
        let emax = n * (n + 1) / 2;
        if b.excells
            .iter()
            .any(|ex| !(ex.qnum == -1 || ex.qnum == -2 || (0..=emax).contains(&ex.qnum)))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                format!("doppelblock excell value outside empty / ? / 0-{emax}"),
            );
        }
    }

    /// pzprjs `BoardExec.adjustBoardData`/`adjustBoardData2`
    /// (`adjustExCellTopLeft_1`/`_2`) for the one-row top band and one-column
    /// left band of `hasexcell` 1. pzprst calls the hooks once per composed
    /// transform, so the pzprjs capture/rewrite sequence is replayed per
    /// operation; `Board::transform` has already moved the excells, and that
    /// layout is undone first.
    fn adjust_board_data(&self, t: Transform, b: &mut Board) {
        if b.geom.hasexcell != 1 || b.excells.is_empty() {
            return;
        }
        let (cols, rows) = (b.cols(), b.rows());
        // Undo pzprst's turnflipGroup: turns keep the order, flips reverse
        // their own band.
        let mut vals: Vec<i32> = b.excells.iter().map(|o| o.qnum).collect();
        let (mut c, mut r) = if t.swaps_axes() {
            (rows, cols)
        } else {
            (cols, rows)
        };
        match t {
            Transform::FlipX => vals[..c].reverse(),
            Transform::FlipY => vals[c..].reverse(),
            Transform::Transpose | Transform::AntiTranspose => vals[cols..].reverse(),
            _ => {}
        }
        let mut top: Vec<i32> = vals[..c].to_vec();
        let mut left: Vec<i32> = vals[c..].to_vec();
        for &op in operations(t) {
            // adjustExCellTopLeft_1: capture both bands.
            let qnumw = left.clone();
            let qnumh = top.clone();
            // turnflipGroup: a flip reverses its own band; a turn only swaps
            // the dimensions.
            match op {
                Transform::FlipX => top.reverse(),
                Transform::FlipY => left.reverse(),
                Transform::Rot90 | Transform::Rot270 => std::mem::swap(&mut c, &mut r),
                _ => {}
            }
            // adjustExCellTopLeft_2.
            match op {
                Transform::FlipX => left = qnumw,
                Transform::FlipY => top = qnumh,
                Transform::Rot90 => {
                    top = qnumw.iter().rev().copied().collect();
                    left = qnumh;
                }
                Transform::Rot270 => {
                    top = qnumw;
                    left = qnumh.iter().rev().copied().collect();
                }
                _ => {}
            }
        }
        if top.len() != cols || left.len() != rows {
            return;
        }
        for (i, v) in top.iter().enumerate() {
            b.excells[i].qnum = *v;
        }
        for (j, v) in left.iter().enumerate() {
            b.excells[cols + j].qnum = *v;
        }
    }
}

/// The pzprjs board operations a `Transform` stands for, in
/// `tools/gen_fixtures.js` order.
fn operations(t: Transform) -> &'static [Transform] {
    match t {
        Transform::Identity => &[],
        Transform::Rot90 => &[Transform::Rot90],
        Transform::Rot180 => &[Transform::Rot90, Transform::Rot90],
        Transform::Rot270 => &[Transform::Rot270],
        Transform::FlipX => &[Transform::FlipX],
        Transform::FlipY => &[Transform::FlipY],
        Transform::Transpose => &[Transform::Rot270, Transform::FlipY],
        Transform::AntiTranspose => &[Transform::Rot90, Transform::FlipY],
    }
}

/// `decodeCellExCellQnumAnumsub`: an excell is its clue, a cell is a question
/// clue (`q<n>`), the answer number, `+` (`qsub`) or `-` (`qans`, a black
/// square).
fn decode_cell_excell_qnum_anumsub(
    r: &mut FileReader,
    b: &mut Board,
    d: &mut Diagnostics,
) -> Result<()> {
    fileio::decode_cell_excell(r, b, d, |obj, group, ca| {
        if ca == "." {
            return Ok(());
        }
        if group == Group::ExCell {
            obj.qnum = fileio::int(ca)?;
        } else if let Some(rest) = ca.strip_prefix('q') {
            obj.qnum = fileio::int(rest)?;
        } else if ca == "+" {
            obj.qsub = 1;
        } else if ca == "-" {
            obj.qans = 1;
        } else {
            // pzprjs splits `[...]` `snum` arrays here when the cell class
            // enables them; `Obj` has no `snum`, so only the marks above are
            // read.
            obj.anum = fileio::int(ca)?;
        }
        Ok(())
    })
}

/// `encodeCellExCellQnumAnumsub`: the question clue as `q<n>` first, then the
/// answer number, `+` (`qsub`), `-` (`qans`) or `.`.
fn encode_cell_excell_qnum_anumsub(w: &mut FileWriter, b: &Board) {
    fileio::encode_cell_excell(w, b, |obj, group| {
        if group == Group::ExCell {
            if obj.qnum != -1 {
                return format!("{} ", obj.qnum);
            }
        } else {
            if obj.qnum != -1 {
                return format!("q{} ", obj.qnum);
            }
            let ca = if obj.anum != -1 {
                obj.anum.to_string()
            } else if obj.qsub == 1 {
                "+".into()
            } else if obj.qans == 1 {
                "-".into()
            } else {
                ".".into()
            };
            return format!("{ca} ");
        }
        ". ".into()
    });
}
