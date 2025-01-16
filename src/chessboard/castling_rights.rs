use crate::{CastleSide, Color};

/// The [`CastlingRights`] struct stores which sides and colors can castle
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CastlingRights(u8);

impl CastlingRights {
    /// White king castling right bit
    const WHITE_KING: u8 = 0b1;

    /// White queen castling right bit
    const WHITE_QUEEN: u8 = 0b10;

    /// Black king castling right bit
    const BLACK_KING: u8 = 0b100;

    /// Black queen castling right bit
    const BLACK_QUEEN: u8 = 0b1000;

    /// Creates a new [`CastlingRights`] struct with no rights stored.
    pub fn new() -> Self {
        Self(0)
    }

    /// Checks if there are no castling rights set.
    pub fn is_none_set(&self) -> bool {
        self.0 == 0
    }

    /// Checks if a castling right is set for a given [`CastleSide`] and [`Color`].
    pub fn is_set(&self, side: CastleSide, color: Color) -> bool {
        match (side, color) {
            (CastleSide::Kingside, Color::Black) => self.0 & Self::BLACK_KING != 0,
            (CastleSide::Kingside, Color::White) => self.0 & Self::WHITE_KING != 0,
            (CastleSide::Queenside, Color::Black) => self.0 & Self::BLACK_QUEEN != 0,
            (CastleSide::Queenside, Color::White) => self.0 & Self::WHITE_QUEEN != 0,
        }
    }

    /// Sets a given castling right for a given [`CastleSide`] and [`Color`].
    pub fn set(&mut self, side: CastleSide, color: Color) {
        match (side, color) {
            (CastleSide::Kingside, Color::Black) => self.0 |= Self::BLACK_KING,
            (CastleSide::Kingside, Color::White) => self.0 |= Self::WHITE_KING,
            (CastleSide::Queenside, Color::Black) => self.0 |= Self::BLACK_QUEEN,
            (CastleSide::Queenside, Color::White) => self.0 |= Self::WHITE_QUEEN,
        };
    }

    /// Unsets a given castling right for a given [`CastleSide`] and [`Color`].
    pub fn unset(&mut self, side: CastleSide, color: Color) {
        match (side, color) {
            (CastleSide::Kingside, Color::Black) => self.0 &= !Self::BLACK_KING,
            (CastleSide::Kingside, Color::White) => self.0 &= !Self::WHITE_KING,
            (CastleSide::Queenside, Color::Black) => self.0 &= !Self::BLACK_QUEEN,
            (CastleSide::Queenside, Color::White) => self.0 &= !Self::WHITE_QUEEN,
        };
    }

    /// Unsets the castling rights for a given [`Color`].
    pub fn unset_color(&mut self, color: Color) {
        match color {
            Color::White => self.0 &= !(Self::WHITE_KING | Self::WHITE_QUEEN),
            Color::Black => self.0 &= !(Self::BLACK_KING | Self::BLACK_QUEEN),
        }
    }
}
