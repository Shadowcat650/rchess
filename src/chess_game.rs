use crate::chessboard::Footprint;
use crate::defs::START_FEN;
use crate::{BitBoard, ChessBoard, Color, Move, MoveGen, Piece, Square, ZobristHash};
use std::collections::HashMap;
use std::ops::Index;

/// The [`GameResult`] enum represents the result of a chess game.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameResult {
    WhiteWins,
    BlackWins,
    Draw { reason: DrawReason },
}

/// The [`DrawReason`] enum represents the thing that caused a draw to occur.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DrawReason {
    InsufficientMaterial,
    Stalemate,
    ThreefoldRepetition,
    FiftyMoves,
}

/// The [`ChessGame`] struct represents a game of chess.
pub struct ChessGame {
    /// The game state.
    state: ChessBoard,

    /// The moves that can be made in the current position.
    position_moves: Vec<Move>,

    /// The reversible move history (for 3-fold repetition checking).
    history: HashMap<Footprint, u8>,

    /// The starting fen of the chess game.
    start_fen: String,

    /// The moves made in the game.
    made_moves: Vec<Move>,

    /// The result of the chess game.
    result: Option<GameResult>,
}

impl ChessGame {
    pub fn new() -> Self {
        let state = ChessBoard::new();
        let position_moves = MoveGen::legal(&state).to_vec();

        // Initialize repetition history.
        let mut history = HashMap::new();
        history.insert(state.footprint(), 1);

        Self {
            state,
            position_moves,
            history,
            start_fen: START_FEN.to_string(),
            made_moves: vec![],
            result: None,
        }
    }

    pub fn moves(&self) -> &Vec<Move> {
        &self.position_moves
    }

    pub fn make_move(&mut self, mv: Move) {
        self.state.make_move(mv);
        self.made_moves.push(mv);

        if let Move::Quiet { .. } = mv {
        } else {
            // Clear repetition history.
            self.history.clear();
        }

        if let Some(count) = self.history.get_mut(&self.state.footprint()) {
            *count += 1;

            // Look for repetition.
            if *count == 3 {
                self.result = Some(GameResult::Draw {
                    reason: DrawReason::ThreefoldRepetition,
                })
            }
        } else {
            self.history.insert(self.state.footprint(), 1);
        }

        self.position_moves = MoveGen::legal(&self.state).to_vec();

        // Look for checkmate/stalemate.
        if self.position_moves.is_empty() {
            if self.state.checkers().is_empty() {
                self.result = Some(GameResult::Draw {
                    reason: DrawReason::Stalemate,
                })
            } else {
                self.result = Some(match self.state.turn() {
                    Color::White => GameResult::BlackWins,
                    Color::Black => GameResult::WhiteWins,
                });
            }
        }

        // Look for 50 move rule.
        if self.state.halfmoves() >= 50 {
            self.result = Some(GameResult::Draw {
                reason: DrawReason::FiftyMoves,
            })
        }

        if self.state.color_occupancy(Color::White).popcnt() == 1 {
            if self.state.color_occupancy(Color::Black).popcnt() == 1 {
                self.result = Some(GameResult::Draw {
                    reason: DrawReason::InsufficientMaterial,
                })
            } else if self.state.color_occupancy(Color::Black).popcnt() == 2 {
                if !self.state.query(Piece::Bishop, Color::Black).is_empty()
                    || !self.state.query(Piece::Knight, Color::Black).is_empty()
                {
                    self.result = Some(GameResult::Draw {
                        reason: DrawReason::InsufficientMaterial,
                    })
                }
            }
        } else if self.state.color_occupancy(Color::White).popcnt() == 2 {
            if self.state.color_occupancy(Color::Black).popcnt() == 2 {
                if self
                    .state
                    .query(Piece::Bishop, Color::White)
                    .overlaps(BitBoard::WHITE_SQUARES)
                {
                    if self
                        .state
                        .query(Piece::Bishop, Color::Black)
                        .overlaps(BitBoard::WHITE_SQUARES)
                    {
                        self.result = Some(GameResult::Draw {
                            reason: DrawReason::InsufficientMaterial,
                        })
                    }
                } else if self
                    .state
                    .query(Piece::Bishop, Color::White)
                    .overlaps(BitBoard::BLACK_SQUARES)
                {
                    if self
                        .state
                        .query(Piece::Bishop, Color::Black)
                        .overlaps(BitBoard::BLACK_SQUARES)
                    {
                        self.result = Some(GameResult::Draw {
                            reason: DrawReason::InsufficientMaterial,
                        })
                    }
                }
            } else if self.state.color_occupancy(Color::Black).popcnt() == 1 {
                if !self.state.query(Piece::Bishop, Color::White).is_empty()
                    || !self.state.query(Piece::Knight, Color::White).is_empty()
                {
                    self.result = Some(GameResult::Draw {
                        reason: DrawReason::InsufficientMaterial,
                    })
                }
            }
        }
    }

    pub fn result(&self) -> Option<GameResult> {
        self.result
    }

    pub fn is_legal_move(&self, start_sq: Square, end_sq: Square) -> bool {
        self.create_move(start_sq, end_sq).is_ok()
    }

    pub fn create_move(&self, start_sq: Square, end_sq: Square) -> Result<Move, ()> {
        if let Some(mv) = self.position_moves.iter().find(|mv| match *mv {
            Move::Quiet { start, end, .. }
            | Move::Capture { start, end, .. }
            | Move::Castle { start, end, .. }
            | Move::DoublePawnPush { start, end, .. }
            | Move::EnPassant { start, end, .. }
            | Move::Promote { start, end, .. }
            | Move::PromoteCapture { start, end, .. } => *start == start_sq && *end == end_sq,
        }) {
            Ok(*mv)
        } else {
            Err(())
        }
    }

    pub fn create_str_move(&self, str: &str) -> Result<Move, ()> {
        if str.len() < 4 {
            return Err(());
        }

        // Get move start & end squares.
        let start = Square::from_string(str.index(0..=1))?;
        let end = Square::from_string(str.index(2..=3))?;

        self.create_move(start, end)
    }

    pub fn board(&self) -> &ChessBoard {
        &self.state
    }
}
