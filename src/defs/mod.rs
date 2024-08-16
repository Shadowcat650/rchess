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

/// The starting chess position's fen.
pub const START_FEN: &str  = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";

/// The [`CastleSide`] enum represents the side a king can castle.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CastleSide {
    Kingside,
    Queenside,
}
