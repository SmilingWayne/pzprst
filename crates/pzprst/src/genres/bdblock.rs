//! Border Block (pzprjs `variety/bdblock.js`, pid `bdblock`): cell numbers,
//! black dots on the cross grid and block borders as answer data.

use super::Genre;
use crate::board::Board;
use crate::encode;
use crate::error::{DiagnosticCode, Diagnostics, Result};
use crate::fileio::{self, FileReader, FileWriter};

pub(crate) static GENRE: &dyn Genre = &BdBlock;

struct BdBlock;

impl Genre for BdBlock {
    fn pid(&self) -> &'static str {
        "bdblock"
    }

    fn hasborder(&self) -> u8 {
        1
    }

    fn hascross(&self) -> u8 {
        2
    }

    fn rule_flags(&self) -> &'static [(char, &'static str)] {
        &[]
    }

    fn decode_url(&self, b: &mut Board, body: &str, d: &mut Diagnostics) -> String {
        let rest = encode::decode_cross_mark(b, body, d);
        // `decodePzpr` drops the `/` between the cross marks and the numbers.
        let rest: String = rest.chars().skip(1).collect();
        encode::decode_number16(b, &rest, d)
    }

    fn encode_url(&self, b: &Board) -> Result<String> {
        let mut body = encode::encode_cross_mark(b);
        body.push('/');
        body.push_str(&encode::encode_number16(b));
        Ok(body)
    }

    fn decode_fileio(&self, r: &mut FileReader, b: &mut Board, d: &mut Diagnostics) -> Result<()> {
        fileio::decode_cell_qnum(r, b, d)?;
        fileio::decode_cross_num(r, b, d)?;
        fileio::decode_border_ans(r, b, d)
    }

    fn encode_fileio(&self, b: &Board, w: &mut FileWriter) -> Result<()> {
        fileio::encode_cell_qnum(w, b);
        fileio::encode_cross_num(w, b);
        fileio::encode_border_ans(w, b);
        Ok(())
    }

    fn validate(&self, b: &Board, d: &mut Diagnostics) {
        if b.cells
            .iter()
            .any(|c| c.qnum != -1 && c.qnum != -2 && !(1..=999).contains(&c.qnum))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "bdblock cell value outside empty / ? / 1-999",
            );
        }
        if b.crosses
            .iter()
            .any(|c| c.qnum != -1 && c.qnum != -2 && !(1..=999).contains(&c.qnum))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "bdblock cross value outside empty / ? / 1-999",
            );
        }
        if b.crosses
            .iter()
            .any(|c| c.qnum == -2 || (2..=999).contains(&c.qnum))
        {
            d.push(
                DiagnosticCode::LossyUrl,
                "a bdblock URL stores only a black dot (1) on a cross",
            );
        }
        if b.borders
            .iter()
            .any(|e| !(0..=1).contains(&e.qans) || !(0..=1).contains(&e.qsub))
        {
            d.push(
                DiagnosticCode::ValueOutOfRange,
                "bdblock border value outside block 0/1 or cross 0/1",
            );
        }
    }
}
