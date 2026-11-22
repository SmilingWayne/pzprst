//! Land Measurement (pzprjs `variety/landmeasure.js`, pid `landmeasure`).
//!
//! Every cross (including the outer frame, `Board.hascross: 2`) stores a
//! distance in `qnum`: 0, 1-2 on the frame corners/edges, up to
//! `((cols * rows) | 1) - 2` inside, and `-2` for the infinity sign. The URL is
//! `genericDecodeNumber16`/`genericEncodeNumber16` over the whole cross grid;
//! FileIO stores the same numbers and then the cell shading (`qans` 1) and
//! background mark (`qsub` 1) answers.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &Landmeasure;

struct Landmeasure;

impl Genre for Landmeasure {
    fn pid(&self) -> &'static str {
        "landmeasure"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    /// pzprjs `Board.hascross` (the inherited default 2: crosses exist on the
    /// even/even points of the whole board, outer frame included).
    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        // `genericDecodeNumber16(bd.cross.length, …)`.
        let n = b.crosses.len();
        let crosses = &mut b.crosses;
        encode::generic_decode_number16(n, body, d, |c, val| crosses[c].qnum = val)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        // `genericEncodeNumber16(bd.cross.length, …)`.
        Ok(encode::generic_encode_number16(b.crosses.len(), |c| {
            b.crosses[c].qnum
        }))
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cross_num(r, b, d)?;
        fileio::decode_cell_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cross_num(w, b);
        fileio::encode_cell_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        // pzprjs `Cross.maxnum`: 1 on a corner, 2 elsewhere on the frame,
        // `((cols * rows) | 1) - 2` inside; `minnum` is 0 and `-2` is the
        // infinity sign.
        let (maxbx, maxby) = (2 * b.cols() as i32, 2 * b.rows() as i32);
        let inside_max = ((b.cols() * b.rows()) | 1) as i32 - 2;
        let maxnum = |bx: i32, by: i32| {
            let edge1 = bx == 0 || bx == maxbx;
            let edge2 = by == 0 || by == maxby;
            if edge1 || edge2 {
                if edge1 && edge2 {
                    1
                } else {
                    2
                }
            } else {
                inside_max
            }
        };
        let bad_cross = (0..b.crosses.len()).any(|i| {
            let qnum = b.crosses[i].qnum;
            let (bx, by) = b.geom.cross_pos(i).unwrap_or((0, 0));
            !(qnum == -1 || qnum == -2 || (0..=maxnum(bx, by)).contains(&qnum))
        });
        if bad_cross {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "landmeasure cross value outside 0-maxnum, empty or infinity",
            );
        }
        if b.cells.iter().any(|c| {
            c.qnum != -1 || !(0..=1).contains(&c.qans) || !(0..=1).contains(&c.qsub)
        }) {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "landmeasure cell value outside empty, shading 0/1 or mark 0/1",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "landmeasure border value outside answers 0/1",
            );
        }
    }
}
