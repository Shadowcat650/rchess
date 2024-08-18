use rchess::{ChessGame, GameResult};
use crate::input_getter::InputGetter;

pub struct GameController {
    game: ChessGame
}

impl GameController {
    pub fn new() -> Self {
        Self {
            game: ChessGame::new(),
        }
    }

    pub fn run(mut self) {
        let mut input_getter = InputGetter::new();

        loop {
            println!("{}", self.game.board());
            println!("{:?} make your move.", self.game.board().turn());

            let mut mv = self.game.create_str_move(&input_getter.get_input());
            while mv.is_err() {
                println!("Invalid move.");
                mv = self.game.create_str_move(&input_getter.get_input());
            }

            self.game.make_move(mv.unwrap());
            if let Some(res) = self.game.result() {
                println!("{}", self.game.board());
                match res {
                    GameResult::WhiteWins => println!("White wins!"),
                    GameResult::BlackWins => println!("Black wins!"),
                    GameResult::Draw { .. } => println!("It's a draw!")
                }
                break;
            }
        }
    }
}