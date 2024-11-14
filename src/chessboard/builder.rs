use crate::chessboard::castling_rights::CastlingRights;
use crate::chessboard::chessboard::BuilderConversionError;
use crate::defs::*;
use crate::{ChessBoard, ZobristHash};
use thiserror::Error;

/// The [`BoardBuilderError`] enum is the error type produced by the [`BoardBuilder`].
#[derive(Error, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BoardBuilderError {
    #[error("tried to insert two kings of the same color")]
    TwoKings,

    #[error("tried to insert a pawn on its last rank")]
    PawnOnLast,

    #[error("tried to insert multiple pieces on the same square")]
    TwoPieces,

    #[error("tried to set the turn, but the turn was already set")]
    TurnAlreadySet,

    #[error("tried to set an already set castling right")]
    CastleRightAlreadySet,

    #[error("tried to set the en passant square, but it was already set")]
    EnPassantAlreadySet,
}

/// The [`BoardBuilder`] struct helps construct a [`ChessBoard`].
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BoardBuilder {
    pub(super) piece_map: [Option<Piece>; 64],
    pub(super) piece_bbs: [BitBoard; 6],
    pub(super) color_bbs: [BitBoard; 2],
    pub(super) turn: Option<Color>,
    pub(super) castling_rights: CastlingRights,
    pub(super) en_passant_square: Option<Square>,
    pub(super) hash: ZobristHash,
}

impl BoardBuilder {
    /// Creates a new [`BoardBuilder`] object.
    #[inline]
    pub fn new() -> Self {
        Self {
            piece_map: [None; 64],
            piece_bbs: [BitBoard::EMPTY; 6],
            color_bbs: [BitBoard::EMPTY; 2],
            turn: None,
            castling_rights: CastlingRights::new(),
            en_passant_square: None,
            hash: ZobristHash::new(),
        }
    }

    /// Adds a piece to the [`BoardBuilder`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BoardBuilder, Color, Piece, PieceType, Square};
    ///
    /// // Create a new board builder with two kings.
    /// let builder = BoardBuilder::new()
    ///     .piece(Square::A1, Piece::WHITE_KING).unwrap()
    ///     .piece(Square::H8, Piece::BLACK_KING).unwrap();
    /// ```
    #[inline]
    pub fn piece(
        mut self,
        square: Square,
        piece: impl Into<Piece>,
    ) -> Result<Self, BoardBuilderError> {
        let piece = piece.into();
        if piece.kind == PieceType::King {
            if !(self.piece_bbs[PieceType::King.index()] & self.color_bbs[piece.color.index()])
                .is_empty()
            {
                return Err(BoardBuilderError::TwoKings);
            }
        } else if piece.kind == PieceType::Pawn {
            match piece.color {
                Color::White => {
                    if square.rank() == Rank::Eighth {
                        return Err(BoardBuilderError::PawnOnLast);
                    }
                }
                Color::Black => {
                    if square.rank() == Rank::First {
                        return Err(BoardBuilderError::PawnOnLast);
                    }
                }
            }
        }

        if self.piece_map[square.index()].is_some() {
            return Err(BoardBuilderError::TwoPieces);
        }

        self.piece_map[square.index()] = Some(piece);
        self.piece_bbs[piece.kind.index()] |= square.bitboard();
        self.color_bbs[piece.color.index()] |= square.bitboard();
        self.hash.piece(square, piece);

        Ok(self)
    }

    /// Sets the turn of the [`BoardBuilder`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BoardBuilder, Color};
    ///
    /// // Create a board builder with the turn set to black.
    /// let builder = BoardBuilder::new()
    ///     .turn(Color::Black).unwrap();
    /// ```
    #[inline]
    pub fn turn(mut self, color: Color) -> Result<Self, BoardBuilderError> {
        if self.turn.is_some() {
            return Err(BoardBuilderError::TurnAlreadySet);
        }

        self.turn = Some(color);
        if color == Color::Black {
            self.hash.toggle_turn();
        }

        Ok(self)
    }

    /// Adds a castling right to the [`BoardBuilder`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BoardBuilder, CastleSide, Color};
    ///
    /// // Create a board builder all castling rights.
    /// let builder = BoardBuilder::new()
    ///     .castle_right(CastleSide::Kingside, Color::White).unwrap()
    ///     .castle_right(CastleSide::Queenside, Color::White).unwrap()
    ///     .castle_right(CastleSide::Kingside, Color::Black).unwrap()
    ///     .castle_right(CastleSide::Queenside, Color::Black).unwrap();
    /// ```
    #[inline]
    pub fn castle_right(
        mut self,
        side: CastleSide,
        color: Color,
    ) -> Result<Self, BoardBuilderError> {
        if self.castling_rights.is_set(side, color) {
            return Err(BoardBuilderError::CastleRightAlreadySet);
        }

        self.castling_rights.set(side, color);
        self.hash.castle_right(side, color);

        Ok(self)
    }

    /// Sets the en passant square of the [`BoardBuilder`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BoardBuilder,Square};
    ///
    /// // Create a board builder with an en passant square set.
    /// let builder = BoardBuilder::new()
    ///     .en_passant(Square::E3).unwrap();
    /// ```
    #[inline]
    pub fn en_passant(mut self, square: Square) -> Result<Self, BoardBuilderError> {
        if self.en_passant_square.is_some() {
            return Err(BoardBuilderError::EnPassantAlreadySet);
        }

        self.en_passant_square = Some(square);
        self.hash.ep(square);

        Ok(self)
    }

    /// Converts the [`BoardBuilder`] into a [`ChessBoard`].
    ///
    /// # Examples
    /// ```no_run
    /// use rchess::{BoardBuilder, Color, Piece, Square};
    ///
    /// // Create a new chess board with two kings with black to move.
    /// let builder = BoardBuilder::new()
    ///     .piece(Square::A1, Piece::WHITE_KING).unwrap()
    ///     .piece(Square::H8, Piece::BLACK_KING).unwrap()
    ///     .turn(Color::Black).unwrap()
    ///     .finish().unwrap();
    /// ```
    #[inline]
    pub fn finish(self) -> Result<ChessBoard, BuilderConversionError> {
        ChessBoard::from_builder(self)
    }
}
