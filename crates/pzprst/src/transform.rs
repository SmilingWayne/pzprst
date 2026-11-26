//! Board symmetries (dihedral group D4), matching pzprjs `BoardExec.turnflipGroup`.

use std::fmt;
use std::str::FromStr;

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Transform {
    Identity,
    /// Clockwise quarter turn (pzprjs `turnr`).
    Rot90,
    Rot180,
    /// Counter-clockwise quarter turn (pzprjs `turnl`).
    Rot270,
    /// Mirror left↔right (pzprjs `flipx`).
    FlipX,
    /// Mirror top↔bottom (pzprjs `flipy`).
    FlipY,
    /// Main-diagonal reflection (`turnl` then `flipy`).
    Transpose,
    /// Anti-diagonal reflection (`turnr` then `flipy`).
    AntiTranspose,
}

impl Transform {
    /// All eight geometric transformations.
    pub const ALL: [Transform; 8] = [
        Self::Identity,
        Self::Rot90,
        Self::Rot180,
        Self::Rot270,
        Self::FlipX,
        Self::FlipY,
        Self::Transpose,
        Self::AntiTranspose,
    ];

    /// `(x', y') = (a x + b y, c x + d y)` on centred coordinates, y pointing down.
    fn matrix(self) -> [i32; 4] {
        match self {
            Self::Identity => [1, 0, 0, 1],
            Self::Rot90 => [0, -1, 1, 0],
            Self::Rot180 => [-1, 0, 0, -1],
            Self::Rot270 => [0, 1, -1, 0],
            Self::FlipX => [-1, 0, 0, 1],
            Self::FlipY => [1, 0, 0, -1],
            Self::Transpose => [0, 1, 1, 0],
            Self::AntiTranspose => [0, -1, -1, 0],
        }
    }

    fn from_matrix(m: [i32; 4]) -> Self {
        *Self::ALL
            .iter()
            .find(|t| t.matrix() == m)
            .expect("D4 is closed")
    }

    pub fn swaps_axes(self) -> bool {
        self.matrix()[0] == 0
    }

    /// Map a doubled coordinate on a `c × r` board to the transformed `nc × nr` board.
    pub fn map_point(self, bx: i32, by: i32, c: i32, r: i32, nc: i32, nr: i32) -> (i32, i32) {
        let [a, b, cc, d] = self.matrix();
        let (x, y) = (bx - c, by - r);
        (a * x + b * y + nc, cc * x + d * y + nr)
    }

    /// `self` followed by `next`.
    pub fn then(self, next: Transform) -> Transform {
        let [a1, b1, c1, d1] = self.matrix();
        let [a2, b2, c2, d2] = next.matrix();
        Self::from_matrix([
            a2 * a1 + b2 * c1,
            a2 * b1 + b2 * d1,
            c2 * a1 + d2 * c1,
            c2 * b1 + d2 * d1,
        ])
    }

    pub fn inverse(self) -> Transform {
        let [a, b, c, d] = self.matrix();
        Self::from_matrix([a, c, b, d])
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::Rot90 => "rot90",
            Self::Rot180 => "rot180",
            Self::Rot270 => "rot270",
            Self::FlipX => "flipx",
            Self::FlipY => "flipy",
            Self::Transpose => "transpose",
            Self::AntiTranspose => "antitranspose",
        }
    }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Transform {
    type Err = Error;

    /// Accepts pzprst names and the pzprjs operation names `turnr`, `turnl`.
    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "turnr" => return Ok(Self::Rot90),
            "turnl" => return Ok(Self::Rot270),
            _ => {}
        }
        Self::ALL
            .into_iter()
            .find(|t| t.name() == s)
            .ok_or_else(|| Error::InvalidTransform(s.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_laws() {
        for t in Transform::ALL {
            assert_eq!(t.then(t.inverse()), Transform::Identity);
            assert_eq!(Transform::Identity.then(t), t);
        }
        assert_eq!(Transform::Rot90.then(Transform::Rot90), Transform::Rot180);
        assert_eq!(
            Transform::Rot270.then(Transform::FlipY),
            Transform::Transpose
        );
        assert_eq!(
            Transform::Rot90.then(Transform::FlipY),
            Transform::AntiTranspose
        );
        let r = Transform::Rot90;
        assert_eq!(r.then(r).then(r).then(r), Transform::Identity);
    }

    #[test]
    fn pzprjs_maps() {
        // turnr: (2R - by, bx); turnl: (by, 2C - bx) on a C=3, R=2 board.
        assert_eq!(Transform::Rot90.map_point(1, 1, 3, 2, 2, 3), (3, 1));
        assert_eq!(Transform::Rot270.map_point(1, 1, 3, 2, 2, 3), (1, 5));
        assert_eq!(Transform::FlipX.map_point(1, 1, 3, 2, 3, 2), (5, 1));
    }
}
