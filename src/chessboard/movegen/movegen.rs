use super::movelist::MoveList;
use crate::chessboard::movegen::generator::{generate_moves, generate_square_moves};
use crate::chessboard::{ChessBoard, Move};
use crate::defs::*;
use std::ops::Index;
use thiserror::Error;

/// The [`StrMoveCreationError`] enum is the error type produced when creating moves.
#[derive(Error, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrMoveCreationError {
    #[error("the move was not formatted correctly")]
    InvalidMove,

    #[error("the move was illegal")]
    IllegalMove(#[from] MoveCreationError),
}

/// The [`MoveCreationError`] struct signifies that there was an error while crating a move.
#[derive(Error, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, MoveGen};
    ///
    /// // Create a new chess board.
    /// let board = ChessBoard::new();
    ///
    /// // Get all the legal moves for the chess board.
    /// let moves = MoveGen::legal(&board);
    ///
    /// // Get all children of the chess board.
    /// let children = moves.into_iter().map(|mv| board.get_child(mv)).collect::<Vec<_>>();
    /// assert_eq!(children.len(), 20);
    /// ```
    #[inline]
    pub fn legal(chessboard: &'a ChessBoard) -> Self {
        let moves = generate_moves::<false>(chessboard);

        Self {
            chessboard,
            moves,
            promote_status: None,
        }
    }

    /// Gets a [`BitBoard`] of legal moves for the [`Piece`] on the given [`Square`].
    ///
    /// If there was no [`Piece`] on the given [`Square`], or it was not that [`Piece`]'s turn, an
    /// empty [`BitBoard`] is returned.
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, ChessBoard, MoveGen, Square};
    ///
    /// // Create a chess board.
    /// let board = ChessBoard::from_fen("5r2/pp3p1k/1qr1p2p/3p3P/3b1P2/2NP4/PP1Q1PR1/2K5 b - -").unwrap();
    ///
    /// // Get the queen moves for that position.
    /// let queen_moves = MoveGen::piece_legal(&board, Square::B6);
    ///
    /// // There should be 9 queen moves.
    /// assert_eq!(queen_moves.popcnt(), 9);
    ///
    /// // There are no moves since there is not a piece on this square.
    /// assert_eq!(MoveGen::piece_legal(&board, Square::A1), BitBoard::EMPTY);
    ///
    /// // It is black to move, so no moves are generated for the white pawn.
    /// assert_eq!(MoveGen::piece_legal(&board, Square::B2), BitBoard::EMPTY);
    /// ```
    #[inline]
    pub fn piece_legal(chessboard: &'a ChessBoard, square: Square) -> BitBoard {
        generate_square_moves::<false>(chessboard, square)
    }

    /// Creates a new [`MoveGen`] that generates only capture moves and king-defending moves.
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, MoveGen};
    ///
    /// // Create a chess board.
    /// let board = ChessBoard::from_fen("4k3/6pp/8/8/8/8/8/4K1nR w - -").unwrap();
    ///
    /// // Get all capture moves for the chess board.
    /// let moves = MoveGen::captures_only(&board);
    ///
    /// // Get all children of the chess board.
    /// let children = moves.into_iter().map(|mv| board.get_child(mv)).collect::<Vec<_>>();
    /// assert_eq!(children.len(), 2);
    /// ```
    #[inline]
    pub fn captures_only(chessboard: &'a ChessBoard) -> Self {
        let moves = generate_moves::<true>(chessboard);

        Self {
            chessboard,
            moves,
            promote_status: None,
        }
    }

    /// Gets a [`BitBoard`] of captures moves and king-defending moves for the [`Piece`] on the
    /// given [`Square`].
    ///
    /// If there was no [`Piece`] on the given [`Square`], or it was not that [`Piece`]'s turn, an
    /// empty [`BitBoard`] is returned.
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, ChessBoard, MoveGen, Square};
    ///
    /// // Create a chess board.
    /// let board = ChessBoard::from_fen("6k1/5r1p/1Q4p1/5p2/8/p1P5/PP3P2/1K3R1R w - -").unwrap();
    ///
    /// // Get the rook captures for that position.
    /// let rook_captures = MoveGen::piece_captures(&board, Square::H1);
    ///
    /// // There should be 1 rook capture.
    /// assert_eq!(rook_captures.popcnt(), 1);
    ///
    /// // There are no moves since there is not a piece on this square.
    /// assert_eq!(MoveGen::piece_captures(&board, Square::D5), BitBoard::EMPTY);
    ///
    /// // It is white to move, so no moves are generated for the black pawn.
    /// assert_eq!(MoveGen::piece_captures(&board, Square::A3), BitBoard::EMPTY);
    /// ```
    ///
    /// ```
    /// use rchess::{ChessBoard, MoveGen, Square};
    ///
    /// // Create a chess board.
    /// let board = ChessBoard::from_fen("6k1/6rp/4Q1p1/8/8/P1P2p2/P4P2/1K3R1R b - -").unwrap();
    ///
    /// // Get the king captures for that position.
    /// let king_captures = MoveGen::piece_captures(&board, Square::G8);
    ///
    /// // There are 2 evasions moves that the king has (no capture moves for the king).
    /// assert_eq!(king_captures.popcnt(), 2);
    /// ```
    #[inline]
    pub fn piece_captures(chessboard: &'a ChessBoard, square: Square) -> BitBoard {
        generate_square_moves::<true>(chessboard, square)
    }

    /// Turns the [`MoveGen`] into a [`Vec<Move>`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, MoveGen};
    ///
    /// // Create a new chess board.
    /// let board = ChessBoard::new();
    ///
    /// // Get a vec of legal moves for the chess board.
    /// let move_vec = MoveGen::legal(&board).to_vec();
    /// assert_eq!(move_vec.len(), 20);
    /// ```
    #[inline]
    pub fn to_vec(self) -> Vec<Move> {
        let mut vec = Vec::with_capacity(self.count_moves() as usize);
        vec.extend(self);
        vec
    }

    /// Returns `true` if no moves can be made on the [`ChessBoard`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, MoveGen};
    ///
    /// // Create a chess board in stalemate.
    /// let board = ChessBoard::from_fen("1r5k/8/8/8/8/8/7r/K7 w - -").unwrap();
    ///
    /// // Generate legal moves for the position.
    /// let moves = MoveGen::legal(&board);
    /// assert!(moves.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    /// Counts the number of legal moves for a given [`ChessBoard`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, MoveGen};
    ///
    /// // Create a new chess board.
    /// let board = ChessBoard::new();
    ///
    /// // Get the number of legal moves for the chess board.
    /// let n_moves = MoveGen::count_legal_moves(&board);
    /// assert_eq!(n_moves, 20);
    /// ```
    #[inline]
    pub fn count_legal_moves(chessboard: &ChessBoard) -> u32 {
        let moves = generate_moves::<false>(chessboard);
        moves.count_moves(chessboard)
    }

    /// Counts the number of moves left in the [`MoveGen`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, MoveGen};
    ///
    /// // Create a new chess board.
    /// let board = ChessBoard::new();
    ///
    /// // Get the legal moves for the chess board.
    /// let moves = MoveGen::legal(&board);
    /// assert_eq!(moves.count_moves(), 20);
    /// ```
    #[inline]
    pub fn count_moves(&self) -> u32 {
        self.moves.count_moves(self.chessboard)
    }

    /// Checks if a move with a given start and end [`Square`] is legal for a [`ChessBoard`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, MoveGen, Square};
    ///
    /// // Create a new chess board.
    /// let board = ChessBoard::new();
    ///
    /// // Check if the move "e2e4" is legal.
    /// assert!(MoveGen::is_legal(&board, Square::E2, Square::E4));
    ///
    /// // Check if the move "e1e2" is legal.
    /// assert!(!MoveGen::is_legal(&board, Square::E1, Square::E2));
    ///
    /// // Check if the move "b7b6" is legal.
    /// assert!(!MoveGen::is_legal(&board, Square::B7, Square::B6));
    /// ```
    #[inline]
    pub fn is_legal(chessboard: &ChessBoard, start: Square, end: Square) -> bool {
        let sq_legal_moves = generate_square_moves::<false>(chessboard, start);
        sq_legal_moves.contains(end)
    }

    /// Attempts to create a [`Move`] from a start and end square.
    ///
    /// If the move is a promotion, it promotes to a queen.
    ///
    /// If the move is illegal, a [`MoveCreationError`] is returned.
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, MoveGen, Square};
    ///
    /// // Create a new chess board.
    /// let board = ChessBoard::new();
    ///
    /// // Create the move "e2e4"
    /// let mv = MoveGen::create_move(&board, Square::E2, Square::E4);
    /// assert!(mv.is_ok());
    ///
    /// // Create the move "e1e2"
    /// let mv = MoveGen::create_move(&board, Square::E1, Square::E2);
    /// assert!(mv.is_err());
    /// ```
    #[inline]
    pub fn create_move(
        chessboard: &ChessBoard,
        start: Square,
        end: Square,
    ) -> Result<Move, MoveCreationError> {
        Self::create_promotion_move(chessboard, start, end, PieceType::Queen)
    }

    /// Creates a [`Move`] from a given [`&str`] for the given [`ChessBoard`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, MoveGen};
    ///
    /// // Create a new chess board.
    /// let board = ChessBoard::new();
    ///
    /// // Create the move "e2e4"
    /// let mv = MoveGen::create_str_move(&board, "e2e4");
    /// assert!(mv.is_ok());
    ///
    /// // Create the move "e1e2"
    /// let mv = MoveGen::create_str_move(&board, "e1e2");
    /// assert!(mv.is_err());
    /// ```
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
                'n' => PieceType::Knight,
                'b' => PieceType::Bishop,
                'r' => PieceType::Rook,
                'q' => PieceType::Queen,
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
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, Move, MoveGen, PieceType, Square};
    ///
    /// // Create a new chess board.
    /// let board = ChessBoard::from_fen("k7/3Q1P2/8/8/8/8/8/K7 w - -").unwrap();
    ///
    /// // Promote pawn to a rook instead of a queen.
    /// let mv = MoveGen::create_promotion_move(&board, Square::F7, Square::F8, PieceType::Rook).unwrap();
    /// assert_eq!(mv, Move::Promote { start: Square::F7, end: Square::F8, target: PieceType::Rook });
    ///
    /// // The move might not be a promotion.
    /// let mv = MoveGen::create_promotion_move(&board, Square::D7, Square::E7, PieceType::Rook).unwrap();
    /// assert_eq!(mv, Move::Quiet { start: Square::D7, end: Square::E7, moving: PieceType::Queen });
    /// ```
    #[inline]
    pub fn create_promotion_move(
        chessboard: &ChessBoard,
        start: Square,
        end: Square,
        target: PieceType,
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
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, Move, MoveGen, PieceType, Square};
    ///
    /// // Create a chess board.
    /// let board = ChessBoard::new();
    ///
    /// // Make sure the move is legal.
    /// assert!(MoveGen::is_legal(&board, Square::E2, Square::E4));
    ///
    /// // Make the move. // SAFETY: The move is legal.
    /// let mv = unsafe { MoveGen::create_promotion_move_unchecked(&board, Square::E2, Square::E4, PieceType::Queen)};
    /// assert_eq!(mv, Move::DoublePawnPush { start: Square::E2, end: Square::E4 });
    /// ```
    #[inline]
    pub unsafe fn create_promotion_move_unchecked(
        chessboard: &ChessBoard,
        start: Square,
        end: Square,
        target: PieceType,
    ) -> Move {
        // Get extra board info.
        let us = chessboard.turn();
        let them = !chessboard.turn();
        let moving = chessboard.piece_at(start).unwrap().kind;

        // Look for special pawn moves.
        if moving == PieceType::Pawn {
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
        else if moving == PieceType::King {
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

        let mut total_nodes = 0;
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
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, MoveGen};
    ///
    /// // Create a new chess board.
    /// let board = ChessBoard::new();
    ///
    /// // Run a perft to depth 3.
    /// let res = MoveGen::perft(board, 3);
    /// assert_eq!(res, 8902);
    /// ```
    #[inline]
    pub fn perft(chessboard: ChessBoard, depth: u8) -> u32 {
        if depth == 0 {
            return 1;
        }
        if depth == 1 {
            return Self::count_legal_moves(&chessboard);
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

/// The [`MoveGen`] struct can iterate through all generated moves.
impl Iterator for MoveGen<'_> {
    type Item = Move;

    /// Gets the next move in the [`MoveGen`].
    ///
    /// # Examples
    /// ```no_run
    /// use rchess::{MoveGen, ChessBoard};
    ///
    /// // Create a new chess board.
    /// let board = ChessBoard::new();
    ///
    /// // Generate moves for the chess board.
    /// let mut moves = MoveGen::legal(&board);
    ///
    /// // Get the next move.
    /// let mv = moves.next().unwrap();
    /// ```
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
                    PieceType::Bishop
                }
                PromoteStatus::PromoteRook => {
                    self.promote_status = Some(PromoteStatus::PromoteQueen);
                    PieceType::Rook
                }
                PromoteStatus::PromoteQueen => {
                    self.promote_status = None;
                    self.moves.back_mut().unwrap().targets ^= end.bitboard();
                    PieceType::Queen
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
            Self::create_promotion_move_unchecked(self.chessboard, start, end, PieceType::Knight)
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
