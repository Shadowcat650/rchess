use super::movelist::MoveList;
use crate::chessboard::movegen::generator::{generate_moves, generate_square_legal};
use crate::chessboard::{ChessBoard, Move};
use crate::defs::*;
use std::ops::Index;
use thiserror::Error;

/// The [`StrMoveCreationError`] enum is the error type produced when classifying moves.
#[derive(Error, Debug, Copy, Clone, Eq, PartialEq)]
pub enum StrMoveCreationError {
    #[error("the move was not formatted correctly")]
    InvalidMove,

    #[error("the move was illegal")]
    IllegalMove(#[from] MoveCreationError),
}

#[derive(Error, Debug, Copy, Clone, Eq, PartialEq)]
#[error("the move was illegal")]
pub struct MoveCreationError;

/// The [`MoveGen`] struct generates moves for a [`ChessBoard`].
pub struct MoveGen<'a> {
    chessboard: &'a ChessBoard,
    moves: MoveList,
    promote_status: Option<PromoteStatus>,
}

/// The piece being promoted to.
enum PromoteStatus {
    PromoteBishop,
    PromoteRook,
    PromoteQueen,
}

impl<'a> MoveGen<'a> {
    /// Creates a new [`MoveGen`] that generates all legal moves.
    #[inline]
    pub fn legal(chessboard: &'a ChessBoard) -> Self {
        let mut moves = generate_moves::<false>(chessboard);

        MoveGen {
            chessboard,
            moves,
            promote_status: None,
        }
    }

    /// Creates a new [`MoveGen`] that generates only capture moves and king-defending moves.
    #[inline]
    pub fn captures_only(chessboard: &'a ChessBoard) -> Self {
        let mut moves = generate_moves::<true>(chessboard);

        MoveGen {
            chessboard,
            moves,
            promote_status: None,
        }
    }

    /// Turns the [`MoveGen`] into a [`Vec<Move>`].
    #[inline]
    pub fn to_vec(self) -> Vec<Move> {
        let mut vec = Vec::with_capacity(self.count_moves() as usize);
        vec.extend(self);
        vec
    }

    /// Returns `true` if no moves can be made on the [`ChessBoard`].
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    /// Counts the number of legal moves for a given [`ChessBoard`].
    #[inline]
    pub fn count_legal_moves(chessboard: &ChessBoard) -> u32 {
        let mut moves = generate_moves::<false>(chessboard);
        moves.count_moves(chessboard)
    }

    /// Counts the number of moves left in the [`MoveGen`].
    #[inline]
    pub fn count_moves(&self) -> u32 {
        self.moves.count_moves(self.chessboard)
    }

    /// Checks if a move with a given start and end square is legal for a chess board.
    #[inline]
    pub fn is_legal(chessboard: &ChessBoard, start: Square, end: Square) -> bool {
        let sq_legal_moves = generate_square_legal(chessboard, start);
        sq_legal_moves.contains(end)
    }

    /// Attempts to create a [`Move`] from a start and end square.
    ///
    /// If the move is a promotion, it promotes to a queen.
    ///
    /// If the move is illegal, a [`MoveCreationError`] is returned.
    #[inline]
    pub fn create_move(
        chessboard: &ChessBoard,
        start: Square,
        end: Square,
    ) -> Result<Move, MoveCreationError> {
        Self::create_promotion_move(chessboard, start, end, Piece::Queen)
    }

    /// Classifies a move string and turns it into a [`Move`] for a [`ChessBoard`].
    #[inline]
    pub fn create_str_move(
        chessboard: &ChessBoard,
        str: &str,
    ) -> Result<Move, StrMoveCreationError> {
        // The move string is too short.
        if str.len() < 4 {
            return Err(StrMoveCreationError::InvalidMove);
        }

        // Get move start & end squares.
        let start =
            Square::from_string(str.index(0..=1)).or(Err(StrMoveCreationError::InvalidMove))?;
        let end =
            Square::from_string(str.index(2..=3)).or(Err(StrMoveCreationError::InvalidMove))?;

        if str.len() == 5 {
            let target = match *str
                .as_bytes()
                .get(4)
                .ok_or(StrMoveCreationError::InvalidMove)? as char
            {
                'n' => Piece::Knight,
                'b' => Piece::Bishop,
                'r' => Piece::Rook,
                'q' => Piece::Queen,
                _ => return Err(StrMoveCreationError::InvalidMove),
            };
            Ok(Self::create_promotion_move(chessboard, start, end, target)?)
        } else {
            Ok(Self::create_move(chessboard, start, end)?)
        }
    }

    /// Attempts to create a [`Move`] from a start and end square.
    ///
    /// The move does not have to be a promotion, the `target` is what piece a pawn will promote to
    /// if there happens to be a promotion.
    ///
    /// If the move is illegal, a [`MoveCreationError`] is returned.
    #[inline]
    pub fn create_promotion_move(
        chessboard: &ChessBoard,
        start: Square,
        end: Square,
        target: Piece,
    ) -> Result<Move, MoveCreationError> {
        // Make sure the move is legal.
        if !Self::is_legal(chessboard, start, end) {
            return Err(MoveCreationError);
        }
        unsafe {
            Ok(Self::create_promotion_move_unchecked(
                chessboard, start, end, target,
            ))
        }
    }

    /// Creates a [`Move`] from a start and end square.
    ///
    /// The move does not have to be a promotion, the `target` is what piece a pawn will promote to
    /// if there happens to be a promotion.
    ///
    /// # Safety
    /// Caller ensures the start and end squares produce a legal move.
    #[inline]
    pub unsafe fn create_promotion_move_unchecked(
        chessboard: &ChessBoard,
        start: Square,
        end: Square,
        target: Piece,
    ) -> Move {
        // Get extra board info.
        let us = chessboard.turn();
        let them = !chessboard.turn();
        let (moving, _) = chessboard.piece_at(start).unwrap();

        // Look for special pawn moves.
        if moving == Piece::Pawn {
            let (start_rank, double_rank, promote_rank) = match us {
                Color::White => (Rank::Second, Rank::Fourth, Rank::Eighth),
                Color::Black => (Rank::Seventh, Rank::Fifth, Rank::First),
            };
            // Look for double pawn push.
            if start.rank() == start_rank && end.rank() == double_rank {
                return Move::DoublePawnPush { start, end };
            }
            // Look for en passant.
            else if chessboard.en_passant_sq().is_some_and(|sq| sq == end) {
                return Move::EnPassant { start, end };
            }
            // Look for promotion.
            else if end.rank() == promote_rank {
                // Look for captures.
                return if end.bitboard().overlaps(chessboard.color_occupancy(them)) {
                    Move::PromoteCapture { start, end, target }
                } else {
                    Move::Promote { start, end, target }
                };
            }
        }
        // Look for castles.
        else if moving == Piece::King {
            let (castle_start, ks_end, qs_end) = match us {
                Color::White => (Square::E1, Square::G1, Square::C1),
                Color::Black => (Square::E8, Square::G8, Square::C8),
            };
            if start == castle_start && end == ks_end {
                return Move::Castle {
                    start,
                    end,
                    side: CastleSide::Kingside,
                };
            } else if start == castle_start && end == qs_end {
                return Move::Castle {
                    start,
                    end,
                    side: CastleSide::Queenside,
                };
            }
        }

        // Look for captures.
        if end.bitboard().overlaps(chessboard.color_occupancy(them)) {
            Move::Capture { start, end, moving }
        } else {
            Move::Quiet { start, end, moving }
        }
    }

    /// Runs a debug perft on a given [`ChessBoard`], where the nodes for each move are printed.
    #[inline]
    pub fn debug_perft(chessboard: ChessBoard, depth: u8) {
        let movegen = MoveGen::legal(&chessboard);

        let mut total_nodes: u64 = 0;
        for mv in movegen {
            let mut child_board = chessboard.clone();
            child_board.make_move(mv);

            let nodes = Self::perft(child_board, depth - 1);
            total_nodes += nodes;

            println!("{}: {}", mv, nodes);
        }

        println!("Total Nodes: {}", total_nodes);
    }

    /// Runs a perft on a given [`ChessBoard`].
    #[inline]
    pub fn perft(chessboard: ChessBoard, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }
        if depth == 1 {
            return Self::count_legal_moves(&chessboard) as u64;
        }

        let movegen = MoveGen::legal(&chessboard);
        let mut total_nodes = 0;

        for mv in movegen {
            let mut child_board = chessboard.clone();
            child_board.make_move(mv);

            total_nodes += Self::perft(child_board, depth - 1);
        }

        total_nodes
    }
}

impl Iterator for MoveGen<'_> {
    type Item = Move;

    /// Gets the next move in the [`MoveGen`].
    fn next(&mut self) -> Option<Self::Item> {
        // Make sure there are moves to generate.
        if self.moves.is_empty() {
            return None;
        }

        if self.moves.back().unwrap().targets.is_empty() {
            self.moves.pop();
            if self.moves.is_empty() {
                return None;
            }
        }

        // Get start and end squares.
        let start = self.moves.back().unwrap().location;
        let end = self.moves.back().unwrap().targets.b_scan_forward().unwrap();

        // Get data about the chess board.
        let them = !self.chessboard.turn();

        // Handle promotion variations.
        if let Some(promote_status) = &self.promote_status {
            let target = match promote_status {
                PromoteStatus::PromoteBishop => {
                    self.promote_status = Some(PromoteStatus::PromoteRook);
                    Piece::Bishop
                }
                PromoteStatus::PromoteRook => {
                    self.promote_status = Some(PromoteStatus::PromoteQueen);
                    Piece::Rook
                }
                PromoteStatus::PromoteQueen => {
                    self.promote_status = None;
                    self.moves.back_mut().unwrap().targets ^= end.bitboard();
                    Piece::Queen
                }
            };

            // Look for captures.
            return Some(
                if end
                    .bitboard()
                    .overlaps(self.chessboard.color_occupancy(them))
                {
                    Move::PromoteCapture { start, end, target }
                } else {
                    Move::Promote { start, end, target }
                },
            );
        }

        // SAFETY: The movegen only contains legal moves.
        let mv = unsafe {
            Self::create_promotion_move_unchecked(self.chessboard, start, end, Piece::Knight)
        };

        // Handle promotion sequence.
        if let Move::Promote { .. } | Move::PromoteCapture { .. } = &mv {
            self.promote_status = Some(PromoteStatus::PromoteBishop);
        } else {
            // Remove the end square from targets.
            self.moves.back_mut().unwrap().targets ^= end.bitboard();
        }

        Some(mv)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let moves_remaining = self.count_moves() as usize;
        (moves_remaining, Some(moves_remaining))
    }
}

impl ExactSizeIterator for MoveGen<'_> {}
