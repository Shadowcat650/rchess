use std::mem::MaybeUninit;
use crate::{ChessBoard, Color, Piece, Rank};
use crate::defs::{BitBoard, Square};

/// The [`PieceMoves`] struct stores the location of and the squares a piece targets.
#[derive(Clone, Copy, Debug)]
pub struct PieceMoves {
    pub location: Square,
    pub targets: BitBoard,
}

impl PieceMoves {
    /// Creates a new [`PieceMoves`] object.
    pub const fn new(location: Square, targets: BitBoard) -> Self {
        Self { location, targets }
    }
}


/// The [`MoveList`] struct stores a list of moves.
#[derive(Debug)]
pub struct MoveList {
    data: MaybeUninit<[PieceMoves;18]>,
    length: usize,
}

impl MoveList {
    /// Creates an empty [`MoveList`].
    pub const fn new() -> Self {
        MoveList {
            data: MaybeUninit::uninit(),
            length: 0,
        }
    }

    /// Returns `true` if the [`MoveList`] has no moves stored.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Adds a new item to the [`MoveList`] if it contains moves.
    pub fn push(&mut self, piece_moves: PieceMoves) {
        if !piece_moves.targets.is_empty() {
            unsafe {
                *self.data.assume_init_mut().get_unchecked_mut(self.length) = piece_moves;
            }
            self.length += 1;
        }
    }

    /// Pops the last move from the [`MoveList`].
    pub fn pop(&mut self) -> Option<PieceMoves> {
        if self.length == 0 {
            return None;
        }

        self.length -= 1;

        unsafe  {
            Some(*self.data.assume_init_ref().get_unchecked(self.length))
        }
    }

    /// Gets a reference to the last item in the [`MoveList`].
    pub fn back(&self) -> Option<&PieceMoves> {
        if self.is_empty() {
            return None;
        }

        unsafe {
            Some(self.data.assume_init_ref().get_unchecked(self.length - 1))
        }
    }

    /// Gets a mutable reference to the last item in the [`MoveList`].
    pub fn back_mut(&mut self) -> Option<&mut PieceMoves> {
        if self.is_empty() {
            return None;
        }

        unsafe {
            Some(self.data.assume_init_mut().get_unchecked_mut(self.length - 1))
        }
    }

    /// Counts the total number of moves in the [`MoveList`].
    pub fn count_moves(&self, chessboard: &ChessBoard) -> usize {
        // The total number of moves.
        let mut total = 0;

        // Count each move.
        for i in 0..self.length {
            let piece_moves = unsafe { self.data.assume_init_ref().get_unchecked(i)};
            let piece_sq = piece_moves.location;
            let (moving, _) = chessboard.piece_at(piece_sq).unwrap();

            // Pawns have special move cases.
            if moving == Piece::Pawn {
                // The rank pawn promote on.
                let promote_rank = match chessboard.turn() {
                    Color::White => BitBoard::from_rank(Rank::Eighth),
                    Color::Black => BitBoard::from_rank(Rank::First),
                };

                // The promotion moves.
                let promotions = piece_moves.targets & promote_rank;

                // The non-promotion moves.
                let normal = piece_moves.targets & !promotions;

                // Each promotion move increases the total 4x.
                total += 4 * promotions.popcnt() as usize;

                // The rest of the moves increase the total normally.
                total += normal.popcnt() as usize;
            } else {
                // Increase the total numer of moves.
                total += piece_moves.targets.popcnt() as usize;
            }
        }

        total
    }
}
