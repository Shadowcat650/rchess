use crate::chessboard::Footprint;
use crate::{
    BitBoard, ChessBoard, Color, FenLoadError, Move, MoveCreationError, MoveGen, Piece, Square,
    StrMoveCreationError,
};
use std::collections::HashMap;

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

    /// The moves made in the game.
    made_moves: Vec<Move>,

    /// The result of the chess game.
    result: Option<GameResult>,
}

impl ChessGame {
    /// Creates a new [`ChessGame`] with the board in the starting position.
    #[inline]
    pub fn new() -> Self {
        let state = ChessBoard::new();
        Self::initialize_game(state)
    }

    /// Creates a new [`ChessGame`] with the board in the given fen position.
    #[inline]
    pub fn from_fen(fen: &str) -> Result<Self, FenLoadError> {
        let state = ChessBoard::from_fen(fen)?;
        Ok(Self::initialize_game(state))
    }

    /// Initializes a new [`ChessGame`].
    fn initialize_game(state: ChessBoard) -> Self {
        // Get the position moves.
        let position_moves = MoveGen::legal(&state).to_vec();

        // Initialize repetition history.
        let mut history = HashMap::new();
        history.insert(state.footprint(), 1);

        // Create the game object.
        let mut game = Self {
            state,
            position_moves,
            history,
            made_moves: vec![],
            result: None,
        };

        // Look for terminal state.
        game.look_for_terminal();
        if game.result.is_some() {
            game.position_moves.clear();
        }

        game
    }

    /// Makes a move.
    ///
    /// If the game is over, an `Err` is returned.
    ///
    /// # Examples
    /// ```
    /// use rchess::ChessGame;
    ///
    /// // Create a new chess game
    /// let mut game = ChessGame::new();
    ///
    /// // Make a move in the chess game.
    /// let mv = game.moves()[0];
    /// game.make_move(mv).unwrap();
    /// ```
    #[inline]
    pub fn make_move(&mut self, mv: Move) -> Result<(), ()> {
        if self.result.is_some() {
            return Err(());
        }

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
                });
                return Ok(());
            }
        } else {
            self.history.insert(self.state.footprint(), 1);
        }

        self.position_moves = MoveGen::legal(&self.state).to_vec();

        self.look_for_terminal();

        Ok(())
    }

    /// Looks for a terminal state that is not a repetition.
    fn look_for_terminal(&mut self) {
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
        if self.state.halfmoves() >= 100 {
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

    /// Returns `true` if the start and end squares make a legal move.
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessGame, Square};
    ///
    /// // Create a new chess game.
    /// let game = ChessGame::new();
    /// assert!(game.is_legal_move(Square::E2, Square::E4));
    /// assert!(!game.is_legal_move(Square::E2, Square::E5));
    /// ```
    #[inline]
    pub fn is_legal_move(&self, start: Square, end: Square) -> bool {
        if self.result.is_some() {
            return false;
        }
        MoveGen::is_legal(&self.state, start, end)
    }

    /// Attempts to turn a start and end square into a [`Move`].
    ///
    /// Promotions default to a queen promotion.
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessGame, Move, Square};
    ///
    /// // Create a new chess game.
    /// let game = ChessGame::new();
    ///
    /// // Create the move "e2e4".
    /// let mv = game.create_move(Square::E2, Square::E4).unwrap();
    /// assert_eq!(mv, Move::DoublePawnPush { start: Square::E2, end: Square::E4 });
    /// ```
    #[inline]
    pub fn create_move(&self, start: Square, end: Square) -> Result<Move, MoveCreationError> {
        if self.result().is_some() {
            return Err(MoveCreationError);
        }
        MoveGen::create_move(&self.state, start, end)
    }

    /// Attempts to turn a start and end square into a [`Move`].
    ///
    /// The promotion target is set, but the move is ot necessarily a promotion.
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessGame, Move, Piece, Square};
    ///
    /// // Create a new chess game.
    /// let game = ChessGame::from_fen("3k4/PK6/8/8/8/8/8/8 w - -").unwrap();
    ///
    /// // Promote to a knight.
    /// let mv = game.create_promote_move(Square::A7, Square::A8, Piece::Knight).unwrap();
    /// assert_eq!(mv, Move::Promote { start: Square::A7, end: Square::A8, target: Piece::Knight });
    ///
    /// // Make a normal non-promotion move.
    /// let mv = game.create_promote_move(Square::B7, Square::B8, Piece::Knight).unwrap();
    /// assert_eq!(mv, Move::Quiet { start: Square::B7, end: Square::B8, moving: Piece::King });
    /// ```
    #[inline]
    pub fn create_promote_move(
        &self,
        start: Square,
        end: Square,
        target: Piece,
    ) -> Result<Move, MoveCreationError> {
        if self.result().is_some() {
            return Err(MoveCreationError);
        }
        MoveGen::create_promotion_move(&self.state, start, end, target)
    }

    /// Attempts to convert a string in algebraic chess notation, into a [`Move`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{ChessGame, Move, Square};
    ///
    /// // Create a new chess game.
    /// let game = ChessGame::new();
    ///
    /// // Create the move "e2e4".
    /// let mv = game.create_str_move("e2e4").unwrap();
    /// assert_eq!(mv, Move::DoublePawnPush { start: Square::E2, end: Square::E4 });
    /// ```
    #[inline]
    pub fn create_str_move(&self, str: &str) -> Result<Move, StrMoveCreationError> {
        if self.result().is_some() {
            return Err(StrMoveCreationError::IllegalMove(MoveCreationError));
        }
        MoveGen::create_str_move(&self.state, str)
    }

    /// Gets s reference to the underlying [`ChessBoard`].
    #[inline]
    pub fn board(&self) -> &ChessBoard {
        &self.state
    }

    /// Gets a reference to all the moves made in the [`ChessGame`].
    #[inline]
    pub fn made_moves(&self) -> &Vec<Move> {
        &self.made_moves
    }

    /// Gets the result of the [`ChessGame`], if any.
    #[inline]
    pub fn result(&self) -> Option<GameResult> {
        self.result
    }

    /// Gets a list of possible moves for the active color to make.
    #[inline]
    pub fn moves(&self) -> &Vec<Move> {
        &self.position_moves
    }
}

impl Default for ChessGame {
    /// The default for a [`ChessGame`] is a chess game in the starting position.
    fn default() -> Self {
        Self::new()
    }
}
