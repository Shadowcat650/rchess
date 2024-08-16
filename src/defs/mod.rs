mod bitboard;
mod color;
mod direction;
mod file;
mod magic;
mod piece;
mod rank;
mod square;

pub use bitboard::*;
pub use color::*;
pub use direction::*;
pub use file::*;
pub use magic::*;
pub use piece::*;
pub use rank::*;
pub use square::*;

/// The direction a king can castle.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CastleSide {
    Kingside,
    Queenside,
}
