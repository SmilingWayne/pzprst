//! Reusable pzpr piece-bank data and serialization.
use crate::error::{Error, Result};
use crate::jsnum;

/// A rectangular bitmap in a piece bank. `qcmp` is a player completion mark.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BankPiece {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<bool>,
    pub qcmp: i32,
}

impl BankPiece {
    /// Decode pzpr's compact base-32 bitmap or its `width:bits` raw form.
    pub fn from_serialized(value: &str) -> Result<Self> {
        let invalid = || Error::ValueOutOfRange(format!("invalid bank piece {value:?}"));
        let (width, height, cells) = if let Some((w, bits)) = value.split_once(':') {
            let width = w.parse::<usize>().map_err(|_| invalid())?;
            if width == 0
                || width > 35
                || bits.is_empty()
                || bits.len() % width != 0
                || bits.len() / width > 35
                || !bits.bytes().all(|b| b == b'0' || b == b'1')
            {
                return Err(invalid());
            }
            (
                width,
                bits.len() / width,
                bits.bytes().map(|b| b == b'1').collect(),
            )
        } else {
            let mut chars = value.chars();
            let width = chars
                .next()
                .and_then(|c| c.to_digit(36))
                .ok_or_else(invalid)? as usize;
            let height = chars
                .next()
                .and_then(|c| c.to_digit(36))
                .ok_or_else(invalid)? as usize;
            if width == 0 || height == 0 {
                return Err(invalid());
            }
            let mut cells = Vec::new();
            for c in chars {
                let n = c.to_digit(32).ok_or_else(invalid)?;
                for shift in (0..5).rev() {
                    if cells.len() < width * height {
                        cells.push(n & (1 << shift) != 0);
                    }
                }
            }
            cells.resize(width * height, false);
            cells.truncate(width * height);
            (width, height, cells)
        };
        if height > 35 {
            return Err(invalid());
        }
        Ok(Self {
            width,
            height,
            cells,
            qcmp: 0,
        })
    }

    /// Encode as pzpr's compact bitmap.
    pub fn serialize(&self) -> Result<String> {
        if self.width == 0
            || self.height == 0
            || self.width > 35
            || self.height > 35
            || self.cells.len() != self.width * self.height
        {
            return Err(Error::ValueOutOfRange(
                "invalid bank piece dimensions".into(),
            ));
        }
        let mut out = format!(
            "{}{}",
            jsnum::to_radix(self.width as u32, 36),
            jsnum::to_radix(self.height as u32, 36)
        );
        for chunk in self.cells.chunks(5) {
            let mut value = 0u32;
            for i in 0..5 {
                value = (value << 1) | u32::from(chunk.get(i).copied().unwrap_or(false));
            }
            out.push_str(&jsnum::to_radix(value, 32));
        }
        while out.ends_with('0') {
            out.pop();
        }
        Ok(out)
    }
}

/// Ordered piece inventory; pieces are part of the puzzle, marks are snapshot data.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PieceBank {
    pub pieces: Vec<BankPiece>,
}

impl PieceBank {
    pub fn from_serialized<'a>(values: impl IntoIterator<Item = &'a str>) -> Result<Self> {
        Ok(Self {
            pieces: values
                .into_iter()
                .map(BankPiece::from_serialized)
                .collect::<Result<Vec<_>>>()?,
        })
    }
    pub fn serialized(&self) -> Result<Vec<String>> {
        self.pieces.iter().map(BankPiece::serialize).collect()
    }
    pub(crate) fn same_question(&self, other: &Self) -> bool {
        self.pieces.len() == other.pieces.len()
            && self
                .pieces
                .iter()
                .zip(&other.pieces)
                .all(|(a, b)| a.width == b.width && a.height == b.height && a.cells == b.cells)
    }
}

/// A genre supplies its own named presets while sharing the pzpr bank codecs.
pub(crate) type Presets = &'static [(&'static str, &'static [&'static str])];

pub(crate) fn from_preset(presets: Presets, name: &str) -> Option<PieceBank> {
    presets
        .iter()
        .find(|(key, _)| *key == name)
        .map(|(_, pieces)| {
            PieceBank::from_serialized(pieces.iter().copied()).expect("valid static preset")
        })
}

fn preset_key(presets: Presets, bank: &PieceBank) -> Option<&'static str> {
    let pieces = bank.serialized().ok()?;
    presets
        .iter()
        .find(|(_, expected)| {
            expected
                .iter()
                .copied()
                .eq(pieces.iter().map(String::as_str))
        })
        .map(|(key, _)| *key)
}

/// `decodePieceBank`: read the URL suffix after the genre's clue data.
pub(crate) fn decode_url(
    body: &str,
    bank: &mut Option<PieceBank>,
    presets: Presets,
    d: &mut crate::error::Diagnostics,
) -> String {
    use crate::error::DiagnosticCode;
    if let Some(rest) = body.strip_prefix("//") {
        let key = rest
            .chars()
            .next()
            .map(|c| c.to_string())
            .unwrap_or_default();
        if let Some(value) = from_preset(presets, &key) {
            *bank = Some(value);
        } else {
            d.push(DiagnosticCode::UnexpectedChar, "unknown piece bank preset");
        }
        return rest.chars().skip(1).collect();
    }
    if let Some(rest) = body.strip_prefix('/') {
        let mut parts = rest.split('/');
        let count = parts
            .next()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0)
            .min(10000);
        let mut pieces = Vec::new();
        for _ in 0..count {
            if let Some(s) = parts.next() {
                match BankPiece::from_serialized(s) {
                    Ok(piece) => pieces.push(piece),
                    Err(_) => d.push(DiagnosticCode::UnexpectedChar, "invalid bank piece"),
                }
            } else {
                d.push(DiagnosticCode::TruncatedBody, "missing bank piece");
                break;
            }
        }
        *bank = Some(PieceBank { pieces });
        return parts.collect::<Vec<_>>().join("/");
    }
    d.push(DiagnosticCode::TruncatedBody, "missing piece bank");
    String::new()
}

/// `encodePieceBank`: append the preset or serialized custom pieces.
pub(crate) fn encode_url(bank: &PieceBank, presets: Presets) -> Result<String> {
    if let Some(key) = preset_key(presets, bank) {
        return Ok(format!("//{key}"));
    }
    let pieces = bank.serialized()?;
    let mut out = format!("/{}", pieces.len());
    for piece in pieces {
        out.push('/');
        out.push_str(&piece);
    }
    Ok(out)
}

/// `decodePieceBank`: a preset line or a count followed by the pieces.
pub(crate) fn decode_fileio(
    r: &mut crate::fileio::FileReader,
    fallback: &PieceBank,
    presets: Presets,
) -> Result<PieceBank> {
    let head = r.read_line().unwrap_or("").to_string();
    if let Ok(count) = head.parse::<usize>() {
        let mut pieces = Vec::new();
        for _ in 0..count.min(10000) {
            pieces.push(BankPiece::from_serialized(r.read_line().unwrap_or(""))?);
        }
        Ok(PieceBank { pieces })
    } else {
        Ok(from_preset(presets, &head).unwrap_or_else(|| fallback.clone()))
    }
}

/// `encodePieceBank`: a preset line or a count followed by the pieces.
pub(crate) fn encode_fileio(
    w: &mut crate::fileio::FileWriter,
    bank: &PieceBank,
    presets: Presets,
) -> Result<()> {
    if let Some(key) = preset_key(presets, bank) {
        w.write_line(key);
    } else {
        let pieces = bank.serialized()?;
        w.write_line(&pieces.len().to_string());
        for piece in pieces {
            w.write_line(&piece);
        }
    }
    Ok(())
}

/// `decodePieceBankQcmp`: player completion marks in bank order.
pub(crate) fn decode_marks(r: &mut crate::fileio::FileReader, bank: &mut PieceBank) {
    for (piece, mark) in bank
        .pieces
        .iter_mut()
        .zip(r.read_line().unwrap_or("").split_whitespace())
    {
        piece.qcmp = jsnum::integer(mark).unwrap_or(0);
    }
}

/// `encodePieceBankQcmp`: player completion marks in bank order.
pub(crate) fn encode_marks(w: &mut crate::fileio::FileWriter, bank: &PieceBank) {
    use std::fmt::Write as _;
    let mut marks = String::new();
    for piece in &bank.pieces {
        write!(marks, "{} ", piece.qcmp).expect("writing to a string cannot fail");
    }
    w.write_line(&marks);
}
