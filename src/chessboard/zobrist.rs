use super::ChessBoard;
use crate::defs::*;

include!(concat!(env!("OUT_DIR"), "/zobrist.rs"));

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// The [`ZobristHash`] is the hash of a [`ChessBoard`].
pub struct ZobristHash(u64);

impl ZobristHash {
    /// Creates a new `ZobristHash` for an empty [`ChessBoard`].
    #[inline]
    pub(super) const fn new() -> Self {
        ZobristHash(0)
    }

    /// Converts the [`ZobristHash`] into a [`u64`].
    #[inline]
    pub const fn to_u64(self) -> u64 {
        self.0
    }

    /// Adds/removes a piece from a square in the [`ZobristHash`].
    #[inline]
    pub(super) fn piece(&mut self, square: Square, piece: Piece, color: Color) {
        self.0 ^= PIECE_ZOBRIST[color.index()][piece.index()][square.index()];
    }

    /// Adds/removes a castle right from the [`ZobristHash`].
    #[inline]
    pub(super) fn castle_right(&mut self, side: CastleSide, color: Color) {
        match side {
            CastleSide::Kingside => self.0 ^= CASTLE_RIGHTS_ZOBRIST[0][color.index()],
            CastleSide::Queenside => self.0 ^= CASTLE_RIGHTS_ZOBRIST[1][color.index()],
        }
    }

    /// Adds/removes the en passant file from the [`ZobristHash`].
    #[inline]
    pub(super) fn ep(&mut self, square: Square) {
        self.0 ^= EN_PASSANT_ZOBRIST[square.file() as usize];
    }

    /// Toggles the turn in the [`ZobristHash`].
    #[inline]
    pub(super) fn toggle_turn(&mut self) {
        self.0 ^= TURN_ZOBRIST;
    }
}
