use crate::chessboard::castling_rights::CastlingRights;
use crate::defs::*;
use crate::{ChessBoard, ZobristHash};

/// The [`BoardBuilder`] struct helps construct a [`ChessBoard`].
pub struct BoardBuilder {
    pub(super) piece_map: [Option<(Piece, Color)>;64],
    pub(super) piece_bbs: [BitBoard;6],
    pub(super) color_bbs: [BitBoard;2],
    pub(super) turn: Option<Color>,
    pub(super) castling_rights: CastlingRights,
    pub(super) en_passant_square: Option<Square>,
    pub(super) hash: ZobristHash
}

impl BoardBuilder {
    pub fn new() -> Self {
        Self {
            piece_map: [None;64],
            piece_bbs: [BitBoard::EMPTY;6],
            color_bbs: [BitBoard::EMPTY;2],
            turn: None,
            castling_rights: CastlingRights::new(),
            en_passant_square: None,
            hash: ZobristHash::new(),
        }
    }

    pub fn piece(mut self, square: Square, piece: Piece, color: Color) -> Result<Self, ()> {
        if piece == Piece::King {
            if !(self.piece_bbs[Piece::King.index()] & self.color_bbs[color.index()]).is_empty() {
                return Err(());
            }
        } else if piece == Piece::Pawn {
            match color {
                Color::White => if square.rank() == Rank::Eighth { return Err(()); },
                Color::Black => if square.rank() == Rank::First { return Err(()); },
            }
        }

        if self.piece_map[square.index()].is_some() {
            return Err(());
        }

        self.piece_map[square.index()] = Some((piece, color));
        self.piece_bbs[piece.index()] |= square.bitboard();
        self.color_bbs[color.index()] |= square.bitboard();
        self.hash.piece(square, piece, color);

        Ok(self)
    }

    pub fn turn(mut self, color: Color) -> Result<Self, ()> {
        if self.turn.is_some() {
            return Err(());
        }

        self.turn = Some(color);
        if color == Color::Black {
            self.hash.toggle_turn();
        }

        Ok(self)
    }

    pub fn castle_right(mut self, side: CastleSide, color: Color) -> Result<Self, ()> {
        if self.castling_rights.is_set(side, color) {
            return Err(());
        }

        self.castling_rights.set(side, color);
        self.hash.castle_right(side, color);

        Ok(self)
    }

    pub fn en_passant(mut self, square: Square) -> Result<Self, ()> {
        if self.en_passant_square.is_some() {
            return Err(());
        }

        self.en_passant_square = Some(square);
        self.hash.ep(square);

        Ok(self)
    }

    pub fn finish(self) -> Result<ChessBoard, ()> {
        ChessBoard::from_builder(self)
    }
}