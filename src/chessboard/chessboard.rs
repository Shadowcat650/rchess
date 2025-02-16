use super::tables;
use super::zobrist::ZobristHash;
use crate::chessboard::builder::{BoardBuilder, BoardBuilderError};
use crate::chessboard::castling_rights::CastlingRights;
use crate::chessboard::tables::{
    get_bishop_attacks, get_king_attacks, get_knight_attacks, get_pawn_attacks, get_rook_attacks,
};
use crate::defs::*;
use crate::{MoveGen, StrMoveCreationError};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use thiserror::Error;

/// The [`Move`] enum represents a move on a chess board.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Move {
    Quiet {
        start: Square,
        end: Square,
        moving: PieceType,
    },
    Capture {
        start: Square,
        end: Square,
        moving: PieceType,
    },
    Castle {
        start: Square,
        end: Square,
        side: CastleSide,
    },
    DoublePawnPush {
        start: Square,
        end: Square,
    },
    EnPassant {
        start: Square,
        end: Square,
    },
    Promote {
        start: Square,
        end: Square,
        target: PieceType,
    },
    PromoteCapture {
        start: Square,
        end: Square,
        target: PieceType,
    },
}

/// The [`BuilderConversionError`] enum is the error type for converting a [`BoardBuilder`] to a [`ChessBoard`].
#[derive(Error, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BuilderConversionError {
    #[error("the turn was not set")]
    TurnNotSet,

    #[error("at least one king was missing")]
    MissingKing,

    #[error("the en passant square was invalid")]
    InvalidEnPassant,

    #[error("a castling right was invalid")]
    InvalidCastleRight,

    #[error("the inactive king can be captured")]
    InactiveKingAttacked,

    #[error("more than 18 pieces were set for a given color")]
    TooManyPieces,
}

/// The [`FenLoadError`] enum is the error type for loading a fen position.
#[derive(Error, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FenLoadError {
    #[error("there was an error with the fen formatting")]
    Formatting(#[from] FenFormatError),

    #[error("there was an error while building the board")]
    Builder(#[from] BoardBuilderError),

    #[error("there was an error while converting the board")]
    Conversion(#[from] BuilderConversionError),
}

/// The [`FenFormatError`] enum is the error type for a fen's formatting.
#[derive(Error, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FenFormatError {
    #[error("the fen's piece section is invalid")]
    InvalidPieceSection,

    #[error("the fen piece section was missing")]
    MissingPieceSection,

    #[error("the fen's turn section is invalid")]
    InvalidTurnSection,

    #[error("the fen turn section was missing")]
    MissingTurnSection,

    #[error("the fen's castling rights section is invalid")]
    InvalidCastleRights,

    #[error("the fen castling rights section was missing")]
    MissingCastleRights,

    #[error("the fen's en passant section is invalid")]
    InvalidEnPassant,

    #[error("the fen en passant section was missing")]
    MissingEnPassant,

    #[error("the halmove clock section was invalid")]
    InvalidHalfMoveSection,
}

/// The [`Footprint`] struct is used to identify a [`ChessBoard`] without extra computed data.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Footprint {
    piece_bbs: [BitBoard; 6],
    color_bbs: [BitBoard; 2],
    castling_rights: CastlingRights,
    en_passant: Option<Square>,
    turn: Color,
    hash: ZobristHash,
}

impl Hash for Footprint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash.to_u64());
    }
}

/// The [`ChessBoard`] struct represents a chess board.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChessBoard {
    /// Where the pieces of a given piece type are on the chess board.
    piece_bbs: [BitBoard; 6],

    /// Where the pieces of a given color are on the chess board.
    color_bbs: [BitBoard; 2],

    /// The castling rights.
    castling_rights: CastlingRights,

    /// The square that can be targeted by en passant.
    en_passant: Option<Square>,

    /// The next color to make a move.
    turn: Color,

    /// The pinned pieces.
    pinned: BitBoard,

    /// The pieces checking the king.
    checkers: BitBoard,

    /// A hash of the board state.
    hash: ZobristHash,

    /// The half move clock.
    half_move_clock: u8,
}

impl ChessBoard {
    /// Creates a new [`ChessBoard`] in the starting position.
    #[inline]
    pub fn new() -> Self {
        ChessBoard::from_fen(START_FEN).unwrap()
    }

    /// Creates a new [`ChessBoard`] with the given [`&str`] moves made.
    ///
    /// The move strings must be in algebraic chess notation.
    ///
    /// # Examples
    /// ```
    /// use rchess::ChessBoard;
    ///
    /// // A chess board with three moves made from the starting position.
    /// let board = ChessBoard::from_str_moves(&["e2e4", "e7e6", "g1f3"]).unwrap();
    /// ```
    #[inline]
    pub fn from_str_moves(moves: &[&str]) -> Result<Self, StrMoveCreationError> {
        let mut board = Self::new();
        for str_move in moves {
            let mv = MoveGen::create_str_move(&board, *str_move)?;
            board.make_move(mv);
        }
        Ok(board)
    }

    /// Attempts to create a new [`ChessBoard`] from the given fen string.
    #[inline]
    pub fn from_fen(fen: &str) -> Result<Self, FenLoadError> {
        // Create a board builder.
        let mut builder = BoardBuilder::new();

        // Split the fen into chunks.
        let mut fen = fen.split_whitespace();

        // Load fen piece positions.
        let mut square_idx = Square::A8.as_u8();
        let fen_pieces = fen.next().ok_or(FenFormatError::MissingPieceSection)?;
        for c in fen_pieces.chars() {
            match c {
                // Insert a piece.
                'p' | 'n' | 'b' | 'r' | 'q' | 'k' | 'P' | 'N' | 'B' | 'R' | 'Q' | 'K' => {
                    let square =
                        Square::from_u8(square_idx).ok_or(FenFormatError::InvalidPieceSection)?;
                    builder = builder.piece(square, Piece::from_char(c).unwrap())?;
                    square_idx += 1;
                }
                // Skip empty squares.
                '1'..='8' => {
                    let empty_squares = c.to_digit(10).unwrap() as u8;
                    square_idx += empty_squares;
                }
                // Move to the next line.
                '/' => square_idx -= 16,
                // Unrecognized character.
                _ => return Err(FenFormatError::InvalidPieceSection.into()),
            }
        }

        // Load fen turn.
        let fen_turn = fen.next().ok_or(FenFormatError::MissingTurnSection)?;
        match fen_turn {
            "w" => builder = builder.turn(Color::White)?,
            "b" => builder = builder.turn(Color::Black)?,
            _ => return Err(FenFormatError::InvalidTurnSection.into()),
        }

        // Load fen castling rights.
        let fen_castling_rights = fen.next().ok_or(FenFormatError::MissingCastleRights)?;
        if fen_castling_rights == "-" {
        } else {
            for c in fen_castling_rights.chars() {
                match c {
                    'K' => builder = builder.castle_right(CastleSide::Kingside, Color::White)?,
                    'Q' => builder = builder.castle_right(CastleSide::Queenside, Color::White)?,
                    'k' => builder = builder.castle_right(CastleSide::Kingside, Color::Black)?,
                    'q' => builder = builder.castle_right(CastleSide::Queenside, Color::Black)?,
                    _ => return Err(FenFormatError::InvalidCastleRights.into()),
                }
            }
        }

        // Load fen en passant square.
        let fen_en_passant = fen.next().ok_or(FenFormatError::MissingEnPassant)?;
        if let Ok(square) = Square::from_string(fen_en_passant) {
            builder = builder.en_passant(square)?;
        } else if fen_en_passant != "-" {
            return Err(FenFormatError::InvalidEnPassant.into());
        }

        let mut board = Self::from_builder(builder)?;

        // Load halfmove clock (if provided).
        if let Some(halfmoves) = fen.next() {
            match halfmoves.parse::<u8>() {
                Ok(halfmoves) if halfmoves <= 100 => board.half_move_clock = halfmoves,
                _ => {
                    return Err(FenLoadError::Formatting(
                        FenFormatError::InvalidHalfMoveSection,
                    ))
                }
            }
        }

        Ok(board)
    }

    /// Creates a new [`ChessBoard`] from the given [`BoardBuilder`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BoardBuilder, ChessBoard, Color, PieceType, Square};
    ///
    /// // Create a board builder.
    /// let builder = BoardBuilder::new()
    ///     .piece(Square::A1, (PieceType::King, Color::White)).unwrap()
    ///     .piece(Square::H8, (PieceType::King, Color::Black)).unwrap()
    ///     .turn(Color::Black).unwrap();
    ///
    /// // Convert the builder to a chess board.
    /// let board = ChessBoard::from_builder(builder).unwrap();
    /// ```
    #[inline]
    pub fn from_builder(board_builder: BoardBuilder) -> Result<Self, BuilderConversionError> {
        if board_builder.color_bbs[Color::White.index()].popcnt() > 18 {
            return Err(BuilderConversionError::TooManyPieces);
        }

        if board_builder.color_bbs[Color::Black.index()].popcnt() > 18 {
            return Err(BuilderConversionError::TooManyPieces);
        }

        if board_builder.turn.is_none() {
            return Err(BuilderConversionError::TurnNotSet);
        }

        let turn = board_builder.turn.unwrap();

        if board_builder.piece_bbs[PieceType::King.index()].popcnt() != 2 {
            return Err(BuilderConversionError::MissingKing);
        }

        if let Some(sq) = board_builder.en_passant_square {
            match turn {
                Color::White => {
                    if sq.rank() != Rank::Sixth {
                        return Err(BuilderConversionError::InvalidEnPassant);
                    }

                    if board_builder.piece_map[sq.down().unwrap().index()]
                        != Some(Piece::BLACK_PAWN)
                    {
                        return Err(BuilderConversionError::InvalidEnPassant);
                    }
                }
                Color::Black => {
                    if sq.rank() != Rank::Third {
                        return Err(BuilderConversionError::InvalidEnPassant);
                    }

                    if board_builder.piece_map[sq.up().unwrap().index()] != Some(Piece::WHITE_PAWN)
                    {
                        return Err(BuilderConversionError::InvalidEnPassant);
                    }
                }
            }
        }

        if board_builder
            .castling_rights
            .is_set(CastleSide::Kingside, Color::White)
        {
            if board_builder.piece_map[Square::E1.index()] != Some(Piece::WHITE_KING)
                || board_builder.piece_map[Square::H1.index()] != Some(Piece::WHITE_ROOK)
            {
                return Err(BuilderConversionError::InvalidCastleRight);
            }
        }

        if board_builder
            .castling_rights
            .is_set(CastleSide::Queenside, Color::White)
        {
            if board_builder.piece_map[Square::E1.index()] != Some(Piece::WHITE_KING)
                || board_builder.piece_map[Square::A1.index()] != Some(Piece::WHITE_ROOK)
            {
                return Err(BuilderConversionError::InvalidCastleRight);
            }
        }

        if board_builder
            .castling_rights
            .is_set(CastleSide::Kingside, Color::Black)
        {
            if board_builder.piece_map[Square::E8.index()] != Some(Piece::BLACK_KING)
                || board_builder.piece_map[Square::H8.index()] != Some(Piece::BLACK_ROOK)
            {
                return Err(BuilderConversionError::InvalidCastleRight);
            }
        }

        if board_builder
            .castling_rights
            .is_set(CastleSide::Kingside, Color::Black)
        {
            if board_builder.piece_map[Square::E8.index()] != Some(Piece::BLACK_KING)
                || board_builder.piece_map[Square::A8.index()] != Some(Piece::BLACK_ROOK)
            {
                return Err(BuilderConversionError::InvalidCastleRight);
            }
        }

        let mut chessboard = Self {
            piece_bbs: board_builder.piece_bbs,
            color_bbs: board_builder.color_bbs,
            castling_rights: board_builder.castling_rights,
            en_passant: None,
            turn,
            pinned: BitBoard::EMPTY,
            checkers: BitBoard::EMPTY,
            hash: board_builder.hash,
            half_move_clock: 0,
        };

        if chessboard.is_attacked(
            chessboard.get_king_square(!chessboard.turn),
            chessboard.turn,
        ) {
            return Err(BuilderConversionError::InactiveKingAttacked);
        }

        chessboard.calculate_extra_data();

        Ok(chessboard)
    }

    /// Copies the [`ChessBoard`] and makes a move on it.
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, MoveGen, Square};
    ///
    /// // The parent chess board.
    /// let mut parent = ChessBoard::new();
    ///
    /// // Get a move for the parent board.
    /// let mv = MoveGen::create_move(&parent, Square::E2, Square::E4).unwrap();
    ///
    /// // Make a child board.
    /// let child = parent.get_child(mv);
    ///
    /// // The child board is equivalent to the parent board that has made the move.
    /// parent.make_move(mv);
    /// assert_eq!(parent, child);
    /// ```
    #[inline]
    pub fn get_child(&self, mv: Move) -> Self {
        let mut child = self.clone();
        child.make_move(mv);
        child
    }

    /// Gets a fen string representing the [`ChessBoard`].
    ///
    /// # Examples
    /// ```
    /// use rchess::ChessBoard;
    ///
    /// // Create a board in the starting position.
    /// let board = ChessBoard::new();
    /// assert_eq!(&board.get_fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -");
    /// ```
    #[inline]
    pub fn get_fen(&self) -> String {
        // Create the fen piece section.
        let mut fen_pieces: String = String::new();
        for rank in RANKS.into_iter().rev() {
            let mut gap = 0;

            for file in FILES {
                let square = Square::at(rank, file);

                match &self.piece_at(square) {
                    // Increase gap
                    None => gap += 1,
                    // Add piece
                    Some(piece) => {
                        if gap != 0 {
                            fen_pieces += gap.to_string().as_str();
                            gap = 0;
                        }

                        fen_pieces.push(piece.to_char());
                    }
                };

                // Add line separator
                if file == File::H {
                    if gap != 0 {
                        fen_pieces += gap.to_string().as_str();
                        gap = 0;
                    }

                    if rank != Rank::First {
                        fen_pieces.push('/');
                    }
                }
            }
        }

        // Create the fen color section.
        let fen_color = self.turn.to_char();

        // Create the fen castling rights section.
        let fen_castle_rights = if self.castling_rights.is_none_set() {
            "-".to_string()
        } else {
            let mut fen_castle_rights = String::with_capacity(4);
            if self.is_castle_right_set(CastleSide::Kingside, Color::White) {
                fen_castle_rights.push('K');
            }
            if self.is_castle_right_set(CastleSide::Queenside, Color::White) {
                fen_castle_rights.push('Q');
            }
            if self.is_castle_right_set(CastleSide::Kingside, Color::Black) {
                fen_castle_rights.push('k');
            }
            if self.is_castle_right_set(CastleSide::Queenside, Color::Black) {
                fen_castle_rights.push('q');
            }
            fen_castle_rights
        };

        // Create the fen en passant square section.
        let fen_ep_square = match self.en_passant {
            None => "-".to_string(),
            Some(sq) => sq.to_string(),
        };

        // Combine all fen sections into a single string.
        format!(
            "{} {} {} {}",
            fen_pieces, fen_color, fen_castle_rights, fen_ep_square
        )
    }

    /// Makes a move on the [`ChessBoard`].
    ///
    /// # Warning
    /// If the move was not generated by a [`MoveGen`], behavior is undefined.
    #[inline]
    pub fn make_move(&mut self, mv: Move) {
        // Get data about the current move & board state.
        let us = self.turn;
        let them = !self.turn;

        self.clear_ep();
        self.toggle_turn();

        let mut reset_halfmoves = false;
        match mv {
            Move::Quiet { start, end, moving } => {
                // Remove relevant castling rights for moving kings or rooks.
                if moving == PieceType::King {
                    self.unset_color_rights(us)
                } else if moving == PieceType::Rook {
                    match (us, start) {
                        (Color::Black, Square::A8) => {
                            self.unset_castle_right(CastleSide::Queenside, us)
                        }
                        (Color::Black, Square::H8) => {
                            self.unset_castle_right(CastleSide::Kingside, us)
                        }
                        (Color::White, Square::A1) => {
                            self.unset_castle_right(CastleSide::Queenside, us)
                        }
                        (Color::White, Square::H1) => {
                            self.unset_castle_right(CastleSide::Kingside, us)
                        }
                        _ => (),
                    }
                } else if moving == PieceType::Pawn {
                    reset_halfmoves = true;
                }

                self.move_piece(start, end, (moving, us));
            }
            Move::Capture { start, end, moving } => {
                // Remove relevant castling rights for moving kings or rooks.
                if moving == PieceType::King {
                    self.unset_color_rights(us)
                } else if moving == PieceType::Rook {
                    match (us, start) {
                        (Color::Black, Square::A8) => {
                            self.unset_castle_right(CastleSide::Queenside, us)
                        }
                        (Color::Black, Square::H8) => {
                            self.unset_castle_right(CastleSide::Kingside, us)
                        }
                        (Color::White, Square::A1) => {
                            self.unset_castle_right(CastleSide::Queenside, us)
                        }
                        (Color::White, Square::H1) => {
                            self.unset_castle_right(CastleSide::Kingside, us)
                        }
                        _ => (),
                    }
                }

                // Remove captured rook castling rights.
                match (us, end) {
                    (Color::Black, Square::A1) => {
                        self.unset_castle_right(CastleSide::Queenside, them)
                    }
                    (Color::Black, Square::H1) => {
                        self.unset_castle_right(CastleSide::Kingside, them)
                    }
                    (Color::White, Square::A8) => {
                        self.unset_castle_right(CastleSide::Queenside, them)
                    }
                    (Color::White, Square::H8) => {
                        self.unset_castle_right(CastleSide::Kingside, them)
                    }
                    _ => (),
                }

                // Remove the captured piece.
                self.remove(end);

                // Move the piece.
                self.move_piece(start, end, (moving, us));

                reset_halfmoves = true;
            }
            Move::Castle { start, end, side } => {
                // Get rook start & end squares.
                let (rook_start, rook_end) = match (us, side) {
                    (Color::Black, CastleSide::Queenside) => (Square::A8, Square::D8),
                    (Color::Black, CastleSide::Kingside) => (Square::H8, Square::F8),
                    (Color::White, CastleSide::Queenside) => (Square::A1, Square::D1),
                    (Color::White, CastleSide::Kingside) => (Square::H1, Square::F1),
                };

                // Move the rook.
                self.move_piece(rook_start, rook_end, (PieceType::Rook, us));

                // Move the king.
                self.move_piece(start, end, (PieceType::King, us));

                // Unset castling rights for the side that moved.
                self.unset_color_rights(us);
            }
            Move::DoublePawnPush { start, end } => {
                // Set the en passant square.
                match us {
                    Color::White => self.set_ep(start.up().unwrap()),
                    Color::Black => self.set_ep(start.down().unwrap()),
                };

                // Move the piece.
                self.move_piece(start, end, (PieceType::Pawn, us));

                reset_halfmoves = true;
            }
            Move::EnPassant { start, end } => {
                // Capture the en-passanted piece.
                match us {
                    Color::White => self.remove(end.down().unwrap()),
                    Color::Black => self.remove(end.up().unwrap()),
                };

                // Move the piece.
                self.move_piece(start, end, (PieceType::Pawn, us));

                reset_halfmoves = true;
            }
            Move::Promote { start, end, target } => {
                // Remove current piece.
                self.remove(start);

                // Insert the promoted piece.
                self.insert(end, (target, us));

                reset_halfmoves = true;
            }
            Move::PromoteCapture { start, end, target } => {
                // Remove captured rook castling rights.
                match (us, end) {
                    (Color::Black, Square::A1) => {
                        self.unset_castle_right(CastleSide::Queenside, them)
                    }
                    (Color::Black, Square::H1) => {
                        self.unset_castle_right(CastleSide::Kingside, them)
                    }
                    (Color::White, Square::A8) => {
                        self.unset_castle_right(CastleSide::Queenside, them)
                    }
                    (Color::White, Square::H8) => {
                        self.unset_castle_right(CastleSide::Kingside, them)
                    }
                    _ => (),
                }

                // Remove the captured piece.
                self.remove(end);

                // Remove current piece.
                self.remove(start);

                // Insert the promoted piece.
                self.insert(end, (target, us));

                reset_halfmoves = true;
            }
        }

        if reset_halfmoves {
            self.half_move_clock = 0;
        } else {
            self.half_move_clock += 1;
        }

        // Calculate non-position data.
        self.calculate_extra_data();
    }

    /// Calculates non-positional data for the [`ChessBoard`].
    fn calculate_extra_data(&mut self) {
        self.calculate_pinned();
        self.calculate_checkers();
    }

    /// Calculates the pinned pieces on the [`ChessBoard`].
    fn calculate_pinned(&mut self) {
        // Get extra data about the board.
        let us = self.turn;
        let them = !self.turn;
        let friendly = self.color_occupancy(us);
        let king_sq = self.get_king_square(us);

        // Reset the current pinned pieces.
        self.pinned = BitBoard::EMPTY;

        // Get enemy potential pinners (rooks, bishops, and queens).
        let enemy_rooks =
            self.query((PieceType::Rook, them)) | self.query((PieceType::Queen, them));
        let enemy_bishops =
            self.query((PieceType::Bishop, them)) | self.query((PieceType::Queen, them));

        // Get the enemy pieces pinning our pieces.
        let rook_pinners =
            enemy_rooks & tables::get_ghost_rook(king_sq, self.occupancy(), friendly);
        let bishop_pinners =
            enemy_bishops & tables::get_ghost_bishop(king_sq, self.occupancy(), friendly);
        let pinners = rook_pinners | bishop_pinners;

        // Add all the pinned pieces to the pinners bitboard.
        for pinner_sq in pinners {
            self.pinned |= friendly & tables::get_direct_connection(pinner_sq, king_sq);
        }
    }

    /// Calculates the checkers for the [`ChessBoard`].
    fn calculate_checkers(&mut self) {
        // Get extra data about the board.
        let us = self.turn;
        let them = !self.turn;
        let king_sq = self.get_king_square(us);

        // Reset the current checkers bitboard.
        self.checkers = BitBoard::EMPTY;

        // Look for pawn checkers.
        let pawn_check_locations = get_pawn_attacks(king_sq, us);
        self.checkers |= self.query((PieceType::Pawn, them)) & pawn_check_locations;

        // Look for knight checkers.
        let knight_check_locations = get_knight_attacks(king_sq);
        self.checkers |= self.query((PieceType::Knight, them)) & knight_check_locations;

        // Look for bishop & queen checkers.
        let bishop_check_locations = get_bishop_attacks(king_sq, self.occupancy());
        self.checkers |= (self.query((PieceType::Bishop, them))
            | self.query((PieceType::Queen, them)))
            & bishop_check_locations;

        // Look for rook & queen checkers.
        let rook_check_locations = get_rook_attacks(king_sq, self.occupancy());
        self.checkers |= (self.query((PieceType::Rook, them))
            | self.query((PieceType::Queen, them)))
            & rook_check_locations;
    }

    /// Returns `true` if the given [`Square`] is attacked by the given [`Color`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, Square, Color};
    ///
    /// // Create a new chess board.
    /// let board = ChessBoard::from_fen("4k3/8/8/8/8/8/4Q3/4K3 b - -").unwrap();
    ///
    /// assert!(board.is_attacked(Square::E8, Color::White));
    /// assert!(!board.is_attacked(Square::E8, Color::Black));
    /// assert!(!board.is_attacked(Square::A8, Color::White));
    /// ```
    #[inline]
    pub fn is_attacked(&self, square: Square, by: Color) -> bool {
        let us = !by;

        // Look for pawn checkers.
        let pawn_check_locations = get_pawn_attacks(square, us);
        if self
            .query((PieceType::Pawn, by))
            .overlaps(pawn_check_locations)
        {
            return true;
        }

        // Look for knight checkers.
        let knight_check_locations = get_knight_attacks(square);
        if self
            .query((PieceType::Knight, by))
            .overlaps(knight_check_locations)
        {
            return true;
        }

        // Look for king checkers.
        let king_check_locations = get_king_attacks(square);
        if self
            .query((PieceType::King, by))
            .overlaps(king_check_locations)
        {
            return true;
        }

        // Look for bishop & queen checkers.
        let bishop_check_locations = get_bishop_attacks(square, self.occupancy());
        if (self.query((PieceType::Bishop, by)) | self.query((PieceType::Queen, by)))
            .overlaps(bishop_check_locations)
        {
            return true;
        };

        // Look for rook & queen checkers.
        let rook_check_locations = get_rook_attacks(square, self.occupancy());
        if (self.query((PieceType::Rook, by)) | self.query((PieceType::Queen, by)))
            .overlaps(rook_check_locations)
        {
            return true;
        }

        false
    }

    /// Inserts a new piece into the [`ChessBoard`].
    ///
    /// Note: This function assumes that there is not already a piece at the given [`Square`].
    fn insert(&mut self, square: Square, piece: impl Into<Piece>) {
        let piece = piece.into();
        self.piece_bbs[piece.kind.index()] |= square.bitboard();
        self.color_bbs[piece.color.index()] |= square.bitboard();
        self.hash.piece(square, piece);
    }

    /// Removes a piece from the [`ChessBoard`]
    ///
    /// Note: This function assumes there is a piece at the given [`Square`].
    fn remove(&mut self, square: Square) {
        let piece = self.piece_at(square).unwrap();
        self.piece_bbs[piece.kind.index()] ^= square.bitboard();
        self.color_bbs[piece.color.index()] ^= square.bitboard();
        self.hash.piece(square, piece);
    }

    /// Moves a piece from one square to another.
    ///
    /// Note: This function assumes that there is a piece at the start square and that the end square is empty.
    fn move_piece(&mut self, start: Square, end: Square, piece: impl Into<Piece>) {
        let piece = piece.into();
        self.piece_bbs[piece.kind.index()] ^= start.bitboard() | end.bitboard();
        self.color_bbs[piece.color.index()] ^= start.bitboard() | end.bitboard();
        self.hash.piece(start, piece);
        self.hash.piece(end, piece);
    }

    /// Toggles the current turn.
    fn toggle_turn(&mut self) {
        self.turn = !self.turn;
        self.hash.toggle_turn();
    }

    /// Sets a castling right.
    fn set_castle_right(&mut self, side: CastleSide, color: Color) {
        self.castling_rights.set(side, color);
        self.hash.castle_right(side, color);
    }

    /// Unsets a castling right.
    fn unset_castle_right(&mut self, side: CastleSide, color: Color) {
        if self.castling_rights.is_set(side, color) {
            self.castling_rights.unset(side, color);
            self.hash.castle_right(side, color);
        }
    }

    /// Unsets all the castling rights for a given color.
    fn unset_color_rights(&mut self, color: Color) {
        self.unset_castle_right(CastleSide::Kingside, color);
        self.unset_castle_right(CastleSide::Queenside, color);
    }

    /// Sets the en passant square.
    fn set_ep(&mut self, square: Square) {
        self.en_passant = Some(square);
        self.hash.ep(square);
    }

    /// Clears the en passant square.
    fn clear_ep(&mut self) {
        if let Some(square) = self.en_passant {
            self.hash.ep(square);
            self.en_passant = None;
        }
    }

    /// Gets the piece at the given [`Square`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessBoard, Square, Piece};
    ///
    /// // Create a new chess board.
    /// let board = ChessBoard::new();
    ///
    /// assert_eq!(board.piece_at(Square::A1), Some(Piece::WHITE_ROOK));
    /// assert_eq!(board.piece_at(Square::A8), Some(Piece::BLACK_ROOK));
    /// assert_eq!(board.piece_at(Square::E5), None);
    /// ```
    #[inline]
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        let color = if self.color_bbs[Color::White.index()].overlaps(square.bitboard()) {
            Color::White
        } else if self.color_bbs[Color::Black.index()].overlaps(square.bitboard()) {
            Color::Black
        } else {
            return None;
        };

        let pnr = self.piece_bbs[PieceType::Pawn.index()]
            | self.piece_bbs[PieceType::Knight.index()]
            | self.piece_bbs[PieceType::Rook.index()];
        let piece = if pnr.overlaps(square.bitboard()) {
            if self.piece_bbs[PieceType::Pawn.index()].overlaps(square.bitboard()) {
                PieceType::Pawn
            } else if self.piece_bbs[PieceType::Knight.index()].overlaps(square.bitboard()) {
                PieceType::Knight
            } else {
                PieceType::Rook
            }
        } else {
            if self.piece_bbs[PieceType::Bishop.index()].overlaps(square.bitboard()) {
                PieceType::Bishop
            } else if self.piece_bbs[PieceType::Queen.index()].overlaps(square.bitboard()) {
                PieceType::Queen
            } else {
                PieceType::King
            }
        };

        Some(Piece::new(piece, color))
    }

    /// Gets a [`BitBoard`] containing the locations of all the pieces of a given piece type and color.
    #[inline]
    pub fn query(&self, piece: impl Into<Piece>) -> BitBoard {
        let piece = piece.into();
        self.piece_bbs[piece.kind.index()] & self.color_bbs[piece.color.index()]
    }

    /// Gets a [`BitBoard`] containing the locations of all the pieces on the [`ChessBoard`].
    #[inline]
    pub fn occupancy(&self) -> BitBoard {
        self.color_occupancy(Color::White) | self.color_occupancy(Color::Black)
    }

    /// Gets a [`BitBoard`] containing the locations of all the pieces of a given color.
    #[inline]
    pub fn color_occupancy(&self, color: Color) -> BitBoard {
        self.color_bbs[color.index()]
    }

    /// Gets a [`BitBoard`] containing the locations of all the pieces of a given piece type.
    #[inline]
    pub fn piece_occupancy(&self, piece: PieceType) -> BitBoard {
        self.piece_bbs[piece.index()]
    }

    /// Checks if the castling right for a given [`CastleSide`] and [`Color`] is set.
    #[inline]
    pub fn is_castle_right_set(&self, side: CastleSide, color: Color) -> bool {
        self.castling_rights.is_set(side, color)
    }

    /// Gets the square potentially targeted by en passant.
    #[inline]
    pub fn en_passant_sq(&self) -> Option<Square> {
        self.en_passant
    }

    /// Gets the current turn.
    #[inline]
    pub fn turn(&self) -> Color {
        self.turn
    }

    /// Gets the pinned pieces.
    #[inline]
    pub fn pinned(&self) -> BitBoard {
        self.pinned
    }

    /// Gets the square of the king of a given [`Color`] on the [`ChessBoard`].
    #[inline]
    pub fn get_king_square(&self, color: Color) -> Square {
        self.query((PieceType::King, color))
            .b_scan_forward()
            .unwrap()
    }

    /// Gets the checkers.
    #[inline]
    pub fn checkers(&self) -> BitBoard {
        self.checkers
    }

    /// Gets a hash for the [`ChessBoard`].
    #[inline]
    pub fn hash(&self) -> ZobristHash {
        self.hash
    }

    /// Gets the half move clock of the [`ChessBoard`].
    #[inline]
    pub fn halfmoves(&self) -> u8 {
        self.half_move_clock
    }

    /// Gets the [`Footprint`] of the [`ChessBoard`].
    #[inline]
    pub fn footprint(&self) -> Footprint {
        Footprint {
            piece_bbs: self.piece_bbs.clone(),
            color_bbs: self.color_bbs.clone(),
            castling_rights: self.castling_rights,
            en_passant: self.en_passant,
            turn: self.turn,
            hash: self.hash,
        }
    }
}

impl PartialEq for ChessBoard {
    fn eq(&self, other: &Self) -> bool {
        (self.piece_bbs == other.piece_bbs)
            && (self.color_bbs == other.color_bbs)
            && (self.turn == other.turn)
            && (self.castling_rights == other.castling_rights)
            && (self.en_passant == other.en_passant)
    }
}

impl Eq for ChessBoard {}

impl Hash for ChessBoard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

const ANSI_RESET_CODE: &str = "\x1b[0m";
const ANSI_GRAY_CODE: &str = "\x1b[90m";

impl Display for ChessBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}  A B C D E F G H{}", ANSI_GRAY_CODE, ANSI_RESET_CODE)?;
        for rank in RANKS.into_iter().rev() {
            write!(
                f,
                "{}{}{} ",
                ANSI_GRAY_CODE,
                rank.to_u8() + 1,
                ANSI_RESET_CODE
            )?;
            for file in FILES {
                let square = Square::at(rank, file);
                let end = if file == File::H { '\n' } else { ' ' };

                match &self.piece_at(square) {
                    None => write!(f, "-{}", end)?,
                    Some(piece) => write!(f, "{}{}", piece.to_char(), end)?,
                };
            }
        }
        write!(f, "")
    }
}

impl Display for Move {
    /// Displays the [`Move`] in algebraic chess notation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Quiet { start, end, .. }
            | Move::Capture { start, end, .. }
            | Move::Castle { start, end, .. }
            | Move::DoublePawnPush { start, end }
            | Move::EnPassant { start, end } => {
                write!(f, "{}{}", start, end)
            }
            Move::Promote { start, end, target } | Move::PromoteCapture { start, end, target } => {
                write!(f, "{}{}{}", start, end, target.to_char())
            }
        }
    }
}

impl Default for ChessBoard {
    /// The default [`Chessboard`] is a chess board in the starting position.
    fn default() -> Self {
        Self::new()
    }
}
