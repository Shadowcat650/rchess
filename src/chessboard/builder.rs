use crate::chessboard::castling_rights::CastlingRights;
use crate::chessboard::chessboard::BuilderConversionError;
use crate::defs::*;
use crate::{ChessBoard, ZobristHash};
use thiserror::Error;

/// The [`BoardBuilderError`] enum is the error type produced by the [`BoardBuilder`].
#[derive(Error, Debug, Copy, Clone, Eq, PartialEq)]
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
pub struct BoardBuilder {
    pub(super) piece_map: [Option<(Piece, Color)>; 64],
    pub(super) piece_bbs: [BitBoard; 6],
    pub(super) color_bbs: [BitBoard; 2],
    pub(super) turn: Option<Color>,
    pub(super) castling_rights: CastlingRights,
    pub(super) en_passant_square: Option<Square>,
    pub(super) hash: ZobristHash,
}

impl BoardBuilder {
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

    pub fn piece(
        mut self,
        square: Square,
        piece: Piece,
        color: Color,
    ) -> Result<Self, BoardBuilderError> {
        if piece == Piece::King {
            if !(self.piece_bbs[Piece::King.index()] & self.color_bbs[color.index()]).is_empty() {
                return Err(BoardBuilderError::TwoKings);
            }
        } else if piece == Piece::Pawn {
            match color {
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

        self.piece_map[square.index()] = Some((piece, color));
        self.piece_bbs[piece.index()] |= square.bitboard();
        self.color_bbs[color.index()] |= square.bitboard();
        self.hash.piece(square, piece, color);

        Ok(self)
    }

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

    pub fn en_passant(mut self, square: Square) -> Result<Self, BoardBuilderError> {
        if self.en_passant_square.is_some() {
            return Err(BoardBuilderError::EnPassantAlreadySet);
        }

        self.en_passant_square = Some(square);
        self.hash.ep(square);

        Ok(self)
    }

    pub fn finish(self) -> Result<ChessBoard, BuilderConversionError> {
        ChessBoard::from_builder(self)
    }
}
